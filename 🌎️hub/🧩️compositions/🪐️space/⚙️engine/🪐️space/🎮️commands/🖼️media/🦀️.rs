//! 🧱️ Studio media and session commands: media graph edits, media and pack import and export, engagement and panel state.

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
use semio_s_space_core::*;

#[path = "../⌨️compiled-dag-engagement-input/🦀️.rs"]
pub mod compiled_dag_engagement_input;
#[path = "../📨️compiled-dag-engagement-submit/🦀️.rs"]
pub mod compiled_dag_engagement_submit;
#[path = "../🔌️connect-media-ports/🦀️.rs"]
pub mod connect_media_ports;
#[path = "../✂️disconnect-media-edge/🦀️.rs"]
pub mod disconnect_media_edge;
#[path = "../📤️export-media/🦀️.rs"]
pub mod export_media;
#[path = "../📜️export-studio-dsl/🦀️.rs"]
pub mod export_studio_dsl;
#[path = "../📦️export-studio-pack/🦀️.rs"]
pub mod export_studio_pack;
#[path = "../🧭️go-home/🦀️.rs"]
pub mod go_home;
#[path = "../🖼️import-media/🦀️.rs"]
pub mod import_media;
#[path = "../🧾️import-media-payload/🦀️.rs"]
pub mod import_media_payload;
#[path = "../📥️import-space-pack/🦀️.rs"]
pub mod import_space_pack;
#[path = "../🧳️import-space-pack-payload/🦀️.rs"]
pub mod import_space_pack_payload;
#[path = "../🗺️navigate-virtual-file-system-node/🦀️.rs"]
pub mod navigate_virtual_file_system_node;
#[path = "../🚪️open-space/🦀️.rs"]
pub mod open_space;
#[path = "../🔧️patch-media-nodes/🦀️.rs"]
pub mod patch_media_nodes;
#[path = "../👥️presence-heartbeat/🦀️.rs"]
pub mod presence_heartbeat;
#[path = "../🎬️set-active-example/🦀️.rs"]
pub mod set_active_example;
#[path = "../⚙️set-active-panel-tab/🦀️.rs"]
pub mod set_active_panel_tab;
#[path = "../📇️set-app-registrations/🦀️.rs"]
pub mod set_app_registrations;
#[path = "../💬️workflow-engagement-input/🦀️.rs"]
pub mod workflow_engagement_input;
#[path = "../✅️workflow-engagement-submit/🦀️.rs"]
pub mod workflow_engagement_submit;
