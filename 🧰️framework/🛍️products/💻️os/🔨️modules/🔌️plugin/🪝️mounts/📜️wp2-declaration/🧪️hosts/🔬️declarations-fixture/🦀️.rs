//! 🔬️ Test host of `declarations`: runs `🔬️app-declarations-fixture` in the facade, where the names it uses are in scope.
//! 🧪️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-C Task 4 — the executable
//! specification: a synthetic TWO-standard, THREE-subset artifact (`crate::declarations::
//! fixture`) declared entirely through this tree, proving open→mutate→save works end to end
//! and that `io_route` finds the `standard 1 / subset strict` conformance-profile hop back to
//! `standard 1 / subset any`. Plugin waves reading this ticket copy this shape verbatim.

use semio_framework::AppDefinition;
use semio_framework::kernel::ActorId;
use semio_framework_artifact_reference::ArtifactKindId;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_artifact_reference::Dialect;
use semio_framework_artifact_reference::StandardId;
use semio_framework::AppRole;
use semio_framework_plugin_reactor_app::NoPluginApp;
use semio_framework_plugin_app_builder::ExampleSource;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_surface::EditorApp;
use semio_framework_plugin_decl_surface::ViewerApp;
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
#[allow(unused_imports)]
use semio_framework_plugin_decl_registry::declarations::{ArtifactDeclaration, IoDeclaration, LanguagePair, MediaDeclaration, NativeCodecs, SchemaDeclaration, StandardDeclaration, SubsetDeclaration, format_descriptor_of, project_artifact_declarations};
#[allow(unused_imports)]
use semio_framework_plugin_instance_app::lifted_tree::{editor_surface, viewer_surface};
include!("../../../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs");
