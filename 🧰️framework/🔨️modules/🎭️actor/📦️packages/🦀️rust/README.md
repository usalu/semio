# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

📦️ Package glue — wiring only. Domain lives at owner `🦀️.rs`. The `wasm_bindgen`
`KernelHost` wrapper lives at owner `🔗️bindings/🦀️.rs`, behind `#[cfg(target_arch = "wasm32")]`,
so the pure crate core never sees `wasm_bindgen`/`web_sys`, and this file stays pure
declaration/wiring (no `struct`/`impl` of its own) — keeping its taxonomy package role
classified as thin delegation rather than implementation, so it does not compete with
`🦀️.rs` for the crate's one canonical implementation slot. It passes only byte buffers
(pack-encoded `Envelope`/`TurnResult`/`Decision`) — no typed value ever crosses the wasm boundary.
