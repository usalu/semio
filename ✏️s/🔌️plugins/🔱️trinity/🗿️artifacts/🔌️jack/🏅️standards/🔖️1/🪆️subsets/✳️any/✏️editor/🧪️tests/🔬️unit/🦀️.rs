//! 🧪️ The intrinsic editor command bridge stays independent of concrete member kinds.
#[allow(unused_imports)]
use crate::editor::jack::commands;
#[allow(unused_imports)]
use crate::editor::jack::transient::{JackEditorWindowTransientOwner, JackResultsWindowTransientOwner};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::graph::{TRINITY_GRAPH_SCHEMA, TRINITY_JACK_DIALECT};
#[allow(unused_imports)]
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
#[allow(unused_imports)]
use semio_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_plugin::ActionDescriptor;
#[allow(unused_imports)]
use semio_plugin::ActionKind;
#[allow(unused_imports)]
use semio_plugin::AppActionRegistry;
#[allow(unused_imports)]
use semio_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_plugin::ArtifactKindSpec;
#[allow(unused_imports)]
use semio_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_plugin::ArtifactOwnedToolJobRequest;
#[allow(unused_imports)]
use semio_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_plugin::ConfigView;
#[allow(unused_imports)]
use semio_plugin::ContextMenuItemSpec;
#[allow(unused_imports)]
use semio_plugin::ContextMenuRequest;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_plugin::DomainTopology;
#[allow(unused_imports)]
use semio_plugin::DraftView;
#[allow(unused_imports)]
use semio_plugin::Editor;
#[allow(unused_imports)]
use semio_plugin::EditorApp;
#[allow(unused_imports)]
use semio_plugin::Effect;
#[allow(unused_imports)]
use semio_plugin::Emit;
#[allow(unused_imports)]
use semio_plugin::EphemeralEmit;
#[allow(unused_imports)]
use semio_plugin::Fault;
#[allow(unused_imports)]
use semio_plugin::GranularityDefinition;
#[allow(unused_imports)]
use semio_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_plugin::InteractionTopology;
#[allow(unused_imports)]
use semio_ui_locale::Label;
#[allow(unused_imports)]
use semio_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_plugin::Media;
#[allow(unused_imports)]
use semio_plugin::MediaClass;
#[allow(unused_imports)]
use semio_plugin::MediaError;
#[allow(unused_imports)]
use semio_plugin::MediaForm;
#[allow(unused_imports)]
use semio_plugin::MediaPayload;
#[allow(unused_imports)]
use semio_plugin::MediaType;
#[allow(unused_imports)]
use semio_plugin::MergeMode;
#[allow(unused_imports)]
use semio_plugin::NoConfig;
#[allow(unused_imports)]
use semio_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_plugin::NoDraft;
#[allow(unused_imports)]
use semio_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_plugin::NoTransient;
#[allow(unused_imports)]
use semio_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_plugin::NodeGraphEdgeRecord;
#[allow(unused_imports)]
use semio_plugin::NodeGraphNodeRecord;
#[allow(unused_imports)]
use semio_plugin::NodeGraphPortRecord;
#[allow(unused_imports)]
use semio_plugin::PanelGroup;
#[allow(unused_imports)]
use semio_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_plugin::TopologyNode;
#[allow(unused_imports)]
use semio_plugin::WindowMeasure;
#[allow(unused_imports)]
use semio_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
#[allow(unused_imports)]
use semio_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL;
#[allow(unused_imports)]
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
#[allow(unused_imports)]
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;
#[allow(unused_imports)]
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
#[allow(unused_imports)]
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
#[allow(unused_imports)]
use semio_os_kernel::Viewport2d;
#[allow(unused_imports)]
use semio_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
#[allow(unused_imports)]
use pack::snapshot::{ArtifactDsl, ArtifactPack};
#[allow(unused_imports)]
use crate::editor::jack::retained_commands::*;
#[allow(unused_imports)]
use crate::editor::jack::modes::edit;
use crate::editor::jack::*;
#[test]fn intrinsic_command_codec_preserves_literal_query(){use protocol::OpBinary;let value=TrinityJackCommand::RunQuery{query:Some("MATCH (a:Piece) RETURN a".into()),results_window_id:"literal@results".into()};let wire=value.encode_op().unwrap();assert_eq!(TrinityJackCommand::decode_op(&wire).unwrap(),value);}
