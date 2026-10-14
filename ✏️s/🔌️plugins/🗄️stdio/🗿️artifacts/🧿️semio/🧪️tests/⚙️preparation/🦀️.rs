//! 🧪️ Runtime law: every semio subset editor publishes through the generic `mutation_apply` document lane over a `RetainedClone` snapshot tree. Apply prepares the exact post snapshot, Undo replays the ordered inverse rows back to the base and Redo prepares the identical post and inverse again.

#[global_allocator]
static ORIGINAL_OWNER_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_os_kernel as store;
use semio_framework_plugin::ArtifactEditor;
use {semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor, semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor, semio_s_artifact_stdio_semio_text::editor::SemioTextEditor, semio_s_artifact_stdio_semio_value::editor::SemioValueEditor};
use {semio_s_artifact_stdio_semio_document::schema::mutations::set_block_content::SetBlockContent, semio_s_artifact_stdio_semio_document::schema::mutations::{DocBlockPath, DocPathSegment, SemioDocumentMutation}};
use semio_s_artifact_stdio_semio_document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot};
use {semio_s_artifact_stdio_semio_flow::schema::mutations::set_node_label::SetNodeLabel, semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation};
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowNode, SemioFlowSnapshot};
use {semio_s_artifact_stdio_semio_text_mutations::schema::mutations::edit_run::EditRun, semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation};
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextRun;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use {semio_s_artifact_stdio_semio_value_mutations::schema::mutations::set_node::SetNode, semio_s_artifact_stdio_semio_value_mutations::schema::mutations::set_value::SetValue, semio_s_artifact_stdio_semio_value_mutations::schema::mutations::{SemioValueMutation, SemioValuePathSegment}};
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueNode;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::ValueId;
use std::fmt::Debug;

fn grant() -> store::os_store::one_item::ArtifactStoreOneItemGrant {
    store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 }
}

fn authoring_law<E: ArtifactEditor>(label: &str, base: E::Snapshot, mutation: E::Mutation, expected: E::Snapshot)
where
    E::Snapshot: Clone + PartialEq + Debug,
    E::Mutation: Clone + PartialEq + Debug + store::ArtifactCanonicalJsonTree,
{
    let factory = <E as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the semio document lane declares a preparation factory");
    let apply = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant());
    assert_ne!(base, expected, "{label}: the mutation must change the snapshot");
    assert_eq!(*apply.post, expected, "{label}: Apply publishes the exact post snapshot");
    let mut current = (*apply.post).clone();
    for row in apply.inverse.iter().cloned() {
        current = (*store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, current, row, grant()).post).clone();
    }
    assert_eq!(current, base, "{label}: Undo restores the base");
    let redo = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, current, mutation, grant());
    assert_eq!(*redo.post, expected, "{label}: Redo publishes the same post snapshot");
    assert_eq!(redo.inverse, apply.inverse, "{label}: Redo derives the same inverse rows");
    eprintln!("[DEBUG] semio {label} apply turns={} undo rows={} redo turns={} retained={}", apply.turns, apply.inverse.len(), redo.turns, apply.retained_capacity_bytes);
}

fn text(value: &str) -> SemioValue {
    SemioValue::Str { value: value.into() }
}

fn value_snapshot(leaf: &str) -> SemioValueSnapshot {
    let inner = SemioValue::Map { entries: vec![SemioValueEntry { key: "inner".into(), value: text(leaf) }] };
    let outer = SemioValue::List { items: vec![SemioValue::Int { lexeme: "1".into() }, inner] };
    SemioValueSnapshot { schema: "stdio.semio.value".into(), root: SemioValue::Map { entries: vec![SemioValueEntry { key: "outer".into(), value: outer }] }, nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Bool { value: true } }] }
}

fn nested_lists(depth: usize, leaf: &str) -> SemioValue {
    (0..depth).fold(text(leaf), |inner, _| SemioValue::List { items: vec![inner] })
}

#[test]
fn value_apply_undo_redo_over_a_recursive_snapshot() {
    let path = vec![SemioValuePathSegment::Key { key: "outer".into() }, SemioValuePathSegment::Index { index: 1 }, SemioValuePathSegment::Key { key: "inner".into() }];
    authoring_law::<SemioValueEditor>("value set-value", value_snapshot("x"), SemioValueMutation::SetValue(SetValue { path, value: text("y") }), value_snapshot("y"));
    let mut expected = value_snapshot("x");
    expected.nodes[0].value = SemioValue::Bytes { value: vec![0, 1, 255] };
    authoring_law::<SemioValueEditor>("value set-node", value_snapshot("x"), SemioValueMutation::SetNode(SetNode { id: ValueId::new("n1"), value: SemioValue::Bytes { value: vec![0, 1, 255] }, at: None }), expected);
}

#[test]
fn value_apply_undo_redo_at_depth_sixty_four() {
    let snapshot = |leaf: &str| SemioValueSnapshot { schema: "stdio.semio.value".into(), root: nested_lists(64, leaf), nodes: Vec::new() };
    let path = vec![SemioValuePathSegment::Index { index: 0 }; 64];
    authoring_law::<SemioValueEditor>("value depth 64", snapshot("deep"), SemioValueMutation::SetValue(SetValue { path, value: text("changed") }), snapshot("changed"));
}

#[test]
fn document_apply_undo_redo_inside_a_nested_quote() {
    let snapshot = |inner: DocBlock| SemioDocumentSnapshot { schema: "s.stdio.semio.document".into(), styles: Vec::new(), images: Vec::new(), blocks: vec![DocBlock::paragraph("alpha"), DocBlock::Quote { blocks: vec![inner] }] };
    let heading = DocBlock::Heading { level: 2, style_id: None, runs: vec![DocRun::plain("beta")] };
    let path = DocBlockPath { segments: vec![DocPathSegment::Quote { block_index: 1 }], index: 0 };
    authoring_law::<SemioDocumentEditor>("document set-block-content", snapshot(DocBlock::paragraph("beta")), SemioDocumentMutation::SetBlockContent(SetBlockContent { path, block: heading.clone() }), snapshot(heading));
}

#[test]
fn text_apply_undo_redo_of_one_run() {
    let snapshot = |second: &str| SemioTextSnapshot { schema: "stdio.semio.text".into(), runs: vec![SemioTextRun { language: "en".into(), content: "alpha".into(), marks: Vec::new() }, SemioTextRun { language: "en".into(), content: second.into(), marks: Vec::new() }] };
    authoring_law::<SemioTextEditor>("text edit-run", snapshot("beta"), SemioTextMutation::EditRun(EditRun { index: 1, new_text: "gamma".into() }), snapshot("gamma"));
}

#[test]
fn flow_apply_undo_redo_of_one_node_label() {
    let snapshot = |label: &str| SemioFlowSnapshot { schema: "stdio.semio.flow".into(), nodes: vec![FlowNode { id: "n1".into(), kind: "task".into(), label: label.into(), ..Default::default() }], edges: Vec::new() };
    authoring_law::<SemioFlowEditor>("flow set-node-label", snapshot("Build"), SemioFlowMutation::SetNodeLabel(SetNodeLabel { id: "n1".into(), label: "Ship".into() }), snapshot("Ship"));
}
