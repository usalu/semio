//! 📏️ Counting: the unified line counters.

use super::extension_of;

//#region 📏️Counting

/// 🔢️ Newline-terminated line count, `wc -l` style: a non-empty file has at least one line.
pub fn physical_line_count(data: &[u8]) -> usize {
    if data.is_empty() {
        return 0;
    }
    data.iter().filter(|byte| **byte == b'\n').count() + 1
}

/// 🧮️ Recursively counts the object keys of a JSON value; arrays contribute only their members.
pub fn count_json_keys(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Object(map) => map.len() + map.values().map(count_json_keys).sum::<usize>(),
        serde_json::Value::Array(items) => items.iter().map(count_json_keys).sum(),
        _ => 0,
    }
}

/// 🗝️ Object-key count of a JSON document, `0` when the bytes are blank or not valid JSON.
pub fn json_key_count(data: &[u8]) -> usize {
    let text = std::str::from_utf8(data).unwrap_or("").trim();
    if text.is_empty() {
        return 0;
    }
    serde_json::from_str::<serde_json::Value>(text).map_or(0, |value| count_json_keys(&value))
}

/// 📄️ Unified LOC of one tracked file body: JSON keys for `.json`/`.jsonc`, physical lines else.
///
/// This is the repository-wide definition of "one line of code". It is identical to the
/// TypeScript library's `countUnifiedLocForFile`, which the language-agnostic
/// `loc-aggregation` case uses as its cross-implementation reference.
pub fn count_unified_loc_for_file(rel: &str, data: &[u8]) -> usize {
    match extension_of(rel).as_str() {
        ".json" | ".jsonc" => {
            let keys = json_key_count(data);
            if keys > 0 {
                return keys;
            }
            if std::str::from_utf8(data).unwrap_or("").trim().is_empty() {
                return 0;
            }
            physical_line_count(data)
        }
        _ => physical_line_count(data),
    }
}

//#endregion 📏️Counting
