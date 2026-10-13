use super::*;
use semio_s_artifact_stdio_semio_audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowNode, SemioFlowSnapshot};
use protocol::command::DiffAlgebra;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn audio_base() -> SemioSnapshot {
    SemioSnapshot { subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn flow_base() -> SemioSnapshot {
    SemioSnapshot { subset: SemioSubsetSnapshot::Flow(SemioFlowSnapshot::default()), ..Default::default() }
}

/// 🧪️ mutation_diff_law + inverse_law: a real wrapped per-field mutation (`Audio(SetSampleRate)`).
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_covers_a_wrapped_variant() {
    let base = audio_base();

    let wrapped = SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 96_000 }) });
    let d2 = <SemioMutation as Mutation<SemioSnapshot>>::diff(&wrapped, &base);
    assert!(matches!(d2.diff(), SemioDiff::Audio(_)));
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &wrapped);
    applied = __next;
    assert_eq!(protocol::apply_diff(d2.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, d2);
    let inv2 = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv2.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv2[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ mutation_diff_law, second wrapped subset (flow's id-keyed `InsertNode`) — proves
/// the dispatch works for a collection-shaped mutation, not just a scalar one.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_flow_insert_node() {
    let base = flow_base();
    let node = FlowNode { id: "n1".into(), kind: "task".into(), label: "N1".into(), params: vec![], position: SemioPoint2 { x: 1.0, y: 2.0 } };
    let wrapped = SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: SemioFlowMutation::InsertNode(semio_s_artifact_stdio_semio_flow::schema::mutations::insert_node::InsertNode { node: node.clone(), at: None }) });
    let mut applied = base.clone();
    let (__next, diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &wrapped);
    applied = __next;
    assert!(matches!(diff.diff(), SemioDiff::Flow(_)));
    match &applied.subset {
        SemioSubsetSnapshot::Flow(s) => assert_eq!(s.nodes, vec![node]),
        other => panic!("expected Flow, got {other:?}"),
    }
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base).expect("valid retained mutation inverse fixture");
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ A wrapped mutation for the wrong kind remains unapplied and records its mismatch diagnostic.
#[semio_framework_async_macros::async_test]
async fn kind_mismatch_wrapped_mutation_records_an_error_outcome() {
    let base = flow_base();
    let wrapped = SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 1 }) });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&wrapped, &base);
    assert_eq!(diff.diff(), &SemioDiff::NoChange);
    assert!(diff.messages().iter().any(|message| message.code.0 == "mutation.target-missing"));
    assert_eq!(protocol::apply_diff(diff.diff(), &base).unwrap(), base);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv, Vec::<SemioMutation>::new(), "a kind-mismatched wrapped mutation has nothing to restore");
}

/// 🧪️ Dispatch-table coverage: every one of the wrapped-kind arms round-trips a harmless
/// representative payload (proves the exhaustive `diff`/`inverse` match compiles and routes
/// correctly for every subset still carrying that vocabulary). `text`, `brep`, `mesh`, `graph`,
/// `object` and `kit` have no harmless-no-op verb, so each is exercised separately below, NOT
/// folded into this loop's `inv.len() == 1` assumption (an absent-target verb correctly returns
/// `Vec::new()` instead, since there is nothing to undo).
#[semio_framework_async_macros::async_test]
async fn all_wrapped_kinds_with_a_harmless_no_op_diff_and_inverse_route_correctly() {
    let bases: Vec<SemioSubsetSnapshot> = vec![
        SemioSubsetSnapshot::Model(Default::default()),
        SemioSubsetSnapshot::Value(Default::default()),
        SemioSubsetSnapshot::Document(Default::default()),
        SemioSubsetSnapshot::Cad(Default::default()),
        SemioSubsetSnapshot::Drawing(Default::default()),
        SemioSubsetSnapshot::Image(Default::default()),
        SemioSubsetSnapshot::Video(Default::default()),
        SemioSubsetSnapshot::Audio(Default::default()),
        SemioSubsetSnapshot::Animation(Default::default()),
        SemioSubsetSnapshot::Presentation(Default::default()),
        SemioSubsetSnapshot::Flow(Default::default()),
    ];
    // 🔧️ `match` stays exhaustive over all 18 `SemioSubsetSnapshot` arms (compiler-enforced);
    // the arms excluded from `bases` above are never reached.
    let wrap_absent_mutation = |s: &SemioSubsetSnapshot| -> SemioMutation {
        match s {
            SemioSubsetSnapshot::Brep(_) => unreachable!("excluded from `bases` above"),
            SemioSubsetSnapshot::Mesh(_) => unreachable!("excluded from `bases` above"),
            SemioSubsetSnapshot::Text(_) => unreachable!("excluded from `bases` above"),
            SemioSubsetSnapshot::Table(_) => unreachable!("excluded from `bases` above"),
            SemioSubsetSnapshot::Graph(_) => unreachable!("excluded from `bases` above"),
            SemioSubsetSnapshot::Object(_) => unreachable!("excluded from `bases` above"),
            SemioSubsetSnapshot::Kit(_) => unreachable!("excluded from `bases` above"),
            SemioSubsetSnapshot::Model(_) => SemioMutation::ApplyModel(apply_model::ApplyModel { mutation: SemioModelMutation::RemoveElement(semio_s_artifact_stdio_semio_model::schema::mutations::remove_element::RemoveElement { id: "absent".into() }) }),
            SemioSubsetSnapshot::Value(_) => SemioMutation::ApplyValue(apply_value::ApplyValue { mutation: SemioValueMutation::RemoveNode(semio_s_artifact_stdio_semio_value::schema::mutations::remove_node::RemoveNode { id: semio_s_artifact_stdio_semio_value::schema::snapshot::ValueId::new("absent") }) }),
            SemioSubsetSnapshot::Document(_) => SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation: SemioDocumentMutation::RemoveStyle(semio_s_artifact_stdio_semio_document::schema::mutations::remove_style::RemoveStyle { id: "absent".into() }) }),
            SemioSubsetSnapshot::Cad(_) => SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation: SemioCadMutation::RemoveEntity(semio_s_artifact_stdio_semio_cad::schema::mutations::remove_entity::RemoveEntity { handle: "absent".into() }) }),
            SemioSubsetSnapshot::Image(_) => SemioMutation::ApplyImage(apply_image::ApplyImage { mutation: SemioImageMutation::SetBitDepth(semio_s_artifact_stdio_semio_image::schema::mutations::set_bit_depth::SetBitDepth { bit_depth: 0 }) }),
            SemioSubsetSnapshot::Video(_) => SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation: SemioVideoMutation::RemoveStream(semio_s_artifact_stdio_semio_video::schema::mutations::remove_stream::RemoveStream { index: 0 }) }),
            SemioSubsetSnapshot::Audio(_) => SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: Default::default() }) }),
            SemioSubsetSnapshot::Animation(_) => SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation: SemioAnimationMutation::RemoveTimeline(semio_s_artifact_stdio_semio_animation::schema::mutations::remove_timeline::RemoveTimeline { index: 0 }) }),
            SemioSubsetSnapshot::Presentation(_) => SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation: SemioPresentationMutation::RemoveSlide(semio_s_artifact_stdio_semio_presentation::schema::mutations::remove_slide::RemoveSlide { index: 0 }) }),
            SemioSubsetSnapshot::Flow(_) => SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: SemioFlowMutation::RemoveNode(semio_s_artifact_stdio_semio_flow::schema::mutations::remove_node::RemoveNode { id: "absent".into() }) }),
            SemioSubsetSnapshot::Drawing(_) => SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing {
                mutation: SemioDrawingMutation::DragNodes(semio_s_artifact_stdio_semio_drawing::schema::mutations::drag_nodes::DragNodes { ats: Vec::new(), offset: SemioPoint2::default() }),
            }),
        }
    };
    for subset in bases {
        let base = SemioSnapshot { schema: "stdio.semio".into(), subset };
        let m = wrap_absent_mutation(&base.subset);
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(diff.diff().is_empty(), "wrapped no-op mutation must diff empty: {diff:?}");
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
        assert!(inv.len() <= 1, "a harmless wrapped mutation undoes in at most one step");
    }
}

/// 🧪️ `text`'s own wrapped-kind coverage: a real `InsertRun` routes through the any-level
/// dispatch, produces a nested `SemioDiff::Text`, and its inverse restores `base` exactly.
#[semio_framework_async_macros::async_test]
async fn wrapped_text_kind_diff_and_inverse_route_correctly() {
    use semio_s_artifact_stdio_semio_text::schema::mutations::insert_run;
    use semio_s_artifact_stdio_semio_text::schema::snapshot::SemioTextRun;

    let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Text(Default::default()) };
    let m = SemioMutation::ApplyText(apply_text::ApplyText { mutation: SemioTextMutation::InsertRun(insert_run::InsertRun { index: 0, run: SemioTextRun { language: "en".into(), content: "hi".into(), marks: vec![] } }) });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
    assert!(matches!(diff.diff(), SemioDiff::Text(_)));
    assert!(!diff.diff().is_empty());
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &m);
    applied = __next;
    assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, diff);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ `brep`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above): a real
/// `CreateVertex` routes through the any-level dispatch, produces a nested `SemioDiff::Brep`,
/// and its inverse restores `base` exactly.
#[semio_framework_async_macros::async_test]
async fn wrapped_brep_kind_diff_and_inverse_route_correctly() {
    use semio_s_artifact_stdio_semio_brep::schema::mutations::create_vertex;

    let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Brep(Default::default()) };
    let m = SemioMutation::ApplyBrep(apply_brep::ApplyBrep {
        mutation: SemioBrepMutation::CreateVertex(create_vertex::CreateVertex { id: "v1".into(), point: semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 }, tol: 1e-7, at: None }),
    });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
    assert!(matches!(diff.diff(), SemioDiff::Brep(_)));
    assert!(!diff.diff().is_empty());
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &m);
    applied = __next;
    assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, diff);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ `mesh`'s own wrapped-kind coverage (mirrors `wrapped_brep_kind_…` above): a real
/// `CreateMesh` routes through the any-level dispatch, produces a nested `SemioDiff::Mesh`, and
/// its inverse restores `base` exactly.
#[semio_framework_async_macros::async_test]
async fn wrapped_mesh_kind_diff_and_inverse_route_correctly() {
    use semio_s_artifact_stdio_semio_mesh::schema::mutations::create_mesh;
    use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMesh;

    let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Mesh(Default::default()) };
    let m = SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::CreateMesh(create_mesh::CreateMesh { mesh: SemioMesh { id: "m1".into(), primitives: vec![] }, at: None }) });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
    assert!(matches!(diff.diff(), SemioDiff::Mesh(_)));
    assert!(!diff.diff().is_empty());
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &m);
    applied = __next;
    assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, diff);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ `table`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above): a real
/// `InsertRow` routes through the any-level dispatch, produces a nested `SemioDiff::Table`, and
/// its inverse restores `base` exactly.
#[semio_framework_async_macros::async_test]
async fn wrapped_table_kind_diff_and_inverse_route_correctly() {
    use semio_s_artifact_stdio_semio_table::schema::mutations::insert_row;
    use semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableRow;

    let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Table(Default::default()) };
    let m = SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: SemioTableMutation::InsertRow(insert_row::InsertRow { index: 0, row: SemioTableRow { cells: vec![] } }) });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
    assert!(matches!(diff.diff(), SemioDiff::Table(_)));
    assert!(!diff.diff().is_empty());
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &m);
    applied = __next;
    assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, diff);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ `graph`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above): a real
/// `CreateNode` routes through the any-level dispatch, produces a nested `SemioDiff::Graph`,
/// and its inverse restores `base` exactly.
#[semio_framework_async_macros::async_test]
async fn wrapped_graph_kind_diff_and_inverse_route_correctly() {
    use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
    use semio_s_artifact_stdio_semio_graph::schema::mutations::create_node;
    use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphNodeId;

    let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Graph(Default::default()) };
    let m = SemioMutation::ApplyGraph(apply_graph::ApplyGraph {
        mutation: SemioGraphMutation::CreateNode(create_node::CreateNode { id: GraphNodeId::new("n1"), kind: "task".into(), label: "N1".into(), position: SemioPoint2::default(), width: 0.0, height: 0.0, ports: vec![], properties: vec![], at: None }),
    });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
    assert!(matches!(diff.diff(), SemioDiff::Graph(_)));
    assert!(!diff.diff().is_empty());
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &m);
    applied = __next;
    assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, diff);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ `object`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above): a real
/// `CreateBrep` routes through the any-level dispatch, produces a nested `SemioDiff::Object`,
/// and its inverse restores `base` exactly. `object` is the first COMPOSITE subset wrapped
/// here — the mutation touches a CHILD slot (`brep`), not a scalar/collection field.
#[semio_framework_async_macros::async_test]
async fn wrapped_object_kind_diff_and_inverse_route_correctly() {
    use semio_s_artifact_stdio_semio_object::schema::mutations::create_brep;

    let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Object(Default::default()) };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "brep-x".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() } };
    let m = SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: SemioObjectMutation::CreateBrep(create_brep::CreateBrep { child_id: "brep-x".into(), target }) });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
    assert!(matches!(diff.diff(), SemioDiff::Object(_)));
    assert!(!diff.diff().is_empty());
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &m);
    applied = __next;
    assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, diff);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ `kit`'s own wrapped-kind coverage (mirrors `wrapped_object_kind_…` above): a real
/// `AddType` routes through the any-level dispatch, produces a nested `SemioDiff::Kit`, and its
/// inverse restores `base` exactly. `kit` is the SECOND composite subset and the first to carry
/// a LINK slot, though this particular case exercises a plain value-collection mutation.
#[semio_framework_async_macros::async_test]
async fn wrapped_kit_kind_diff_and_inverse_route_correctly() {
    use semio_s_artifact_stdio_semio_kit::schema::mutations::add_type;

    let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Kit(Default::default()) };
    let m = SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: SemioKitMutation::AddType(add_type::AddType { id: "chair".into(), name: "Chair".into(), category: "furniture".into(), at: None }) });
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
    assert!(matches!(diff.diff(), SemioDiff::Kit(_)));
    assert!(!diff.diff().is_empty());
    let mut applied = base.clone();
    let (__next, returned_diff) = semio_s_artifact_stdio_semio_base::applied(&applied, &m);
    applied = __next;
    assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), applied);
    assert_eq!(returned_diff, diff);
    let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
    let mut restored = applied;
    let (__next, _) = semio_s_artifact_stdio_semio_base::applied(&restored, &inv[0]);
    restored = __next;
    assert_eq!(restored, base);
}

/// 🧪️ op_text_binary_roundtrip_law across a wrapped variant.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let _base = audio_base();
    let cases = [
        SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 22_050 }) }),
    ];
    for m in cases {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}

//#region 🔖️CatalogLaw
/// 🏷️ The wildcard-free spelling map that makes [`KINDS`] compiler-checked: a nineteenth arm has
/// no case here, so the crate stops building until both this match and `KINDS` name it.
fn kind_of(mutation: &SemioMutation) -> &'static str {
    match mutation {
        SemioMutation::ApplyBrep(_) => "apply-brep",
        SemioMutation::ApplyMesh(_) => "apply-mesh",
        SemioMutation::ApplyModel(_) => "apply-model",
        SemioMutation::ApplyValue(_) => "apply-value",
        SemioMutation::ApplyDocument(_) => "apply-document",
        SemioMutation::ApplyCad(_) => "apply-cad",
        SemioMutation::ApplyDrawing(_) => "apply-drawing",
        SemioMutation::ApplyImage(_) => "apply-image",
        SemioMutation::ApplyVideo(_) => "apply-video",
        SemioMutation::ApplyAudio(_) => "apply-audio",
        SemioMutation::ApplyAnimation(_) => "apply-animation",
        SemioMutation::ApplyPresentation(_) => "apply-presentation",
        SemioMutation::ApplyFlow(_) => "apply-flow",
        SemioMutation::ApplyText(_) => "apply-text",
        SemioMutation::ApplyTable(_) => "apply-table",
        SemioMutation::ApplyGraph(_) => "apply-graph",
        SemioMutation::ApplyObject(_) => "apply-object",
        SemioMutation::ApplyKit(_) => "apply-kit",
    }
}

fn enveloped(subset: SemioSubsetSnapshot) -> SemioSnapshot {
    SemioSnapshot { schema: crate::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset }
}

/// 🏷️ `KINDS` must name every declared variant, in declaration order and in the exact spelling
/// the committed `semio-v1-base` catalog carries — the framework never parses Rust, so this is
/// the only thing that keeps the catalog honest against the enum. The eighteen wrapper spellings
/// are checked against `semio_subset_tag`, the envelope's OWN runtime discriminator, rather than
/// against a second hand-written list: a routed mutation that reported under a name the catalog
/// does not know would otherwise pass unnoticed.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let arms = [
        SemioSubsetSnapshot::Brep(Default::default()),
        SemioSubsetSnapshot::Mesh(Default::default()),
        SemioSubsetSnapshot::Model(Default::default()),
        SemioSubsetSnapshot::Value(Default::default()),
        SemioSubsetSnapshot::Document(Default::default()),
        SemioSubsetSnapshot::Cad(Default::default()),
        SemioSubsetSnapshot::Drawing(Default::default()),
        SemioSubsetSnapshot::Image(Default::default()),
        SemioSubsetSnapshot::Video(Default::default()),
        SemioSubsetSnapshot::Audio(Default::default()),
        SemioSubsetSnapshot::Animation(Default::default()),
        SemioSubsetSnapshot::Presentation(Default::default()),
        SemioSubsetSnapshot::Flow(Default::default()),
        SemioSubsetSnapshot::Text(Default::default()),
        SemioSubsetSnapshot::Table(Default::default()),
        SemioSubsetSnapshot::Graph(Default::default()),
        SemioSubsetSnapshot::Object(Default::default()),
        SemioSubsetSnapshot::Kit(Default::default()),
    ];
    assert_eq!(KINDS.len(), arms.len(), "KINDS must name each subset arm");
    for (kind, arm) in KINDS.iter().zip(arms) {
        assert_eq!(*kind, format!("apply-{}", semio_subset_tag(&enveloped(arm))), "KINDS must follow SemioSubsetSnapshot's own declaration order and wrap the envelope's own runtime subset tag");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🔖️CatalogLaw
