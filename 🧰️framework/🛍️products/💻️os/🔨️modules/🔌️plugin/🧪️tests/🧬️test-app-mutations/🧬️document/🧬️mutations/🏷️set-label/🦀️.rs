//#region 🏷️SetLabel
use super::super::{TestDiff,TestMutation,TestSnapshot};
use protocol::{MutationKind,MutationOutcome,SemanticDescriptor};

#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize,dsl::DslRecord,dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub(crate) struct SetLabel { pub value:String }

impl MutationKind<TestSnapshot,TestMutation> for SetLabel { const SEMANTICS:SemanticDescriptor=SemanticDescriptor{verb:"set",entity:"label",kind:"set-label",record:"SetLabel"}; fn diff(&self,_:&TestSnapshot)->MutationOutcome<TestDiff>{MutationOutcome::new(TestDiff{count:None,label:Some(self.value.clone())})} fn inverse(&self,base:&TestSnapshot)->Vec<TestMutation>{vec![Self{value:base.label.clone()}.into()]} fn label(&self)->String{format!("Set label to {}",self.value)} }

#[cfg(test)]
mod tests { use super::*; use protocol::MutationLeaf; #[test] fn descriptor_has_set_label_identity(){assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.semantic_kind,"set-label");assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.text_opcode,Some("set-label"));assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.binary_tag,Some(1));} }
//#endregion 🏷️SetLabel
