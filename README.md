# Template: axum + htmx + tailwind

## Backend

- __[Axum](https://github.com/tokio-rs/axum)__
- __[PostgreSQL](https://www.postgresql.org)__
- __[minijinja](https://docs.rs/minijinja/latest/minijinja/)__

## Frontend

- __[htmx](https://htmx.org)__
- __[TypeScript](https://www.typescriptlang.org)__
- __[Tailwind CSS](https://tailwindcss.com)__
- __[esbuild](https://esbuild.github.io)__

## Getting Started

_This template is a work-in-progress. Additionally, it builds against the
unstable main branch of `axum`, not the crates.io version._

## Notes

- Internally caches JavaScript and CSS files, compressing them with `brotli` at
  max compression level.
- Dynamically compresses HTML fragments with `brotli` at a lower compression
  level.
- Sets Cache-Control headers for CSS, JS, WEBP, SVG, and WOFF2.
- Uses `esbuild` via the `build.rs` script to bundle, hash, and minify
  JS/TS/CSS.
- Run with `cargo watch -x run` to automatically rebuild on asset / source
  changes.

## Other Templates

- __[Axum + SolidJS](https://github.com/robertwayne/template-axum-solidjs-spa)__
- __[Rocket + Svelte](https://github.com/robertwayne/template-rocket-svelte-spa)__
