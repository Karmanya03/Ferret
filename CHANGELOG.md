# Changelog

All notable changes to Ferret will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-01-18

### 🔥 Added - Pentesting & Security Features

#### New Commands
- **`fr ls`** - Enhanced directory listing command
  - Color-coded output (directories, executables, symlinks)
  - Long format support with permissions and sizes (like `ls -l`)
  - Human-readable file sizes (like `ls -h`)
  - Recursive listing (like `ls -R`)
  - Show all files including hidden (like `ls -a`)
  - Optional permission explanations with `-e` flag
  - Works on both Unix and Windows
  - Flags: `-a` (all), `-l` (long), `-R` (recursive), `-H` (human-readable), `-e` (explain permissions)

- **`fr suid`** - Find SUID binaries (setuid - runs with owner privileges)
  - Essential for privilege escalation enumeration
  - Replaces: `find / -perm -4000 -type f 2>/dev/null`
  - Flags: `-q` (quiet), `-v` (verbose), `-o` (output to file)

- **`fr sgid`** - Find SGID binaries (setgid - runs with group privileges)
  - Group-based privilege escalation vectors
  - Replaces: `find / -perm -2000 -type f 2>/dev/null`
  - Flags: `-q` (quiet), `-v` (verbose), `-o` (output to file)

- **`fr writable`** - Find world-writable files and directories
  - Identify files/dirs anyone can modify
  - Perfect for finding persistence locations
  - Flags: `-q` (quiet), `-v` (verbose), `-d` (dirs only), `-f` (files only), `-o` (output)

- **`fr caps`** - Find files with Linux capabilities
  - Often overlooked privilege escalation vector
  - Checks for `cap_setuid`, `cap_dac_override`, etc.
  - Flags: `-q` (quiet), `-v` (verbose), `-o` (output to file)

- **`fr configs`** - Hunt for interesting configuration files and credentials
  - Searches for: passwords, SSH keys, certificates, .env files, configs
  - Finds: `*.conf`, `*.key`, `*.pem`, `id_rsa`, `passwd`, `shadow`, etc.
  - Flags: `-q` (quiet), `-v` (verbose with sizes), `-o` (output to file)

- **`fr recent`** - Find recently modified files
  - Detect system changes after running exploits
  - Monitor file modifications in real-time
  - Flags: `-t` (time in minutes), `-q` (quiet), `-v` (verbose), `-o` (output)

- **`fr dn`** - Dev null helper for cleaner command output
  - Simplified output redirection
  - `fr dn command` = `command 2>/dev/null`
  - `fr dn -e command` = `command 1>/dev/null` (show errors only)
  - No more typing `2>/dev/null` repeatedly!

#### New Module
- Added `src/pentest.rs` module with all security enumeration functions
- Optimized for speed and accuracy
- Progress indicators for long scans
- Colored output for better readability

### 📚 Documentation
- Added comprehensive pentesting section to README.md
- Created PENTEST_CHEATSHEET.md for quick reference
- Included real-world workflow examples
- Added comparison table with traditional commands
- Documented all flags and use cases

### 🎯 Target Audience
- Pentesters and ethical hackers
- Red team operators
- Blue team defenders
- Security researchers
- CTF players
- Linux system administrators

### ⚡ Performance
- All security commands use efficient file system walking
- Built on Rust for blazing fast performance
- Progress indicators for long-running scans
- Parallel processing where applicable
- Cross-platform support with conditional compilation

### 🔧 Improvements
- Updated version to 0.1.1
- Enhanced package description to include pentesting features
- Updated keywords in Cargo.toml
- Cross-platform compatibility for Unix and Windows
- Proper handling of file permissions on different platforms

### 📦 Dependencies
- No new dependencies required
- All features use existing Rust std library and project dependencies

## [0.1.0] - 2026-01-XX

### Added
- Initial release
- `fr find` - Advanced file search with glob and regex support
- `fr organize` - Smart file organization by type, date, or size
- `fr stats` - Directory statistics and analysis
- Combined flags support (netcat-style: `-ivH`, `-qr`, etc.)
- Multiple output formats (default, detailed, JSON)
- Size filters with human-readable formats (K/M/G)
- Time-based filters
- File type filtering
- Execute commands on found files
- Dry-run mode for organization
- Recursive operations
- Hidden file support
- Follow symlinks option
- Progress indicators
- Colored output
- Cross-platform support (Linux/Unix)

### Features
- Glob pattern matching
- Regex pattern matching
- Case-insensitive search
- Size-based filtering
- Date-based filtering
- File type categorization
- Directory statistics with charts
- Batch file operations

---

## Future Roadmap

### Planned for 1.3.0
- [ ] Network enumeration features
- [ ] Port scanner integration
- [ ] Process enumeration
- [ ] Service discovery
- [ ] User enumeration
- [ ] Kernel exploit suggester
- [ ] Auto-update checker

### Planned for 1.4.0
- [ ] Web server enumeration
- [ ] Database credential finder
- [ ] API key scanner
- [ ] JWT token finder
- [ ] Docker/container enumeration
- [ ] Kubernetes secret finder

### Planned for 2.0.0
- [ ] Plugin system for custom checks
- [ ] Report generation (HTML/PDF)
- [ ] Integration with exploit-db
- [ ] CVE checking
- [ ] Compliance checking
- [ ] Cloud storage enumeration (AWS, Azure, GCP)

---

## Contributing

We welcome contributions! Please see our contributing guidelines for more information.

## Security

Found a security vulnerability? Please report it privately to karmanya03@proton.me

## License

MIT License - See LICENSE file for details
