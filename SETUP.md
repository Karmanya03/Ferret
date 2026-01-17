# Setup Instructions for Package Distribution

## ✅ What's Been Done

1. **Fixed terminal width truncation** - File paths now display fully based on your terminal width
2. **Updated Cargo.toml** - Package name is now `ferret` (version 1.1.2) ready for crates.io
3. **Created Homebrew formula** - Located in `homebrew/ferret.rb`
4. **Created AUR PKGBUILD** - Located in `aur/PKGBUILD`
5. **Updated README** - Added multiple installation methods

## 🚀 Next Steps to Enable Distribution

### 1. Publish to crates.io (Easiest - Do This First)

```bash
# One-time: Login with your crates.io API token
cargo login

# Publish (users can then: cargo install ferret)
cargo publish
```

Get your API token from: https://crates.io/me

### 2. Create GitHub Release

Before setting up AUR or Homebrew, create a release:

```bash
# Tag the release
git tag v1.1.2
git push origin v1.1.2

# Then go to GitHub and create a release from the tag
# https://github.com/Karmanya03/Ferret/releases/new
```

### 3. Setup Arch Linux (AUR)

After creating the GitHub release:

```bash
# Get the sha256 of your release tarball
wget https://github.com/Karmanya03/Ferret/archive/v1.1.2.tar.gz
sha256sum v1.1.2.tar.gz

# Update aur/PKGBUILD:
# - Line 2: Change maintainer to your info
# - Line 14: Replace 'SKIP' with the sha256 from above

# Test it works
cd aur/
makepkg -si

# Create AUR repository (need AUR account first)
git clone ssh://aur@aur.archlinux.org/ferret.git ferret-aur
cp PKGBUILD ferret-aur/
cd ferret-aur/
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: ferret 1.1.2"
git push
```

Register at: https://aur.archlinux.org/register

### 4. Setup Homebrew (macOS)

After creating the GitHub release:

```bash
# Get the sha256
wget https://github.com/Karmanya03/Ferret/archive/refs/tags/v1.1.2.tar.gz
shasum -a 256 v1.1.2.tar.gz

# Update homebrew/ferret.rb line 5 with the sha256

# Create a tap repository on GitHub named: homebrew-ferret
# Then:
git clone https://github.com/Karmanya03/homebrew-ferret.git
cp homebrew/ferret.rb homebrew-ferret/Formula/
cd homebrew-ferret/
git add Formula/ferret.rb
git commit -m "Add ferret formula"
git push
```

## 📦 How Users Will Install

Once you complete the steps above:

**Cargo (All platforms):**
```bash
cargo install ferret
```

**Arch Linux:**
```bash
yay -S ferret
```

**macOS:**
```bash
brew tap Karmanya03/ferret
brew install ferret
```

**From source (still works):**
```bash
git clone https://github.com/Karmanya03/Ferret.git
cd Ferret
cargo build --release
sudo cp target/release/fr /usr/local/bin/
```

## 📝 Important Files Created

- `homebrew/ferret.rb` - Homebrew formula
- `aur/PKGBUILD` - Arch Linux build script
- `DISTRIBUTION.md` - Complete guide for maintaining packages
- Updated `Cargo.toml` - Ready for crates.io
- Updated `README.md` - Installation instructions
- Fixed `src/utils.rs` - Terminal width detection

## 🔄 For Future Releases

When you release v0.2.0 or later:

1. Update version in `Cargo.toml`
2. Update version in `aur/PKGBUILD`
3. Update version in `homebrew/ferret.rb`
4. Create new GitHub release
5. Update sha256 sums
6. `cargo publish`
7. Push updated PKGBUILD to AUR
8. Push updated formula to Homebrew tap

See `DISTRIBUTION.md` for the complete checklist.

## 🧪 Test Before Publishing

```bash
# Build and test locally
cargo build --release
cargo test

# Test the binary works
./target/release/fr --version
./target/release/fr find "*.rs"
```

## ⚠️ One-Time Setup Required

- **crates.io**: Create account, get API token
- **AUR**: Create account, add SSH key
- **Homebrew tap**: Create `homebrew-ferret` repository on GitHub

That's it! Start with `cargo publish` - it's the easiest and reaches the most Rust users.
