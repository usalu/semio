//! 🧪 Native source and registered composition contracts over shared informative IFC inputs.
use super::*;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin::{ArtifactComposer, ArtifactSerializer};

fn minimal(data: &str) -> String {
    format!("ISO-10303-21;HEADER;FILE_DESCRIPTION(('test'),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;{data}ENDSEC;END-ISO-10303-21;")
}

#[test]
fn official_models_roundtrip_the_complete_graph() {
    for (text, count) in [(include_str!("triangulated.ifc"), 35), (include_str!("alignment.ifc"), 3161), (include_str!("structural.ifc"), 188)] {
        let snapshot = parse_ifc4x3_step(text).unwrap();
        assert_eq!(snapshot.entities.len(), count);
        assert_eq!(parse_ifc4x3_step(&write_ifc4x3_step(&snapshot).unwrap()).unwrap(), snapshot);
        assert_eq!(IfcSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
        assert_eq!(snapshot.schema, "stdio.ifc");
    }
}

#[test]
fn strict_source_refuses_foreign_and_malformed_graphs() {
    for text in [
        minimal("#1=IFCA();#1=IFCA();"),
        minimal("#0=IFCA();"),
        minimal("#1=IFCA((#2));"),
        minimal("#1=IFCA(1.E999);"),
        minimal("").replace("IFC4X3_ADD2", "IFC4"),
        minimal("").replace("HEADER;", "HEADER;EXTRA();"),
        minimal("").replace("END-ISO-10303-21;", ""),
        minimal("") + "TRAILING;",
    ] {
        assert!(parse_ifc4x3_step(&text).is_err(), "{text}");
    }
}

#[test]
fn exact_values_and_cancellation_survive_the_native_boundary() {
    let text = minimal("#9007199254740993=(IFCA('a''b',-0.,IFCLENGTHMEASURE(1.E-12),\"31\") IFCB((#9007199254740993,$,*),.T.));");
    let snapshot = parse_ifc4x3_step(&text).unwrap();
    assert_eq!(snapshot.entities[0].id, 9007199254740993);
    assert!(matches!(snapshot.entities[0].args[1], IfcValue::Real(value) if value.to_bits() == (-0.0f64).to_bits()));
    assert_eq!(snapshot.entities[0].args[3], IfcValue::Binary("31".into()));
    assert_eq!(parse_ifc4x3_step(&write_ifc4x3_step(&snapshot).unwrap()).unwrap(), snapshot);
    let mut calls = 0;
    assert!(parse_ifc4x3_step_controlled(&text, &mut |_| {
        calls += 1;
        calls < 4
    })
    .is_err());
    let original = snapshot.clone();
    assert!(write_ifc4x3_step_controlled(&snapshot, &mut |_| false).is_err());
    assert_eq!(snapshot, original);
}

#[test]
fn composer_and_exporter_use_the_registered_dialects() {
    let text = minimal("#1=IFCCARTESIANPOINT((0.,1.));");
    let sources = [ComposeSource { dialect: TXT_DIALECT, payload: AnalyzeSource::Text(&text) }];
    let snapshot = <Ifc4x3Composer as ArtifactComposer>::compose(&sources).unwrap().snapshot;
    assert_eq!(<Ifc4x3Composer as ArtifactComposer>::WRITES, IFC4X3_DIALECT);
    let exported = semio_framework_async::poll::resolve_ready(Ifc4x3ToTxt::serialize(&snapshot)).unwrap();
    assert_eq!(parse_ifc4x3_step(&exported.to_body()).unwrap(), snapshot);
    let rows = entries();
    assert!(rows.iter().any(|row| row.writes == IFC4X3_DIALECT && row.reads.contains(&TXT_DIALECT)));
    assert!(rows.iter().any(|row| row.writes == TXT_DIALECT && row.reads.contains(&IFC4X3_DIALECT)));
    let native = snapshot.encode_pack();
    let sources = [ComposeSource { dialect: IFC4X3_DIALECT, payload: AnalyzeSource::Binary(&native) }];
    assert_eq!(<Ifc4x3Composer as ArtifactComposer>::compose(&sources).unwrap().snapshot, snapshot);
}

#[test]
fn erased_entries_execute_natural_source_and_native_pack_routes() {
    use semio_framework_plugin::io::{ErasedComposeSource, IoPayload};
    let text = minimal("#1=IFCCARTESIANPOINT((0.,1.));");
    let input = TxtSnapshot::from_body(&text);
    let import = entries().iter().find(|entry| entry.writes == IFC4X3_DIALECT && entry.reads.contains(&TXT_DIALECT)).unwrap();
    let imported = semio_framework_async::poll::resolve_ready((import.compose)(&[ErasedComposeSource { dialect: TXT_DIALECT, payload: IoPayload::Binary(input.encode_pack()) }])).unwrap();
    assert_eq!(imported.dialect, IFC4X3_DIALECT);
    let IoPayload::Binary(bytes) = imported.payload else {
        panic!("native graph pack expected");
    };
    let snapshot = IfcSnapshot::decode_pack(&bytes).unwrap();
    assert_eq!(snapshot.header.file_schema, vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])]);
    let export = entries().iter().find(|entry| entry.writes == TXT_DIALECT && entry.reads.contains(&IFC4X3_DIALECT)).unwrap();
    let exported = semio_framework_async::poll::resolve_ready((export.compose)(&[ErasedComposeSource { dialect: IFC4X3_DIALECT, payload: IoPayload::Binary(bytes) }])).unwrap();
    let IoPayload::Binary(bytes) = exported.payload else {
        panic!("text artifact pack expected");
    };
    let source = TxtSnapshot::decode_pack(&bytes).unwrap().to_body();
    assert!(source.starts_with("ISO-10303-21;"));
    assert_eq!(parse_ifc4x3_step(&source).unwrap(), snapshot);
    let mut foreign = snapshot;
    foreign.header.file_schema = vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])];
    let invalid = [ErasedComposeSource { dialect: IFC4X3_DIALECT, payload: IoPayload::Binary(foreign.encode_pack()) }];
    assert!(semio_framework_async::poll::resolve_ready((import.compose)(&invalid)).is_err());
    assert!(semio_framework_async::poll::resolve_ready((export.compose)(&invalid)).is_err());
    assert_eq!(entries().len(), 2);
}

#[test]
fn source_limits_cover_headers_escaped_text_and_complete_framing() {
    let mut snapshot = parse_ifc4x3_step(&minimal("")).unwrap();
    snapshot.header.file_name[0] = IfcValue::String("a".repeat(200));
    assert_eq!(write_with_limits(&snapshot, &mut |_| true, 4096, 128).unwrap_err().code, "step.record-limit");
    snapshot.header.file_name[0] = IfcValue::String("ä".repeat(30));
    assert_eq!(write_with_limits(&snapshot, &mut |_| true, 4096, 128).unwrap_err().code, "step.record-limit");
    snapshot.header.file_name[0] = IfcValue::String("short".into());
    snapshot.entities = parse_ifc4x3_step(&minimal("#1=IFCA();")).unwrap().entities;
    let source = write_ifc4x3_step(&snapshot).unwrap();
    assert_eq!(write_with_limits(&snapshot, &mut |_| true, source.len() - 1, MAX_RECORD_BYTES).unwrap_err().code, "step.source-limit");
    assert_eq!(write_with_limits(&snapshot, &mut |_| true, source.len(), MAX_RECORD_BYTES).unwrap(), source);
    assert_eq!(parse_ifc4x3_step(&source).unwrap(), snapshot);
    let mut records = Records { text: &source, at: 0, max_record_bytes: MAX_RECORD_BYTES };
    let mut largest = 0;
    while let Some(record) = records.next(&mut |_| true).unwrap() {
        largest = largest.max(record.len());
    }
    assert!(write_with_limits(&snapshot, &mut |_| true, source.len(), largest).is_ok());
    assert_eq!(write_with_limits(&snapshot, &mut |_| true, source.len(), largest - 1).unwrap_err().code, "step.record-limit");
}

#[test]
fn semantic_source_errors_keep_original_nested_token_byte_offsets() {
    for (last, code, token) in [("#1=IFCB();", "step.duplicate-id", "#1"), ("#2=IFCB((IFCLABEL('inside'),(#999)));", "step.reference", "#999"), ("#2=IFCB((IFCLENGTHMEASURE(1.E999)));", "step.real", "1.E999")] {
        let source = minimal(&format!("/* ä 🏗️ #999 1.E999 */\n#1=IFCA('ä 🏗️ #999 1.E999');\n{last}"));
        let error = parse_ifc4x3_step(&source).unwrap_err();
        assert_eq!(error.code, code);
        assert_eq!(error.offset, source.rfind(token).unwrap());
    }
}

#[test]
fn neutral_alphabet_and_print_controls_cross_the_native_ifc_boundary() {
    let alphabets: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets/cases.json")).unwrap();
    for row in alphabets["cases"].as_array().unwrap() {
        let source = minimal(&format!("#1=IFCLABEL('{}');", row["source"].as_str().unwrap()));
        let result = parse_ifc4x3_step(&source);
        if let Some(expected) = row["expected"].as_str() { assert_eq!(result.unwrap_or_else(|error| panic!("{}: {error}", row["name"])).entities[0].args[0], IfcValue::String(expected.into()), "{}", row["name"]); }
        else { assert!(result.is_err(), "{}", row["name"]); }
    }
    let cases: serde_json::Value = serde_json::from_str(include_str!("control-cases.json")).unwrap();
    for row in cases["lexical"].as_array().unwrap() {
        let snapshot = parse_ifc4x3_step(&minimal(row["data"].as_str().unwrap())).unwrap_or_else(|error| panic!("{}: {error}", row["name"]));
        assert_eq!(snapshot.entities[0].name, row["nameAfter"].as_str().unwrap());
        let expected = if let Some(binary) = row["binary"].as_str() { IfcValue::Binary(binary.into()) } else { IfcValue::String(row["value"].as_str().unwrap().into()) };
        assert_eq!(snapshot.entities[0].args[0], expected, "{}", row["name"]);
    }
}

#[test]
fn native_lexical_source_offsets_and_scanner_cancellation_are_original() {
    let source = minimal("#1=IFCLABEL('ä',\r\n?);");
    assert_eq!(parse_ifc4x3_step(&source).unwrap_err().offset, source.find('?').unwrap());
    let source = minimal("#1=IFCLABEL('x');").replace("FILE_SCHEMA", "FILE_\r\nSCHEMA");
    assert!(parse_ifc4x3_step(&source).is_ok());
    for text in [format!("/*{}*/", "x".repeat(16384)), format!("#1=IFCLABEL('{}');", "x".repeat(16384)), format!("#1=IFCLABEL('\\S\\{}');", "\n".repeat(16384))] {
        let mut seen = false;
        let result = parse_ifc4x3_step_controlled(&minimal(&text), &mut |progress| { if progress.phase == "scan" && progress.completed >= 4096 { seen = true; false } else { true } });
        assert!(seen); assert_eq!(result.unwrap_err().code, "io.cancelled");
    }
    assert!(parse_ifc4x3_step(&minimal("#1=IFCLABEL(1E5);")).is_err());
}
