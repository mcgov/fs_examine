ext4


what works(ish)

64/32-bit
reading superblock, reading basic block groups (bg), inodes, directory entries, extent tree, file content from extent leaves, reading names of extended attrs, getting access/creation/modified times, crc32c and crc16 checksum

what sort of works:
checksums (checks some of them), magic fields (checks some of them)

in progress:
reading bg hash trees


what doesn't work for sure
checksums (some of them)
compression
enciphered anything
big attrs
huge inodes
reading stuff in 'extrasize' after inode
reading extended attrs values (can read keys)
reading the journal
inline data in inodes
META-BGs

writing

