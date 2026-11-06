# Design Notes

## Preserved Source Rationale

<a id="source-notes-50"></a>

### 🦀️.rs:50

#region 🏷️Label
🏷️ Display-ready UI text carried on the wire.

⚠️ Decision (flagged per packet brief): the old `UiNode`'s `Label` (`crate::wgpu::Label` in
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs`) is NOT reused here. It
is defined inside the old wgpu-target UI package, imports that package's own `Locale`/
`Terminology` axes, and its whole point (`From<LabelText>` wired to the `app_labels!` macro,
no `From<&str>`) is a compile-time-checked-label enforcement mechanism that lives at the
authoring boundary, not the wire boundary — and this crate must not depend on that package at
all (no engine, no wgpu; see `🦀️.rs`). This is therefore a minimal, independent transparent
string. The localization/terminology resolution that used to happen via `LabelText::fill` still
happens upstream of the runtime (manifest/host), before a `Label` ever reaches this contract.
