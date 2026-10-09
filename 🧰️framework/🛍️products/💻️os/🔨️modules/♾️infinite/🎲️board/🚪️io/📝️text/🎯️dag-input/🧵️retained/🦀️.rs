//! 🧵️ Retained physical grammar and complete typed admission preserve the original owner on refusal.
use crate::infinite::board::schema::dag_input::{DagComputingProgress,DagNodeStatuses,DagSelectionDomains,DagChannelRef,DagChannelDirection,DagNodeEvaluationStatus};
use semio_framework_value::{DslValue,NativeDecodeControl,ValueError,retirement::*};
use semio_framework_value::{ordered::{SharedOwner,UpdateCursor},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy};
#[derive(Clone,Copy)]
pub enum DagInputKind{Progress,Statuses,Selection,Channels}
pub enum DagInputFacts{Progress(DagComputingProgress),Statuses(DagNodeStatuses),Selection(DagSelectionDomains),Channels(Vec<DagChannelRef>)}
pub struct DagInputCursor{kind:DagInputKind,grammar:JsonGrammarCursor<DslValue>,raw:Option<DslValue>,facts:Option<DagInputFacts>,pending_status:Option<DagNodeEvaluationStatus>,pending_key:Option<SharedOwner<String>>,pending_value:Option<SharedOwner<DagNodeEvaluationStatus>>,pending_update:Option<UpdateCursor<DagNodeEvaluationStatus>>,pending_retired:Option<(DagNodeStatuses,UpdateCursor<DagNodeEvaluationStatus>)>,phase:u8,field:usize,index:usize,complete:bool,progress:RetainedCloneProgress}
impl DagInputCursor{
    pub fn new(kind:DagInputKind)->Self{Self{kind,grammar:JsonGrammarCursor::new(JsonMemberPolicy::Reject),raw:None,facts:None,pending_status:None,pending_key:None,pending_value:None,pending_update:None,pending_retired:None,phase:0,field:0,index:0,complete:false,progress:Default::default()}}
    pub fn take_retired_update(&mut self)->Option<(DagNodeStatuses,UpdateCursor<DagNodeEvaluationStatus>)>{self.pending_retired.take()}
    pub fn restore_retired_update(&mut self,value:(DagNodeStatuses,UpdateCursor<DagNodeEvaluationStatus>)){assert!(self.pending_retired.is_none());self.pending_retired=Some(value);}
    pub fn position(&self)->usize{self.grammar.position()}
    pub fn facts(&self)->Option<&DagInputFacts>{if self.complete{self.facts.as_ref()}else{None}}
    pub fn take_facts(&mut self)->Option<DagInputFacts>{if self.complete{self.facts.take()}else{None}}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.progress}
    pub fn normal_step_demands(&self,source:&str)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if self.raw.is_none(){return self.grammar.normal_step_demands(source).map_err(|error|error.into_value_error())}
        let raw=self.raw.as_ref().unwrap();if self.phase==0{match self.kind{DagInputKind::Progress=>super::closed(raw,&["active","stale"] )?,DagInputKind::Selection=>super::closed(raw,&["nodes","edges","handles"] )?,_=>{}}}let mut demand=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
        let vector=|value:&DslValue,size:usize|->Result<usize,ValueError>{value.as_array().ok_or_else(super::refuse)?.len().checked_mul(size).ok_or_else(super::refuse)};
        demand.capacity_bytes=if self.phase==0{match self.kind{DagInputKind::Progress=>vector(raw.get("stale").ok_or_else(super::refuse)?,std::mem::size_of::<String>())?,DagInputKind::Channels=>vector(raw,std::mem::size_of::<DagChannelRef>())?,_=>0}}else{match self.facts.as_ref().unwrap(){
            DagInputFacts::Selection(_)if self.phase==1=>vector(raw.get(["nodes","edges","handles"][self.field]).ok_or_else(super::refuse)?,std::mem::size_of::<String>())?,
            DagInputFacts::Statuses(_)if self.field<raw.as_object().ok_or_else(super::refuse)?.len()=>{let row=&raw.as_object().unwrap()[self.field].1;if self.phase==1&&row.as_object().ok_or_else(super::refuse)?.len()>2{return Err(super::refuse())}match self.phase{
                1 if row.get("status").and_then(DslValue::as_str)==Some("blocked")=>vector(row.get("ports").ok_or_else(super::refuse)?,std::mem::size_of::<String>())?,
                3=>SharedOwner::<String>::allocation_bytes(),4=>SharedOwner::<DagNodeEvaluationStatus>::allocation_bytes(),
                6=>{let update=self.pending_update.as_ref().unwrap();demand.copy_bytes=update.next_copy_byte_demand();demand.depth=update.next_depth_demand();update.next_capacity_byte_demand()?},_=>0}},_=>0}};Ok(demand)
    }
    pub fn step(&mut self,source:&str,maximum_units:usize,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<bool,ValueError>{
        self.progress=Default::default();if maximum_units==0||grant.maximum_items==0{return Ok(false)}control.checkpoint()?;if self.pending_retired.is_some(){return Ok(false)}if self.complete{return Ok(true)}
        let demand=self.normal_step_demands(source)?;if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes||demand.depth>grant.maximum_depth{return Ok(false)}
        control.admit_turn_capacity(grant.maximum_capacity_bytes)?;
        if self.raw.is_some(){let before=control.owned_bytes();self.project_one(control,grant)?;self.progress.copied_items=1;self.progress.retained_capacity_bytes=control.owned_bytes()-before;control.step()?;return Ok(self.complete)}
        let result=self.grammar.step(source,1,control,grant).map_err(|error|error.into_value_error());self.progress=self.grammar.normal_step_progress();self.raw=result?;Ok(false)
    }
    fn project_one(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<(),ValueError>{
        let raw=self.raw.as_mut().unwrap();
        if self.phase==0{
            self.facts=Some(match self.kind{
                DagInputKind::Progress=>{super::closed(raw,&["active","stale"])?;let active=raw.get("active").ok_or_else(super::refuse)?;if !active.is_null(){super::identifier(active)?;}let stale=control.allocate_vec(array(field(raw,"stale")?)?.len())?;let active=match field(raw,"active")?{DslValue::Null=>None,value=>Some(take_text(value,false)?)};DagInputFacts::Progress(DagComputingProgress{active,stale})},
                DagInputKind::Selection=>{super::closed(raw,&["nodes","edges","handles"])?;for name in ["nodes","edges","handles"]{array(field(raw,name)?)?;}DagInputFacts::Selection(DagSelectionDomains::default())},
                DagInputKind::Channels=>DagInputFacts::Channels(control.allocate_vec(array(raw)?.len())?),
                DagInputKind::Statuses=>{object(raw)?;DagInputFacts::Statuses(DagNodeStatuses::new())},
            });self.phase=1;return Ok(())
        }
        match self.facts.as_mut().unwrap(){
            DagInputFacts::Progress(facts)=>{let source=array(field(raw,"stale")?)?;if self.index==source.len(){self.complete=true;}else{facts.stale.push(take_text(&mut source[self.index],true)?);self.index+=1;}},
            DagInputFacts::Selection(facts)=>{
                let name=["nodes","edges","handles"][self.field];let source=array(field(raw,name)?)?;let output=match self.field{0=>&mut facts.nodes,1=>&mut facts.edges,_=>&mut facts.handles};
                if self.phase==1{*output=control.allocate_vec(source.len())?;self.phase=2;}
                else if self.index==source.len(){self.field+=1;self.index=0;self.phase=1;self.complete=self.field==3;}
                else{output.push(take_text(&mut source[self.index],true)?);self.index+=1;}
            },
            DagInputFacts::Channels(facts)=>{
                let rows=array(raw)?;if self.index==rows.len(){self.complete=true;return Ok(())}let row=&mut rows[self.index];super::channel(row)?;
                let direction=match row.get("direction").and_then(DslValue::as_str){Some("in")=>DagChannelDirection::In,Some("out")=>DagChannelDirection::Out,_=>return Err(super::refuse())};
                let widget_id=take_text(field(row,"widgetId")?,true)?;let port=take_text(field(row,"port")?,true)?;facts.push(DagChannelRef{widget_id,port,direction});self.index+=1;
            },
            DagInputFacts::Statuses(facts)=>{
                let rows=object(raw)?;if self.field==rows.len(){self.complete=true;return Ok(())}let(id,row)=&mut rows[self.field];
                if self.phase==1{
                    if id.is_empty(){return Err(super::refuse())}let status=row.get("status").and_then(DslValue::as_str).ok_or_else(super::refuse)?;
                    let names:&[&str]=match status{"ok"|"queued"|"computing"=>&["status"],"error"=>&["status","message"],"blocked"=>&["status","ports"],_=>return Err(super::refuse())};super::closed(row,names)?;
                    self.pending_status=Some(match status{"ok"=>DagNodeEvaluationStatus::Ok{},"queued"=>DagNodeEvaluationStatus::Queued{},"computing"=>DagNodeEvaluationStatus::Computing{},"error"=>DagNodeEvaluationStatus::Error{message:take_text(field(row,"message")?,false)?},"blocked"=>DagNodeEvaluationStatus::Blocked{ports:control.allocate_vec(array(field(row,"ports")?)?.len())?},_=>return Err(super::refuse())});
                    self.phase=if matches!(self.pending_status,Some(DagNodeEvaluationStatus::Blocked{..})){2}else{3};self.index=0;
                }else if self.phase==2{
                    let ports=array(field(row,"ports")?)?;if self.index==ports.len(){self.phase=3;}else{let DagNodeEvaluationStatus::Blocked{ports:output}=self.pending_status.as_mut().unwrap()else{unreachable!()};output.push(take_text(&mut ports[self.index],true)?);self.index+=1;}
                }else if self.phase==3{
                    let bytes=SharedOwner::<String>::allocation_bytes();control.charge(bytes)?;let value=std::mem::take(id);
                    match SharedOwner::admit(value,grant){Ok((owner,_))=>self.pending_key=Some(owner),Err((error,value))=>{*id=value;return Err(error)}}self.phase=4;
                }else if self.phase==4{
                    let bytes=SharedOwner::<DagNodeEvaluationStatus>::allocation_bytes();control.charge(bytes)?;let value=self.pending_status.take().unwrap();
                    match SharedOwner::admit(value,grant){Ok((owner,_))=>self.pending_value=Some(owner),Err((error,value))=>{self.pending_status=Some(value);return Err(error)}}self.phase=5;
                }else if self.phase==5{self.pending_update=Some(facts.begin_set_shared(self.pending_key.take().unwrap(),self.pending_value.take().unwrap()));self.phase=6;}
                else if self.phase==6{
                    let cursor=self.pending_update.as_mut().unwrap();self.progress=match cursor.advance_insert_controlled(grant,control)?{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};if cursor.is_complete(){self.phase=7;}
                }else{
                    let mut cursor=self.pending_update.take().unwrap();let next=cursor.take_result().expect("completed admitted ordered status root");let previous=std::mem::replace(facts,next);self.pending_retired=Some((previous,cursor));self.field+=1;self.phase=1;
                }
            },
        }Ok(())
    }

}
impl RetireOwned for DagInputFacts{
    fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Progress(value)=>sequence(vec![deferred(value)]),Self::Statuses(value)=>sequence(vec![deferred(value)]),Self::Selection(value)=>sequence(vec![deferred(value)]),Self::Channels(value)=>sequence(vec![deferred(value)])}}
    fn retirement_birth_bytes(&self)->Option<usize>{let bytes=match self{Self::Progress(value)=>deferred_birth_bytes_for(value),Self::Statuses(value)=>deferred_birth_bytes_for(value),Self::Selection(value)=>deferred_birth_bytes_for(value),Self::Channels(value)=>deferred_birth_bytes_for(value)};sequence_birth_bytes(&[bytes])}
    fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for DagInputCursor{
    fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.grammar),deferred(self.raw),deferred(self.facts),deferred(self.pending_status),deferred(self.pending_key),deferred(self.pending_value),deferred(self.pending_update),deferred(self.pending_retired)])}
    fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.grammar),deferred_birth_bytes_for(&self.raw),deferred_birth_bytes_for(&self.facts),deferred_birth_bytes_for(&self.pending_status),deferred_birth_bytes_for(&self.pending_key),deferred_birth_bytes_for(&self.pending_value),deferred_birth_bytes_for(&self.pending_update),deferred_birth_bytes_for(&self.pending_retired)])}
    fn controlled_retirement_supported()->bool{true}
}

fn array(value:&mut DslValue)->Result<&mut Vec<DslValue>,ValueError>{match value{DslValue::Array(values)=>Ok(values),_=>Err(super::refuse())}}
fn object(value:&mut DslValue)->Result<&mut Vec<(String,DslValue)>,ValueError>{match value{DslValue::Object(values)=>Ok(values),_=>Err(super::refuse())}}
fn field<'a>(value:&'a mut DslValue,name:&str)->Result<&'a mut DslValue,ValueError>{object(value)?.iter_mut().find(|(key,_)|key==name).map(|(_,value)|value).ok_or_else(super::refuse)}
fn take_text(value:&mut DslValue,nonempty:bool)->Result<String,ValueError>{if value.as_str().is_none_or(|text|nonempty&&text.is_empty()){return Err(super::refuse())}let DslValue::String(text)=std::mem::replace(value,DslValue::Null)else{unreachable!()};Ok(text)}
