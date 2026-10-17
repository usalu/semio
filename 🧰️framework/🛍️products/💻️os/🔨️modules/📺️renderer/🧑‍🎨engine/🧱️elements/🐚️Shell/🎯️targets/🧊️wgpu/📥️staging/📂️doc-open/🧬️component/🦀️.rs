//! 📂️ `shell-doc-open` component structs: `OpeningState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 📂️ `OpeningState`: URI history, pending opens, import transfers, document services and the document opening pipeline.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `opening`.
pub struct OpeningState {
    pub uri_history: Vec<String>,
    pub uri_index: usize,
    pub open_space_id: Option<String>,
    pub pending_shell_uri_apply: bool,
    /// 📤️ File-open requests the guest asked for, parked on the SAME flush for the same reason.
    #[cfg(target_arch = "wasm32")]
    pub pending_file_opens: Vec<PendingFileOpen>,
    /// 📥️ Picked-file imports moving into the session's program chunk by chunk, oldest first ([`ImportTransfer`]).
    pub import_transfers: VecDeque<ImportTransfer>,
    pub import_transfer_next: u64,
    pub last_envelope_dsl: Option<String>,
    #[cfg(not(target_arch = "wasm32"))]
    pub document_services: Vec<InstalledServiceContributionV1>,
    /// 🚪️ The ONE document open in flight or settled, advanced by the frame pump
    /// ([`Self::advance_document_opening`]) and painted as its own band with a cancel control.
    pub document_opening: Option<ShellDocumentOpening>,
    pub icon_export: Option<IconExportBatch>,
    /// 🪪️ The verified execution-target lease fields for the open document. Native document opening
    /// retains only a canonical surface-id preference today — `document_socket_surface_from_descriptor`
    /// was deliberately downgraded from a forgeable partial authority by the execution-target-lease
    /// lane — so nothing native fills this in yet and every port refuses with a localized terminal. Both builds retain
    /// the lease a hub document's open admitted and drop it on detach; only the native inference port reads it.
    pub document_execution_target_lease: Option<semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1>,
}
