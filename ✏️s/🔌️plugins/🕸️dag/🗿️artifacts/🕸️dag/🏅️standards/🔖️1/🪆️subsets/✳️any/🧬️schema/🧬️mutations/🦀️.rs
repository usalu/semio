//! 🕸️ DAG parent mutation vocabulary — empty by design (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §20.15):
//! a DAG's nodes and edges live in its composed `content` child (`s.stdio.semio@v1/graph`), so every content edit is a
//! child-lane leaf in that child's store (`create-node`, `drag-nodes`, `set-node-property`, `create-edge`, …) and the parent
//! owns no leaf that could read the child.

use crate::diff::DagDiff;
use crate::DagSnapshot;

//#region 🔖️Store
pub type DagEnvelope = store::ArtifactEnvelope<DagSnapshot, DagMutation>;
pub type DagStore = store::ArtifactStore<DagSnapshot, DagMutation>;
//#endregion 🔖️Store

//#region 🔖️Aggregate
/// 🕳️ The uninhabited parent vocabulary of a document whose whole content is its composed child.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum DagMutation {}

impl protocol::Mutation<DagSnapshot> for DagMutation {
    type Diff = DagDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match *self {}
    }
    fn diff(&self, _base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        match *self {}
    }
    fn inverse(&self, _base: &DagSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        match *self {}
    }
}

/// 🏷️ No parent kind exists, so no parent operation is ever labelled; child leaves label their own rows.
impl protocol::SemanticMutation<DagSnapshot> for DagMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        &[]
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        match *self {}
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match *self {}
    }
    fn target(&self) -> Vec<String> {
        match *self {}
    }
}

/// 📝️ No parent operation line exists.
impl protocol::OpText for DagMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "a DAG has no parent-lane mutation; content edits are child-lane leaves", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

/// 💾️ No parent operation record exists.
impl protocol::OpBinary for DagMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "dag-mutation", offset: 0, detail: "a DAG has no parent-lane mutation; content edits are child-lane leaves".into() })
    }
}
//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
