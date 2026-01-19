# Ferret-rs

Yeah, it's another file finder. But this one doesn't suck.

Written in Rust because we're not savages who enjoy waiting for Python to boot up. The binary is called `fr` because typing is overrated and your time is valuable.

## Why This Exists

Look, we've all been there. You're trying to find that one config file from 2019, and you're staring at a `find` command that looks like someone had a seizure on their keyboard. Or you're using `fd` but still typing out long flag names like it's 1995.

Ferret does three things well:
1. Finds your files fast (like, really fast)
2. Organizes your disaster of a Downloads folder
3. Tells you what's eating all your disk space

No PhD required. Just `fr find "*.whatever"` and get on with your life.

## What It Does

**File Search** - Glob patterns, regex, filters for size/date/type. You know, the stuff that should be easy but never is.

**Smart Organization** - Automatically sorts your files by type, date, or size. Your future self will thank you.

**Directory Stats** - Shows you what's actually in that folder. With pretty charts because humans like pretty things.

**Pentesting & Security Tools** - SUID/SGID finder, writable file scanner, capabilities checker, config hunter, and more. Built for pentesters, red teamers, and blue teamers.

**Combined Flags** - Because if netcat can do `nc -nlvp`, you should be able to do `fr find "*.log" -ivH`. Flag combining works like you'd expect it to.

**Output Options** - Default view, detailed with timestamps and sizes, or JSON if you're into that sort of thing.

## Getting Started

### Installation

Pick your poison:

#### **Via Cargo (Recommended - Works Everywhere)**

**🚀 One-Command Install with Auto PATH Setup:**
```bash
curl -fsSL https://raw.githubusercontent.com/Karmanya03/Ferret/main/install.sh | bash
```
*The installer will ask if you want cargo install (with auto PATH setup) or system-wide install.*

**Or manual cargo install:**
```bash
# 1. Install
cargo install ferret-rs

# 2. Add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc
```

#### **Arch Linux (AUR)**
```bash
yay -S ferret-rs
# or
paru -S ferret-rs
```

#### **macOS (Homebrew)**
```bash
# Add the tap (first time only)
brew tap Karmanya03/ferret

# Install
brew install ferret-rs
```

#### **From Source (The Hard Way)**
```bash
# Clone it
git clone https://github.com/Karmanya03/Ferret.git
cd Ferret

# Build it (grab a coffee, Rust is compiling)
cargo build --release

# Install it
sudo cp target/release/fr /usr/local/bin/
```

**What you need:**
- Rust 1.70 or newer (for cargo install or building from source)
- A Linux/Unix box (Arch, Ubuntu, Kali, Debian, Fedora - whatever floats your boat)
- Basic understanding that `sudo` means business

### Troubleshooting

#### "fr: command not found" after cargo install

This means `~/.cargo/bin` isn't in your PATH. Here's the fix:

**Quick Fix:**
```bash
# Run the automated installer (it handles PATH setup)
curl -fsSL https://raw.githubusercontent.com/Karmanya03/Ferret/main/install.sh | bash
```

**Or manually:**
```bash
# Check if the binary exists
ls -la ~/.cargo/bin/fr

# If it's there, add to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc
```

**Verify it worked:**
```bash
which fr          # Should show: /home/username/.cargo/bin/fr
fr --version      # Should show: fr 0.1.1
```

### Upgrading

Already have Ferret installed? Here's how to upgrade to the latest version:

#### **🚀 One-Command Upgrade (Recommended)**
```bash
curl -fsSL https://raw.githubusercontent.com/Karmanya03/Ferret/main/install.sh | bash
```
*Automatically detects if you have cargo/brew and upgrades to the latest version*

#### **Via Cargo**
```bash
cargo install ferret-rs --force
```

#### **Arch Linux (AUR)**
```bash
yay -Syu ferret-rs
# or
paru -Syu ferret-rs
```

#### **macOS (Homebrew)**
```bash
brew update
brew upgrade ferret-rs
```

#### **From Source**
```bash
cd Ferret
git pull origin main
cargo build --release
sudo cp target/release/fr /usr/local/bin/
```

### Uninstalling

Need to remove Ferret from your system? Here's how:

#### **Via Cargo**
```bash
cargo uninstall ferret-rs
```

#### **Arch Linux (AUR)**
```bash
yay -R ferret-rs
# or
paru -R ferret-rs
# or using pacman directly
sudo pacman -R ferret-rs
```

#### **macOS (Homebrew)**
```bash
brew uninstall ferret-rs
# or
brew remove ferret-rs
```

#### **From Source (Manual Installation)**
```bash
# Remove the binary
sudo rm /usr/local/bin/fr

# Optional: Clean up the source directory
rm -rf ~/path/to/Ferret
```

#### **Clean Cargo Cache (Optional)**
If you want to completely remove all traces including cached builds:
```bash
# Remove cargo cache for ferret-rs
rm -rf ~/.cargo/registry/cache/*/ferret-rs*
rm -rf ~/.cargo/registry/src/*/ferret-rs*

# Remove build artifacts from source directory
cd ~/path/to/Ferret
cargo clean
```

---

## Usage Guide & Quick Reference

<table>
<tr>
<td width="50%" valign="top">

### Find Command Basics

The bread and butter. Finding files without wanting to throw your keyboard.

```bash
# Find Rust files
fr find "*.rs"

# Case-insensitive (because who remembers capitalization?)
fr find "readme*" -i

# Regex for the brave
fr find "test_.*\.rs$" -r

# Combine flags like a pro
fr find "*config*" -iH     # ignore-case + hidden
fr find "*error*" -ivH     # ignore-case + verbose + hidden
fr find "*.log" -qi        # quiet + ignore-case
```

**Size Filters**
```bash
# Files over 100MB (find the chunky boys)
fr find "*" --min-size 100M

# Small files under 1KB
fr find "*" --max-size 1K

# Specific range
fr find "*.log" --min-size 10M --max-size 100M
```

**Time Filters**
```bash
# Modified in last week
fr find "*" --modified-days 7

# Recent logs
fr find "*.log" -m 1  # yesterday's logs

# Old backups
fr find "*backup*" -m 365
```

**Type Filters**
```bash
# Only files (not directories)
fr find "*.conf" -t file

# Only directories
fr find "*test*" -t dir

# Symlinks
fr find "*" -t symlink
```

</td>
<td width="50%" valign="top">

### Quick Command Reference

| Command | What It Does |
|---------|-------------|
| `fr find PATTERN` | Find files matching pattern |
| `fr organize PATH` | Clean up the mess |
| `fr stats PATH` | See what's taking up space |
| `fr ls PATH` | List directory contents (like ls) |
| `fr suid` | Find SUID binaries (pentesting) |
| `fr sgid` | Find SGID binaries (pentesting) |
| `fr writable` | Find writable files/dirs (pentesting) |
| `fr caps` | Find files with capabilities (pentesting) |
| `fr configs` | Hunt for credentials/configs (pentesting) |
| `fr recent` | Find recently modified files (pentesting) |
| `fr dn CMD` | Run command (optional output control) |

**Common Find Patterns**

```bash
# The essentials
fr find "*.rs"              # Rust files
fr find "*test*"            # Anything with "test"
fr find "*" -H              # Show hidden files too

# With output options
fr find "*.pdf" -o detailed # Show size & dates
fr find "*.jpg" -o json     # JSON output
fr find "*.txt" -q          # Quiet mode (no fluff)

# Power moves
fr find "*.tmp" -x "rm {}"  # Delete temp files
fr find "*.log" | wc -l     # Count log files
fr find "*" -d 3            # Only 3 levels deep
```

**All Find Flags**

| Flag | What |
|------|------|
| `-p PATH` | Where to search |
| `-i` | Ignore case |
| `-r` | Use regex |
| `-t TYPE` | file/dir/symlink |
| `--min-size SIZE` | Min size (K/M/G) |
| `--max-size SIZE` | Max size |
| `-m DAYS` | Modified within N days |
| `-d DEPTH` | Max depth |
| `-H` | Include hidden |
| `-o FORMAT` | Output: default/detailed/json |
| `-x CMD` | Execute command |
| `-v` | Verbose mode |
| `-q` | Quiet mode |
| `-l` | Follow symlinks |

</td>
</tr>
<tr>
<td width="50%" valign="top">

### Organize Command

Stop living in chaos. Let the computer do the sorting.

```bash
# Organize current directory by type
fr organize

# Organize Downloads (we know it's a disaster)
fr organize ~/Downloads

# Preview first (dry-run)
fr organize -n

# Copy instead of moving (safety first)
fr organize -c

# Custom output location
fr organize -o ~/sorted_stuff

# Go deep (recursive)
fr organize -r

# Combine flags (netcat-style)
fr organize ~/Downloads -nrv  # dry-run + recursive + verbose
fr organize ~/Pictures -crv   # copy + recursive + verbose
```

**Organization Methods**

```bash
# By file type (default)
fr organize -m type
# Creates: documents/, images/, videos/, code/, etc.

# By date
fr organize -m date
# Creates: 2026/01-January/, 2025/12-December/, etc.

# By size
fr organize -m size
# Creates: tiny/, small/, medium/, large/, huge/
```

**Organize Flags**

| Flag | What |
|------|------|
| `-m METHOD` | type/date/size |
| `-o PATH` | Output directory |
| `-n` | Dry run (preview) |
| `-c` | Copy (don't move) |
| `-r` | Recursive |
| `-H` | Include hidden |
| `-v` | Verbose |

</td>
<td width="50%" valign="top">

### Stats Command

Figure out where all your disk space went.

```bash
# Current directory stats
fr stats

# Recursive analysis (the whole tree)
fr stats -r

# Specific directory
fr stats ~/Documents -r

# Include hidden files
fr stats -H

# Verbose output
fr stats -rv
```

**What You Get:**
- Total files and directories
- Size distribution (with ASCII charts)
- Top file types by count and size
- Largest files
- Everything color-coded because terminal UX matters

**Stats Flags**

| Flag | What |
|------|------|
| `-r` | Recursive |
| `-H` | Include hidden |
| `-v` | Verbose |

---

### File Type Categories

When you organize by type, here's where stuff goes:

- **documents** - pdf, doc, docx, txt, md, rtf
- **images** - jpg, png, gif, svg, webp, bmp
- **videos** - mp4, avi, mkv, mov, webm
- **audio** - mp3, wav, flac, aac, ogg
- **archives** - zip, tar, gz, 7z, rar
- **code** - rs, py, js, c, cpp, go, java
- **web** - html, css, json, xml, yaml
- **spreadsheets** - xls, xlsx, csv
- **presentations** - ppt, pptx, key
- **executables** - exe, deb, rpm, apk
- **databases** - db, sqlite, sql
- **fonts** - ttf, otf, woff

Files with no extension? They go in `no-extension/`.
Unknown extensions? They get their own folder like `xyz-files/`.

</td>
</tr>
</table>

---

## Real-World Examples

Because theory is boring and you probably just skimmed to this part anyway.

### Cleaning Up Your Downloads

```bash
# See what you're dealing with
cd ~/Downloads
fr stats

# Preview the organization
fr organize -n

# Actually do it
fr organize -m type

# Or organize by date if you're fancy
fr organize -m date -r
```

### Finding That File You Know Exists

```bash
# You know it has "config" in the name
fr find "*config*" -i

# It's probably a .conf file
fr find "*.conf"

# You edited it last week
fr find "*config*" --modified-days 7

# Screw it, show me everything recent
fr find "*" -m 7 -o detailed
```

### Nuking Temp Files

```bash
# Find them first
fr find "*.tmp"

# Delete them (quietly, no drama)
fr find "*.tmp" -x "rm {} 2>/dev/null"

# Or old backups
fr find "*~" -m 30 -x "rm -f {}"
```

### Batch Operations

```bash
# Count lines in all Python files
fr find "*.py" | xargs wc -l

# Grep for TODO comments
fr find "*.rs" | xargs grep "TODO"

# Compress old logs
fr find "*.log" -m 30 -x "gzip {}"

# Get total size (JSON + jq)
fr find "*.mp4" -o json | jq '.[].size' | awk '{sum+=$1} END {print sum}'
```

### Finding Space Hogs

## 🔥 Pentesting & Security Features

Ferret now includes powerful security enumeration tools designed for pentesters, red teamers, and blue teamers. All commands are short, fast, and designed for privilege escalation reconnaissance.

### Quick Security Command Reference

| Command | What It Does | Typical Usage |
|---------|-------------|---------------|
| `fr suid` | Find SUID binaries | Privilege escalation vectors |
| `fr sgid` | Find SGID binaries | Group privilege escalation |
| `fr writable` | Find world-writable files/dirs | Writable paths for exploitation |
| `fr caps` | Find files with capabilities | Linux capabilities abuse |
| `fr configs` | Find interesting config files | Credentials, keys, passwords |
| `fr recent` | Find recently modified files | Detect recent system changes |
| `fr dn CMD` | Run command (optional output control) | Execute with optional redirection |
| `fr ls` | List directory contents | Like ls but with colors |

### List Command (ls)

Enhanced directory listing with color-coded output:

```bash
# List current directory
fr ls

# List with all files including hidden (like ls -a)
fr ls -a

# Long format with details (like ls -l)
fr ls -l

# Long format with human-readable sizes (like ls -lh)
fr ls -lH

# List recursively (like ls -R)
fr ls -R

# Combine flags (like ls -laR)
fr ls -laR

# List specific directory
fr ls /etc

# Full featured listing
fr ls /var/log -laH
```

**Color coding:**
- **Directories** - Cyan and bold
- **Executable files** - Green and bold (Unix/Linux only)
- **Symlinks** - Purple
- **Regular files** - Default color

**Flags:**
- `-a` / `--all` - Show hidden files
- `-l` / `--long` - Long format with permissions, size, date
- `-R` / `--recursive` - List subdirectories recursively
- `-H` / `--human` - Human-readable file sizes (KB, MB, GB)
- `-e` / `--explain-perms` - Explain permissions (e.g., owner:rw-, group:r--, other:r--)

**Examples with permission explanations:**
```bash
# Show detailed permission explanations
fr ls -le           # Long format with explanations
fr ls -lHe          # Long format, human sizes, with explanations
fr ls -laHe         # All files, long format, human sizes, with explanations

# Without explanations (default, cleaner output)
fr ls -lH           # Just permissions symbols
```

### SUID Binary Scanner

Find SUID binaries (run with owner's privileges) - essential for privilege escalation:

```bash
# Find all SUID binaries (classic)
fr suid

# Search from root (comprehensive scan)
fr suid /

# Quiet mode - just the paths (for scripts)
fr suid -q

# Verbose - show permissions
fr suid -v

# Save results to file
fr suid -o suid_findings.txt

# Search specific directory
fr suid /usr/bin
```

**Equivalent to:**
```bash
find / -perm -4000 -type f 2>/dev/null
find / -perm -u=s -type f 2>/dev/null
```

### SGID Binary Scanner

Find SGID binaries (run with group's privileges):

```bash
# Find all SGID binaries
fr sgid

# Full system scan
fr sgid /

# Quiet mode
fr sgid -q

# With detailed permissions
fr sgid -v

# Save to file
fr sgid -o sgid_findings.txt
```

**Equivalent to:**
```bash
find / -perm -2000 -type f 2>/dev/null
find / -perm /2000 -type f 2>/dev/null
```

### World-Writable Files Scanner

Find files and directories that anyone can modify:

```bash
# Find all world-writable files and directories
fr writable

# Only directories (for backdoor placement)
fr writable -d

# Only files
fr writable -f

# Quiet mode for scripting
fr writable -q

# Verbose with permissions
fr writable -v

# Search specific path
fr writable /var

# Save results
fr writable -o writable_paths.txt
```

**Common use cases:**
- Find writable directories for persistence
- Locate configuration files you can modify
- Identify temp directories with weak permissions

### Linux Capabilities Scanner

Find files with special capabilities (often overlooked privilege escalation vector):

```bash
# Find all files with capabilities
fr caps

# System-wide scan
fr caps /

# Quiet mode
fr caps -q

# Verbose output
fr caps -v

# Save to file
fr caps -o capabilities.txt

# Check specific directory
fr caps /usr/bin
```

**Why this matters:**
- `cap_setuid` can be used to spawn root shell
- `cap_dac_override` can read/write any file
- `cap_sys_admin` can mount filesystems
- Often missed by standard priv-esc scanners

### Config & Credential Hunter

Find interesting configuration files, credentials, keys, and sensitive data:

```bash
# Hunt for configs and credentials
fr configs

# Full system search
fr configs /

# Quiet mode - just paths
fr configs -q

# Verbose - show file sizes
fr configs -v

# Save findings
fr configs -o interesting_files.txt

# Search home directories
fr configs /home

# Check /etc for configs
fr configs /etc
```

**Searches for:**
- Configuration files (*.conf, *.cfg, *.ini, *.yaml, *.json)
- Password files (passwd, shadow, credentials)
- SSH keys (id_rsa, id_dsa, id_ecdsa, id_ed25519)
- SSL/TLS certificates and keys (*.pem, *.key, *.crt)
- Shell config files (.bashrc, .zshrc, .profile)
- Environment files (.env, *.env)
- And more...

### Recent Changes Monitor

Detect recently modified files (useful for finding newly created files or changes):

```bash
# Files modified in last 60 minutes (default)
fr recent

# Files modified in last 10 minutes
fr recent -t 10

# Last 24 hours (1440 minutes)
fr recent -t 1440

# Quiet mode
fr recent -q

# Verbose - show how long ago
fr recent -v

# Search from root
fr recent / -t 30

# Save to file
fr recent -o recent_changes.txt
```

**Use cases:**
- Detect file changes after running exploits
- Monitor system modifications
- Find newly created backdoors
- Track configuration changes

### Dev Null Helper (dn command)

Quick command executor with optional output redirection. By default, shows all output like normal - add flags to suppress output when needed:

```bash
# Run command normally (shows all output)
fr dn find / -name "*.conf"
fr dn find / -type f -perm -4000

# Hide stdout only
fr dn -s find / -name "password"

# Hide stderr only (suppress errors)
fr dn -e find / -name "*.conf"

# Hide all output (quiet mode)
fr dn -q nmap -sV 192.168.1.0/24

# Works with any command
fr dn ls -la /root
fr dn cat /etc/shadow
fr dn ping -c 4 google.com
```

**Flags:**
- No flags = Show all output (normal behavior)
- `-s` / `--hide-stdout` = Hide stdout, show stderr
- `-e` / `--hide-stderr` = Hide stderr, show stdout  
- `-q` / `--quiet` = Hide all output (both stdout and stderr)

**Equivalent to:**
```bash
# fr dn command (default)
command

# fr dn -s command
command 1>/dev/null

# fr dn -e command
command 2>/dev/null

# fr dn -q command
command 2>/dev/null 1>/dev/null
```

**Why this is useful:**
- No more "Permission denied" spam
- Cleaner output when searching system-wide
- Faster than typing redirection manually
- Perfect for one-liners

### Pentesting Workflow Examples

**Quick Privilege Escalation Enumeration:**
```bash
# One-liner to check common vectors
fr suid -q > suid.txt && fr sgid -q > sgid.txt && fr caps -q > caps.txt && fr writable -d -q > writable.txt

# Or more verbose for analysis
fr suid -v
fr sgid -v
fr caps -v
fr writable -v
```

**Hunt for Credentials:**
```bash
# Find interesting files
fr configs / -o all_configs.txt

# Then grep for juicy stuff
cat all_configs.txt | grep -i "password\|credential\|secret\|api"

# Or search specific locations
fr configs /home -v
fr configs /var/www -v
fr configs /opt -v
```

**Monitor System Changes:**
```bash
# Before running exploit
fr recent / -t 5 -q > before.txt

# After running exploit
fr recent / -t 5 -q > after.txt

# Compare
diff before.txt after.txt
```

**Find Writable Paths for Persistence:**
```bash
# Find writable directories
fr writable / -d -q

# Find writable directories in common locations
fr writable /etc -d
fr writable /var -d
fr writable /usr/local -d
```

**Script Integration:**
```bash
#!/bin/bash
# Quick privilege escalation enumeration script

echo "[*] Scanning for SUID binaries..."
fr suid -q > /tmp/suid_bins.txt

echo "[*] Scanning for SGID binaries..."
fr sgid -q > /tmp/sgid_bins.txt

echo "[*] Scanning for capabilities..."
fr caps -q > /tmp/caps.txt

echo "[*] Finding writable directories..."
fr writable -d -q > /tmp/writable_dirs.txt

echo "[*] Hunting for credentials..."
fr configs /home -q > /tmp/home_configs.txt
fr configs /etc -q > /tmp/etc_configs.txt

echo "[!] Results saved to /tmp/"
ls -lh /tmp/*.txt
```

### Pro Tips for Pentesters

1. **Combine with grep for targeted searches:**
   ```bash
   fr suid -q | grep -E "nmap|vim|find|less|more|nano"
   ```

2. **Use quiet mode in scripts:**
   ```bash
   SUID_COUNT=$(fr suid -q | wc -l)
   echo "Found $SUID_COUNT SUID binaries"
   ```

3. **Save everything to files for later analysis:**
   ```bash
   fr suid -o results/suid.txt
   fr sgid -o results/sgid.txt
   fr caps -o results/caps.txt
   ```

4. **Search specific high-value directories first:**
   ```bash
   fr configs /home -v     # User configs
   fr configs /var/www -v  # Web configs
   fr configs /opt -v      # Third-party apps
   ```

5. **Use dn for cleaner enumeration:**
   ```bash
   fr dn find / -name "*.conf" > configs.txt
   fr dn find / -type f -perm -4000 > suid.txt
   ```

---

### Finding Space Hogs

```bash
# Files over 1GB
fr find "*" --min-size 1G -o detailed

# Large videos
fr find "*.mp4" --min-size 500M

# Show me the biggest stuff
fr stats -r
```

---

## Comparison with find

Look, `find` is fine if you enjoy suffering. Here's why you might want to use `fr` instead:

| Task | Old Way (find) | New Way (fr) |
|------|----------------|--------------|
| Find .rs files | `find . -name "*.rs"` | `fr find "*.rs"` |
| Large files | `find . -size +100M` | `fr find "*" --min-size 100M` |
| Recent files | `find . -mtime -7` | `fr find "*" -m 7` |
| Delete temps | `find . -name "*.tmp" -exec rm {} \;` | `fr find "*.tmp" -x "rm {}"` |
| Case-insensitive | `find . -iname "*readme*"` | `fr find "*readme*" -i` |

**Speed:** Faster because Rust and parallel processing.
**Syntax:** Simpler because it's not from 1974.
**Output:** Actually readable without piping to 3 other commands.

---

## Performance Tips

### Make It Fast

1. **Be specific** - `fr find "test_*.rs"` beats `fr find "*"` every time
2. **Limit depth** - `fr find "*.txt" -d 2` if you know it's shallow
3. **Filter early** - Use size/date filters to narrow the search
4. **Skip detailed output** - Default view is faster for large result sets
5. **Use quiet mode** - `fr find "*.log" -q | other_command` for scripting

### When Things Get Slow

```bash
# Network drives suck, limit the damage
fr find "*.pdf" -d 2

# Permission errors everywhere? Redirect stderr
fr find "*" -p /etc 2>/dev/null

# Too many results? Add filters
fr find "*.log" --min-size 1M -m 7
```

---

## Building & Development

### Build It Yourself

```bash
# Debug build (fast compile, slow runtime)
cargo build

# Release build (slow compile, fast runtime)
cargo build --release

# Run tests
cargo test

# Run without installing
cargo run -- find "*.rs"

# Format code
cargo fmt

# Check for issues
cargo clippy
```

### Create an Alias

Because typing the full path is for people with too much free time.

```bash
# Bash
echo 'alias fr="/usr/local/bin/fr"' >> ~/.bashrc
source ~/.bashrc

# Zsh
echo 'alias fr="/usr/local/bin/fr"' >> ~/.zshrc
source ~/.zshrc
```

---

## Troubleshooting

### Permission Denied

```bash
# Use sudo for system directories
sudo fr find "*.conf" -p /etc

# Or just ignore the errors
fr find "*" 2>/dev/null
```

### Command Not Found

```bash
# Check your PATH
echo $PATH

# Use full path
/usr/local/bin/fr find "*.txt"

# Or create an alias (see above)
```

### It's Too Slow

```bash
# Limit depth
fr find "*.pdf" -d 3

# Add size filters
fr find "*" --min-size 1M

# Use quiet mode
fr find "*" -q
```

---

## License

MIT License - Do whatever you want with it. Credit appreciated but not required. See LICENSE file if you care about the legal stuff.

## Contributing

Found a bug? Have an idea? Cool. Open an issue or send a PR. No strict rules, just don't be a jerk and write decent commit messages.

## What's Next

Maybe. Probably. No promises.

- Duplicate file detection
- Content search (grep integration)
- Interactive TUI mode
- Custom organization rules
- Config file support
- Cloud storage integration

---

Made for Linux/Unix users who value their time and sanity.

If this saved you 5 minutes of typing find commands, consider it a win.
