# Maintainer: FabricSoul <your-email@example.com>
pkgname=gitfetch
pkgver=0.1.1
pkgrel=1
pkgdesc="A command-line tool to fetch and display Git contribution information"
arch=('x86_64' 'i686' 'arm' 'armv6h' 'armv7h' 'aarch64')
url="https://github.com/FabricSoul/gitfetch"
license=('GPL3')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/FabricSoul/gitfetch/archive/$pkgver.tar.gz")
sha256sums=('0') # This will be automatically updated by the GitHub Action

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}
