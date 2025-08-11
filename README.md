# auto_dr - minimal ls -l and dir replacement
## Installation
The installation script can be called via the following command:
```bash
$ sudo bash install.sh
```
It requires sudo privileges to copy the compiled executable into the `/usr/bin/` directory.
Setting up an alias like `alias dir="auto_dr"` is recommended.

## Usage
The program can be used as follows:
```bash
$ auto_dr [path]
```
If `[path]` is left empty, the program will execute in the current directory.
