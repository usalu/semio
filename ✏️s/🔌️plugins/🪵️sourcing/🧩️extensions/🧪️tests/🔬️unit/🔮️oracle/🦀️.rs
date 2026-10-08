/// 🔎️ Compares real catalog JSON to the language-neutral typology and kind witnesses through serde_json.
pub fn verify(module_id: &str, typology_json: &str, kinds_json: &str) {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let case = cases.as_array().unwrap().iter().find(|row| row["moduleId"] == module_id).unwrap();
    let typology: serde_json::Value = serde_json::from_str(typology_json).unwrap();
    let kinds: serde_json::Value = serde_json::from_str(kinds_json).unwrap();
    assert_eq!(typology, case["typology"]);
    let kinds = kinds.as_array().unwrap();
    assert_eq!(serde_json::Value::Array(kinds.iter().map(|kind| kind["id"].clone()).collect()), case["kindIds"]);
    assert_eq!(serde_json::Value::Array(kinds.iter().map(|kind| kind["availability"].clone()).collect()), case["availability"]);
    assert!(kinds.iter().all(|kind| kind["moduleId"] == module_id));
    eprintln!("[DEBUG] Independent serde_json Sourcing catalog {module_id}: exact typology and {} kinds", kinds.len());
}
