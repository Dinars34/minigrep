# minigrep

A command-line search tool built in Rust. Searches for a query string inside a file and prints every line that contains it. Built as part of learning [The Rust Programming Language](https://doc.rust-lang.org/book/), Chapter 12.

## Features

- Search for a string inside any text file
- Case-sensitive search by default
- Case-insensitive search via environment variable
- Clean error messages to stderr, results to stdout

## Usage

```bash
cargo run -- <query> <file_path>
```

**Example:**

```bash
cargo run -- safe poem.txt
```

Output:
```
safe, fast, productive.
```

## Case-Insensitive Search

Set the `IGNORE_CASE` environment variable to search regardless of letter case.

**Linux / macOS:**
```bash
IGNORE_CASE=1 cargo run -- rust poem.txt
```

**Windows (PowerShell):**
```powershell
$Env:IGNORE_CASE=1; cargo run -- rust poem.txt
```

## Project Structure

```
minigrep/
├── src/
│   ├── main.rs   # Entry point, argument parsing, config
│   └── lib.rs    # Search logic and tests
├── Cargo.toml
└── README.md
```

## How It Works

1. Reads command-line arguments (query + file path)
2. Reads the file content
3. Iterates line by line and collects matching lines
4. Prints results to stdout, errors to stderr

## Running Tests

```bash
cargo test
```

## What I Learned

- Separating binary and library crates (`main.rs` + `lib.rs`)
- Returning `Result<T, E>` for recoverable error handling
- Using `Box<dyn Error>` as a flexible error type
- Reading environment variables with `std::env::var`
- Writing unit tests for core logic
- Lifetime annotations on functions returning string slices

## Built With

- Rust (TRPL 3rd Edition, Chapter 12)
