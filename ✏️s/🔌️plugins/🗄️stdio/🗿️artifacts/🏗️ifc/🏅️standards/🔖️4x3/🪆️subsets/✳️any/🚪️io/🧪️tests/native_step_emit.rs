//! 🧪 Bounded native STEP fragments preserve the complete admitted source graph.
use super::*;

fn minimal(data: &str) -> String { format!("ISO-10303-21;HEADER;FILE_DESCRIPTION(('test'),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;{data}ENDSEC;END-ISO-10303-21;") }

#[test]
fn complete_neutral_graphs_roundtrip_through_actual_bounded_emitter() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("step-emission.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let snapshot = parse_ifc4x3_step(&minimal(row["data"].as_str().unwrap())).unwrap();
        let mut cursor = emit::IfcStepEmitCursor::new();
        let mut output = Vec::new();
        while let Some(chunk) = cursor.next(&snapshot).unwrap() { assert!(chunk.as_bytes().len() <= cases["fragmentBytes"].as_u64().unwrap() as usize); output.extend_from_slice(chunk.as_bytes()); }
        let output = String::from_utf8(output).unwrap();
        assert_eq!(parse_ifc4x3_step(&output).unwrap(), snapshot, "{}", row["name"]);
        if row["name"] == "Exact Integer and Negative Zero" { assert!(matches!(parse_ifc4x3_step(&output).unwrap().entities[0].args[1], IfcValue::Real(value) if value.to_bits() == (-0.0f64).to_bits())); }
    }
}

#[test]
fn one_large_source_string_never_becomes_one_output_fragment() {
    let snapshot = parse_ifc4x3_step(&minimal(&format!("#1=IFCA('{}');", "ä😀'".replace('\'', "''").repeat(4096)))).unwrap();
    let pointer = match &snapshot.entities[0].args[0] { IfcValue::String(value) => value.as_ptr(), _ => unreachable!() };
    let mut cursor = emit::IfcStepEmitCursor::new();
    let mut count = 0;
    while let Some(chunk) = cursor.next(&snapshot).unwrap() { assert!(chunk.as_bytes().len() <= 64); count += 1; }
    assert!(count > 1024);
    assert_eq!(match &snapshot.entities[0].args[0] { IfcValue::String(value) => value.as_ptr(), _ => unreachable!() }, pointer);
}
