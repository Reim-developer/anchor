# Table of Content

- [What is the Anchor?](#what-is-the-anchor)
- [Supported File Formats](#supported-file-formats)
- [Quick Start](#quick-start)
- [Features](#features)
- [Example Usage](#example-usage)
- [Bug Reports](#bug-reports)

# What is the Anchor?

**Anchor** _is a lightweight, efficient Rust-based command-line tool for managing, organizing, and processing files on Linux, Windows, and macOS, designed for speed, simplicity, and optimal performance._

# Supported File Formats:

| File extension | File type |
| :------------: | :-------- |
|     .json      | JSON File |
|      .xml      | XML File  |
|      .yml      | YAML File |
|      .md       | Markdown File |

# Quick Start

## 🚀 Install & Run  

**1️⃣ Download the latest binary** [here](https://github.com/Reim-developer/anchor/releases)  
**2️⃣ Extract & Run**  

### Linux & macOS
```sh
chmod +x anchor && ./anchor --help
```

### Windows
```cmd
anchor.exe --help
```

# Features

- ✅ JSON/Markdown/XML/YAML Formatter – Beautify your messy files  
- ✅ Hash Checker – Verify file integrity  

# Example Usage

### Format a file  
```sh
anchor fmt -f YOUR_FILE
```
📌 **Note:** See the list of supported formats [here](#supported-file-formats).

### Get hash of a file  
```sh
anchor hash -f myfile.txt
```
📌 **Tip:** Use `--debug` to print logs to stdout.

# Bug Reports

🐛 Found a bug? Open an issue [here](https://github.com/Reim-developer/anchor/issues) and let me know how bad it is.

