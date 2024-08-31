# filesculptor
> A tool for modifying text files by replacing special characters with their ASCII equivalents.

`filesculptor` is a command-line tool designed to help users normalize text files by converting special characters to their ASCII counterparts. It is particularly useful for preparing text files for systems that require or perform better with ASCII characters.

## Table of Contents

1.  [Installing / Getting started](#installing--getting-started)
2.  [Initial Configuration](#initial-configuration)
3.  [Usage](#usage)
4.  [Developing](#developing)
5.  [Building](#building)
6.  [Deploying / Publishing](#deploying--publishing)
7.  [Features](#features)
8.  [Configuration](#configuration)
9.  [Contributing](#contributing)
10. [Links](#links)
11. [Licensing](#licensing)

## Installing / Getting started

To build this project from source, ensure you have [Rust](https://www.rust-lang.org/) installed on your system. Follow these steps to set up `filesculptor`:

1. Install Rustup if you haven't done so already by visiting: https://rustup.rs/
2. Add the Rust package manager (Cargo) to your system:
   ``` console
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Update your Rust toolchain to the latest version:
   ``` console
   $ rustup update
   ```
4. Navigate to the project directory and build the program:
   ``` console
   $ cargo build --release
   ```
5. Move the generated binary (located at `target/release/filesculptor`) to your desired location and execute it.

Alternatively, you can download precompiled binaries from the [Releases page](https://github.com/walker84837/filesculptor-rs/releases).

### Initial Configuration

There is no initial configuration required for FileSculptor beyond having Rust installed and building the project from source.

## Usage

Use the built program to normalize text files:

``` console
$ ./filesculptor path/to/input.txt -o path/to/output.txt -c path/to/config.json
```

Replace `path/to/*` with the actual paths to your input file, output file, and configuration file respectively. The tool will read the input file, apply transformations based on the configuration file, and output the normalized text to the specified output file.

## Developing

To start contributing to FileSculptor, clone the repository and set up your environment:

``` console
$ git clone https://github.com/walker84837/filesculptor-rs.git
$ cd filesculptor/
```

Ensure your code changes are formatted with `rustfmt` and that you're using the Rust stable toolchain.

### Building

If you make changes to the code, you will need to rebuild the project:

``` console
$ cargo build --release
```

This command compiles the code and generates a binary in the `target/release/` directory.

## Features

`filesculptor` provides several key features:
* Normalizes text files by replacing special characters with ASCII equivalents.
* Supports custom configurations via a JSON configuration file.
* Fast and lightweight, built with Rust for performance.

## Configuration

## Contributing

We appreciate contributions! If you'd like to contribute, please fork the repository and use a feature branch. Pull requests are warmly welcome.

## Links

- Repository: <https://github.com/walker84837/filesculptor-rs>
- Issue tracker: <https://github.com/walker84837/filesculptor-rs/issues>
  - In case of sensitive bugs like security vulnerabilities, please contact me directly instead of using the issue tracker.

## License

The code in this project is licensed under the [GPL v3.0](LICENSE.md).
