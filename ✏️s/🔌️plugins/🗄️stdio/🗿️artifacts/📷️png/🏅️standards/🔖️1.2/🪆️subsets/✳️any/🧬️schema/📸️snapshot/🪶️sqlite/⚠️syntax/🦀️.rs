//! ⚠️ Literal classification uses inline syntax facts; caller refusals retain their owned typed cause.
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(Debug)] pub(super) enum Error{Syntax(&'static str),Refusal(ValueError)}
impl From<ValueError> for Error{fn from(value:ValueError)->Self{Self::Refusal(value)}}
impl Error{pub fn into_value_error(self)->ValueError{match self{Self::Syntax(message)=>ValueError::new(ValueRefusalKind::InvalidValue,message),Self::Refusal(error)=>error}}}
