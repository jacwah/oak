# Oak :deciduous_tree:
`oak` is a modern reincarnation of the classic command line utility `tree`.
It recursively lists directories and files, visualizing their hierarchy.

## Basic usage

```
$ oak
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── filters.rs
    ├── lib.rs
    ├── main.rs
    ├── print_processor.rs
    ├── tree.rs
    └── tree_processor.rs

1 directory, 10 files
```

Run `oak --help` for a full reference.

## Motivation
I love `tree`! It's a simple and useful addition to the tool belt. Oak
provides features that better into the modern user's workflow. For instance,
in a Git repository, `.gitignore`d files are filtered out by default.

## Contact
I'd love to discuss Oak with you! Open an issue on GitHub or send me an email!

## License
Oak is licensed under the MPL 2.0. See LICENSE for the full license text.

*Copyright 2016 Jacob Wahlgren.*
