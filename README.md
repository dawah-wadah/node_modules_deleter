# Node Modules Cleaner

This is a simple command-line utility written in Rust to help you reclaim disk space by deleting `node_modules` directories in your project's directory tree. These directories are typically created when working with Node.js projects and can occupy a significant amount of disk space.

## Prerequisites

Before you can use this utility, make sure you have the following installed:

- Rust: You can download and install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

Clone this repository to your local machine:

```bash
git clone https://github.com/dawah-wadah/node_modules_deleter.git
cd node_modules_deleter
```

Build the utility using `cargo`:

```bash
cargo build --release
```

## Usage

To reclaim disk space by deleting `node_modules` directories, use the following command:

```bash
./target/release/node_modules_deleter <root_directory_path> [--dry-run]
```

- `<root_directory_path>` is the path to the directory where you want to start searching for `node_modules` directories.
- `--dry-run` is an optional flag. When used, it will show you the directories that would be deleted without actually deleting them. This is useful for previewing the changes before committing.

## Example

Here's an example of how to use the utility:

```bash
./target/release/node_modules_deleter /path/to/your/project --dry-run
```

This command will scan the specified project directory and its subdirectories for `node_modules` directories and display a summary of the space that can be reclaimed.

## License

This utility is open-source and released under the MIT License. You can find the full license text in the `LICENSE` file.

## Disclaimer

Use this utility with caution, and always make sure to back up your data before running it. Deleting `node_modules` directories will remove dependencies, and your project may not work correctly without them.
