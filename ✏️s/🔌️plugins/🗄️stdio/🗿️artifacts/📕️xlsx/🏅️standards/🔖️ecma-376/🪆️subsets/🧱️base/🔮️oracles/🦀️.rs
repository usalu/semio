//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `calamine` (reader) + `rust_xlsxwriter` (writer) reference pairing so the subject's
//! own mutation has an independent result to be compared against instead of being checked against
//! its own reading.
//!
//! **The constraint that shapes this module** — no single crate both reads and modifies an XLSX.
//! `calamine` 0.36 parses a workbook into resolved cell values but exposes no accessor for the raw
//! shared-string table (`Xlsx<RS>::strings: Vec<String>` and `read_shared_strings` are both private
//! — confirmed by reading `calamine-0.36.1/src/xlsx/mod.rs`); it collapses `t="s"` shared-string
//! references and `t="inlineStr"` literal text into the same resolved `Data::String`. `rust_xlsxwriter`
//! 0.96 can only assemble a brand-new package — never open and patch an existing one — and its own
//! shared-string table (`shared_strings_table.rs`) is populated ONLY as a byproduct of `write_string`
//! on a cell; there is no API to insert, remove or target a pool entry independent of a cell write.
//! Concretely: sheet/cell mutations (`InsertSheet`, `RemoveSheet`, `RenameSheet`, `SetCell`,
//! `RemoveCell`) round-trip through "read the whole workbook into a
//! grid, apply the change to the grid, rebuild the whole workbook from the grid" — a genuine second
//! producer, hence `@mode-differential`. `InsertSharedString`/`RemoveSharedString`/`SetSharedString`
//! address the shared-string pool by an INDEX that is independent of any cell reference — exactly the
//! axis neither reference crate exposes — so this module cannot independently perform them; it
//! reports that honestly (see `oracle_apply_mutation`'s dispatch below) rather than faking a
//! differential result, per the fleet brief's §6.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

#[path = "🔢️grid-and-cell-addressing/🦀️.rs"]
mod grid_and_cell_addressing;
use grid_and_cell_addressing::*;

#[path = "🧮️mutation-dispatch/🦀️.rs"]
mod mutation_dispatch;
pub use mutation_dispatch::*;

#[path = "🧵️shared-string-pool/🦀️.rs"]
mod shared_string_pool;
pub use shared_string_pool::*;

#[path = "🔌️plumbing-and-projection/🦀️.rs"]
mod plumbing_and_projection;
pub use plumbing_and_projection::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
