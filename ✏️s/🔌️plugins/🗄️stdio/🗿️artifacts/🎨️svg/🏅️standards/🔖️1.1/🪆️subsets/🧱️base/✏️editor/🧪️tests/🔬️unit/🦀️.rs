use crate::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
use crate::SVG_ANY_DIALECT;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use crate::editor::svg_any::modes::edit;
#[allow(unused_imports)]
use crate::editor::svg_any::modes::edit::windows::main;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::base::schema::mutations::edit_rules;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation;
#[allow(unused_imports)]
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
#[allow(unused_imports)]
use semio_framework_plugin::{retained_command::ArtifactRetainedCommandInputs, retained_command::ArtifactRetainedCommandJob, retained_command::ArtifactRetainedCommandPayload, retained_command::BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_framework_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactStoreInitializationJob;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
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
use semio_framework_2d_compute::EngineHandles;
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::editing;
use crate::editor::svg_any::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_svg_any_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, SVG_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SvgAnyEditor as ArtifactEditor>::DIALECT, SVG_ANY_DIALECT);
}

#[test]
fn natural_file_route_round_trips_svg_with_an_independent_xml_oracle_and_fresh_snapshot_event() {
    let codec = <SvgAnyEditor as ArtifactEditor>::natural_file_codec().expect("SVG natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.svg@1.1", ".svg", "image/svg+xml", false));
    let source = include_bytes!("../../../🧪️tests/🧫️fixtures/🎨️native-third-party-shape.svg");
    let current = <SvgAnyEditor as ArtifactEditor>::initial_snapshot();
    let imported = <SvgAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("SVG natural import");
    let exported = <SvgAnyEditor as ArtifactEditor>::encode_natural_file(&imported).expect("SVG natural export");
    let independent = semio_s_artifact_stdio_svg_test_oracle::standards::v1_1::subsets::base::oracle_round_trip(&exported).expect("quick-xml reopens SVG export");
    let independent = SvgSnapshot::import_utf8(&independent).expect("independent SVG output reopens");
    assert_eq!(independent.doc.root, imported.doc.root);
    assert_eq!(current, <SvgAnyEditor as ArtifactEditor>::initial_snapshot(), "opening does not replace the selected owner before publication");
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", crate::editor::svg_any::SvgAnyEditor, || semio_framework_plugin::App { definition: crate::editor::svg_any::create_svg_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.1/🪆️subsets/🧱️base");
