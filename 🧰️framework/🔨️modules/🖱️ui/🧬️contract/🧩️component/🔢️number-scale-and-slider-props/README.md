# Design Notes

## Preserved Source Rationale

<a id="source-notes-113"></a>

### 🦀️.rs:113

🎚️ Props for `Component::Slider`. `on_change` moved to the record's `bindings`. `min`/`max` are the travel;
`snaps` are its detents: strictly ascending, finite, inside `min..=max`, at most [`crate::UI_FIXED_LIST_ITEMS`]
of them — every renderer paints one tick per snap at its [`slider_axis_position`] and resolves a pointer value
through [`slider_pointer_value`] and a key through [`slider_key_value`]. Values are stored values; the readout,
the spoken value and a typed value are display values (`stored × display_factor`, at `precision`, beside
`display_unit` else `unit`, read back by [`ui_number_typed_value`]). A typed value is refused by `limits` (the
travel itself when absent).
