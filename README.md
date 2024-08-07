# Objective
Yet another fuzzy finder binary for use in vim, POSIX shells and other programs.
Will support interactive and un-interactive modes

# Methodology
- We will recursively expand the file system direcotries and insert all contents into a TRIE or PATRICIA tree, so that the files can be efficiently searched by their names.
- Names will be broken into n-grams for fuzzy finding in the tree (not just searching by prefix).
    - This might blow up space complexity but memory is relatively cheap
- This tree will be saved in a temp file so that further queries on a certain path are faster.
- Generated cache is invalidated after a specific ammount of time (can be configured at compile time) 
- "Search again" keyboard command will be available to account for new files
    - Extra heuristics will attempt to get FS updates automatically
