# Quickstart: Sporefall Mushroom Sorting Game

## Goal

Run the Rust/WASM browser shell locally and prepare it for static deployment on
GitHub Pages.

## Expected Tooling

- Rust stable toolchain
- `wasm32-unknown-unknown` target
- A WASM bundler such as Trunk

## Local Development Flow

1. Install Rust and add the `wasm32-unknown-unknown` target.
2. Install Trunk.
3. From the repository root, run the local dev server for the static shell.
4. Open the served page and confirm the canvas loads and the Rust prototype text
   renders.
5. Run the Rust tests for the gameplay core before expanding browser behavior.

## Deployment Flow

1. Build the static site for release.
2. Publish the generated static assets to GitHub Pages.
3. Use the repository name `sporefall` as the public base path when publishing a
   project site.

## First Validation Targets

- `cargo test -p game_core`
- Browser smoke check that the WASM shell renders to the canvas
- Bundle-size and feedback-latency checks before adding more assets
