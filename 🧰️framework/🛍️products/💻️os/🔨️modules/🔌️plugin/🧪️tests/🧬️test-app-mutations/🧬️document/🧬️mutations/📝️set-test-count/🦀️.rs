//#region 🔢️SetCount
use super::super::{TestDiff,TestMutation,TestSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use protocol::{MutationKind,MutationOutcome,SemanticDescriptor};

#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize,ToValue, FromValue, dsl::DslRecord,dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetCount { pub value:i32 }

impl MutationKind<TestSnapshot,TestMutation> for SetCount { const SEMANTICS:SemanticDescriptor=SemanticDescriptor{verb:"set",entity:"count",kind:"set-count",record:"SetCount"}; fn diff(&self,_:&TestSnapshot)->MutationOutcome<TestDiff>{MutationOutcome::new(TestDiff{count:Some(self.value),label:None})} fn inverse(&self,base:&TestSnapshot)->Vec<TestMutation>{vec![Self{value:base.count}.into()]} fn label(&self)->String{format!("Set count to {}",self.value)} }

#[cfg(test)]
mod tests { use super::*; use protocol::MutationLeaf; #[test] fn descriptor_has_set_count_identity(){assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.semantic_kind,"set-count");assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.text_opcode,Some("set-count"));assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.binary_tag,Some(0));} }
//#endregion 🔢️SetCount
