run:
  cargo run --bin synth-rt

run-term:
  cargo run --bin synth-term

trimui-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/aarch64" PKG_CONFIG_PATH="$PWD/cross-build-deps/aarch64/usr/lib/pkgconfig/" cargo build --target aarch64-unknown-linux-gnu --bin synth-console
  adb push ./target/aarch64-unknown-linux-gnu/debug/synth-console /userdata/roms/ports/Synth/

surface-build:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-rt
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-rt -r
  # PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo build --target armv7-unknown-linux-musleabihf --bin synth-rt -r

surface-build-term:
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-term
  PKG_CONFIG_SYSROOT_DIR="$PWD/cross-build-deps/armv7/" PKG_CONFIG_PATH="$PWD/cross-build-deps/armv7/usr/lib/pkgconfig/" cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.36 --bin synth-term -r

setup-aarch64:
  mkdir -p ./cross-build-deps/aarch64/
  # TODO: download dep archives stuff
  # TODO: extract
  # TODO: rm dep archive files

setup-armv7:
  # mkdir -p ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libx11-1.8.10-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/pkgconf-2.1.1-1-armv7h.pkg.tar.xz 
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/glibc-2.39+r52+gf8e4623421-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/alsa-lib-1.2.12-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/wayland-1.23.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/systemd-libs-256.7-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/gcc-libs-14.1.1+r1+g43b730b9134-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libcap-2.70-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libffi-3.4.6-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/openssl-3.4.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/gtk3-1:3.24.43-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/appmenu-gtk-module-24.05-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libappindicator-gtk3-12.10.0.r298-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/librsvg-2:2.59.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/cairo-1.18.2-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/glib2-2.82.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/pango-1:1.54.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/harfbuzz-10.0.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libxml2-2.13.4-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/zlib-1:1.3.1-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/xz-5.6.3-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/gdk-pixbuf2-2.42.12-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/at-spi2-atk-2.38.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libsoup3-3.6.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/at-spi2-core-2.54.0-2-armv7h.pkg.tar.xz
  # # wget -P ./cross-build-deps/armv7/ http://tardis.tiny-vps.com/aarm/packages/w/webkit2gtk-4.1/webkit2gtk-4.1-2.38.0-2-armv7h.pkg.tar.xz
  # # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/w/webkit2gtk/libwebkit2gtk-4.1-dev_2.46.2-1_armhf.deb
  # # cd ./cross-build-deps/armv7/; debtap -Q ./libwebkit2gtk-4*.deb; rm libwebkit2gtk-4*.deb
  # wget -P ./cross-build-deps/armv7/ http://security.debian.org/debian-security/pool/updates/main/w/webkit2gtk/libwebkit2gtk-4.1-dev_2.46.0-2~deb12u1_armhf.deb
  # cd ./cross-build-deps/armv7/; debtap -Q ./libwebkit2gtk-4*.deb; rm libwebkit2gtk-4*.deb
  wget -P ./cross-build-deps/armv7/ http://security.debian.org/debian-security/pool/updates/main/w/webkit2gtk/libwebkit2gtk-4.1-0_2.46.0-2~deb12u1_armhf.deb
  cd ./cross-build-deps/armv7/; debtap -Q ./libwebkit2gtk-4*.deb; rm libwebkit2gtk-4*.deb
  # wget -P ./cross-build-deps/armv7/ http://security.debian.org/debian-security/pool/updates/main/w/webkit2gtk/webkit2gtk-driver_2.46.0-2~deb12u1_armhf.deb
  # cd ./cross-build-deps/armv7/; debtap -Q ./webkit2gtk*; rm webkit2gtk*
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/fontconfig-2:2.15.0-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxkbcommon-1.7.0-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/dbus-1.14.10-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/fribidi-1.0.16-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libepoxy-1.5.10-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxi-1.8.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libcloudproviders-0.3.6-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/tinysparql-3.8.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxfixes-6.0.1-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxext-1.3.6-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxcursor-1.2.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxdamage-1.1.6-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxcomposite-0.4.6-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxrandr-1.5.4-1-armv7h.pkg.tar.xz 
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxinerama-1.1.5-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libthai-0.1.29-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libpng-1.6.44-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/freetype2-2.13.3-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxrender-0.9.11-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxcb-1.17.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/pixman-0.43.4-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libjpeg-turbo-3.0.4-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/aarch64/extra/libtiff-4.7.0-1-aarch64.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/sqlite-3.46.1-1-armv7h.pkg.tar.xz 
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libpsl-0.21.5-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/brotli-1.1.0-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libnghttp2-1.64.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/pam_mount-2.20-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/pcre2-10.44-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/
  # wget -P ./cross-build-deps/armv7/ 
  cd ./cross-build-deps/armv7; ls *.pkg.tar.*; for f in $(ls *.pkg.tar.*); do tar xf ./$f; rm $f; done
  cp -r cross-build-deps/armv7/usr/lib/arm-linux-gnueabihf/* cross-build-deps/armv7/usr/lib/

surface-transfer:
  python -m http.server -d target/armv7-unknown-linux-gnueabihf/debug/ 8080

get-font:
  mkdir -p assets/fonts
  wget -P ./assets/fonts https://www.marksimonson.com/assets/content/fonts/AnonymousPro-1_002.zip
  cd ./assets/fonts && unzip *.zip
  mv ./assets/fonts/AnonymousPro-1.002.001/*.ttf ./assets/fonts/
  rm -r ./assets/fonts/AnonymousPro-1.002.001/
  rm ./assets/fonts/*.zip
