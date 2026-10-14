//! 🎞️ Complete media domain fields remain in their source slot until their original context funds publication.
use semio_framework::{MediaError,MediaType};
use semio_framework_job::{OriginalDomainProjection,OriginalDomainView as V,OriginalDomainFailure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};

#[derive(Debug,semio_framework_value::RetireOwned)]
pub(crate) struct MediaOperationRefusal{pub(crate) source:MediaOperationSource}
#[derive(Debug,semio_framework_value::RetireOwned)]
pub(crate) enum MediaOperationSource{Media(MediaError),Dispatch(semio_framework::ToolDispatchError)}
impl From<MediaError> for MediaOperationSource{fn from(source:MediaError)->Self{Self::Media(source)}}
impl OriginalDomainFailure for MediaOperationRefusal{
 fn domain_code(&self)->&str{match &self.source{MediaOperationSource::Dispatch(_)=>"media.dispatch",MediaOperationSource::Media(source)=>match source{MediaError::UnknownPort(_)=>"media.unknown-port",MediaError::Incompatible{..}=>"media.incompatible",MediaError::Payload(..)=>"media.payload",MediaError::NotImplemented=>"media.not-implemented"}}}
 fn domain_message(&self)->&str{match &self.source{MediaOperationSource::Dispatch(source)=>match source{semio_framework::ToolDispatchError::UnknownController{controller_id}=>controller_id,semio_framework::ToolDispatchError::Factory{detail,..}=>detail,semio_framework::ToolDispatchError::RawWireLimit{..}=>"media dispatch raw wire limit exceeded"},MediaOperationSource::Media(source)=>match source{MediaError::UnknownPort(port)=>port,MediaError::Incompatible{port,..}=>port,MediaError::Payload(_,message)=>message,MediaError::NotImplemented=>"media ports are not implemented for this app"}}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original media domain projection path changed")}
fn media_type(value:&MediaType,path:&[usize])->Result<V<'static>,ValueError>{Ok(match path{[]=>V::IntrinsicObject(2),[0]=>V::Enum(value.class as u32),[1]=>V::Enum(value.form as u32),_=>return Err(invalid())})}
impl OriginalDomainProjection for MediaOperationRefusal{
 fn projection_identity(&self)->usize{&self.source as*const MediaOperationSource as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{
  let source=match &self.source{MediaOperationSource::Media(source)=>source,MediaOperationSource::Dispatch(source)=>return Ok(match path{
   []=>V::IntrinsicObject(6),[0]=>V::IntrinsicText(match source{semio_framework::ToolDispatchError::UnknownController{..}=>"unknownController",semio_framework::ToolDispatchError::Factory{..}=>"factory",semio_framework::ToolDispatchError::RawWireLimit{..}=>"rawWireLimit"}),
   [1]=>V::IntrinsicText(match source{semio_framework::ToolDispatchError::UnknownController{controller_id}|semio_framework::ToolDispatchError::Factory{controller_id,..}|semio_framework::ToolDispatchError::RawWireLimit{controller_id,..}=>controller_id}),
   [2]=>match source{semio_framework::ToolDispatchError::Factory{tool_id,..}|semio_framework::ToolDispatchError::RawWireLimit{tool_id,..}=>V::IntrinsicText(tool_id),_=>V::IntrinsicNull},
   [3]=>match source{semio_framework::ToolDispatchError::Factory{detail,..}=>V::IntrinsicText(detail),_=>V::IntrinsicNull},
   [4]=>match source{semio_framework::ToolDispatchError::RawWireLimit{actual,..}=>V::IntrinsicNumber(semio_framework_value::Number::UInt(*actual as u64)),_=>V::IntrinsicNull},
   [5]=>match source{semio_framework::ToolDispatchError::RawWireLimit{maximum,..}=>V::IntrinsicNumber(semio_framework_value::Number::UInt(*maximum as u64)),_=>V::IntrinsicNull},_=>return Err(invalid())})};project_media_error(source,path)
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match &self.source{MediaOperationSource::Dispatch(_)=>match path{[]=>["kind","controllerId","toolId","detail","actual","maximum"].get(index).copied().ok_or_else(invalid),_=>Err(invalid())},MediaOperationSource::Media(source)=>match path{[]=>["kind","port","message","produced","accepted"].get(index).copied().ok_or_else(invalid),[3]|[4] if matches!(source,MediaError::Incompatible{..})=>["class","form"].get(index).copied().ok_or_else(invalid),_=>Err(invalid())}}}
}
/// 🫴️ Projects the borrowed media enum without moving its original payload.
pub(crate) fn project_media_error<'a>(source:&'a MediaError,path:&[usize])->Result<V<'a>,ValueError>{Ok(match path{
  []=>V::IntrinsicObject(5),
  [0]=>V::IntrinsicText(match source{MediaError::UnknownPort(_)=>"unknownPort",MediaError::Incompatible{..}=>"incompatible",MediaError::Payload(..)=>"payload",MediaError::NotImplemented=>"notImplemented"}),
  [1]=>match source{MediaError::UnknownPort(port)|MediaError::Incompatible{port,..}|MediaError::Payload(port,_)=>V::IntrinsicText(port),MediaError::NotImplemented=>V::IntrinsicNull},
  [2]=>match source{MediaError::Payload(_,message)=>V::IntrinsicText(message),_=>V::IntrinsicNull},
  [3,rest @ ..]=>match source{MediaError::Incompatible{produced,..}=>return media_type(produced,rest),_ if rest.is_empty()=>V::IntrinsicNull,_=>return Err(invalid())},
  [4,rest @ ..]=>match source{MediaError::Incompatible{accepted,..}=>return media_type(accepted,rest),_ if rest.is_empty()=>V::IntrinsicNull,_=>return Err(invalid())},
  _=>return Err(invalid()),
 })}
/// 🔑️ Borrows media fields for enclosing domain projections.
pub(crate) fn key_media_error<'a>(source:&'a MediaError,path:&[usize],index:usize)->Result<&'a str,ValueError>{match path{[]=>["kind","port","message","produced","accepted"].get(index).copied().ok_or_else(invalid),[3]|[4]if matches!(source,MediaError::Incompatible{..})=>["class","form"].get(index).copied().ok_or_else(invalid),_=>Err(invalid())}}



/// 🧳️ Keeps domain output and original physical/context failure distinct before paid publication.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub(crate) enum MediaOperationFailure{Domain(MediaOperationSource),Original(semio_framework_job::OriginalTurnFailure)}
impl From<MediaError> for MediaOperationFailure{fn from(source:MediaError)->Self{Self::Domain(source.into())}}
impl From<semio_framework::Fault> for MediaOperationFailure{fn from(source:semio_framework::Fault)->Self{Self::Original(source.into())}}
impl From<semio_framework_job::OriginalTurnFailure> for MediaOperationFailure{fn from(source:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(source)}}
impl From<crate::PluginLifecycleFailure> for MediaOperationFailure{fn from(source:crate::PluginLifecycleFailure)->Self{Self::Original(source.into())}}
impl From<ValueError> for MediaOperationFailure{fn from(source:ValueError)->Self{Self::Original(source.into())}}

impl From<semio_framework::ToolDispatchError> for MediaOperationFailure{fn from(source:semio_framework::ToolDispatchError)->Self{Self::Domain(MediaOperationSource::Dispatch(source))}}

#[cfg(any(test,feature="media-turn-testing"))]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
