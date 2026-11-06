# Design Notes

## Preserved Source Rationale

<a id="source-notes-209"></a>

### 🦀️.rs:209

🆕️ `direction` is an AXIS direction (`row`/`column`). `stack` is not an axis — it is the
tabbed leaf container — so a `stack` request is answered with the single stack root
[`create_stack_layout`] mints. Minting `Axis { kind: "stack" }` instead put a stack node
under a node the whole shell reads AS a stack, and every consumer that walks a stack's
children as window leaves (the React seed resolver
`convertFrameworkLayoutNodeToModeLayout`) then produced windows with no id — which is how
`🎞️animate` booted with zero panes (ticket 26/09/18, slice F1).
