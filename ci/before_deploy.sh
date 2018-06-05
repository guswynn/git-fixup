#!/bin/bash

# package the build artifacts

set -ex

. "$(dirname "$0")/utils.sh"

# Generate artifacts for release
mk_artifacts() {
    cargo build --target "$TARGET" --release
}

mk_tarball() {
    # When cross-compiling, use the right `strip` tool on the binary.
    local gcc_prefix="$(gcc_prefix)"
    # Create a temporary dir that contains our staging area.
    # $tmpdir/$name is what eventually ends up as the deployed archive.
    local tmpdir="$(mktemp -d)"
    local name="${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"
    local staging="$tmpdir/$name"
    # The deployment directory is where the final archive will reside.
    # This path is known by the .travis.yml configuration.
    local out_dir="$(pwd)/deployment"
    mkdir -p "$staging" "$out_dir"

    # Copy the binary and strip it.
    cp "target/$TARGET/release/git-fixup" "$staging/git-fixup"
    # Copy the licenses and README.
    cp README.md LICENSE-{MIT,APACHE} "$staging/"

    (cd "$tmpdir" && tar -czf "$out_dir/$name.tar.gz" "$name")
    rm -rf "$tmpdir"
}

main() {
    mk_artifacts
    mk_tarball
}

main
