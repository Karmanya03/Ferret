# Ferret Distribution Guide

This guide explains how to publish Ferret to various package managers so users can install it easily.

## 1. Publishing to crates.io (Cargo)

**Status:** ✅ Ready to publish

### Steps:

1. **Login to crates.io** (one-time setup)
   ```bash
   cargo login
   ```
   Get your API token from: https://crates.io/me

2. **Verify the package builds**
   ```bash
   cargo build --release
   cargo test
   ```

3. **Publish to crates.io**
   ```bash
   cargo publish
   ```

4. **Users can now install via:**
   ```bash
   cargo install ferret-rs
   ```

**Notes:**
- Package name: `ferret-rs`
- Version: 0.1.0
- The binary is called `fr` (configured in Cargo.toml)
- Once published, you can't unpublish (only yank versions)

---

## 2. Arch Linux (AUR)

**Status:** ✅ PKGBUILD created

The PKGBUILD file is located in `aur/PKGBUILD`.

### Steps to Publish:

1. **Create a GitHub release first**
   - Go to: https://github.com/Karmanya03/Ferret/releases/new
   - Tag: `v0.1.0`
   - Upload the release
   - Copy the tar.gz URL

2. **Update the PKGBUILD sha256sum**
   ```bash
   # Download the release tarball
   wget https://github.com/Karmanya03/Ferret/archive/v0.1.0.tar.gz
   
   # Generate sha256
   sha256sum v0.1.0.tar.gz
   
   # Update sha256sums=('...') in PKGBUILD with the output
   ```

3. **Test the PKGBUILD locally**
   ```bash
   cd aur/
   makepkg -si
   ```

4. **Create AUR repository**
   ```bash
   # Clone the AUR repository (you'll need an AUR account)
   git clone ssh://aur@aur.archlinux.org/ferret.git ferret-aur
   
   # Copy PKGBUILD and generate .SRCINFO
   cp PKGBUILD ferret-aur/
   cd ferret-aur/
   makepkg --printsrcinfo > .SRCINFO
   
   # Commit and push
   git add PKGBUILD .SRCINFO
   git commit -m "Initial commit: ferret-rs 0.1.0"
   git push
   ```

5. **Users can now install via:**
   ```bash
   yay -S ferret-rs
   # or
   paru -S ferret-rs
   ```

**Requirements:**
- AUR account: https://aur.archlinux.org/register
- SSH key added to your AUR account
- Update maintainer info in PKGBUILD (line 2)

---

## 3. macOS (Homebrew)

**Status:** ✅ Formula created

The formula is located in `homebrew/ferret.rb`.

### Steps to Publish:

1. **Create a GitHub release** (if not done already)
   - Tag: `v1.1.2`
   - Upload the release

2. **Update the formula sha256**
   ```bash
   # Download the release tarball
   wget https://github.com/Karmanya03/Ferret/archive/refs/tags/v0.1.0.tar.gz
   
   # Generate sha256
   shasum -a 256 v0.1.0.tar.gz
   
   # Update sha256 in homebrew/ferret.rb
   ```

3. **Create a Homebrew tap**
   ```bash
   # Create a new GitHub repository named: homebrew-ferret
   # Clone it
   git clone https://github.com/Karmanya03/homebrew-ferret.git
   
   # Copy the formula
   cp homebrew/ferret.rb homebrew-ferret/Formula/
   
   # Commit and push
   cd homebrew-ferret/
   git add Formula/ferret.rb
   git commit -m "Add ferret formula"
   git push
   ```

4. **Users can now install via:**
   ```bash
   # Add your tap
   brew tap Karmanya03/ferret
   
   # Install
   brew install ferret-rs
   ```

**Alternative - Submit to Homebrew Core:**
- More complex process
- Needs 75+ GitHub stars and 30+ watchers
- See: https://docs.brew.sh/Adding-Software-to-Homebrew

---

## 4. Debian/Ubuntu (.deb packages)

**Status:** ⚠️ Not yet configured (optional)

### To add .deb support:

1. **Install cargo-deb**
   ```bash
   cargo install cargo-deb
   ```

2. **Add metadata to Cargo.toml**
   ```toml
   [package.metadata.deb]
   maintainer = "Your Name <your.email@example.com>"
   copyright = "2026, Ferret Contributors"
   depends = "$auto"
   section = "utility"
   priority = "optional"
   assets = [
       ["target/release/fr", "usr/bin/", "755"],
       ["README.md", "usr/share/doc/ferret/", "644"],
       ["LICENSE", "usr/share/doc/ferret/", "644"],
   ]
   ```

3. **Generate .deb**
   ```bash
   cargo deb
   ```

4. **Users install via:**
   ```bash
   sudo dpkg -i ferret-rs_0.1.0_amd64.deb
   ```

---

## Release Checklist

When releasing a new version:

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `aur/PKGBUILD` (pkgver)
- [ ] Update version in `homebrew/ferret.rb` (version, url)
- [ ] Update CHANGELOG.md
- [ ] Test build locally: `cargo build --release`
- [ ] Run tests: `cargo test`
- [ ] Create GitHub release with tag (e.g., `v0.1.0`)
- [ ] Update sha256sums in PKGBUILD and Homebrew formula
- [ ] Publish to crates.io: `cargo publish`
- [ ] Update AUR repository
- [ ] Update Homebrew tap repository

---

## Quick Start (First Release)

1. **Create GitHub Release:**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
   Then create release on GitHub

2. **Publish to Cargo:**
   ```bash
   cargo publish
   ```

3. **For AUR & Homebrew:**
   - Download release tarball
   - Generate sha256 sums
   - Update PKGBUILD and ferret.rb
   - Follow steps above to publish

---

## Support Matrix

| Platform | Method | Status | Users Install Via |
|----------|--------|--------|-------------------|
| Any (with Rust) | Cargo | ✅ Ready | `cargo install ferret-rs` |
| Arch Linux | AUR | ✅ Ready | `yay -S ferret-rs` |
| macOS | Homebrew | ✅ Ready | `brew install ferret-rs` |
| Ubuntu/Debian | .deb | ⚠️ Optional | `dpkg -i ferret.deb` |
| From Source | Git | ✅ Ready | `git clone && cargo build` |

---

## Resources

- **crates.io:** https://crates.io
- **AUR:** https://wiki.archlinux.org/title/AUR_submission_guidelines
- **Homebrew:** https://docs.brew.sh/Formula-Cookbook
- **cargo-deb:** https://github.com/kornelski/cargo-deb
