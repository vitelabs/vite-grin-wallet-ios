cd rust
cargo lipo --targets aarch64-apple-ios x86_64-apple-ios armv7-apple-ios armv7s-apple-ios --release
cd ..
#cp rust/target/aarch64-apple-ios/release/libwallet.a Vite_GrinWallet/Library/grin_aarch64-apple-ios.a
#cp rust/target/armv7-apple-ios/release/libwallet.a Vite_GrinWallet/Library/grin_armv7-apple-ios.a
#cp rust/target/armv7s-apple-ios/release/libwallet.a Vite_GrinWallet/Library/grin_armv7s-apple-ios.a
#cp rust/target/x86_64-apple-ios/release/libwallet.a Vite_GrinWallet/Library/grin_x86_64-apple-ios.a
cp rust/target/universal/release/libwallet.a Vite_GrinWallet/Library/libwallet.a
echo "build grin ios lib successfully"
