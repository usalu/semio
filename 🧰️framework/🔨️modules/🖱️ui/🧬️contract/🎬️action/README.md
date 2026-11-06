# Design Notes

## Preserved Source Rationale

<a id="source-notes-8"></a>

### 🦀️.rs:8

🆔️ A versioned action address. `scope` names the controller/domain (the old `ActionDescriptor`'s
stringly `controller_id`, e.g. `"cad-play"`, grepped verbatim from the plugin fleet's
`ActionFactory::new(CONTROLLER_ID)` call sites), `name` the verb (the old `action`, e.g.
`"objectMove"`/`"setValue"`/`"addWidget"`), and `version` is new: it lets a renderer reject or
migrate a stale action instead of silently invoking the wrong one — the one axis the old stringly
pair never carried.

<a id="source-notes-63"></a>

### 🦀️.rs:63

🔗️ One node-carried binding from a [`Trigger`] moment to a versioned [`ActionId`]. Replaces every
old `on_change`/`action`/`drop_action`/... field scattered across the wgpu target's per-component
node structs — a record's `bindings: Vec<ActionBinding>` is the one place any of them now live.
🌱️ No `ToValue`/`FromValue` here: `args: Option<UiValue>` and `UiValue` itself is the deliberate
DslValue-free exception (see `UiValue`'s docstring below) — a derived impl would need
`UiValue: ToValue`, which is exactly the forbidden conversion.
