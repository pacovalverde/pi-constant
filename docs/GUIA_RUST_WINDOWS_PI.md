# Guía para principiantes: calcular π con Rust en Windows

Este proyecto es un programa de consola que calcula el número π con la cantidad
de decimales que elijas. No usa dependencias externas ni los tipos de coma
flotante habituales, que solo conservan un número limitado de cifras.

La guía utiliza la terminal **MSYS2 MINGW64**. Los comandos destinados a ella
aparecen en bloques etiquetados como `bash`. Cuando corresponda, también se
incluye la alternativa para PowerShell.

## 1. Instalar Rust

1. Entra en <https://rustup.rs/>.
2. Descarga y abre `rustup-init.exe`.
3. Selecciona la instalación predeterminada.
4. Si se solicitan herramientas de Visual Studio, instala la carga de trabajo
   **Desktop development with C++** y el SDK de Windows recomendado.
5. Cierra y vuelve a abrir la terminal MSYS2 MINGW64 para que detecte los cambios
   realizados en `PATH`.

Comprueba que Rust y Cargo están disponibles:

```bash
rustc --version
cargo --version
```

Cada orden debe mostrar un número de versión.

## 2. Entrar en el proyecto

```bash
cd /c/proyectos/rust/pi-constant
```

En PowerShell, la orden equivalente es:

```powershell
cd C:\proyectos\rust\pi-constant
```

En esta carpeta deben estar `Cargo.toml`, la carpeta `src` y la carpeta `docs`.

## 3. Compilar el programa

```bash
cargo build --release
```

La opción `--release` genera una versión optimizada, importante cuando se piden
muchos decimales. El ejecutable se guardará en
`target/release/pi-constant.exe`.

## 4. Calcular π

Para obtener 100 decimales:

```bash
cargo run --release -- 100
```

El número situado después de `--` es la cantidad de cifras posteriores al punto
decimal. Puedes cambiarlo:

```bash
cargo run --release -- 5
cargo run --release -- 1000
cargo run --release -- 10000
```

Si no indicas ninguna cantidad, se calculan 100 decimales:

```bash
cargo run --release
```

También puedes ejecutar directamente el programa ya compilado:

```bash
./target/release/pi-constant.exe 1000
```

Para consultar la ayuda:

```bash
cargo run --release -- --help
```

## 5. Guardar el resultado

Este ejemplo guarda 10 000 decimales en `pi.txt`:

En MSYS2 MINGW64:

```bash
cargo run --release --quiet -- 10000 > pi.txt
```

En PowerShell:

```powershell
cargo run --release --quiet -- 10000 | Out-File -Encoding utf8 pi.txt
```

## Cómo funciona

El programa utiliza la fórmula de Machin:

```text
π = 16 × arctan(1/5) − 4 × arctan(1/239)
```

Cada arcotangente se calcula mediante una serie matemática. Los números grandes
se almacenan como bloques de nueve cifras y se añaden decimales extra de
seguridad antes de redondear el resultado.

## Problemas frecuentes

- **MSYS2 no encuentra `cargo`:** cierra la terminal, vuelve a abrirla y, si
  es necesario, reinicia Windows.
- **Aparece un error sobre `link.exe`:** instala las herramientas de compilación
  de C++ y el SDK de Windows desde Visual Studio Installer.
- **El cálculo tarda demasiado:** prueba primero con 100 o 1 000 decimales y
  asegúrate de incluir `--release`.
- **El argumento no es válido:** utiliza un entero no negativo sin puntos ni
  separadores; por ejemplo, `5000`.
