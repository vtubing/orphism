# orphism [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/orphism.svg
[crates.io]: https://crates.io/crates/orphism

## Overview

As a VTuber, your model probably came with a bunch of files in a `runtime`
directory. `orphism` is a library for the Rust programming language that lets
you load and interpret those files your own programs, assuming those programs
are written in Rust.

What you do from there is up to you.

## What is this NOT?

- There is no UI, this is a library for building other things with.
- It does not provide a way to **create**, **edit** models, beyond data serialization and deserialization.
  - If you need this, the format vendor's official editor provides this.
- It does not provide any sort of non-Rust interface to these assets.
  - If you need this, the format vendor's official SDK provides this.

While it may be possible to build some or all of those things using this library,
they are all **very deliberately** out of scope for this project.

## What IS it then?

- It provides a collection of data types to read, validate, and analyze these files.
- It provides a foundation for other (more interesting) things to build on top of.
- It is completely unencumbered by the format vendor's EULA and licensing requirements,
  since it does not share or include any code whatsoever with their SDK or any other
  licensed work.

## Why does this exist?

1. As a token of my gratitude for the VTubing community and all they do every day.

2. To open a path for other Rustaceans to build cool things in the VTubing space.

## How do I obtain this majestic tool?

Run the following Cargo command in your project directory (assuming you have [cargo-edit](https://github.com/killercup/cargo-edit) installed):

```fish
cargo add orphism
```

Or add the following line to your `Cargo.toml` (in the `[dependencies]` array):

```toml
orphism = "^ 0.2"
```

## How do I use it?

A minimal program might look something like this:

```rust
use orphism::Runtime;
use orphism::moc3::Model;
use std::path::PathBuf;

fn main() {
  let runtime = Runtime::new_from_runtime_path(PathBuf::from("./path/to/model/runtime")).expect("failed to load runtime directory");
  let moc3 = runtime.load_model().expect("failed to load model").data;
  let model = Model::read(moc3).expect("failed to parse model");
}
```

## Core Data Types

`orphism` primary revolves around two data types.

### Runtime

The `Runtime` struct in `orphism` represents the `runtime/` directory that
contains your runtime model assets. This is essentially the state on disk, and
mechanisms for loading from that state.

Most runtimes contain a single model, along with other files relating to that
model, and `orphism` will attempt to detect this. If there are multiple models
detected in a given runtime directory, it will return an error indicating the
issue, along with the path it was attempting to load from.

For these cases, there is a `Runtime::new_from_model_path` that allows you to
disambiguate by targeting a specific model file.

Once you have a `Runtime`, you can call `.load_model()` on it to load the `Model`.

### Model

The `Model` struct in `orphism` represents the contents of those files, once
loaded and parsed. This is essentially the state in memory, and mechanisms for
accessing and using the data.

## Project Crates

`orphism` is made up of several related crates. The top-level crate is this one,
and it re-exports all the other sub-crates (with the exception of `orphist`),
in addition to providing unified error handling.

If you don't need all of it, each of the supported data formats has a separate
crate, so you can load only what you need for whatever you want to do.

- [caff-archive](https://github.com/vtubing/caff-archive)
- [cdi3](https://github.com/vtubing/cdi3)
- [exp3](https://github.com/vtubing/exp3)
- [moc3](https://github.com/vtubing/moc3)
- [model3](https://github.com/vtubing/model3)
- [motionsync3](https://github.com/vtubing/motionsync3)
- [physics3](https://github.com/vtubing/physics3)
- [pose3](https://github.com/vtubing/pose3)
- [userdata3](https://github.com/vtubing/userdata3)

There's also a command-line interface that's mostly useful for testing and validation.

- [orphist](https://github.com/vtubing/orphist)

## How was this made?

- Carefully, without using or referencing any code or libraries from the format vendor.
- The [ImHex](https://github.com/WerWolv/ImHex) highlighting patterns from the [MOC3ingbird Exploit](https://github.com/OpenL2D/moc3ingbird) (CVE-2023-27566) were instrumental in understanding this format.
- The discovery process for undocumented JSON formats is described [here](https://gist.github.com/colstrom/44b30fdddc8b0a9bfb44b09972a68676).
- The discovery process for undocumented binary formats is described [here](https://gist.github.com/colstrom/f671d1583662de47b505a42a75b3a44b).

## License

`orphism` is available under the MIT License. See `LICENSE.txt` for the full text.

While the license is short, it's still written in fancy lawyer-speak. If you
prefer more down-to-earth language, consider the following:

- tl;drLegal has a simple visual summary available [here](https://www.tldrlegal.com/license/mit-license).
- FOSSA has a more in-depth overview available [here](https://fossa.com/blog/open-source-licenses-101-mit-license/).
