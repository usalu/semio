use crate::apply_mutation;
use super::*;
use crate::schema::diff::BcfDiff;
use crate::schema::mutations::{
    insert_comment, insert_topic, insert_viewpoint, remove_comment, remove_topic, remove_viewpoint, set_comment, set_topic_markup, set_version, set_viewpoint_camera, set_viewpoint_components, set_viewpoint_snapshot,
    BcfMutation,
};
use crate::standards::v2_1::subsets::any::schema::snapshot::{demo_bcf_snapshot, empty_bcf_snapshot};
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText, Mutation, MutationDiff, OpBinary, OpText};

//#region Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn perspective_camera() -> BcfCamera {
    BcfCamera::Perspective { view_point: BcfPoint3 { x: 1.0, y: 2.0, z: 3.0 }, direction: BcfPoint3 { x: 0.0, y: 0.0, z: -1.0 }, up_vector: BcfPoint3 { x: 0.0, y: 1.0, z: 0.0 }, field_of_view: 60.0 }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn orthogonal_camera() -> BcfCamera {
    BcfCamera::Orthogonal { view_point: BcfPoint3 { x: 4.0, y: 5.0, z: 6.0 }, direction: BcfPoint3 { x: 1.0, y: 0.0, z: 0.0 }, up_vector: BcfPoint3 { x: 0.0, y: 0.0, z: 1.0 }, view_to_world_scale: 2.5 }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_components() -> BcfComponents {
    BcfComponents {
        selection: vec!["2O2Fr$t4X7Zf8NOew3FLOH".into()],
        visibility: BcfVisibility { default_visibility: false, exceptions: vec!["1yQBoo7d5EEBLiyMxGgTLc".into()] },
        coloring: vec![BcfColoring { color: "FFFF0000".into(), components: vec!["0BTBFw6f90Nfh9rP1dl_3n".into()] }],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_viewpoint(guid: &str) -> BcfViewpoint {
    BcfViewpoint { guid: guid.into(), camera: Some(perspective_camera()), components: Some(sample_components()), snapshot: Some(vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_comment(guid: &str, viewpoint_ref: Option<&str>) -> BcfComment {
    BcfComment { guid: guid.into(), date: "2024-01-01T00:00:00+00:00".into(), author: "ueli@example.com".into(), text: "Please review this clash.".into(), viewpoint_ref: viewpoint_ref.map(|s| s.to_string()) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_topic(guid: &str) -> BcfTopic {
    BcfTopic {
        guid: guid.into(),
        title: "Clash on Level 2".into(),
        description: "MEP duct clashes with structural beam.".into(),
        status: "Open".into(),
        priority: "High".into(),
        labels: vec!["Clash".into(), "MEP".into()],
        creation_date: "2024-01-01T00:00:00+00:00".into(),
        creation_author: "ueli@example.com".into(),
        comments: vec![sample_comment("c1", Some("vp1"))],
        viewpoints: vec![sample_viewpoint("vp1")],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> BcfSnapshot {
    BcfSnapshot { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), version: "2.1".into(), topics: vec![sample_topic("t1")], parts: vec![BcfRawPart { name: "project.bcfp".into(), data: b"<ProjectExtension/>".to_vec() }] }
}
//#endregion Fixtures

/// 🧪️ Full round trip through the real zip+xml codecs: version, topic markup (incl. the
/// previously-mismodeled `Priority`/`Description`/`Labels`/`CreationDate`/`CreationAuthor`
/// child elements), comments (incl. `viewpoint_ref`), and a viewpoint's camera/components/
/// snapshot all survive.
#[semio_framework_async_macros::async_test]
async fn decode_of_encode_recovers_full_typed_model() {
    let snap = sample_snapshot();
    let bytes = encode_bcf(&snap).expect("encode");
    let decoded = decode_bcf(&bytes).expect("decode");

    assert_eq!(decoded.version, "2.1");
    assert_eq!(decoded.topics.len(), 1);
    let topic = &decoded.topics[0];
    assert_eq!(topic.guid, "t1");
    assert_eq!(topic.title, "Clash on Level 2");
    assert_eq!(topic.description, "MEP duct clashes with structural beam.");
    assert_eq!(topic.status, "Open");
    assert_eq!(topic.priority, "High");
    assert_eq!(topic.labels, vec!["Clash".to_string(), "MEP".to_string()]);
    assert_eq!(topic.creation_date, "2024-01-01T00:00:00+00:00");
    assert_eq!(topic.creation_author, "ueli@example.com");

    assert_eq!(topic.comments.len(), 1);
    assert_eq!(topic.comments[0].guid, "c1");
    assert_eq!(topic.comments[0].viewpoint_ref.as_deref(), Some("vp1"));

    assert_eq!(topic.viewpoints.len(), 1);
    let vp = &topic.viewpoints[0];
    assert_eq!(vp.guid, "vp1");
    assert_eq!(vp.camera, Some(perspective_camera()));
    assert_eq!(vp.components, Some(sample_components()));
    assert_eq!(vp.snapshot, Some(vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]));

    assert!(decoded.parts.iter().any(|p| p.name == "project.bcfp"));
}

/// 🧪️ Orthogonal camera round-trips too (the `xs:choice` sibling of `PerspectiveCamera`).
#[semio_framework_async_macros::async_test]
async fn orthogonal_camera_round_trips() {
    let mut snap = sample_snapshot();
    snap.topics[0].viewpoints[0].camera = Some(orthogonal_camera());
    let decoded = decode_bcf(&encode_bcf(&snap).unwrap()).unwrap();
    assert_eq!(decoded.topics[0].viewpoints[0].camera, Some(orthogonal_camera()));
}

/// 🧪️ A topic folder with no `markup.bcf` (only stray files) is retained verbatim as raw
/// parts, never fabricated into a bogus topic.
#[semio_framework_async_macros::async_test]
async fn folder_without_markup_becomes_raw_parts() {
    let zip_snap = semio_s_artifact_stdio_zip::ZipSnapshot {
        schema: semio_s_artifact_stdio_zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: vec![ZipEntry { name: "bcf.version".into(), data: bcf_version_bytes("2.1"), ..Default::default() }, ZipEntry { name: "stray/notes.txt".into(), data: b"not a topic".to_vec(), ..Default::default() }],
        comment: String::new(),
        ..Default::default()
    };
    let bytes = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_zip(&zip_snap).unwrap();
    let decoded = decode_bcf(&bytes).unwrap();
    assert!(decoded.topics.is_empty());
    assert!(decoded.parts.iter().any(|p| p.name == "stray/notes.txt"));
}

/// 📷️ Every committed jszip `⬅️before.bcf` of this subset's corpus carries `viewpoint-01` with a perspective camera and a
/// two-guid selection; the decoder must hand both over and a re-encode must keep them. The generator once wrote
/// `<Visibility DefaultVisibility/>` (a bare HTML-style attribute), the strict XML reader refused that `.bcfv`, and the
/// decoder dropped camera and components without a word.
#[semio_framework_async_macros::async_test]
async fn committed_viewpoint_fixture_keeps_camera_and_components_through_a_round_trip() {
    const BEFORE: &[u8] = include_bytes!("../../../🧫️fixtures/✏️set-comment-applied/⬅️before.bcf");
    let decoded = decode_bcf(BEFORE).expect("decode the committed fixture");
    let topic = decoded.topics.iter().find(|topic| topic.guid == "topic-clash-01").expect("topic-clash-01");
    let viewpoint = topic.viewpoints.iter().find(|viewpoint| viewpoint.guid == "viewpoint-01").expect("viewpoint-01");
    assert_eq!(viewpoint.camera, Some(BcfCamera::Perspective { view_point: BcfPoint3 { x: 10.0, y: 5.0, z: 2.0 }, direction: BcfPoint3 { x: 0.0, y: 0.0, z: -1.0 }, up_vector: BcfPoint3 { x: 0.0, y: 1.0, z: 0.0 }, field_of_view: 60.0 }));
    assert_eq!(viewpoint.components, Some(BcfComponents { selection: vec!["ifc-beam-1".into(), "ifc-duct-1".into()], visibility: BcfVisibility { default_visibility: true, exceptions: Vec::new() }, coloring: Vec::new() }));
    assert_eq!(decode_bcf(&encode_bcf(&decoded).expect("encode")).expect("decode the re-encoded archive"), decoded);
}

/// 🚫️ A `.bcfv` the markup references but no well-formed `<VisualizationInfo>` fills is REFUSED with the part's name, never
/// decoded as a viewpoint without camera and components.
#[semio_framework_async_macros::async_test]
async fn malformed_visualization_info_is_refused() {
    let markup = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><Markup><Topic Guid=\"t\" TopicStatus=\"Open\"><Title>T</Title></Topic><Viewpoints Guid=\"v\"><Viewpoint>v.bcfv</Viewpoint></Viewpoints></Markup>".to_vec();
    let visinfo = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><VisualizationInfo Guid=\"v\"><Components><Visibility DefaultVisibility/></Components></VisualizationInfo>".to_vec();
    let zip_snap = semio_s_artifact_stdio_zip::ZipSnapshot {
        schema: semio_s_artifact_stdio_zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: vec![ZipEntry { name: "bcf.version".into(), data: bcf_version_bytes("2.1"), ..Default::default() }, ZipEntry { name: "t/markup.bcf".into(), data: markup, ..Default::default() }, ZipEntry { name: "t/v.bcfv".into(), data: visinfo, ..Default::default() }],
        comment: String::new(),
        ..Default::default()
    };
    let bytes = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_zip(&zip_snap).unwrap();
    assert_eq!(decode_bcf(&bytes), Err("t/v.bcfv is not a well-formed BCF 2.1 <VisualizationInfo> document".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_bcf_snapshot();
    assert_eq!(snapshot.schema, STDIO_BCF_DOCUMENT_SCHEMA);
    assert!(snapshot.topics.is_empty());
    assert!(snapshot.parts.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();

    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <BcfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed, snap);

    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <BcfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region 🧪️Law1_MutationDiffLaw
/// ⚖️ Law 1 — `mutation_diff_law`: for every mutation variant, applying via
/// `apply_mutation` matches `m.diff(base).diff().apply(base)`, and the returned diff equals
/// `m.diff(base)`.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();
    let mutations = vec![
        BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2"), index: None }),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
            guid: "t1".into(),
            title: Some("Renamed".into()),
            description: None,
            status: Some("Closed".into()),
            priority: None,
            labels: Some(vec!["Renamed".into()]),
            creation_date: None,
            creation_author: None,
        }),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: sample_comment("c2", None), index: None }),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t1".into(), guid: "c1".into() }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: None, author: None, text: Some("Updated".into()), viewpoint_ref: Some(None) }),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t1".into(), viewpoint: sample_viewpoint("vp2"), index: None }),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(orthogonal_camera()) }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: None }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: None }),
    ];
    for m in mutations {
        let mut snap = base.clone();
        let returned = apply_mutation(&mut snap, &m);
        let expected_diff = m.diff(&base);
        assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
        assert_eq!(snap, protocol::apply_diff(expected_diff.diff(), &base).expect("diff must apply to base"), "apply mismatch for {m:?}");
    }
}
//#endregion

//#region 🧪️Law2_InverseLaw
/// ⚖️ Law 2 — `inverse_law`: every mutation round-trips (mutation-level) and every diff
/// round-trips (diff-level `d.diff().inverse(base).apply(&d.diff().apply(base)) == base`).
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();
    let mutations = vec![
        BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2"), index: None }),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t1".into(), title: Some("Renamed".into()), description: Some("New desc".into()), status: None, priority: None, labels: None, creation_date: None, creation_author: None }),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: sample_comment("c2", None), index: None }),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t1".into(), guid: "c1".into() }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: Some("2025-01-01T00:00:00+00:00".into()), author: None, text: None, viewpoint_ref: None }),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t1".into(), viewpoint: sample_viewpoint("vp2"), index: None }),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: None }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: Some(sample_components()) }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: Some(vec![1, 2, 3]) }),
    ];
    for m in mutations {
        let mut snap = base.clone();
        apply_mutation(&mut snap, &m);
        for inv in m.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
            let mut undone = snap.clone();
            apply_mutation(&mut undone, &inv);
            assert_eq!(undone, base, "mutation-level inverse mismatch for {m:?}");
        }

        let d = m.diff(&base);
        let after = protocol::apply_diff(d.diff(), &base).expect("diff must apply to base");
        let d_inv = d.diff().inverse(&base);
        assert_eq!(protocol::apply_diff(&d_inv, &after).expect("inverse diff must apply to after"), base, "diff-level inverse mismatch for {m:?}");
    }
}
//#endregion

//#region 🧪️Law3_AbsorbLaw
/// ⚖️ Law 3 — `absorb_law`: sequential-coalesce over a curated op list incl. the canonical
/// cases (Insert+Remove-before, Insert+Insert-same-key-both-survive [name-keyed: no
/// same-key clash needed, tested via disjoint-then-merge], Add+SetField patches into added,
/// Modify+Remove annihilates) plus associativity.
#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    let base = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();

    // Insert+Remove-before: insert t2, then remove t1 -- both survive independently (name-keyed,
    // no interaction), net effect must match sequential application.
    let d1 = BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2"), index: None }).diff(&base);
    let mid = protocol::apply_diff(d1.diff(), &base).expect("d1 must apply to base");
    let d2 = BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }).diff(&mid);
    assert_absorb_matches_sequential(&base, d1.clone(), d2.clone());

    // Add+SetField: insert a comment, then immediately edit that SAME comment -- the edit must
    // patch into the carried `added` payload, not become a dangling `modified` entry.
    let comment = sample_comment("c9", None);
    let d1 = BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: comment.clone(), index: None }).diff(&base);
    let mid = protocol::apply_diff(d1.diff(), &base).expect("d1 must apply to base");
    let d2 = BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c9".into(), date: None, author: None, text: Some("edited after insert".into()), viewpoint_ref: None }).diff(&mid);
    let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
    let topics_diff = absorbed.topics.as_ref().expect("topics diff");
    let t1_diff = &topics_diff.modified.iter().find(|m| m.index == 0).expect("t1 modified").diff;
    let comments_diff = t1_diff.comments.as_ref().expect("comments diff");
    assert!(comments_diff.modified.is_empty(), "edit-after-insert must patch into added, not appear as modified");
    let added_comment = comments_diff.added.iter().find(|c| c.item.guid == "c9").expect("c9 still in added");
    assert_eq!(added_comment.item.text, "edited after insert");

    // Modify+Remove: edit a viewpoint's camera, then remove that same viewpoint -- must
    // annihilate to a plain removal, not a dangling modify+remove pair.
    let d1 = BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(orthogonal_camera()) }).diff(&base);
    let mid = protocol::apply_diff(d1.diff(), &base).expect("d1 must apply to base");
    let d2 = BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }).diff(&mid);
    let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
    let topics_diff = absorbed.topics.as_ref().expect("topics diff");
    let t1_diff = &topics_diff.modified.iter().find(|m| m.index == 0).expect("t1 modified").diff;
    let viewpoints_diff = t1_diff.viewpoints.as_ref().expect("viewpoints diff");
    assert_eq!(viewpoints_diff.removed, vec![0usize]);
    assert!(viewpoints_diff.modified.is_empty());

    // Associativity: absorb(absorb(d1,d2),d3) == absorb(d1,absorb(d2,d3)).
    let d1 = BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }).diff(&base);
    let mid1 = protocol::apply_diff(d1.diff(), &base).expect("d1 must apply to base");
    let d2 = BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t3"), index: None }).diff(&mid1);
    let mid2 = protocol::apply_diff(d2.diff(), &mid1).expect("d2 must apply to mid1");
    let d3 = BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t3".into(), title: Some("Renamed t3".into()), description: None, status: None, priority: None, labels: None, creation_date: None, creation_author: None }).diff(&mid2);

    let mut left = d1.diff().clone();
    MutationDiff::absorb(&mut left, d2.diff().clone());
    MutationDiff::absorb(&mut left, d3.diff().clone());

    let mut d2_d3 = d2.diff().clone();
    MutationDiff::absorb(&mut d2_d3, d3.diff().clone());
    let mut right = d1.diff().clone();
    MutationDiff::absorb(&mut right, d2_d3);

    assert_eq!(protocol::apply_diff(&left, &base).expect("left must apply to base"), protocol::apply_diff(&right, &base).expect("right must apply to base"), "absorb must be associative");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &BcfSnapshot, d1: protocol::MutationOutcome<BcfDiff>, d2: protocol::MutationOutcome<BcfDiff>) -> BcfDiff {
    let mid = protocol::apply_diff(d1.diff(), base).expect("d1 must apply to base");
    let sequential = protocol::apply_diff(d2.diff(), &mid).expect("d2 must apply to mid");
    let mut absorbed = d1.diff().clone();
    MutationDiff::absorb(&mut absorbed, d2.diff().clone());
    assert_eq!(protocol::apply_diff(&absorbed, base).expect("absorbed diff must apply to base"), sequential, "absorb(d1,d2).apply(base) must equal sequential application");
    absorbed
}
//#endregion

//#region 🧪️Law5_CodecRetentionLaw
/// ⚖️ Law 5 — `codec_retention_law`: decode(encode(x)) == x (this artifact's documented
/// normal form for viewpoint/snapshot filenames -- see the snapshot module's `BcfViewpoint`
/// doc comment).
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();
    let re_encoded = encode_bcf(&snap).unwrap();
    let re_decoded = decode_bcf(&re_encoded).unwrap();
    assert_eq!(re_decoded, snap);
}
//#endregion

//#region 🧪️Law6_FieldSweep
/// ⚖️ Law 6 — `field_sweep` (the acceptance criterion): `sweep_a`/`sweep_b` differ in EVERY
/// mutable field, incl. per index-keyed collection one removed (forward) / added (backward) tail row and one
/// modified-in-every-field row, and every tri-state field exercising `Some(None)`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> BcfSnapshot {
    BcfSnapshot {
        schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        version: "2.1".into(),
        topics: vec![
            BcfTopic {
                guid: "keep".into(),
                title: "Keep-topic before".into(),
                description: "before desc".into(),
                status: "Open".into(),
                priority: "Low".into(),
                labels: vec!["before".into()],
                creation_date: "2024-01-01T00:00:00+00:00".into(),
                creation_author: "a@example.com".into(),
                comments: vec![
                    BcfComment { guid: "c-keep".into(), date: "2024-01-01T00:00:00+00:00".into(), author: "a@example.com".into(), text: "before text".into(), viewpoint_ref: Some("vp-remove".into()) },
                    BcfComment { guid: "c-remove".into(), date: "2024-01-01T00:00:00+00:00".into(), author: "a@example.com".into(), text: "will be removed".into(), viewpoint_ref: Some("vp-keep".into()) },
                ],
                viewpoints: vec![
                    BcfViewpoint { guid: "vp-keep".into(), camera: Some(perspective_camera()), components: Some(sample_components()), snapshot: Some(vec![2]) },
                    BcfViewpoint { guid: "vp-remove".into(), camera: Some(perspective_camera()), components: Some(sample_components()), snapshot: Some(vec![1]) },
                ],
            },
            BcfTopic {
                guid: "topic-remove".into(),
                title: "Will be removed".into(),
                description: String::new(),
                status: "Open".into(),
                priority: String::new(),
                labels: Vec::new(),
                creation_date: String::new(),
                creation_author: String::new(),
                comments: Vec::new(),
                viewpoints: Vec::new(),
            },
        ],
        parts: vec![BcfRawPart { name: "part-keep.txt".into(), data: b"before".to_vec() }, BcfRawPart { name: "part-remove.txt".into(), data: b"gone".to_vec() }],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> BcfSnapshot {
    BcfSnapshot {
        schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        version: "2.2".into(),
        topics: vec![
            BcfTopic {
                guid: "keep".into(),
                title: "Keep-topic after".into(),
                description: "after desc".into(),
                status: "Closed".into(),
                priority: "High".into(),
                labels: vec!["after".into(), "second".into()],
                creation_date: "2024-02-02T00:00:00+00:00".into(),
                creation_author: "b@example.com".into(),
                comments: vec![
                    BcfComment { guid: "c-keep".into(), date: "2024-02-02T00:00:00+00:00".into(), author: "b@example.com".into(), text: "after text".into(), viewpoint_ref: None },
                ],
                viewpoints: vec![
                    BcfViewpoint { guid: "vp-keep".into(), camera: Some(orthogonal_camera()), components: None, snapshot: None },
                ],
            },
        ],
        parts: vec![BcfRawPart { name: "part-keep.txt".into(), data: b"after".to_vec() }],
    }
}

//#endregion

//#region 🧪️Law7_OpTextBinaryRoundtripLaw
/// ⚖️ Law 7 — `op_text_binary_roundtrip_law` (F6): `OpText`/`OpBinary` round-trip laws for the
/// hand-rolled `BcfMutation` grammar (`f6-bcf-report.md`) — exercises every variant incl.
/// `SetViewpointCamera`'s `BcfCamera` enum payload (both `Perspective`/`Orthogonal`, plus
/// `None`) and `SetComment`'s tri-state `viewpoint_ref` (both `Some(None)` and
/// `Some(Some(_))`).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let mutations = vec![
        BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2"), index: None }),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
            guid: "t1".into(),
            title: Some("Renamed".into()),
            description: None,
            status: Some("Closed".into()),
            priority: None,
            labels: Some(vec!["Renamed".into(), "Second".into()]),
            creation_date: None,
            creation_author: None,
        }),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: sample_comment("c2", Some("vp1")), index: None }),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t1".into(), guid: "c1".into() }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: None, author: None, text: Some("Updated".into()), viewpoint_ref: Some(None) }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: Some("2025-01-01T00:00:00+00:00".into()), author: Some("a@example.com".into()), text: None, viewpoint_ref: Some(Some("vp2".into())) }),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t1".into(), viewpoint: sample_viewpoint("vp2"), index: None }),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(perspective_camera()) }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(orthogonal_camera()) }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: None }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: Some(sample_components()) }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: None }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: Some(vec![1, 2, 3]) }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: None }),
    ];
    for m in mutations {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = BcfMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = BcfMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion

//#region 🧪️Law8_DiffCodecTextBinaryRoundtripLaw
/// ⚖️ Law 8 — `diff_codec_text_binary_roundtrip_law` (F6): `DiffCodec` round-trip laws for the
/// hand-rolled `BcfDiff` grammar over the declaratively built `demo_diff_cases`.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in crate::schema::diff::demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = BcfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = BcfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion

//#region 🔖️ConformanceLaws
/// 🧪️ FG-wave: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
/// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
/// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
/// bytes, and the fixture-honesty round-trip. Lives here (the engine's own test region), not
/// any framework file -- same placement `📜️docx/…/⚙️engine/🦀️.rs`'s own
/// `conformance_laws` module uses; these tests are this artifact's OWN early-warning, plus
/// direct coverage of the mutations/diff facets the framework's `m5` auto-discovery does not
/// reach at all.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffBinary,DiffCodec,DiffText, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect -- independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
    /// message).
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", crate::standards::v2_1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", crate::standards::v2_1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", crate::standards::v2_1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", crate::standards::v2_1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", crate::standards::v2_1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", crate::standards::v2_1::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO)] {
            semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// 🗣️ The authored snapshot grammar admits actual complete logical native text.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v2_1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");

        for source in [demo_bcf_snapshot(), BcfSnapshot::default()] {
            let text = store::ArtifactDsl::print_dsl(&source);
            let (_, body) = store::semio_format::split_text_preamble(&text).unwrap();
            assert!(recognizer.recognize(body).unwrap(), "{body}");
            assert_eq!(<BcfSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(), source);
        }
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `BcfMutation` variant (`mutations::demo_mutation_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v2_1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `BcfDiff` (`diff::demo_diff_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v2_1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// 📡️ Logical snapshot records and tagged operation protocols admit actual owned bytes.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = semio_framework_dsl::parse_protocol(crate::standards::v2_1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let demo = demo_bcf_snapshot();
        let packed = store::ArtifactPack::encode_pack(&demo);
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        assert_eq!(pack_spec.schema, "stdio.bcf");
        assert_eq!(pack_spec.blocks.iter().filter(|block| matches!(block, semio_framework_dsl::Block::Record { .. })).count(), 11);
        let spec = <BcfSnapshot as store::ArtifactPack>::record_spec().unwrap();
        let (record, _) = store::pack_rt::decode_document(&inner, &spec, &store::PackDecodeOptions::default()).unwrap();
        assert_eq!(record, demo.__dsl_to_record());
        assert_eq!(<BcfSnapshot as store::ArtifactPack>::decode_pack(&packed).unwrap(), demo);

        let op_spec = semio_framework_dsl::parse_protocol(crate::standards::v2_1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = semio_framework_dsl::parse_protocol(crate::standards::v2_1::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ Shipped DSL and structural Pack retain the complete authored demo snapshot.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_bcf_snapshot();

        let parsed = <BcfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_bcf_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_bcf_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <BcfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_bcf_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_bcf_snapshot()) drifted from the shipped .pack.semio fixture");
    }

}
//#endregion 🔖️ConformanceLaws

/// ⚖️ `bcf_mutation_inverse_sum_law`: for every leaf the inverse diffs sum to the negative forward diff; every ordered
/// collection is exercised with a MIDDLE row so the inverse must restore the original position, not append.
#[semio_framework_async_macros::async_test]
async fn bcf_mutation_inverse_sum_law_holds_for_every_leaf() {
    let mut base = sample_snapshot();
    let mut middle = sample_topic("t2");
    middle.comments = vec![sample_comment("c1", None), sample_comment("c2", Some("vp1")), sample_comment("c3", None)];
    middle.viewpoints = vec![sample_viewpoint("vp1"), sample_viewpoint("vp2"), sample_viewpoint("vp3")];
    base.topics.push(middle);
    base.topics.push(sample_topic("t3"));
    for mutation in [
        BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t9"), index: None }),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t9"), index: Some(1) }),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t2".into() }),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t2".into(), title: Some("Renamed".into()), description: None, status: Some("Closed".into()), priority: None, labels: Some(vec!["Only".into()]), creation_date: None, creation_author: None }),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t2".into(), comment: sample_comment("c9", None), index: Some(1) }),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t2".into(), guid: "c2".into() }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t2".into(), guid: "c2".into(), date: None, author: Some("b@example.com".into()), text: Some("Edited".into()), viewpoint_ref: Some(None) }),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t2".into(), viewpoint: sample_viewpoint("vp9"), index: Some(2) }),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t2".into(), guid: "vp2".into() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t2".into(), guid: "vp2".into(), camera: Some(orthogonal_camera()) }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t2".into(), guid: "vp2".into(), components: None }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t2".into(), guid: "vp2".into(), snapshot: Some(vec![1, 2]) }),
        BcfMutation::SetParts(set_parts::SetParts { parts: vec![crate::schema::snapshot::BcfRawPart { name: "a.bin".into(), data: vec![1] }] }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
