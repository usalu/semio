//! 📡️ Test host of `plugin_runtime`: runs `🔬️plugin-runtime-contributed-mutation-wire` in the facade, where the names it uses are in scope.

use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework::FaultFrom;
#[allow(unused_imports)]
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
#[allow(unused_imports)]
use semio_framework_plugin_plugin_program::{decode_wire_serialized, encode_wire_serialized, wire_artifact_mutation_plan};
include!("../../../../🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs");
