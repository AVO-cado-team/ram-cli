# ram-cli
> A command line tool for ramemu

## Installation
```bash
git clone git@github.com:AVO-cado-team/ram-cli.git
cd ram-cli
cargo install --path .
```

## Usage
### Help
Prints help information
```bash
ram-cli --help
```

### Version
Prints the version information
```bash
ram-cli --version
```

### Check
Validates a RAM file
```bash
ram-cli check <file>
```

### Run
Runs a RAM file
#### Options
- `--input` - The input file
- `--output` - The output file

```bash
# Run a file
ram-cli run <file>

# Run a file with a custom input and output
ram-cli run --input <input> --output <output> <file>
```

## Compile
Compiles a RAM file
#### Options
- `--turingmachine` - Compile RAM to Turing Machine
```bash
# Compile a file
ram-cli compile <file>

# Compile a file to Turing Machine
ram-cli compile --turingmachine <file>
```

### Debug
Start 