//! 🎞️ Lift file: 4 item(s), about 14 lines, of `🎞️media/⚠️turn/🦀️.rs` that belong to crate `decl-app` (WP2).
//!
//! WP3 owns the source file and cuts the items here (invalid, media_type, project_media_error, key_media_error); WP2 wires this file with one `mod` line in the crate root. Until `decl-app` exists, the source module re-exports the items with `pub use`.
use semio_framework::MediaError;
use semio_framework::MediaType;
use semio_framework_job::OriginalDomainView as V;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original media domain projection path changed")}
pub fn media_type(value:&MediaType,path:&[usize])->Result<V<'static>,ValueError>{Ok(match path{[]=>V::IntrinsicObject(2),[0]=>V::Enum(value.class as u32),[1]=>V::Enum(value.form as u32),_=>return Err(invalid())})}

/// 🫴️ Projects the borrowed media enum without moving its original payload.
pub fn project_media_error<'a>(source:&'a MediaError,path:&[usize])->Result<V<'a>,ValueError>{Ok(match path{
  []=>V::IntrinsicObject(5),
  [0]=>V::IntrinsicText(match source{MediaError::UnknownPort(_)=>"unknownPort",MediaError::Incompatible{..}=>"incompatible",MediaError::Payload(..)=>"payload",MediaError::NotImplemented=>"notImplemented"}),
  [1]=>match source{MediaError::UnknownPort(port)|MediaError::Incompatible{port,..}|MediaError::Payload(port,_)=>V::IntrinsicText(port),MediaError::NotImplemented=>V::IntrinsicNull},
  [2]=>match source{MediaError::Payload(_,message)=>V::IntrinsicText(message),_=>V::IntrinsicNull},
  [3,rest @ ..]=>match source{MediaError::Incompatible{produced,..}=>return media_type(produced,rest),_ if rest.is_empty()=>V::IntrinsicNull,_=>return Err(invalid())},
  [4,rest @ ..]=>match source{MediaError::Incompatible{accepted,..}=>return media_type(accepted,rest),_ if rest.is_empty()=>V::IntrinsicNull,_=>return Err(invalid())},
  _=>return Err(invalid()),
 })}
/// 🔑️ Borrows media fields for enclosing domain projections.
pub fn key_media_error<'a>(source:&'a MediaError,path:&[usize],index:usize)->Result<&'a str,ValueError>{match path{[]=>["kind","port","message","produced","accepted"].get(index).copied().ok_or_else(invalid),[3]|[4]if matches!(source,MediaError::Incompatible{..})=>["class","form"].get(index).copied().ok_or_else(invalid),_=>Err(invalid())}}
