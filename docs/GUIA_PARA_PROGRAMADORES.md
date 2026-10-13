# Guía para programadores

El proyecto calcula π con precisión arbitraria y no depende de crates externos.
Esta guía supone que el repositorio ya está clonado y que la terminal está
abierta en su directorio raíz.

## Requisitos

- Toolchain estable de Rust, preferiblemente administrado con `rustup`.
- Con el toolchain MSVC de Windows, Visual Studio Build Tools con **Desktop
  development with C++** y un SDK de Windows.
- Terminal MSYS2 MINGW64 para ejecutar los bloques etiquetados como `bash`.
  Aunque se use Bash como terminal, Rust compila el proyecto con el toolchain
  MSVC.

Comprobación del entorno:

```bash
rustup toolchain install stable
rustup override set stable
rustc --version
cargo --version
```

## Dependencias, pruebas y compilación

La implementación usa exclusivamente `std`, por lo que no hay crates de
terceros que instalar. Cargo sigue siendo responsable de validar el manifiesto y
preparar cualquier dependencia que pueda añadirse posteriormente:

```bash
cargo fetch
cargo test
cargo build --release
```

El binario optimizado queda en `target/release/pi-constant.exe`.

## Interfaz de línea de comandos

El programa acepta un único argumento posicional opcional:

```text
pi-constant [DECIMALES]
```

`DECIMALES` es un entero no negativo. Si se omite, el valor predeterminado es
100. Con Cargo:

```bash
cargo run --release -- 1000
```

El primer `--` separa las opciones de Cargo de los argumentos del binario. La
ejecución directa equivalente es:

```bash
./target/release/pi-constant.exe 1000
```

Para cálculos grandes puede redirigirse la salida:

```bash
cargo run --release --quiet -- 100000 > pi.txt
```

Los valores negativos, no numéricos y los argumentos adicionales producen un
mensaje por `stderr` y un código de salida `2`.

## Diseño del cálculo

Se aplica la identidad de Machin:

```text
π = 16·atan(1/5) − 4·atan(1/239)
```

Las arcotangentes se evalúan con la serie alternante de Taylor. La aritmética de
precisión arbitraria se implementa internamente con bloques little-endian en
base `10^9`. El cálculo usa una escala decimal fija, cifras de guarda y redondeo
final, evitando el límite de precisión de `f64`.

El límite práctico depende de la memoria y del tiempo de CPU. El coste aumenta
con la precisión solicitada, así que las ejecuciones grandes deben utilizar el
perfil `release`.

## Comprobaciones recomendadas

Antes de integrar cambios:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --release
```

Si el repositorio conserva `Cargo.lock` y se necesita una compilación bloqueada:

```bash
cargo build --release --locked
```
