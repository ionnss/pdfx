# pdfx

<div align="center">
  <img src="img/bg_rc_logo.png" alt="pdfx logo" width="300"/>
  <br><br>
  
  **🔍 A lightning-fast terminal-native PDF finder and analyzer**
  
  [![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![GitHub release](https://img.shields.io/github/release/ionnss/pdfx.svg)](https://github.com/ionnss/pdfx/releases)
</div>

---

## ✨ Features

- **🚀 Blazingly Fast**: Recursively scan directories for PDF files with Docker-style progress bars
- **🎯 Smart Search**: Find PDFs in current directory or any specified path
- **📊 Analytics**: Get detailed statistics about your PDF collection
- **🛡️ Safe**: Gracefully handles permission errors and continues searching
- **💻 Cross-Platform**: Works on Linux, macOS, and Windows
- **🎨 Beautiful Output**: Clean, modern terminal interface

---

## 📦 Installation

### Method 1: Install from GitHub (Recommended)

Make sure you have [Rust](https://rustup.rs/) installed, then run:

```bash
cargo install --git https://github.com/ionnss/pdfx
```

### Method 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/ionnss/pdfx.git
cd pdfx

# Build and install
cargo install --path .
```

### Method 3: Download Binary (Coming Soon)

Pre-built binaries for major platforms will be available in the [Releases](https://github.com/ionnss/pdfx/releases) section.

---

## 🚀 Usage

### Basic Usage

```bash
# Search for PDFs in current directory
pdfx

# Search for PDFs in a specific directory
pdfx /path/to/directory

# Search in your home directory
pdfx ~

# Search in Downloads folder
pdfx ~/Downloads
```

### Examples

```bash
# Find all PDFs in your Documents folder
pdfx ~/Documents

# Search your entire home directory (be patient!)
pdfx /Users/yourusername

# Search a project directory
pdfx /Users/yourusername/Projects/research
```

### Sample Output

```
Searching for PDF files in: "/Users/ions/Downloads"

Counting files...

⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣯    1247/1324 files (94%) Found 23 PDFs

Found 23 PDF files:

"/Users/user/Downloads/TheRustProgrammingLanguage.pdf"
"/Users/user/Downloads/research_paper.pdf"
"/Users/user/Downloads/presentation.pdf"
...
```

---

## 🛠️ Requirements

- **Rust**: 1.70 or later
- **Operating System**: Linux, macOS, or Windows
- **Terminal**: Any modern terminal with Unicode support

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup

```bash
git clone https://github.com/ionnss/pdfx.git
cd pdfx
cargo build
cargo run
```

---

## 🐛 Issues & Bug Reports

Found a bug? Have a feature request? Please check the [Issues](https://github.com/ionnss/pdfx/issues) page first, then feel free to open a new issue with:

- **System information** (OS, Rust version)
- **Steps to reproduce** the issue
- **Expected vs actual behavior**

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Uses [walkdir](https://crates.io/crates/walkdir) for efficient directory traversal
- Progress bars powered by [indicatif](https://crates.io/crates/indicatif)

---

## 🗺️ Roadmap

- [ ] **Interactive Dashboard** - Terminal UI for PDF analytics
- [ ] **Smart Search** - Full-text search inside PDF content
- [ ] **Duplicate Detection** - Find duplicate PDFs by content hash
- [ ] **Export Options** - JSON/CSV output formats
- [ ] **AI Integration** - PDF summarization and analysis
- [ ] **Performance Optimizations** - Parallel processing and caching

---

<div align="center">
  <strong>Made with ❤️ and Rust</strong>
  <br>
  <a href="https://github.com/ionnss/pdfx">⭐ Star this project if you find it useful!</a>
</div>