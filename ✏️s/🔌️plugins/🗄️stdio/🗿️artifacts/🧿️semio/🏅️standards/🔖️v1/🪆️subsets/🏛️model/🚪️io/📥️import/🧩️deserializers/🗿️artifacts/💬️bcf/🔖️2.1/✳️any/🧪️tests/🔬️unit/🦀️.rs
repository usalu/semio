use super::*;
use semio_s_artifact_stdio_bcf::schema::snapshot::{BcfComment, BcfComponents as BcfComponentsT, BcfViewpoint, BcfVisibility};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> BcfSnapshot {
    BcfSnapshot {
        schema: semio_s_artifact_stdio_bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(),
        version: "2.1".into(),
        topics: vec![BcfTopic {
            guid: "topic-1".into(),
            title: "Clash between wall and duct".into(),
            description: "Duct penetrates load-bearing wall".into(),
            status: "Open".into(),
            priority: "High".into(),
            labels: vec!["clash".into(), "mep".into()],
            creation_date: "2026-08-10T12:00:00Z".into(),
            creation_author: "ueli@iek.uni-hannover.de".into(),
            comments: vec![BcfComment { guid: "comment-1".into(), date: "2026-08-10T12:05:00Z".into(), author: "ueli@iek.uni-hannover.de".into(), text: "Please reroute the duct.".into(), viewpoint_ref: Some("vp-1".into()) }],
            viewpoints: vec![BcfViewpoint { guid: "vp-1".into(), camera: None, components: Some(BcfComponentsT { selection: vec!["wall-guid".into(), "duct-guid".into()], visibility: BcfVisibility::default(), coloring: vec![] }), snapshot: None }],
        }],
        parts: vec![],
    }
}

#[semio_framework_async_macros::async_test]
async fn topic_becomes_element_with_two_psets_and_reference_relations() {
    let model = model_from_bcf(&fixture());
    assert!(model.spatial.is_empty());
    let topic_el = model.elements.iter().find(|e| e.id == "topic-1").expect("topic element");
    assert_eq!(topic_el.class, ElementClass::Other { name: "BcfTopic".into() });
    let topic_pset = topic_el.psets.iter().find(|p| p.name == "Pset_BcfTopic").expect("topic pset");
    assert!(topic_pset.properties.contains(&Property { key: "title".into(), value: PsetValue::Text { value: "Clash between wall and duct".into() } }));
    assert!(topic_pset.properties.contains(&Property { key: "labels".into(), value: PsetValue::Text { value: "clash|mep".into() } }));
    let comments_pset = topic_el.psets.iter().find(|p| p.name == "Pset_BcfComments").expect("comments pset");
    assert!(comments_pset.properties.contains(&Property { key: "comment_0_text".into(), value: PsetValue::Text { value: "Please reroute the duct.".into() } }));
    assert!(comments_pset.properties.contains(&Property { key: "comment_0_viewpointRef".into(), value: PsetValue::Text { value: "vp-1".into() } }));

    assert!(model.elements.iter().any(|e| e.id == "wall-guid" && e.class == ElementClass::Other { name: "BcfReferencedComponent".into() }));
    assert!(model.elements.iter().any(|e| e.id == "duct-guid"));
    assert!(model.relations.iter().any(|r| r.from == "topic-1" && r.to == "wall-guid" && r.kind == RelationKind::Other { label: "BcfReferences".into() }));
    assert!(model.relations.iter().any(|r| r.from == "topic-1" && r.to == "duct-guid"));
}
