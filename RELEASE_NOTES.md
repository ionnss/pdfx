# Release Notes

## v0.1.0 - Initial Release 🚀  
*Released: January 2025*

### 🎉 **First Official Release**

We're excited to introduce **pdfx** - a lightning-fast terminal-native PDF indexing and search toolkit built with Rust!

### ✨ **What's New**

#### **🔧 Build System Improvements**
- **🪟 Windows Build Fix**: Added bundled SQLite feature to resolve Windows linking issues
- **🚀 Automated Releases**: GitHub Actions now automatically builds binaries for all platforms
- **📦 Four Platform Support**: Linux x86_64, Windows x86_64, macOS Intel, macOS Apple Silicon
- **🔄 GitHub Actions v4 Compatibility**: Updated to `actions/upload-artifact@v4` and `actions/download-artifact@v4`
- **🚫 Artifact Conflicts**: Resolved v4 immutability issues with matrix build strategy  
- **🔒 Reproducible Builds**: Updated `Cargo.lock` for consistent dependency versions
- **🔐 Release Permissions**: Granted `contents: write` to GitHub Token for release creation

#### **🧹 Repository Cleanup**
- **📝 Code Formatting**: Applied `rustfmt` standards across entire codebase
- **🔍 Linter Compliance**: Fixed all `clippy` warnings for better code quality
- **🗂️ Module Organization**: Resolved module inception issues (`database.rs` → `db.rs`, `indexer.rs` → `scanner.rs`)
- **📋 Enhanced `.gitignore`**: Prevents accidental commit of binaries and temporary files

#### **⚙️ CI/CD Pipeline**
- **✅ Continuous Integration**: Automated testing, formatting, and linting on every commit
- **🔐 Security Auditing**: Daily dependency vulnerability scanning
- **📊 Build Status**: Real-time build status badges
- **🏗️ Multi-Platform Builds**: Simultaneous building across Linux, Windows, and macOS

#### **📦 Core Features** *(Stable)*
- **🔍 Smart PDF Indexing**: SQLite-powered database with metadata extraction
- **⚡ Lightning Fast**: Two-phase scanning with accurate progress tracking  
- **🎨 Beautiful Progress Bars**: Custom Braille character indicators (`⣿⣷⣯⣟⡿⢿⠿⠟⠛⠋`)
- **📊 Zero Duplicates**: Intelligent `INSERT OR REPLACE` prevents duplicate entries
- **🛡️ System-Aware**: Gracefully skips protected directories (Photos, System, etc.)
- **🧹 Clean Uninstall**: Complete data removal with `pdfx cleanup`
- **🔒 Security Fixed**: Removed unused `rust-bert` dependency to resolve `protobuf` vulnerability

### **🖥️ Commands Available**
```bash
pdfx init [path]        # Index PDFs in specified directory (one-time setup)
pdfx search "query"     # Search indexed PDFs (coming in v0.2.0)
pdfx list [-a]          # List indexed PDFs (coming in v0.2.0)  
pdfx recent [-l N]      # Show recent PDFs (coming in v0.2.0)
pdfx cleanup            # Complete data cleanup and uninstall
```

#### **Database & Storage**
- **Local SQLite Database**: Stored in OS-appropriate data directories
- **Complete Privacy**: No cloud storage, no tracking, no data sharing
- **Clean Uninstall**: `pdfx cleanup` removes all traces

### 🚀 **Getting Started**

1. **Install**:
   ```bash
   cargo install --git https://github.com/ionnss/pdfx
   ```

2. **Index your PDFs**:
   ```bash
   pdfx init ~/Documents
   ```

3. **Enjoy lightning-fast PDF management!**

### 📋 **System Requirements**
- **Operating System**: Linux, macOS, or Windows  
- **Architecture**: x86_64 (Intel/AMD64) or ARM64 (Apple Silicon)
- **Terminal**: Modern terminal with Unicode/UTF-8 support
- **Storage**: ~50MB for installation, variable for database (depends on PDF count)

---

## 🗺️ **Roadmap**

### **v0.2.0 - Search & Discovery** *(Next Release)*
- **🔍 Search Implementation**: Full-text search through indexed PDFs
- **📋 List Command**: Display and filter indexed PDFs with sorting options
- **📅 Recent Command**: Show recently modified PDFs with timestamps
- **🎯 Enhanced Filtering**: Search by file size, modification date, path patterns
- **📊 Statistics**: Show indexing statistics and storage usage

### **v0.3.0 - Content Intelligence** *(Future)*
- **📄 PDF Content Extraction**: Index text content for full-text search
- **🔍 Advanced Search**: Search inside PDF content, not just filenames  
- **🏷️ Auto-Tagging**: Automatic categorization based on content
- **📈 Analytics Dashboard**: Visual statistics and insights

### **v0.4.0 - AI Integration** *(Vision)*
- **🤖 AI Summaries**: Automatic PDF content summarization
- **❓ Question Generation**: Study questions from PDF content
- **🎯 Key Point Extraction**: Highlight important information
- **🔗 Smart Linking**: Connect related PDFs automatically

---

## 📋 **System Requirements**
- **Operating System**: Linux, macOS, or Windows  
- **Architecture**: x86_64 (Intel/AMD64) or ARM64 (Apple Silicon)
- **Terminal**: Modern terminal with Unicode/UTF-8 support
- **Storage**: ~50MB for installation, variable for database (depends on PDF count)

---

## 🙏 **Acknowledgments**
Built with excellence using:
- **[Rust](https://rust-lang.org)** 🦀 - Systems programming language
- **[rusqlite](https://crates.io/crates/rusqlite)** - SQLite database operations
- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing
- **[indicatif](https://crates.io/crates/indicatif)** - Progress bars and spinners
- **[walkdir](https://crates.io/crates/walkdir)** - Recursive directory traversal
- **[chrono](https://crates.io/crates/chrono)** - Date and time handling
- **[dirs](https://crates.io/crates/dirs)** - Cross-platform directory paths

---

## 🐛 **Known Issues**
- **Command Placeholders**: Search, List, and Recent commands show "not implemented" messages
- **System Permissions**: Some directories may show permission warnings (this is normal)
- **Large Directories**: Very large directory trees (>100k files) may take longer to scan
- **Unicode Filenames**: Some special Unicode characters in filenames may display incorrectly

---

## 🔗 **Links**
- **📦 Download**: [GitHub Releases](https://github.com/ionnss/pdfx/releases)
- **💻 Source Code**: [GitHub Repository](https://github.com/ionnss/pdfx)  
- **🐛 Bug Reports**: [GitHub Issues](https://github.com/ionnss/pdfx/issues)
- **📚 Documentation**: [README.md](https://github.com/ionnss/pdfx/blob/main/README.md)


