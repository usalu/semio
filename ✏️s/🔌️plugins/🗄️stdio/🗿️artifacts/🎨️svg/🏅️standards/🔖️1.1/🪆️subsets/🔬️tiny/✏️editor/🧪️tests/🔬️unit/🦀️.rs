use crate::standards::v1_1::subsets::tiny::schema::snapshot::SvgSnapshot;
use crate::SVG_TINY_DIALECT;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use crate::editor::svg_tiny::modes::edit;
#[allow(unused_imports)]
use crate::editor::svg_tiny::modes::edit::windows::main;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::tiny::schema::mutations::edit_rules;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::tiny::schema::mutations::SvgTinyMutation;
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
use semio_framework_2d::compute::EngineHandles;
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::editing;
use crate::editor::svg_tiny::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_svg_tiny_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, SVG_TINY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SvgTinyEditor as ArtifactEditor>::DIALECT, SVG_TINY_DIALECT);
}

#[test]
fn natural_file_route_enforces_tiny_and_publishes_the_decoded_snapshot() {
    let codec = <SvgTinyEditor as ArtifactEditor>::natural_file_codec().expect("SVG Tiny natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.svg@1.1", ".svg", "image/svg+xml", false));
    let source = include_bytes!("../../../🧪️tests/🧫️fixtures/✍️set-text-applied/➡️after.svg");
    let imported = <SvgTinyEditor as ArtifactEditor>::decode_natural_file(source).expect("SVG Tiny natural import");
    let exported = <SvgTinyEditor as ArtifactEditor>::encode_natural_file(&imported).expect("SVG Tiny natural export");
    let independent = semio_s_artifact_stdio_svg_test_oracle::standards::v1_1::subsets::tiny::oracle_round_trip(&exported).expect("quick-xml reopens SVG Tiny export");
    SvgSnapshot::import_utf8(&independent).expect("independent SVG Tiny output reopens");

    let outside = br#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" baseProfile="tiny"><filter/></svg>"#;
    assert!(<SvgTinyEditor as ArtifactEditor>::decode_natural_file(outside).is_err());
    let outside = SvgSnapshot::import_utf8(outside).unwrap();
    assert!(<SvgTinyEditor as ArtifactEditor>::encode_natural_file(&outside).is_err());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", crate::editor::svg_tiny::SvgTinyEditor, || semio_framework_plugin::App { definition: crate::editor::svg_tiny::create_svg_tiny_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny");
