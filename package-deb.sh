#!/bin/sh
set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
output_dir=$(dirname -- "$project_dir")
package_version=0.2.7
architecture=$(dpkg --print-architecture)

if [ "$architecture" != "amd64" ]; then
    echo "Questa build del pacchetto supporta amd64; architettura rilevata: $architecture" >&2
    exit 1
fi

cargo build --release --offline --manifest-path "$project_dir/Cargo.toml"
stage_dir=$(mktemp -d)
trap 'rm -rf "$stage_dir"' EXIT HUP INT TERM

install -D -m 0755 "$project_dir/target/release/rusty-browser" \
    "$stage_dir/usr/bin/rusty-browser"
install -D -m 0644 "$project_dir/packaging/rusty-browser.desktop" \
    "$stage_dir/usr/share/applications/rusty-browser.desktop"
install -D -m 0644 "$project_dir/assets/rusty-browser.svg" \
    "$stage_dir/usr/share/icons/hicolor/scalable/apps/rusty-browser.svg"
install -D -m 0644 "$project_dir/assets/rusty-browser.png" \
    "$stage_dir/usr/share/icons/hicolor/512x512/apps/rusty-browser.png"
install -D -m 0644 "$project_dir/README.md" \
    "$stage_dir/usr/share/doc/rusty-browser/README.md"

mkdir -p "$stage_dir/DEBIAN"
cat > "$stage_dir/DEBIAN/control" <<EOF
Package: rusty-browser
Version: $package_version
Section: web
Priority: optional
Architecture: $architecture
Depends: libc6, libwebkit2gtk-4.1-0 (>= 2.34.0), curl
Maintainer: Local package <local@localhost>
Description: Browser web scritto in Rust con motore WebKitGTK
 Browser desktop minimale con supporto JavaScript, CSS, cookie,
 contenuti interattivi, navigazione avanti e indietro.
EOF

package_path="$output_dir/rusty-browser_${package_version}_${architecture}.deb"
dpkg-deb --build --root-owner-group "$stage_dir" "$package_path"
install -D -m 0644 "$package_path" \
    "$project_dir/dist/rusty-browser_${package_version}_${architecture}.deb"
echo "Pacchetto creato: $package_path"
