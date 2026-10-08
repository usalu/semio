//! 🧪️ Shared editable document fixtures and incremental cancellation.
#[test]
fn svg_documents_preserve_owned_hierarchy_and_paint() {
    let fixtures:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures.as_array().unwrap(){
        let load=||->Result<crate::DrawingSnapshot,String>{let mut job=super::SvgImportJob::new(case["source"].as_str().unwrap(),"import")?;while !job.step(1)?.done{}job.take()};
        if case["after"].is_null(){assert!(load().is_err(),"{}",case["name"]);continue;}
        assert_eq!(load().unwrap(),serde_json::from_value::<crate::DrawingSnapshot>(case["after"].clone()).unwrap(),"{}",case["name"]);
        println!("[DEBUG] decoded SVG native document {} preserves neutral hierarchy, geometry and paint",case["name"]);
    }
}
#[test]
fn svg_import_cancels_without_a_partial_document(){
    let source=format!("<svg>{}</svg>","<path d=\"M0 0L1 1\"/>".repeat(100));
    let mut job=super::SvgImportJob::new(&source,"cancelled").unwrap();
    let progress=job.step(2).unwrap();assert!(!progress.done);assert_eq!(progress.completed,2);assert!(job.take().is_err());
    job.cancel();assert!(job.step(1).is_err());assert!(job.take().is_err());
}

#[semio_framework_async_macros::async_test]
async fn registered_svg_deserializer_accepts_text_and_utf8_bytes() {
    use semio_framework::io::io_mechanism::Deserializer;
    use semio_framework::io_schema::IoPayload;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::svg::v1_1::any::SvgIntoDraw;
    let source="<svg><g><path d=\"M0 0L2 3\"/></g></svg>";
    let text=SvgIntoDraw::deserialize(&IoPayload::Text(source.into())).await.unwrap().value;
    let bytes=SvgIntoDraw::deserialize(&IoPayload::Binary(source.as_bytes().to_vec())).await.unwrap().value;
    assert_eq!(text,bytes);assert_eq!(text.layers.len(),1);
    assert!(SvgIntoDraw::deserialize(&IoPayload::Binary(vec![255])).await.is_err());
}

#[test]
fn svg_route_runs_a_multi_batch_document_through_the_actual_io_entry() {
    use semio_framework::io_schema::IoPayload;
    let declaration=crate::standards::v1::subsets::any::io::io();
    let entry=declaration.entries.iter().find(|entry|entry.from.artifact_kind=="s.stdio.svg"&&entry.into.artifact_kind=="s.draw.drawing").expect("registered SVG import");
    let source=format!("<svg>{}</svg>","<path d=\"M0 0L1 1\"/>".repeat(100));
    assert!((entry.run)(&IoPayload::Text(source)).is_ok());
}

#[test]
fn svg_import_normalizes_css_blends_and_accepts_exported_isolation() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../🧬️schema/🎬️scene/🧩️compositing/🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let nodes:Vec<crate::schema::DrawingSceneNode>=semio_framework_pack_json::from_json_str(&case["nodes"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let source=crate::standards::v1::subsets::any::io::export::serializers::artifacts::svg::v1_1::any::drawing_scene_to_svg(&nodes,[0.0,0.0,24.0,16.0]).unwrap();
        let mut job=super::SvgImportJob::new(&source,"roundtrip").unwrap();while !job.step(3).unwrap().done {}
        let document=job.take().unwrap();
        let output=crate::schema::flatten_drawing_document_to_scene_nodes(&document);
        assert_eq!(output.len(),nodes.len(),"{}",case["name"]);
        let authored=nodes.iter().flat_map(|node|node.groups.iter().map(|group|group.blend_mode.as_str()).chain(std::iter::once(node.blend_mode.as_str()))).filter(|mode|*mode!="normal").collect::<std::collections::BTreeSet<_>>();
        let imported=output.iter().flat_map(|node|node.groups.iter().map(|group|group.blend_mode.as_str()).chain(std::iter::once(node.blend_mode.as_str()))).filter(|mode|*mode!="normal").collect::<std::collections::BTreeSet<_>>();
        assert_eq!(authored,imported,"{}",case["name"]);
    }
}
