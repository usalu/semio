#[path = "📝️set-surface-count/🦀️.rs"] pub mod set_surface_count;
pub(crate) use set_surface_count::SetSurfaceCount;
#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize,dsl::Mutations)] #[serde(tag="operation",content="payload",rename_all="camelCase",deny_unknown_fields)] #[mutations(snapshot=super::SurfaceSnapshot,diff=super::SurfaceDiff,schema="plugin.testkit.surface")] pub(crate) enum SurfaceMutation { SetSurfaceCount(SetSurfaceCount) }
impl protocol::OpText for SurfaceMutation { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(SetSurfaceCount::parse_op(line)?.into())} fn print_op(&self)->String{match self{Self::SetSurfaceCount(value)=>value.print_op()}} }
impl protocol::OpBinary for SurfaceMutation { fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{match self{Self::SetSurfaceCount(value)=>value.encode_op()}} fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{Ok(SetSurfaceCount::decode_op(bytes)?.into())} }
