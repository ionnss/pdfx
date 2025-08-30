# pdfx

<div align="center">
  <img src="assets/bg_rc_logo.png" alt="pdfx logo" width="300"/>
  <br><br>
  
  **A lightning-fast terminal-native PDF indexing and search toolkit**
  
  [![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![GitHub release](https://img.shields.io/github/release/ionnss/pdfx.svg)](https://github.com/ionnss/pdfx/releases)
  [![Release](https://github.com/ionnss/pdfx/actions/workflows/release.yml/badge.svg)](https://github.com/ionnss/pdfx/actions/workflows/release.yml)
  
  [![Build Status](https://img.shields.io/github/actions/workflow/status/ionnss/pdfx/ci.yml?branch=master)](https://github.com/ionnss/pdfx/actions)
  [![Crates.io](https://img.shields.io/crates/v/pdfx)](https://crates.io/crates/pdfx)
  [![Downloads](https://img.shields.io/crates/d/pdfx)](https://crates.io/crates/pdfx)
  [![Contributors](https://img.shields.io/github/contributors/ionnss/pdfx)](https://github.com/ionnss/pdfx/graphs/contributors)

  [![GitHub Stars](https://img.shields.io/github/stars/ionnss/pdfx?style=social)](https://github.com/ionnss/pdfx/stargazers)
  [![GitHub Issues](https://img.shields.io/github/issues/ionnss/pdfx)](https://github.com/ionnss/pdfx/issues)

  
</div>

---

## Features

- **Fast PDF Indexing**: SQLite-powered database with metadata extraction
- **Lightning Search**: Instant filename-based search across indexed PDFs
- **List & Browse**: View all indexed PDFs with detailed information
- **Export Data**: Export your PDF library to JSON, CSV, Markdown, PDF, YAML, and HTML
- **Cross-Platform**: Native support for Linux, macOS, and Windows
- **Clean UI**: Beautiful progress bars and organized output
- **Zero Dependencies**: No external system requirements
- **Smart Cleanup**: Complete data removal with `pdfx cleanup`

---

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/ionnss/pdfx.git
cd pdfx

# Build and install
cargo install --path .
```

### From GitHub

```bash
cargo install --git https://github.com/ionnss/pdfx
```

---

## Usage

### Basic Commands

```bash
# Initialize PDF index
pdfx init                    # Index current directory
pdfx init ~/Documents        # Index specific directory
pdfx init ~                  # Index entire home directory

# Search indexed PDFs
pdfx search "machine learning"   # Search for keyword in filenames

# List all indexed PDFs
pdfx list                    # Show all PDFs with details

# Export your PDF library
pdfx export                  # Export all formats to Downloads folder
pdfx export --format json    # Export only JSON format
pdfx export --format csv,yaml # Export multiple formats

# Clean up
pdfx cleanup                 # Remove all indexed data
```

### Workflow Example

```bash
# 1. First time setup - index your PDFs
pdfx init ~/Documents
# ✅ Scan complete! 170 PDFs found | 2500 files processed | 25 directories skipped
# Indexed 170 PDFs in /Users/user/Library/Application Support/pdfx/db.sqlite

# 2. Browse your PDF library
pdfx list
# 📋 All Indexed PDFs
# 📊 Total: 170 PDFs
# 📄 1. The Rust Programming Language.pdf
#     Size: 14.37 MB
#     Path: /Users/user/Documents/books/rust.pdf
#     Modified: 2025-01-15 10:30:00

# 3. Search your indexed PDFs instantly
pdfx search "rust programming"

# 4. Export your library for sharing or backup
pdfx export
# Exporting 170 PDFs to /Users/user/Downloads/pdfx_exports
#   ✅ Generated pdfs.json
#   ✅ Generated pdfs.csv
#   ✅ Generated pdfs.md
#   ✅ Generated pdfs.yaml
#   ✅ Generated pdfs.html
# 🎉 Export complete!

# 5. When you're done (optional cleanup)
pdfx cleanup
```

---

## Export Formats

pdfx supports multiple export formats for your PDF library:

### Available Formats
- **JSON**: Machine-readable format with full metadata
- **CSV**: Spreadsheet-compatible format for data analysis
- **Markdown**: Human-readable format with tables
- **YAML**: Structured format for configuration files
- **HTML**: Web-ready format for sharing online

### Export Examples
```bash
# Export all formats to Downloads folder
pdfx export

# Export specific formats
pdfx export --format json
pdfx export --format csv,yaml
pdfx export --format html
```

### Export Location
- **Default**: `~/Downloads/pdfx_exports/` (or equivalent on your OS)
- **Files**: `pdfs.json`, `pdfs.csv`, `pdfs.md`, `pdfs.yaml`, `pdfs.html`

---

## Database & Storage

### Where Your Data Lives
```bash
# macOS
~/Library/Application Support/pdfx/db.sqlite

# Linux  
~/.local/share/pdfx/db.sqlite

# Windows
%APPDATA%/pdfx/db.sqlite
```

### Privacy & Security
- **Local Storage Only**: No cloud, no tracking, no data sharing
- **SQLite Database**: Industry-standard, portable format
- **Complete Cleanup**: `pdfx cleanup` removes all traces

---

## Requirements

- **Rust**: 1.70 or later
- **Operating System**: Linux, macOS, or Windows
- **Terminal**: Any modern terminal with Unicode support

---

## Development

### Setup

```bash
git clone https://github.com/ionnss/pdfx.git
cd pdfx
cargo build
cargo run -- --help
```

### Project Structure

```
src/
├── cli/          # Command-line interface
├── database/     # SQLite database operations
├── indexer/      # PDF file discovery and indexing
├── helpers/      # Utility functions
└── types.rs      # Core data structures
```

---

## Contributing

We welcome contributions! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

---

## Troubleshooting

### Common Issues

**Q: "Permission denied" errors during scanning**
```bash
# This is normal on macOS/Linux - system directories are protected
# pdfx will skip these and continue scanning accessible directories
```

**Q: Database seems corrupted or giving errors**
```bash
pdfx cleanup    # Remove database and start fresh
pdfx init       # Rebuild index
```

**Q: Where is my data stored?**
```bash
# View database location after running pdfx init
# Path is shown in success message
# Use `pdfx cleanup` to remove all data
```

---

## Roadmap

### Current Status (v0.2.0)
- ✅ **PDF Indexing**: SQLite-based PDF database with metadata
- ✅ **Filename Search**: Fast, case-insensitive filename search
- ✅ **List Command**: Display all indexed PDFs with detailed information
- ✅ **Export Data**: Export to JSON, CSV, Markdown, YAML, and HTML formats
- ✅ **Cross-Platform**: Works on Linux, macOS, and Windows
- ✅ **Clean UI**: Progress bars and organized output

### Planned Features
- 📅 **Recent Command**: Show recently modified PDFs
- 🔍 **Advanced Search**: Filter by size, date, path
- 📊 **Statistics**: Show indexing statistics and storage usage
- 🏷️ **Tagging System**: Categorize and tag PDFs for better organization

See [FUTURE.md](FUTURE.md) for detailed roadmap and feature plans.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

Built with excellence using:
- **[Rust](https://rust-lang.org)** - Systems programming language
- **[rusqlite](https://crates.io/crates/rusqlite)** - SQLite database operations
- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing
- **[indicatif](https://crates.io/crates/indicatif)** - Progress bars and spinners
- **[walkdir](https://crates.io/crates/walkdir)** - Recursive directory traversal
- **[chrono](https://crates.io/crates/chrono)** - Date and time handling