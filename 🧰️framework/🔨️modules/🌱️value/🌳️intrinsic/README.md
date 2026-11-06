# Design Notes

## Preserved Source Rationale

<a id="source-notes-12"></a>

### 🦀️.rs:12

#region 🔖️Number
👁️ A JSON-equivalent number that keeps the writer's-eye distinction a bare `f64` erases:
an integer literal (`UInt`/`Int`) round-trips without a decimal point, `Float` always keeps one
(or an exponent). Mirrors [`pack::json::Number`]'s shape exactly so the wire bridge between them
(`🎒️pack/🔤️json/🦀️.rs`) is a straight variant-to-variant map, never a widen-then-guess. See
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
🔍️research/📓️dslvalue-integer-fidelity.md`.

<a id="source-notes-338"></a>

### 🦀️.rs:338

#region 🔖️Literal
🧾️ Builds a [`DslValue`] from a JSON-shaped literal — the first-party replacement for
`serde_json::json!` + `DslValue::from` at every action-argument site. Grammar: `null`, `true`,
`false`, `[ … ]`, `{ key: value, … }` (a key is a string literal or any expression convertible
`Into<String>`, e.g. a `const`), and any other expression, converted through [`ToValue`] by method call
(so a borrowed `&String`/`&[T]` auto-derefs to its implementation, as `serde_json::json!` accepts it). Object
entries keep their written order (a `DslValue::Object` is ordered); trailing commas are
accepted. The equivalence law against `serde_json::json!` lives in `🧪️tests/🔬️unit/🦀️.rs`.
