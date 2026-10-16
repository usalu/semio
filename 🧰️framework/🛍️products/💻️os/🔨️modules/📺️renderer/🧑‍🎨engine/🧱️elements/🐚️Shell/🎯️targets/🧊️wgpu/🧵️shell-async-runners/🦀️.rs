//! 🧵️ Detached work, pool futures, directory/inference runners and document retirement.
//!
//! `ShellDetached`, the pool future, the directory and inference runners, the retained Home projection and the
//! document-retirement registry that drain asynchronous work back into [`ShellState`].
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellTypes

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn close_unowned_directory_dial<T: DirectoryWsConnection>(result: Result<T, TransportError>) {
    if let Ok(mut connection) = result {
        connection.close();
    }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-shell-pool-future/🦀️.rs"]
mod shell_pool_future_tests;

//#region 📄️ShellDocumentRetirement
const SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES: usize = std::mem::size_of::<[u64; SHELL_DOCUMENT_RETIREMENT_WORDS]>() + std::mem::size_of::<u64>();
const _: [(); 72] = [(); SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ShellDocumentRetirementAdmission {
    pub(crate) index: usize,
    pub(crate) epoch: u64,
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-shell-document-retirement/🦀️.rs"]
mod shell_document_retirement_tests;
//#endregion 📄️ShellDocumentRetirement
//#endregion ShellTypes
