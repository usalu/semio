# Design Notes

## Preserved Source Rationale

<a id="source-notes-58"></a>

### 🦀️.rs:58

🌉️ Hand-written, not derived: `#[value(...)]` has no way to express `skip_serializing_if` on an
enum VARIANT's own named field (the derive's internally-tagged codegen never reads that
attribute there — struct-field position only) — a derive here would silently start writing
`appId`/`windowKindId`/`modeId` as `null` instead of omitting them, changing the wire shape.
Mirrors `#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]`
exactly, field-for-field with the omit-if-`None` behavior serde's own `skip_serializing_if`
gives it (and the matching implicit "missing `Option` field decodes to `None`" serde gives every
`Option<T>` field, `#[serde(default)]` or not).
