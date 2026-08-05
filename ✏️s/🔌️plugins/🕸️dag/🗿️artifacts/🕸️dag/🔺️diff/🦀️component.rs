//! 🔺️ DAG artifact — the operation diff (constitutional: diff).
//!
//! `OperationDiff<DagDocument> for DagDiff` and its `apply`/`absorb` logic are implemented directly in
//! the DAG kernel crate (`infinite_board_port_directed_dag`, `🔖️DocumentVcs` region) alongside
//! `DagDocument`/`DagOperation` themselves — see `crate::artifacts::dag::op`'s doc for why. This module
//! only re-exports the kernel's `DagDiff` type under this crate's taxonomy node so sibling components
//! depend on a stable app-owned path instead of reaching into the kernel path directly, mirroring
//! `dsl`/`pack`/`spr`'s equivalent re-export pattern.

//#region 🔖️Types
pub use infinite_board_port_directed_dag::DagDiff;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dag_diff_default_has_no_pending_writes() {
        let diff = DagDiff::default();
        assert_eq!(diff, DagDiff { document: None, nodes: None, edges: None, set_nodes: None, set_edges: None });
    }
}
//#endregion 🧪️Tests
