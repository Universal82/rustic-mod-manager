# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Your Name <youremail@domain.com>
pkgname="RusticSkyrimModManager"
pkgver=1.0.0
pkgrel=1
epoch=
pkgdesc="A custom mod manager for currently only SkyrimSE"
arch=('any')
url=""
license=('unknown')
groups=()
depends=()
makedepends=()
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=()
noextract=()
md5sums=() #generate with 'makepkg -g'

build() {
	cargo build --release
}


package() {
    ls ..
    mkdir -p ../pkg/$pkgname/usr/share/applications
    mkdir -p ../pkg/$pkgname/usr/share/icons/rsmm/32
    mkdir -p ../pkg/$pkgname/usr/share/icons/rsmm/64
    mkdir -p ../pkg/$pkgname/usr/share/icons/rsmm/128
    mkdir -p ../pkg/$pkgname/usr/bin

    cp ./assets/icon32x32.png ../pkg/$pkgname/usr/share/icons/rsmm/32/rsmm.png
    cp ./assets/icon64x64.png ../pkg/$pkgname/usr/share/icons/rsmm/64/rsmm.png
    cp ./assets/icon128x128.png ../pkg/$pkgname/usr/share/icons/rsmm/128/rsmm.png
    cp ../target/release/rustic-mod-manager ../pkg/$pkgname/usr/bin/rsmm
    chmod +x ../pkg/$pkgname/usr/bin/rsmm
    cp ./assets/rsmm.desktop ../pkg/$pkgname/usr/share/applications/rsmm.desktop
}