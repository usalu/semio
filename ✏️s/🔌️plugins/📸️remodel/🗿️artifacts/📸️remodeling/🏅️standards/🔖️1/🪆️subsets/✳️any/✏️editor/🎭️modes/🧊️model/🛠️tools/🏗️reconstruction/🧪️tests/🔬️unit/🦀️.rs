use super::*;

const RUN_SCHEMA: &str = include_str!("../../../../../../🧵️reconstruction-session/🔣️.json");

#[test]
fn the_tool_declares_the_mutating_run_under_its_schema_id_and_label() {
    let schema: serde_json::Value = serde_json::from_str(RUN_SCHEMA).unwrap();
    let table = &schema["x-semio-toolRun"];
    let tool = definition();
    assert_eq!(tool.id, table["toolId"]);
    assert_eq!(tool.label, LocalizedLabel::native(table["label"]["en"].as_str().unwrap(), table["label"]["de"].as_str().unwrap()));
    assert_eq!(tool.keys, None, "the framework chords drive the run, never a tool-local binding");
    let run = tool.run.expect("the reconstruction tool declares its run");
    assert!(run.mutating, "a reconstruction result is document content");
    assert_eq!(run, reconstruction_run_definition());
    run.validate().expect("the run definition is valid");
}
