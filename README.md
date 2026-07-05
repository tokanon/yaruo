# yaruo

Early-stage game project.

## Direction

- Primary stack: Rust + Bevy.
- Primary commercial target: Steam native build.
- Planned co-distribution target: Epic Games Store native build.
- Private verification target: Web/WASM build.
- Monetization: premium / buy-once.

The core rule is to keep game logic portable and store-independent:

```text
Game Core -> Platform API -> Platform Adapter
```

Platform SDK details such as Steamworks, Epic, EOS, iOS, Android, or browser APIs should not leak into core game rules or save data.

## Current Workspace Shape

```text
crates/
  game_app/         Bevy app, currently rendering the first Yaruo AA prototype
assets/
  aa/               Curated AA assets
  fonts/            Project-controlled fonts for AA rendering
```

Future crates such as `game_core`, `platform_api`, and platform adapters should be added when their boundaries are needed.

## Prototype

The current prototype opens a Bevy window and renders the prepared Yaruo AA asset with the bundled AA font:

```bash
cargo run --package game_app
```

The AA source is stored at `assets/aa/yaruo/neutral.txt`. The display font is loaded from `assets/fonts/Saitamaar.ttf`.

## Development

Expected fast checks:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all
```

When Web/WASM support exists, verify that path separately, for example:

```bash
trunk build --release
```

Only run commands that exist in the repository at the time.

## License

MIT. See [LICENSE](LICENSE).
