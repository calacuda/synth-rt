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
  # # cd ./cross-build-deps/armv7/; debtap -Q ./libwebkit2gtk-4*.deb; rm libwebkit2gtk-4*.deb
  # wget -P ./cross-build-deps/armv7/ http://security.debian.org/debian-security/pool/updates/main/w/webkit2gtk/libwebkit2gtk-4.1-0_2.46.0-2~deb12u1_armhf.deb
  # # cd ./cross-build-deps/armv7/; debtap -Q ./libwebkit2gtk-4*.deb; rm libwebkit2gtk-4*.deb
  # wget -P ./cross-build-deps/armv7/ http://security.debian.org/debian-security/pool/updates/main/w/webkit2gtk/webkit2gtk-driver_2.46.0-2~deb12u1_armhf.deb
  # # cd ./cross-build-deps/armv7/; debtap -Q ./webkit2gtk*; rm webkit2gtk*
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
  # # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libjpeg-turbo-3.0.4-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/sqlite-3.46.1-1-armv7h.pkg.tar.xz 
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libpsl-0.21.5-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/brotli-1.1.0-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libnghttp2-1.64.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/pam_mount-2.20-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/pcre2-10.44-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libtiff-4.7.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/krb5-1.21.3-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/glibmm-2.66.7-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/util-linux-libs-2.40.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/icu-75.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libwebp-1.4.0-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/giflib-5.2.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libjpeg6-turbo-1.5.3-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxslt-1.1.42-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libgcrypt-1.11.0-2-armv7h.pkg.tar.xz 
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libgpg-error-1.50-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/lcms2-2.16-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/woff2-1.0.2-5-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/gst-plugins-bad-libs-1.24.8-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/gst-plugins-base-libs-1.24.8-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/gstreamer-1.24.8-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libelf-0.192-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libunwind-1.8.1-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/i/icu/libicu-dev_72.1-3_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/i/icu/libicu72_72.1-3_armhf.deb
  # # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libavif-1.1.1-2-armv7h.pkg.tar.xz 
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libyuv-r2426+464c51a03-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/dav1d-1.4.3-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/aom-3.10.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://security.debian.org/debian-security/pool/updates/main/w/webkit2gtk/libjavascriptcoregtk-4.1-dev_2.46.0-2~deb12u1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://security.debian.org/debian-security/pool/updates/main/w/webkit2gtk/libjavascriptcoregtk-4.1-0_2.46.0-2~deb12u1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/harfbuzz-icu-10.0.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/liba/libavif/libavif-dev_0.11.1-1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/liba/libavif/libavif15_0.11.1-1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/graphite-1:1.3.14-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/enchant-2.8.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libsecret-0.21.4-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/tpm2-tss-4.1.3-1-armv7h.pkg.tar.xz 
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/json-c-0.18-1-armv7h.pkg.tar.xz
  # # # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/curl-8.10.1-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libtasn1-4.19.0-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/hyphen-2.8.8-6-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/hyphen-en-2.8.8-6-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libmanette-0.2.9-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libgudev-238-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libevdev-1.13.3-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libseccomp-2.5.5-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/egl-gbm-1.1.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/mesa-1:24.2.5-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libdrm-2.4.123-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libpciaccess-0.18.1-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/eglexternalplatform-1.2-2-any.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/expat-2.6.3-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/json-glib-1.10.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libdatrie-0.2.13-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/bzip2-1.0.8-6-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxau-1.0.11-3-armv7h.pkg.tar.xz
  # # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/aarch64/extra/xorgproto-2024.1-2-any.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxdmcp-1.1.5-1.1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/zstd-1.5.6-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/lz4-1:1.10.0-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/jbig2dec-0.20-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/jbigkit-2.1-8-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libunistring-1.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libidn2-2.3.7-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/e2fsprogs-1.47.1-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/keyutils-1.6.3-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/orc-0.4.40-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/gegl-0.4.48-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/egl-gbm-1.1.2-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/egl-wayland-4:1.1.16-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/eglexternalplatform-1.2-2-any.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/freeglut-3.6.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/glu-9.0.3-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/d/dav1d/dav1d_1.0.0-2+deb12u1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/d/dav1d/libdav1d-dev_1.0.0-2+deb12u1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/d/dav1d/libdav1d6_1.0.0-2+deb12u1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/rav1e-0.7.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/chafa-1.14.2-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/harfbuzz-cairo-10.0.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/ffmpeg-2:7.0.2-3.1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libglvnd-1.7.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxext-1.3.6-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/libg/libgav1/libgav1-1_0.18.0-1+b1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/libg/libgav1/libgav1-dev_0.16.0-5_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/llvm-18.1.8-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/llvm-libs-18.1.8-4-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/libxshmfence-1.3.2-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/lm_sensors-1:3.6.0.r41.g31d1f125-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/s/svt-av1/svt-av1_1.4.1+dfsg-1_armhf.deb
  # # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/abseil-cpp-20240722.0-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/a/abseil/libabsl20220623_20220623.1-1_armhf.deb
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/libedit-20240517_3.1-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/core/ncurses-6.5-3-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/s/svt-av1/libsvtav1-dev_1.4.1+dfsg-1_all.deb
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/gst-libav-1.24.8-1-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/aarch64/extra/ffnvcodec-headers-12.2.72.0-1-any.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://mirror.archlinuxarm.org/armv7h/extra/srt-1.5.3-2-armv7h.pkg.tar.xz
  # wget -P ./cross-build-deps/armv7/ http://http.us.debian.org/debian/pool/main/s/svt-av1/libsvtav1enc1_1.4.1+dfsg-1_armhf.deb
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
  # wget -P ./cross-build-deps/armv7/ 
  # wget -P ./cross-build-deps/armv7/ 
  # wget -P ./cross-build-deps/armv7/ 
  cd ./cross-build-deps/armv7/ && ls *.deb; \
  for f in $(ls *.deb); do \
    debtap -Q ./$f &&  rm $f; \
  done
  cd ./cross-build-deps/armv7 && ls *.pkg.tar.*; \
  for f in $(ls *.pkg.tar.*); do \
    tar xf ./$f && rm $f; \
  done
  cp -r cross-build-deps/armv7/usr/lib/arm-linux-gnueabihf/* cross-build-deps/armv7/usr/lib/
  ln -f -s libyuv.so cross-build-deps/armv7/usr/lib/libyuv.so.0
  ln -f -s librav1e.so.0.7 ./cross-build-deps/armv7/usr/lib/librav1e.so.0
  # ln -f -s libabsl_synchronization.so ./cross-build-deps/armv7/usr/lib/libabsl_synchronization.so.20220623
  ln -f -s arm-linux-gnueabihf/ ./cross-build-deps/armv7/usr/lib/arm-linux-musleabihf

surface-transfer:
  python -m http.server -d target/armv7-unknown-linux-gnueabihf/debug/ 8080

get-font:
  mkdir -p assets/fonts
  wget -P ./assets/fonts https://www.marksimonson.com/assets/content/fonts/AnonymousPro-1_002.zip
  cd ./assets/fonts && unzip *.zip
  mv ./assets/fonts/AnonymousPro-1.002.001/*.ttf ./assets/fonts/
  rm -r ./assets/fonts/AnonymousPro-1.002.001/
  rm ./assets/fonts/*.zip
