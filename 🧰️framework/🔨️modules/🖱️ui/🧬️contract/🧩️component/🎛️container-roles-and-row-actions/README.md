# Design Notes

## Preserved Source Rationale

<a id="source-notes-22"></a>

### 🦀️.rs:22

⌨️ The closed set of input kinds. Grepped from the fleet's actual `UiInputNode.input_kind` string
literals (`"text"`, `"textarea"`, `"number"`, `"file"`) plus the values the React `🟦️Interpreter`
already branches on for `inputKind` (`"number"`, `"longText"`, `"date"`, `"color"`, `"file"`,
default `"text"`) — the union of both sides, since the renderer supports more kinds than any
current plugin emits yet. Note the old Rust/TS spelling mismatch this closes: Rust emitted
`"textarea"`, TS checked `"longText"`; this enum has exactly one spelling (`LongText`) for that
kind. No `Search` kind exists anywhere in the fleet today, so it is not included (see the
packet's own report for the grep evidence — adding it back is a one-variant change, not a design
change, if a plugin needs it later).

<a id="source-notes-146"></a>

### 🦀️.rs:146

🎯️ The ONE action target of a row: the controller `scope` at its contract `version` that every
[`RowAction::verb`] and the row's `activation` fire in, the ONE argument map they all inherit, and the
verb the row's primary activation (open, select) fires. A host dispatches a row verb as
[`RowTarget::binding`] — the same [`crate::ActionBinding`] shape any record binding has, so a tree row
and a table row with the same target dispatch identically. `args` is a [`crate::UiMap`] handle, not an
inline [`crate::UiValue`], so a targeted row costs one scope and one activation verb inline.
🌱️ No `ToValue`/`FromValue` here: `args` embeds `UiMap`, the deliberate DslValue-free exception —
see `UiValue`'s docstring in `🎬️action.rs`.
