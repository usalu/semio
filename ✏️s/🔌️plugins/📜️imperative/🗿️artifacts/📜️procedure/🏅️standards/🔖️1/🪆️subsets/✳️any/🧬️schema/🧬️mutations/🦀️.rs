//! 📜️ Procedure parent mutation vocabulary — empty by design (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12,
//! §20.15): the program lives in the composed `flow` child (`s.stdio.semio@v1/flow`) and the seed in the `text` child, so
//! every content edit is a child-lane leaf in that child's store (`insert-node`, `set-node-param`, `insert-edge`, …) and the
//! parent owns no leaf that could read a child.

use crate::diff::ProcedureDiff;
use crate::ProcedureSnapshot;

//#region 🔖️Aggregate
/// 🕳️ The uninhabited parent vocabulary of a document whose whole content is its composed children.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum ProcedureMutation {}

impl protocol::Mutation<ProcedureSnapshot> for ProcedureMutation {
    type Diff = ProcedureDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match *self {}
    }
    fn diff(&self, _base: &ProcedureSnapshot) -> protocol::MutationOutcome<ProcedureDiff> {
        match *self {}
    }
    fn inverse(&self, _base: &ProcedureSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        match *self {}
    }
}

/// 🏷️ No parent kind exists, so no parent operation is ever labelled; child leaves label their own rows.
impl protocol::SemanticMutation<ProcedureSnapshot> for ProcedureMutation {
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
impl protocol::OpText for ProcedureMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "a procedure has no parent-lane mutation; content edits are child-lane leaves", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

/// 💾️ No parent operation record exists.
impl protocol::OpBinary for ProcedureMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "procedure-mutation", offset: 0, detail: "a procedure has no parent-lane mutation; content edits are child-lane leaves".into() })
    }
}

/// 🧊️ Nothing to retire: the vocabulary is uninhabited.
impl neural_engine::ColdRetire for ProcedureMutation {
    fn retire_cold(self) {
        match self {}
    }
}
//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
