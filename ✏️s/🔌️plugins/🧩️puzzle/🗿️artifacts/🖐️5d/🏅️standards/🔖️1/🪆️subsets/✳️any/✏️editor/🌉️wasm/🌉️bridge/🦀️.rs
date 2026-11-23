//! 🌐️ Puzzle 5d browser DSL parser. The envelope handle lives in the sibling envelope module.

pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::wasm::bridge::fault_bridge;

mod envelope;
pub use envelope::*;

use wasm_bindgen::prelude::*;

//#region 🔖️WasmBridge
/// 🔤️ Parses `.puzzle5d` DSL text (`Puzzle5dSnapshot`'s `dsl::DslArtifact` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.5d.json` fixture — lets
/// non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the
/// DSL grammar.
#[wasm_bindgen::prelude::wasm_bindgen(js_name = puzzle5dParseDslJson)]
pub fn puzzle5d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use pack::snapshot::ArtifactDsl;
    let projection = crate::Puzzle5dSnapshot::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    Ok(semio_framework_pack_json::to_json_string(&projection))
}
//#endregion 🔖️WasmBridge
