//! 🔧️ DAG artifact — the operation enum + laws (constitutional: op).
//!
//! `DagOperation`, its `protocol::Operation`/`OperationDiff` impls, and the `apply`/`diff`/`backwards`
//! logic all live in the shared DAG kernel crate (`infinite_board_port_directed_dag`,
//! `framework/kernel/infinite/board/port/directed/dag/rs`, `🔖️DocumentVcs` region) alongside the
//! `DagDocument` projection they mutate — the DAG board is shared infrastructure used by more than this
//! play app, so none of it is this crate's to own. This module only re-exports the kernel's
//! `DagOperation` type under this crate's taxonomy node so sibling components (`⚙️engine`,
//! `🎮️commands/*`, app `🦀️component.rs`) depend on an app-owned name instead of reaching into the kernel
//! path directly.

//#region 🔖️Types
pub use infinite_board_port_directed_dag::DagOperation;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Operation;

    #[test]
    fn set_nodes_backwards_restores_the_pre_operation_nodes() {
        let document = infinite_board_port_directed_dag::default_dag_document();
        let operation = DagOperation::SetNodes { nodes: Vec::new() };
        let inverse = operation.backwards(&document);
        assert_eq!(inverse, vec![DagOperation::SetNodes { nodes: document.nodes }]);
    }
}
//#endregion 🧪️Tests
