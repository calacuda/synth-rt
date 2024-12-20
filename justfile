run:
  cargo run --bin synth-rt

run-term:
  cargo run --bin synth-term

trimui-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/aarch64" PKG_CONFIG_PATH="$PWD/cross-build-deps/aarch64/usr/lib/pkgconfig/" cargo zigbuild --target aarch64-unknown-linux-gnu.2.38 --bin synth-rt -r
  adb push ./target/aarch64-unknown-linux-gnu/release/synth-rt /userdata/roms/ports/Synth/

surface-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-rt
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-rt -r

surface-build-term:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-term
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-term -r

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
  # wget -P ./cross-build-deps/armv7/ http://tardis.tiny-vps.com/aarm/packages/g/glibc/glibc-2.35-5.1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/alsa-lib-1.2.12-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/wayland-1.23.1-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/systemd-libs-256.7-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/gcc-libs-14.1.1+r1+g43b730b9134-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libcap-2.70-1-armv7h.pkg.tar.xz
  wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libffi-3.4.6-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/musl-1.2.5-2-armv7h.pkg.tar.xz
  cd ./cross-build-deps/armv7; for f in $(ls *.pkg.tar.xz); do echo "extracting archiver: $f"; tar xf $f; rm $f; done

surface-transfer:
  python -m http.server -d target/armv7-unknown-linux-gnueabihf/debug/ 8080

get-font:
  mkdir -p assets/fonts
  wget -P ./assets/fonts https://www.marksimonson.com/assets/content/fonts/AnonymousPro-1_002.zip
  cd ./assets/fonts && unzip *.zip
  mv ./assets/fonts/AnonymousPro-1.002.001/*.ttf ./assets/fonts/
  rm -r ./assets/fonts/AnonymousPro-1.002.001/
  rm ./assets/fonts/*.zip

new-window NAME CMD:
  tmux new-w -t synth-rt -n "{{NAME}}"
  tmux send-keys -t synth-rt:"{{NAME}}" "{{CMD}}" ENTER

tmux:
  tmux new -ds synth-rt -n "README"
  tmux send-keys -t synth-rt:README 'nv ./README.md "+set wrap"' ENTER
  @just new-window "Edit" ""
  @just new-window "Run" ""
  @just new-window "git" "git status"
  tmux a -t synth-rt
