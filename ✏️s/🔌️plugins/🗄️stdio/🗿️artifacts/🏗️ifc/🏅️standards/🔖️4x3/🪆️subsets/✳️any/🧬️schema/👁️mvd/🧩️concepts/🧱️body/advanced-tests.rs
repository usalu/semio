//! 🧪️ Native AdvancedSweptSolid shares the exact audited reference contract and fixed Body owner.
use super::*;
use super::native_tests::evaluate;
use super::tessellation_tests::{graph, request};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
#[test]
fn native_body_advanced_swept_matches_all_twenty_four_selected_neutral_rows() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🌪️advanced-swept-solid/cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let source = graph(row);
        evaluate(BodyPolicy::AdvancedSweptSolid, &source, &request(&[("1", row["applicable"].as_bool())]), |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap());
            assert_eq!(report.targets[0].status, report.status);
            assert_eq!(report.concept, "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_AdvancedSweptSolid_Geometry");
            assert_eq!(report.source.sha256, "c492bb1ef2c769a85c1b4dfd6d4e7caf03614bd03b9b8bf59669d705dd04658b");
            assert_eq!(report.source.commit, "610ee3f95db9d0380b1eb8de4dc6e17833531518");
        });
    }
}
#[test]
fn native_body_advanced_swept_rejects_schema_only_horizontal_sectioned_family() {
    let source = graph(&serde_json::json!({"identifier":"Body","type":"AdvancedSweptSolid","items":["IfcSectionedSolidHorizontal"]}));
    evaluate(BodyPolicy::AdvancedSweptSolid, &source, &request(&[("1", Some(true))]), |report| {
        assert_eq!(report.status, "fail");
        let diagnostic = &report.targets[0].diagnostics[0];
        assert_eq!(diagnostic.code, "mvd.items.type");
        assert_eq!(diagnostic.pointer.as_deref(), Some("/entities/0/args/3/value/0"));
        assert_eq!(diagnostic.label.en, "A representation item is neither a swept area solid nor a swept disk solid.");
        assert_eq!(diagnostic.label.de, "Ein Repräsentationselement ist weder ein Flächen-Sweep-Körper noch ein Kreis-Sweep-Körper.");
    });
}
#[test]
fn native_body_advanced_swept_two_family_cursor_waits_for_late_unknown_and_survives_restart_move() {
    for names in [
        &["IFCEXTRUDEDAREASOLIDTAPERED", "IFCUNKNOWN"][..],
        &["IFCSWEPTDISKSOLIDPOLYGONAL", "IFCUNKNOWN"][..],
        &["IFCSWEPTAREASOLID", "IFCSWEPTDISKSOLID", "IFCUNKNOWN"][..],
    ] {
        let mut source = graph(&serde_json::json!({"identifier":"Body","type":"AdvancedSweptSolid","items":[names[0]]})).as_ref().clone();
        source.entities[1].complex = names[1..].iter().map(|name| IfcComplexType { name: (*name).into(), args: vec![] }).collect();
        let source = Arc::new(source);
        evaluate(BodyPolicy::AdvancedSweptSolid, &source, &request(&[("1", Some(true))]), |report| assert_eq!(report.status, "incomplete"));
        let mut cursor = BodyFamilyMembership::new(BodyPolicy::AdvancedSweptSolid);
        cursor.start(1);
        assert_eq!(cursor.result(), BodyFamilyResult::Pending);
        let mut done = false;
        for _ in 0..4096 {
            let mut moved = cursor;
            done = moved.advance(source.as_ref());
            cursor = moved;
            if done { break; }
            assert_eq!(cursor.result(), BodyFamilyResult::Pending);
        }
        assert!(done);
        assert_eq!(cursor.result(), BodyFamilyResult::Unknown);
        cursor.start(1);
        assert_eq!(cursor.result(), BodyFamilyResult::Pending);
        while !cursor.advance(source.as_ref()) {}
        assert_eq!(cursor.result(), BodyFamilyResult::Unknown);
    }
}
