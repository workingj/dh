# hlp
A small commandline tool for showing simple custom help files or notes.

## Usage

### build

```$ cargo build --release```

Register hlp.exe in the environment variables, so you can run it from the commanndline with  
```$ hlp [argument]```

### needed files

Needed config file must be named and look like this:

help.hlp

```
Command  Description
help    this file!
test    Test...
```

If the help file doesn't exist, it will be created automaticly.   
Help files must be named like the command in the config file and can be anny simple plain text.

### Example

Directory:
```
└── hlp
    ├── cargo.hlp
    ├── git.hlp
    ├── help.hlp
    └── hlp.exe
```
Usage:

```$ hlp git```

Will show the contents of the git.help file.