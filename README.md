# Template: axum + htmx + tailwind

## Backend

- __[Axum](https://github.com/tokio-rs/axum)__
- __[PostgreSQL](https://www.postgresql.org)__
- __[minijinja](https://docs.rs/minijinja/latest/minijinja/)__

## Frontend

- __[htmx](https://htmx.org)__
- __[TypeScript](https://www.typescriptlang.org)__
- __[Tailwind](https://tailwindcss.com)__
- __[bun](https://bun.sh/)__

## Getting Started

_This is an experimental template that I use as a base for my personal projects.
There are a lot of opinionated and probably controversial design choices that
I've made here. I cannot recommend using this template for your own projects if
you're new to Rust._

## Notes

- Internally caches asset files. JavaScript and CSS files are pre-compressed at
  startup with `brotli` with a max compression level.
- Compresses HTML fragments with `brotli` at a lower compression level via
  `tower-compression` at runtime.
- Sets Cache-Control headers for CSS, JS, WEBP, SVG, and WOFF2 by default.
- Uses `bun` via the `build.rs` script to minify, hash, and bundle JS/TS/CSS.
- Run with `cargo watch -x run` to automatically rebuild on asset / source
  changes.

## Other Templates

- __[Axum + SolidJS](https://github.com/robertwayne/template-axum-solidjs-spa)__
- __[Rocket +
  Svelte](https://github.com/robertwayne/template-rocket-svelte-spa)__
