# Ferret v0.1.1 - New Features Summary

## 📦 Version History
- **v0.1.0** - Initial release with basic file finding, organizing, and stats
- **v0.1.1** - Added pentesting features, ls command, and security tools

## ✅ All Errors Fixed (v0.1.1)
- Cross-platform compatibility implemented (Unix/Linux and Windows)
- Conditional compilation for Unix-specific features
- All compiler warnings resolved

## 🆕 New Commands Added in v0.1.1

### 1. **fr ls** - Enhanced Directory Listing
Like the classic `ls` command but with modern features:

```bash
fr ls           # List current directory
fr ls -a        # Show all files (including hidden)
fr ls -l        # Long format with details
fr ls -lH       # Long format with human-readable sizes
fr ls -R        # Recursive listing
fr ls -laR      # Combine all flags
```

**Features:**
- Color-coded output (directories in cyan, executables in green, symlinks in purple)
- Human-readable file sizes
- Detailed permissions display
- Works on both Windows and Unix/Linux
- Flags can be combined (like `-lah`)

---

### 2. **fr suid** - SUID Binary Scanner
Find binaries with SUID bit set (privilege escalation):

```bash
fr suid         # Scan current directory
fr suid /       # Full system scan
fr suid -q      # Quiet mode (paths only)
fr suid -v      # Verbose with permissions
fr suid -o file.txt  # Save to file
```

**Equivalent to:** `find / -perm -4000 -type f 2>/dev/null`

---

### 3. **fr sgid** - SGID Binary Scanner
Find binaries with SGID bit set:

```bash
fr sgid         # Scan current directory
fr sgid /       # Full system scan
fr sgid -q      # Quiet mode
fr sgid -v      # Verbose output
```

**Equivalent to:** `find / -perm -2000 -type f 2>/dev/null`

---

### 4. **fr writable** - World-Writable File Scanner
Find files and directories anyone can modify:

```bash
fr writable     # Find all writable files/dirs
fr writable -d  # Only directories
fr writable -f  # Only files
fr writable -v  # Verbose with permissions
fr writable /var -q  # Quiet scan of /var
```

**Perfect for finding persistence locations**

---

### 5. **fr caps** - Linux Capabilities Scanner
Find files with special capabilities (often missed!):

```bash
fr caps         # Scan current directory
fr caps /       # Full system scan
fr caps -v      # Verbose output
fr caps -o caps.txt  # Save results
```

**Finds:** `cap_setuid`, `cap_dac_override`, `cap_sys_admin`, etc.

---

### 6. **fr configs** - Credential & Config Hunter
Find interesting files:

```bash
fr configs      # Search current dir
fr configs /    # Full system search
fr configs /home -v  # User configs with sizes
fr configs /var/www  # Web server configs
```

**Searches for:**
- Password files (`passwd`, `shadow`, `credential`)
- SSH keys (`id_rsa`, `id_dsa`, `authorized_keys`)
- Certificates (`*.pem`, `*.key`, `*.crt`)
- Config files (`*.conf`, `*.cfg`, `*.ini`, `*.yaml`, `*.json`)
- Environment files (`*.env`)
- Shell configs (`.bashrc`, `.zshrc`, `.profile`)

---

### 7. **fr recent** - Recent File Monitor
Detect recently modified files:

```bash
fr recent       # Last 60 minutes (default)
fr recent -t 10 # Last 10 minutes
fr recent -t 1440  # Last 24 hours
fr recent / -t 30  # System-wide, last 30 min
fr recent -v    # Show time since modification
```

**Use case:** Monitor changes after running exploits

---

### 8. **fr dn** - Dev Null Helper
Simplified output redirection:

```bash
fr dn find / -name "*.conf"     # Hide all output
fr dn -e find / -name "password"  # Show errors only
fr dn nmap -sV 192.168.1.0/24   # Silent nmap
```

**Equivalent to:**
- `fr dn command` = `command 2>/dev/null`
- `fr dn -e command` = `command 1>/dev/null`

---

## 🎯 Common Pentesting Workflows

### Quick PrivEsc Enumeration
```bash
fr suid -q > suid.txt
fr sgid -q > sgid.txt
fr caps -q > caps.txt
fr writable -d -q > writable_dirs.txt
```

### Hunt for Credentials
```bash
fr configs / -o all_configs.txt
cat all_configs.txt | grep -i "password\|credential\|secret\|api"
```

### Monitor System Changes
```bash
# Before exploit
fr recent / -t 5 -q > before.txt

# After exploit
fr recent / -t 5 -q > after.txt

# Compare
diff before.txt after.txt
```

---

## 🔧 Technical Improvements

### Cross-Platform Support
- All features work on Windows AND Unix/Linux
- Conditional compilation for platform-specific features
- Unix-only features show helpful warnings on Windows
- File permissions handled correctly per platform

### Performance
- Fast Rust implementation
- Efficient file system walking
- Progress indicators for long scans
- Optimized for large directory trees

### Code Quality
- No compiler errors
- No compiler warnings
- Clean, maintainable code
- Well-documented functions

---

## 📚 Documentation

### Updated Files
1. **README.md** - Comprehensive pentesting section with examples
2. **PENTEST_CHEATSHEET.md** - Quick reference for security researchers
3. **CHANGELOG.md** - Detailed changelog of all changes
4. **This summary** - Quick overview of features

### Key Documentation Sections
- Installation instructions
- Command reference tables
- Real-world workflow examples
- Pro tips for pentesters
- Comparison with traditional Unix commands
- Integration with other security tools

---

## 🚀 Quick Start

```bash
# Install
cargo install ferret-rs

# Or use the installer
curl -fsSL https://raw.githubusercontent.com/Karmanya03/Ferret/main/install.sh | bash

# Try it out
fr ls -lah           # List files
fr suid /usr/bin     # Find SUID binaries
fr configs /etc      # Find configs
fr writable /tmp     # Find writable files
```

---

## 📊 Command Comparison

| Traditional | Ferret | Notes |
|------------|--------|-------|
| `ls -lah` | `fr ls -lah` | Color-coded output |
| `find / -perm -4000 -type f 2>/dev/null` | `fr suid /` | Much cleaner! |
| `find / -perm -2000 -type f 2>/dev/null` | `fr sgid /` | Easier syntax |
| `find / -writable 2>/dev/null` | `fr writable /` | More readable |
| `getcap -r / 2>/dev/null` | `fr caps /` | Better output |
| `command 2>/dev/null` | `fr dn command` | Shorter! |

---

## ✨ What Makes This Special

1. **Shorter Commands** - Type less, do more
2. **Better Output** - Colors and formatting for readability  
3. **Cross-Platform** - Works on Windows too
4. **Fast** - Rust performance
5. **Complete** - All common pentesting checks in one tool
6. **Modern** - Built with modern development practices
7. **Well-Documented** - Extensive docs and examples

---

## 🎓 For Pentesters

Ferret v0.1.1 gives you:
- Quick privilege escalation enumeration
- Credential hunting capabilities
- System change monitoring
- Cleaner command syntax
- Scriptable output (quiet mode)
- File output for all commands
- Integration with existing workflows

Perfect for:
- 🔴 Red Team operations
- 🔵 Blue Team defense
- 🧪 Security research
- 🏁 CTF competitions
- 💻 System administration
- 🔍 Forensics

---

**Version:** 0.1.1 (upgraded from v0.1.0)
**Date:** January 18, 2026  
**Status:** ✅ Production Ready  
**Platform:** Windows, Linux, macOS, Unix
