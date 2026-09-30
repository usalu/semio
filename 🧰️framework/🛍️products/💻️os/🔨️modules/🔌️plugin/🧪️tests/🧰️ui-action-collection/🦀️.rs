//! 🧰️ Generic action observation for native app composition laws.

use std::collections::BTreeSet;

fn collect(value: &serde_json::Value, scoped: bool, found: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(entries) => {
            if !scoped || (entries.contains_key("scope") && entries.contains_key("version")) {
                if let Some(name) = entries.get(if scoped { "name" } else { "action" }).and_then(serde_json::Value::as_str) {
                    found.insert(name.to_owned());
                }
            }
            for entry in entries.values() {
                collect(entry, scoped, found);
            }
        }
        serde_json::Value::Array(entries) => {
            for entry in entries {
                collect(entry, scoped, found);
            }
        }
        _ => {}
    }
}

/// 🕸️ Observes scoped action identities throughout a serialized UI projection.
pub fn emitted_action_ids(projection: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    collect(&serde_json::from_str(projection).expect("projection json"), true, &mut found);
    found
}

/// 🎛️ Observes action descriptors throughout a native window's chrome measures.
pub fn measure_action_ids(measures: &[crate::WindowMeasure]) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    collect(&serde_json::to_value(measures).expect("measures json"), false, &mut found);
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_neutral_action_collections_agree_with_json_pointer_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧰️ui-action-collection/🔣️.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let expected: BTreeSet<String> = serde_json::from_value(case["expected"].clone()).unwrap();
            let oracle: BTreeSet<String> = case["pointers"].as_array().unwrap().iter().map(|pointer| case["document"].pointer(pointer.as_str().unwrap()).unwrap().as_str().unwrap().to_owned()).collect();
            let scoped = case["kind"] == "projection";
            let mut actual = BTreeSet::new();
            collect(&case["document"], scoped, &mut actual);
            assert_eq!(oracle, expected);
            assert_eq!(actual, expected);
            if scoped {
                assert_eq!(emitted_action_ids(&case["document"].to_string()), expected);
            } else if case["kind"] == "measures" {
                let measures: Vec<crate::WindowMeasure> = serde_json::from_value(case["document"].clone()).unwrap();
                assert_eq!(measure_action_ids(&measures), expected);
            }
        }
    }
}
