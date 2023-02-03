# dh (display help)

![MIT licensed][license-image]

[license-image]: https://img.shields.io/github/license/workingj/dh.svg

A simple commandline tool for showing custom help files or notes.
Waits for retrun key after 32 lines and shows the next 32 lines.

## build

```$ cargo build --release```

Register dh in the environment variables, so you can run it from the commandline with  
```$ dh [argument]```

Will show the contents of the git.toml file.

## INSTRUCTION

Create your helpfiles in the root directory of dh or,
set a location with the environment var 'DH_LIBRARY'
and put your helpfiles there.

**Files**

Use '.toml' for File extension.

### USAGE EXAMPLE

```text
Filename      Command       Output
help.toml     $ dh help     this file!
yours.toml    $ dh yours    what ever you have jotted down.
              $ dh          Lists all Helpfiles within the 'DH_LIBRARY'-Location
```

### HIGHLIGHTING

```text
__Lines that start with '#','##','###, *, //' will be highlighted__
'# '   HEADER 1   in Green
'## '  Header 2  in Blue
'### ' Header 3 in Turkoise
'$ ' Commad Line  in Oragen
'// '  Comment   in Slate Blue
'* ' Bullet Point in Chartreuse
```
