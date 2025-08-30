# mini-grep

A simple, grep-like command-line tool written in Rust. It allows you to search for a specific string pattern within a text file.

## Features

*   Search for a text pattern in a specified file.
*   Case-sensitive search by default.
*   Case-insensitive search can be enabled via an environment variable.

## Usage

To use `mini-grep`, you need to provide two arguments: a search pattern and the path to the file you want to search in.

### Basic Search (Case-Sensitive)

To perform a case-sensitive search, run the program with the following command structure:

```sh
cargo run -- <pattern> <file_path>
```

**Example:**

```sh
cargo run -- "hello" poem.txt
```

This command will search for the exact word "hello" in the `poem.txt` file and print any lines that contain it.

### Case-Insensitive Search

You can perform a case-insensitive search by setting the `IGNORE_CASE` environment variable before running the command.

On Linux/macOS:
```sh
IGNORE_CASE=1 cargo run -- <pattern> <file_path>
```

On Windows (PowerShell):
```powershell
$env:IGNORE_CASE=1; cargo run -- <pattern> <file_path>
```

**Example:**

```sh
IGNORE_CASE=1 cargo run -- "rUsT" poem.txt
```

This command will search for "rUsT" (and "rust", "Rust", "RUST", etc.) in `poem.txt` and print all matching lines, regardless of their casing.
