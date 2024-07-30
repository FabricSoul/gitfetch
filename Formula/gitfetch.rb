class Gitfetch < Formula
  desc "A command-line tool to fetch and display Git contribution information"
  homepage "https://github.com/FabricSoul/gitfetch"
  url "https://github.com/FabricSoul/gitfetch/archive/0.1.1.tar.gz"
  sha256 "0"
  license "GPL-3.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "gitfetch", shell_output("#{bin}/gitfetch --version")
  end
end
