# Design Notes

## Preserved Source Rationale

<a id="source-notes-275"></a>

### 🦀️.rs:275

🎨️ A colour-picker recipe over an sRGB colour with components in `0..=1`: a labelled, wrapping
`ContainerRole::Group` holding the swatch (`InputKind::Color` keyed `swatch`, `#rrggbb`), the hex field
(`InputKind::Text` keyed `hex`, `#rrggbb` — `#rrggbbaa` with alpha — committed on blur) and, with alpha, the alpha
slider (keyed `alpha`, `0..=1` in steps of `0.01`). The swatch and the hex field commit a hex string
([`crate::parse_ui_color_hex`] reads it back), the slider the straight alpha. Costs 3 records, 4 with alpha. Build
with [`color_input`], fill with [`ColorInputBuilder::try_color`].
