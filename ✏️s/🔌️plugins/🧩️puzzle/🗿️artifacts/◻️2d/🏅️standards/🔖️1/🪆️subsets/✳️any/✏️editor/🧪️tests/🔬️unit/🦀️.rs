use crate::editor::puzzle2d::snapshot::{PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, Puzzle2dInteractionSnapshot, Puzzle2dScene, puzzle2d_entity_hidden, board_snapshot_nodes, board_snapshot_edges, concrete_forest_example_json, nakagin_example_json, PUZZLE2D_PANES, puzzle2d_node_display_label, puzzle2d_next_node_label, puzzle2d_node_kind_options, PUZZLE2D_NODE_KIND_OPTIONS_MAX, puzzle2d_node_kind_rows, PUZZLE2D_SHIPPED_NODE_KINDS, empty_board_snapshot};
use crate::editor::puzzle2d::commands::set_active_example;
use crate::editor::puzzle2d::config::Puzzle2dConfig;
use crate::editor::puzzle2d::engine::BoardHost;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::{area_brush as area_brush_utility, brush as brush_utility, select as select_utility};
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview};
use crate::editor::puzzle2d::panels::settings;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_pack_json::Value;
#[allow(unused_imports)]
use crate::editor::puzzle2d::snapshot::{puzzle2d_scene_for, puzzle2d_scene_with, PUZZLE2D_GENERIC_TOOL_IDS, PUZZLE2D_RETAINED_TOOL_IDS, PUZZLE2D_ADD_NODE_DIALOG_ID, puzzle2d_active_utility, puzzle2d_addresses_locked_entity, puzzle2d_entity_label, puzzle2d_interaction_definition, puzzle2d_node_kind_arg, puzzle2d_select_scope, puzzle2d_selected_node_ids, puzzle2d_selection_is_locked, puzzle2d_window_and_engagements_scope, puzzle2d_window_only_scope};
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::board_host::drain_board_events_json;
#[allow(unused_imports)]
use crate::editor::puzzle2d::recorder::{puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_paste_operations_on};
#[allow(unused_imports)]
use crate::editor::puzzle2d::config::Puzzle2dConfigMutation;
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::Point;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit;
#[allow(unused_imports)]
use crate::editor::puzzle2d::precompute::fill as fill_run;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::selection;
#[allow(unused_imports)]
use crate::editor::puzzle2d::panels::{artifact, catalogue, inspection};
#[allow(unused_imports)]
use crate::editor::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
#[allow(unused_imports)]
use crate::editor::puzzle2d::window::{self, Puzzle2dWindowConfig, Puzzle2dWindowTransient};
#[allow(unused_imports)]
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, PastePlacement};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::ActionRef;
#[allow(unused_imports)]
use semio_framework_plugin::AppIo;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactPresentation;
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
use semio_framework_plugin::DialogDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTarget;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionVerb;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MediaClass;
#[allow(unused_imports)]
use semio_framework_plugin::MediaForm;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortDirection;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortSpec;
#[allow(unused_imports)]
use semio_framework_plugin::MediaType;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::PortMultiplicity;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_pack_json::json;
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet};
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest};
#[allow(unused_imports)]
use crate::editor::puzzle2d::kit::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::retained::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::session::*;
use semio_framework_test_fixtures::fixture_text;

include!("🧰️history-acceptance-source-laws-and-camera-helper/🦀️.rs");
include!("🧮️operations-and-command-envelope-laws/🦀️.rs");
include!("🎯️board-event-and-ui-scope-laws/🦀️.rs");
include!("📜️manifest-convergence-and-registry-laws/🦀️.rs");
include!("🧩️engine-parse-and-handle-suggestion-laws/🦀️.rs");
include!("🏷️display-label-inspector-edit-and-window-option-laws/🦀️.rs");
include!("🕹️gumball-region-set-verb-and-snapshot-laws/🦀️.rs");

/// 👥️ Puzzle camera publication uses the same native transport and departure law as FEM.
#[semio_framework_async_macros::async_test]
async fn mounted_camera_and_interaction_presence_reaches_peers_without_document_edits() {
    use semio_framework_value::FromValue;
    use semio_framework_plugin::ArtifactEditor;
    use semio_framework_plugin::EditorApp;
    use semio_framework_plugin::artifact_app_laws::assert_presence_round_trip;
    use semio_framework_plugin::artifact_app_laws::fixture_identity;
    use semio_framework_plugin::artifact_app_laws::fixture_mounted_policy;
    use semio_framework_plugin::artifact_app_laws::meta;
    use semio_framework_plugin::artifact_app_laws::paired_registered_apps;
    use semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation;
    use semio_framework_plugin::artifact_app_laws::with_document_view;
    use semio_framework_plugin::PluginApp;
    let fixture = semio_framework_pack_json::parse(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/📷️camera/🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("portable shared presence fixture");
    let expected = protocol::PresenceWindowView::from_value(semio_framework_pack_json::to_dsl_value(&fixture["admission"][0]["value"])).expect("admitted shared camera");
    let kind = semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID;
    let view = window_view(kind, &expected.window_id);
    let protocol::PresenceViewKind::Canvas { x, y, zoom } = expected.kind else { panic!("portable canvas pose") };
    let command = <Puzzle2dPlayApp as ArtifactEditor>::command_from_action("setCamera", Some(&semio_framework_value::DslValue::from(&json!({"camera": {"x": x, "y": y, "zoom": zoom}})))).expect("typed shared camera command");
    let mut identity=fixture_identity();let policy=fixture_mounted_policy();
    let(mut a,mut b)=paired_registered_apps::<EditorApp<Puzzle2dPlayApp>,_,_>("mem://puzzle-presence-real-node",||async{semio_framework_plugin::App{definition:create_puzzle2d_app(),examples:Vec::new()}},policy,&mut identity).await;
    let setup=<Puzzle2dPlayApp as ArtifactEditor>::command_from_action("addNode",Some(&semio_framework_value::DslValue::from(&json!({"kind":"node"})))).expect("registered source entity command");a.dispatch_typed(setup,&meta("local"),&mut identity).await.expect("source node event admission");settle_registered_typed_operation(&mut a,meta("local").instance_id,policy,&mut identity).await.expect("source node event publication");b.tick_backbone(&mut identity).await.expect("peer receives original node event");
    let selected=with_document_view(&mut a,|doc|doc.snapshot.typed().nodes.first().expect("actual published source node").id.to_string_owner()).await.expect("original source document lease");
    assert_eq!(a.document_pack().await.expect("source shared document"),b.document_pack().await.expect("peer shared document"),"original source events converge before ephemeral interaction");
    assert_presence_round_trip(&mut a,&mut b,command,view,expected,(PUZZLE2D_INTERACTION_DOMAIN,Some((PUZZLE2D_GRANULARITY_NODE,selected.as_str()))),policy,&mut identity).await;
}

