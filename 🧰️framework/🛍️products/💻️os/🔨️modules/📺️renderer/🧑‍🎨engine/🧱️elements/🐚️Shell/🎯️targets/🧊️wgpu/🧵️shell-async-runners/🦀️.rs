//! 🧵️ Detached work, pool futures, directory/inference runners and document retirement.
//!
//! `ShellDetached`, the pool future, the directory and inference runners, the retained Home projection and the
//! document-retirement registry that drain asynchronous work back into [`ShellState`].
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellTypes



//#region 📄️ShellDocumentRetirement
const _: [(); 72] = [(); SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES];

//#endregion 📄️ShellDocumentRetirement
//#endregion ShellTypes
