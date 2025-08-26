# pdfx

<div align="center">
  <img src="assets/bg_rc_logo.png" alt="pdfx logo" width="500"/>
  <br><br>
  
  **🛠️ A lightning-fast terminal-native PDF indexing and search toolkit**
  
  [![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![GitHub release](https://img.shields.io/github/release/ionnss/pdfx.svg)](https://github.com/ionnss/pdfx/releases)
</div>

---

## ✨ Features

- **🚀 Lightning Fast**: Index PDFs once, search instantly with beautiful Braille progress bars
- **🎯 Smart Indexing**: Build a searchable database of all your PDF files
- **📊 No Duplicates**: Intelligent duplicate prevention - re-indexing updates existing entries
- **🛡️ Robust**: Gracefully handles permission errors and system directories
- **🧹 Clean Uninstall**: Complete cleanup with `pdfx cleanup` command
- **💻 Cross-Platform**: Works on Linux, macOS, and Windows
- **🎨 Beautiful Interface**: Docker-style progress bars with modern terminal UI

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

### Basic Commands

```bash
# Initialize PDF index (one-time setup)
pdfx init                    # Index current directory
pdfx init ~/Documents        # Index specific directory
pdfx init ~                  # Index entire home directory

# Search indexed PDFs
pdfx search "machine learning"   # Search for keyword

# List indexed PDFs
pdfx list                    # Show all indexed PDFs
pdfx list -a                 # Show all with details

# Show recent PDFs
pdfx recent                  # Show 10 most recent
pdfx recent -l 20            # Show 20 most recent

# Clean up
pdfx cleanup                 # Remove all indexed data
```

### Workflow

```bash
# 1. First time setup - index your PDFs
pdfx init ~/Documents
# ⠋ Counting files... 10847
# 🔍 Scanning for PDFs... [00:00:03] [⣿⣿⣿⣿⣿⣷⣯⣟⡿⢿⠿⠟⠛⠋    ] 1247/2500 files | 400/s | ETA: 00:03
# ✅ Scan complete! 170 PDFs found | 2500 files processed | 25 directories skipped
# Indexed 170 PDFs in /Users/user/Library/Application Support/pdfx/db.sqlite

# 2. Search your indexed PDFs instantly
pdfx search "rust programming"

# 3. List recent PDFs
pdfx recent -l 5

# 4. When you're done (optional cleanup)
pdfx cleanup
```

### Sample Output

```
⠋ Counting files... 10847
🔍 Scanning for PDFs... [00:00:03] [⣿⣿⣿⣿⣿⣷⣯⣟⡿⢿⠿⠟⠛⠋    ] 1247/2500 files | 400/s | ETA: 00:03
✅ Scan complete! 170 PDFs found | 2500 files processed | 25 directories skipped

Indexed 170 PDFs in /Users/user/Library/Application Support/pdfx/db.sqlite
```

---

## 🗄️ Database & Storage

### **Where Your Data Lives**
```bash
# macOS
~/Library/Application Support/pdfx/db.sqlite

# Linux  
~/.local/share/pdfx/db.sqlite

# Windows
%APPDATA%/pdfx/db.sqlite
```

### **Database Schema**
Your PDFs are stored with:
- **Path & Filename** - Full file location and name
- **Size & Modified Date** - File metadata for change detection
- **Indexed Timestamp** - When the PDF was added to database
- **Unique Path Constraint** - Prevents duplicate entries

### **Privacy & Security**
- **Local Storage Only** - No cloud, no tracking, no data sharing
- **SQLite Database** - Industry-standard, portable format
- **Complete Cleanup** - `pdfx cleanup` removes all traces

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

## 🔧 Troubleshooting

### **Common Issues**

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

**Q: Progress bar not showing or looks broken**
```bash
# Ensure your terminal supports Unicode/UTF-8
# Try a modern terminal like: iTerm2, Terminal.app, or Windows Terminal
```

**Q: Where is my data stored?**
```bash
# View database location after running pdfx init
# Path is shown in success message
# Use `pdfx cleanup` to remove all data
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
- Database powered by [rusqlite](https://crates.io/crates/rusqlite) for fast SQLite operations
- CLI interface built with [clap](https://crates.io/crates/clap) for beautiful argument parsing
- Progress bars powered by [indicatif](https://crates.io/crates/indicatif) with custom Braille characters
- Directory traversal using [walkdir](https://crates.io/crates/walkdir) for efficient file scanning
- Date/time handling with [chrono](https://crates.io/crates/chrono)

---

## 🗺️ Roadmap

### **✅ Completed (v0.1.0)**
- [x] **PDF Indexing** - SQLite-based PDF database with metadata
- [x] **Beautiful Progress Bars** - Braille character progress indicators  
- [x] **Duplicate Prevention** - Smart re-indexing without duplicates
- [x] **Clean Uninstall** - Complete data cleanup with `pdfx cleanup`
- [x] **Cross-Platform Support** - Works on Linux, macOS, and Windows

### **🚧 In Progress (v0.2.0)**
- [ ] **Smart Search** - Full-text search inside indexed PDFs  
- [ ] **List Command** - Display indexed PDFs with filtering
- [ ] **Recent Command** - Show recently modified PDFs

### **🔮 Future (v0.3.0+)**
- [ ] **Interactive Dashboard** - Terminal UI for PDF analytics
- [ ] **Duplicate Detection** - Find duplicate PDFs by content hash
- [ ] **Export Options** - JSON/CSV output formats
- [ ] **AI Integration** - PDF summarization and analysis
- [ ] **File Watching** - Auto-update index when PDFs change

---

<div align="center">
  <strong>Made with ❤️ and Rust</strong>
  <br>
  <a href="https://github.com/ionnss/pdfx">⭐ Star this project if you find it useful!</a>
</div>