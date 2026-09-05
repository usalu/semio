//#region 🧬️TestDocumentMutationAggregate
#[path="📝️set-test-count/🦀️.rs"] pub mod set_count;
#[path="🏷️set-label/🦀️.rs"] pub mod set_label;
pub(crate) use set_count::SetCount;
pub(crate) use set_label::SetLabel;

#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize,semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslOps,dsl::Mutations)]
#[serde(tag="operation",content="payload",rename_all="camelCase",deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot=super::TestSnapshot,diff=super::TestDiff,schema="plugin.testkit.document")]
pub(crate) enum TestMutation { SetCount(SetCount), SetLabel(SetLabel) }

impl protocol::OpText for TestMutation {
    fn parse_op(line:&str)->Result<Self,crate::store::TextError>{
        for (keyword,spec) in <Self as dsl::DslVariants>::variants() {
            let prefix=format!("{keyword} ");
            if line==keyword||line.starts_with(&prefix) {
                let body=line[keyword.len()..].trim_start();
                return <Self as dsl::DslVariants>::from_named_record(&keyword,&dsl::parse(body,&spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Inline})?);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self)->String{let(keyword,record)=<Self as dsl::DslVariants>::to_named_record(self);let spec=<Self as dsl::DslVariants>::variants().into_iter().find(|(candidate,_)|candidate==&keyword).expect("declared variant").1;let body=dsl::print(&record,&spec(),dsl::JoinMode::Inline);if body.is_empty(){keyword}else{format!("{keyword} {body}")}}
}

impl protocol::OpBinary for TestMutation { fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{dsl::variants_binary::encode_op(self)} fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{dsl::variants_binary::decode_op(bytes)} }

#[cfg(test)]
mod tests { use super::*; use super::super::TestSnapshot; use protocol::{Mutation,MutationDiff,OpBinary,OpText}; #[test] fn direct_leaves_preserve_generic_document_codecs_and_laws(){let base=TestSnapshot{count:3,label:"before".into()};for mutation in [TestMutation::SetCount(SetCount{value:-4}),TestMutation::SetLabel(SetLabel{value:"quote \" newline\n☃".into()})]{assert_eq!(TestMutation::parse_op(&mutation.print_op()).unwrap(),mutation);assert_eq!(TestMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(),mutation);let after=mutation.diff(&base).diff().apply(&base).unwrap();let inverse=mutation.inverse(&base);assert_eq!(inverse.len(),1);assert_eq!(inverse[0].diff(&after).diff().apply(&after).unwrap(),base);}} }
//#endregion 🧬️TestDocumentMutationAggregate
