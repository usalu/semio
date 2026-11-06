# Design Notes

## Preserved Source Rationale

<a id="source-notes-86"></a>

### 🦀️.rs:86

📖️ Reads `<owner_root>/🔣️.json` and decodes it as a real `semio_framework::PackageDescriptor`
— the same JSON `semio-plugin-describe` writes (packet E1/E2, `📓️status.md`'s
"E2-builder-descriptor" entry) and the same type `🏃️run/🦀️.rs`'s own `WasmtimeNodeHost`
doc comment names as the eventual manifest source, once wired. Reading the COMMITTED descriptor
JSON directly (rather than instantiating the wasm to call `describe()` at runtime, the way
`semio-plugin-describe` does at BUILD time) is deliberate: `🏃️run`'s own
`load_runtime_recursive` is blocked precisely because no `describe()`-decoder is wired into that
crate — this sidesteps that gap entirely for any plugin that (like `🗒️note`) already has a real
committed descriptor, without needing a second wasmtime linkage inside the gateway just to read a
manifest.
