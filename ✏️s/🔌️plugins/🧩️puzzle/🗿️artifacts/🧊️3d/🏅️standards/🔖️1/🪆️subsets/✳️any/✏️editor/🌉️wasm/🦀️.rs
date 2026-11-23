//! 🌐️ Puzzle 3d browser DSL parser. Envelope loading lives in the sibling envelope module.

#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]

mod envelope;
pub use envelope::*;

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use wasm_bindgen::prelude::*;

/// 🔤️ Parses `.puzzle3d` DSL text (`Puzzle3dSnapshot`'s `dsl::DslArtifact` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.3d.json` scene_snapshot — lets
/// non-Rust consumers load the real example fixtures without duplicating the DSL grammar.
#[wasm_bindgen(js_name = puzzle3dParseDslJson)]
pub fn puzzle3d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use pack::snapshot::ArtifactDsl;
    let projection = Puzzle3dSnapshot::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    Ok(semio_framework_pack_json::to_json_string(&projection))
}
