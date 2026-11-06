# Design Notes

## Preserved Source Rationale

<a id="source-notes-164"></a>

### 🦀️.rs:164

🧬️ Derives the utility bar `UtilityNode` tree from resolved utilities and the host-owned active utility id.
Each utility becomes a `Toggle` whose `pressed` reflects `active_utility_id == Some(id)` and whose
`on_change` dispatches `setActiveUtility { utilityId }` against `controller_id`. Utilities sharing a `group`
collapse into one `Collection` (placed where the group first appears, in utility order); ungrouped
utilities stay flat siblings. A group that ends with exactly one child is hoisted to a top-level toggle —
a lone `group:transform`/`transform` pair must not render as two nested "Transform" rows. This is the
single source of truth for the utility bar — `ArtifactApp::utilities` no longer exists.

<a id="source-notes-234"></a>

### 🦀️.rs:234

#region role_chrome
👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: role-aware wgpu
shell chrome primitives, parity with the React shell (lane 1-C) against the SAME frozen
strings/command ids as `📋️contract-freeze.md` §3/§5. Domain-neutral: this crate never depends
on `semio_framework` (see the `wgpu` feature's `Cargo.toml` deps), so `ChromeRole` is a local
wire-compatible mirror of `semio_framework::AppRole`/the TS host's own `AppRole` mirror in
`🎠️kernel/🧭️app-router/🟦️.ts` — same boundary this file already draws around `PluginCatalog`-style
product data. A concrete `AppRouter`/`ConfigStore` never appears here: callers (the renderer
product) resolve real entries and hand this module only already-resolved data to render.

<a id="source-notes-384"></a>

### 🦀️.rs:384

🖱️ Builds the "Open with…" `ContextMenuItemSpec` submenu row — contract freeze §5: entries
grouped by role, viewer group first then editor (matching `AppRole`'s own declaration order,
contract §1 C1), each headed by a labeled separator (`layout::context_menu_is_header`'s own
convention: a separator carrying a `label` is a non-interactive section header). Every entry
carries a nested "Set as default" toggle child dispatching `OS_SET_DEFAULT_VIEWER`/`_EDITOR`
when turning default ON, `OS_CLEAR_DEFAULT_APP` when turning it OFF — the toggle direction is
resolved here (not left to the host) because there is no single OS command that flips a
boolean; `checked` mirrors `entry.is_default` for a caller to paint a checkmark. Clicking the
entry itself dispatches `PALETTE_OPEN_ARTIFACT_WITH_VIEWER`/`_EDITOR` with `{pluginId,
appId}` args. An empty `entries` list still returns the "Open with…" row with zero children —
the caller decides whether to omit an empty menu.
