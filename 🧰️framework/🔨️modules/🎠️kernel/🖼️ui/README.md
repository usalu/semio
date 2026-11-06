# Design Notes

## Preserved Source Rationale

<a id="source-notes-17"></a>

### 🦀️.rs:17

🐢️ What part of the shell's rendered UI an action actually invalidates — lets `refresh-ui`
skip re-rendering/re-fetching sections nothing touched. Absent from JSON (older/unmodified plugins)
deserializes to `Full`, so any program that never sets this keeps today's whole-shell-refresh
behavior exactly. `None` means "nothing to re-render at all" (e.g. a pure telemetry/heartbeat action).
🐢️ `rename_all = "camelCase"` alone only renames the *variant* names (Full/None/Partial ->
full/none/partial via `tag = "kind"`) — it does NOT cascade into a struct variant's own fields, which
would otherwise serialize as snake_case (`window_bodies`) and silently desync from the TS
`UiDirtyScope` type's camelCase `windowBodies`. `rename_all_fields` is the attribute that renames
fields *within* variants; both are needed together.

<a id="source-notes-72"></a>

### 🦀️.rs:72

⚖️ The selection law both shells answer to, and the union one coalesced refresh pass owes.

The React shell has always read a scope (`🛠️ShellHelpers/🟦️.tsx`'s `uiRefreshWants*` +
`buildUiRefreshRequest`); the wgpu shell's `refresh_ui` walked EVERY window and panel on every
settle and threw `InvocationResult::ui_scope` away — 116 of 137 renders per converging edit
answered `patched=0`, and the flow window was re-minted eight times for nothing
(ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7).
These predicates are that shell's half of the same law, driven by the same fixture
(`🧫️fixtures/🐢️ui-dirty-scope/🔣️.json`) as the TypeScript twin in `🎠️kernel/🎯️ui-dirty-scope/🟦️.ts`.

<a id="source-notes-168"></a>

### 🦀️.rs:168

🩹️ `wit-flip` (26/08/20): re-exported from `semio-ui-contract`, the language-neutral
contract crate's own `UiPatch`/`UiPatchOp` (`🦀️document.rs`) — this file no longer declares its
own copy, so there is exactly one definition to keep in sync with `component.wit`'s
`ui-patch`/`patch-op` (node-id addressed, not path addressed) rather than two that could drift.
Requires a `semio-ui-contract` dependency on every crate that `#[path]`-mounts this
file — see this packet's report for the exact registrar-request lines (this crate is not on the
registrar-only list for `Cargo.toml`, so the dependency itself is not added here).

<a id="source-notes-185"></a>

### 🦀️.rs:185

📏️ How many surfaces one turn result may carry. This is the fixed CAPACITY of [`UiTurnPatches`],
not the admission rule — admission is the per-turn BYTE budget (`Budget::max_patch_bytes`, bounded
by [`UI_TURN_PATCH_BUDGET_BYTES`]), so a turn carries every patch that is ready and fits.

🐛️ Until 2026-09-15 this was `1`, and the wire contract "one patch per crossing" made N published
surfaces cost N + 1 host round trips by construction — the floor
`📓️reactor-reconcile-spin-2026-09-14.md` §7 named and could not move. It is DERIVED, never chosen:
the page is an inline exact owner parked by value in every transport slot and every handback, so
the guest's declared contiguous-request ceiling is what one such block may cost, at
[`UI_TURN_PATCH_OWNER_BYTES`] per publication.
