//! 🧪️ Native Clipping retains only the released item-family clause and shared Body authority.
use super::*;
use super::native_tests::evaluate;
use super::tessellation_tests::{graph, request};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcComplexType, IfcEntity};
#[test]
fn native_body_clipping_matches_all_twenty_selected_neutral_rows() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../✂️clipping/cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let source = graph(row);
        evaluate(BodyPolicy::Clipping, &source, &request(&[("1", row["applicable"].as_bool())]), |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap());
            assert_eq!(report.targets[0].status, report.status);
            assert_eq!(report.concept, "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Clipping_Geometry");
            assert_eq!(report.source.sha256, "35ca73abc9904a2e97a0ad5ff0896a1df91285259537ebab269df2cfe7c01210");
        });
    }
}
#[test]
fn native_body_clipping_separates_csg_operator_schema_rules_and_late_unknown_evidence() {
    let mut source = graph(&serde_json::json!({"identifier":"Body","type":"Clipping","items":["IFCCSGSOLID"]}));
    evaluate(BodyPolicy::Clipping, &source, &request(&[("1", Some(true))]), |report| {
        assert_eq!(report.status, "fail");
        let diagnostic = &report.targets[0].diagnostics[0];
        assert_eq!(diagnostic.code, "mvd.items.type");
        assert_eq!(diagnostic.pointer.as_deref(), Some("/entities/0/args/3/value/0"));
        assert_eq!(diagnostic.label.en, "A representation item is not a boolean clipping result.");
        assert_eq!(diagnostic.label.de, "Ein Repräsentationselement ist kein boolesches Beschneidungsergebnis.");
    });
    let value = Arc::get_mut(&mut source).unwrap();
    value.entities[1].name = "IFCBOOLEANCLIPPINGRESULT".into();
    value.entities[1].args = vec![IfcValue::Enum("UNION".into()), IfcValue::Reference(3), IfcValue::Reference(4)];
    value.entities.extend([IfcEntity { id: 3, name: "IFCEXTRUDEDAREASOLID".into(), args: vec![], complex: vec![] }, IfcEntity { id: 4, name: "IFCHALFSPACESOLID".into(), args: vec![], complex: vec![] }]);
    evaluate(BodyPolicy::Clipping, &source, &request(&[("1", Some(true))]), |report| assert_eq!(report.status, "pass"));
    Arc::get_mut(&mut source).unwrap().entities[1].complex.push(IfcComplexType { name: "IFCUNKNOWN".into(), args: vec![] });
    evaluate(BodyPolicy::Clipping, &source, &request(&[("1", Some(true))]), |report| {
        assert_eq!(report.status, "incomplete");
        assert_eq!(report.targets[0].diagnostics[0].code, "mvd.items.unknown");
    });
}
