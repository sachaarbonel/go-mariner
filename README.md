
# GoMariner (WIP)

GoMariner is a simple and extensible linter for the Golang programming language. It is written in Rust and uses a Golang parser to analyze Golang code and identify potential issues based on a predefined set of rules.

## Features (Goals)

- Fast and memory-efficient linting, thanks to Rust's performance characteristics.
- Easy-to-extend architecture with a clear separation between linting rules and core components.
- Predefined rules for common Golang code problems (e.g., unused variables, shadowed variables, etc.).
- Simple command-line interface.

## Installing GoMariner

To build GoMariner from source, you will need the Rust toolchain installed. You can download Rust from the [official website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone the GoMariner repository and build the project using `cargo`.

```sh
$ git clone https://github.com/yourusername/GoMariner.git
$ cd GoMariner
$ cargo build --release
```

The compiled binary will be placed in the `target/release` directory.

## Usage

Using GoMariner is simple. Just run the binary with the `-f` or `--file` flag followed by the path to the Golang file you want to lint.

```sh
$ ./target/release/gomariner -f path/to/your/file.go
```

GoMariner will then analyze the Golang file and report any issues it finds.

## Contributing

We welcome contributions to GoMariner! Please open an issue or submit a pull request on the GitHub repository with any bug fixes, improvements, or new rule suggestions.

## License

GoMariner is released under the MIT license


## TODOs:

- [x] Unused variables: Detect and report variables that are declared but never used in the code.
- [ ] Shadowing: Detect and report variable shadowing, where a variable declared in an inner scope has the same name as a variable in an outer scope, which can lead to hard-to-find bugs.
- [ ] Formatting: Enforce a consistent code style and formatting rules as specified in the Go language specification and Effective Go guidelines.
- [ ] Error handling: Ensure that error values returned by functions are always checked, and recommend proper error handling techniques.
- [ ] Dead code: Identify and report unused functions, types, constants, and other dead code that does not contribute to the program's functionality.

What else?