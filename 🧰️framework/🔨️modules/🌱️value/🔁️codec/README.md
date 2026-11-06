# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🔁️ `ToValue`/`FromValue` — the first-party analog of `serde::Serialize`/
`serde::de::DeserializeOwned`, over [`super::DslValue`] instead of a generic visitor.

Exists to break the forced `serde` dependency `MutationDiff`/`Mutation` (`crate::mutation`)
used to bake into every implementor: those traits now bound on `ToValue + FromValue`, so a
plugin implementing them never needs to depend on `serde`. See
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
🔍️research/📓️serde-replacement-surface.md` for the design rationale and the older
`super::dsl_value_serde` bridge this supersedes for plugin-facing code (that bridge itself
requires `serde::Serialize`/`serde::de::DeserializeOwned` on its input, so it cannot be the
plugin-facing seam — it stays only for framework-internal callers that still speak serde).

`#[derive(ToValue, FromValue)]` (`semio-framework-value-derive`, `🌱️value/✨️derive`) implements
both traits for a `#[value(...)]`-annotated struct/enum; the scalar/container leaves below are
the hand-written base cases every derived impl bottoms out on.
