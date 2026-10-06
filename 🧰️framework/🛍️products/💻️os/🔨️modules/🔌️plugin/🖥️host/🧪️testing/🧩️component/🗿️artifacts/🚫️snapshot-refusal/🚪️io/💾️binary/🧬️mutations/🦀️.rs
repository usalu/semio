//! 🚪️ Native artifact representation codecs.
use super::super::super::*;
use semio_framework_os_kernel::{os_spr as protocol,os_store as store};

impl protocol::OpBinary for Mutation {
    fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{match *self{}}
    fn decode_op(_bytes:&[u8])->Result<Self,protocol::ProtocolError>{Err(protocol::ProtocolError::Malformed{what:"snapshot-refusal-mutation",offset:0,detail:"snapshot-refusal has no mutations".into()})}
}
