//! 🧰️ Shared fixture of the puzzle3d editor integration targets — the raw app, its runtime budgets, render and dispatch drivers, and the heap witness.

use super::*;
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, MAINTENANCE_STAGES, PluginApp, PluginLifecycleStep, VcsArtifactApp, ViewModel, ViewWindowInstance, artifact_app_laws};

include!("🏗️raw-app-fixture/🦀️.rs");

include!("🎛️settle-and-dispatch-drivers/🦀️.rs");

include!("🖼️render-and-scene-probes/🦀️.rs");

include!("📏️measure-and-tool-run-probes/🦀️.rs");
