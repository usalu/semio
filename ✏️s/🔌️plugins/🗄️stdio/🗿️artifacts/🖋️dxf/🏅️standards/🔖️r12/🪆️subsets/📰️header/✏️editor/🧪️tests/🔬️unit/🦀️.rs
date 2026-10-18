use crate::standards::v_r12::subsets::any::schema::mutations::DxfMutation;
use crate::standards::v_r12::subsets::any::schema::snapshot::DxfSnapshot;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
use semio_s_artifact_stdio_contract::editing;
#[allow(unused_imports)]
use crate::editor::dxf::modes::edit;
#[allow(unused_imports)]
use crate::editor::dxf::modes::edit::windows::main;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_plugin::{retained_command::ArtifactRetainedCommandInputs, retained_command::ArtifactRetainedCommandJob, retained_command::ArtifactRetainedCommandPayload, retained_command::BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_framework_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresence;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransient;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_artifact_reference::StandardId;
#[allow(unused_imports)]
use semio_framework_artifact_reference::SubsetId;
#[allow(unused_imports)]
use semio_framework_plugin::ToolExecutionContract;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_plugin::ToolOperationSpec;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::apply_mutation;
use crate::editor::dxf::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_dxf_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DXF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DxfAnyEditor as ArtifactEditor>::DIALECT, DXF_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<DxfAnyEditor, crate::viewer::dxf::DxfAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", crate::editor::dxf::DxfAnyEditor, || semio_framework_plugin::App { definition: crate::editor::dxf::create_dxf_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️r12/🪆️subsets/📰️header");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_row() {
    use crate::schema::snapshot::{DxfEntity, DxfLine, DxfHeaderVar, DxfLayer, DxfValue};
    use crate::standards::v_r12::subsets::any::schema::mutations::{DxfMutation};
    let mut base = DxfSnapshot::default();
    base.tables.layers.push(DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] });
    base.entities.push(DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }));
    base.header_vars.push(DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] });
    let emit = |event: editing::SnapshotEditEvent| <DxfAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let recolor = emit(editing::SnapshotEditEvent::SetValue { path: "/tables/layers/0/color".into(), value: semio_framework_value::DslValue::uint(3) }).expect("a layer field edit resolves");
    let [mutation @ DxfMutation::SetLayer(_)] = recolor.artifact_mutations.as_slice() else { panic!("a layer field edit raises the layer kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.tables.layers[0].color, 3);
    let removal = emit(editing::SnapshotEditEvent::RemoveValue { path: "/entities/0".into() }).expect("an entity removal resolves");
    assert!(matches!(removal.artifact_mutations.as_slice(), [DxfMutation::RemoveEntity(_)]));
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/headerVars/0".into() }).expect("a header variable removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [DxfMutation::RemoveHeaderVar(_)]));
    let unnamed = emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("the schema stamp has no kind");
    assert_eq!(unnamed.code.0, "snapshot-edit.unsupported-path");
}
