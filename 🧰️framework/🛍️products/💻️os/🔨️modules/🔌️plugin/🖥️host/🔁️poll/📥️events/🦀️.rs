//! 📥️ Bounded borrowed Event projection shares the canonical kernel wire contract.
use semio_framework_kernel::{Event,MessageEndpoint,CommandPageCursor,ColdDocumentPairPage,ColdDocumentPairHeader,ColdDocumentPairFrontier};
use semio_framework_actor::instance_lifetime::{ActorInstanceLifetime,ActorInstanceOpenRequest,ActorInstanceCloseRequest,ActorInstanceLifecycleReceipt,ActorInstanceLifecycleAck,ActorUiPatchReceipt};
use semio_framework_kernel_identity::{ActivationEvent,BrokerCapabilityGrant,CapabilityChange,QuotaSchema,RequestOutcome};
use semio_framework_pack_json::{JsonWriteNode,JsonWriteSource};
use semio_framework_value::{Number,ValueError,ValueRefusalKind};

/// 📨️ Lends one immutable input batch without scanning or allocating a mirror.
pub struct EventJsonSource<'a>(pub &'a[Event]);

/// 🎛️ Poll request data narrows the authenticated scalar loan and carries no Source identity.
pub use semio_framework_kernel_identity::PollRequestLimits;
pub struct PollJsonSource<'a>{pub events:&'a[Event],pub limits:&'a PollRequestLimits}

#[derive(Clone,Copy)]
enum Node<'a>{Poll(&'a PollJsonSource<'a>),Limits(&'a PollRequestLimits),Policy(semio_framework_value::RetainedCloneGrant),Null,Events(&'a[Event]),Event(&'a Event),Body(&'a Event),Bytes(&'a[u8]),Text(&'a str),UInt(u64),Quoted(u128),Bool(bool),Assets(&'a[(String,Vec<u8>)]),Asset(&'a(String,Vec<u8>)),Grants(&'a[BrokerCapabilityGrant]),Grant(&'a BrokerCapabilityGrant),Quota(&'a QuotaSchema),Activation(&'a ActivationEvent),ActivationBody(&'a ActivationEvent),Change(&'a CapabilityChange),ChangeBody(&'a CapabilityChange),Endpoint(&'a MessageEndpoint),EndpointBody(&'a MessageEndpoint),Outcome(&'a RequestOutcome),Open(ActorInstanceOpenRequest),Close(ActorInstanceCloseRequest),Lifetime(ActorInstanceLifetime),Receipt(ActorInstanceLifecycleReceipt),Ack(ActorInstanceLifecycleAck),Patch(ActorUiPatchReceipt),Cursor(&'a CommandPageCursor),ColdPage(&'a ColdDocumentPairPage),ColdHeader(&'a ColdDocumentPairHeader),Frontier(&'a ColdDocumentPairFrontier)}

fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed Event JSON path is absent")}
fn uint<'a,T:Into<u64>>(value:T)->Node<'a>{Node::UInt(value.into())}
fn quoted<'a,T:Into<u128>>(value:T)->Node<'a>{Node::Quoted(value.into())}
macro_rules! fields{($index:expr;$($key:literal=>$value:expr),*$(,)?)=>{[$(($key,$value)),*].get($index).copied()};}
macro_rules! optional_fields{($index:expr;$($key:literal=>$value:expr),*$(,)?)=>{[$($value.map(|value|($key,Node::UInt(value)))),*].into_iter().flatten().nth($index)};}

impl<'a> Node<'a>{
 fn tag(self)->Option<&'static str>{Some(match self{
  Self::Event(value)=>match value{Event::InstanceOpen{..}=>"instanceOpen",Event::InstanceClose(_)=>"instanceClose",Event::InstanceLifecycleAck(_)=>"instanceLifecycleAck",Event::Activate{..}=>"activate",Event::SuspendRequest=>"suspendRequest",Event::CapabilityChanged{..}=>"capabilityChanged",Event::QuotaChanged{..}=>"quotaChanged",Event::CommandIngressPage{..}=>"commandIngressPage",Event::ColdDocumentPairPage(_)=>"coldDocumentPairPage",Event::UiIntent{..}=>"uiIntent",Event::SurfaceVisible{..}=>"surfaceVisible",Event::StagedFormRequested{..}=>"stagedFormRequested",Event::SurfaceHidden{..}=>"surfaceHidden",Event::SurfaceResized{..}=>"surfaceResized",Event::PatchAck{..}=>"patchAck",Event::PatchRejected{..}=>"patchRejected",Event::Completed{..}=>"completed",Event::HttpChunk{..}=>"httpChunk",Event::JobProgress{..}=>"jobProgress",Event::JobCompleted{..}=>"jobCompleted",Event::Message{..}=>"message",Event::Timer{..}=>"timer",Event::Wake=>"wake",Event::Request{..}=>"request"},
  Self::Activation(value)=>match value{ActivationEvent::OnCommand{..}=>"onCommand",ActivationEvent::OnViewVisible{..}=>"onViewVisible",ActivationEvent::OnFileType{..}=>"onFileType",ActivationEvent::OnArtifactKind{..}=>"onArtifactKind",ActivationEvent::OnExtensionRequest{..}=>"onExtensionRequest",ActivationEvent::OnStartupFinished=>"onStartupFinished"},
  Self::Change(value)=>match value{CapabilityChange::Granted{..}=>"granted",CapabilityChange::Revoked{..}=>"revoked",CapabilityChange::Narrowed{..}=>"narrowed"},
  Self::Endpoint(value)=>match value{MessageEndpoint::Shell{..}=>"shell",MessageEndpoint::Backbone{..}=>"backbone",MessageEndpoint::PluginInstance{..}=>"pluginInstance",MessageEndpoint::Extension{..}=>"extension",MessageEndpoint::Topic{..}=>"topic"},
  Self::Outcome(value)=>match value{RequestOutcome::Ok(_)=>"ok",RequestOutcome::Err(_)=>"err"},_=>return None})}
 fn unit(self)->bool{matches!(self,Self::Event(Event::SuspendRequest|Event::Wake)|Self::Activation(ActivationEvent::OnStartupFinished))}
 fn field(self,index:usize)->Option<(&'static str,Self)>{
  if let Some(tag)=self.tag(){if self.unit(){return None}return(index==0).then(||(tag,match self{Self::Event(Event::InstanceClose(v))=>Self::Close(*v),Self::Event(Event::InstanceLifecycleAck(v))=>Self::Ack(*v),Self::Event(Event::ColdDocumentPairPage(v))=>Self::ColdPage(v),Self::Event(v)=>Self::Body(v),Self::Activation(v)=>Self::ActivationBody(v),Self::Change(v)=>Self::ChangeBody(v),Self::Endpoint(v)=>Self::EndpointBody(v),Self::Outcome(RequestOutcome::Ok(v)|RequestOutcome::Err(v))=>Self::Bytes(v),_=>unreachable!()}))}
  match self{
   Self::Poll(v)=>fields!(index;"events"=>Self::Events(v.events),"command_page"=>Self::Null,"cold_pair_page"=>Self::Null,"limits"=>Self::Limits(v.limits)),
   Self::Limits(v)=>fields!(index;"retainedPolicy"=>Self::Policy(v.retained_policy),"workFuel"=>uint(v.work_fuel),"fuel"=>uint(v.fuel),"deadlineUs"=>uint(v.deadline_us),"maxEffects"=>uint(v.max_effects),"maxPatchBytes"=>uint(v.max_patch_bytes),"maxFrames"=>uint(v.max_frames)),
   Self::Policy(v)=>fields!(index;"maximumItems"=>uint(v.maximum_items as u64),"maximumCopyBytes"=>uint(v.maximum_copy_bytes as u64),"maximumCapacityBytes"=>uint(v.maximum_capacity_bytes as u64),"maximumReleaseBytes"=>uint(v.maximum_release_bytes as u64),"maximumDepth"=>uint(v.maximum_depth as u64)),
   Self::Body(v)=>match v{
    Event::InstanceOpen{request,app_id,actor,config,assets,capabilities,quotas}=>fields!(index;"request"=>Self::Open(*request),"appId"=>Self::Text(&app_id.0),"actor"=>Self::Text(actor),"config"=>Self::Bytes(config),"assets"=>Self::Assets(assets),"capabilities"=>Self::Grants(capabilities),"quotas"=>Self::Quota(quotas)),
    Event::Activate{reason}=>fields!(index;"reason"=>Self::Activation(reason)),Event::CapabilityChanged{change}=>fields!(index;"change"=>Self::Change(change)),Event::QuotaChanged{quotas}=>fields!(index;"quotas"=>Self::Quota(quotas)),
    Event::CommandIngressPage{cursor,bytes}=>fields!(index;"cursor"=>Self::Cursor(cursor),"bytes"=>Self::Bytes(bytes.as_slice())),
    Event::UiIntent{instance,intent}=>fields!(index;"instance"=>Self::Text(&instance.0),"intent"=>Self::Bytes(intent)),
    Event::SurfaceVisible{surface,body_key,view_state}=>fields!(index;"surface"=>Self::Text(surface),"bodyKey"=>Self::Text(body_key),"viewState"=>Self::Bytes(view_state)),
    Event::StagedFormRequested{instance,surface,body_key,request}=>fields!(index;"instance"=>uint(*instance),"surface"=>Self::Text(surface),"bodyKey"=>Self::Text(body_key),"request"=>Self::Bytes(request)),
    Event::SurfaceHidden{surface}=>fields!(index;"surface"=>Self::Text(surface)),Event::SurfaceResized{surface,width,height}=>fields!(index;"surface"=>Self::Text(surface),"width"=>uint(*width),"height"=>uint(*height)),
    Event::PatchAck{receipt,surface,revision}=>fields!(index;"receipt"=>Self::Patch(*receipt),"surface"=>Self::Text(surface),"revision"=>uint(*revision)),
    Event::PatchRejected{receipt,surface,revision,reason}=>fields!(index;"receipt"=>Self::Patch(*receipt),"surface"=>Self::Text(surface),"revision"=>uint(*revision),"reason"=>Self::Text(reason)),
    Event::Completed{req,result}=>fields!(index;"req"=>uint(req.0),"result"=>Self::Outcome(result)),Event::HttpChunk{req,bytes,done}=>fields!(index;"req"=>uint(req.0),"bytes"=>Self::Bytes(bytes),"done"=>Self::Bool(*done)),
    Event::JobProgress{job,progress}=>if index==0{Some(("job",uint(*job)))}else if index==1{progress.as_deref().map(|bytes|("progress",Self::Bytes(bytes)))}else{None},Event::JobCompleted{job,result}=>fields!(index;"job"=>uint(*job),"result"=>Self::Outcome(result)),
    Event::Message{source,payload}=>fields!(index;"source"=>Self::Endpoint(source),"payload"=>Self::Bytes(payload)),Event::Timer{id}=>fields!(index;"id"=>uint(*id)),Event::Request{req,from,capability,payload}=>fields!(index;"req"=>uint(req.0),"from"=>Self::Endpoint(from),"capability"=>Self::Text(capability),"payload"=>Self::Bytes(payload)),Event::InstanceClose(_)|Event::InstanceLifecycleAck(_)|Event::ColdDocumentPairPage(_)|Event::SuspendRequest|Event::Wake=>None},
   Self::Open(v)=>fields!(index;"activationGeneration"=>quoted(v.activation_generation),"instanceId"=>uint(v.instance_id),"requestSequence"=>uint(v.request_sequence)),
   Self::Close(v)=>fields!(index;"lifetime"=>Self::Lifetime(v.lifetime),"requestSequence"=>uint(v.request_sequence)),
   Self::Lifetime(v)=>fields!(index;"activationGeneration"=>quoted(v.activation_generation),"instanceId"=>uint(v.instance_id),"guestLifetime"=>quoted(v.guest_lifetime)),
   Self::Receipt(v)=>match index{0=>Some(("kind",Self::Text(match v{ActorInstanceLifecycleReceipt::Captured{..}=>"captured",ActorInstanceLifecycleReceipt::Accepted{..}=>"accepted",ActorInstanceLifecycleReceipt::Retired{..}=>"retired"}))),1=>Some(("lifetime",Self::Lifetime(v.lifetime()))),2=>Some(("requestSequence",uint(v.request_sequence()))),3=>v.close_generation().map(|value|("closeGeneration",quoted(value))),_=>None},
   Self::Ack(v)=>fields!(index;"receipt"=>Self::Receipt(v.receipt)),Self::Patch(v)=>fields!(index;"lifetime"=>Self::Lifetime(v.lifetime),"patchSequence"=>quoted(v.patch_sequence)),
   Self::ActivationBody(v)=>match v{ActivationEvent::OnCommand{id}|ActivationEvent::OnViewVisible{id}=>fields!(index;"id"=>Self::Text(id)),ActivationEvent::OnFileType{ext}=>fields!(index;"ext"=>Self::Text(ext)),ActivationEvent::OnArtifactKind{kind}=>fields!(index;"kind"=>Self::Text(kind)),ActivationEvent::OnExtensionRequest{point}=>fields!(index;"point"=>Self::Text(point)),ActivationEvent::OnStartupFinished=>None},
   Self::ChangeBody(v)=>match v{CapabilityChange::Granted{id,grant}|CapabilityChange::Narrowed{id,grant}=>fields!(index;"id"=>Self::Text(&id.0),"grant"=>Self::Grant(grant)),CapabilityChange::Revoked{id}=>fields!(index;"id"=>Self::Text(&id.0))},
   Self::Grant(v)=>match index{0=>Some(("token",quoted(v.token.0))),1=>Some(("id",Self::Text(&v.id.0))),2=>Some(("scope",Self::Text(&v.scope))),3=>v.expires_ms.map(|value|("expiresMs",uint(value))),_=>None},
   Self::Quota(v)=>optional_fields!(index;"memoryBytes"=>v.memory_bytes,"fuelPerTurn"=>v.fuel_per_turn,"turnDeadlineMs"=>v.turn_deadline_ms,"tables"=>v.tables,"mailboxLen"=>v.mailbox_len,"messageBytes"=>v.message_bytes,"outstandingRequests"=>v.outstanding_requests,"timers"=>v.timers,"storageBytes"=>v.storage_bytes,"networkBytesPerMin"=>v.network_bytes_per_min,"uiNodes"=>v.ui_nodes,"patchBytes"=>v.patch_bytes,"patchHz"=>v.patch_hz,"blobResidentBytes"=>v.blob_resident_bytes,"gpuMsPerFrame"=>v.gpu_ms_per_frame,"backgroundMsPerMin"=>v.background_ms_per_min,"logBytesPerMin"=>v.log_bytes_per_min),
   Self::EndpointBody(v)=>match v{MessageEndpoint::Shell{instance}=>fields!(index;"instance"=>Self::Text(&instance.0)),MessageEndpoint::Backbone{uri}=>fields!(index;"uri"=>Self::Text(uri)),MessageEndpoint::PluginInstance{id}=>fields!(index;"id"=>Self::Text(&id.0)),MessageEndpoint::Extension{id}=>fields!(index;"id"=>Self::Text(id)),MessageEndpoint::Topic{name}=>fields!(index;"name"=>Self::Text(name))},
   Self::Cursor(v)=>fields!(index;"owner"=>uint(v.owner),"generation"=>uint(v.generation),"commandIndex"=>uint(v.command_index),"commandCount"=>uint(v.command_count),"instance"=>uint(v.instance),"seq"=>uint(v.seq),"kind"=>uint(v.kind),"pageIndex"=>uint(v.page_index),"pageCount"=>uint(v.page_count),"itemCount"=>uint(v.item_count),"metadata"=>uint(v.metadata)),
   Self::ColdPage(v)=>fields!(index;"header"=>Self::ColdHeader(&v.header),"pageIndex"=>uint(v.page_index),"bytes"=>Self::Bytes(&v.bytes)),
   Self::ColdHeader(v)=>fields!(index;"lifetime"=>Self::Lifetime(v.lifetime),"transferGeneration"=>uint(v.transfer_generation),"descriptorSha256"=>Self::Bytes(&v.descriptor_sha256),"baselineFrontier"=>Self::Frontier(&v.baseline_frontier),"packSha256"=>Self::Bytes(&v.pack_sha256),"sprSha256"=>Self::Bytes(&v.spr_sha256),"aggregateSha256"=>Self::Bytes(&v.aggregate_sha256),"packLength"=>uint(v.pack_length),"sprLength"=>uint(v.spr_length),"pageCount"=>uint(v.page_count)),
   Self::Frontier(v)=>fields!(index;"documentId"=>Self::Text(&v.document_id),"headEditOrdinal"=>uint(v.head_edit_ordinal),"headEditId"=>Self::Text(&v.head_edit_id),"lastCommitSeq"=>uint(v.last_commit_seq),"chainSha256"=>Self::Bytes(&v.chain_sha256)),_=>None,
  }
 }
 fn child(self,index:usize)->Option<Self>{match self{Self::Events(v)=>v.get(index).map(Self::Event),Self::Bytes(v)=>v.get(index).map(|v|uint(*v)),Self::Assets(v)=>v.get(index).map(Self::Asset),Self::Asset(v)=>match index{0=>Some(Self::Text(&v.0)),1=>Some(Self::Bytes(&v.1)),_=>None},Self::Grants(v)=>v.get(index).map(Self::Grant),_=>self.field(index).map(|(_,value)|value)}}
 fn view(self)->JsonWriteNode<'a>{match self{Self::Null=>JsonWriteNode::Null,Self::Text(v)=>JsonWriteNode::String(v),Self::UInt(v)=>JsonWriteNode::Number(Number::UInt(v)),Self::Quoted(v)=>JsonWriteNode::QuotedUnsigned(v),Self::Bool(v)=>JsonWriteNode::Bool(v),Self::Events(v)=>JsonWriteNode::Array(v.len()),Self::Bytes(v)=>JsonWriteNode::Array(v.len()),Self::Assets(v)=>JsonWriteNode::Array(v.len()),Self::Asset(_)=>JsonWriteNode::Array(2),Self::Grants(v)=>JsonWriteNode::Array(v.len()),_ if self.unit()=>JsonWriteNode::String(self.tag().expect("unit has canonical tag")),_=>JsonWriteNode::Object((0..=17).take_while(|index|self.field(*index).is_some()).count())}}
}
impl EventJsonSource<'_>{fn at(&self,path:&[usize])->Result<Node<'_>,ValueError>{at(Node::Events(self.0),path)}}
impl JsonWriteSource for EventJsonSource<'_>{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{Ok(self.at(path)?.view())}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.at(path)?.field(index).map(|(key,_)|key).ok_or_else(absent)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

fn at<'a>(mut node:Node<'a>,path:&[usize])->Result<Node<'a>,ValueError>{for index in path{validate(node)?;node=node.child(*index).ok_or_else(absent)?}validate(node)?;Ok(node)}
fn validate(node:Node<'_>)->Result<(),ValueError>{if match node{Node::Open(v)=>!v.is_valid(),Node::Close(v)=>!v.is_valid(),Node::Lifetime(v)=>!v.is_valid(),Node::Receipt(v)=>!v.is_valid(),Node::Patch(v)=>!v.is_valid(),_=>false}{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"borrowed Event has invalid lifecycle authority"))}else{Ok(())}}
impl JsonWriteSource for PollJsonSource<'_>{fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{Ok(at(Node::Poll(self),path)?.view())}fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{at(Node::Poll(self),path)?.field(index).map(|(key,_)|key).ok_or_else(absent)}}

pub(super) fn limits_node<'a>(limits:&'a PollRequestLimits,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{Ok(at(Node::Limits(limits),path)?.view())}
pub(super) fn limits_key<'a>(limits:&'a PollRequestLimits,path:&[usize],index:usize)->Result<&'a str,ValueError>{at(Node::Limits(limits),path)?.field(index).map(|(key,_)|key).ok_or_else(absent)}
