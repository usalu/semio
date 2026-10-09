//! 📝️ Retains the original selection, streams each borrowed replacement and admits one inverse row.
use super::{TestConfig as State,TestConfigMutation as Mutation,ChangeTestConfigSelection};
use crate::store;
use crate::component::window_config::preparation_custody::WindowEditCustody;
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep};
use semio_framework_value::{ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,RetainedCloneBirthDemand}};
use std::mem::size_of;
fn selection_json_bytes(original:Option<&str>)->Option<usize>{match original{None=>Some(4),Some(text)=>text.chars().try_fold(2usize,|bytes,ch|bytes.checked_add(match ch{'"'|'\\'|'\u{8}'|'\t'|'\n'|'\u{c}'|'\r'=>2,ch if ch<='\u{1f}'=>6,ch=>ch.len_utf8()}))}}
fn selection_binary_bytes(original:Option<&str>)->Option<usize>{original.map_or(Some(2),|text|text.len().checked_add(6))}
pub(crate) fn selection_prepared_bytes(base:Option<&str>,next:Option<&str>)->Option<usize>{13usize.checked_add(selection_json_bytes(next)?).and_then(|bytes|bytes.checked_add(selection_binary_bytes(next)?)).and_then(|bytes|bytes.checked_add(selection_binary_bytes(base)?))}
type Fields=(Option<ControlledRetirement<Option<String>>>,Option<Option<String>>,Option<Vec<Mutation>>);
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct SelectionRetainedEdit;
pub(crate) struct SelectionRetainedEditCursor{phase:u8,offset:usize,allocated:bool,cancelled:bool,custody:WindowEditCustody<Fields>}
impl RetainedCloneEdit<State,Mutation> for SelectionRetainedEdit{
 type Cursor=SelectionRetainedEditCursor;
 fn preflight(&self,mutation:&Mutation,lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String>{let Mutation::ChangeTestConfigSelection(value)=mutation;let bytes=value.selected.as_ref().map_or(0,String::len);let retained_bytes=bytes.checked_mul(4).and_then(|bytes|bytes.checked_add(65536)).ok_or("original selection preparation footprint overflow")?;if lane!=store::HistoryLane::Document||retained_bytes>store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES||selection_prepared_bytes(None,value.selected.as_deref()).is_none_or(|bytes|bytes>1024){return Err("original selection preparation exceeds lane or physical bound".into());}Ok(store::ArtifactStoreOneItemFootprint{work_items:2,retained_bytes})}
 fn begin_demand(&self)->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:0,depth:1}}
 fn snapshot_cursor_birth_demand(&self)->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:0,depth:1}}
 fn begin(&self,grant:RetainedCloneGrant)->Result<(Self::Cursor,RetainedCloneProgress),ValueError>{let mut receipt=self.begin_demand().admit(grant)?;receipt.copied_bytes=size_of::<Self::Cursor>();if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original selection cursor transfer exceeds copy grant"));}Ok((SelectionRetainedEditCursor{phase:0,offset:0,allocated:false,cancelled:false,custody:WindowEditCustody::new((None,None,None))},receipt))}
}
impl SelectionRetainedEditCursor{
 pub(crate) fn normal_demand(&self,base:&State,mutation:&Mutation,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{use semio_framework_value::RetirementDemand;Ok(match self.phase{
  0=>RetirementDemand{copy_bytes:size_of::<Option<String>>()+size_of::<ControlledRetirement<Option<String>>>(),depth:1,..Default::default()},
  1=>{let owner=self.custody.original().0.as_ref().expect("original displaced selection retained");if owner.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Option<String>>>>(),depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original selection normal depth overflow"))?}}},
  2|3=>{let original=if self.phase==2{let Mutation::ChangeTestConfigSelection(value)=mutation;value.selected.as_deref()}else{base.selected.as_deref()};if !self.allocated{RetirementDemand{copy_bytes:size_of::<Option<Option<String>>>(),capacity_bytes:original.map_or(0,str::len),depth:1,..Default::default()}}else if let Some(original)=original.filter(|value|self.offset<value.len()){RetirementDemand{copy_bytes:original[self.offset..].chars().next().unwrap().len_utf8()+size_of::<usize>()*2,depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:size_of::<Option<String>>()*2+size_of::<u8>(),depth:1,..Default::default()}}},
  4=>RetirementDemand{copy_bytes:size_of::<Vec<Mutation>>(),capacity_bytes:size_of::<Mutation>(),depth:1,..Default::default()},
  5=>RetirementDemand{copy_bytes:size_of::<Mutation>()*2+size_of::<Option<String>>(),depth:1,..Default::default()},_=>Default::default()})}
 fn copy_text(&mut self,original:Option<&str>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,String>{
  if !self.allocated{let bytes=original.map_or(0,str::len);let copy=size_of::<Option<Option<String>>>();if grant.maximum_capacity_bytes<bytes||grant.maximum_copy_bytes<copy{return Ok(None);}let value=original.map(|_|{let mut text=String::new();text.try_reserve_exact(bytes).map_err(|_|"original selection text allocation failed")?;Ok::<_,&str>(text)}).transpose()?;self.custody.original_mut().1=Some(value);self.allocated=true;return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:bytes,released_bytes:0}));}
  let Some(original)=original else{return Ok(None)};if self.offset==original.len(){return Ok(None);}let overhead=size_of::<usize>()*2;if grant.maximum_copy_bytes<=overhead{return Ok(None);}let mut end=(self.offset+grant.maximum_copy_bytes-overhead).min(original.len());while !original.is_char_boundary(end){end-=1;}if end==self.offset{return Ok(None);}let text=self.custody.original_mut().1.as_mut().and_then(Option::as_mut).ok_or("original selection partial text absent")?;text.push_str(&original[self.offset..end]);let copied=end-self.offset;self.offset=end;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copied+overhead,..Default::default()}))
 }
}
impl RetainedCloneEditCursor<State,Mutation> for SelectionRetainedEditCursor{
 fn advance(&mut self,base:RetainedCloneRef<'_,State>,post:&mut State,mutation:RetainedCloneRef<'_,Mutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,String>{
  if self.cancelled||self.custody.is_closing()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
  if self.phase==0{let Mutation::ChangeTestConfigSelection(value)=mutation.get();if selection_prepared_bytes(base.get().selected.as_deref(),value.selected.as_deref()).is_none_or(|bytes|bytes>1024){return Err("original selection prepared Pack and forward/inverse rows exceed owner publication bound".into());}}
  let demand=self.normal_demand(base.get(),mutation.get(),grant.maximum_copy_bytes).map_err(ValueError::into_message)?;if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes||demand.depth>grant.maximum_depth{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
  let receipt=match self.phase{
   0=>{let copy=size_of::<Option<String>>()+size_of::<ControlledRetirement<Option<String>>>();if grant.maximum_copy_bytes<copy{return Ok(RetainedCloneEditStep::Progress(Default::default()));}let original=post.selected.take();self.custody.original_mut().0=Some(ControlledRetirement::new(original).map_err(|(error,_)|error.into_message())?);self.phase=1;RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}},
   1=>{let original=self.custody.original_mut().0.as_mut().ok_or("original displaced selection absent")?;if original.terminal_is_empty(){let copy=size_of::<Option<ControlledRetirement<Option<String>>>>();if grant.maximum_copy_bytes<copy{return Ok(RetainedCloneEditStep::Progress(Default::default()));}self.custody.original_mut().0=None;self.phase=2;RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}}else{if grant.maximum_depth<original.next_depth_demand().map_err(ValueError::into_message)?.saturating_add(1){return Ok(RetainedCloneEditStep::Progress(Default::default()));}original.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(ValueError::into_message)?.progress()}},
   2|3=>{let selected=if self.phase==2{let Mutation::ChangeTestConfigSelection(value)=mutation.get();value.selected.as_deref()}else{base.get().selected.as_deref()};if let Some(progress)=self.copy_text(selected,grant)?{progress}else{if !self.allocated||self.offset!=selected.map_or(0,str::len){return Ok(RetainedCloneEditStep::Progress(Default::default()));}let copy=size_of::<Option<String>>()*2+size_of::<u8>();if grant.maximum_copy_bytes<copy{return Ok(RetainedCloneEditStep::Progress(Default::default()));}if self.phase==2{post.selected=self.custody.original_mut().1.take().ok_or("original forward selection absent")?;}self.phase+=1;self.offset=0;self.allocated=false;RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}}},
   4=>{let copy=size_of::<Vec<Mutation>>();let capacity=size_of::<Mutation>();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity{return Ok(RetainedCloneEditStep::Progress(Default::default()));}let mut rows=Vec::new();rows.try_reserve_exact(1).map_err(|_|"original selection inverse row allocation failed")?;self.custody.original_mut().2=Some(rows);self.phase=5;RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,released_bytes:0}},
   5=>{let copy=size_of::<Mutation>()*2+size_of::<Option<String>>();if grant.maximum_copy_bytes<copy{return Ok(RetainedCloneEditStep::Progress(Default::default()));}let fields=self.custody.original_mut();let selected=fields.1.take().ok_or("original inverse selection absent")?;fields.2.as_mut().ok_or("original inverse rows absent")?.push(Mutation::ChangeTestConfigSelection(ChangeTestConfigSelection{selected}));self.phase=6;RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}},
   _=>return Ok(RetainedCloneEditStep::Complete(Default::default())),
  };Ok(if self.phase==6{RetainedCloneEditStep::Complete(receipt)}else{RetainedCloneEditStep::Progress(receipt)})
 }
 fn take_inverse(&mut self)->Option<Vec<Mutation>>{(self.phase==6).then(||self.custody.original_mut().2.take()).flatten()}
 fn foreign_step_presence(&self)->bool{false}
 fn cancel(&mut self){self.cancelled=true;}
 fn begin_close(&mut self)->bool{self.custody.begin_close()}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.custody.step(grant)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.custody.demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.custody.demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.custody.demands(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.custody.demands(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.custody.terminal_is_empty()}
}
