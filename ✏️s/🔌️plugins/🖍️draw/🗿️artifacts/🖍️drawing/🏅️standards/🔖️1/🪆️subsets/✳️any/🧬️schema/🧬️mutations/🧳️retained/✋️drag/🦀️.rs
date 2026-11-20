//! ✋️ Original multi-target drags retain target paths, sparse rows and exact before transforms.
use crate::DrawingMutation;
use protocol::{RetainedFlight,retained_native_child,retained_polled,retained_routed};
use semio_framework_value::{RetainedAllocationEffect,RetirementPollExit,OriginalRetirementCloseLoan,OriginalRetirementPoll};
use semio_s_artifact_draw_drawing_model::{DrawingLayerNode,DrawingTransform,layer_base,snapshot::{DrawingSnapshot,lookup::{original::{DrawingOriginalLookup,DrawingOriginalLayerAddress as Address},original_text::DrawingOriginalText,equality::DrawingOriginalEquality}}};
use semio_s_artifact_draw_drawing_diff::{DrawingDiff,DrawingLayersDelta,DrawingLayerModification,DrawingLayerPatch};
use semio_s_artifact_draw_drawing_geometry::{multiply,drawing_dragged_transform};
use protocol::{RetainedFieldPreparation,RetainedMutationCloseContext,MutationMessage,MutationMessageLedgerRetirement};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,retained_clone_grant_admits,close_retained_binding}};
use semio_framework_value::factory::vector::FactoryVectorRetirement;
use std::{mem::{ManuallyDrop,size_of},fmt::Write};
type Text=semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
#[derive(semio_framework_value::RetireOwned)]struct Work{at:Option<Address>,status:u8}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing drag original custody changed")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
/// 📐️ Canonical wire target cardinality bounds the retained original rows without decoding its body.
pub fn retained_target_row_bytes()->usize{size_of::<Work>()+size_of::<DrawingLayerModification>()+size_of::<DrawingMutation>()+size_of::<Address>()+2*size_of::<String>()}
fn metadata()->usize{2*size_of::<DrawingMutation>()+2*size_of::<Address>()+512}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(metadata()).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-metadata(),maximum_depth:g.maximum_depth-1,..g}}
fn paid(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=metadata();p}
fn copy(t:&DrawingTransform)->DrawingTransform{DrawingTransform{x:t.x,y:t.y,scale_x:t.scale_x,scale_y:t.scale_y,rotation:t.rotation,shear:t.shear}}
fn count(m:&DrawingMutation)->Result<usize,ValueError>{match m{DrawingMutation::DragLayers(v)=>Ok(v.targets.len()),_=>Err(invalid())}}
fn target<'a>(m:RetainedCloneRef<'a,DrawingMutation>,i:usize)->RetainedCloneRef<'a,Text>{m.project(0,|m|match m{DrawingMutation::DragLayers(v)=>&v.targets,_=>unreachable!()}).project(i,move|t|&t[i])}
fn before(a:Address,b:Address)->bool{a.indices[..a.length]<b.indices[..b.length]}
fn ancestor(a:Address,b:Address)->bool{a.length<b.length&&a.indices[..a.length]==b.indices[..a.length]}
/// 🧭️ One immutable command and base source fund every decision, inverse and diagnostic target.
pub struct DrawingOriginalDrag{flight:RetainedFlight,base:Option<RetainedCloneBinding>,command:Option<RetainedCloneBinding>,work:ManuallyDrop<Option<Vec<Work>>>,diff:ManuallyDrop<Option<DrawingDiff>>,rows:ManuallyDrop<Option<Vec<DrawingMutation>>>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,active:ManuallyDrop<Option<MutationMessage>>,retirement:RetainedCloneClose,row_close:Option<FactoryVectorRetirement<DrawingMutation>>,lookup:DrawingOriginalLookup,equality:DrawingOriginalEquality,text:DrawingOriginalText,native_id:RetainedFieldPreparation<Text>,phase:u8,index:usize,pair:usize,scan:usize,selected:Option<usize>,at:Option<Address>,parent:[f64;6],parent_depth:usize,editable:bool,covered:bool,next:Option<DrawingTransform>,applies:bool,reason:Option<u8>,unchanged:bool,counts:[usize;3],target_bytes:usize,diag:usize,diag_index:usize,diag_offset:usize,closing:bool}
impl Default for DrawingOriginalDrag{fn default()->Self{Self{flight:Default::default(),base:None,command:None,work:ManuallyDrop::new(None),diff:ManuallyDrop::new(None),rows:ManuallyDrop::new(None),messages:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),active:ManuallyDrop::new(None),retirement:Default::default(),row_close:None,lookup:Default::default(),equality:Default::default(),text:Default::default(),native_id:Default::default(),phase:0,index:0,pair:1,scan:0,selected:None,at:None,parent:[1.0,0.0,0.0,1.0,0.0,0.0],parent_depth:1,editable:true,covered:false,next:None,applies:false,reason:None,unchanged:false,counts:[0;3],target_bytes:0,diag:0,diag_index:0,diag_offset:0,closing:false}}}
impl DrawingOriginalDrag{
 fn selected(&self)->Result<usize,ValueError>{self.selected.ok_or_else(invalid)}
 fn id<'a>(&self,b:RetainedCloneRef<'a,DrawingSnapshot>)->Result<RetainedCloneRef<'a,Text>,ValueError>{Ok(self.at.ok_or_else(invalid)?.project(b).project(0,|n|&layer_base(n).id))}
 fn diagnostic(&self)->Option<(semio_framework_diagnostic::Severity,&'static str,&'static str)>{use semio_framework_diagnostic::Severity::*;if let Some(r)=self.reason{return (self.diag==0).then_some((if r<4{Fatal}else{Error},if r<4{"mutation.invariant"}else{"mutation.target-missing"},match r{1=>"a drag offset must be finite",2=>"a selection transform addresses at least one layer",3=>"a selection transform addresses every layer once",_=>""}))}match self.diag{0..=2 if self.counts[self.diag]!=0=>Some((Warning,"mutation.partial","")),3 if self.unchanged=>Some((Warning,"mutation.no-op","no layer changes its transform")),_=>None}}
 fn diag_matches(&self,i:usize)->bool{self.reason.is_some()||self.diag==3||self.work.as_ref().is_some_and(|w|w[i].status==[4,3,5][self.diag])}
 pub fn demand(&self,b:RetainedCloneRef<'_,DrawingSnapshot>,m:RetainedCloneRef<'_,DrawingMutation>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}let n=count(m.get())?;if !b.check_binding(&self.base)?{return Ok(fixed(b.binding_copy_bytes()))}if !m.check_binding(&self.command)?{return Ok(fixed(m.binding_copy_bytes()))}let inner=body.saturating_sub(metadata());Ok(match self.phase{
  0=>RetirementDemand{capacity_bytes:n.checked_mul(size_of::<Work>()+size_of::<DrawingLayerModification>()+size_of::<DrawingMutation>()).ok_or_else(invalid)?,..fixed(metadata())},
  1=>if self.index<n&&self.pair<n{nested(self.equality.demand(target(m,self.index),target(m,self.pair))?)?}else{fixed(metadata())},2=>nested(self.equality.close_demand(inner)?)?,
  3=>if self.index<n{nested(self.lookup.demand(b,target(m,self.index),inner)?)?}else{fixed(metadata())},4=>nested(self.lookup.close_demand(inner)?)?,
  5..=9=>fixed(metadata()+4096),10=>nested(self.text.demand(self.id(b)?,inner)?)?,11=>nested(self.text.take_demand()?)?,12=>nested(self.native_id.demand(self.id(b)?,inner,max.saturating_sub(1))?)?,13=>nested(self.native_id.exchange_demand()?)?,14=>nested(self.text.close_demand(inner)?)?,15=>nested(if self.native_id.close_started(){self.native_id.close_demand(inner)?}else{self.native_id.begin_close_demand()})?,16=>fixed(metadata()),
  30=>fixed(metadata()),31=>{let (_,code,_)=self.diagnostic().ok_or_else(invalid)?;RetirementDemand{capacity_bytes:4*size_of::<MutationMessage>()+n*size_of::<String>()+code.len()+256+self.target_bytes+n.saturating_mul(2),..fixed(metadata()+256)}},
  32=>fixed(metadata()),33=>nested(self.text.demand(target(m,self.diag_index),inner)?)?,34=>nested(self.text.take_demand()?)?,35=>fixed(metadata()+inner.min(4096).max(4)),36=>nested(self.text.close_demand(inner)?)?,37=>fixed(metadata()),38=>fixed(metadata()),40=>fixed(metadata()),_=>return Err(invalid())})}

 pub fn work_demand(&self,b:RetainedCloneRef<'_,DrawingSnapshot>,m:RetainedCloneRef<'_,DrawingMutation>,body:usize,max:u64)->Result<u64,ValueError>{
    if !b.check_binding(&self.base)?{return b.binding_work_demand(&self.base)}
    if !m.check_binding(&self.command)?{return m.binding_work_demand(&self.command)}
    let count=count(m.get())?;
    let inner=body.saturating_sub(metadata());
    let nested=match self.phase{
        1 if self.index<count&&self.pair<count=>self.equality.advance_work_demand(target(m,self.index),target(m,self.pair))?,
        2=>self.equality.close_work_demand(inner)?,
        3 if self.index<count=>self.lookup.advance_work_demand(b,target(m,self.index))?,
        4=>self.lookup.close_work_demand(inner)?,
        10=>self.text.advance_work_demand(self.id(b)?)?,
        12=>self.native_id.advance_work_demand(self.id(b)?,inner,max.saturating_sub(1))?,
        13=>1,
        14|36=>self.text.close_work_demand(inner)?,
        15 if self.native_id.close_started()=>self.native_id.close_work_demand(inner)?,
        15=>self.native_id.begin_close_work_demand(),
        33=>self.text.advance_work_demand(target(m,self.diag_index))?,
        _=>0,
    };
    let units=nested.checked_add(1).ok_or_else(invalid)?;
    Ok(if units<=max{units}else{0})
 }
 pub fn advance(&mut self,b:RetainedCloneRef<'_,DrawingSnapshot>,m:RetainedCloneRef<'_,DrawingMutation>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    let d=if recover{Default::default()}else{
        let d=self.demand(b,m,body,max)?;
        let required=self.work_demand(b,m,body,max)?;
        if required==0||required>effect.remaining_work_units()?||!retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !b.check_binding(&self.base)?{let t=b.bind(&mut self.base,effect);*work+=t.work;return Ok(RetainedCloneStep::Progress(t.result?.unwrap_or_default()))}
        if !m.check_binding(&self.command)?{let t=m.bind(&mut self.command,effect);*work+=t.work;return Ok(RetainedCloneStep::Progress(t.result?.unwrap_or_default()))}
        d
    };
    let inner=body.saturating_sub(metadata());
    let limit=max.saturating_sub(1);
    if self.phase==12{
        let id=self.id(b)?;
        let step=retained_routed!(self.flight,effect,recover,12,inner,limit,None,*work,|r| self.native_id.advance_into(id,inner,limit,&mut effect.nested(child(g)),r),self.native_id.recover_advance_into(id,inner,limit,&mut effect.nested(child(g)),r))?;
        if self.native_id.value().is_some(){self.phase=13}
        *work+=1;
        return Ok(RetainedCloneStep::Progress(paid(step.progress())))
    }
    if self.phase==15{
        let progress=if !self.native_id.close_started(){
            let turn=self.native_id.begin_close_paid(&mut effect.nested(child(g)));*work+=turn.work;turn.result?
        }else{
            retained_routed!(self.flight,effect,recover,15,inner,0,None,*work,|r| self.native_id.close_into(inner,&mut effect.nested(child(g)),r),self.native_id.recover_close_into(inner,&mut effect.nested(child(g)),r))?.progress()
        };
        if self.native_id.terminal_is_empty(){self.phase=16}
        *work+=1;
        return Ok(RetainedCloneStep::Progress(paid(progress)))
    }
    if recover{return Err(invalid().into())}
    if self.phase==13{
        let mut id=None;
        let step=retained_native_child(effect,work,|e,r|self.native_id.take_into(&mut id,&mut e.nested(child(g)),r))??;
        let old=&layer_base(self.at.ok_or_else(invalid)?.node(b.get()).ok_or_else(invalid)?).transform;
        self.rows.as_mut().unwrap().push(crate::update_layer_transform(id.ok_or_else(invalid)?,copy(old)));
        self.text.begin_close();self.phase=14;*work+=1;
        return Ok(RetainedCloneStep::Progress(paid(step.progress())))
    }
    let start=effect.performed_work();
    let n=count(m.get())?;let DrawingMutation::DragLayers(v)=m.get()else{return Err(invalid().into())};let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let result=(||->Result<(),ValueError>{match self.phase{
  0=>{if n!=0{if effect.actual().is_some(){return Err(invalid())}*effect.actual()=Some(0)}*self.work=Some(Vec::new());*self.diff=Some(DrawingDiff{layers:Some(Default::default()),..Default::default()});*self.rows=Some(Vec::new());self.phase=254;let a=self.work.as_mut().unwrap().try_reserve_exact(n);p.retained_capacity_bytes=self.work.as_ref().unwrap().capacity()*size_of::<Work>();if n!=0{*effect.actual()=Some(p.retained_capacity_bytes)}if a.is_err(){return Err(invalid())}let a=self.diff.as_mut().unwrap().layers.as_mut().unwrap().modified.try_reserve_exact(n);p.retained_capacity_bytes+=self.diff.as_ref().unwrap().layers.as_ref().unwrap().modified.capacity()*size_of::<DrawingLayerModification>();if n!=0{*effect.actual()=Some(p.retained_capacity_bytes)}if a.is_err(){return Err(invalid())}let a=self.rows.as_mut().unwrap().try_reserve_exact(n);p.retained_capacity_bytes+=self.rows.as_ref().unwrap().capacity()*size_of::<DrawingMutation>();if n!=0{*effect.actual()=Some(p.retained_capacity_bytes)}if a.is_err()||!p.fits(g){return Err(invalid())}self.reason=if !v.dx.is_finite()||!v.dy.is_finite(){Some(1)}else if n==0{Some(2)}else{None};self.phase=if self.reason.is_some(){30}else{1}},
  1=>{if self.index>=n{self.index=0;self.phase=3}else if self.pair>=n{self.index+=1;self.pair=self.index+1}else{p=paid(self.equality.advance(target(m,self.index),target(m,self.pair),&mut effect.nested(child(g)))?.progress());if let Some(eq)=self.equality.result(){if eq{self.reason=Some(3)}self.equality.begin_close();self.phase=2}}},
  2=>{p=paid(self.equality.close_step(inner,&mut effect.nested(child(g)))?.progress());if self.equality.terminal_is_empty(){self.equality=Default::default();self.pair+=1;self.phase=if self.reason.is_some(){30}else{1}}},
  3=>{if self.index==n{self.scan=0;self.selected=None;self.phase=5}else{p=paid(self.lookup.advance(b,target(m,self.index),inner,&mut effect.nested(child(g)))?.progress());if self.lookup.ready(){let at=self.lookup.address()?;self.work.as_mut().unwrap().push(Work{at,status:if at.is_some(){0}else{4}});if at.is_none(){self.counts[0]+=1}self.target_bytes=self.target_bytes.checked_add(v.targets[self.index].len()).ok_or_else(invalid)?;self.lookup.begin_close();self.phase=4}}},
  4=>{p=paid(self.lookup.close_step(inner,&mut effect.nested(child(g)))?.progress());if self.lookup.terminal_is_empty(){self.lookup=Default::default();self.index+=1;self.phase=3}},
  5=>{if self.scan<n{let w=&self.work.as_ref().unwrap()[self.scan];if w.status==0&&self.selected.is_none_or(|i|before(w.at.unwrap(),self.work.as_ref().unwrap()[i].at.unwrap())){self.selected=Some(self.scan)}self.scan+=1}else if let Some(i)=self.selected{self.at=self.work.as_ref().unwrap()[i].at;self.parent=[1.0,0.0,0.0,1.0,0.0,0.0];self.parent_depth=1;self.editable=true;self.covered=false;self.scan=0;self.phase=6}else{if !self.applies{self.reason=Some(4)}self.unchanged=self.diff.as_ref().unwrap().layers.as_ref().unwrap().modified.is_empty();self.phase=30}},
  6=>{let at=self.at.ok_or_else(invalid)?;if self.parent_depth<at.length{let mut parent=at;parent.length=self.parent_depth;let node=parent.node(b.get()).ok_or_else(invalid)?;let base=layer_base(node);self.editable&=base.visible&&!base.locked;self.parent=multiply(self.parent,semio_s_artifact_draw_drawing_model::affine::drawing_transform_to_matrix(&base.transform));self.parent_depth+=1}else{let base=layer_base(at.node(b.get()).ok_or_else(invalid)?);self.editable&=base.visible&&!base.locked;self.phase=7}},
  7=>{if self.scan<n{let w=&self.work.as_ref().unwrap()[self.scan];if (w.status==1||w.status==2)&&w.at.is_some_and(|a|ancestor(a,self.at.unwrap())){self.covered=true}self.scan+=1}else{self.phase=8}},
  8=>{let i=self.selected()?;if self.covered{self.work.as_mut().unwrap()[i].status=2;self.applies=true;self.phase=9}else if !self.editable{self.work.as_mut().unwrap()[i].status=3;self.counts[1]+=1;self.phase=9}else{let old=&layer_base(self.at.unwrap().node(b.get()).ok_or_else(invalid)?).transform;match drawing_dragged_transform(old,self.parent,[v.dx,v.dy]){None=>{self.work.as_mut().unwrap()[i].status=5;self.counts[2]+=1;self.phase=9},Some(next)=>{self.work.as_mut().unwrap()[i].status=1;self.applies=true;if next==*old{self.phase=9}else{self.next=Some(next);self.phase=10}}}}},
  9=>{self.selected=None;self.scan=0;self.phase=5},
  10=>{p=paid(self.text.advance(self.id(b)?,inner,&mut effect.nested(child(g)))?.progress());if self.text.ready(){self.phase=11}},
  11=>{let(id,q)=self.text.take(child(g))?;let next=self.next.as_ref().ok_or_else(invalid)?;self.diff.as_mut().unwrap().layers.as_mut().unwrap().modified.push(DrawingLayerModification{id:id.ok_or_else(invalid)?,patch:DrawingLayerPatch{transform:Some(copy(next)),..Default::default()}});p=paid(q);self.phase=12},
  14=>{p=paid(self.text.close_step(inner,&mut effect.nested(child(g)))?.progress());if self.text.terminal_is_empty(){self.phase=15}},16=>{self.text=Default::default();self.native_id=Default::default();self.next=None;self.phase=9},
  30=>{if self.diag>=if self.reason.is_some(){1}else{4}{self.phase=40}else if self.diagnostic().is_some(){self.phase=31}else{self.diag+=1}},
  31=>{if effect.actual().is_some(){return Err(invalid())}*effect.actual()=Some(0);let(level,code,text)=self.diagnostic().ok_or_else(invalid)?;if self.messages.is_none(){*self.messages=Some(Vec::new());let r=self.messages.as_mut().unwrap().try_reserve_exact(4);p.retained_capacity_bytes=self.messages.as_ref().unwrap().capacity()*size_of::<MutationMessage>();*effect.actual()=Some(p.retained_capacity_bytes);if r.is_err(){return Err(invalid())}}*self.active=Some(MutationMessage{level,code:semio_framework_diagnostic::FaultCode(String::new()),message:String::new(),target:Vec::new(),op_index:None});self.phase=254;let row=self.active.as_mut().unwrap();let a=row.code.0.try_reserve_exact(code.len());let b=row.message.try_reserve_exact(256+self.target_bytes+n.saturating_mul(2));let c=row.target.try_reserve_exact(n);p.retained_capacity_bytes+=row.code.0.capacity()+row.message.capacity()+row.target.capacity()*size_of::<String>();*effect.actual()=Some(p.retained_capacity_bytes);if a.is_err()||b.is_err()||c.is_err()||!p.fits(g){return Err(invalid())}row.code.0.push_str(code);if self.reason==Some(4){write!(&mut row.message,"none of the {} target(s) is a visible, unlocked layer this transform can place",n).map_err(|_|invalid())?}else if self.reason.is_none()&&self.diag<3{write!(&mut row.message,"{} of {} target(s) skipped ({}): ",self.counts[self.diag],n,["not in this drawing","locked or hidden","placed through a singular transform"][self.diag]).map_err(|_|invalid())?}else{row.message.push_str(text)}self.diag_index=0;self.phase=32},
  32=>{if self.diag_index==n{self.phase=38}else if self.diag_matches(self.diag_index){self.phase=33}else{self.diag_index+=1}},
  33=>{p=paid(self.text.advance(target(m,self.diag_index),inner,&mut effect.nested(child(g)))?.progress());if self.text.ready(){self.phase=34}},
  34=>{let(id,q)=self.text.take(child(g))?;let row=self.active.as_mut().ok_or_else(invalid)?;if self.reason.is_none()&&self.diag<3&& !row.target.is_empty(){row.message.push_str(", ")}row.target.push(id.ok_or_else(invalid)?);self.diag_offset=0;p=paid(q);self.phase=35},
  35=>{let row=self.active.as_mut().ok_or_else(invalid)?;let id=row.target.last().ok_or_else(invalid)?;if self.reason.is_none()&&self.diag<3&&self.diag_offset<id.len(){let mut end=(self.diag_offset+body.saturating_sub(metadata()).min(4096).max(4)).min(id.len());while !id.is_char_boundary(end){end-=1}row.message.push_str(&id[self.diag_offset..end]);self.diag_offset=end}else{self.text.begin_close();self.phase=36}},
  36=>{p=paid(self.text.close_step(inner,&mut effect.nested(child(g)))?.progress());if self.text.terminal_is_empty(){self.phase=37}},37=>{self.text=Default::default();self.diag_index+=1;self.phase=32},38=>{self.messages.as_mut().unwrap().push(self.active.take().ok_or_else(invalid)?);self.diag+=1;self.phase=30},40=>{},_=>return Err(invalid())}Ok(())})();*work+=1+effect.performed_work().saturating_sub(start);if let Err(e)=result{let mut q=e.retained_progress();q.copied_items=q.copied_items.max(p.copied_items);q.copied_bytes=q.copied_bytes.saturating_add(metadata()).max(p.copied_bytes);q.retained_capacity_bytes=q.retained_capacity_bytes.max(p.retained_capacity_bytes);return Err(e.with_retained_progress(q).into())}Ok(if self.phase==40{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn ready(&self)->bool{!self.closing&&self.phase==40}
 pub fn outcome(&self)->(Option<&'static str>,bool){(self.reason.map(|r|if r<4{"mutation.invariant"}else{"mutation.target-missing"}),self.unchanged)}
 pub fn take_decision_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<DrawingDiff>>()+2*size_of::<Vec<MutationMessage>>()))}
 pub fn take_decision(&mut self,g:RetainedCloneGrant)->Result<(Option<DrawingDiff>,Vec<MutationMessage>,RetainedCloneProgress),ValueError>{let d=self.take_decision_demand()?;if !retained_clone_grant_admits(g,d){return Ok((None,Vec::new(),Default::default()))}let output=if self.reason.is_some(){None}else if self.unchanged{Some(DrawingDiff::default())}else{self.diff.take()};self.phase=41;Ok((output,self.messages.take().unwrap_or_default(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn take_inverse_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=41{return Err(invalid())}Ok(fixed(2*size_of::<Option<Vec<DrawingMutation>>>()+size_of::<u8>()))}
 pub fn take_inverse(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<DrawingMutation>>,RetainedCloneProgress),ValueError>{let d=self.take_inverse_demand()?;if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=42;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.lookup.begin_close();self.equality.begin_close();self.text.begin_close();}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,DrawingMutation>,body:usize)->Result<RetirementDemand,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.close_demand(body)}if !self.equality.terminal_is_empty(){return self.equality.close_demand(body)}if !self.text.terminal_is_empty(){return self.text.close_demand(body)}if !self.native_id.terminal_is_empty(){return if self.native_id.close_started(){self.native_id.close_demand(body)}else{Ok(self.native_id.begin_close_demand())}}if let Some(l)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_demand(l,body)}if self.active.is_some()||self.messages.is_some(){return Ok(fixed(size_of::<MutationMessageLedgerRetirement>()+2*size_of::<Vec<MutationMessage>>()+size_of::<MutationMessage>()))}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DrawingDiff>(false,&None,body)}if self.diff.is_some(){return self.retirement.next_owner_demands::<DrawingDiff>(true,&None,body)}if self.work.is_some(){return self.retirement.next_owner_demands::<Vec<Work>>(true,&None,body)}if let Some(r)=self.row_close.as_ref(){return if r.terminal_is_empty(){Ok(fixed(2*size_of::<Option<FactoryVectorRetirement<DrawingMutation>>>()))}else{r.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<DrawingMutation>::constructor_demand())}if self.command.is_some(){return RetainedCloneBinding::close_demand(&self.command,body)}RetainedCloneBinding::close_demand(&self.base,body)}

 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,DrawingMutation>,body:usize)->Result<u64,ValueError>{
    if !self.lookup.terminal_is_empty(){return self.lookup.close_work_demand(body)}
    if !self.equality.terminal_is_empty(){return self.equality.close_work_demand(body)}
    if !self.text.terminal_is_empty(){return self.text.close_work_demand(body)}
    if !self.native_id.terminal_is_empty(){return if self.native_id.close_started(){self.native_id.close_work_demand(body)}else{Ok(self.native_id.begin_close_work_demand())}}
    if let Some(ledger)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_computational_work(ledger,body)}
    if self.active.is_some()||self.messages.is_some(){return Ok(1)}
    if !self.retirement.is_empty(){return self.retirement.next_owner_work::<DrawingDiff>(false,&None,body)}
    if self.diff.is_some(){return self.retirement.next_owner_work::<DrawingDiff>(true,&None,body)}
    if self.work.is_some(){return self.retirement.next_owner_work::<Vec<Work>>(true,&None,body)}
    if let Some(rows)=self.row_close.as_ref(){return if rows.terminal_is_empty(){Ok(1)}else{rows.close_work_demand(c.mutations,body)}}
    if self.rows.is_some(){return Ok(1)}
    if self.command.is_some(){return RetainedCloneBinding::close_work_demand(&self.command,body)}
    RetainedCloneBinding::close_work_demand(&self.base,body)
 }
 pub fn requires_original_birth(&self)->bool{self.original_control(true)}
 pub fn requires_original_control(&self)->bool{self.original_control(false)}
 fn original_control(&self,birth:bool)->bool{
    if !self.lookup.terminal_is_empty()||!self.equality.terminal_is_empty()||!self.text.terminal_is_empty(){return false}
    if !self.native_id.terminal_is_empty(){return self.native_id.close_started()&&if birth{self.native_id.requires_original_birth()}else{self.native_id.requires_original_control()}}
    if self.ledger.is_some()||self.active.is_some()||self.messages.is_some(){return false}
    if !self.retirement.is_empty(){return if birth{self.retirement.requires_original_birth()}else{self.retirement.requires_original_control()}}
    if self.diff.is_some()||self.work.is_some(){return false}
    self.row_close.as_ref().is_some_and(|r|if birth{r.requires_original_birth()}else{r.requires_original_control()})
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.native_id.terminal_is_empty(){return self.native_id.close_original_birth_step(original,g)}
    if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,g)}
    if let Some(rows)=self.row_close.as_mut(){return rows.close_original_birth_step(original,g)}
    semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(invalid().into()),work:0})
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,DrawingMutation>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(invalid().into())}
    let g=effect.grant();
    let d=if recover{Default::default()}else{let d=self.close_demand(c,body)?;if !retained_clone_grant_admits(g,d)||self.close_work_demand(c,body)?>effect.remaining_work_units()?{return Ok(RetainedCloneStep::Progress(Default::default()))}d};
    if !recover{
        let start=effect.performed_work();
        let leaf=if !self.lookup.terminal_is_empty(){Some(self.lookup.close_step(body,effect))}
        else if !self.equality.terminal_is_empty(){Some(self.equality.close_step(body,effect))}
        else if !self.text.terminal_is_empty(){Some(self.text.close_step(body,effect))}else{None};
        if let Some(step)=leaf{*work+=effect.performed_work().saturating_sub(start);return Ok(RetainedCloneStep::Progress(step?.progress()))}
    }
    if !self.native_id.terminal_is_empty()||self.flight.route()==Some(50){
        let step=crate::retained::field_close(&mut self.native_id,&mut self.flight,50,original,body,effect,recover,work)?;
        return Ok(RetainedCloneStep::Progress(step.progress()))
    }
    if let Some(ledger)=self.ledger.as_mut(){
        let step=retained_polled(&mut self.flight,effect,recover,51,body,work,|e| semio_framework_value::ErasedSnapshotRetirement::close_step(ledger,body,e))??;
        if ledger.terminal_is_empty(){self.ledger.take();}
        return Ok(RetainedCloneStep::Progress(step.progress()))
    }
    if self.active.is_some()||self.messages.is_some(){
        let rows=self.messages.as_mut().ok_or_else(invalid)?;
        if self.active.is_some()&&rows.len()==rows.capacity(){return Err(invalid().into())}
        if let Some(row)=self.active.take(){rows.push(row)}
        *self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),self.messages.take().unwrap()));
        *work+=1;
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
    }
    if !self.retirement.is_empty()||self.diff.is_some()||self.flight.route()==Some(52){
        return crate::retained::owner_close(&mut self.retirement,&mut self.diff,&mut self.flight,52,original,body,effect,recover,work)
    }
    if self.work.is_some(){return crate::retained::owner_close(&mut self.retirement,&mut self.work,&mut self.flight,52,original,body,effect,recover,work)}
    if let Some(rows)=self.row_close.as_mut(){
        if rows.terminal_is_empty(){self.row_close=None;*work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
        return crate::retained::poll_close(&mut self.flight,53,original,body,effect,recover,work,|original,e|if let Some(loan)=original{rows.close_original_step(loan,body,e)}else{rows.close_step(c.mutations,body,e)})
    }
    if self.rows.is_some(){let step=retained_polled(&mut self.flight,effect,recover,54,body,work,|e|FactoryVectorRetirement::admit(&mut self.rows,&mut self.row_close,e))??;return Ok(RetainedCloneStep::Progress(step.progress()))}
    if self.command.is_some()||self.flight.route()==Some(55){let step=retained_polled(&mut self.flight,effect,recover,55,body,work,|e|close_retained_binding(&mut self.command,body,e))??;return Ok(RetainedCloneStep::Progress(step.progress()))}
    Ok(retained_polled(&mut self.flight,effect,recover,56,body,work,|e|close_retained_binding(&mut self.base,body,e))??)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.flight.idle()&&self.base.is_none()&&self.command.is_none()&&self.lookup.terminal_is_empty()&&self.equality.terminal_is_empty()&&self.text.terminal_is_empty()&&self.native_id.terminal_is_empty()&&self.work.is_none()&&self.diff.is_none()&&self.rows.is_none()&&self.messages.is_none()&&self.active.is_none()&&self.ledger.is_none()&&self.row_close.is_none()&&self.retirement.is_empty()}
}
impl Drop for DrawingOriginalDrag{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Drawing drag requires original-source cancellation closure");}}
