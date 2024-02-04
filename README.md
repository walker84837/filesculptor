# file-normalizer-rs

file-normalizer-rs is a simple tool designed to replace special characters in an
input file with their equivalent ASCII characters. Its primary purpose is to
ease text processing tasks by reducing inconsistencies caused by non-ASCII
symbols.

## Table of Contents

  - [Installation](#installation)
      - [Building From Source](#building-from-source)
  - [Configuration](#configuration)
  - [Examples](#examples)
  - [Contributing](#contributing)
  - [License](#license)

## Installation

The easiest way to obtain file-normalizer-rs is via pre-compiled packages
available on its GitHub releases page and choose the appropriate release archive
depending on your operating system (Windows or Linux).

### Building from source

To build from source you need [rustup](https://rustup.rs/). Follow these
instructions to build file-normalizer-rs from source:

1.  Clone the repository to your local machine:
    ``` console
    git clone https://github.com/walker84837/file-normalizer-rs.git
    cd file-normalizer-rs
    ```
2.  Run the following commands to build the binary:
    ``` console
    cargo build --release
    ```
3.  After the compilation, find the generated binary inside the `target/release`
    folder.

## Configuration

file-normalizer is configured by a JSON file. Consult the [documentation](docs/)
within the repo for reference.

## Examples

Normalize a file named `input.txt`:

``` console
file-normalizer-rs --input input.txt --output normalized_input.txt --config custom-config.json
```

## Contributing

We appreciate all kinds of contributions – bug reports, feature requests,
documentation updates, etc. Feel free to submit a PR or open an issue discussing
potential enhancements. Before contributing:

  - Follow the [code of conduct](CODE_OF_CONDUCT.md).
  - Keep a consistent coding style. To make sure your coding style remains the
    same, format your code with:
    ``` console
    $ rustfmt --edition 2021 path/to/source_code
    ```
  - Use Rust stable, rather than Rust nightly.
  - If you have to use an external library, please use lightweight ones (ie.
    `ureq` over `reqwest`, `async-std` over `tokio`)
  - Prefer using the standard library over reinventing the wheel.

Please stick to Rust's official style guidelines while submitting patches.

## License

file-normalizer-rs is distributed under the terms of [GNU GPLv3](LICENSE.md).
