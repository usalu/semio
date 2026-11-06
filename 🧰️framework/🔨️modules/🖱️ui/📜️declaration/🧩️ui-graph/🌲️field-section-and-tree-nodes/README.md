# Design Notes

## Preserved Source Rationale

<a id="source-notes-90"></a>

### 🦀️.rs:90

🪟️ The materialised slice of a logically `total`-long tree child list: the node's `items` are
the entries `[offset, offset + items.len())` of that list. `total > 0` with no materialised
items means expandable-but-not-yet-loaded, never "empty" — a renderer pitches the
unmaterialised rows as empty spacer bands so the scrollbar spans the whole document. The
legacy-node mirror of `ui_contract::TreeWindow`; the two are bridged in
`🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`. The retained layout pitches the unmaterialised rows as spacers
(`layout::tree_window_spacer_px`), `Ui::tree_window_measures` reads every windowed container against its scroll
viewport, and the one request rule (`🧱️elements/🌳️Tree/🪟️window/🦀️.rs`, React's `treeWindowServedRequestsV1`) asks
back the rows the viewport needs — the same `ViewModel.tree_windows` React's host reports. There is deliberately no
`+N` continuation-row fallback.

<a id="source-notes-296"></a>

### 🦀️.rs:296

🖌️ Stamps `selected`/`previewed`/`color`/`peers` per-item presence across every item in every
section of a tree — the framework-side counterpart of a `UiTreeNode.interaction_domain`
binding. `previewed` wins visually over a plain `selected` item only insofar as both are
representable simultaneously (an item can be selected AND previewed). `own_color` (ticket
26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.6) is stamped onto
every item unconditionally — it names the color THIS session renders its own hover/selection
as, whether or not this particular item is currently marked; `peer_marks_for` resolves an
item's own peer roster by id (called once per item, not pre-collected, since the caller's
`InteractionView::peers_selecting`/`peers_hovering` are themselves per-id lookups).
