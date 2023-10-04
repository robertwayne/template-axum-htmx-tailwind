# Template: axum + htmx + tailwind

## Backend

- __[Axum](https://github.com/tokio-rs/axum)__
- __[PostgreSQL](https://www.postgresql.org)__
- __[minijinja](https://docs.rs/minijinja/latest/minijinja/)__

## Frontend

- __[htmx](https://htmx.org)__
- __[TypeScript](https://www.typescriptlang.org)__
- __[Tailwind CSS](https://tailwindcss.com)__
- __[bun](https://bun.sh/)__

## Getting Started

_This template is a work-in-progress with various experimental features. It
builds against the unstable main branch of `axum`, not the crates.io version,
among other things. I cannot recommend using this for a production project right
now._

## Notes

- Internally caches JavaScript and CSS files, compressing them with at startup
  with `brotli` at max compression level.
- Compresses HTML fragments with `brotli` at a lower compression
  level via `tower-compression` at runtime.
- Sets Cache-Control headers for CSS, JS, WEBP, SVG, and WOFF2 by default.
- Uses `bun` via the `build.rs` script to minify, hash, and bundle
  JS/TS/CSS.
- Run with `cargo watch -x run` to automatically rebuild on asset / source
  changes.

## Other Templates

- __[Axum + SolidJS](https://github.com/robertwayne/template-axum-solidjs-spa)__
- __[Rocket + Svelte](https://github.com/robertwayne/template-rocket-svelte-spa)__
