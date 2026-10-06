# Blog Platzi 🦀

Un blog simple y rápido desarrollado en **Rust** usando **Actix-web**, **Diesel ORM**, **PostgreSQL** y el motor de plantillas **Tera**.

---

## 📸 Vista Previa

![Vista Principal](image.png)

---

## 🚀 Tecnologías y Herramientas

- **Lenguaje:** [Rust](https://www.rust-lang.org/) (Edición 2024)
- **Framework Web:** [Actix-web](https://actix.rs/) (v4)
- **ORM & Migraciones:** [Diesel](https://diesel.rs/) (v2.2 con soporte PostgreSQL y `r2d2`)
- **Motor de Plantillas:** [Tera](https://keats.github.io/tera/) (v1.20)
- **Serialización:** [Serde](https://serde.rs/) & [serde_json](https://docs.rs/serde_json)
- **Variables de Entorno:** [dotenvy](https://docs.rs/dotenvy)

---

## 📁 Estructura del Proyecto

```text
blog_platzi/
├── Cargo.toml
├── diesel.toml
├── .env
├── image.png             # Captura de pantalla de la vista principal
├── migrations/           # Migraciones de base de datos con Diesel
├── src/
│   ├── actions.rs        # Operaciones CRUD sobre la base de datos
│   ├── handlers.rs       # Controladores de las rutas y vistas
│   ├── main.rs           # Punto de entrada y configuración del servidor
│   ├── models.rs         # Modelos de datos y estructuras de formularios
│   └── schema.rs         # Esquema generado automáticamente por Diesel
└── templates/            # Plantillas HTML renderizadas con Tera
    ├── index.html        # Vista principal con listado de posts
    └── post.html         # Vista de detalle de un post individual
```

---

## ⚙️ Requisitos Previos

- [Rust y Cargo](https://www.rust-lang.org/tools/install) instalados.
- [PostgreSQL](https://www.postgresql.org/) en ejecución.
- CLI de Diesel instalado:
  ```bash
  cargo install diesel_cli --no-default-features --features postgres
  ```

---

## 🛠️ Configuración y Puesta en Marcha

### 1. Clonar el repositorio y configurar variables de entorno

Crea o edita el archivo `.env` en la raíz del proyecto:

```env
DATABASE_URL=postgres://usuario:password@localhost/blog_platzi
```

### 2. Configurar la base de datos y ejecutar migraciones

```bash
# Inicializar la base de datos
diesel setup

# Ejecutar las migraciones pendientes
diesel migration run
```

> **Comandos útiles de Diesel:**
>
> - `diesel migration redo` - Re-ejecuta la última migración.
> - `diesel migration revert` - Revierte la última migración.
> - `diesel migration generate <nombre>` - Crea una nueva migración.

### 3. Ejecutar el servidor

```bash
cargo run
```

El servidor iniciará en `http://localhost:8081` (o `http://0.0.0.0:8081`).

---

## 🛣️ Endpoints y Rutas

| Método   | Ruta           | Descripción                                  | Tipo        |
| -------- | -------------- | -------------------------------------------- | ----------- |
| `GET`    | `/`            | Lista todos los posts en la vista principal  | HTML (Tera) |
| `GET`    | `/post/{slug}` | Muestra el detalle de un post por su slug    | HTML (Tera) |
| `POST`   | `/new-post`    | Crea un nuevo post                           | JSON API    |
| `PATCH`  | `/update/{id}` | Actualiza el título y/o contenido de un post | JSON API    |
| `DELETE` | `/delete/{id}` | Elimina un post por su ID                    | JSON API    |

### Ejemplo de Payload (Crear / Actualizar Post)

```json
{
  "title": "Mi Primer Post",
  "body": "Contenido del post en texto o HTML..."
}
```

---

## 📄 Licencia

Este proyecto está bajo la licencia MIT / Open Source con fines educativos.
