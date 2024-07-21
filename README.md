# tsutil

tsutil is a syntax tree printer Rust utility that reads a source code file, guesses the language based on the file extension, and prints the syntax tree using Tree-sitter.

## Features

- Supports Rust and Python source files.
- Prints detailed syntax tree nodes.
- Uses Tree-sitter for parsing.

## Requirements

- Rust (latest stable version)
- Cargo

## Usage

1. Build the project:
   ```sh
   cargo build
   ```

2. Run the program with a file path as an argument:
   ```sh
   cargo run <file-path>
   ```

## Example

To print the syntax tree of a Rust source file:

```sh
cargo run src/main.rs
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Acknowledgements

- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for the parsing library.
- [Pretty_env_logger](https://crates.io/crates/pretty_env_logger) for the logging setup.
```
