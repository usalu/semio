//! 🧫️ Verifies the shared cycle, identity and cancellation corpus.
use super::{discover, DiscoveryEntry};

#[test]
fn cycle_safe_discovery_matches_closed_graph_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("discovery corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let nodes = &case["nodes"];
        let result = discover(
            case["root"].as_str().expect("root").to_owned(),
            |key: &String| Ok(nodes[key]["identity"].as_str().expect("identity").to_owned()),
            |key: &String| Ok(nodes[key]["entries"].as_array().expect("entries").iter().map(|entry| DiscoveryEntry { value: entry["value"].as_str().expect("entry").to_owned(), directory: entry["directory"].as_bool().expect("entry kind") }).collect()),
            |key: &String| key.ends_with(".grammar.semio"),
            |count, _| case["cancelAfter"].as_u64().is_none_or(|limit| (count as u64) < limit),
        ).expect("discovery");
        let mut files = result.files;
        files.sort();
        let expected = &case["expected"];
        assert_eq!(serde_json::json!({ "files": files, "directories": result.directories, "cancelled": result.cancelled }), *expected, "{}", case["id"]);
    }
    println!("[DEBUG] cycle-safe discovery: nine closed graph cases retained");
}
