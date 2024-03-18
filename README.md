# transmutext-rs

This program takes an input file containing text with special characters and
replaces them with their corresponding ASCII equivalents.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)

## Installation

To build this project from source, you'll need to install
[rustup](https://rustup.rs/), if you don't have it already. Otherwise, you can
download the precompiled binaries, from the Releases page.

Once Rust and Cargo are installed, you can build and install this program using
the following command:

``` console
$ cargo build --release
```

If you'd like to, you can move the build executable
(target/release/transmutext-rs) to wherever you'd like and run it.

## Usage

After building from source, you can use this program to normalize text files.

``` console
$ ./transmutext-rs --input path/to/input.txt --output path/to/output.txt --config config_file.json
```

For example:

``` console
$ ./transmutext-rs -i input.txt -o output.txt -c normalize.json
```

## Contributing

Contributions are always welcome! If you'd like to contribute, please:

  - Follow the [code of conduct](CODE_OF_CONDUCT.md).
  - Keep a consistent coding style. To make sure your coding style remains the
    same, format your code with:
    ``` console
    $ rustfmt --edition 2021 path/to/source_code
    ```
  - Use Rust stable, rather than Rust nightly. If you notice my code contains
    code from Rust nightly, feel free to change it to "stable" code.
  - If you have to use an external library, please use lightweight ones (eg.
    `ureq` over `reqwest`, `async-std` over `tokio`)
  - Prefer using the standard library over reinventing the wheel.
  - For proposing big changes, open an issue and describe:
      - Why should the changes be implemented?
      - What's the difference between using it and not using it?

## License

This project is licensed under the [GNU General Public License,
version 3](LICENSE.md).
