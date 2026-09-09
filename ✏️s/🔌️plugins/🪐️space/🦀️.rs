//! 🔌️ Space composition of independently packaged Home and Space Index artifacts.

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};
use semio_framework_os::OS_SPACE_SCHEMA;
pub use semio_s_artifact_space_space::space_core::*;

//#region ⚙️Engine
/// 🕳️ `🏠️home` moved out into `✏️editor`/`👁️viewer` above (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET W2 packet P7). `🪐️space` (studio) has no artifact
/// of its own (`ArtifactApp::Snapshot`/`::Mutation` are the framework-owned `WorkflowSnapshot`/
/// `WorkflowMutation`, a deliberately OS-owned "peer kernel crate" document per ticket
/// 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT's `w4b-workflow.md`/`w4b-space.md` — not a
/// per-subset editor/viewer surface, W2-END packet). Relocated out of the retired `🎛️apps/` taxonomy
/// dir into this plugin-root `⚙️engine/` facet (mirroring `🏗️fem`'s own plugin-root `⚙️engine/
/// 🖥️app-surface/` precedent from packet P7b) — same content, same `.document_app()`/
/// `.foreign_document_codec()` registration, module path only.
#[path = "."]
pub mod engine {
    #[path = "."]
    pub mod space {
        #[path = "⚙️engine/🪐️space/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "⚙️engine/🪐️space/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "⚙️engine/🪐️space/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "⚙️engine/🪐️space/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "⚙️engine/🪐️space/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "⚙️engine/🪐️space/⚙️engine/🦀️.rs"]
        pub mod engine;
        #[path = "⚙️engine/🪐️space/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "⚙️engine/🪐️space/🎮️commands/➕️add-parameter/🦀️.rs"]
            pub mod add_parameter;
            #[path = "⚙️engine/🪐️space/🎮️commands/🔗️bind-parameter-field/🦀️.rs"]
            pub mod bind_parameter_field;
            #[path = "⚙️engine/🪐️space/🎮️commands/❎️close-focused-instance/🦀️.rs"]
            pub mod close_focused_instance;
            #[path = "⚙️engine/🪐️space/🎮️commands/⌨️compiled-dag-engagement-input/🦀️.rs"]
            pub mod compiled_dag_engagement_input;
            #[path = "⚙️engine/🪐️space/🎮️commands/📨️compiled-dag-engagement-submit/🦀️.rs"]
            pub mod compiled_dag_engagement_submit;
            #[path = "⚙️engine/🪐️space/🎮️commands/🔌️connect-media-ports/🦀️.rs"]
            pub mod connect_media_ports;
            #[path = "⚙️engine/🪐️space/🎮️commands/📋️copy-app-instance/🦀️.rs"]
            pub mod copy_app_instance;
            #[path = "⚙️engine/🪐️space/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "⚙️engine/🪐️space/🎮️commands/✂️disconnect-media-edge/🦀️.rs"]
            pub mod disconnect_media_edge;
            #[path = "⚙️engine/🪐️space/🎮️commands/👯️duplicate-app-instance/🦀️.rs"]
            pub mod duplicate_app_instance;
            #[path = "⚙️engine/🪐️space/🎮️commands/📤️export-media/🦀️.rs"]
            pub mod export_media;
            #[path = "⚙️engine/🪐️space/🎮️commands/📜️export-studio-dsl/🦀️.rs"]
            pub mod export_studio_dsl;
            #[path = "⚙️engine/🪐️space/🎮️commands/📦️export-studio-pack/🦀️.rs"]
            pub mod export_studio_pack;
            #[path = "⚙️engine/🪐️space/🎮️commands/🧭️go-home/🦀️.rs"]
            pub mod go_home;
            #[path = "⚙️engine/🪐️space/🎮️commands/🖼️import-media/🦀️.rs"]
            pub mod import_media;
            #[path = "⚙️engine/🪐️space/🎮️commands/🧾️import-media-payload/🦀️.rs"]
            pub mod import_media_payload;
            #[path = "⚙️engine/🪐️space/🎮️commands/📥️import-space-pack/🦀️.rs"]
            pub mod import_space_pack;
            #[path = "⚙️engine/🪐️space/🎮️commands/🧳️import-space-pack-payload/🦀️.rs"]
            pub mod import_space_pack_payload;
            #[path = "⚙️engine/🪐️space/🎮️commands/🚚️move-media-node/🦀️.rs"]
            pub mod move_media_node;
            #[path = "⚙️engine/🪐️space/🎮️commands/🗺️navigate-virtual-file-system-node/🦀️.rs"]
            pub mod navigate_virtual_file_system_node;
            #[path = "⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "⚙️engine/🪐️space/🎮️commands/🖱️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "⚙️engine/🪐️space/🎮️commands/🔍️open-instance/🦀️.rs"]
            pub mod open_instance;
            #[path = "⚙️engine/🪐️space/🎮️commands/🚪️open-space/🦀️.rs"]
            pub mod open_space;
            #[path = "⚙️engine/🪐️space/🎮️commands/📌️paste-app-instance/🦀️.rs"]
            pub mod paste_app_instance;
            #[path = "⚙️engine/🪐️space/🎮️commands/🩺️patch-app-instances/🦀️.rs"]
            pub mod patch_app_instances;
            #[path = "⚙️engine/🪐️space/🎮️commands/🔧️patch-media-nodes/🦀️.rs"]
            pub mod patch_media_nodes;
            #[path = "⚙️engine/🪐️space/🎮️commands/🩹️patch-parameter/🦀️.rs"]
            pub mod patch_parameter;
            #[path = "⚙️engine/🪐️space/🎮️commands/👥️presence-heartbeat/🦀️.rs"]
            pub mod presence_heartbeat;
            #[path = "⚙️engine/🪐️space/🎮️commands/🚮️remove-app-instance/🦀️.rs"]
            pub mod remove_app_instance;
            #[path = "⚙️engine/🪐️space/🎮️commands/➖️remove-parameter/🦀️.rs"]
            pub mod remove_parameter;
            #[path = "⚙️engine/🪐️space/🎮️commands/🏷️rename-app-instance/🦀️.rs"]
            pub mod rename_app_instance;
            #[path = "⚙️engine/🪐️space/🎮️commands/🗂️reorganize-workflow/🦀️.rs"]
            pub mod reorganize_workflow;
            #[path = "⚙️engine/🪐️space/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "⚙️engine/🪐️space/🎮️commands/⚙️set-active-panel-tab/🦀️.rs"]
            pub mod set_active_panel_tab;
            #[path = "⚙️engine/🪐️space/🎮️commands/📇️set-app-registrations/🦀️.rs"]
            pub mod set_app_registrations;
            #[path = "⚙️engine/🪐️space/🎮️commands/🚀️spawn-app/🦀️.rs"]
            pub mod spawn_app;
            #[path = "⚙️engine/🪐️space/🎮️commands/🔓️unbind-parameter-field/🦀️.rs"]
            pub mod unbind_parameter_field;
            #[path = "⚙️engine/🪐️space/🎮️commands/💬️workflow-engagement-input/🦀️.rs"]
            pub mod workflow_engagement_input;
            #[path = "⚙️engine/🪐️space/🎮️commands/✅️workflow-engagement-submit/🦀️.rs"]
            pub mod workflow_engagement_submit;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                #[path = "⚙️engine/🪐️space/🎭️modes/🌐️main/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod workflow {
                        #[path = "⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🎚️options/🎯️active-instance/🦀️.rs"]
                            pub mod active_instance;
                        }
                    }

                    #[path = "."]
                    pub mod media_vfs {
                        #[path = "⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🗂️media-vfs/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }

                    #[path = "."]
                    pub mod compiled_dag {
                        #[path = "⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "⚙️engine/🪐️space/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "⚙️engine/🪐️space/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "⚙️engine/🪐️space/📌️panels/🔢️parameters/🦀️.rs"]
            pub mod parameters;
        }
    }
}
//#endregion ⚙️Engine

//#region 🔌️Registration
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the home, space-index, and studio surfaces.
    pub enum SpaceApps: PluginApp {
        HomeEditor(VcsArtifactApp<EditorApp<semio_s_artifact_space_home::editor::home::HomeApp>>),
        HomeViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_space_home::viewer::home::HomeViewer>>),
        SpaceIndexEditor(VcsArtifactApp<EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>>),
        SpaceIndexViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer>>),
        Studio(VcsArtifactApp<engine::space::SpaceApp>),
    }
}

/// 🔌️ Builds the S Studio plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) builds all artifact, app-schema, and codec
/// contributions as immutable data before the aggregate registration commit. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
/// M6-remaining, `📓️design-abi.md` §3/§6) are this crate's migration proof: one `OnArtifactKind`
/// event per owned kind (`home`/`space`, read live from each's own `artifact_kind().id`), `Isolated`
/// execution (grepped for `.handler(…)`, a `🧩️extensions/` dir, and self-tick loops — none found,
/// despite this crate having the heaviest `Effect` usage in the repo), and one `documents.write` ask
/// covering both editors' persisted mutations. No quota declared — no measured need found.
// 🚫️async: `plugin_exports!`'s `__semio_install_plugin_bundle` calls this from a bare `fn()`
// pointer slot (`PLUGIN_BUNDLE_INSTALLER: OnceLock<fn()>`), which can never itself be `async fn` —
// so this fn stays sync and bridges its still-async `home`/`engine` sub-calls with `resolve_ready`
// (in scope via `plugin_app_close_prelude::*` above), same poll-once guarantee as every other E5
// executor-bridge call site in this crate; the `space` artifact/editor/viewer calls below are
// already sync (no bridge needed).
pub fn plugin() -> Result<Plugin<SpaceApps>, PluginAssemblyError> {
    Plugin::<SpaceApps>::builder("space")
        .label("S Studio")
        .version("0.1.0")
        .package_id("semio:space")
        .local_backbone_storage()
        .artifact(resolve_ready(semio_s_artifact_space_home::declaration()).map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_space_home::editor::home::HomeApp>(resolve_ready(semio_s_artifact_space_home::editor::home::create_home_app()))
        .editor_mutation_roster::<semio_s_artifact_space_home::editor::home::HomeApp>()
        .viewer::<semio_s_artifact_space_home::viewer::home::HomeViewer>(resolve_ready(semio_s_artifact_space_home::viewer::home::create_home_viewer()))
        .viewer_mutation_roster::<semio_s_artifact_space_home::viewer::home::HomeViewer>()
        .artifact(semio_s_artifact_space_space::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>(semio_s_artifact_space_space::editor::space_index::create_space_index_editor())
        .editor_mutation_roster::<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>()
        .viewer::<semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer>(semio_s_artifact_space_space::viewer::space_index::create_space_index_viewer())
        .viewer_mutation_roster::<semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer>()
        .document_app::<engine::space::SpaceApp>(resolve_ready(engine::space::create_space_app()))
        .foreign_document_codec::<engine::space::SpaceApp>(OS_SPACE_SCHEMA)
        .activation(ActivationEvent::OnArtifactKind { kind: resolve_ready(semio_s_artifact_space_home::artifact_kind()).id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_space_space::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist home/space-index edits to the open document".into(), optional: false })
        .try_build()
}
//#endregion 🔌️Registration

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

//#region 🧪️SpaceIndexProjectionTests
#[cfg(test)]
#[path = "🧪️tests/🔬️space-index-projection/🦀️.rs"]
mod space_index_projection_tests;
//#endregion 🧪️SpaceIndexProjectionTests

//#region 🧪️InteractiveJobCatalogTests
#[cfg(test)]
#[path = "🧪️tests/🔬️interactive-job-catalog/🦀️.rs"]
mod interactive_job_catalog_tests;
//#endregion 🧪️InteractiveJobCatalogTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, SpaceApps);
