use super::*;
use crate::standards::v1::subsets::model::io::import::deserializers::artifacts::bcf::v2_1::any::model_from_bcf;
use semio_s_artifact_stdio_bcf::schema::snapshot::{BcfComment as BcfCommentT, BcfComponents as BcfComponentsT, BcfTopic as BcfTopicT, BcfViewpoint as BcfViewpointT, BcfVisibility as BcfVisibilityT};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> BcfSnapshot {
    BcfSnapshot {
        schema: semio_s_artifact_stdio_bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(),
        version: "2.1".into(),
        topics: vec![BcfTopicT {
            guid: "topic-1".into(),
            title: "Clash between wall and duct".into(),
            description: "Duct penetrates load-bearing wall".into(),
            status: "Open".into(),
            priority: "High".into(),
            labels: vec!["clash".into(), "mep".into()],
            creation_date: "2026-08-10T12:00:00Z".into(),
            creation_author: "ueli@iek.uni-hannover.de".into(),
            comments: vec![BcfCommentT { guid: "comment-1".into(), date: "2026-08-10T12:05:00Z".into(), author: "ueli@iek.uni-hannover.de".into(), text: "Please reroute the duct.".into(), viewpoint_ref: Some("vp-1".into()) }],
            viewpoints: vec![BcfViewpointT { guid: "vp-1".into(), camera: None, components: Some(BcfComponentsT { selection: vec!["wall-guid".into(), "duct-guid".into()], visibility: BcfVisibilityT::default(), coloring: vec![] }), snapshot: None }],
        }],
        parts: vec![],
    }
}

/// 🧪️ Required proof: bcf -> model -> bcf -> model round trip preserves everything `model`
/// can represent (topic metadata, comments, referenced-guid relations, stub elements).
#[semio_framework_async_macros::async_test]
async fn bcf_to_model_to_bcf_to_model_round_trips() {
    let s1 = model_from_bcf(&fixture());
    let bcf_x = bcf_from_model(&s1);
    let s2 = model_from_bcf(&bcf_x);
    assert_eq!(s1, s2, "model-level round trip through the reconstructed BCF must be exact");
}

#[semio_framework_async_macros::async_test]
async fn non_topic_elements_and_spatial_are_dropped_not_forced() {
    let mut s1 = model_from_bcf(&fixture());
    // hand-add content BCF cannot represent
    s1.spatial.push(crate::standards::v1::subsets::model::schema::snapshot::SpatialNode {
        id: "site-1".into(),
        kind: crate::standards::v1::subsets::model::schema::snapshot::SpatialKind::Site,
        name: "Unrepresentable Site".into(),
        parent_id: None,
        placement: crate::standards::v1::subsets::base::schema::geometry::SemioTransform::identity(),
    });
    let bcf_x = bcf_from_model(&s1);
    assert_eq!(bcf_x.topics.len(), 1, "only the BcfTopic-classed element becomes a topic");
    assert_eq!(bcf_x.version, "2.1");
    assert!(bcf_x.parts.is_empty());
}
