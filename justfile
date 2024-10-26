run:
  cargo run --bin synth-rt

run-term:
  cargo run --bin synth-term

trimui-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/aarch64" PKG_CONFIG_PATH="$PWD/cross-build-deps/aarch64/usr/lib/pkgconfig/" cargo build --target aarch64-unknown-linux-gnu --bin synth-console
  adb push ./target/aarch64-unknown-linux-gnu/debug/synth-console /userdata/roms/ports/Synth/

surface-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo build --target armv7-unknown-linux-gnueabihf --bin synth-rt

setup-aarch64:
  mkdir -p ./cross-build-deps/aarch64/
  # TODO: download dep archives stuff
  # TODO: extract
  # TODO: rm dep archive files

setup-armv7:
  mkdir -p ./cross-build-deps/armv7/
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libx11-1.8.10-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/pkgconf-2.1.1-1-armv7h.pkg.tar.xz 
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/glibc-2.39+r52+gf8e4623421-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/alsa-lib-1.2.12-1-armv7h.pkg.tar.xz
  cd ./cross-build-deps/armv7; for f in $(ls *.pkg.tar.xz); do tar xf $f; done

