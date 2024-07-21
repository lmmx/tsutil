# tsutil

tsutil is a syntax tree printer Rust utility that reads a source code file, guesses the language based on the file extension, and prints the syntax tree using Tree-sitter.

## Features

- Supports Rust and Python source files.
- Prints detailed syntax tree nodes.
- Uses Tree-sitter for parsing.
- Syntax highlighting for parse trees.
- Lists all node IDs and their corresponding names for a language.

## Requirements

- Rust (latest stable version)
- Cargo

## Usage

1. Build the project:
   ```sh
   cargo build
   ```

2. Run the program with a file path as an argument to print the syntax tree:
   ```sh
   cargo run <file-path>
   ```

3. Run the program with the `--highlight` flag to print the syntax tree with syntax highlighting:
   ```sh
   cargo run <file-path> --highlight
   ```

4. Run the program with the `--list-ids` flag to list all node IDs and their corresponding names for the language:
   ```sh
   cargo run <file-path> --list-ids
   ```

## Example

To print the syntax tree of a Rust source file:

```sh
cargo run src/main.rs
```

To print the syntax tree with syntax highlighting:

```sh
cargo run src/main.rs --highlight
```

To list all node IDs and their corresponding names for a Rust source file:

```sh
cargo run src/main.rs --list-ids
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Acknowledgements

- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for the parsing library.
- [Pretty_env_logger](https://crates.io/crates/pretty_env_logger) for the logging setup.
- [Colored](https://crates.io/crates/colored) for terminal colorization.
