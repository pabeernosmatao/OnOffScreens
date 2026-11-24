#!/bin/bash
# Instalador de "pantallas" (perfiles de monitores para KDE Plasma)
# Uso:  ./instalar.sh [--sin-autostart]
set -e

SRC="$(cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")" && pwd)"
BIN="${HOME}/.local/bin"
DATA="${XDG_DATA_HOME:-$HOME/.local/share}"
AUTOSTART="si"

for arg in "$@"; do
  case "$arg" in
    --sin-autostart) AUTOSTART="no" ;;
    -h|--help)
      echo "Uso: $0 [--sin-autostart]"
      exit 0
      ;;
  esac
done

echo "==> Comprobando requisitos"
command -v kscreen-doctor >/dev/null || {
  echo "ERROR: falta kscreen-doctor (paquete 'kscreen')." >&2
  exit 1
}
command -v python3 >/dev/null || {
  echo "ERROR: falta python3." >&2
  exit 1
}
python3 - <<'PY' || echo "  aviso: sin python-gobject/libappindicator el icono de bandeja no funcionará"
import gi
gi.require_version("Gtk", "3.0")
gi.require_version("AppIndicator3", "0.1")
from gi.repository import AppIndicator3  # noqa
PY
command -v kdialog >/dev/null || echo "  aviso: falta kdialog (menú con ventana)"

echo "==> Instalando programa en $BIN"
mkdir -p "$BIN"
install -m 755 "$SRC/pantallas" "$BIN/pantallas"
install -m 755 "$SRC/desinstalar.sh" "$BIN/pantallas-desinstalar"

echo "==> Instalando iconos"
mkdir -p "$DATA/icons/hicolor"
cp -r "$SRC/iconos/hicolor/." "$DATA/icons/hicolor/"

echo "==> Creando lanzador del menú"
mkdir -p "$DATA/applications"
sed "s|__EXEC__|$BIN/pantallas|g" "$SRC/plantillas/pantallas-perfiles.desktop" \
  > "$DATA/applications/pantallas-perfiles.desktop"

if [ "$AUTOSTART" = "si" ]; then
  echo "==> Activando arranque automático del icono de bandeja"
  mkdir -p "$HOME/.config/autostart"
  sed "s|__EXEC__|$BIN/pantallas|g" "$SRC/plantillas/autostart.desktop" \
    > "$HOME/.config/autostart/pantallas-tray.desktop"
fi

echo "==> Perfiles"
if [ -f "$HOME/.config/pantallas/perfiles.conf" ]; then
  echo "    ya existe ~/.config/pantallas/perfiles.conf (se conserva)"
else
  "$BIN/pantallas" init
fi

echo "==> Actualizando cachés"
gtk-update-icon-cache -f -t "$DATA/icons/hicolor" >/dev/null 2>&1 || true
command -v update-desktop-database >/dev/null && update-desktop-database "$DATA/applications" >/dev/null 2>&1 || true
command -v kbuildsycoca6 >/dev/null && kbuildsycoca6 --noincremental >/dev/null 2>&1 || true

echo
echo "Listo. Resumen:"
echo "  programa     : $BIN/pantallas"
echo "  desinstalar  : $BIN/pantallas-desinstalar"
echo "  perfiles     : $HOME/.config/pantallas/perfiles.conf"
echo "  mapa pantallas: ${XDG_STATE_HOME:-$HOME/.local/state}/pantallas/mapa.json"
echo "  lanzador     : $DATA/applications/pantallas-perfiles.desktop"
[ "$AUTOSTART" = "si" ] && echo "  autostart    : $HOME/.config/autostart/pantallas-tray.desktop"
echo
echo "Ahora:"
echo "  1) $BIN/pantallas mapa      # comprueba qué número es cada pantalla"
echo "  2) $BIN/pantallas status    # estado y perfil actual"
echo "  3) cierra sesión y entra, o lanza:  $BIN/pantallas tray &"
