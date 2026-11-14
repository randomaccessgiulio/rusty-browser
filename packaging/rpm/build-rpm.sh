#!/bin/sh
set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
project_name=rusty-browser
package_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$project_dir/Cargo.toml" | head -n 1)
if [ -z "$package_version" ]; then
    echo "Impossibile leggere la versione da Cargo.toml" >&2
    exit 1
fi

build_root=$(mktemp -d)
trap 'rm -rf "$build_root"' EXIT HUP INT TERM
for directory in BUILD BUILDROOT RPMS SOURCES SPECS SRPMS; do
    mkdir -p "$build_root/$directory"
done

tar -czf "$build_root/SOURCES/${project_name}-${package_version}.tar.gz" \
    --exclude="${project_name}/.git" \
    --exclude="${project_name}/target" \
    --exclude="${project_name}/dist" \
    -C "$(dirname -- "$project_dir")" "$project_name"

rpmbuild -ba \
    --define "_topdir $build_root" \
    "$project_dir/packaging/rpm/rusty-browser.spec"

mkdir -p "$project_dir/dist"
find "$build_root/RPMS" -type f -name '*.rpm' -exec cp -f {} "$project_dir/dist/" \;
echo "Pacchetto RPM creato in $project_dir/dist"
