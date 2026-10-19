//! 🎫️ Exact scoped request ownership shares work while preserving each consumer's admission.
use super::fixed_key_slots::{FixedKeySlot,FixedKeySlots};
use crate::{RetainedCloneGrant,RetainedCloneProgress};

/// 🔏️ Complete framed identities use the existing revision content-addressing authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ContentRequestKey{digest:[u8;32],length:usize}
impl ContentRequestKey{
 /// 🪪️ Ordered segment lengths preserve full source namespaces and opaque text without aliases.
 pub fn from_segments(segments:&[&[u8]])->Option<Self>{if segments.is_empty()||segments.len()>32{return None;}let length=segments.iter().try_fold(0usize,|length,segment|length.checked_add(segment.len()).and_then(|length|length.checked_add(8)))?;if length>131_072{return None;}let mut hash=semio_framework_hash::Hasher::new();for segment in segments{hash.update(&(segment.len()as u64).to_le_bytes());hash.update(segment);}Some(Self{digest:*hash.finalize().as_bytes(),length})}
 /// #️⃣ Lends the fixed selector while original full text remains under the request owner.
 pub fn digest(&self)->&[u8;32]{&self.digest}
}

/// 🔐️ Framed selectors advance under copy credit without copying original identity payloads.
pub struct ContentRequestKeyCursor{hash:semio_framework_hash::Hasher,lengths:[usize;32],count:usize,segment:usize,offset:usize,length:usize,key:Option<ContentRequestKey>}
impl ContentRequestKeyCursor{
 /// 📏️ A selector birth retains only fixed segment lengths and the first-party hash state.
 pub fn new(segments:&[&[u8]])->Option<Self>{if segments.is_empty()||segments.len()>32{return None;}let mut lengths=[0;32];let mut length=0usize;for(index,segment)in segments.iter().enumerate(){length=length.checked_add(segment.len())?.checked_add(8)?;lengths[index]=segment.len();}if length>131_072{return None;}Some(Self{hash:semio_framework_hash::Hasher::new(),lengths,count:segments.len(),segment:0,offset:0,length,key:None})}
 /// 🧾️ A fixed framed chunk consumes the same currency as retained original-text copying.
 pub fn step(&mut self,segments:&[&[u8]],grant:RetainedCloneGrant)->Result<RequestTextStep,&'static str>{
  if segments.len()!=self.count||segments.iter().enumerate().any(|(index,segment)|segment.len()!=self.lengths[index]){return Err("request-selector-input-changed");}if self.key.is_some(){return Ok(RequestTextStep{fault:None,ready:true,..Default::default()});}if grant.maximum_items==0||grant.maximum_copy_bytes==0||grant.maximum_depth==0{return Ok(Default::default());}
  let prefix=(self.lengths[self.segment]as u64).to_le_bytes();let data=if self.offset<8{&prefix[self.offset..]}else{&segments[self.segment][self.offset-8..]};let bytes=data.len().min(grant.maximum_copy_bytes);self.hash.update(&data[..bytes]);self.offset+=bytes;
  if self.offset==8+self.lengths[self.segment]{self.segment+=1;self.offset=0;}if self.segment==self.count{self.key=Some(ContentRequestKey{digest:*self.hash.finalize().as_bytes(),length:self.length});}
  Ok(RequestTextStep{fault:None,ready:self.key.is_some(),progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}})
 }
 /// 🔎️ Only a fully funded framed selector can address an exact-text retained owner.
 pub fn key(&self)->Option<ContentRequestKey>{self.key}
}

struct ConsumerText{original:String,length:usize,verified:bool,complete:bool,retiring:bool,comparison:crate::ExactKeyCursor}

/// 📏️ Original text preparation quotes every independent work currency before custody changes.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RequestTextDemand{pub items:usize,pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}
impl RequestTextDemand{
 /// 🎟️ A zero or short currency leaves the original cursor and claim unchanged.
 pub fn admits(self,grant:RetainedCloneGrant)->bool{grant.maximum_items>=self.items&&grant.maximum_copy_bytes>=self.copy_bytes&&grant.maximum_capacity_bytes>=self.capacity_bytes&&grant.maximum_release_bytes>=self.release_bytes&&grant.maximum_depth>=self.depth}
}
/// 🧾️ Each text cursor returns its actual allocation, comparison, copy or release receipt.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RequestTextStep{pub ready:bool,pub progress:RetainedCloneProgress,pub fault:Option<&'static str>}

/// 🧾️ Original source authority survives progress-overlay acknowledgement and repeated render polls.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RequestSourceVersion{pub base_revision:u64,pub generation:u64,pub canonical_revision:[u8;32]}
/// 🪪️ Original document and consumer-config authority keep borrowed selector input immutable.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RequestCandidateVersion{pub source:RequestSourceVersion,pub consumer_generation:u64,pub consumer_revision:[u8;32]}
struct PendingSelector{version:RequestCandidateVersion,cursor:ContentRequestKeyCursor}
/// 🔏️ A consumer-owned selector is published only after all framed source bytes were charged.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RequestSelectorStep{pub key:Option<ContentRequestKey>,pub progress:RetainedCloneProgress}
/// 🔎️ Snapshot positions are derived operands after bounded exact original-text comparison.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RequestLookupOutcome{Searching,Matched,Missing}
#[derive(Clone,Copy)]
struct SourceLookup{version:RequestSourceVersion,cursor:crate::ExactKeyCursor}
/// 🧾️ Original request-source lookup charges each comparison chunk and retains no copied identifiers.
#[derive(Clone,Copy,Debug)]
pub struct RequestLookupStep{pub position:usize,pub outcome:RequestLookupOutcome,pub progress:RetainedCloneProgress}
/// 🚦️ Terminal metadata never copies the original worker's fault payload.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RequestTerminalKind{Validated,Faulted,Cancelled}
/// 📬️ The exact original job and source version own one terminal receipt.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RequestTerminalReceipt{pub source:RequestSourceVersion,pub job:u64,pub kind:RequestTerminalKind}

/// 🪪️ Complete bounded identity bytes remain exact, with no digest or ordinal alias.
#[derive(Clone,Copy)]
pub struct FixedRequestKey<const N:usize>{bytes:[u8;N],len:usize}
impl<const N:usize> FixedRequestKey<N>{
 /// 📥️ Admission refuses excess identity bytes before copying the owner.
 pub fn from_bytes(bytes:&[u8])->Option<Self>{if bytes.is_empty()||bytes.len()>N{return None;}let mut key=Self{bytes:[0;N],len:bytes.len()};key.bytes[..bytes.len()].copy_from_slice(bytes);Some(key)}
 /// 🔎️ Lends the complete admitted identity.
 pub fn bytes(&self)->&[u8]{&self.bytes[..self.len]}
}
impl<const N:usize> PartialEq for FixedRequestKey<N>{fn eq(&self,other:&Self)->bool{self.bytes()==other.bytes()}}
impl<const N:usize> Eq for FixedRequestKey<N>{}
impl<const N:usize> std::fmt::Debug for FixedRequestKey<N>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_tuple("FixedRequestKey").field(&self.bytes()).finish()}}

/// 📬️ A successful consumer rebind retains the prior owner for child-first retirement.
#[derive(Clone,Copy,Debug)]
pub struct FixedRequestAdmission{pub owner:FixedKeySlot,pub fresh:bool,pub retired:Option<FixedKeySlot>}
/// 🎟️ Fixed ownership edits return their admitted metadata receipt alongside the exact lease.
#[derive(Clone,Copy,Debug)]
pub struct FixedRequestAdmissionStep{pub admission:FixedRequestAdmission,pub progress:RetainedCloneProgress}
/// 🪦️ Consumer departure returns exact old authority and its admitted fixed metadata receipt.
#[derive(Clone,Copy,Debug)]
pub struct FixedRequestDepartureStep{pub owner:Option<FixedKeySlot>,pub progress:RetainedCloneProgress}

/// 🧺️ Request keys and consumer bindings are fixed; payload and child cleanup stay with the caller.
pub struct FixedRequestOwners<K,W,const R:usize,const C:usize>{owners:FixedKeySlots<(u32,K),R>,keys:[Option<(u32,K)>;R],consumers:FixedKeySlots<(u32,W),C>,bindings:[Option<FixedKeySlot>;C],binding_ready:[bool;C],consumer_closing:[bool;C],pending_bindings:[Option<FixedKeySlot>;C],selectors:[Vec<PendingSelector>;C],references:[usize;R],names:[Option<ConsumerText>;C],request_names:[Option<ConsumerText>;R],closing:[bool;R],executions:[Option<(RequestSourceVersion,u64)>;R],terminals:[Option<RequestTerminalReceipt>;R],lookups:[Option<SourceLookup>;R]}
impl<K:Copy+Eq,W:Copy+Eq,const R:usize,const C:usize> FixedRequestOwners<K,W,R,C>{
 /// 🌱️ Installs declared owner and consumer capacities without payload allocation.
 pub fn new()->Self{Self{owners:Default::default(),keys:[None;R],consumers:Default::default(),bindings:[None;C],binding_ready:[false;C],consumer_closing:[false;C],pending_bindings:[None;C],selectors:std::array::from_fn(|_|Vec::new()),references:[0;R],names:std::array::from_fn(|_|None),request_names:std::array::from_fn(|_|None),closing:[false;R],executions:[None;R],terminals:[None;R],lookups:[None;R]}}
 /// 🎯️ A terminal original source cannot resubmit merely because its overlay slot was acknowledged.
 pub fn begin_execution(&mut self,owner:FixedKeySlot,source:RequestSourceVersion,job:u64)->bool{if !self.ready(owner)||self.terminal(owner,source).is_some()||self.executions[owner.index()].is_some(){return false;}self.executions[owner.index()]=Some((source,job));true}
 /// 📥️ Only the current original job may publish a source-exact terminal receipt.
 pub fn record_terminal(&mut self,owner:FixedKeySlot,source:RequestSourceVersion,job:u64,kind:RequestTerminalKind)->bool{if self.key(owner).is_none()||self.executions[owner.index()]!=Some((source,job)){return false;}self.terminals[owner.index()]=Some(RequestTerminalReceipt{source,job,kind});self.executions[owner.index()]=None;true}
 /// 🔎️ Reads terminal status without borrowing or copying the original fault body.
 pub fn terminal(&self,owner:FixedKeySlot,source:RequestSourceVersion)->Option<RequestTerminalReceipt>{self.key(owner)?;self.terminals[owner.index()].filter(|terminal|terminal.source==source)}
 /// 🔁️ Explicit retry clears only the addressed original terminal source.
 pub fn retry(&mut self,owner:FixedKeySlot,source:RequestSourceVersion)->bool{if self.terminal(owner,source).is_none(){return false;}self.terminals[owner.index()]=None;true}
 /// 📥️ Full physical source text remains under one granted original owner across all sharing panes.
 pub fn prepare_request(&mut self,scope:u32,key:K,original:&str,grant:RetainedCloneGrant)->Result<RequestTextStep,&'static str>{
  Self::validate_text(scope,original)?;let existing=self.owners.lookup((scope,key));if existing.is_some_and(|owner|self.closing[owner.index()]){return Err("request-owner-retiring");}
  if let Some(owner)=existing{return Self::prepare_text(&mut self.request_names[owner.index()],original,grant);}
  if !self.owners.can_claim((scope,key)){return Err("request-owner-capacity");}let demand=self.request_preparation_demand(scope,key,original.len());if !demand.admits(grant){return Ok(Default::default());}
  let mut text=None;let mut step=Self::prepare_text(&mut text,original,grant)?;step.progress.copied_bytes=demand.copy_bytes;let Some(owner)=self.owners.claim((scope,key))else{return Err("request-owner-capacity");};let index=owner.index();self.keys[index]=Some((scope,key));self.request_names[index]=text;Ok(step)
 }
 fn validate_text(scope:u32,original:&str)->Result<(),&'static str>{if scope==0||original.is_empty()||original.len()>65_536{Err("request-original-identity-limit")}else{Ok(())}}
 fn text_demand(text:Option<&ConsumerText>,length:usize)->RequestTextDemand{match text{None=>RequestTextDemand{items:1,capacity_bytes:length,depth:1,..Default::default()},Some(text)=>RequestTextDemand{items:1,copy_bytes:if text.complete{crate::ExactKeyCursor::fixed_copy_bytes()+2}else{4.min(length.saturating_sub(text.original.len()))},depth:1,..Default::default()}}}
 fn prepare_text(target:&mut Option<ConsumerText>,original:&str,grant:RetainedCloneGrant)->Result<RequestTextStep,&'static str>{
  let demand=Self::text_demand(target.as_ref(),original.len());if !demand.admits(grant){return Ok(Default::default());}
  let Some(text)=target.as_mut()else{let mut stored=String::new();stored.try_reserve_exact(original.len()).map_err(|_|"request-original-allocation")?;let bytes=stored.capacity();*target=Some(ConsumerText{original:stored,length:original.len(),verified:false,complete:false,retiring:false,comparison:crate::ExactKeyCursor::new()});return Ok(RequestTextStep{fault:None,ready:false,progress:RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}});};
  if text.retiring||text.length!=original.len(){return Err("request-original-selector-collision");}
  if text.complete{if text.verified{text.comparison=crate::ExactKeyCursor::new();}let step=text.comparison.advance(text.original.as_bytes(),Some(original.as_bytes()),grant).map_err(|_|"request-original-comparison-invariant")?;if text.comparison.next_index()!=0{return Ok(RequestTextStep{fault:Some("request-original-selector-collision"),ready:false,progress:step.progress()});}text.verified=text.comparison.found()==Some(0);return Ok(RequestTextStep{fault:None,ready:text.verified,progress:step.progress()});}
  let start=text.original.len();let mut end=original.len().min(start.saturating_add(grant.maximum_copy_bytes));while end>start&&!original.is_char_boundary(end){end-=1;}if end==start{return Ok(Default::default());}text.original.push_str(&original[start..end]);text.complete=end==original.len();text.verified=text.complete;Ok(RequestTextStep{fault:None,ready:text.complete,progress:RetainedCloneProgress{copied_items:1,copied_bytes:end-start,..Default::default()}})
 }
 /// 📐️ The next original source turn can be admitted without mutating custody.
 pub fn request_preparation_demand(&self,scope:u32,key:K,length:usize)->RequestTextDemand{let owner=self.lookup(scope,key);let mut demand=Self::text_demand(owner.and_then(|owner|self.request_names[owner.index()].as_ref()),length);if owner.is_none(){demand.copy_bytes=std::mem::size_of::<(u32,K)>()*2+std::mem::size_of::<FixedKeySlot>();}demand}
 /// 📐️ The next original consumer turn can be admitted without mutating custody.
 pub fn consumer_preparation_demand(&self,scope:u32,consumer:W,length:usize)->RequestTextDemand{let slot=self.consumer_index(scope,consumer);let mut demand=Self::text_demand(slot.and_then(|index|self.names[index].as_ref()),length);if slot.is_none(){demand.copy_bytes=std::mem::size_of::<(u32,W)>()+std::mem::size_of::<FixedKeySlot>();}demand}
 /// 🔎️ Numerical execution resolves this complete original id against its own admitted snapshot.
 pub fn request_name(&self,owner:FixedKeySlot)->Option<&str>{self.key(owner)?;self.request_names[owner.index()].as_ref().filter(|text|text.complete).map(|text|text.original.as_str())}
 /// 🧭️ A new original source revision cannot reuse a prior snapshot's numerical position.
 pub fn source_lookup(&self,owner:FixedKeySlot,version:RequestSourceVersion)->Option<(usize,RequestLookupOutcome)>{self.key(owner)?;let lookup=self.lookups[owner.index()]?;(lookup.version==version).then_some((lookup.cursor.next_index(),Self::lookup_outcome(lookup.cursor)))}
 fn lookup_outcome(cursor:crate::ExactKeyCursor)->RequestLookupOutcome{if !cursor.ready(){RequestLookupOutcome::Searching}else if cursor.found().is_some(){RequestLookupOutcome::Matched}else{RequestLookupOutcome::Missing}}
 /// 🔬️ The shared exact-key primitive advances against one original snapshot candidate under its manager's version.
 pub fn source_lookup_step(&mut self,owner:FixedKeySlot,version:RequestSourceVersion,candidate:Option<&str>,grant:RetainedCloneGrant)->Result<RequestLookupStep,&'static str>{
  self.key(owner).ok_or("request-lookup-owner-missing")?;let previous=self.lookups[owner.index()];if previous.is_none_or(|lookup|lookup.version!=version){let bytes=std::mem::size_of::<SourceLookup>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(RequestLookupStep{position:0,outcome:RequestLookupOutcome::Searching,progress:Default::default()});}self.lookups[owner.index()]=Some(SourceLookup{version,cursor:crate::ExactKeyCursor::new()});return Ok(RequestLookupStep{position:0,outcome:RequestLookupOutcome::Searching,progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}});}
  let original=self.request_names[owner.index()].as_ref().filter(|text|text.complete).ok_or("request-original-preparation-pending")?.original.as_bytes();let lookup=self.lookups[owner.index()].as_mut().unwrap();let step=lookup.cursor.advance(original,candidate.map(str::as_bytes),grant).map_err(|_|"request-source-lookup-invariant")?;Ok(RequestLookupStep{position:lookup.cursor.next_index(),outcome:Self::lookup_outcome(lookup.cursor),progress:step.progress()})
 }
 /// ♻️ The original source allocation and selector closure are quoted independently.
 pub fn request_close_demand(&self,owner:FixedKeySlot)->Option<RequestTextDemand>{self.key(owner)?;self.request_names[owner.index()].as_ref().map(|text|RequestTextDemand{items:1,copy_bytes:std::mem::size_of::<Option<ConsumerText>>(),release_bytes:text.original.capacity(),depth:1,..Default::default()})}
 /// ♻️ Partial consumer custody has the same observable close quote as completed text.
 pub fn consumer_close_demand(&self,index:usize)->Option<RequestTextDemand>{self.consumer_at(index)?;let bytes=self.selectors[index].capacity()*std::mem::size_of::<PendingSelector>();Some(if bytes!=0{RequestTextDemand{items:1,copy_bytes:std::mem::size_of::<Vec<PendingSelector>>(),release_bytes:bytes,depth:1,..Default::default()}}else{RequestTextDemand{items:1,copy_bytes:std::mem::size_of::<(u32,W)>()+std::mem::size_of::<FixedKeySlot>()*2+std::mem::size_of::<Option<ConsumerText>>()+std::mem::size_of::<bool>(),release_bytes:self.names[index].as_ref().map_or(0,|text|text.original.capacity()),depth:1,..Default::default()}})}
 /// 🧭️ A provisional request cannot execute until both original source and a consumer are complete.
 pub fn ready(&self,owner:FixedKeySlot)->bool{if self.key(owner).is_none()||self.closing[owner.index()]||self.request_names[owner.index()].as_ref().is_some_and(|text|!text.complete){return false;}self.bindings.iter().enumerate().any(|(index,binding)|*binding==Some(owner)&&self.binding_ready[index]&&self.names[index].as_ref().is_none_or(|text|text.complete&&!text.retiring))}
 /// ♻️ Original request text retires with a funded allocation before the exact selector is released.
 pub fn retire_request_step(&mut self,owner:FixedKeySlot,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{self.key(owner)?;if self.references[owner.index()]!=0{return Some(Default::default());}let demand=self.request_close_demand(owner)?;if !demand.admits(grant){return Some(Default::default());}self.request_names[owner.index()]=None;Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})}
 /// 🪦️ Retained identity custody is observable independently of bound request children.
 pub fn has_consumer(&self,scope:u32)->bool{(0..C).any(|index|self.consumer_at(index).is_some_and(|(bound_scope,_)|bound_scope==scope))}
 /// 🪦️ Typed pending payloads return before the corresponding original consumer name.
 pub fn consumer_retiring(&self,index:usize)->bool{self.consumer_closing.get(index).copied().unwrap_or(false)}
 /// 📥️ Consumer text is copied in granted UTF8 chunks before its request binding becomes live.
 pub fn prepare_consumer(&mut self,scope:u32,consumer:W,original:&str,grant:RetainedCloneGrant)->Result<RequestTextStep,&'static str>{
  Self::validate_text(scope,original)?;if let Some(index)=self.consumer_index(scope,consumer){return Self::prepare_text(&mut self.names[index],original,grant);}
  if !self.consumers.can_claim((scope,consumer)){return Err("request-consumer-capacity");}let demand=self.consumer_preparation_demand(scope,consumer,original.len());if !demand.admits(grant){return Ok(Default::default());}let mut text=None;let mut step=Self::prepare_text(&mut text,original,grant)?;step.progress.copied_bytes=demand.copy_bytes;let slot=self.consumers.claim((scope,consumer)).ok_or("request-consumer-capacity")?;self.names[slot.index()]=text;Ok(step)
 }
 /// 🔎️ Exact source custody protects equal fixed selectors from consumer-name aliasing.
 pub fn consumer_name(&self,index:usize)->Option<&str>{self.names.get(index)?.as_ref().filter(|name|name.complete&&!name.retiring).map(|name|name.original.as_str())}
 /// 🎫️ The source owner stages a typed demand beside this exact consumer position.
 pub fn consumer_index(&self,scope:u32,consumer:W)->Option<usize>{self.consumers.lookup((scope,consumer)).map(FixedKeySlot::index)}
 /// 🎟️ A typed fixed consumer without opaque text still receives an original funded claim.
 pub fn claim_consumer_controlled(&mut self,scope:u32,consumer:W,grant:RetainedCloneGrant)->Option<RequestTextStep>{if scope==0{return None;}if let Some(index)=self.consumer_index(scope,consumer){return (!self.consumer_closing[index]).then_some(RequestTextStep{fault:None,ready:true,..Default::default()});}let bytes=std::mem::size_of::<(u32,W)>()+std::mem::size_of::<FixedKeySlot>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Some(Default::default());}self.consumers.claim((scope,consumer))?;Some(RequestTextStep{fault:None,ready:true,progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}})}
 /// 📏️ The original consumer claims selector capacity before any source hashing takes place.
 pub fn selector_demand(&self,scope:u32,consumer:W,version:RequestCandidateVersion)->Option<RequestTextDemand>{let index=self.consumer_index(scope,consumer)?;let selectors=&self.selectors[index];Some(if selectors.capacity()==0{RequestTextDemand{items:1,copy_bytes:std::mem::size_of::<Vec<PendingSelector>>(),capacity_bytes:std::mem::size_of::<PendingSelector>(),depth:1,..Default::default()}}else if selectors.first().is_none_or(|selector|selector.version!=version){RequestTextDemand{items:1,copy_bytes:std::mem::size_of::<PendingSelector>(),depth:1,..Default::default()}}else{RequestTextDemand{items:1,copy_bytes:1,depth:1,..Default::default()}})}
 /// 🔏️ Borrowed immutable source/config versions drive one funded framed-selector turn.
 pub fn prepare_selector(&mut self,scope:u32,consumer:W,version:RequestCandidateVersion,segments:&[&[u8]],grant:RetainedCloneGrant)->Result<RequestSelectorStep,&'static str>{
  let index=self.consumer_index(scope,consumer).ok_or("request-selector-consumer-missing")?;if self.consumer_retiring(index){return Err("request-consumer-retiring");}let demand=self.selector_demand(scope,consumer,version).unwrap();if !demand.admits(grant){return Ok(Default::default());}
  let selectors=&mut self.selectors[index];if selectors.capacity()==0{selectors.try_reserve_exact(1).map_err(|_|"request-selector-allocation")?;return Ok(RequestSelectorStep{key:None,progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:selectors.capacity()*std::mem::size_of::<PendingSelector>(),..Default::default()}});}
  if selectors.first().is_none_or(|selector|selector.version!=version){let cursor=ContentRequestKeyCursor::new(segments).ok_or("request-selector-identity-limit")?;selectors.clear();selectors.push(PendingSelector{version,cursor});return Ok(RequestSelectorStep{key:None,progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}});}
  let selector=selectors.first_mut().unwrap();let step=selector.cursor.step(segments,grant)?;Ok(RequestSelectorStep{key:selector.cursor.key(),progress:step.progress})
 }
 /// 🔎️ Only the exact original source/config version can reuse its completed selector.
 pub fn selector(&self,scope:u32,consumer:W,version:RequestCandidateVersion)->Option<ContentRequestKey>{let index=self.consumer_index(scope,consumer)?;let selector=self.selectors[index].first()?;(selector.version==version).then(||selector.cursor.key()).flatten()}
 /// ♻️ One funded original text allocation returns before its consumer claim is reusable.
 pub fn retire_consumer_step(&mut self,scope:u32,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{for index in 0..C{let Some((bound_scope,consumer))=self.consumer_at(index)else{continue;};if bound_scope!=scope||!self.consumer_closing[index]{continue;}let demand=self.consumer_close_demand(index).unwrap();if !demand.admits(grant){return Some(Default::default());}if self.selectors[index].capacity()!=0{self.selectors[index]=Vec::new();return Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()});}self.names[index]=None;self.consumer_closing[index]=false;let lease=self.consumers.lookup((scope,consumer)).unwrap();assert!(self.consumers.release((scope,consumer),lease));return Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()});}None}
 /// 🎯️ Refusal leaves an existing consumer binding untouched.
 pub fn admit(&mut self,scope:u32,consumer:W,key:K)->Option<FixedRequestAdmission>{
  self.bind(scope,consumer,key,false)
 }
 /// 🎫️ A provisional binding protects granted identity preparation while remaining non-executable.
 pub fn stage(&mut self,scope:u32,consumer:W,key:K)->Option<FixedRequestAdmission>{self.bind(scope,consumer,key,true)}
 /// 📏️ Fixed owner and consumer metadata have one domain-neutral admission quote.
 pub fn binding_demand()->RequestTextDemand{RequestTextDemand{items:1,copy_bytes:std::mem::size_of::<(u32,K)>()*2+std::mem::size_of::<(u32,W)>()*2+std::mem::size_of::<FixedKeySlot>()*4+std::mem::size_of::<usize>()*4,depth:1,..Default::default()}}
 /// 🧾️ Provisional binding changes are refused before any independent work currency is spent.
 pub fn stage_controlled(&mut self,scope:u32,consumer:W,key:K,grant:RetainedCloneGrant)->Option<FixedRequestAdmissionStep>{let demand=Self::binding_demand();if !demand.admits(grant){return None;}let admission=self.stage(scope,consumer,key)?;Some(FixedRequestAdmissionStep{admission,progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})}
 /// 🧾️ Verified promotion charges the original binding owner without replacing a sibling request.
 pub fn admit_controlled(&mut self,scope:u32,consumer:W,key:K,grant:RetainedCloneGrant)->Option<FixedRequestAdmissionStep>{let demand=Self::binding_demand();if !demand.admits(grant){return None;}let admission=self.admit(scope,consumer,key)?;Some(FixedRequestAdmissionStep{admission,progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})}
 fn drop_reference(&mut self,owner:FixedKeySlot)->Option<FixedKeySlot>{self.references[owner.index()]-=1;if self.references[owner.index()]==0{self.closing[owner.index()]=true;Some(owner)}else{None}}
 fn bind(&mut self,scope:u32,consumer:W,key:K,pending:bool)->Option<FixedRequestAdmission>{
  if scope==0{return None;}let existing=self.owners.lookup((scope,key));
  if existing.is_some_and(|owner|self.closing[owner.index()]||(!pending&&self.request_names[owner.index()].as_ref().is_some_and(|text|!text.complete||!text.verified))){return None;}
  if self.consumer_index(scope,consumer).is_some_and(|index|self.consumer_closing[index]||self.names[index].as_ref().is_some_and(|name|!pending&&(!name.complete||!name.verified))){return None;}
  let owner=self.owners.claim((scope,key))?;let fresh=existing.is_none();let Some(binding)=self.consumers.claim((scope,consumer))else{if fresh{self.owners.release((scope,key),owner);}return None;};let index=binding.index();if fresh{self.keys[owner.index()]=Some((scope,key));}
  if pending{
   if self.bindings[index]==Some(owner){let previous=self.pending_bindings[index].take();let retired=previous.and_then(|previous|self.drop_reference(previous));return Some(FixedRequestAdmission{owner,fresh,retired});}
   if self.pending_bindings[index]==Some(owner){return Some(FixedRequestAdmission{owner,fresh,retired:None});}
   let previous=self.pending_bindings[index].replace(owner);self.references[owner.index()]+=1;let retired=previous.and_then(|previous|self.drop_reference(previous));return Some(FixedRequestAdmission{owner,fresh,retired});
  }
  let candidate=self.pending_bindings[index].take();let previous=self.bindings[index].replace(owner);self.binding_ready[index]=true;if candidate!=Some(owner)&&previous!=Some(owner){self.references[owner.index()]+=1;}
  let retired_candidate=candidate.filter(|candidate|*candidate!=owner).and_then(|candidate|self.drop_reference(candidate));let retired_previous=previous.filter(|previous|*previous!=owner).and_then(|previous|self.drop_reference(previous));Some(FixedRequestAdmission{owner,fresh,retired:retired_previous.or(retired_candidate)})
 }
 /// 🔎️ Exact request lookup retains its original slot generation.
 pub fn lookup(&self,scope:u32,key:K)->Option<FixedKeySlot>{self.owners.lookup((scope,key))}
 /// 👁️ One consumer reads only its current exact request owner.
 pub fn binding(&self,scope:u32,consumer:W)->Option<FixedKeySlot>{let index=self.consumers.lookup((scope,consumer))?.index();self.pending_bindings[index].or(self.bindings[index])}
 /// 👁️ A provisional candidate cannot read a sibling's result until full text comparison is promoted.
 pub fn live_binding(&self,scope:u32,consumer:W)->Option<FixedKeySlot>{let slot=self.consumers.lookup((scope,consumer))?;self.binding_ready[slot.index()].then(||self.bindings[slot.index()]).flatten()}
 /// 🧾️ A denied departure preserves both original live and provisional sibling references.
 pub fn remove_consumer_controlled(&mut self,scope:u32,consumer:W,grant:RetainedCloneGrant)->Option<FixedRequestDepartureStep>{let index=self.consumer_index(scope,consumer)?;if self.consumer_closing[index]{return None;}let demand=Self::binding_demand();if !demand.admits(grant){return None;}let owner=self.remove_consumer(scope,consumer);Some(FixedRequestDepartureStep{owner,progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})}
 /// 🧵️ Departure returns the removed binding; zero remaining references authorizes child retirement.
 pub fn remove_consumer(&mut self,scope:u32,consumer:W)->Option<FixedKeySlot>{let binding=self.consumers.lookup((scope,consumer))?;let index=binding.index();let owner=self.bindings[index].take();let pending=self.pending_bindings[index].take();self.binding_ready[index]=false;if let Some(name)=self.names[index].as_mut(){name.retiring=true;}if self.names[index].is_some()||self.selectors[index].capacity()!=0{self.consumer_closing[index]=true;}else{assert!(self.consumers.release((scope,consumer),binding));}if let Some(owner)=owner{self.drop_reference(owner);}if let Some(pending)=pending{self.drop_reference(pending);}owner.or(pending)}
 /// 📊️ Counts current consumers while closing owners remain retained.
 pub fn references(&self,owner:FixedKeySlot)->usize{self.key(owner).map_or(0,|_|self.references[owner.index()])}
 /// 🪪️ Lends an owner only under its exact current generation.
 pub fn key(&self,owner:FixedKeySlot)->Option<(u32,K)>{let key=*self.keys.get(owner.index())?.as_ref()?;(self.owners.lookup(key)==Some(owner)).then_some(key)}
 /// 🧭️ Fair bounded scans visit scoped requests without fabricating key indices.
 pub fn at(&self,index:usize)->Option<(FixedKeySlot,u32,K)>{let(scope,key)=*self.keys.get(index)?.as_ref()?;Some((self.owners.lookup((scope,key))?,scope,key))}
 /// ♻️ The caller releases only an unreferenced owner after its original children return.
 pub fn release(&mut self,owner:FixedKeySlot)->bool{let Some(key)=self.key(owner)else{return false;};if self.references[owner.index()]!=0||self.request_names[owner.index()].is_some()||!self.owners.release(key,owner){return false;}self.keys[owner.index()]=None;self.closing[owner.index()]=false;self.executions[owner.index()]=None;self.terminals[owner.index()]=None;self.lookups[owner.index()]=None;true}
 /// 🪦️ App close removes one consumer per bounded call while retaining all request children.
 pub fn remove_one_scoped_consumer(&mut self,scope:u32)->Option<FixedKeySlot>{for index in 0..C{if let Some((bound_scope,consumer))=self.consumer_at(index){if bound_scope==scope&&!self.consumer_closing[index]{let removed=self.remove_consumer(scope,consumer);return removed.or_else(||self.consumers.lookup((scope,consumer)));}}}None}
 /// 🧾️ App close admits one original consumer metadata frontier without inventing a request lease.
 pub fn remove_one_scoped_consumer_controlled(&mut self,scope:u32,grant:RetainedCloneGrant)->Option<FixedRequestDepartureStep>{for index in 0..C{if let Some((bound_scope,consumer))=self.consumer_at(index){if bound_scope==scope&&!self.consumer_closing[index]{return self.remove_consumer_controlled(scope,consumer,grant);}}}None}
 /// 🪟️ Exact consumer roster reads support window retirement without changing siblings.
 pub fn consumer_at(&self,index:usize)->Option<(u32,W)>{self.consumers.key_at(index)}
}
impl<K:Copy+Eq,W:Copy+Eq,const R:usize,const C:usize> Default for FixedRequestOwners<K,W,R,C>{fn default()->Self{Self::new()}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
