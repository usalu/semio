//! 🧰️ Shared fixture of the puzzle3d editor integration targets — the raw app, its runtime budgets, render and dispatch drivers, and the heap witness.

use super::*;
use semio_framework_plugin::ActionMeta;
use semio_framework_plugin::App;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::InvocationResult;
use semio_framework_plugin::MAINTENANCE_STAGES;
use semio_framework_plugin::PluginApp;
use semio_framework_plugin::PluginLifecycleStep;
use semio_framework_plugin::VcsArtifactApp;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::ViewWindowInstance;
use semio_framework_plugin::artifact_app_laws;

include!("🏗️raw-app-fixture/🦀️.rs");

include!("🎛️settle-and-dispatch-drivers/🦀️.rs");

include!("🖼️render-and-scene-probes/🦀️.rs");

include!("📏️measure-and-tool-run-probes/🦀️.rs");
