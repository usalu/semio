//! 🎞️ Complete media domain fields remain in their source slot until their original context funds publication.
use semio_framework_plugin_decl_app::invalid;
use semio_framework_plugin_decl_app::project_media_error;
use semio_framework::MediaError;
use semio_framework_job::{OriginalDomainProjection, OriginalDomainView as V, OriginalDomainFailure};
use semio_framework_value::{ValueError, RetainedCloneProgress};

#[doc(hidden)]
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct MediaOperationRefusal{pub source:MediaOperationSource}
#[doc(hidden)]
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum MediaOperationSource{Media(MediaError),Dispatch(semio_framework::ToolDispatchError)}
impl From<MediaError> for MediaOperationSource{fn from(source:MediaError)->Self{Self::Media(source)}}
impl OriginalDomainFailure for MediaOperationRefusal{
 fn domain_code(&self)->&str{match &self.source{MediaOperationSource::Dispatch(_)=>"media.dispatch",MediaOperationSource::Media(source)=>match source{MediaError::UnknownPort(_)=>"media.unknown-port",MediaError::Incompatible{..}=>"media.incompatible",MediaError::Payload(..)=>"media.payload",MediaError::NotImplemented=>"media.not-implemented"}}}
 fn domain_message(&self)->&str{match &self.source{MediaOperationSource::Dispatch(source)=>match source{semio_framework::ToolDispatchError::UnknownController{controller_id}=>controller_id,semio_framework::ToolDispatchError::Factory{detail,..}=>detail,semio_framework::ToolDispatchError::RawWireLimit{..}=>"media dispatch raw wire limit exceeded"},MediaOperationSource::Media(source)=>match source{MediaError::UnknownPort(port)=>port,MediaError::Incompatible{port,..}=>port,MediaError::Payload(_,message)=>message,MediaError::NotImplemented=>"media ports are not implemented for this app"}}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
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


/// 🧳️ Keeps domain output and original physical/context failure distinct before paid publication.
#[doc(hidden)]
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum MediaOperationFailure{Domain(MediaOperationSource),Original(semio_framework_job::OriginalTurnFailure)}
impl From<MediaError> for MediaOperationFailure{fn from(source:MediaError)->Self{Self::Domain(source.into())}}
impl From<semio_framework::Fault> for MediaOperationFailure{fn from(source:semio_framework::Fault)->Self{Self::Original(source.into())}}
impl From<semio_framework_job::OriginalTurnFailure> for MediaOperationFailure{fn from(source:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(source)}}
impl From<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> for MediaOperationFailure{fn from(source:semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure)->Self{Self::Original(source.into())}}
impl From<ValueError> for MediaOperationFailure{fn from(source:ValueError)->Self{Self::Original(source.into())}}

impl From<semio_framework::ToolDispatchError> for MediaOperationFailure{fn from(source:semio_framework::ToolDispatchError)->Self{Self::Domain(MediaOperationSource::Dispatch(source))}}


pub use semio_framework_plugin_decl_app::media_operation_refusal::*;
