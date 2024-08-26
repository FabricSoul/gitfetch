class Gitfetch < Formula
  desc "A command-line tool to fetch and display Git contribution information"
  homepage "https://github.com/FabricSoul/gitfetch"
  url "https://github.com/FabricSoul/gitfetch/archive/0.1.1.tar.gz"
  sha256 "7931778c658de43fa4d85864f4fb7ee22a32000cbbf95fc2a73d6f60721622d5"
  license "GPL-3.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "gitfetch", shell_output("#{bin}/gitfetch --version")
  end
end
