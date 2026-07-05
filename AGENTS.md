# Project Context For AI Sessions

This file captures the current working assumptions for future AI coding sessions.
It is intentionally similar in role to a `CLAUDE.md` project memory file.

## Owner And Workflow

- The owner is a senior game engineer with 20+ years of experience.
- The owner wants to use AI-assisted "vibe coding" for most implementation work.
- The owner's main role is review, technical direction, and decision making.
- Future agents should be proactive: inspect the repo, implement, run checks, and report concise outcomes.
- Avoid workflows that require heavy manual editor interaction unless the owner explicitly asks for it.

## Current Prototype State

- Rust/Cargo is installed through Rustup.
- Visual Studio 2022 Build Tools with the C++ workload is installed because the MSVC `link.exe` linker was missing.
- The current Bevy version is `0.19.0`.
- The repository now has a minimal Rust workspace with `crates/game_app`.
- `game_app` opens a Bevy window and renders a Yaruo AA prototype with `Text2d`.
- Current AA asset: `assets/aa/yaruo/neutral.txt`.
- Current AA display font: `assets/fonts/Saitamaar.ttf`, supplied by the owner.
- Bevy asset loading is configured from the `game_app` crate by setting `AssetPlugin.file_path` to `../../assets`; without this, Bevy looks under `crates/game_app/assets` or `target/debug/assets` and the window can appear black because the AA font is not loaded.
- A dark backing panel and left accent line are rendered behind the AA so a missing text layer is visually easier to distinguish from a blank screen.
- The Yaruo AA was visually verified through a Computer Use screenshot after the asset path fix.
- Verification commands that have passed: `cargo fmt --all`, `cargo check --workspace`, `cargo test --workspace`, and `cargo build --package game_app`.
- The first Bevy build is slow because Bevy, `wgpu`, text/font shaping, windowing, rendering, ECS, and asset dependencies compile; incremental checks are fast once `target/` is warm.

## Local Environment Notes

- `target/` had an `Access is denied (os error 5)` issue after Build Tools installation and elevated build activity.
- It was repaired by deleting/recreating `target/` and fixing workspace ACLs.
- `cargo check --workspace` now works against the normal `target/` directory.
- If Access Denied recurs inside the Codex sandbox, use an escalated cargo command or a temporary `CARGO_TARGET_DIR` to unblock verification.
- The owner will handle git operations.

## Current Product Direction

- Start from one main technical direction: Rust + Bevy.
- Primary commercial target: Steam native build.
- Planned co-distribution target: Epic Games Store native build.
- Private verification target: Web/WASM build, not public launch by default.
- Mobile apps are not the current starting point, but iOS/Android premium app releases must remain viable.
- Monetization is premium / buy-once.
- Do not assume ads, IAP, subscriptions, or mobile LiveOps.
- Store-specific features should be optional adapters, not core game assumptions.

## Yaruo And ASCII Art Asset Direction

- The project intends to use Yaruo / Yaruo-style ASCII art as a core presentation direction.
- In-game AA assets should primarily be prepared, curated, and supplied by the owner or explicitly approved project sources.
- Do not scrape, bulk-import, or assume permission to use third-party AA collections unless the owner explicitly approves the source and licensing/provenance.
- Treat AA blocks as project assets with stable IDs and metadata, not as incidental hardcoded strings scattered through systems.
- The game repository should focus on consuming, rendering, validating, and packaging prepared AA assets.
- Do not build or scaffold AA authoring, conversion, or generation tools in this repository unless the owner explicitly reopens that decision.
- Future AA creation tooling is expected to live in a separate repository or separate tool project. This repository may still contain small asset import or validation helpers when needed for the game build.

## Localization And Internationalization

Localization support should be designed in from the start.

Rules:

- Do not hardcode player-facing text directly in game logic, UI systems, events, save data, achievements, or platform integration code.
- Use stable text IDs / localization keys for UI labels, dialogue, event text, item names, skill names, status effects, achievements, and system messages.
- Keep localized strings in reviewable text-based data files, separate from deterministic game rules.
- Treat Japanese as an initial authoring language if useful, but do not make Japanese text or layout assumptions part of the core architecture.
- Save data, replay data, telemetry-like debug records, and achievement state should store stable IDs, not localized display strings.
- UI layout must tolerate translated text with different lengths, line wrapping, font metrics, and reading conventions.
- ASCII art and AA-style character presentation may need locale-specific variants; keep AA assets addressable by stable IDs and avoid coupling game logic to a specific rendered text block.
- AA rendering must use a project-controlled bundled font with stable metrics; do not depend on system fonts for AA layout.
- Do not use proprietary, OS-bundled, or otherwise restricted fonts as required project fonts unless the project has explicit redistribution rights for every target platform.
- MS Gothic / MS PGothic / MS UI Gothic are examples of fonts that must not be assumed available or redistributable.
- Choose fonts only after checking license compatibility for commercial use, bundling, modification if needed, and redistribution on Steam, Epic Games Store, Web/WASM verification builds, and possible future iOS/Android releases.
- Separate AA display fonts from general UI text fonts if needed; AA correctness takes priority for AA blocks, while UI text can use a more readable localized font stack.
- Font selection and text rendering should support Japanese and future Latin-alphabet localization at minimum, with explicit fallback fonts planned where practical.
- Avoid concatenating translated fragments in code when grammar can vary by language; prefer full localized message templates with named parameters.
- Add lightweight localization validation when practical, such as missing-key checks, placeholder consistency checks, and pseudo-localized UI smoke tests.

## Engine And Language

- Preferred stack: Rust + Bevy.
- Use Bevy as the main runtime for native builds.
- Use Web/WASM only as a verification/demo build target when feasible.
- Do not introduce Unity, Godot, Tauri, or Next.js as the main game stack unless the owner explicitly reopens that decision.
- Tauri, Next.js, or other app/web shells may still be considered later for companion tools, account pages, launcher-like utilities, or non-game app surfaces.
- Prefer code-first design over editor-driven content workflows.
- Prefer text-based assets and data formats that are easy to review in diffs.

## Architecture Direction

Keep game logic portable and store-independent.

Suggested workspace shape:

```text
crates/
  game_core/        Rules, generation, save data, replay, achievements conditions
  game_app/         Bevy app, rendering, input, audio, screens
  platform_api/     Traits and shared platform-facing types
  platform_null/    Local development and Web/WASM verification
  platform_steam/   Steamworks adapter, only when needed
  platform_epic/    Epic Games Store adapter, only when needed
  platform_eos/     EOS adapter, only if cross-store online services are needed
assets/
tools/
```

The exact layout can change once real code exists, but preserve the boundary:

```text
Game Core -> Platform API -> Platform Adapter
Game Core must not directly depend on Steamworks, Epic, EOS, or browser APIs.
```

## App And Mobile Compatibility

Keep future iOS/Android app development possible even though Steam is the first market target.

Rules:

- Treat mobile as a future premium / buy-once release path, not an ads/IAP product by default.
- Do not make desktop-only assumptions in `game_core`.
- Keep rendering, file paths, platform services, window management, and input mapping outside pure game rules.
- Design UI and gameplay so they can adapt to touch, controller, mouse, keyboard, and different aspect ratios.
- Support suspend/resume, pause, and save-on-background semantics at the app shell layer.
- Keep save data portable across native desktop, mobile, and Web where practical.
- Avoid making Steam Cloud, Steam user IDs, filesystem paths, or desktop window behavior mandatory for progression.
- Prefer asset loading and configuration formats that can work on packaged mobile builds.
- If mobile-specific platform code is added, isolate it behind the same `platform_api` boundary as Steam/Epic/Web.

Potential future targets:

```text
platform_android  -> Android app store build, premium/buy-once
platform_ios      -> iOS App Store build, premium/buy-once
platform_mobile   -> Shared mobile app shell concerns if useful
```

Mobile-specific concerns that agents should remember:

- Touch target sizes, safe areas, orientation, and variable aspect ratios.
- App lifecycle events: background, foreground, interrupted audio, lost focus.
- Local storage permissions and sandbox paths.
- Battery, thermal, memory, and startup-time constraints.
- No assumption that mobile builds have Steamworks, Epic overlay, or desktop filesystem access.

## Platform Services

Treat platform integration as optional services. A future interface may include:

```rust
trait PlatformServices {
    fn user_id(&self) -> Option<UserId>;
    fn unlock_achievement(&self, id: AchievementId);
    fn submit_score(&self, board: LeaderboardId, score: i64);
    fn cloud_save_enabled(&self) -> bool;
}
```

Rules:

- Start with `platform_null`.
- Add Steamworks, Epic Games Store, EOS, iOS, or Android service adapters only after the game needs those features.
- Never let platform SDK types leak into game rules or save data.
- Steam and Epic are distribution targets first, not mandatory gameplay dependencies.
- EOS is deferred unless cross-store online features become necessary.
- Mobile store services are deferred unless iOS/Android release work begins.

## Bevy Style

- Use Bevy ECS idiomatically: small systems, data-only components, resources for global state, states for screen/app flow.
- Keep systems focused and test pure logic outside Bevy where practical.
- Put deterministic rules, procedural generation, economy, replay logic, and save migration in `game_core`.
- Keep rendering, input mapping, audio, camera, and UI in `game_app`.
- Avoid large monolithic plugins that mix unrelated gameplay, UI, and platform code.
- Prefer explicit app states and typed events/messages over hidden global flags.
- Design input through an internal `GameInput` layer so keyboard, controller, touch, and Web input can map into the same game actions.
- Keep UI state and layout logic data-driven enough that mobile/touch variants can be added without rewriting gameplay.

## Web/WASM Verification

- Web/WASM is for private validation and easy sharing, not the default public product.
- Web builds should use `platform_null` unless a specific online integration is requested.
- Web save should be isolated from native save assumptions.
- Avoid depending on browser-only features in core gameplay.
- Keep asset loading paths and build scripts friendly to WASM where reasonable.

## Store Strategy

- Steam is the main market validation path.
- Prefer Steam Coming Soon, demo, and/or Steam Playtest for early market feedback.
- Epic Games Store is a planned co-distribution target, with exact timing still open.
- iOS/Android are future app targets if the genre, controls, session length, and market response justify it.
- Store-specific features such as achievements, cloud save, overlay, rich presence, invites, and leaderboards are optional adapters.
- Do not make Steamworks or EOS required for local development.

## Testing And Verification

Prefer fast, repeatable checks:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all
```

When Web/WASM support exists, also verify the web build path, for example:

```bash
trunk build --release
```

Only add commands that actually exist in the repo. If a command cannot run because the project is not scaffolded yet, say so plainly.

## AI Implementation Guidelines

- Read existing code and follow local patterns before adding abstractions.
- Keep changes reviewable and narrow.
- Prefer strong types, explicit data flow, and tests around pure game logic.
- Make generated code boring and maintainable.
- Avoid premature integrations with Steamworks, Epic, EOS, mobile store SDKs, analytics, ads, or IAP.
- Mobile app release work may add iOS/Android adapters when needed; do not block future app development by hardcoding desktop or Steam assumptions.
- Do not create editor-heavy workflows unless requested.
- Do not bury important behavior in generated assets when Rust code or plain data is clearer.

## Open Decisions

These are intentionally not locked yet:

- Exact game genre and scale.
- Whether Steamworks features are used beyond distribution/playtest.
- Whether EOS is needed for cross-store online services.
- Whether Epic ships at the same time as Steam or after Steam validation.
- Whether Web/WASM remains private-only or later becomes a public demo.
- Whether iOS/Android premium app builds are worth pursuing after Steam validation.
