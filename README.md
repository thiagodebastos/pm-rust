## A terminal project manager written in Rust

![Rust workflow](https://github.com/thiagodebastos/pm-rust/actions/workflows/rust.yml/badge.svg)


### Features:

#### Core:

To begin with, I want feature parity with [EivindArvesen's
prm](https://github.com/EivindArvesen/prm)


### Todo:

- [ ]  List active project instances.
- [ ]  Add project(s).
- [ ]  Copy project.
- [ ]  Edit project(s).
- [ ]  List all projects.
- [ ]  Remove project(s).
- [ ]  Rename project.
- [ ]  Start project.
- [ ]  Stop active project.
- [ ]  Display this information.
- [ ]  Display version info.


## Usage

```shell
usage: prm <option> [<args>] ...

Options:
  active                   List active project instances.
  add <project name>       Add project(s).
  copy <old> <new>         Copy project.
  edit <project name>      Edit project(s).
  list                     List all projects.
  remove <project name>    Remove project(s).
  rename <old> <new>       Rename project.
  start <project name>     Start project.
  stop                     Stop active project.
  -h --help                Display this information.
  -v --version             Display version info.
```
