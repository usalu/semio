# 🕸️ WASM Build

Compiles one browser package with wasm-pack or cargo plus wasm-bindgen under an explicit policy, and hands the finite package to one consumer.

| Member | Role |
| --- | --- |
| `buildWasmWebV1` | Admits the Cargo Source, retains the admitted bodies, compiles, proves the consumed bytes and consumes the package. |
| `runWasmCargoCommandV1` | Runs one owned compiler command under the drift watch; with `retained`, an edit to a retained file is deferred instead of cancelling. |
| `ProducedWasmPackageViewV1.sources` | Provenance of the artifact: admitted roster digest, proof count and deferred edits. |
| `verifyProducedWasmPackageV1`, `withProducedWasmPackageTaskV1` | Recheck and use the retained package while its store is held. |
| `readWasmBuildPolicyV1`, `wasmCompilerSelectionArgumentsV1`, `wasmPackBuildArgumentsV1` | Policy and closed compiler selection. |

The source mechanism is shared with the descriptor emitter: [retained Cargo Source](../🏗️native-build/📥️source/📎️retained/README.md).
