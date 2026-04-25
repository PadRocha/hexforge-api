# PadRocha API Template

Plantilla minima para construir una API en Rust con Axum y MongoDB.

Este proyecto fue limpiado para funcionar como lienzo en blanco: no conserva endpoints de negocio, entidades, casos de uso, autenticacion, procesamiento de imagenes ni repositorios del proyecto anterior. Solo queda la base necesaria para levantar el servidor, conectar a MongoDB, tener estado compartido, manejar errores HTTP comunes, CORS, tracing y un health check tecnico.

## Stack

- Rust 2024
- Axum 0.8
- Tokio
- MongoDB Rust Driver
- dotenvy para variables de entorno
- tower-http para CORS y trazas HTTP
- tracing/tracing-subscriber para logs de consola y `app.log`
- thiserror para errores tipados

## Estructura

```text
src/
  main.rs                  # Entrada del binario
  lib.rs                   # Modulos publicos del crate
  domain/                  # Entidades, value objects y reglas de dominio
  application/             # Casos de uso, puertos y errores de aplicacion
  adapters/
    http/                  # Estado, rutas, respuestas y errores HTTP
    persistence/           # Repositorios/adaptadores MongoDB
  infra/                   # Configuracion, MongoDB, tracing y creacion de app
tests/
  health.rs                # Smoke test del endpoint tecnico de salud
```

## Endpoints Incluidos

Solo hay endpoints tecnicos:

- `GET /api/health` verifica que el servidor responde.
- `GET /api/health/ready` hace `ping` a MongoDB y devuelve `503` si la base no esta disponible.

Los endpoints de negocio deben agregarse desde cero en `src/adapters/http/routes`.

## Configuracion

Crea tu archivo local de variables:

```bash
cp .env.example .env
```

Variables principales:

```env
SERVER_ADDRESS=127.0.0.1:3001
CORS_ALLOWED_ORIGIN=http://localhost:5173
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=example
MONGODB_APP_NAME=padrocha
```

Opcionales:

```env
MONGODB_MAX_POOL_SIZE=20
MONGODB_MIN_POOL_SIZE=0
MONGODB_MAX_IDLE_TIME_SECS=60
```

## Ejecutar

Con MongoDB levantado:

```bash
cargo run
```

Prueba rapida:

```bash
curl http://127.0.0.1:3001/api/health
curl http://127.0.0.1:3001/api/health/ready
```

## Desarrollo

Flujo recomendado para agregar una feature:

1. Define entidades o value objects en `src/domain`.
2. Define el caso de uso y sus puertos en `src/application`.
3. Implementa persistencia MongoDB en `src/adapters/persistence`.
4. Expon la ruta HTTP en `src/adapters/http/routes`.
5. Registra la ruta en `src/adapters/http/routes/mod.rs`.
6. Agrega dependencias compartidas al `AppState` solo cuando realmente sean globales.
7. Cubre la feature con pruebas enfocadas en `tests/`.

Para validar:

```bash
cargo fmt
cargo check
cargo test
```

No uses `cargo run src/main.rs`; en Cargo eso pasa `src/main.rs` como argumento al binario. Usa `cargo run` o `cargo run -- <args>`.
