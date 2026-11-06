# Design Notes

## Preserved Source Rationale

<a id="source-notes-70"></a>

### 🦀️.rs:70

🔑️ Joins the segments of a windowed container's **window path** — the identity both sides of the
streaming loop address a container by (`ViewModel::tree_windows[].node_key`).

🔑️ A container's path is the node keys of its enclosing windowed containers, outermost first, then
its own key. A top-level section's path is therefore just its own key, so every flat request stays
exactly what it was; a load case's loads nested under `fem3d-play-artifact.load-cases` are
`"fem3d-play-artifact.load-cases\u{1f}dead"`. This is what makes container identity collision-free
BY CONSTRUCTION while node keys and pick target ids stay untouched: a document whose load case and
whose combination are both called `uls` gives them different paths, because their parents differ.
Two containers can still collide only as true siblings under one parent — which the UI document
itself already refuses (`DuplicateSiblingKey`).

🔑️ The separator is U+241F SYMBOL FOR UNIT SEPARATOR — the PRINTABLE glyph, not the C0 control
U+001F it depicts. A path crosses the process boundary inside `ViewModel::tree_windows`, and the
view-context admission both sides run
(`admitCrossingViewContext`/`parseResolvedPluginViewState`, `🛂️manifest/🟦️.ts`) refuses ANY
identifier carrying a C0 or DEL code point or exceeding 256 code points — a control separator made
every refresh, action and pick carrying a nested path throw `view context: invalid identifier` the
moment one nested container existed. A path is therefore a view-context identifier like any other:
printable, and at most 256 code points. A container whose own key already contains this glyph is
refused at assembly (`ui.tree-window.separator-in-key`) so a path can never be ambiguous.

<a id="source-notes-115"></a>

### 🦀️.rs:115

🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and
the old inline `control: Option<UiControlNode>` are now ordinary children on the record (the
`UiControlNode` enum does not get ported — every one of its old variants is already a
[`Component`] variant in its own right, so a control-as-child-node needs no separate wrapper
type). [`TreeItemProps::inline_toolbar`] identifies the one direct horizontal Toolbar child whose
real Button children remain independently focusable and actionable inside the row, while
[`TreeItemProps::detail`] identifies one direct Surface child placed below the row. The row's
primary click action (old `action: Option<ActionDescriptor>`) is [`RowTarget::activation`], a verb on
the row's target — never a record `Trigger::Activate` binding.
🌱️ No `ToValue`/`FromValue` here: `target: Option<RowTarget>` embeds `UiValue` — see [`RowTarget`].
