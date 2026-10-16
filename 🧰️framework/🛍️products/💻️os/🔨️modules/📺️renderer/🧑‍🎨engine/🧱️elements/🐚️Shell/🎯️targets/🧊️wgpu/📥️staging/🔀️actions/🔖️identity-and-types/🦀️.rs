//! 🔀️ `shell-actions` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🔖️IdentityPure

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: true, hub: true };

#[cfg(target_arch = "wasm32")]
pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: false, hub: true };

/// 🛂️ Refuses, before any actor exists, a binding the host would accept and then never serve: an
/// unserved folder or hub binding would hand the plugin a document that silently persists nowhere.
/// An empty binding set is the declared `EphemeralLocalOnly` class and every host serves it.
pub(crate) fn document_bindings_admitted(transports: ShellDocumentTransports, bindings: &[PersistenceBinding]) -> Result<(), &'static str> {
    for binding in bindings {
        match binding {
            PersistenceBinding::Hub { .. } if !transports.hub => return Err("document-binding.hub-transport-unavailable"),
            PersistenceBinding::Folder { .. } if !transports.folder => return Err("document-binding.folder-unavailable"),
            _ => {}
        }
    }
    Ok(())
}

/// 🌐️ The wgpu twin of the React shell's `parseRemoteBackboneUri` (`💻️os/🟦️.ts`): everything up to
/// the first slash is the host, up to the second the space, the rest the document; a uri missing a
/// part names no hub document.
pub(crate) fn parse_remote_backbone_uri(uri: &str) -> Option<RemoteBackboneUri> {
    let rest = uri.strip_prefix("remote://")?;
    let first = rest.find('/').filter(|index| *index > 0)?;
    let second = rest[first + 1..].find('/').map(|index| index + first + 1).filter(|index| *index > 0)?;
    Some(RemoteBackboneUri { host_port: rest[..first].to_string(), space_id: rest[first + 1..second].to_string(), document_id: rest[second + 1..].to_string() })
}

//#endregion 🔖️IdentityPure
//#region 🔖️CheckInPure

/// 🧾️ ticket §C5 — uncommitted-since-last-checkpoint count: the SAME "since the last Change" fold the
/// React shell's `uncommittedEditCount` (`ShellHost/🟦️.tsx`) uses — every applied
/// mutation-kind entry counts, reset to 0 the moment a `commitCheckpoint` history-kind entry is seen,
/// walked oldest-first.
pub(crate) fn uncommitted_edit_count(entries: &BTreeMap<String, semio_framework::kernel::HistoryEntry>) -> u32 {
    let mut pending = 0u32;
    for entry in history_rows_oldest_first(entries) {
        if entry.kind == "history" && entry.action_id == "commitCheckpoint" {
            pending = 0;
            continue;
        }
        if entry.kind == "mutation" && entry.applied {
            pending += 1;
        }
    }
    pending
}

/// 📌️ ticket §C5 item 4 — the pure gate `checkpoint_before_detach` evaluates: checkpoint-on-close
/// only when the session can check in at all (item 5's viewer guard), a document is actually attached
/// (nothing to check in otherwise), and there really is something uncommitted.
pub(crate) fn should_checkpoint_before_detach(role: semio_framework::manifest::AppRole, has_attached_document: bool, uncommitted_count: u32) -> bool {
    can_check_in(role) && has_attached_document && uncommitted_count > 0
}

//#endregion 🔖️CheckInPure

