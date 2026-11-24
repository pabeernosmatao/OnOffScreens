#!/bin/bash
# Desinstalador de "pantallas"
# Uso:  ./desinstalar.sh [--borrar-datos]
set -e

BIN="${HOME}/.local/bin"
DATA="${XDG_DATA_HOME:-$HOME/.local/share}"
BORRAR="no"

[ "$1" = "--borrar-datos" ] && BORRAR="si"

pkill -f "bin/pantallas tray" 2>/dev/null || true
sleep 1

rm -f "$BIN/pantallas"
rm -f "$DATA/applications/pantallas-perfiles.desktop"
rm -f "$HOME/.config/autostart/pantallas-tray.desktop"
find "$DATA/icons/hicolor" -name "pantallas-perfiles.*" -delete 2>/dev/null || true

if [ "$BORRAR" = "si" ]; then
  rm -rf "$HOME/.config/pantallas"
  rm -rf "${XDG_STATE_HOME:-$HOME/.local/state}/pantallas"
  echo "Configuración y mapa de pantallas eliminados."
else
  echo "Se conservan tus perfiles (~/.config/pantallas) y el mapa de pantallas"
  echo "(~/.local/state/pantallas). Para borrarlos también:"
  echo "  rm -rf ~/.config/pantallas ~/.local/state/pantallas"
fi

gtk-update-icon-cache -f -t "$DATA/icons/hicolor" >/dev/null 2>&1 || true
command -v update-desktop-database >/dev/null && update-desktop-database "$DATA/applications" >/dev/null 2>&1 || true
command -v kbuildsycoca6 >/dev/null && kbuildsycoca6 --noincremental >/dev/null 2>&1 || true

# si se está ejecutando la copia instalada, se borra a sí misma
rm -f "$BIN/pantallas-desinstalar" 2>/dev/null || true

echo "Desinstalado."
