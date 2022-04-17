use crate::headers::reader;
use crate::headers::reader::OnDisk;
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct ExtentHeader {
    pub eh_magic: u16,      // 	Magic number, 0xF30A.
    pub eh_entries: u16,    // 	Number of valid entries following the header.
    pub eh_max: u16,        // 	Maximum number of entries that could follow the header.
    pub eh_depth: u16, // 	Depth of this extent node in the extent tree. 0 = this extent node points to data blocks; otherwise, this extent node points to other extent nodes. The extent tree can be at most 5 levels deep: a logical block number can be at most 2^32, and the smallest n that satisfies 4*(((blocksize - 12)/12)^n) >= 2^32 is 5.
    pub eh_generation: u32, // 	Generation of the tree. (Used by Lustre, but not standard ext4).
}

impl ExtentHeader {
    pub fn check_magic(&self) -> bool {
        self.eh_magic == 0xF30A
    }
}

#[derive(Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct ExtentNode {
    pub ei_block: u32,   // 	This index node covers file blocks from 'block' onward.
    pub ei_leaf_lo: u32, // 	Lower 32-bits of the block number of the extent node that is the next level lower in the tree. The tree node pointed to can be either another internal node or a leaf node, described below.
    pub ei_leaf_hi: u16, // 	Upper 16-bits of the previous field.
    pub ei_unused: u16,  //
}

impl ExtentNode {
    pub fn get_block(&self) -> u64 {
        self.ei_leaf_lo as u64 | ((self.ei_leaf_hi as u64) << 32)
    }
}

#[derive(Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct ExtentLeaf {
    pub ee_block: u32,    // 	First file block number that this extent covers.
    pub ee_len: u16, // 	Number of blocks covered by extent. If the value of this field is <= 32768, the extent is initialized. If the value of the field is > 32768, the extent is uninitialized and the actual extent length is ee_len - 32768. Therefore, the maximum length of a initialized extent is 32768 blocks, and the maximum length of an uninitialized extent is 32767.
    pub ee_start_hi: u16, // 	Upper 16-bits of the block number to which this extent points.
    pub ee_start_lo: u32, // 	Lower 32-bits of the block number to which this extent points.
}

#[derive(Deserialize, Copy, Clone)]
#[repr(packed)]
pub struct ExtentTail {
    pub eb_checksum: u32, // 	Checksum of the extent block, crc32c(uuid+inum+igeneration+extentblock)
}

#[derive(Deserialize, Clone)]
pub struct ExtentTree {
    pub hdr: ExtentHeader,
    pub branch: Option<Vec<ExtentNode>>,
    pub leaf: Option<Vec<ExtentLeaf>>,
    pub subtrees: Vec<ExtentTree>,
    pub tail: ExtentTail,
}

impl ExtentLeaf {
    pub fn all_zero(&self) -> bool {
        return self.ee_block == 0
            && self.ee_len == 0
            && self.ee_start_hi == 0
            && self.ee_start_lo == 0;
    }
    pub fn content_block(&self) -> u64 {
        self.ee_start_lo as u64 | ((self.ee_start_hi as u64) << 32)
    }
    pub fn leaf_initialized(&self) -> bool {
        self.ee_len <= 32768
    }
    pub fn block_length(&self) -> u16 {
        if self.leaf_initialized() {
            return self.ee_len;
        }
        0
    }
    pub fn bytes_covered(&self, block_size: u64) -> u64 {
        self.block_length() as u64 * block_size
    }
    pub fn get_file_content(
        &mut self,
        reader: &mut OnDisk,
        block_0: u64,
        block_size: u64,
        content_size: u64,
    ) -> Vec<u8> {
        let block = self.ee_block;
        assert_eq!(self.leaf_initialized(), true);
        let bytes_in_bloc = self.bytes_covered(block_size);
        // println!(
        //     "File block: 0x{:X} bytesin_bloc: 0x{:X} size: 0x{:X}",
        //     block, bytes_in_bloc, content_size
        // );

        let offset =
            reader::get_offset_from_block_number(block_0, self.content_block(), block_size);
        if content_size > bytes_in_bloc {
            return reader.read_bytes_from_file(offset, bytes_in_bloc);
        }
        reader.read_bytes_from_file(offset, content_size)
    }
}

impl ExtentTree {
    pub fn init(block: &[u8]) -> ExtentTree {
        let header = reader::read_header_from_bytes::<ExtentHeader>(&block);
        let sz_hdr = std::mem::size_of::<ExtentHeader>();
        let entries = header.eh_entries as usize;
        let max_entries = header.eh_max;
        let magic = header.eh_magic;
        assert_eq!(0xF30A, magic);
        if entries >= max_entries as usize {
            panic!("oops, entries larger than max entries");
        }
        let mut leaf_op: Option<Vec<ExtentLeaf>> = None;
        let mut branch_op: Option<Vec<ExtentNode>> = None;
        let tail: ExtentTail;
        let leaf_size = std::mem::size_of::<ExtentLeaf>();
        let node_size = std::mem::size_of::<ExtentLeaf>();

        if header.eh_depth == 0 {
            // leaf town
            let mut leafs: Vec<ExtentLeaf> = vec![];
            for i in 0..entries {
                let leaf =
                    reader::read_header_from_bytes::<ExtentLeaf>(&block[sz_hdr + i * leaf_size..]);
                //println!("b:{:x} l:{:x}", leaf.ee_block, leaf.ee_len);
                if leaf.all_zero() {
                    //println!("empty leaf");
                }
                leafs.push(leaf);
            }
            leaf_op = Some(leafs);
            tail = reader::read_header_from_bytes(&block[sz_hdr + entries * leaf_size as usize..]);
        } else {
            //node city
            let mut branches: Vec<ExtentNode> = vec![];

            for i in 0..entries {
                let branch =
                    reader::read_header_from_bytes::<ExtentNode>(&block[sz_hdr + i * node_size..]);
                // println!(
                //     "{:x} {:x} {:x}",
                //     branch.ei_block, branch.ei_leaf_lo, branch.ei_leaf_hi
                // );

                branches.push(branch);
            }
            branch_op = Some(branches);
            tail = reader::read_header_from_bytes(&block[sz_hdr + entries * node_size as usize..]);
        }

        let newtree = ExtentTree {
            hdr: header,
            leaf: leaf_op,
            branch: branch_op,
            tail: tail,
            subtrees: vec![],
        };
        return newtree;
    }

    pub fn ascend(&mut self, reader: &mut OnDisk, block_0: u64, block_size: u64) {
        let depth = self.hdr.eh_depth;
        //println!("ascending: node depth: {}", depth);
        let entries = self.hdr.eh_max;
        if self.hdr.eh_depth != 0 {
            if matches!(self.branch, None) {
                panic!("Extent error: depth was not 0 but there were no branches");
            }
            // FIXME: this is broken, internal node reading is incorrect
            for node in self.branch.as_ref().unwrap() {
                // get the address of the next block
                let addr = node.get_block();
                let offset = reader::get_offset_from_block_number(block_0, addr, block_size);
                // init the block
                let bytes = reader.read_bytes_from_file(offset, 12); //read a block
                let header = reader::read_header_from_bytevec::<ExtentHeader>(bytes);
                assert_eq!(header.check_magic(), true);
                let next_block_size = 12 + (header.eh_entries as u64 * 12) + 4;
                let next_node_block = reader.read_bytes_from_file(offset, next_block_size);
                // add it to the list
                let mut tree = ExtentTree::init(&next_node_block);
                tree.ascend(reader, block_0, block_size);
                self.subtrees.push(tree);
            }
        }
    }

    pub fn walk(
        &mut self,
        reader: &mut OnDisk,
        block_0: u64,
        block_size: u64,
        f_size: usize,
    ) -> Vec<u8> {
        //TODO: this is dumb, loading a huge file right into memory will broke thin
        let depth = self.hdr.eh_depth;
        let mut content: Vec<u8> = vec![];
        //println!("walkng: node depth: {} size_left: {:X}", depth, f_size);
        let mut bytes_left: u64 = 0;
        if self.subtrees.len() == 0 {
            match &self.leaf {
                Some(leafs) => {
                    //println!("leaves: {}", leafs.len());
                    for mut leaf in leafs.clone() {
                        bytes_left = (f_size - content.len()).try_into().unwrap();
                        let mut bytes =
                            leaf.get_file_content(reader, block_0, block_size, bytes_left);
                        content.append(&mut bytes);
                    }
                }
                None => {}
            }
            // we've populated already so subtrees should contain the info needed
            // to get to the other leaves
        }
        for mut tree in self.subtrees.clone() {
            // this doesn't take into account the file block order yet
            bytes_left = (f_size - content.len()).try_into().unwrap();
            let mut tree_bytes = tree.walk(reader, block_0, block_size, bytes_left as usize);
            content.append(&mut tree_bytes);
        }
        //println!("Found content length: {:X}", content.len());
        content
    }
}
