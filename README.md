RDKit-Sys
---

Rust code that binds to the C++ rdkit library!

How does it work?
---

RDKit is a C++ mega-library, full of cheminformatics wisdom. We don't want to rewrite RDKit in Rust, we should instead meet somewhere in the middle and
"bridge" Rust to C++ through some C bindings.

Prerequisites
---

On Mac:

    brew install boost cmake llvm

boost is a RDKit C++ dependency, cmake is how you create RDKit's Makefile, llvm is used by bind-gen to create the rust
bindings.rs file with all the dylib definitions required to use RDKit from rust.

Testing
---

Get the dylib built first, with some concurrent job spawning to go faster:

    NUM_JOBS=10 cargo build -vv

great for exercising the build.rs script.

Or just run the test suite:

    cargo test

TODO
---

 - [X] build rdkit, with useful flags, from source
 - [X] copy rdkit library files to the "right spot" in the cargo filesystem
 - [X] provide function C function definitions in a format that Rust can use
 - [X] rewrap C functions with useful high level Rust flavors (copied from chrissly31415's repo)
 - [ ] figure out how to `cargo publish` without `--no-verify` (otherwise it detects changes outside of OUTDIR)
 - [X] specify path to RDKit's cffiwrapper.h and all required search paths for other dependent headers
 - [ ] use conditional rebuild logic to make the library build experience more reliable (for now, if you get stuck, try `cargo clean` and retry with `cargo build -vv`)

Related Documentation
---

 - https://www.rdkit.org/docs/cppapi/index.html
 - https://cxx.rs/

Prior art
---

 - https://github.com/apahl/rdkit_cxx
 - [rdkafka's excellent librdkafka build.rs](https://github.com/fede1024/rust-rdkafka/blob/master/rdkafka-sys/build.rs)
 - https://iwatobipen.wordpress.com/2022/01/29/use-rdkit-from-rust-rdkit-rdkitcffi-rust/
 - [an attempt at using rdkit in rust but without docs on how to build rdkit](https://github.com/iwatobipen/rust_rdkit/)
