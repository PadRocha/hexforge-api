# PadRocha API Template

Plantilla base para construir APIs HTTP en Rust con Axum, MongoDB y una separación por capas inspirada en arquitectura clean/hexagonal.

El proyecto está pensado como punto de partida profesional: incluye arranque HTTP, configuración por entorno, conexión a MongoDB, estado compartido, CORS, tracing, manejo de errores HTTP y endpoints técnicos de salud. No incluye entidades ni endpoints de negocio para evitar acoplar la plantilla a un dominio específico.

## Stack

- Rust 2024
- Axum 0.8
- Tokio
- MongoDB Rust Driver 3.5
- dotenvy para cargar `.env`
- tower-http para CORS y trazas HTTP
- tracing/tracing-subscriber para consola y `app.log`
- thiserror para errores tipados
- serde/serde_json para serialización JSON

## Arquitectura

La estructura separa responsabilidades para que el dominio y la aplicación no dependan de frameworks ni drivers concretos:

- `domain`: entidades, objetos de valor y reglas puras del negocio.
- `application`: casos de uso, servicios de aplicación, puertos y errores de aplicación.
- `adapters/http`: entrada HTTP con Axum, rutas, estado compartido, respuestas y traducción de errores.
- `adapters/persistence`: implementaciones de repositorios e integración con MongoDB.
- `infra`: configuración, MongoDB, tracing, CORS y ensamblaje del router raíz.

Los adaptadores deben depender de la aplicación. La aplicación puede definir traits como puertos. Las implementaciones concretas de esos puertos deben vivir en `adapters`.

## Estructura

```text
src/
  main.rs                         # Entrada del binario y ciclo de arranque
  lib.rs                          # Módulos públicos del crate
  domain/
    mod.rs                        # Tipos y reglas puras del dominio
  application/
    mod.rs                        # Casos de uso y puertos
    app_error.rs                  # Error común de aplicación
  adapters/
    mod.rs                        # Adaptadores de entrada y salida
    http/
      app_error_impl.rs           # Mapeo AppError -> respuesta HTTP
      app_state.rs                # Estado compartido de Axum
      responses.rs                # Envoltorios JSON comunes
      routes/
        health.rs                 # Endpoints técnicos de salud
    persistence/
      mod.rs                      # Repositorios MongoDB futuros
  infra/
    app.rs                        # Router raíz, CORS y tracing HTTP
    config.rs                     # Variables de entorno y configuración
    db.rs                         # Inicialización de MongoDB
    setup.rs                      # Ensamble de estado y tracing
    startup_error.rs              # Errores de arranque
tests/
  health.rs                       # Prueba de humo del endpoint /api/health
```

## Configuración

Crea un archivo local a partir del ejemplo:

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

Variables opcionales para el pool de MongoDB:

```env
MONGODB_MAX_POOL_SIZE=20
MONGODB_MIN_POOL_SIZE=0
MONGODB_MAX_IDLE_TIME_SECS=60
```

`SERVER_ADDRESS`, `CORS_ALLOWED_ORIGIN` y `MONGODB_APP_NAME` tienen valores por defecto en el código. `MONGODB_URI` y `MONGODB_DATABASE` son obligatorias.

También puedes controlar el nivel de logs con `RUST_LOG`. Si no se define, se usa:

```env
RUST_LOG=apihules=debug,tower_http=debug
```

## Levantar MongoDB

Con Docker:

```bash
docker run --name apihules-mongo -p 27017:27017 -d mongo:7
```

Si el contenedor ya existe:

```bash
docker start apihules-mongo
```

Con una instalación local, inicia `mongod` dejando disponible `mongodb://localhost:27017` o ajusta `MONGODB_URI` en `.env`.

## Ejecutar

Con MongoDB disponible:

```bash
cargo run
```

El servidor escucha en `SERVER_ADDRESS`. Con la configuración de ejemplo:

```text
http://127.0.0.1:3001
```

## Endpoints básicos

La plantilla solo expone endpoints técnicos:

- `GET /api/health`: verifica que el proceso HTTP responde.
- `GET /api/health/ready`: ejecuta `ping` contra MongoDB y devuelve `503` si la base no está disponible.

Pruebas rápidas:

```bash
curl http://127.0.0.1:3001/api/health
curl http://127.0.0.1:3001/api/health/ready
```

Respuestas esperadas:

```json
{"status":"ok"}
```

```json
{"status":"ready"}
```

## Tests y validación

Formatea y valida el proyecto:

```bash
cargo fmt
cargo check
cargo test
```

`cargo test` también ejecuta doctests del crate. Si quieres validar solo ejemplos de documentación:

```bash
cargo test --doc
```

Genera documentación local:

```bash
cargo doc --no-deps --open
```

Para generar documentación incluyendo dependencias:

```bash
cargo doc --open
```

## Convenciones

- Usa `//!` para explicar la responsabilidad de módulos importantes.
- Usa `///` en APIs públicas: structs, enums, traits, funciones públicas y errores públicos.
- Documenta contratos, errores y efectos secundarios; evita comentar código evidente.
- Mantén el dominio libre de Axum, MongoDB y variables de entorno.
- Define traits de repositorio o servicios externos en `application` cuando sean contratos de casos de uso.
- Implementa integraciones concretas en `adapters`, por ejemplo MongoDB en `adapters/persistence`.
- Expón rutas HTTP en `adapters/http/routes` y móntalas en `routes/mod.rs`.
- Agrega dependencias globales a `AppState` solo cuando sean compartidas por varios handlers.
- Mapea errores de aplicación con `AppError`; no expongas detalles internos en respuestas HTTP.

No uses `cargo run src/main.rs`; Cargo interpreta `src/main.rs` como argumento del binario. Usa `cargo run` o `cargo run -- <args>`.
