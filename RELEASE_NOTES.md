# Release Notes

## v0.1.0 - Initial Release 🚀
*Released: January 2025*

### 🎉 **First Official Release**

We're excited to introduce **pdfx** - a lightning-fast terminal-native PDF indexing and search toolkit built with Rust!

### ✨ **What's New**

#### **Core Features**
- **🔍 Smart PDF Indexing**: Build a SQLite database of all your PDF files with metadata
- **⚡ Lightning Fast**: Two-phase scanning with real-time progress tracking
- **🎨 Beautiful Progress Bars**: Custom Braille character progress indicators (`⣿⣷⣯⣟⡿⢿⠿⠟⠛⠋`)
- **📊 Zero Duplicates**: Intelligent duplicate prevention with `INSERT OR REPLACE`
- **🛡️ System-Aware**: Gracefully handles protected directories and permission errors
- **💻 Cross-Platform**: Native support for Linux, macOS, and Windows

#### **Commands Available**
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

### 🛠️ **Technical Highlights**

- **Architecture**: Clean modular design with proper separation of concerns
- **Error Handling**: Robust error handling with graceful degradation
- **Performance**: Efficient two-phase scanning for accurate progress tracking
- **Memory Safe**: Built with Rust for memory safety and performance

### 📊 **Database Schema**
Your PDFs are indexed with:
- Full file path and filename
- File size and modification timestamps  
- Indexing timestamp for change detection
- Unique path constraints to prevent duplicates

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

### 🗺️ **What's Next (v0.2.0)**
- **Search Implementation**: Full-text search through indexed PDFs
- **List Command**: Display and filter indexed PDFs
- **Recent Command**: Show recently modified PDFs
- **Enhanced Error Messages**: Better user feedback

### 📋 **System Requirements**
- **Rust**: 1.70 or later (for building from source)
- **OS**: Linux, macOS, or Windows
- **Terminal**: Unicode/UTF-8 support recommended

### 🙏 **Acknowledgments**
Built with love using:
- [Rust](https://rust-lang.org) 🦀
- [rusqlite](https://crates.io/crates/rusqlite) for database operations
- [clap](https://crates.io/crates/clap) for CLI interface
- [indicatif](https://crates.io/crates/indicatif) for progress bars
- [walkdir](https://crates.io/crates/walkdir) for directory traversal

### 🐛 **Known Issues**
- Search, List, and Recent commands are placeholders (implementation coming in v0.2.0)
- Some system directories may show permission warnings (this is normal and expected)

### 📝 **Full Changelog**
- Initial implementation of PDF indexing system
- SQLite database with metadata extraction
- Beautiful Braille progress bars
- Cross-platform data directory support
- Duplicate prevention system
- Complete cleanup functionality
- Professional CLI interface with clap
- Comprehensive documentation and README

---

**Download**: [GitHub Releases](https://github.com/ionnss/pdfx/releases/tag/v0.1.0)  
**Source**: [GitHub Repository](https://github.com/ionnss/pdfx)  
**Issues**: [Bug Reports & Feature Requests](https://github.com/ionnss/pdfx/issues)
