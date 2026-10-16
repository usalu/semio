//! 🔖️ Identity, replay refusal, check-in and chrome type vocabulary.
//!
//! The pure half of the wgpu shell: framework tab/panel ids, find items, context-menu helpers, directory and hub
//! bootstrap helpers, document open/install phases, replay refusal, conflict and check-in laws, and the
//! chrome-thread build/present state types every other shell chunk builds on.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

pub use semio_framework_os_renderer_wgpu_contracts::{try_push_find_item, ActiveShellFindItems, ShellFindItem, ShellFindItemBinding, ShellFindItems, SHELL_FIND_ITEM_CAPACITY, SHELL_FIND_PAYLOAD_BYTES};

//#region 🔖️IdentityPure

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetHubEnv)]
pub fn semio_wgpu_set_hub_env(hub_url: String, user: String, data_dir: String) {
    let data_dir = if data_dir.is_empty() { None } else { Some(data_dir) };
    BOOT_HUB_ENV.with(|cell| *cell.borrow_mut() = Some((hub_url, user, data_dir)));
}

//#region 📣️ReplayRefusal

//#endregion 📣️ReplayRefusal

//#endregion 🔖️IdentityPure



//#region 🧪️IdentityDirectoryPresenceTests
//#endregion 🧪️IdentityDirectoryPresenceTests

//#region ShellTypes


//#region 🔖️ChromeThreadBoundary

//#endregion 🔖️ChromeThreadBoundary
//#endregion ShellTypes
