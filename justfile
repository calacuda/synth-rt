run:
  cargo run --bin synth-rt

run-term:
  cargo run --bin synth-terminal

trimui-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/aarch64" PKG_CONFIG_PATH="$PWD/cross-build-deps/aarch64/usr/lib/pkgconfig/" cargo build --target aarch64-unknown-linux-gnu --bin synth-console
  adb push ./target/aarch64-unknown-linux-gnu/debug/synth-console /userdata/roms/ports/Synth/

surface-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/i686/" PKG_CONFIG_PATH="$PWD/cross-build-deps/i686/usr/lib/pkgconfig/" cargo build --target i686-unknown-linux-gnu --bin synth-rt

setup-aarch64:
  mkdir -p ./cross-build-deps/aarch64/
  # TODO: download dep archives stuff
  # TODO: extract
  # TODO: rm dep archive files

