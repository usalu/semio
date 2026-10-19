//! 🔌️ framework/products/os/modules/renderer/engine/elements/🌉️ProgramBridge/component.rs — wgpu
//! plugin-bridge implementation for the ProgramBridge element, extracted from lib.rs's inline
//! `pub mod program_bridge { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired
//! via `#[path = "../../../../🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod program_bridge;` in
//! lib.rs in place of the former inline block; the module name `program_bridge` is unchanged, so
//! every existing `crate::program_bridge::...` call site elsewhere in the crate keeps resolving with
//! zero other changes.
//! 🔌️ Plugin bridge for wasm C-ABI modules (browser JS loader + wasmtime host).

use semio_framework::kernel::Effect;
use semio_framework::{PluginManifest, ViewModel};
use std::collections::HashMap;
use ui_contract::UiDocumentLease;
#[cfg(not(target_arch = "wasm32"))]
use ui_contract::{SurfaceId, UI_DOCUMENT_LEASE_SLOTS, UI_DOCUMENT_NODES, UI_DOCUMENT_PATCH_OPS};
use semio_framework_ui_declaration::layout::{WindowEngagement, WindowMeasure};

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};
#[cfg(target_arch = "wasm32")]
use semio_framework_async::browser::JsFuture;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_kernel_pool::KernelClient;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_kernel_protocol::{ExchangeOutcome, MountedProductReplayAdmission};

pub use semio_framework_os_renderer_wgpu_contracts::ProgramFault;

#[path = "🧳️wasm-program-exchange/🦀️.rs"]
mod wasm_program_exchange_child;
#[cfg(not(target_arch="wasm32"))]
use wasm_program_exchange_child::wasm_program_exchange;

#[path = "🌉️program-bridge-entry/🦀️.rs"]
mod program_bridge_entry;
pub use program_bridge_entry::{
    tool_measures_from_section, window_engagements_from_section, window_measures_from_section,
    ProgramBridgeEntry, ProgramEphemeralSnapshot,
};
#[cfg(any(test,feature="testkit"))]
pub use program_bridge_entry::ProgramFixtureDocument;

#[path = "📡️js-calls-and-operation-publications/🦀️.rs"]
mod js_calls_and_operation_publications;
#[cfg(target_arch="wasm32")]
use js_calls_and_operation_publications::{
    browser_component_codec, browser_ephemeral, call_js, call_js_bytes, context_menu_js,
    create_app_js, describe_js_rejection, destroy_app_js, dispatch_invoke_extension_js,
    document_backbone_js, handle_action_js, handle_command_js, json_window_around,
    push_scoped_contributions_js, receive_document_backbone_js, take_operation_publications_js,
    BrowserRetainedDocument, BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES,
};
pub use js_calls_and_operation_publications::OperationPublication;
#[cfg(any(target_arch="wasm32",test))]
pub use js_calls_and_operation_publications::operation_publications_from_json;

#[path = "🧰️document-render-plugin-install-and-native-loading/🦀️.rs"]
mod document_render_plugin_install_and_native_loading;
#[cfg(target_arch="wasm32")]
use document_render_plugin_install_and_native_loading::{
    get_fn, invocation_pack_base64, render_with_document_js, view_state_pack_base64,
};
pub use document_render_plugin_install_and_native_loading::{
    filter_plugins, resolve_artifact_kind_activation_owner, retained_publication_refusal,
};
#[cfg(target_arch="wasm32")]
pub use document_render_plugin_install_and_native_loading::{
    cancel_js_plugin_install, install_js_extension, install_js_plugin,
    js_extension_install_door_available, js_plugin_install_door_available, parse_plugin_entries,
    retire_js_extension,
};
#[cfg(not(target_arch="wasm32"))]
pub use document_render_plugin_install_and_native_loading::{
    load_resolved_program, load_wasm_plugins,
};

#[cfg(test)]
#[path = "../../🧪️tests/📡️wgpu-document-backbone-effect/🦀️.rs"]
mod document_backbone_effect_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/📦️frame/🦀️.rs"]
mod frame_tests;

#[cfg(test)]
#[path = "../../🧪️tests/📏️wgpu-window-measures-section/🦀️.rs"]
pub mod window_measures_section_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🎬️wgpu-window-engagements-section/🦀️.rs"]
pub mod window_engagements_section_tests;

//#region 🏠️🧳️PluginHostConfig
// 🐛️ `generated_plugin_hosts` is declared at the crate root (below, outside this inline `program_bridge`
// module) and re-exported here — a `#[path]` file-module declared *inside* an inline `mod` block resolves
// relative to a virtual `<enclosing-file-dir>/program_bridge/` directory that has no real counterpart on
// disk, and POSIX path resolution requires every component a `../../../🌉️ProgramBridge/🎯️targets` traverses through (even one that's
// lexically cancelled out) to actually exist; no number of `../../../🌉️ProgramBridge/🎯️targets`s fixes that. Declaring it at the crate
// root instead (where `program_bridge/`'s directory is real) and re-exporting preserves the
// `crate::program_bridge::{PluginHostConfig, ...}` path every call site already depends on.
pub use crate::plugin_catalog::{is_space_mode, resolve_playground_app_id, resolve_plugin_host_config, resolve_registry_plugin_id, PluginHostConfig, plugin_artifact_kind_activations};

//#endregion 🏠️🧳️PluginHostConfig
