//! 🧰️ Shared fixture of the puzzle3d editor integration targets — the raw app, its runtime budgets, render and dispatch drivers, and the heap witness.

use super::*;
use semio_plugin::ActionMeta;
use semio_plugin::App;
use semio_plugin::EditorApp;
use semio_plugin::InvocationResult;
use semio_plugin::MAINTENANCE_STAGES;
use semio_plugin::PluginApp;
use semio_plugin::PluginLifecycleStep;
use semio_plugin::VcsArtifactApp;
use semio_plugin::ViewModel;
use semio_plugin::ViewWindowInstance;
use semio_plugin::artifact_app_laws;

include!("🏗️raw-app-fixture/🦀️.rs");

include!("🎛️settle-and-dispatch-drivers/🦀️.rs");

include!("🖼️render-and-scene-probes/🦀️.rs");

include!("📏️measure-and-tool-run-probes/🦀️.rs");
