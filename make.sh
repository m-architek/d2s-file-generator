cargo build --release \
  --target x86_64-pc-windows-gnu \
  --target i686-pc-windows-gnu \
  --target x86_64-unknown-linux-gnu \
  --target i686-unknown-linux-gnu &&

cp ./target/x86_64-pc-windows-gnu/release/d2s-file-generator.exe ./target/d2s-file-generator_x86_64.exe &&
cp ./target/i686-pc-windows-gnu/release/d2s-file-generator.exe ./target/d2s-file-generator_i686.exe &&
cp ./target/x86_64-unknown-linux-gnu/release/d2s-file-generator ./target/d2s-file-generator_x86_64 &&
cp ./target/i686-unknown-linux-gnu/release/d2s-file-generator ./target/d2s-file-generator_i686;
