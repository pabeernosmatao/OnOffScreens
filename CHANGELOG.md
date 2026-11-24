# Changelog

## 2.1.0 — 2026-11-24

### Añadido

- Selector de idioma en *Opciones*: automático, español, catalán e inglés.
  El modo automático detecta variantes de `es`, variantes de `ca` y usa inglés
  para el resto.
- Traducción completa al catalán de la interfaz, bandeja, notificaciones y CLI.
- Acción **Reconfigurar pantallas** en la bandeja del sistema y en *Opciones →
  Avanzado* para recuperar la imagen cuando los colores se ven mal tras iniciar
  sesión.
- La versión de la aplicación se muestra junto al nombre en la cabecera.
- La tarjeta de pantallas detectadas se puede plegar y recuerda su estado.
- Las AppImages de release se generan también con un flujo de GitHub Actions.

### Cambiado

- Interfaz más compacta (zoom global del 88 %) para escritorios de 1080p y 1440p.
- Al aplicar un perfil solo se envían los cambios necesarios: no se vuelven a
  encender pantallas ya encendidas ni se recolocan las que ya están en su sitio.
- Si el perfil ya coincide con el estado actual no se llama a `kscreen-doctor`.
- Todas las llamadas a `kscreen-doctor` se serializan para evitar lecturas y
  aplicaciones simultáneas.
- La comprobación posterior a aplicar un perfil es más ligera y acotada.
- El refresco automático de la interfaz se pausa cuando la ventana está oculta.
- Sin sondeo en segundo plano: mientras la app está en la bandeja no se consulta
  KScreen. El menú de la bandeja se construye solo con los perfiles y ajustes
  locales, y el estado de pantallas se lee al abrir la ventana, aplicar un perfil
  o ejecutar una acción.

### Corregido

- Se reduce el riesgo de bloqueo de KWin al cambiar de perfil en configuraciones
  con HDR y alta frecuencia de refresco.
