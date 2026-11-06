# Design Notes

## Preserved Source Rationale

<a id="source-notes-97"></a>

### 🦀️.rs:97

🎬️ The 15 product scene structs (`Canvas2dScene`, `World3dScene`, `NodeGraphScene` + its
nested port/node/edge/viewport/operator records, `TextEditorScene`, `TableScene`,
`Paint2dScene`, `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene`, `Board2dScene`,
`InkCanvasScene`, `GraphTimelineScene`, `DiffViewScene`, `EventFeedScene`, `BlockListScene`)
relocated to `semio-ui-scene` — ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
RUNTIME packet `scene-surface`. Re-exported here (not a compatibility shim: this crate
genuinely still builds `UiNode`s from them, see `build_table_scene` etc. below) so every
Scene payload ownership belongs to the independent scene contract.
`TableScene::drop_action_json` and `NodeGraphOperatorChannelRecord::default_json` are now
opaque JSON strings, not live `ActionDescriptor`/`serde_json::Value` — see the scene crate's
own `🦀️scenes.rs` header for why (that crate is wasm-safe and depends on nothing beyond
`ui_contract`/`serde`, so it cannot carry either type).

<a id="source-notes-144"></a>

### 🦀️.rs:144

#region 🔖️ValueRoundTripTests
🌱️ `ToValue`/`FromValue` round-trip tests for ticket
26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS's additive pass over
`pub mod ui`'s standalone node types, plus (since `semio-ui-scene` landed hand-
written `ToValue`/`FromValue` for all 15 embedded scene payload types) the previously-
blocked recursive-through-`UiNode` family: `UiStackNode`/`UiGroupNode`/`UiFieldNode`/
`UiSectionNode`/`UiInspectorFieldGroup`/`UiComponentSceneNode`/`UiNode` itself.
