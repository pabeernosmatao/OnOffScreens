# OnOffScreens 2.1.0 — AppImages

Dos versiones para **x86_64**, ambas generadas desde el mismo código:

| Versión | Tamaño | Librerías | Recomendada para |
|---|---|---|---|
| `OnOffScreens_2.1.0_amd64-full.AppImage` | ~78 MB | Incluye GTK, WebKitGTK y bandeja | Cualquier distro, incluida Ubuntu 22.04 |
| `OnOffScreens_2.1.0_amd64-portable.AppImage` | ~3,3 MB | Usa GTK/WebKitGTK del sistema; incluye la bandeja | Arch/CachyOS y sistemas actualizados |

Las dos están compiladas sobre Ubuntu 22.04 y requieren como máximo **glibc 2.35**.

> Las releases oficiales se generan en Ubuntu 22.04 mediante GitHub Actions. Si
> compilas las AppImages localmente en una distribución más nueva, pueden no
> arrancar en Ubuntu 22.04 u otros sistemas con glibc más antigua.

## Ejecutar

```bash
chmod +x OnOffScreens_2.1.0_amd64-full.AppImage
./OnOffScreens_2.1.0_amd64-full.AppImage
```

o la ligera:

```bash
chmod +x OnOffScreens_2.1.0_amd64-portable.AppImage
./OnOffScreens_2.1.0_amd64-portable.AppImage
```

Solo el icono de bandeja:

```bash
./OnOffScreens_2.1.0_amd64-full.AppImage --tray
```

## Requisitos

Las dos necesitan **KDE Plasma** y `kscreen` (`kscreen-doctor`) para detectar y
aplicar perfiles.

La versión **portable/ligera** necesita además las librerías del sistema:

- Arch/CachyOS: `sudo pacman -S webkit2gtk-4.1 gtk3`
- Ubuntu/Debian: `sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0`
- Fedora: `sudo dnf install webkit2gtk4.1 gtk3`

La bandeja (`libayatana-appindicator` y sus dependencias) va incluida en ambas.

En Ubuntu 24.04, si no tienes FUSE: `sudo apt install libfuse2t64` o ejecuta
el AppImage con `--appimage-extract-and-run`.

## Verificar

```bash
sha256sum -c OnOffScreens_2.1.0_amd64-full.AppImage.sha256
sha256sum -c OnOffScreens_2.1.0_amd64-portable.AppImage.sha256
```
