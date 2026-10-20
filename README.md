# Pi

Aplicación de línea de comandos escrita en Rust para calcular el número pi con precisión arbitraria:

```text
π = 16 × arctan(1/5) − 4 × arctan(1/239)
```

El programa utiliza la fórmula de Machin y aritmética de enteros de precisión arbitraria implementada en el propio código, sin dependencias externas, para generar la cantidad de cifras decimales indicada por el usuario. Si no se proporciona un argumento, calcula 100 decimales; el resultado se redondea a la precisión solicitada.

## Compilar

```bash
cargo build --release
```

## Ejecutar

Mediante Cargo:

```bash
cargo run --release -- 100
```

O ejecutando directamente el binario compilado:

```bash
./target/release/pi-constant.exe 100
```

El argumento `100` indica la cantidad de cifras decimales que se mostrarán.

## Documentación

- Ayuda del programa: `pi-constant.exe --help`
- Ejecutar las pruebas: `cargo test`
- Calcular SHA-256: `sha256sum ./target/release/pi-constant.exe`

## Proyecto

Este programa es un recurso del proyecto [SciSymbols](https://scisymbols.com).
