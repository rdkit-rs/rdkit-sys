RDKit-Sys
---

Build a cffi-able rdkit! Using the magic of [cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

Similar in spirit to [rdkitcffi](https://github.com/chrissly31415/rdkitcffi) but you don't want to rely on magic .so files
committed to a git repo. This repo also supports OS X. Use this crate and cargo will automatically build rdkit, with all the
right cmake definitions, from scratch.

How does it work?
---

RDKit is a C++ mega-library, full of cheminformatics wisdom. We don't want to rewrite RDKit in Rust and C++ is not a straight-forward integration
target. C++, from what I can tell, does not have a stable "application binary interface" (ABI), i.e., no stable cross-language calling convention.
And even if it did have a stable C++ ABI, rust doesn't appear to support any flavor of it.
So we instead rely on a RDKit C++ wrapper, part of the official rdkit project, called the RDKit CFFI, or `c foreign function interface` where
all C++ code, inheritance and all, is wrapped with flat C style functions. Good news, calling out to C is Rust's bread and butter and part of
its wonderfully infectious design. [Read more about CFFI from the RDKit lead](https://greglandrum.github.io/rdkit-blog/technical/2021/05/01/rdkit-cffi-part1.html).

This CFFI is not a super popular build option of RDKIt, so I don't know if it's common among various operating system distributions.  Rust to the rescue,
we can use `build.rs` and the easy to use `cmake` helper library to build RDKit with the `RDK_BUILD_CFFI_LIB` definition set. The helper library also makes
the output RDKit library pieces go to a discoverable spot for Rust programs.

The CFFI flavor of RDKit is still emitted as a dynamically loaded library so our Rust software will always need to "find" the RDKit library somewhere
on disk. I could not figure out how to create a `.a` variant of the library, which would be suitable for static linking. Until I can create a `.a` just be
aware it might be a pain to run the RDKit program outside of a carefully constructed `cargo run ...` invocation.

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

Prior art
---

 - [rdkafka's excellent librdkafka build.rs](https://github.com/fede1024/rust-rdkafka/blob/master/rdkafka-sys/build.rs)
 - https://iwatobipen.wordpress.com/2022/01/29/use-rdkit-from-rust-rdkit-rdkitcffi-rust/
 - [an attempt at using rdkit in rust but without docs on how to build rdkit](https://github.com/iwatobipen/rust_rdkit/)

Working with git submodules
---

Initialize the contents of the submodules: `git submodule update --init --recursive`

Update the submodules: `git submodule update --recursive --remote`

Reset the RDKit submodule: `git submodule add --force https://github.com/rdkit/rdkit.git rdkit`
 