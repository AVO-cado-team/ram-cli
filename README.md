# RAM Emulator CLI

![Version 0.1.0](https://img.shields.io/badge/version-0.1.0-blue.svg)
[![License](https://img.shields.io/badge/license-GNU3-blue.svg)]

This CLI is a companion tool for the RAM Emulator library, allowing you to
easily check, compile, and run RAM assembly code from the command line.

## Features

- Check RAM assembly code for syntax errors
- Compile and run RAM assembly code
- Customizable input and output options

## Installation

You can install the RAM Emulator CLI using Cargo:

```bash
git clone https://github.com/Ddystopia/ram-cli.git
cd ram-cli
cargo install --path .
```

## Usage

To use the RAM Emulator CLI, you can run the following commands: Help

```bash
ram-cli help
```

### Check

```bash
ram-cli check <FILE>
```

Checks the RAM assembly code in the specified file for syntax errors. Run

```bash
ram-cli run [OPTIONS] <FILE>
```

Compiles and runs the RAM assembly code in the specified file. You can also
specify input and output options with -i/--input and -o/--output flags.

### Examples

Check a RAM assembly code file:

```bash
ram-cli check example.ram
```

Run a RAM assembly code file with custom input and output files:

```bash
ram-cli run -i input.txt -o output.txt example.ram
```

## Contributing

Contributions to the project are welcome. You can report bugs, request features,
or submit pull requests. Before submitting a pull request, make sure your
changes are well-tested and documented. License

This project is licensed under the GNU General Public License v3.0. See the
LICENSE file for details.
