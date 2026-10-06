//! 🚪️ Native artifact representation codecs.
use super::super::super::*;
use semio_framework_os_kernel::{os_spr as protocol,os_store as store};

impl protocol::OpText for Mutation {
    fn parse_op(_line:&str)->Result<Self,store::TextError>{Err(store::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "snapshot-refusal has no mutations",store::TextSpan::at(1,1)))}
    fn print_op(&self)->String{match *self{}}
}
