# Ferret

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
fr --version      # Should show: fr 0.1.0
```

### Upgrading

Already have Ferret installed? Here's how to upgrade to the latest version:

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
