# Design Notes

## Preserved Source Rationale

<a id="source-notes-56"></a>

### 🦀️.rs:56

#region 🔀️ArrayWire
🔌️ Gated (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02): `ToValue`/
`FromValue` above is the real wire codec every non-test caller uses. Plain `#[cfg(test)]`
alone would only cover THIS crate's own `🧪️tests/🧺️set/🦀️.rs` differential proof against real
`serde_json` — a downstream crate's `#[cfg(test)]` build (e.g. `os-flow`'s test target) never
activates a DEPENDENCY crate's own `#[cfg(test)]` code, yet `os-flow`'s `Widget::OutputPreview`/
`FlowPreviewGui` (`💻️os/🔨️modules/🌊️flow/🗿️artifact/🦀️.rs`) carry
`#[cfg_attr(test, derive(Serialize, Deserialize))]` over a field of this type: the derive macro
requires the `OrderedSet: Serialize`/`Deserialize` bound to exist at THEIR compile time, even
though it's never called at runtime (`ToValue`/`FromValue` is). So this also honors the
`ordered-set-serde` feature (see this crate's `Cargo.toml`), which `os-flow` enables on its
`semio-framework-replication` dependency for exactly that cross-crate derive to type-check.
