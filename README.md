#Objective
The idea of this project is to create a fuzzy finder binary for use in vim, shell and other possibilities.

#Methodology
- We will recursively expand the file system direcotries and insert all contents into a TRIE or PATRICIA tree, so that the files can be efficiently searched by the start of their name.
- This tree will be saved in a temp file so that further queries on a path are faster.
- Cache is invalidated after an hour and the tree is re-generated (can be configured at compile time)
- If file is missed, cache is re-generated to accout for new files.
