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
thread_local! {
    /// 🌐️ Browser boot environment supplied by `semioWgpuSetHubEnv` from the TypeScript host.
    pub(super) static BOOT_HUB_ENV: std::cell::RefCell<Option<(String, String, Option<String>)>> = std::cell::RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetHubEnv)]
pub fn semio_wgpu_set_hub_env(hub_url: String, user: String, data_dir: String) {
    let data_dir = if data_dir.is_empty() { None } else { Some(data_dir) };
    BOOT_HUB_ENV.with(|cell| *cell.borrow_mut() = Some((hub_url, user, data_dir)));
}

//#region 📣️ReplayRefusal

/// 🗣️ One reason's notice text per supported locale.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReplayRefusalLabelV1 {
    pub en: String,
    pub de: String,
}

/// 🧪️ One shared vector both shells' laws answer: a reason in a locale reads `text` under `code`.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReplayRefusalVectorV1 {
    pub reason: ReplayRefusalReasonV1,
    pub locale: String,
    pub text: String,
    pub code: String,
}

/// 📜️ The shared vocabulary file.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ReplayRefusalVocabularyV1 {
    pub schema: String,
    pub description: String,
    pub code_prefix: String,
    pub reasons: BTreeMap<String, ReplayRefusalLabelV1>,
    pub vectors: Vec<ReplayRefusalVectorV1>,
}

/// 📜️ The shared replay-refusal vocabulary, parsed once from the file both shells read.
pub(crate) static REPLAY_REFUSAL_V1: std::sync::LazyLock<ReplayRefusalVocabularyV1> =
    std::sync::LazyLock::new(|| serde_json::from_str(include_str!("../../../../🏛️ShellHost/📣️replay-refusal/🔣️.json")).expect("the shared replay-refusal vocabulary parses"));

//#endregion 📣️ReplayRefusal

//#endregion 🔖️IdentityPure



//#region 🧪️IdentityDirectoryPresenceTests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"]
mod identity_directory_presence_tests;
//#endregion 🧪️IdentityDirectoryPresenceTests

//#region ShellTypes


//#region 🔖️ChromeThreadBoundary

#[cfg(all(test, not(target_arch = "wasm32")))]
/// 🧵️ Compile-time proof that chrome construction state can move to a worker.
pub(crate) fn assert_shell_chrome_build_state_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<ShellChromeBuildState>();
}
//#endregion 🔖️ChromeThreadBoundary
//#endregion ShellTypes
