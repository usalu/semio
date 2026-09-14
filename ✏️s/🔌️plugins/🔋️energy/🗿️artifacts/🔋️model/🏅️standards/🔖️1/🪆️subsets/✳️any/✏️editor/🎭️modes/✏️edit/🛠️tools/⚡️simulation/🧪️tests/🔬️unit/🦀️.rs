use super::*;

const RUN_SCHEMA: &str = include_str!("../../../../../../🧵️simulation-session/🔣️.json");

#[test]
fn the_tool_declares_the_read_only_run_under_its_schema_id_and_label() {
    let schema: serde_json::Value = serde_json::from_str(RUN_SCHEMA).unwrap();
    let table = &schema["x-semio-toolRun"];
    let tool = definition();
    assert_eq!(tool.id, table["toolId"]);
    assert_eq!(tool.label, LocalizedLabel::native(table["label"]["en"].as_str().unwrap(), table["label"]["de"].as_str().unwrap()));
    assert_eq!(tool.keys, None, "the framework chords start and stop the run, never a tool-local binding");
    let run = tool.run.expect("the simulation tool declares its run");
    assert!(!run.mutating, "adopting a simulation result never wrote the document, so the run is read-only");
    assert_eq!(run, energy_simulation_run_definition());
}
