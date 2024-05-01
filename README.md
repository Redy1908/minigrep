# A (lot) simple implementation of the *grep* command in Rust

Returns the lines that contain the specified *pattern*.

## Build

### Prerequisites

Install Rust and Cargo

Linux:

```sh
curl https://sh.rustup.rs -sSf | sh
```

For other OS check the [official documentation](https://www.rust-lang.org/tools/install)

```sh
cd minigrep
cargo build
```

The executable will be in `minigrep/target/debug`

## Usage

```sh
./minigrep {pattern} {file path}
```

## Example

Given a file `test.txt` with the following line: `Hello, World!` we, for example, can search for the word: `Hello`: 

<img src="images/example.png" alt="example">

Minigrep by default will use case sensitive matching, you can enable case insensitive matching with the param `-i` (ignore case).

Minigrep can also display the line numbers, use the param `-n`.
