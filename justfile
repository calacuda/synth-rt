run:
  cargo run --bin synth-rt

run-term:
  cargo run --bin synth-term

trimui-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/aarch64" PKG_CONFIG_PATH="$PWD/cross-build-deps/aarch64/usr/lib/pkgconfig/" cargo build --target aarch64-unknown-linux-gnu --bin synth-console
  adb push ./target/aarch64-unknown-linux-gnu/debug/synth-console /userdata/roms/ports/Synth/

surface-build:
  # PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo build --target armv7-unknown-linux-gnueabihf --bin synth-rt -r
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo build --target armv7-unknown-linux-musleabihf --bin synth-rt -r

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
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/wayland-1.23.1-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/systemd-libs-256.7-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/gcc-libs-14.1.1+r1+g43b730b9134-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libcap-2.70-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libffi-3.4.6-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/musl-1.2.5-2-armv7h.pkg.tar.xz
  cd ./cross-build-deps/armv7; for f in $(ls *.pkg.tar.xz); do tar xf $f; rm $f; done

surface-transfer:
  python -m http.server -d target/armv7-unknown-linux-gnueabihf/debug/ 8080
