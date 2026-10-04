//#region 🧬️ChildrenTestMutation
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
pub(super) enum ChildrenTestMutation {}

impl protocol::Mutation<ChildrenTestSnapshot> for ChildrenTestMutation {
    type Diff = ChildrenTestDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { match *self {} }
    fn diff(&self, _snapshot: &ChildrenTestSnapshot) -> protocol::MutationOutcome<ChildrenTestDiff> {
        match *self {}
    }
    fn inverse(&self, _snapshot: &ChildrenTestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        match *self {}
    
    })())
}
}
//#endregion 🧬️ChildrenTestMutation

//#region 📡️EmptyChildrenCodecs
impl protocol::OpText for ChildrenTestMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> { Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "children test mutations do not exist", semio_framework_diagnostic::TextSpan::at(1, 1))) }
    fn print_op(&self) -> String { match *self {} }
}
impl protocol::OpBinary for ChildrenTestMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { match *self {} }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { Err(protocol::ProtocolError::Malformed { what: "children-test-mutation", offset: 0, detail: "children test mutations do not exist".into() }) }
}
//#endregion 📡️EmptyChildrenCodecs

//#region 🧪️ChildrenMutationTests
#[cfg(test)]
mod tests {
    include!("../🧒️children-fixture-unit/🦀️.rs");
}
//#endregion 🧪️ChildrenMutationTests
