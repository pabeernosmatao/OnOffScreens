#!/usr/bin/env bash
# Genera las dos AppImage de release a partir del bundle "full" de Tauri:
#   - OnOffScreens_<version>_amd64-full.AppImage
#   - OnOffScreens_<version>_amd64-portable.AppImage
#
# La full la produce `npm run appimage` (Tauri + linuxdeploy). La portable se
# reconstruye a partir de la full dejando fuera GTK/WebKitGTK para usar los del
# sistema y conservando la pila de la bandeja.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="${VERSION:-$(node -p "require('./package.json').version")}"
ARCH="amd64"
BUNDLE_DIR="$ROOT/src-tauri/target/release/bundle/appimage"
FULL_SRC="$BUNDLE_DIR/OnOffScreens_${VERSION}_${ARCH}.AppImage"
if [ ! -f "$FULL_SRC" ]; then
  ALT_SRC="$BUNDLE_DIR/OnOffScreens_${VERSION}_x86_64.AppImage"
  if [ -f "$ALT_SRC" ]; then
    FULL_SRC="$ALT_SRC"
  fi
fi
RELEASE_DIR="${RELEASE_DIR:-$ROOT/release}"
FULL_OUT="$RELEASE_DIR/OnOffScreens_${VERSION}_${ARCH}-full.AppImage"
PORTABLE_OUT="$RELEASE_DIR/OnOffScreens_${VERSION}_${ARCH}-portable.AppImage"

if [ ! -f "$FULL_SRC" ]; then
  echo "No se encontró $FULL_SRC" >&2
  echo "Ejecuta antes: npm run tauri build -- --bundles appimage" >&2
  exit 1
fi

mkdir -p "$RELEASE_DIR"
cp "$FULL_SRC" "$FULL_OUT"
chmod +x "$FULL_OUT"

# appimagetool incluye mksquashfs y zsyncmake, así que no hacen falta paquetes
# extra del sistema. Se ejecuta con APPIMAGE_EXTRACT_AND_RUN para no depender
# de FUSE.
TOOLS_DIR="${TOOLS_DIR:-$ROOT/src-tauri/target/tools}"
APPIMAGETOOL="$TOOLS_DIR/appimagetool-x86_64.AppImage"
mkdir -p "$TOOLS_DIR"
if [ ! -x "$APPIMAGETOOL" ]; then
  echo "Descargando appimagetool…"
  curl -L --fail --retry 3 -o "$APPIMAGETOOL" \
    https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage
  chmod +x "$APPIMAGETOOL"
fi

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

(
  cd "$WORK"
  "$FULL_OUT" --appimage-extract >/dev/null
)
APPDIR="$WORK/squashfs-root"
chmod -R a+rX "$APPDIR"

# La portable usa GTK y WebKitGTK del sistema. Solo se empaqueta la bandeja.
KEEP_LIBS="libayatana-appindicator3.so.1 libayatana-ido3-0.4.so.0 libayatana-indicator3.so.7 libdbusmenu-glib.so.4 libdbusmenu-gtk3.so.4"
for entry in "$APPDIR"/usr/lib/*; do
  [ -e "$entry" ] || continue
  base="$(basename "$entry")"
  keep=0
  for lib in $KEEP_LIBS; do
    if [ "$base" = "$lib" ]; then
      keep=1
      break
    fi
  done
  if [ "$keep" -eq 0 ]; then
    rm -rf "$entry"
  fi
done

rm -rf "$APPDIR/apprun-hooks" "$APPDIR/AppRun.wrapped"
rm -rf "$APPDIR/usr/share/glib-2.0" "$APPDIR/usr/share/doc"

cat > "$APPDIR/AppRun" <<'EOF'
#!/bin/sh
HERE="$(dirname "$(readlink -f "$0")")"
export LD_LIBRARY_PATH="$HERE/usr/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
exec "$HERE/usr/bin/onoffscreens" "$@"
EOF
chmod +x "$APPDIR/AppRun"

APPIMAGE_EXTRACT_AND_RUN=1 ARCH=x86_64 "$APPIMAGETOOL" \
  --no-appstream "$APPDIR" "$PORTABLE_OUT"
chmod +x "$PORTABLE_OUT"

(
  cd "$RELEASE_DIR"
  sha256sum "OnOffScreens_${VERSION}_${ARCH}-full.AppImage" \
    > "OnOffScreens_${VERSION}_${ARCH}-full.AppImage.sha256"
  sha256sum "OnOffScreens_${VERSION}_${ARCH}-portable.AppImage" \
    > "OnOffScreens_${VERSION}_${ARCH}-portable.AppImage.sha256"
)

ls -lh "$FULL_OUT" "$PORTABLE_OUT"
