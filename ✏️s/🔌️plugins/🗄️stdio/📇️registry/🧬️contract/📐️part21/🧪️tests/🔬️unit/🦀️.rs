use super::*;

const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\n#3=CARTESIAN_POINT('',(10.,10.,0.));\n#4=DIRECTION('',(0.,0.,1.));\n#5=VERTEX_POINT('',#1);\n#6=VERTEX_POINT('',#2);\n#7=VERTEX_POINT('',#3);\n#8=EDGE_CURVE('',#5,#6,#20,.T.);\n#9=EDGE_CURVE('',#6,#7,#21,.T.);\n#10=EDGE_CURVE('',#7,#5,#22,.T.);\n#20=LINE('',#1,#30);\n#21=LINE('',#2,#31);\n#22=LINE('',#3,#32);\n#30=VECTOR('',#4,1.);\n#31=VECTOR('',#4,1.);\n#32=VECTOR('',#4,1.);\n#11=ORIENTED_EDGE('',*,*,#8,.T.);\n#12=ORIENTED_EDGE('',*,*,#9,.T.);\n#13=ORIENTED_EDGE('',*,*,#10,.T.);\n#14=EDGE_LOOP('',(#11,#12,#13));\n#15=FACE_OUTER_BOUND('',#14,.T.);\n#16=PLANE('',#40);\n#40=AXIS2_PLACEMENT_3D('',#1,#4,$);\n#17=ADVANCED_FACE('',(#15),#16,.T.);\n#18=CLOSED_SHELL('',(#17));\n#19=MANIFOLD_SOLID_BREP('',#18);\nENDSEC;\nEND-ISO-10303-21;\n";

#[test]
fn round_trip_parse_serialize_reparse() {
    let doc = parse_part21(FIXTURE).expect("parse fixture");
    assert!(!doc.instances.is_empty());
    assert_eq!(doc.header.file_schema, vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])]);
    let text = write_part21(&doc);
    let reparsed = parse_part21(&text).expect("reparse generated text");
    assert_eq!(doc, reparsed, "round trip must be lossless at the graph level");
}

#[test]
fn instance_count_and_types_preserved() {
    let doc = parse_part21(FIXTURE).expect("parse");
    assert_eq!(doc.instances.len(), 26);
    assert_eq!(doc.by_type("CARTESIAN_POINT").count(), 3);
    assert_eq!(doc.by_type("ADVANCED_FACE").count(), 1);
    let derived_placement = doc.instance(40).expect("axis2placement");
    let args = derived_placement.entity("AXIS2_PLACEMENT_3D").expect("typed");
    assert!(matches!(args[3], Part21Value::Unset));
}

#[test]
fn oriented_edge_derived_attrs_are_star() {
    let doc = parse_part21(FIXTURE).expect("parse");
    let oe = doc.instance(11).expect("oriented edge");
    let args = oe.entity("ORIENTED_EDGE").expect("typed");
    assert_eq!(args[1], Part21Value::Derived);
    assert_eq!(args[2], Part21Value::Derived);
}

#[test]
fn complex_instance_keeps_every_type() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=(IFCQUANTITYAREA($,$,$,10.5,$)IFCPHYSICALSIMPLEQUANTITY($,$,$,$));\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(text).expect("parse complex instance");
    let inst = doc.instance(1).expect("instance 1");
    assert_eq!(inst.entities.len(), 2);
    assert_eq!(inst.entities[0].0, "IFCQUANTITYAREA");
    assert_eq!(inst.entities[1].0, "IFCPHYSICALSIMPLEQUANTITY");
    let round = write_part21(&doc);
    assert_eq!(parse_part21(&round).expect("reparse"), doc);
}

#[test]
fn typed_value_wrapper_round_trips() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROPERTYSINGLEVALUE('Height',$,IFCLENGTHMEASURE(3000.),$);\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(text).expect("parse");
    let args = doc.instance(1).unwrap().entity("IFCPROPERTYSINGLEVALUE").unwrap();
    let (name, inner) = args[2].as_typed().expect("typed value");
    assert_eq!(name, "IFCLENGTHMEASURE");
    assert_eq!(inner[0].as_real(), Some(3000.0));
    assert_eq!(parse_part21(&write_part21(&doc)).unwrap(), doc);
}

#[test]
fn string_escapes_round_trip() {
    for raw in ["it's a test", "unicode: \u{20AC} \u{4E2D}\u{6587}", "back\\slash", "", "plain"] {
        let text = format!("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('{}');\nENDSEC;\nEND-ISO-10303-21;\n", escape_part21_string(raw));
        let doc = parse_part21(&text).unwrap_or_else(|e| panic!("parse {raw:?}: {e}"));
        let got = doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str().unwrap();
        assert_eq!(got, raw, "escape round trip for {raw:?}");
    }
}

#[test]
fn doubled_quote_escape() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('it''s here');\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(text).expect("parse");
    assert_eq!(doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str(), Some("it's here"));
}

#[test]
fn unicode_x2_escape() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('\\X2\\4E2D6587\\X0\\');\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(text).expect("parse");
    assert_eq!(doc.instance(1).unwrap().entity("LABEL").unwrap()[0].as_str(), Some("中文"));
}

/// 🧪️ ISO 10303-21 §6.4.2's remaining control directives, each read back from a literal
/// spelled the way the standard spells it. The `\\` row is the one the real committed
/// IfcOpenShell export carries (`'\\'` at byte 138718 of `🏗️nakagin-capsule-tower.ifc`) and the
/// one this lexer used to reject outright, failing all 22 executable `🏗️mutate-ifc-4` subject
/// scenarios while `ruststep` read the same file without complaint. `\X\` carries NO
/// terminator per the grammar's own `arbitrary = "\X\" hex_one`, which is why the `\X\41\S\A`
/// row matters: demanding one would swallow the next directive's opener.
#[test]
fn every_iso_10303_21_string_directive_is_read() {
    let label = |literal: &str| {
        let text = format!("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('{literal}');\nENDSEC;\nEND-ISO-10303-21;\n");
        parse_part21(&text).unwrap_or_else(|e| panic!("parse {literal:?}: {e}")).instance(1).unwrap().entity("LABEL").unwrap()[0].as_str().unwrap().to_string()
    };
    assert_eq!(label(r"\\"), "\\", "the doubled reverse solidus is ONE literal backslash");
    assert_eq!(label(r"a\\b"), "a\\b");
    assert_eq!(label(r"\X\41"), "A", "\\X\\ takes exactly two hex digits and no terminator");
    assert_eq!(label(r"\X\41\S\A"), "A\u{00C1}", "\\X\\ must not swallow the next directive's opener");
    assert_eq!(label(r"\S\A"), "\u{00C1}", "\\S\\ shifts the character by 128 on the default alphabet");
    assert_eq!(label(r"\PA\\S\A"), "\u{00C1}", "\\PA\\ selects ISO 8859-1, which is the default");
    assert_eq!(label(r"\X4\0001F600\X0\"), "\u{1F600}", "\\X4\\ reads UCS-4 groups");
    assert_eq!(label(r"\X2\4E2D6587\X0\"), "中文");
}

/// 🧪️ Undefined shifted positions are typed refusals instead of replacement characters.
#[test]
fn undefined_iso_8859_positions_are_refused() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=LABEL('\\PF\\\\S\\!');\nENDSEC;\nEND-ISO-10303-21;\n";
    let error = parse_part21(text).expect_err("page F byte A1 is undefined");
    assert!(error.to_string().contains("ISO 8859 page F"), "the refusal must identify the undefined page: {error}");
}

#[test]
fn unset_and_derived_values() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=THING($,*,1);\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(text).expect("parse");
    let args = doc.instance(1).unwrap().entity("THING").unwrap();
    assert_eq!(args[0], Part21Value::Unset);
    assert_eq!(args[1], Part21Value::Derived);
    assert_eq!(args[2], Part21Value::Int(1));
}

#[test]
fn nested_lists_round_trip() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=MATRIX(((1.,0.),(0.,1.)));\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(text).expect("parse");
    let args = doc.instance(1).unwrap().entity("MATRIX").unwrap();
    let outer = args[0].as_list().expect("outer list");
    assert_eq!(outer.len(), 2);
    assert_eq!(outer[0].as_list().unwrap()[0].as_real(), Some(1.0));
    assert_eq!(parse_part21(&write_part21(&doc)).unwrap(), doc);
}

#[test]
fn malformed_input_is_typed_error_not_fabrication() {
    let bad = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('X'));\nENDSEC;\nDATA;\n#1=THING(;\nENDSEC;\nEND-ISO-10303-21;\n";
    assert!(parse_part21(bad).is_err());
}

/// 🌱️ The value projection IS the contract shape (`Part21Value`/`Part21Instance`/`Part21Header` in the ifc 2x3 schema and its
/// TS twin), for every value kind — serde_json is the independent reader.
#[test]
fn value_projection_is_the_canonical_contract_shape() {
    let document = parse_part21(
        "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','z');\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\n#1=X(#2,'s',.E.,3,1.5,(1,2),Y(3.),$,*);\n#2=(A(1)B(2));\nENDSEC;\nEND-ISO-10303-21;\n",
    )
    .expect("parses");
    let json: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&document)).expect("json");
    assert_eq!(json["header"]["fileSchema"], serde_json::json!([{ "kind": "list", "values": [{ "kind": "str", "value": "IFC2X3" }] }]));
    assert_eq!(
        json["instances"][0],
        serde_json::json!({ "id": 1, "entities": [{ "typeName": "X", "arguments": [
            { "kind": "ref", "value": 2 }, { "kind": "str", "value": "s" }, { "kind": "enum", "value": "E" }, { "kind": "int", "value": 3 },
            { "kind": "real", "value": { "negative": false, "coefficient": "15", "scale": 1 } },
            { "kind": "list", "values": [{ "kind": "int", "value": 1 }, { "kind": "int", "value": 2 }] },
            { "kind": "typed", "typeName": "Y", "values": [{ "kind": "real", "value": { "negative": false, "coefficient": "3", "scale": 0 } }] },
            { "kind": "unset" }, { "kind": "derived" }
        ] }] })
    );
    assert_eq!(json["instances"][1]["entities"], serde_json::json!([{ "typeName": "A", "arguments": [{ "kind": "int", "value": 1 }] }, { "typeName": "B", "arguments": [{ "kind": "int", "value": 2 }] }]));
    let reopened: Part21Document = semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_json_string(&document), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the projection decodes");
    assert_eq!(reopened, document);
    assert!(semio_framework_pack_json::from_json_str::<Part21Value>(r#"{"kind":"str","value":"s","extra":1}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "an undeclared member is refused");
    assert!(semio_framework_pack_json::from_json_str::<Part21Value>(r#"{"kind":"blob"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "an unknown kind is refused");
}

/// ✍️ The canonical writer is a fixed point: writing a parsed canonical text reproduces it byte for byte.
#[test]
fn canonical_text_is_a_fixed_point() {
    let canonical = write_part21(&parse_part21(FIXTURE).expect("parse"));
    assert_eq!(write_part21(&parse_part21(&canonical).expect("reparse canonical")), canonical);
}

/// 🧭️ Path edits address the projection: an argument inside a complex instance edits in place.
#[test]
fn path_edits_address_the_projection() {
    let mut document = parse_part21(FIXTURE).expect("parse");
    let path = ["instances", "0", "entities", "0", "arguments", "0"];
    document.edit_value_at_path(&path, ValueEdit::Set(DslValue::object([("kind".to_string(), DslValue::String("str".into())), ("value".to_string(), DslValue::String("edited".into()))]))).expect("argument edit");
    assert_eq!(document.instances[0].entities[0].1[0], Part21Value::Str("edited".into()));
}

#[path = "../🚦️sqlite-cohort/🦀️.rs"]
mod sqlite_cohort;


#[test]
fn binary_literals_preserve_padding_and_refuse_malformed_content() {
    for content in ["0", "0F", "17", "23", "31", "00ABCDEF"] {
        let source = format!("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;#1=IFCBLOB(\"{content}\");ENDSEC;END-ISO-10303-21;");
        let document = parse_part21(&source).unwrap();
        assert_eq!(document.instances[0].entities[0].1[0], Part21Value::Binary(content.into()));
        let printed = write_part21(&document);
        assert_eq!(parse_part21(&printed).unwrap(), document);
    }
    for content in ["", "4", "1", "18", "24", "32", "0G"] {
        let source = format!("ISO-10303-21;HEADER;ENDSEC;DATA;#1=IFCBLOB(\"{content}\");ENDSEC;END-ISO-10303-21;");
        assert!(parse_part21(&source).is_err(), "{content}");
    }
}

#[test]
fn supplementary_unicode_round_trips_without_invalid_x2_groups() {
    let literal = escape_part21_string("Ä😀中");
    assert_eq!(literal, r"\X4\000000C40001F60000004E2D\X0\");
    let text = FIXTURE.replace("CARTESIAN_POINT('',", &format!("CARTESIAN_POINT('{}',", literal));
    assert_eq!(parse_part21(&text).unwrap().instance(1).unwrap().entities[0].1[0].as_str(), Some("Ä😀中"));
}

#[test]
fn x2_utf16_surrogate_pairs_decode_as_one_scalar() {
    let text = FIXTURE.replace("CARTESIAN_POINT('',", r"CARTESIAN_POINT('\X2\D83DDE00\X0\',");
    assert_eq!(parse_part21(&text).unwrap().instance(1).unwrap().entities[0].1[0].as_str(), Some("😀"));
}

#[test]
fn unicode_directives_reject_unpaired_surrogates_and_invalid_scalars() {
    for literal in [r"\X2\D83D\X0\", r"\X2\DE00\X0\", r"\X2\D83D0041\X0\", r"\X4\00110000\X0\", r"\X4\0000D800\X0\", r"\S\Ä"] {
        let text = FIXTURE.replace("CARTESIAN_POINT('',", &format!("CARTESIAN_POINT('{}',", literal));
        assert!(parse_part21(&text).is_err(), "accepted malformed directive {literal}");
    }
}

#[test]
fn separately_encoded_header_instances_and_footer_match_canonical_and_independent_step(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../✍️writer/🧪️fixtures/🔣️stream.json")).unwrap();let doc=parse_part21(fixture["input"].as_str().unwrap()).unwrap();let mut text=write_part21_header(&doc.header);for instance in &doc.instances{text.push_str(&write_part21_instance(instance));}text.push_str(write_part21_footer());assert_eq!(text,write_part21(&doc));let reference=semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::oracle_round_trip(text.as_bytes()).unwrap();assert_eq!(semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::project_step_ap214_any(text.as_bytes()).unwrap(),semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::project_step_ap214_any(&reference).unwrap());
}
