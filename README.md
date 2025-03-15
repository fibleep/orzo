# Orzo 

A simple Rust tool that copies file contents from a directory to your clipboard.

## Installation

```
cargo install orzo
```

## Usage

```
orzo /path/to/folder
```

This will:
- Copy all text file contents to your clipboard
- Skip binary files (images, PDFs, etc.) and only copy their paths
- Format everything with clear separators

## Format

```
=========== file/path.txt =============
File contents here...
============================

=========== other/file.md =============
More content here...
============================
```

## License

MIT
