//! 🎬️ The fixture documents an empty mutation vocabulary.
use super::{Snapshot,Diff};
use semio_framework_os_kernel::{os_spr as protocol,os_store as store};
use semio_framework_value_derive::{ToValue,FromValue};
#[derive(Clone,Debug,PartialEq,ToValue,FromValue)]
pub enum Mutation {}
impl protocol::Mutation<Snapshot> for Mutation {
    type Diff=Diff;
    const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[];
    fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{match *self{}}
    fn diff(&self,_base:&Snapshot)->protocol::MutationOutcome<Diff>{match *self{}}
    fn inverse(&self,_base:&Snapshot)-> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {match *self{}
    })())
}
}
/// 🕳️ An empty vocabulary has no kinds, so no operation of it is ever labelled.
impl semio_framework_os_kernel::SemanticMutation<Snapshot> for Mutation {
    fn kinds()->&'static[semio_framework_os_kernel::SemanticDescriptor]{&[]}
    fn semantics(&self)->&'static semio_framework_os_kernel::SemanticDescriptor{match *self{}}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel{match *self{}}
    fn target(&self)->Vec<String>{match *self{}}
}
impl protocol::OpText for Mutation {
    fn parse_op(_line:&str)->Result<Self,store::TextError>{Err(store::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "snapshot-refusal has no mutations",store::TextSpan::at(1,1)))}
    fn print_op(&self)->String{match *self{}}
}
impl protocol::OpBinary for Mutation {
    fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{match *self{}}
    fn decode_op(_bytes:&[u8])->Result<Self,protocol::ProtocolError>{Err(protocol::ProtocolError::Malformed{what:"snapshot-refusal-mutation",offset:0,detail:"snapshot-refusal has no mutations".into()})}
}
