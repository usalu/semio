//! 🧪️ Tree drops use sibling-relative final indices and reject cycles.
use super::*;
use protocol::{Mutation, MutationDiff};

#[test]
fn layer_drop_shared_vectors_match_json_tree_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot: RasterSnapshot = dsl::os_pack::json::from_json_str(&fixture["document"].to_string()).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let payload = MoveLayer { layer_id: row["layerId"].as_str().unwrap().into(), target_row_id: row["targetRowId"].as_str().unwrap().into(), drop_position: row["dropPosition"].as_str().unwrap().into() };
        let resolved = resolve_drop(&payload, &snapshot).unwrap();
        if row["noOp"].as_bool().unwrap() { assert!(resolved.is_none(), "{row}"); continue; }
        let change = resolved.unwrap();
        assert_eq!(serde_json::json!({"parentId": change.parent_id, "index": change.index}), serde_json::json!({"parentId": row["parentId"], "index": row["index"]}), "{row}");
        let mutation = RasterMutation::ReorderLayers(change);
        let (diff, _) = mutation.diff(&snapshot).into_parts();
        let result = diff.apply(&snapshot).unwrap();
        let encoded: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&result)))).unwrap();
        let container = match row["parentId"].as_str() { Some(id) => &json_layer(&encoded["layers"], id).unwrap()["children"], None => &encoded["layers"] };
        assert_eq!(container[row["index"].as_u64().unwrap() as usize]["id"], row["layerId"], "{row}");
        assert_eq!(json_count(&encoded["layers"]), json_count(&fixture["document"]["layers"]));
    }
    for row in fixture["invalid"].as_array().unwrap() {
        let payload = MoveLayer { layer_id: row["layerId"].as_str().unwrap().into(), target_row_id: row["targetRowId"].as_str().unwrap().into(), drop_position: row["dropPosition"].as_str().unwrap().into() };
        assert!(resolve_drop(&payload, &snapshot).is_err(), "{row}");
    }
}

fn json_layer<'a>(layers: &'a serde_json::Value, id: &str) -> Option<&'a serde_json::Value> {
    layers.as_array()?.iter().find_map(|layer| if layer["id"] == id { Some(layer) } else { json_layer(&layer["children"], id) })
}

fn json_count(layers: &serde_json::Value) -> usize {
    layers.as_array().map_or(0, |layers| layers.iter().map(|layer| 1 + json_count(&layer["children"])).sum())
}
