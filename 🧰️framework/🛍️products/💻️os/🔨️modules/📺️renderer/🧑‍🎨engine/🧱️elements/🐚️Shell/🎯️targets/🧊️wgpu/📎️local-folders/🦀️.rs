//! 📎️ The device's remembered folder bindings on the wgpu shell (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, W2-C
//! follow-up 5) — React's `🏛️ShellHost/📎️local-folders`.
//!
//! `os.config.local-folders` is an event-sourced config facet persisted local-only, in the shell's own preferences
//! (`semio.os.config` → `preferences["os.config.local-folders"]`, React's `OsShellConfig.setPreference`): never shared, never
//! in a URL. A folder attach of a program's document records `attachLocalFolder`, keyed by the document's own identity
//! (the id its store stamps on every envelope); a detach records `detachLocalFolder`. When a program boots holding a
//! document this device once attached, the native shell — which owns a folder transport and needs no gesture to open a
//! folder — reattaches it directly, once per document, and restores its archive like a fresh load; when that reattach
//! fails it offers the accessible "Reconnect folder" / "Forget folder" band. The browser build has no filesystem: like
//! React's worker it keeps a folder- or file-bound document's archive through the dev host's backbone route
//! (`GET|PUT /semio-backbone`, asked through the page-owned `semioWgpuHostIo` door because the shell runs in the frame
//! Worker) — read and restored on attach, written after every history change — and, like React's browser shell, always
//! offers the band and waits for the person's gesture.
//!
//! The event log, the reconnect offer, the folder name and the band's copy are pinned for both shells by
//! `🧑‍🎨engine/🧫️fixtures/📎️local-folder-bindings/🔣️.json`; the browser folder door by
//! `🧑‍🎨engine/🧫️fixtures/🧫️wgpu-backbone-folder-door/🔣️.json`.

use super::*;
use semio_framework_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};


//#region 🔖️Store
#[cfg(test)]
thread_local! {
    /// 🧪️ The preference slot a law's shells share, outliving each of them like the real store outlives a restart.
    pub(crate) static TEST_LOCAL_FOLDERS_LOG: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

//#endregion 🔖️Store

//#region 🔖️Reconnect

/// 🚪️ Whether this build reopens a remembered folder by itself: the native shell owns a folder transport; the browser
/// reaches folders only through the dev host's route and, like React's browser shell, waits for the person's gesture.
pub(crate) const LOCAL_FOLDER_DIRECT_REATTACH: bool = cfg!(not(target_arch = "wasm32"));

/// 🗣️ The band's copy — React's `ui.sync.reconnect.{label,message,attach,forget}` at the normal terminology.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LocalFolderText {
    Label,
    Message,
    Attach,
    Forget,
    Unidentified,
}

/// 🗣️ One line of the band (or of a reconnect refusal) in `locale`; `folder` fills the message.
pub(crate) fn local_folder_text(text: LocalFolderText, folder: &str, locale: Locale) -> String {
    match (text, locale) {
        (LocalFolderText::Label, Locale::En) => "Folder of this document".into(),
        (LocalFolderText::Label, Locale::De) => "Ordner dieses Dokuments".into(),
        (LocalFolderText::Message, Locale::En) => format!("This document was attached to the folder “{folder}”."),
        (LocalFolderText::Message, Locale::De) => format!("Dieses Dokument war mit dem Ordner „{folder}“ verbunden."),
        (LocalFolderText::Attach, Locale::En) => "Reconnect folder".into(),
        (LocalFolderText::Attach, Locale::De) => "Ordner wieder verbinden".into(),
        (LocalFolderText::Forget, Locale::En) => "Forget folder".into(),
        (LocalFolderText::Forget, Locale::De) => "Ordner vergessen".into(),
        (LocalFolderText::Unidentified, Locale::En) => "This program has no document to attach".into(),
        (LocalFolderText::Unidentified, Locale::De) => "Dieses Programm hat kein Dokument zum Verbinden".into(),
    }
}

/// 🎬️ The band's two verbs, dispatched on the shell's own sync controller.
pub(crate) const FOLDER_RECONNECT_ACTION: &str = "reconnectFolder";
pub(crate) const FOLDER_FORGET_ACTION: &str = "forgetFolder";

/// 🆔️ The notice code a folder attach of a program holding no document is told under.
pub(crate) const SYNC_DOCUMENT_UNIDENTIFIED_CODE: &str = "sync.attach.document-unidentified";

//#endregion 🔖️Reconnect

//#region 🔖️HostRouteFolder

#[cfg(test)]
thread_local! {
    /// 🧪️ The dev host a law's browser folder door reaches: stored archives by `(uri, documentId)`, and every request.
    pub(crate) static TEST_BACKBONE_FOLDER_HOST: std::cell::RefCell<(BTreeMap<(String, String), Vec<u8>>, Vec<Value>)> = const { std::cell::RefCell::new((BTreeMap::new(), Vec::new())) };
}

/// 🗃️ A document kept through the dev host's backbone route: its uri, identity and artifact schema, the digest of the
/// archive last written there and the history cursor it was written at.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HostRouteFolder {
    pub uri: String,
    pub document_id: String,
    pub schema: String,
    pub written: u64,
    pub written_cursor: u64,
}

/// 🚪️ Whether a sync-card uri names a document the browser keeps through the host route.
#[cfg(target_arch = "wasm32")]
pub(crate) fn is_host_route_uri(uri: &str) -> bool {
    uri.starts_with("folder://") || uri.starts_with("file://")
}
//#endregion 🔖️HostRouteFolder

#[cfg(test)]
#[path = "../../../🧪️tests/🧪️wgpu-local-folders/🦀️.rs"]
mod tests;
