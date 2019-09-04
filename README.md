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

### Usage:

```$ dh git```

Will show the contents of the git.toml file.
