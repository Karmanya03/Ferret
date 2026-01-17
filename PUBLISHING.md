# Quick Publishing Guide

## 🚀 Fastest Path to Distribution

### Step 1: Publish to Cargo (5 minutes)
```bash
cargo login                    # Get token from crates.io/me
cargo publish                  # Users can now: cargo install ferret
```
✅ **Done!** Most Rust users can now install your tool.

---

### Step 2: Create GitHub Release (2 minutes)
```bash
git tag v1.1.2
git push origin v1.1.2
```
Then go to: https://github.com/Karmanya03/Ferret/releases/new
- Select tag `v1.1.2`
- Click "Publish release"

---

### Step 3: Arch Linux AUR (10 minutes)

1. **Get sha256 sum:**
```bash
wget https://github.com/Karmanya03/Ferret/archive/v1.1.2.tar.gz
sha256sum v1.1.2.tar.gz
```

2. **Update `aur/PKGBUILD`:**
- Line 2: Your name/email
- Line 14: Replace `SKIP` with the sha256 from above

3. **Publish to AUR:**
```bash
# Create AUR account first: https://aur.archlinux.org/register
git clone ssh://aur@aur.archlinux.org/ferret.git ferret-aur
cp aur/PKGBUILD ferret-aur/
cd ferret-aur/
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: ferret 1.1.2"
git push
```

✅ **Done!** Arch users can now: `yay -S ferret`

---

### Step 4: Homebrew Tap (10 minutes)

1. **Get sha256 sum:**
```bash
wget https://github.com/Karmanya03/Ferret/archive/refs/tags/v1.1.2.tar.gz
shasum -a 256 v1.1.2.tar.gz
```

2. **Update `homebrew/ferret.rb`:**
- Line 5: Replace empty quotes with the sha256 from above

3. **Create Homebrew tap repository:**
- Create new GitHub repo: `homebrew-ferret`
```bash
git clone https://github.com/Karmanya03/homebrew-ferret.git
cp homebrew/ferret.rb homebrew-ferret/Formula/
cd homebrew-ferret/
git add Formula/ferret.rb
git commit -m "Add ferret formula"
git push
```

✅ **Done!** macOS users can now:
```bash
brew tap Karmanya03/ferret
brew install ferret
```

---

## 📝 Summary of Changes Made

### Files Modified:
- ✅ `Cargo.toml` - Package name is now `ferret`, ready for crates.io
- ✅ `src/utils.rs` - Fixed path truncation to use full terminal width
- ✅ `README.md` - Added installation methods for cargo/AUR/Homebrew

### Files Created:
- ✅ `homebrew/ferret.rb` - Homebrew formula for macOS
- ✅ `aur/PKGBUILD` - Package build script for Arch Linux
- ✅ `DISTRIBUTION.md` - Complete distribution guide
- ✅ `SETUP.md` - Quick setup instructions
- ✅ `PUBLISHING.md` - This file

---

## 🎯 What Users Will Run

| Platform | Command |
|----------|---------|
| **Any (with Rust)** | `cargo install ferret` |
| **Arch Linux** | `yay -S ferret` or `paru -S ferret` |
| **macOS** | `brew tap Karmanya03/ferret && brew install ferret` |
| **From Source** | `git clone ... && cargo build --release` |

---

## 🔧 Terminal Width Fix

The "..." truncation issue has been fixed! The tool now:
- Detects terminal width automatically
- Displays full file paths
- Adjusts column widths dynamically
- Falls back to 80 columns if detection fails

**Before:** `tor-browser/Browser/TorBrowser/Data/Tor/Pluggab...`
**After:** Full path displayed based on your terminal width!

---

## 📞 Need Help?

- See `DISTRIBUTION.md` for detailed instructions
- See `SETUP.md` for step-by-step setup
- Test locally: `cargo build --release && cargo test`

Start with Step 1 (cargo publish) - it's the easiest and reaches the most users!
