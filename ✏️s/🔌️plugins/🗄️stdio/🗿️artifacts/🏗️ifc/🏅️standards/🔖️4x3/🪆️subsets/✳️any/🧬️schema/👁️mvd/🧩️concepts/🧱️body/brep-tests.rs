//! 🧪️ Native Brep policies retain exact released alternatives and shared Body authority.
use super::*;
use super::native_tests::evaluate;
use super::tessellation_tests::{graph, request};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
#[test]
fn native_body_brep_policies_match_all_thirty_eight_neutral_rows() {
    for (policy, input, hash) in [
        (BodyPolicy::Brep, include_str!("../🧊️brep/cases.json"), "1e1658d668430d01c2f796a3d6ca217ebd89a9f7c18b9a17d9937ff170cfed17"),
        (BodyPolicy::AdvancedBrep, include_str!("../💎️advanced-brep/cases.json"), "368bf65e75dcf96e3a6c778fc6ac29be57b8f1bea2f4315e8df0ea68fa4eb5a1"),
    ] {
        let cases: serde_json::Value = serde_json::from_str(input).unwrap();
        for row in cases["cases"].as_array().unwrap() {
            let source = graph(row);
            evaluate(policy, &source, &request(&[("1", row["applicable"].as_bool())]), |report| {
                assert_eq!(report.status, row["expected"].as_str().unwrap());
                assert_eq!(report.targets[0].status, report.status);
                assert_eq!(report.concept, policy.concept());
                assert_eq!(report.source.sha256, hash);
            });
        }
    }
}
#[test]
fn native_body_brep_rejects_advanced_while_advanced_brep_rejects_the_abstract_only_projection() {
    for (policy, name, en, de) in [
        (BodyPolicy::Brep, "IFCADVANCEDBREPWITHVOIDS", "A representation item is not a faceted boundary representation solid.", "Ein Repräsentationselement ist kein facettierter BRep-Körper."),
        (BodyPolicy::AdvancedBrep, "IFCMANIFOLDSOLIDBREP", "A representation item is neither an advanced nor a faceted boundary representation solid.", "Ein Repräsentationselement ist weder ein erweiterter noch ein facettierter BRep-Körper."),
    ] {
        let source = graph(&serde_json::json!({"identifier":"Body","type":policy.representation_type(),"items":[name]}));
        evaluate(policy, &source, &request(&[("1", Some(true))]), |report| {
            assert_eq!(report.status, "fail");
            let diagnostic = &report.targets[0].diagnostics[0];
            assert_eq!(diagnostic.code, "mvd.items.type");
            assert_eq!(diagnostic.pointer.as_deref(), Some("/entities/0/args/3/value/0"));
            assert_eq!(diagnostic.label.en, en);
            assert_eq!(diagnostic.label.de, de);
        });
    }
}
#[test]
fn native_body_brep_voids_keep_late_unknown_dominance_without_admitting_physical_validity() {
    for (policy, name) in [
        (BodyPolicy::Brep, "IFCFACETEDBREPWITHVOIDS"),
        (BodyPolicy::AdvancedBrep, "IFCADVANCEDBREPWITHVOIDS"),
        (BodyPolicy::AdvancedBrep, "IFCFACETEDBREPWITHVOIDS"),
    ] {
        let mut source = graph(&serde_json::json!({"identifier":"Body","type":policy.representation_type(),"items":[name]}));
        evaluate(policy, &source, &request(&[("1", Some(true))]), |report| assert_eq!(report.status, "pass"));
        Arc::get_mut(&mut source).unwrap().entities[1].complex.push(IfcComplexType { name: "IFCUNKNOWN".into(), args: vec![] });
        evaluate(policy, &source, &request(&[("1", Some(true))]), |report| {
            assert_eq!(report.status, "incomplete");
            assert_eq!(report.targets[0].diagnostics[0].code, "mvd.items.unknown");
        });
    }
}
