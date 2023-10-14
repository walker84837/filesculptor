# file-normalizer-rs

This program takes an input file containing text with special characters and replaces them with their corresponding ASCII equivalents.

**Disclaimer**: Not all characters are supported, so please don't expect this program to work as you might expect.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)
5.  [Acknowledgments](#acknowledgments)
6.  [Contact](#contact)

## Installation

To use this program, you'll need to install Rust and its package manager, Cargo. Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install) to get them set up.

Once Rust and Cargo are installed, you can build and install this program using the following command:

``` bash
cargo install --path .
```

## Usage

After installing, you can use this program to normalize text files. Here's how to use it:

``` bash
./file-normalizer-rs <input_file_path> <output_file_path>
```

Replace `<input_file_path>` with the path to the input text file that you want to normalize and `<output_file_path>` with the path where you want to save the normalized text.

Example:

``` bash
./file-normalizer-rs input.txt output.txt
```

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix: `git checkout -b feature/new-feature`.
3.  Make your changes and commit them.
4.  Push your changes to your fork: `git push origin feature/new-feature`.
5.  Create a pull request to the `main` branch of the original repository.

## License

This project is licensed under the GNU GPLv3 License. See the [LICENSE](LICENSE.md) file for details.

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this project:

  - [StructOpt](https://crates.io/crates/structopt) - for command-line argument parsing in Rust.

## Contact

If you have any questions or need further assistance, you can contact me at <walker84837@gmail.com>.

