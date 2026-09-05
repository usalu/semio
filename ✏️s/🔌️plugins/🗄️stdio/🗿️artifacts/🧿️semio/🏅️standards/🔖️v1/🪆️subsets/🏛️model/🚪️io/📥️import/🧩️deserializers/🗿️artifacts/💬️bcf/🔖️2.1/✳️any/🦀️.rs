//! 📥️ `SemioModelFromBcf` — BCF-XML 2.1 issue-tracking bridge into `semio/v1/model`.
//!
//! BCF is NOT a spatial/geometric format (unlike ifc) — it is a collaboration/issue-tracking
//! container whose `Topic`s reference geometric elements only BY GUID (in `Components`
//! selection/visibility-exception/coloring lists), never by defining them. Per this pair's brief
//! ("topics/comments/viewpoints map onto model relations/psets where sensible... document what
//! does NOT map"), the honest mapping built here is:
//!
//! - Each **topic** becomes one `SemioModelElement` (`class: Other{"BcfTopic"}`, `id` = the
//!   topic's own guid) carrying two synthesized property sets: `Pset_BcfTopic` (title/status/
//!   priority/description/creationDate/creationAuthor/labels, all as `Text`) and
//!   `Pset_BcfComments` (a `count` `Number` plus `comment_<i>_{guid,date,author,text}` `Text`
//!   properties, `comment_<i>_viewpointRef` only when the source comment had one) — this is the
//!   real, natural home for BCF's textual metadata, since `model`'s only free-form container is a
//!   named property bag.
//! - Every guid referenced by ANY of a topic's viewpoints' `Components` (selection ∪
//!   visibility.exceptions ∪ coloring[].components, deduped, first-seen order, per-viewpoint
//!   distinction flattened — see below) gets: (a) a STUB `SemioModelElement`
//!   (`class: Other{"BcfReferencedComponent"}`, no geometry/placement/psets — BCF never defines
//!   what a referenced component actually IS, only that it exists) if one doesn't already exist
//!   for that guid, and (b) a `ModelRelation{kind: Other{"BcfReferences"}, from: topic_guid, to:
//!   component_guid}`.
//!
//! **Explicitly NOT mapped** (real, honest gaps — never forced into a field that doesn't fit):
//! - `BcfSnapshot.version` and `.parts` (raw-retained files, e.g. `project.bcfp`) — no `model`
//!   field represents container-level metadata or unmodeled sidecar files.
//! - `BcfCamera` (perspective/orthogonal viewpoint geometry) and `BcfViewpoint.snapshot` (the PNG
//!   preview image) — `model` has no camera/image concept.
//! - Per-viewpoint distinction: if a topic has MULTIPLE viewpoints, their `Components` lists are
//!   flattened into ONE deduped relation set per topic — which specific viewpoint referenced which
//!   guid is lost. `BcfComponents.visibility.default_visibility` and the per-guid `coloring` HEX
//!   COLOR itself are also lost (only the referenced-guid identity survives, via `Other` relations
//!   uniformly — visibility/coloring/selection are not distinguished from one another either).
//!
//! `model.spatial` is always empty from this bridge (BCF has no spatial-structure concept).

use crate::artifacts::bcf::schema::snapshot::{BcfComponents, BcfTopic};
use crate::artifacts::bcf::BcfSnapshot;
use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, STDIO_SEMIOMODEL_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioModelFromBcf;

impl ArtifactDeserializer for SemioModelFromBcf {
    type From = BcfSnapshot;
    type Into = SemioModelSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(model_from_bcf(from))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Convert
/// 🕸️ Every guid referenced by `components`, first-seen order, deduped across
/// selection/visibility-exceptions/coloring (per-list distinction flattened — see module doc).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn referenced_guids(components: &BcfComponents, out: &mut Vec<String>) {
    for guid in &components.selection {
        if !out.contains(guid) {
            out.push(guid.clone());
        }
    }
    for guid in &components.visibility.exceptions {
        if !out.contains(guid) {
            out.push(guid.clone());
        }
    }
    for coloring in &components.coloring {
        for guid in &coloring.components {
            if !out.contains(guid) {
                out.push(guid.clone());
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn topic_pset(topic: &BcfTopic) -> PropertySet {
    let mut properties = vec![Property { key: "title".into(), value: PsetValue::Text { value: topic.title.clone() } }, Property { key: "status".into(), value: PsetValue::Text { value: topic.status.clone() } }];
    if !topic.priority.is_empty() {
        properties.push(Property { key: "priority".into(), value: PsetValue::Text { value: topic.priority.clone() } });
    }
    if !topic.description.is_empty() {
        properties.push(Property { key: "description".into(), value: PsetValue::Text { value: topic.description.clone() } });
    }
    if !topic.creation_date.is_empty() {
        properties.push(Property { key: "creationDate".into(), value: PsetValue::Text { value: topic.creation_date.clone() } });
    }
    if !topic.creation_author.is_empty() {
        properties.push(Property { key: "creationAuthor".into(), value: PsetValue::Text { value: topic.creation_author.clone() } });
    }
    if !topic.labels.is_empty() {
        properties.push(Property { key: "labels".into(), value: PsetValue::Text { value: topic.labels.join("|") } });
    }
    PropertySet { name: "Pset_BcfTopic".into(), properties }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn comments_pset(topic: &BcfTopic) -> Option<PropertySet> {
    if topic.comments.is_empty() {
        return None;
    }
    let mut properties = vec![Property { key: "count".into(), value: PsetValue::Number { value: topic.comments.len() as f64 } }];
    for (i, comment) in topic.comments.iter().enumerate() {
        properties.push(Property { key: format!("comment_{i}_guid"), value: PsetValue::Text { value: comment.guid.clone() } });
        properties.push(Property { key: format!("comment_{i}_date"), value: PsetValue::Text { value: comment.date.clone() } });
        properties.push(Property { key: format!("comment_{i}_author"), value: PsetValue::Text { value: comment.author.clone() } });
        properties.push(Property { key: format!("comment_{i}_text"), value: PsetValue::Text { value: comment.text.clone() } });
        if let Some(vref) = &comment.viewpoint_ref {
            properties.push(Property { key: format!("comment_{i}_viewpointRef"), value: PsetValue::Text { value: vref.clone() } });
        }
    }
    Some(PropertySet { name: "Pset_BcfComments".into(), properties })
}
//#endregion 🔖️Convert

//#region 🔖️Entry
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn model_from_bcf(from: &BcfSnapshot) -> SemioModelSnapshot {
    let mut elements: Vec<SemioModelElement> = Vec::new();
    let mut relations: Vec<ModelRelation> = Vec::new();
    let mut known_component_ids: Vec<String> = Vec::new();

    for topic in &from.topics {
        let mut psets = vec![topic_pset(topic)];
        if let Some(comments) = comments_pset(topic) {
            psets.push(comments);
        }
        elements.push(SemioModelElement { id: topic.guid.clone(), class: ElementClass::Other { name: "BcfTopic".into() }, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets });

        let mut refs: Vec<String> = Vec::new();
        for vp in &topic.viewpoints {
            if let Some(components) = &vp.components {
                referenced_guids(components, &mut refs);
            }
        }
        for guid in &refs {
            if !known_component_ids.contains(guid) {
                known_component_ids.push(guid.clone());
                elements.push(SemioModelElement { id: guid.clone(), class: ElementClass::Other { name: "BcfReferencedComponent".into() }, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
            }
            relations.push(ModelRelation { id: format!("rel-bcfreferences-{}-{}", topic.guid, guid), kind: RelationKind::Other { label: "BcfReferences".into() }, from: topic.guid.clone(), to: guid.clone() });
        }
    }

    SemioModelSnapshot { schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(), spatial: Vec::new(), elements, relations }
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::bcf::schema::snapshot::{BcfComment, BcfComponents as BcfComponentsT, BcfViewpoint, BcfVisibility};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture() -> BcfSnapshot {
        BcfSnapshot {
            schema: crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(),
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
                viewpoints: vec![BcfViewpoint {
                    guid: "vp-1".into(),
                    camera: None,
                    components: Some(BcfComponentsT { selection: vec!["wall-guid".into(), "duct-guid".into()], visibility: BcfVisibility::default(), coloring: vec![] }),
                    snapshot: None,
                }],
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
}
//#endregion 🧪️Tests
