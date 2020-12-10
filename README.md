# WTF is this?
a small init process intended for a custome built linux distribution 
this init process decides it's processes boot order at compile time. 
this is done too limit the possible attack surface one can have (when combined with over security measures like limiting forking in the system)
this init process can be usefull when creating a minimal foot print linux powered device. this is because unlike more powerful init processes this one doesn't load any terminal or third-party utils 

# How do i use it?
1. Download the repo 
2. run `CONFIG_FILE=example.json cargo build` to build it