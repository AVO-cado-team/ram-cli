# RAM Emulator CLI

![Version 0.1.1](https://img.shields.io/badge/version-0.1.1-blue.svg)
[![License](https://img.shields.io/badge/license-GNU3-blue.svg)](./LICENSE)
[![ic-it](https://img.shields.io/badge/GitHub-ic--it-blue.svg)](https://github.com/ic-it)
[![Mykhailo Sichkaruk](https://img.shields.io/badge/GitHub-Mykhailo--Sichkaruk-blue.svg)](https://github.com/Mykhailo-Sichkaruk)

This CLI is a companion tool for the [RAM Emulator library](https://github.com/AVO-cado-team/ramemu), allowing you to easily check, compile, and run RAM assembly code from the command line.

## Features

- Check RAM assembly code for syntax errors
- Compile and run RAM assembly code
- Customizable input and output options

## Installation

### From Source

```bash
git clone https://github.com/Ddystopia/ram-cli.git
cd ram-cli
cargo install --path .
```

### For your Operating System
Install binary for your operating system from [releases](https://github.com/AVO-cado-team/ram-cli/releases/tag/v0.1.1).


## Usage

[![asciicast](https://asciinema.org/a/RTRODL0nZnaUeBMUDrLnJYi2W.svg)](https://asciinema.org/a/RTRODL0nZnaUeBMUDrLnJYi2W)

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
specify input and output options with `-i`/`--input` and `-o`/`--output` flags.

### Generate shell completion  

With `gen-completion` command, you can generate shell completion script for one of the following shells: `bash`, `zsh`, `fish`, `elvish` and `powershell`.

```bash
ram-cli gen-completion <SHELL>
```

After you generate the completion script, you should add it to your shell configuration file.
On windows powershell, you should run the generated **.ps1** file and reload the shell to enable shell completion.

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
