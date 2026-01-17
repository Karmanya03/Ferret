class Ferret < Formula
  desc "Blazingly fast file finder and organizer for Unix systems"
  homepage "https://github.com/Karmanya03/Ferret"
  url "https://github.com/Karmanya03/Ferret/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "" # Update this after creating a release
  license "MIT"
  head "https://github.com/Karmanya03/Ferret.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    # Test basic functionality
    system "#{bin}/fr", "--version"
  end
end
