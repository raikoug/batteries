#!/bin/bash
set -e

echo "Building the project..."
cargo build --release

echo "Preparing the package structure..."
mkdir -p packaging/usr/bin
mkdir -p target/deb
cp target/release/batteries packaging/usr/bin/
chmod 755 packaging/usr/bin/batteries

echo "Building the .deb package..."
dpkg-deb --build packaging target/deb/batteries_0.1.0_amd64.deb
echo "Package created: target/deb/batteries_0.1.0_amd64.deb"

