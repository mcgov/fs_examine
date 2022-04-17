ext4


what works(ish)

64/32-bit
reading superblock, reading basic block groups (bg), inodes, directory entries, extent tree, file content from extent leaves, reading names of extended attrs, getting access/creation/modified times, crc32c and crc16 checksum

what sort of works:
checksums (checks some of them), magic fields (checks some of them)



what doesn't work for sure
checksums (some of them)
compression
enciphered anything
big attrs
huge inodes
reading stuff in 'extrasize' after inode
reading bg hash trees
reading extended attrs values
reading the journal
inline data in inodes
META-BGs

