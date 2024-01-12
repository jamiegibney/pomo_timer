version=$(sed -n 's/^version = "*"//p' Cargo.toml | tr -d '"')

tar_dir="./builds/$version"
mkdir $tar_dir

# intel version
echo "\nBuilding Intel binary..."
cargo build --release --target-dir=target
mv ./target/release/pomo $tar_dir/pomo
zip "$tar_dir/pomo_macos_intel_$version.zip" $tar_dir/pomo
rm -rf $tar_dir/pomo

# arm version
echo "\nBuilding Universal binary..."
cargo build --release --target-dir=target --target=aarch64-apple-darwin
mv ./target/aarch64-apple-darwin/release/pomo $tar_dir/pomo
zip "$tar_dir/pomo_macos_universal_$version.zip" $tar_dir/pomo
rm -rf $tar_dir/pomo

echo "\nCreated binaries for version v$version in $tar_dir/"
open $tar_dir
