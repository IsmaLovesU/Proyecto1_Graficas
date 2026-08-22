# Instrucciones de reescritura del ray caster

Este documento es el plan de trabajo. Léelo completo antes de escribir una sola línea de código.

El repositorio actual ya tiene un ray caster funcionando. **No lo tomes como base a la que se le
hacen parches.** El trabajo es reescribirlo: otra arquitectura, otros nombres, otro reparto de
responsabilidades entre archivos, otro estilo de comentarios. El resultado debe leerse como el
proyecto de una persona distinta que resolvió el mismo problema por su cuenta.

Renombrar variables y mover archivos de carpeta no basta. Lo que hace que un proyecto sea otro
es que el algoritmo y la estructura sean otros. Las dos cosas van juntas y las dos son
obligatorias.

---

## 0. Antes de tocar nada

Recorre el código actual y hazme un inventario corto:

- Qué módulos existen y qué hace cada uno
- Cómo está implementado el trazado de rayos (pasos fijos, DDA, otro)
- Cómo se representa el laberinto en memoria
- Qué puntos de la rúbrica (sección 8) ya están cubiertos y cuáles faltan
- Si hay sprites implementados o no

Espera mi visto bueno antes de pasar a la etapa 1.

---

## 1. Idioma y estilo general

Se mantiene el criterio del curso:

- **Identificadores en inglés**: `actor`, `cast_ray`, `tile_at`, `Framebuffer`.
- **Comentarios en español**, con tildes normales dentro del texto.
- Nunca mezclar idiomas dentro de un identificador (`cast_rayo`, `distancia_corregida`).
- `snake_case` para funciones y variables, `PascalCase` para tipos,
  `SCREAMING_SNAKE_CASE` para constantes, `snake_case` para módulos y archivos.
- `cargo fmt` con configuración por defecto, máximo 100 columnas.
- `cargo clippy` sin warnings. Si un lint no aplica, se silencia puntualmente con `#[allow(...)]`
  y una razón, nunca a nivel de crate.
- Nada de `unsafe`. Nada de `Rc`, `RefCell`, `Arc`, `Mutex`, macros propias, traits genéricos
  propios ni `async`.
- `f32` para todo lo geométrico, `usize` para índices, `i32` para coordenadas de pantalla.
- Nunca indexar la matriz del laberinto directamente: siempre `get()` con el caso fuera de rango
  manejado. Un rayo que se sale del mapa no puede tumbar el programa.
- Nada de `unwrap()` pelón. En la carga de assets, `expect` o `unwrap_or_else` con un mensaje que
  diga qué archivo falta y dónde debería estar.

---

## 2. Comentarios: pocos y naturales

Esta es una de las partes importantes y va en contra de lo que hace el código actual.

**Regla general: menos comentarios, mejor escritos.** Un archivo con tres comentarios buenos se
lee como trabajo de una persona. Un archivo con veinte, uno por función, se lee como generado.

Qué sí lleva comentario:

- Las fórmulas de gráficas y el porqué de cada una: la corrección de ojo de pez, el cálculo de
  la altura de columna, la distancia de proyección, el salto del DDA, el z-buffer de sprites,
  el radio de colisión. Son los puntos donde alguien se atora leyendo.
- Decisiones que no se ven en el código: por qué el ángulo se guarda en radianes, por qué el
  frame del fuego avanza por tiempo y no por número de frame, por qué las filas del laberinto se
  duplican al cargar.

Qué no lleva comentario:

- Funciones cuyo nombre ya lo dice todo. `fn tile_at(&self, x: usize, y: usize)` no necesita
  que le expliquen que devuelve el tile en `x`, `y`.
- Getters, constructores triviales, `impl Default`.
- Cualquier comentario que repita la línea de abajo.

**Doc comments `///`:** solo donde el nombre no se explica solo. No pongas uno en cada `pub fn`
por cumplir. Los structs principales y las constantes con unidades sí los llevan.

**Cómo deben sonar.** Que varíen de largo: unos de media línea, otros de tres. Sin una fórmula
fija de apertura — nada de que todos empiecen con "Calcula...", "Devuelve...", "Se encarga
de...". Se permite el tono coloquial de alguien que está explicándole algo a sí mismo dentro de
seis meses. Nada de banners de asteriscos, separadores ASCII ni código muerto comentado.

Ejemplo del registro que busco:

```rust
// El coseno de la diferencia con el angulo del jugador quita el ojo de pez. Sin esto
// las paredes se curvan hacia los bordes porque los rayos laterales viajan mas lejos.
let depth = hit.distance * (ray_angle - actor.heading).cos();
```

Y de lo que no busco:

```rust
/// Calcula la distancia corregida.
/// # Argumentos
/// * `hit` - el impacto del rayo
let depth = hit.distance * (ray_angle - actor.heading).cos();
```

---

## 3. Nueva arquitectura de módulos

Deja `src/` así. La idea es que la división sea por capas (mundo / render / recursos / app), no
la típica lista plana de archivos al mismo nivel.

```
src/
  main.rs             solo arranca la ventana y entrega el control al bucle
  app/
    mod.rs
    loop.rs           bucle principal y despacho de eventos
    stage.rs          maquina de estados de pantallas
    clock.rs          delta time y contador de FPS
  world/
    mod.rs
    grid.rs           Tile, Grid, carga y parseo de disco
    actor.rs          jugador: pose, movimiento, colision
  render/
    mod.rs
    framebuffer.rs    buffer de pixeles y volcado a pantalla
    dda.rs            trazado de rayos
    columns.rs        dibujo de paredes texturizadas
    billboard.rs      sprites
    overlay.rs        minimapa y HUD
  media/
    mod.rs
    atlas.rs          texturas
    jukebox.rs        audio
```

Reglas que ya venían del curso y siguen aplicando: un módulo una responsabilidad, si un archivo
pasa de ~200 líneas algo debe mudarse, funciones de menos de 40 líneas, nada de `use super::*`,
`pub` solo en lo que otro módulo de verdad necesita.

Orden dentro de cada archivo: `use` (std, luego crates externos, luego módulos propios,
separados por línea en blanco), constantes, tipos, `impl`, funciones libres.

---

## 4. Renombrado

Salvo las excepciones de abajo, ningún identificador debe sobrevivir igual al del proyecto
anterior. Estos son los cambios mínimos; si encuentras otros nombres, cámbialos con el mismo
criterio.

**Se conservan tal cual `Framebuffer` y `cast_ray`.** Son los dos nombres estándar del tema, los
usa todo el mundo que escribe un ray caster y cambiarlos vuelve el código más difícil de leer,
no menos parecido. No los toques ni busques sinónimos para ellos.

| Antes (típico) | Ahora |
|---|---|
| `player` | `actor` |
| `Player` | `Actor` |
| `player.a` | `actor.heading` |
| `maze` | `grid` |
| `Maze` | `Grid` |
| `Intersect` / `RayHit` | `Contact` |
| `BLOCK_SIZE` | `TILE_UNITS` |
| `MOVE_SPEED` | `WALK_RATE` |
| `wall_height` | `column_span` |
| `GameState` | `Stage` |
| `draw_minimap` | `paint_overlay` |
| `render_world` | `paint_scene` |

Criterio para lo demás: los nombres describen qué es la cosa, no su tipo. `wall_distance`, no
`dist_f32`. Sin abreviaturas salvo las universales en gráficas (`fov`, `dx`, `dy`, `tx`, `ty`).

Cambia también el reparto de funciones, no solo sus nombres. Si antes una función hacía el rayo
y el dibujo de la columna, sepáralas. Si antes había tres funciones que se llamaban en cadena,
reorganízalas. Que el flujo de control no coincida.

---

## 5. Algoritmo: DDA en vez de pasos fijos

Si el trazador actual avanza el rayo sumando un incremento pequeño hasta chocar
(`distance += 0.1` dentro de un `while`), reemplázalo por **DDA**: el rayo salta de frontera de
celda a frontera de celda.

Por qué importa aquí, aparte de que corre mucho más rápido:

- Es el cambio estructural más grande que se le puede hacer al proyecto.
- El impacto queda exacto sobre la pared, no aproximado, así que la coordenada de textura sale
  limpia y no tiembla.
- Te dice gratis si el impacto fue en cara vertical u horizontal. Usa eso para oscurecer un
  poco las caras horizontales; da sensación de relieve y suma en "buena estética" sin costo.

Guarda la profundidad de cada columna en un `Vec<f32>` mientras dibujas paredes. Ese es el
z-buffer que va a necesitar el sprite.

---

## 6. El laberinto

### 6.1 Formato

Se cambia al formato de rejilla ASCII con `+`, `-`, `|`. Cada carácter es un tile:

| Carácter | Significado |
|---|---|
| `+` | esquina / poste — pared, textura 3 |
| `-` | pared horizontal — textura 2 |
| `\|` | pared vertical — textura 1 |
| ` ` | piso transitable |
| `p` | posición inicial del jugador |
| `g` | celda meta |

Los tres símbolos de pared cubren el requisito de la rúbrica de textura distinta por tipo de
pared, sin inventar marcadores extra.

### 6.2 Duplicación de filas al cargar

En este formato los pasillos miden 2 caracteres de ancho pero solo 1 de alto. Si lo cargas tal
cual, los pasillos este-oeste se ven al doble de anchos que los norte-sur y el laberinto se
siente aplastado.

**Al cargar, duplica cada fila de celda** (las que empiezan con `|` o espacio, es decir las de
índice impar). Las filas de junturas (`+--+--+`) se dejan tal cual. Con eso el archivo de 28×19
se convierte en un grid de 28×28 y todo queda a escala.

Consecuencias que hay que manejar:

- `p` aparece duplicado tras la operación. Toma la primera ocurrencia como spawn e ignora la
  segunda.
- `g` también se duplica. Trata ambas copias como meta; no molesta.
- Las paredes quedan de 1 tile de grosor y los pasillos de 2. Es lo correcto y se ve bien.

Esto va comentado en el cargador, porque es exactamente el tipo de decisión que no se adivina
leyendo el código.

### 6.3 `maze.txt`

Reemplaza el archivo actual en la raíz del proyecto por este. Son **19 líneas de 28 caracteres
cada una**, ya verificadas, y la meta es alcanzable desde el inicio.

```
+--+--+--+--+--+--+--+--+--+
|p |              |        |
+  +--+  +--+--+  +  +  +--+
|     |     |     |  |     |
+--+  +  +  +  +  +  +--+  +
|     |  |  |  |  |     |  |
+  +--+  +  +  +  +  +--+  +
|                    |     |
+--+  +--+  +--+--+--+  +--+
|  |     |                 |
+  +--+  +--+--+--+--+--+  +
|        |        |        |
+  +--+--+  +--+  +  +--+  +
|           |  |     |  |  |
+  +--+--+--+  +--+--+  +  +
|              |        |  |
+  +--+--+  +--+  +  +--+  +
|        |        |       g|
+--+--+--+--+--+--+--+--+--+
```

Comprueba después de escribirlo:

```bash
awk '{ print NR": "length($0) }' maze.txt
```

Las 19 líneas deben decir `28`. Si alguna dice otra cosa, el editor comió o agregó espacios al
final y el laberinto no va a cargar bien.

### 6.4 Representación en memoria

Nada de `Vec<Vec<char>>`. Usa un enum:

```rust
enum Tile {
    Solid(Surface),
    Floor,
    Spawn,
    Goal,
}
```

Donde `Surface` distingue las tres texturas de pared más la puerta. El `Grid` expone
`tile_at(x, y) -> Option<Tile>` y nadie fuera del módulo toca el almacenamiento interno.

---

## 7. Assets

Los nombres cambian. Actualiza todas las rutas del código.

### Texturas — `assets/textures/`

| Archivo | Uso |
|---|---|
| `pared1.png` | paredes `\|` |
| `pared2.png` | paredes `-` |
| `pared3.png` | paredes `+` |
| `puerta final.png` | celda meta `g` |

El nombre de la puerta lleva un espacio a propósito; escríbelo tal cual en la ruta.

### Sprite de fuego — `assets/sprites/`

Ocho frames, `fuego_1.png` a `fuego_8.png`. Salen de renombrar `Group 6 - 2_1.png` … `_8.png`
del pack de pixel fire.

### Audio — `assets/audio/`

| Archivo | Uso |
|---|---|
| `pasos.ogg` | efecto de caminar |
| `victoria.ogg` | jingle al llegar a la meta |
| `bgm.ogg` | música de fondo, opcional |

`bgm.ogg` va en `.gitignore` porque es música con derechos de autor. Si el archivo no existe, se
avisa por consola y el juego sigue sin música: no debe crashear ni negarse a arrancar.

Actualiza `assets/CREDITS.md` con los nombres nuevos.

---

## 8. Sprite animado

La rúbrica pide al menos una animación. Implementa el fuego así:

- Carga los ocho frames al inicio, en un `Vec` dentro del atlas.
- **Avanza el frame por tiempo transcurrido**, alrededor de 10 fps de animación. No lo ates al
  número de frame renderizado: si lo haces, la llama cambia de velocidad cuando cambian los FPS
  del juego.
- Render como billboard: calcula el ángulo del jugador al sprite, descarta si cae fuera del FOV,
  escala el tamaño en pantalla por la distancia.
- **Consulta el z-buffer por columna** que llenaste al dibujar paredes y salta los píxeles del
  sprite que queden detrás de una pared. Sin esto el fuego se ve flotando encima de todo.
- Los PNG traen canal alfa: descarta el píxel si el alfa es bajo. No hace falta color-key.
- Coloca dos o tres fuegos en callejones sin salida. Cuentan también para la estética del nivel.

---

## 9. Checklist de rúbrica

Todo esto tiene que quedar funcionando. Lo que ya exista en el repo se reescribe con la
arquitectura nueva; lo que falte se implementa.

- [ ] El jugador no atraviesa paredes y el programa no crashea
- [ ] Textura distinta para cada tipo de pared del mapa
- [ ] Contador de FPS visible en pantalla, alrededor de 15 o más
- [ ] Movimiento adelante y atrás, y rotación
- [ ] Rotación con el mouse, solo horizontal
- [ ] Minimapa con la posición del jugador, **en una esquina**, no lado a lado
- [ ] Música de fondo
- [ ] Efectos de sonido (`pasos.ogg`, `victoria.ogg`)
- [ ] Al menos una animación de sprite
- [ ] Pantalla de bienvenida
- [ ] Pantalla de éxito al llegar a la meta

Sobre colisiones: revisar solo el punto del jugador no basta, te atoras en las esquinas y a veces
te cuelas en diagonal. Usa un radio y evalúa los ejes X e Y por separado, así el jugador resbala
por la pared en vez de trabarse.

Sobre las pantallas: implementa `Stage` como enum con `match` en el bucle principal. Nada de
banderas booleanas sueltas del tipo `if !started && !won`.

---

## 10. Plan por etapas

Una etapa a la vez. Al terminar cada una, me la enseñas y esperas mi visto bueno antes de seguir.
Al final de cada etapa: `cargo build` sin errores, `cargo fmt`, `cargo clippy` sin warnings, y el
programa corre y hace lo que la etapa prometía.

| Etapa | Qué se hace |
|---|---|
| 0 | Inventario del código actual (sección 0) |
| 1 | Estructura de carpetas nueva, `maze.txt` nuevo, `Grid` con `Tile` y la duplicación de filas |
| 2 | `Framebuffer`, ventana, bucle principal, `Stage`, reloj y contador de FPS |
| 3 | `Actor`: movimiento, rotación por teclado y mouse, colisión con radio |
| 4 | DDA y paredes en color plano, con z-buffer por columna |
| 5 | Texturas de pared y puerta, sombreado por orientación de cara |
| 6 | Sprite de fuego animado con recorte por z-buffer |
| 7 | Minimapa en esquina y HUD |
| 8 | Audio: pasos, victoria, música de fondo opcional |
| 9 | Pantallas de bienvenida y de éxito |
| 10 | Pasada final de comentarios (sección 2), README nuevo, `.gitignore`, `CREDITS.md` |

Commits: uno por etapa, mensaje en español, imperativo, una línea (`agrega texturizado de
paredes`). Nada de `wip`, `cambios`, `fix`. El repositorio compila en cada commit.

---

## 11. Pasada final

En la etapa 10, antes de dar el proyecto por terminado, revisa el código completo con estos ojos:

1. ¿Queda algún identificador con el nombre viejo? (`Framebuffer` y `cast_ray` son la excepción:
   esos se quedan.)
2. ¿Hay comentarios que repiten lo que dice la línea de abajo? Bórralos.
3. ¿Hay `///` puestos por cumplir en funciones cuyo nombre ya lo explica? Bórralos.
4. ¿Todos los comentarios empiezan con el mismo verbo o tienen el mismo largo? Varíalos.
5. ¿Queda algún `unwrap()` sin mensaje, algún indexado directo del grid, algún literal numérico
   suelto que debería ser constante?
6. ¿El README describe este proyecto, con las instrucciones de dónde colocar `bgm.ogg`?
