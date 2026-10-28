//! 🧪️ Native SurfaceModel preserves the released wider TessellatedItem family and original Body authority.
use super::*;
use super::native_tests::evaluate;
use super::tessellation_tests::{graph, request};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
#[test]
fn native_body_surface_model_matches_all_twenty_five_selected_neutral_rows() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🫧️surface-model/cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let source = graph(row);
        evaluate(BodyPolicy::SurfaceModel, &source, &request(&[("1", row["applicable"].as_bool())]), |report| {
            assert_eq!(report.status, row["expected"].as_str().unwrap());
            assert_eq!(report.targets[0].status, report.status);
            assert_eq!(report.concept, "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SurfaceModel_Geometry");
            assert_eq!(report.source.sha256, "f4ac42de0851bd18e4a3ca56104a6ec69dd547b5c3a3fff1ddc4a5912239cc95");
            assert_eq!(report.source.commit, "610ee3f95db9d0380b1eb8de4dc6e17833531518");
        });
    }
}
#[test]
fn native_body_surface_model_keeps_indexed_faces_and_every_late_unknown_constituent() {
    for name in ["IFCINDEXEDPOLYGONALFACE", "IFCINDEXEDPOLYGONALFACEWITHVOIDS", "IFCTRIANGULATEDIRREGULARNETWORK", "IFCSHELLBASEDSURFACEMODEL", "IFCFACEBASEDSURFACEMODEL"] {
        let mut source = graph(&serde_json::json!({"identifier":"Body","type":"SurfaceModel","items":[name]}));
        evaluate(BodyPolicy::SurfaceModel, &source, &request(&[("1", Some(true))]), |report| assert_eq!(report.status, "pass"));
        Arc::get_mut(&mut source).unwrap().entities[1].complex.push(IfcComplexType { name: "IFCUNKNOWN".into(), args: vec![] });
        evaluate(BodyPolicy::SurfaceModel, &source, &request(&[("1", Some(true))]), |report| {
            assert_eq!(report.status, "incomplete");
            let diagnostic = &report.targets[0].diagnostics[0];
            assert_eq!(diagnostic.code, "mvd.items.unknown");
            assert_eq!(diagnostic.pointer.as_deref(), Some("/entities/0/args/3/value/0"));
        });
    }
    let source = graph(&serde_json::json!({"identifier":"Body","type":"SurfaceModel","items":["IFCEXTRUDEDAREASOLID"]}));
    evaluate(BodyPolicy::SurfaceModel, &source, &request(&[("1", Some(true))]), |report| {
        assert_eq!(report.status, "fail");
        let label = &report.targets[0].diagnostics[0].label;
        assert_eq!(label.en, "A representation item is neither a tessellated item nor a shell-based or face-based surface model.");
        assert_eq!(label.de, "Ein Repräsentationselement ist weder ein tesselliertes Element noch ein schalen- oder flächenbasiertes Oberflächenmodell.");
    });
}
