RDKIT-Sys
---

Build a cffi-able rdkit! Using the magic of [cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

Similar in spirit to [rdkitcffi](https://github.com/chrissly31415/rdkitcffi) but you don't want to rely on magic .so files
committed to a git repo. Use this crate and your computer will build rdkit, with all the right cmake definitions, from scratch.