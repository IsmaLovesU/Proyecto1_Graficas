# Raycaster Laberinto

Ray caster en primera persona escrito en Rust. Usa DDA para trazar rayos y raylib para la ventana y el audio.

## Controles

| Tecla | Acción |
|---|---|
| Flecha arriba / abajo | Avanzar / retroceder |
| Flecha izquierda / derecha | Rotar |
| Mouse (horizontal) | Rotar |
| ENTER | Comenzar / reiniciar |
| ESC | Salir |

## Objetivo

Llega a la celda marcada con `g` en el mapa. Las llamas son decorativas — no hacen daño.

## Compilar y ejecutar

```
cargo run --release
```

El ejecutable espera estar en la raíz del repositorio para encontrar `maze.txt` y la carpeta `assets/`.

## Recursos necesarios

Todos los siguientes archivos deben existir antes de correr el juego:

### Texturas — `assets/textures/`
- `pared1.png` — paredes verticales `|`
- `pared2.png` — paredes horizontales `-`
- `pared3.png` — esquinas `+`
- `puerta final.png` — celda meta `g` (el espacio en el nombre es intencional)

### Sprites — `assets/sprites/`
- `fuego_1.png` a `fuego_8.png` — frames de la llama animada (PNG con canal alfa)

### Audio — `assets/audio/`
- `pasos.ogg` — efecto de pasos al caminar
- `victoria.ogg` — jingle al llegar a la meta
- `bgm.ogg` — música de fondo (**opcional**; si no existe el juego arranca sin música)

`bgm.ogg` está excluido del repositorio por derechos de autor. Coloca cualquier pista OGG
con ese nombre en `assets/audio/` y el juego la reproducirá en bucle.

### Imágenes de pantalla — `assets/images/`
- `imagenBienvenida.png` — pantalla de bienvenida
- `imagenExito.png` — pantalla de éxito

## Arquitectura

```
src/
  main.rs           constantes globales y punto de entrada
  app/
    loop.rs         bucle principal y maquina de estados
    stage.rs        enum Stage { Welcome, Playing, Success }
    clock.rs        delta time acumulado
  world/
    grid.rs         Tile, Grid, carga del laberinto ASCII
    actor.rs        posicion, rotacion, colision con radio
  render/
    framebuffer.rs  buffer RGBA volcado a GPU por textura
    dda.rs          DDA — trazado de rayos (cast_ray)
    columns.rs      paredes texturizadas + z-buffer
    billboard.rs    sprites de fuego con recorte por z-buffer
    overlay.rs      minimapa en esquina inferior derecha
  media/
    atlas.rs        texturas de pared y frames del fuego
    jukebox.rs      pasos, victoria y musica de fondo
```

## Requisitos

- Rust 1.85 o superior (`rustup update stable`)
- En Windows: toolchain MSVC (instalado con `rustup`)
