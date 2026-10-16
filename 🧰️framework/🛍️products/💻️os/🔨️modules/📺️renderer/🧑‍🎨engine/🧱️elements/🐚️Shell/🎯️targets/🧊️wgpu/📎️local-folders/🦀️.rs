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

//#endregion 🔖️Store

//#region 🔖️Reconnect

//#endregion 🔖️Reconnect

//#region 🔖️HostRouteFolder

//#endregion 🔖️HostRouteFolder
#[cfg(test)]
#[path = "../../../🧪️tests/🧪️wgpu-local-folders/🦀️.rs"]
mod tests;

