# Ntree :evergreen_tree:
`ntree` is a modern reimplementation of the classic command line utility `tree`.
It recursively lists directories and files, visualising their hierarchy.

## Basic usage

```
$ ntree
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── notes.md
└── src
    ├── main.rs
    ├── processor
    │   ├── dummy_processor.rs
    │   ├── mod.rs
    │   ├── print_processor.rs
    │   └── tree_processor.rs
    └── tree.rs

2 directories, 10 files
```

Run `ntree --help` for a full reference.

## Motivation
I love `tree`! It's a simple and useful addition to the toolbelt. I've been
thinking about some improvements I'd like to make for a long. So naturally, when
I started learning the Rust programming language and needed a project, a
`tree`-clone is what came to mind!

Some new features in `ntree` include:

- Filter git-ignored files with `-g`.
- Foolproof `-P` and `-I` semantics. Separating patterns with `|` can cause
trouble when filenames include the pipe character itself. `ntree` instead
accepts `-P` and `-I` being applied multiple times with different arguments
(TBD).
- A modularized and DRY codebase. It's easy to add new features!
- Blazing fast runtime. Although ordinarily irrelevant, `ntree /` outperforms
`tree /` by a factor of 5 on a hot cache on my local machine :)
(tested 2016-10-26).

## Contact
I'd love to discuss Ntree with you! If you feel inclined, open an issue on GitHub or send an email.

## License
Ntree is licensed under the MPL 2.0. See LICENSE for the full license text.

*Copyright 2016 Jacob Wahlgren*
