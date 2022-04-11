ext4


what works(ish)

32-bit
reading superblock, reading basic block groups (bg), inodes, directory entries, extent headers that point directly to leaves, file content from extent leaves, reading names of extended attrs, getting access/creation/modified times

what doesn't work for sure
checksums
compression
enciphered anything
big attrs
huge inodes
reading stuff in 'extrasize' after inode
reading an entire extent tree
reading bg hash trees
reading extended attrs values
64bit
reading the journal
inline data in inodes
META-BGs

