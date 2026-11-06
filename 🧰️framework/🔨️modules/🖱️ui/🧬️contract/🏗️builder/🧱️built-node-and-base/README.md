# Design Notes

## Preserved Source Rationale

<a id="source-notes-17"></a>

### 🦀️.rs:17

🧬️ The contract-local shape a builder terminates into — everything a [`crate::UiNodeRecord`]
carries except `id` (minted by the runtime at reconciliation, never by an author) and with
`children` nested inline rather than addressed by [`crate::UiNodeId`], since a freshly authored
tree has no ids yet to address by. Every field below serializes away at its default, mirroring the
wire-cost guarantee this whole builder family exists to make automatic.
🌱️ No `ToValue`/`FromValue` here (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
ARTIFACTS): `component: crate::Component` (and `bindings`/`menu`) embed `UiValue`, the deliberate
DslValue-free exception — see its docstring in `🎬️action.rs`.
