# dh (display help)

![MIT licensed][license-image]

[license-image]: https://img.shields.io/github/license/workingj/dh.svg

A small commandline tool for showing simple custom help files or notes.
Waits for retrun key after 24 lines and shows the next 24 lines.

## Usage

### build

```$ cargo build --release```

Register dh in the environment variables, so you can run it from the commandline with  
```$ dh [argument]```

### needed files

Needed config file must be named and look like this:

help.toml

```text
Command  Description
help    this file!
test    Test...
```

If the help file doesn't exist, it will be created automaticly.  
Help files must be named like the command in the config file and can be anny simple plain text.

### Example

Directory:

```text
└── dh
    ├── cargo.toml
    ├── git.toml
    ├── help.toml
    └── dh.exe (bin)
```

Usage:

```$ dh git```

Will show the contents of the git.toml file.
