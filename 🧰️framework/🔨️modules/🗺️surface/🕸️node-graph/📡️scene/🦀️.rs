//! 📡️ Explicit Pack framing retains scene projection and embedded text across finite work grants.
use super::{GraphNodeRecord,GraphPortRecord,GraphEdgeRecord,NodeGraphScenePayload,Viewport2d};
use pack::{PackLimits,PackRefusal,intrinsic::{IntrinsicFormat,RetainedIntrinsicBody,RetainedIntrinsicDocument,RetainedIntrinsicInput,RetainedIntrinsicStep}};
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind,NativeEncodeControl,native_encoding::NativeEncodeContinuation,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress},retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement}};
use semio_framework_pack_json::JsonWriteCursor;
use semio_framework_io_base64::{Base64Input,Base64InputRange,Base64DecodeCursor,Base64Control,Base64ControlError};
use std::mem::{ManuallyDrop,size_of};

/// 🎟️ One materialization ceiling applies to source, parser, typed metadata and embedded output births.
#[derive(Clone,Copy,Debug)]
pub struct SceneDecodeLimits{pub maximum_owned_bytes:usize,pub maximum_depth:usize,pub maximum_items:usize}
impl SceneDecodeLimits{fn pack(self)->PackLimits{PackLimits{max_total_alloc:self.maximum_owned_bytes as u64,max_depth:self.maximum_depth.min(u16::MAX as usize)as u16,max_items:self.maximum_items as u64,..Default::default()}}}
/// 📊️ Actual work and cumulative materialization custody are observable at each yield.
#[derive(Clone,Copy,Debug,Default)]
pub struct SceneDecodeStep{pub units:usize,pub admitted_bytes:usize,pub complete:bool}
/// 🚦️ The current owner phase exposes cancellation boundaries without advancing work.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SceneDecodePhase{Input,Projection,Base64,Pack,JsonMeasure,JsonWrite,Retirement,Complete}
impl SceneDecodePhase{pub fn as_str(self)->&'static str{match self{Self::Input=>"input",Self::Projection=>"projection",Self::Base64=>"base64",Self::Pack=>"pack",Self::JsonMeasure=>"json-measure",Self::JsonWrite=>"json-write",Self::Retirement=>"retirement",Self::Complete=>"complete"}}}

enum Wire<'a>{Body(RetainedIntrinsicBody<'a>),Document(RetainedIntrinsicDocument<'a>)}
macro_rules! wire_dispatch{($self:expr,$method:ident $(,$argument:expr)*)=>{match $self{Wire::Body(owner)=>owner.$method($($argument),*),Wire::Document(owner)=>owner.$method($($argument),*)}}}
impl<'a> Wire<'a>{fn new(input:RetainedIntrinsicInput<'a>,format:IntrinsicFormat,limits:SceneDecodeLimits)->Result<Self,(PackRefusal,RetainedIntrinsicInput<'a>)>{match format{IntrinsicFormat::Body=>RetainedIntrinsicBody::new(input,limits.pack(),limits.maximum_owned_bytes).map(Self::Body),IntrinsicFormat::Document=>RetainedIntrinsicDocument::new(input,limits.pack(),limits.maximum_owned_bytes).map(Self::Document)}}}

semio_framework_value::artifact_retire_struct!(GraphPortRecord{id,label,code,abbreviation,full_name,artifact_kind,value_type});
semio_framework_value::artifact_retire_struct!(GraphNodeRecord{id,label,instance_id,plugin_id,app_id,icon,x,y,width,height,inputs,outputs});
semio_framework_value::artifact_retire_struct!(GraphEdgeRecord{id,source_node_id,source_port_id,target_node_id,target_port_id});
impl RetireOwned for NodeGraphScenePayload{
 fn retirement(self)->Box<dyn RetirementCursor>{use semio_framework_value::retirement::{sequence,deferred};let Self{nodes,edges,viewport:_,preview_off_json,lod_json,controls_json,clusters_json,computing_json,status_json,capabilities_json,host_snapshot_json}=self;sequence(vec![deferred(nodes),deferred(edges),deferred(preview_off_json),deferred(lod_json),deferred(controls_json),deferred(clusters_json),deferred(computing_json),deferred(status_json),deferred(capabilities_json),deferred(host_snapshot_json)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for as birth};sequence_birth_bytes(&[birth(&self.nodes),birth(&self.edges),birth(&self.preview_off_json),birth(&self.lod_json),birth(&self.controls_json),birth(&self.clusters_json),birth(&self.computing_json),birth(&self.status_json),birth(&self.capabilities_json),birth(&self.host_snapshot_json)])}
 fn controlled_retirement_supported()->bool{true}
}
struct Owned{root:DslValue,payload:Option<NodeGraphScenePayload>,node:GraphNodeRecord,edge:GraphEdgeRecord,port:GraphPortRecord,texts:Vec<String>,pending_text:String,pending_bytes:Vec<u8>,writers:Vec<JsonWriteCursor<DslValue>>,writer:Option<JsonWriteCursor<DslValue>>}
semio_framework_value::artifact_retire_struct!(Owned{root,payload,node,edge,port,texts,pending_text,pending_bytes,writers,writer});
impl Default for Owned{fn default()->Self{Self{root:DslValue::Null,payload:Some(Default::default()),node:Default::default(),edge:Default::default(),port:Default::default(),texts:Vec::new(),pending_text:String::new(),pending_bytes:Vec::new(),writers:Vec::new(),writer:None}}}
const ABSENT:usize=usize::MAX;
const FIELDS:[&str;11]=["nodes","edges","viewport","previewOffJson","lodJson","controlsJson","clustersJson","computingJson","statusJson","capabilitiesJson","hostSnapshotJson"];
const NODE:[&str;12]=["id","label","instanceId","pluginId","appId","icon","x","y","width","height","inputs","outputs"];
const PORT:[&str;7]=["id","label","code","abbreviation","fullName","resourceKind","valueType"];
const EDGE:[&str;5]=["id","sourceNodeId","sourcePortId","targetNodeId","targetPortId"];
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"invalid typed scene field")}
fn capacity()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"scene materialization ownership exceeds caller limit")}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"scene metadata allocation failed")}
fn pack_error(error:PackRefusal)->ValueError{ValueError::literal(error.kind(),"scene intrinsic framing refused")}
fn base64_error(error:Base64ControlError)->ValueError{ValueError::literal(match error{Base64ControlError::OutputLimit=>ValueRefusalKind::OwnershipLimit,Base64ControlError::Cancelled=>ValueRefusalKind::Canceled,_=>ValueRefusalKind::InvalidValue},"scene embedded Base64 refused")}
fn entries(value:&DslValue)->Option<&[(String,DslValue)]>{if let DslValue::Object(entries)=value{Some(entries)}else{None}}
fn array(value:&DslValue)->Option<&[DslValue]>{if let DslValue::Array(values)=value{Some(values)}else{None}}
fn number(value:&DslValue)->Option<f64>{match value{DslValue::Number(Number::Int(value))=>Some(*value as f64),DslValue::Number(Number::UInt(value))=>Some(*value as f64),DslValue::Number(Number::Float(value))=>Some(*value),_=>None}.filter(|value|value.is_finite())}
fn optional_text(value:&DslValue)->bool{matches!(value,DslValue::Null|DslValue::String(_))}
fn take_text(value:&mut DslValue)->Option<String>{if let DslValue::String(text)=value{Some(std::mem::take(text))}else{None}}

/// 🎮️ No owner is implicitly discarded on refusal, cancellation, withdrawal or successful output transfer.
pub struct SceneDecodeCursor<'a>{wire:Wire<'a>,limits:SceneDecodeLimits,owned:ManuallyDrop<Owned>,retiring:Option<ControlledRetirement<Owned>>,indices:[usize;11],stage:u8,scan:usize,collection:usize,record:usize,field:usize,port:usize,port_field:usize,port_source:usize,mask:u16,port_mask:u16,valid:bool,viewport:[f64;3],embedded:usize,base64:Option<Base64DecodeCursor<'static>>,base64_admitted:usize,inner:[Option<RetainedIntrinsicBody<'static>>;8],inner_admitted:usize,writer_receipt:Option<NativeEncodeContinuation>,admitted:usize,complete:bool,closing:bool,closed:bool,fault:Option<ValueError>}
impl<'a> SceneDecodeCursor<'a>{
 /// 📥️ Admits exact original source custody without parsing or allocating a complete payload clone.
 pub fn new(input:RetainedIntrinsicInput<'a>,format:IntrinsicFormat,limits:SceneDecodeLimits)->Result<Self,(ValueError,RetainedIntrinsicInput<'a>)>{let wire=Wire::new(input,format,limits).map_err(|(error,input)|(pack_error(error),input))?;let admitted=wire_dispatch!(&wire,admitted_bytes);Ok(Self{wire,limits,owned:ManuallyDrop::new(Owned::default()),retiring:None,indices:[ABSENT;11],stage:0,scan:0,collection:0,record:0,field:0,port:0,port_field:0,port_source:ABSENT,mask:0,port_mask:0,valid:true,viewport:[0.0;3],embedded:0,base64:None,base64_admitted:0,inner:std::array::from_fn(|_|None),inner_admitted:0,writer_receipt:None,admitted,complete:false,closing:false,closed:false,fault:None})}
 fn remaining(&self)->usize{self.limits.maximum_owned_bytes.saturating_sub(self.admitted)}
 fn admit(&mut self,bytes:usize)->Result<(),ValueError>{self.admitted=self.admitted.checked_add(bytes).filter(|bytes|*bytes<=self.limits.maximum_owned_bytes).ok_or_else(capacity)?;Ok(())}
 fn reserve<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=self.remaining()&&*bytes<=isize::MAX as usize).ok_or_else(capacity)?;let _=bytes;let mut values=Vec::new();values.try_reserve_exact(count).map_err(|_|allocation())?;self.admit(values.capacity()*size_of::<T>())?;Ok(values)}
 fn root_field(&self,index:usize)->Option<&DslValue>{entries(&self.owned.root)?.get(self.indices[index]).map(|(_,value)|value)}
 fn records(&self)->Option<&[DslValue]>{array(self.root_field(self.collection)?)}
 fn current_entries(&self)->Option<&[(String,DslValue)]>{entries(self.records()?.get(self.record)?)}
 fn current_ports(&self)->Option<&[DslValue]>{array(&self.current_entries()?.get(self.port_source)?.1)}
 fn field_mut(&mut self,port:bool)->&mut DslValue{let DslValue::Object(root)=&mut self.owned.root else{unreachable!()};let DslValue::Array(records)=&mut root[self.indices[self.collection]].1 else{unreachable!()};let DslValue::Object(fields)=&mut records[self.record]else{unreachable!()};if port{let DslValue::Array(ports)=&mut fields[self.port_source].1 else{unreachable!()};let DslValue::Object(fields)=&mut ports[self.port]else{unreachable!()};&mut fields[self.port_field].1}else{&mut fields[self.field].1}}
 fn begin_collection(&mut self){self.record=0;self.field=0;self.mask=0;self.valid=self.records().is_some();self.stage=if self.valid{2}else{4};}
 fn fail_collection(&mut self){self.valid=false;self.stage=4;}
 fn validate_record(&mut self){
  let Some(records)=self.records()else{self.fail_collection();return};if self.record==records.len(){self.stage=4;return}
  let Some(fields)=self.current_entries()else{self.fail_collection();return};if self.field==fields.len(){let required=if self.collection==0{1}else{31};if self.mask&required!=required{self.fail_collection();return}self.record+=1;self.field=0;self.mask=0;return}
  let(key,value)=&fields[self.field];let names=if self.collection==0{&NODE[..]}else{&EDGE[..]};let Some(kind)=names.iter().position(|name|*name==key)else{self.field+=1;return};if self.mask&(1<<kind)!=0{self.fail_collection();return}let valid=if self.collection==1||kind==0{matches!(value,DslValue::String(_))}else if kind<6{optional_text(value)}else if kind<10{matches!(value,DslValue::Null)||number(value).is_some()}else{matches!(value,DslValue::Null|DslValue::Array(_))};
  let ports=self.collection==0&&kind>=10&&matches!(value,DslValue::Array(_));if !valid{self.fail_collection();return}self.mask|=1<<kind;
  if ports{self.port_source=self.field;self.port=0;self.port_field=0;self.port_mask=0;self.field+=1;self.stage=3}else{self.field+=1}
 }
 fn validate_port(&mut self){let ports=self.current_ports().unwrap();if self.port==ports.len(){self.stage=2;return}let Some(fields)=entries(&ports[self.port])else{self.fail_collection();return};if self.port_field==fields.len(){if self.port_mask&1==0{self.fail_collection();return}self.port+=1;self.port_field=0;self.port_mask=0;return}let(key,value)=&fields[self.port_field];if let Some(kind)=PORT.iter().position(|name|*name==key){if self.port_mask&(1<<kind)!=0||if kind==0{!matches!(value,DslValue::String(_))}else{!optional_text(value)}{self.fail_collection();return}self.port_mask|=1<<kind;}self.port_field+=1;}
 fn reserve_records(&mut self)->Result<(),ValueError>{if self.valid{let count=self.records().unwrap().len();if self.collection==0{let values=self.reserve(count)?;self.owned.payload.as_mut().unwrap().nodes=values}else{let values=self.reserve(count)?;self.owned.payload.as_mut().unwrap().edges=values}self.record=0;self.field=0;self.stage=5}else{self.stage=7}Ok(())}
 fn build_record(&mut self)->Result<(),ValueError>{
  if self.record==self.records().unwrap().len(){self.stage=7;return Ok(())}let fields=self.current_entries().unwrap();if self.field==fields.len(){if self.collection==0{let node=std::mem::take(&mut self.owned.node);self.owned.payload.as_mut().unwrap().nodes.push(node)}else{let edge=std::mem::take(&mut self.owned.edge);self.owned.payload.as_mut().unwrap().edges.push(edge)}self.record+=1;self.field=0;return Ok(())}
  let names=if self.collection==0{&NODE[..]}else{&EDGE[..]};let Some(kind)=names.iter().position(|name|*name==fields[self.field].0)else{self.field+=1;return Ok(())};
  if self.collection==1{let text=take_text(self.field_mut(false)).unwrap();match kind{0=>self.owned.edge.id=text,1=>self.owned.edge.source_node_id=text,2=>self.owned.edge.source_port_id=text,3=>self.owned.edge.target_node_id=text,_=>self.owned.edge.target_port_id=text};}
  else if kind<6{let text=take_text(self.field_mut(false));match kind{0=>self.owned.node.id=text.unwrap(),1=>self.owned.node.label=text,2=>self.owned.node.instance_id=text,3=>self.owned.node.plugin_id=text,4=>self.owned.node.app_id=text,_=>self.owned.node.icon=text};}
  else if kind<10{let value=number(&fields[self.field].1);match kind{6=>self.owned.node.x=value,7=>self.owned.node.y=value,8=>self.owned.node.width=value,_=>self.owned.node.height=value};}
  else if let Some(values)=array(&fields[self.field].1){let count=values.len();let ports=self.reserve(count)?;if kind==10{self.owned.node.inputs=Some(ports)}else{self.owned.node.outputs=Some(ports)}self.port_source=self.field;self.port=0;self.port_field=0;self.stage=6;}
  self.field+=1;Ok(())
 }
 fn build_port(&mut self){let ports=self.current_ports().unwrap();if self.port==ports.len(){self.stage=5;return}let fields=entries(&ports[self.port]).unwrap();if self.port_field==fields.len(){let port=std::mem::take(&mut self.owned.port);let input=self.current_entries().unwrap()[self.port_source].0=="inputs";if input{self.owned.node.inputs.as_mut().unwrap().push(port)}else{self.owned.node.outputs.as_mut().unwrap().push(port)}self.port+=1;self.port_field=0;return}if let Some(kind)=PORT.iter().position(|name|*name==fields[self.port_field].0){let text=take_text(self.field_mut(true));match kind{0=>self.owned.port.id=text.unwrap(),1=>self.owned.port.label=text,2=>self.owned.port.code=text,3=>self.owned.port.abbreviation=text,4=>self.owned.port.full_name=text,5=>self.owned.port.artifact_kind=text,_=>self.owned.port.value_type=text};}self.port_field+=1;}
 fn viewport(&mut self)->Result<(),ValueError>{if self.indices[2]==ABSENT{self.stage=9;return Ok(())}let fields=entries(self.root_field(2).unwrap()).ok_or_else(invalid)?;if self.scan==fields.len(){if self.mask!=7{return Err(invalid())}let viewport=Viewport2d{x:self.viewport[0],y:self.viewport[1],zoom:self.viewport[2]};viewport.validate()?;self.owned.payload.as_mut().unwrap().viewport=Some(viewport);self.stage=9;return Ok(())}let(key,value)=&fields[self.scan];let kind=["x","y","zoom"].iter().position(|name|*name==key).ok_or_else(invalid)?;if self.mask&(1<<kind)!=0{return Err(invalid())}self.viewport[kind]=number(value).ok_or_else(invalid)?;self.mask|=1<<kind;self.scan+=1;Ok(())}
 fn output_text(&mut self,text:String){let payload=self.owned.payload.as_mut().unwrap();let field=match self.embedded{0=>&mut payload.preview_off_json,1=>&mut payload.lod_json,2=>&mut payload.controls_json,3=>&mut payload.clusters_json,4=>&mut payload.computing_json,5=>&mut payload.status_json,6=>&mut payload.capabilities_json,_=>&mut payload.host_snapshot_json};*field=Some(text);self.embedded+=1;self.stage=9;}
 fn embedded(&mut self)->Result<(),ValueError>{if self.embedded==8{self.complete=true;return Ok(())}let index=self.indices[self.embedded+3];if index==ABSENT{self.embedded+=1;return Ok(())}let DslValue::Object(root)=&mut self.owned.root else{unreachable!()};let Some(text)=take_text(&mut root[index].1)else{self.embedded+=1;return Ok(())};if !text.starts_with("pk:"){self.output_text(text);return Ok(())}self.owned.pending_text=text;
  if self.owned.texts.capacity()==0{let texts=self.reserve(8)?;self.owned.texts=texts;let writers=self.reserve(8)?;self.owned.writers=writers;return Ok(())}
  let text=std::mem::take(&mut self.owned.pending_text);let end=text.len();self.base64=Some(Base64DecodeCursor::new_range(Base64Input::OwnedText(text),Base64InputRange{start:3,end}));self.base64_admitted=0;self.stage=10;Ok(())
 }
 fn one(&mut self,demand:usize)->Result<(),ValueError>{match self.stage{
  0=>{let before=wire_dispatch!(&self.wire,admitted_bytes);let result:Result<RetainedIntrinsicStep,_>=wire_dispatch!(&mut self.wire,advance,1,false);let after=wire_dispatch!(&self.wire,admitted_bytes);self.admit(after.checked_sub(before).ok_or_else(invalid)?)?;if result.map_err(pack_error)?.complete{self.owned.root=wire_dispatch!(&mut self.wire,take_output).unwrap();self.stage=1}},
  1=>{let Some(fields)=entries(&self.owned.root)else{self.stage=2;self.begin_collection();return Ok(())};if self.scan==fields.len(){self.begin_collection()}else{if let Some(index)=FIELDS.iter().position(|name|*name==fields[self.scan].0){self.indices[index]=self.scan}self.scan+=1}},
  2=>self.validate_record(),3=>self.validate_port(),4=>self.reserve_records()?,5=>self.build_record()?,6=>self.build_port(),
  7=>{if self.collection==0{self.collection=1;self.begin_collection()}else{self.stage=8;self.scan=0;self.mask=0}},8=>self.viewport()?,9=>{
   if !self.owned.pending_text.is_empty(){let text=std::mem::take(&mut self.owned.pending_text);let end=text.len();self.base64=Some(Base64DecodeCursor::new_range(Base64Input::OwnedText(text),Base64InputRange{start:3,end}));self.base64_admitted=0;self.stage=10}else{self.embedded()?}
  },
  10=>{let maximum=self.remaining();let mut allow=|_|true;let mut control=Base64Control{maximum_output_bytes:maximum+self.base64_admitted,progress:&mut allow};let cursor=self.base64.as_mut().unwrap();let result=cursor.step(demand,&mut control);let born=cursor.output_capacity().checked_sub(self.base64_admitted).ok_or_else(invalid)?;self.base64_admitted+=born;let complete=cursor.is_complete();self.admit(born)?;result.map_err(base64_error)?;if complete{let parts=self.base64.take().unwrap().into_parts();let Base64Input::OwnedText(text)=parts.input else{unreachable!()};self.owned.texts.push(text);self.owned.pending_bytes=parts.output.unwrap();self.stage=11}},
  11=>{let bytes=std::mem::take(&mut self.owned.pending_bytes);let source=bytes.capacity();let mut limits=self.limits;limits.maximum_owned_bytes=self.remaining().checked_add(source).ok_or_else(capacity)?;match RetainedIntrinsicBody::new(RetainedIntrinsicInput::OwnedBytes(bytes),limits.pack(),limits.maximum_owned_bytes){Ok(owner)=>{self.inner_admitted=owner.admitted_bytes();self.inner[self.embedded]=Some(owner);self.stage=12},Err((error,input))=>{if let RetainedIntrinsicInput::OwnedBytes(bytes)=input{self.owned.pending_bytes=bytes}return Err(pack_error(error))}}},
  12=>{let owner=self.inner[self.embedded].as_mut().unwrap();let result=owner.advance(1,false);let after=owner.admitted_bytes();let born=after.checked_sub(self.inner_admitted).ok_or_else(invalid)?;self.inner_admitted=after;self.admit(born)?;if result.map_err(pack_error)?.complete{let value=self.inner[self.embedded].as_mut().unwrap().take_output().unwrap();self.owned.writer=Some(JsonWriteCursor::new(value));let mut allow=|_|true;self.writer_receipt=Some(NativeEncodeControl::new(self.remaining(),&mut allow).pause()?);self.stage=13}},
  13=>{
   let demand=self.owned.writer.as_ref().unwrap().normal_step_demands()?;
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   let mut allow=|_|true;let mut control=NativeEncodeControl::resume(self.writer_receipt.take().unwrap(),&mut allow)?;let before=control.owned_bytes();let maximum=control.maximum_bytes();
   let result=control.scoped_maximum(maximum,|control|self.owned.writer.as_mut().unwrap().step(1,control,grant));
   let progress=self.owned.writer.as_ref().unwrap().normal_step_progress();let born=control.owned_bytes().checked_sub(before).ok_or_else(invalid)?;self.writer_receipt=Some(control.pause()?);self.admit(born)?;
   if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"scene JSON writer exceeded original turn admission"))}
   if let Some(output)=result?{let writer=self.owned.writer.take().unwrap();self.owned.writers.push(writer);self.writer_receipt.take();self.output_text(output)}
  },
  _=>return Err(invalid())
 }Ok(())}
 /// 📏️ The defining Base64 chunk demand remains visible and undersized grants preserve all positions.
 pub fn next_work_demand(&self)->usize{if self.complete||self.closing||self.fault.is_some(){0}else if self.stage==10{self.base64.as_ref().unwrap().next_work_demand().max(1)}else{1}}
 /// 👁️ Borrows the original immutable outer wire while its owner remains live.
 pub fn input(&self)->Option<&[u8]>{wire_dispatch!(&self.wire,input)}
 pub fn admitted_bytes(&self)->usize{self.admitted}
 /// 🚥️ Reports the actual retained parser, projection, encoder or retirement owner.
 pub fn phase(&self)->SceneDecodePhase{if self.closing{SceneDecodePhase::Retirement}else if self.complete{SceneDecodePhase::Complete}else{match self.stage{0=>SceneDecodePhase::Input,10=>SceneDecodePhase::Base64,11|12=>SceneDecodePhase::Pack,13=>if self.owned.writer.as_ref().unwrap().progress().1{SceneDecodePhase::JsonWrite}else{SceneDecodePhase::JsonMeasure},_=>SceneDecodePhase::Projection}}}
 /// ⏱️ Returns only performed work, with no accumulated credit or synchronous fallback.
 pub fn advance(&mut self,maximum_units:usize,cancelled:bool)->Result<SceneDecodeStep,ValueError>{if let Some(error)=&self.fault{return Err(error.clone())}if cancelled{let error=ValueError::literal(ValueRefusalKind::Canceled,"scene materialization canceled");self.fault=Some(error.clone());return Err(error)}if self.closing{return Err(invalid())}let mut units=0;while !self.complete{let demand=self.next_work_demand();if maximum_units-units<demand{break}units+=demand;if let Err(error)=self.one(demand){self.fault=Some(error.clone());return Err(error)}}Ok(SceneDecodeStep{units,admitted_bytes:self.admitted,complete:self.complete})}
 /// 📤️ Transfers the actual typed owner only after all fields have completed successfully.
 pub fn take_output(&mut self)->Option<NodeGraphScenePayload>{if self.complete&&!self.closing&&self.fault.is_none(){self.owned.payload.take()}else{None}}
 fn closing_inner(&self)->Option<usize>{self.inner.iter().position(|owner|owner.as_ref().is_some_and(|owner|!owner.terminal_is_empty()))}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_copy_byte_demand()}}if let Some(index)=self.closing_inner(){return self.inner[index].as_ref().unwrap().next_close_copy_byte_demand()}wire_dispatch!(&self.wire,next_close_copy_byte_demand)}
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_capacity_byte_demand(body)}}if let Some(index)=self.closing_inner(){return self.inner[index].as_ref().unwrap().next_close_capacity_byte_demand(body)}wire_dispatch!(&self.wire,next_close_capacity_byte_demand,body)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_release_byte_demand()}}if let Some(index)=self.closing_inner(){return self.inner[index].as_ref().unwrap().next_close_release_byte_demand().map_err(pack_error)}wire_dispatch!(&self.wire,next_close_release_byte_demand).map_err(pack_error)}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_depth_demand()}}if let Some(index)=self.closing_inner(){return self.inner[index].as_ref().unwrap().next_close_depth_demand()}wire_dispatch!(&self.wire,next_close_depth_demand)}
 /// ♻️ Original source, partial typed projection, codec output and JSON scaffolds close under independent grants.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let progress=RetainedCloneProgress::default();if self.closed{return Ok(RetainedCloneStep::Complete(progress))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(progress))}if !self.closing{if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"scene close requires one structural depth"))}if let Some(cursor)=self.base64.take(){let parts=cursor.into_parts();if let Base64Input::OwnedText(text)=parts.input{self.owned.pending_text=text}if let Some(bytes)=parts.output{self.owned.pending_bytes=bytes}}let owned=std::mem::take(&mut*self.owned);match ControlledRetirement::new(owned){Ok(owner)=>self.retiring=Some(owner),Err((error,owned))=>{*self.owned=owned;return Err(error)}}self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if let Some(owner)=&mut self.retiring{if !owner.terminal_is_empty(){let step=owner.step(grant)?;return Ok(RetainedCloneStep::Progress(step.progress()))}}
  if let Some(index)=self.closing_inner(){let step=self.inner[index].as_mut().unwrap().close_step(grant).map_err(pack_error)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  let step=wire_dispatch!(&mut self.wire,close_step,grant).map_err(pack_error)?;if wire_dispatch!(&self.wire,terminal_is_empty){self.inner.iter_mut().for_each(|owner|{owner.take();});self.retiring.take();self.closed=true;}Ok(if self.closed{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.base64.is_none()&&self.retiring.is_none()&&wire_dispatch!(&self.wire,terminal_is_empty)&&self.inner.iter().all(Option::is_none)}
}
impl Drop for SceneDecodeCursor<'_>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"scene materialization abandoned before terminal close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owned)}}}}
