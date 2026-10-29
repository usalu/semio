//! 🏛️ Original host configuration values retain their accepted input, native JSON owners and exact output through handoff.
use semio_framework_plugin_decl_emission::{OriginalHostConfigurationDeclaration as Declaration,OriginalHostConfigurationProjection as Projection,OriginalContributionProjection as Contributions};
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,NativeEncodeControl,NativeEncodeContinuation,retirement::RetireOwned,retained_clone::RetainedCloneClose};
use semio_framework_pack_json::{JsonGrammarCursor,JsonSourceCursor,JsonReadLimits,JsonMemberPolicy,JsonError,JsonBorrowedWriteCursor,JsonWriteSource,JsonWriteNode};
use std::mem::{ManuallyDrop,size_of};
#[cfg(test)]
#[global_allocator]
static HOST_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Owners<M:RetireOwned+Send+'static>{args:Option<DslValue>,parser:Option<JsonGrammarCursor<DslValue>>,root:Option<DslValue>,error:Option<JsonError>,selected:Vec<usize>,writer:Option<JsonBorrowedWriteCursor>,encoding:Option<NativeEncodeContinuation>,text:Option<String>,mutation:Option<M>}
/// 🧳️ The declared host producer owns every accepted field until its original receiving slot accepts it.
pub struct OriginalHostConfiguration<M:RetireOwned+Send+'static>{declaration:&'static Declaration<M>,binding:OriginalSourceBinding,decoder:(u64,u64),owners:ManuallyDrop<Option<Owners<M>>>,retirement:RetainedCloneClose,body:usize,allocation:usize,phase:u8,position:usize,entry:usize,output_bytes:usize,empty:bool,in_string:bool,escaped:bool,in_scalar:bool,depth:usize,items:usize,string_bytes:usize,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original host configuration changed source or receiving custody")}
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn funds(g:semio_framework_value::RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn field<'a>(value:&'a DslValue,key:&str)->Option<&'a DslValue>{value.get(key)}
fn entries(value:&DslValue)->Option<&[DslValue]>{if let DslValue::Array(values)=value{Some(values)}else{None}}
fn topic(value:&DslValue)->Option<&DslValue>{field(value,"topicContribution").filter(|value|!matches!(value,DslValue::Null))}
fn payload(value:&DslValue)->Option<&DslValue>{topic(value).and_then(|value|field(value,"payload"))}
fn module(value:&DslValue)->Option<&str>{payload(value).and_then(|value|field(value,"moduleId")).and_then(DslValue::as_str)}
fn valid_entry(value:&DslValue)->bool{
    if !matches!(value,DslValue::Object(_))||field(value,"pluginId").and_then(DslValue::as_str).is_none(){return false}
    match topic(value){None=>true,Some(topic)=>matches!(topic,DslValue::Object(_))&&field(topic,"topic").and_then(DslValue::as_str).is_some()&&field(topic,"payload").is_some()}
}
fn addressed(value:&DslValue,policy:Contributions)->bool{
    let Some(topic)=topic(value)else{return false};if field(topic,"topic").and_then(DslValue::as_str)!=Some(policy.topic){return false}
    let Some(payload)=field(topic,"payload")else{return false};if !matches!(payload,DslValue::Object(_)){return false}
    policy.payload_fields.iter().all(|name|field(payload,name).and_then(DslValue::as_str).is_some())&&field(payload,"appId").and_then(DslValue::as_str)==Some(policy.app_id)
}
struct View<'a>{root:&'a DslValue,selected:&'a[usize],candidate:Option<usize>}
impl View<'_>{
    fn entry(&self,index:usize)->Result<&DslValue,ValueError>{let index=if let Some(candidate)=self.candidate{if index!=0{return Err(invalid())}candidate}else{*self.selected.get(index).ok_or_else(invalid)?};entries(self.root).and_then(|entries|entries.get(index)).ok_or_else(invalid)}
    fn leaf<'a>(&'a self,path:&[usize])->Result<(&'a DslValue,usize),ValueError>{let entry=self.entry(*path.first().ok_or_else(invalid)?)?;match path.get(1..){Some([0])=>Ok((field(entry,"pluginId").ok_or_else(invalid)?,path.len())),Some([1,0])=>Ok((field(topic(entry).ok_or_else(invalid)?,"topic").ok_or_else(invalid)?,path.len())),Some([1,1,..])=>Ok((payload(entry).ok_or_else(invalid)?,3)),_=>Err(invalid())}}
}
impl JsonWriteSource for View<'_>{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{match path{[]=>Ok(JsonWriteNode::Array(if self.candidate.is_some(){1}else{self.selected.len()})),[_]|[_,1]=>Ok(JsonWriteNode::Object(2)),_=>{let(value,offset)=self.leaf(path)?;value.node_at_path(&path[offset..])}}}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{[_]=>["pluginId","topicContribution"].get(index).copied().ok_or_else(invalid),[_,1]=>["topic","payload"].get(index).copied().ok_or_else(invalid),_=>{let(value,offset)=self.leaf(path)?;value.object_key_at_path(&path[offset..],index)}}}
}
impl<M:RetireOwned+Send+'static> OriginalHostConfiguration<M>{
    /// 🎟️ A declaration is selected before this paid move of the actual accepted argument owner.
    pub fn admit(declaration:&'static Declaration<M>,args:&mut Option<DslValue>,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
        if declaration.maximum_input_bytes==0||declaration.maximum_output_bytes<2{return Err(invalid().into())}
        if let Projection::Contributions(policy)=declaration.projection{if policy.payload_fields.len()>63||policy.maximum_depth==0||policy.maximum_items==0||policy.maximum_string_bytes==0||policy.maximum_modules<policy.excluded_module_ids.len(){return Err(invalid().into())}}
        let d=RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<(Self,Option<DslValue>,Option<DslValue>)>()};let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let allocation=cx.retained_grant().maximum_capacity_bytes;
        admit_original_construction(cx,output,construction(d),|control|Ok((Box::new(Self{declaration,binding,decoder:(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(invalid)?),owners:ManuallyDrop::new(Some(Owners{args:args.take(),parser:None,root:None,error:None,selected:Vec::new(),writer:None,encoding:None,text:None,mutation:None})),retirement:Default::default(),body,allocation,phase:0,position:0,entry:0,output_bytes:2,empty:false,in_string:false,escaped:false,in_scalar:false,depth:0,items:0,string_bytes:0,closing:false}),receipt(d))))
    }
    fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}Ok(())}
    fn owners(&self)->Result<&Owners<M>,ValueError>{self.owners.as_ref().ok_or_else(invalid)}
    fn input_text(&self)->Result<&str,ValueError>{Ok(self.owners()?.args.as_ref().and_then(|args|args.get("json")).and_then(DslValue::as_str).unwrap_or("[]"))}
    /// 👁️ The tag producer borrows the same original argument allocation while its config value advances.
    pub fn args(&self)->Option<&DslValue>{self.owners.as_ref().and_then(|owners|owners.args.as_ref())}
    pub fn ready(&self)->bool{!self.closing&&self.phase==10&&self.owners.as_ref().is_some_and(|owners|owners.mutation.is_some())}
    fn metadata(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut Self))->Result<bool,OriginalTurnFailure>{let d=fixed::<Self>();let mut done=None;admit_original_construction(cx,&mut done,construction(d),|_|{run(self);Ok(((),receipt(d)))})}
    fn scan(&mut self)->Result<(),ValueError>{
        let policy=match self.declaration.projection{Projection::RawText=>{self.phase=1;return Ok(())},Projection::Contributions(policy)=>policy};
        let text=self.input_text()?;if text.len()>self.declaration.maximum_input_bytes{self.empty=true;self.phase=1;self.position=0;return Ok(())}
        let length=text.len();let end=(self.position+64).min(length);let mut position=self.position;let mut in_string=self.in_string;let mut escaped=self.escaped;let mut in_scalar=self.in_scalar;let mut depth=self.depth;let mut items=self.items;let mut string_bytes=self.string_bytes;let mut invalid=false;
        while position<end{
            let byte=text.as_bytes()[position];position+=1;
            if in_string{if escaped{string_bytes=string_bytes.saturating_add(1);escaped=false}else if byte==92{string_bytes=string_bytes.saturating_add(1);escaped=true}else if byte==34{in_string=false}else{string_bytes=string_bytes.saturating_add(1)}if string_bytes>policy.maximum_string_bytes{invalid=true;break}continue}
            if in_scalar{if byte.is_ascii_whitespace()||matches!(byte,44|93|125){in_scalar=false}else{continue}}
            match byte{34=>{items+=1;in_string=true;string_bytes=0},91|123=>{depth+=1;items+=1;if depth>policy.maximum_depth{invalid=true;break}},93|125=>{depth=depth.saturating_sub(1)},byte if byte.is_ascii_whitespace()||matches!(byte,44|58)=>{},_=>{items+=1;in_scalar=true}}
            if items>policy.maximum_items{invalid=true;break}
        }
        self.position=position;self.in_string=in_string;self.escaped=escaped;self.in_scalar=in_scalar;self.depth=depth;self.items=items;self.string_bytes=string_bytes;
        if invalid{self.empty=true;self.phase=1;self.position=0}else if position==length{self.phase=3;self.position=0}Ok(())
    }
    /// ⏳️ Each call retains one bounded native parse, projection, measurement, copy or mutation-header turn.
    pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(cx)?;if self.closing{return Err(invalid().into())}if self.ready(){return Ok(true)}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original host configuration was cancelled").into())}if cx.should_yield(){return Ok(false)}
        let grant=cx.retained_grant();
        match self.phase{
            0=>{let d=RetirementDemand{copy_bytes:64+size_of::<Self>(),depth:1,..Default::default()};let mut done=None;admit_original_construction(cx,&mut done,construction(d),|_|{self.scan()?;Ok(((),receipt(d)))})?;},
            1=>{
                let length=if self.empty{2}else{self.input_text()?.len()};if length>self.declaration.maximum_input_bytes||length>self.declaration.maximum_output_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"host configuration exceeds its declared output extent").into())}
                let d=RetirementDemand{capacity_bytes:length,..fixed::<Option<String>>()};let mut text=None;if admit_original_construction(cx,&mut text,construction(d),|_|Ok((String::with_capacity(length),receipt(d))))?{self.owners.as_mut().unwrap().text=text;self.phase=2;self.position=0}
            },
            2=>{
                let text=if self.empty{"[]"}else{self.input_text()?};if self.position==text.len(){self.metadata(cx,|owner|owner.phase=9)?;return Ok(false)}
                let mut end=(self.position+64).min(text.len());while !text.is_char_boundary(end){end-=1}let d=RetirementDemand{copy_bytes:end-self.position+2*size_of::<usize>(),depth:1,..Default::default()};let mut done=None;
                admit_original_construction(cx,&mut done,construction(d),|_|{let owners=self.owners.as_mut().unwrap();let text=if self.empty{"[]"}else{owners.args.as_ref().and_then(|args|args.get("json")).and_then(DslValue::as_str).unwrap_or("[]")};owners.text.as_mut().unwrap().push_str(&text[self.position..end]);self.position=end;Ok(((),receipt(d)))})?;
            },
            3=>{
                let policy=match self.declaration.projection{Projection::Contributions(policy)=>policy,_=>return Err(invalid().into())};let d=fixed::<JsonGrammarCursor<DslValue>>();let mut parser=None;
                admit_original_construction(cx,&mut parser,construction(d),|_|{let limits=JsonReadLimits{maximum_bytes:self.declaration.maximum_input_bytes as u64,maximum_allocation_bytes:self.allocation,maximum_depth:policy.maximum_depth,maximum_items:policy.maximum_items as u64};let(_,parser)=JsonSourceCursor::<_,DslValue>::new(self.input_text()?,JsonMemberPolicy::Reject,limits)?.into_grammar();Ok((parser,receipt(d)))})?;
                if parser.is_some(){self.owners.as_mut().unwrap().parser=parser;self.phase=4}
            },
            4=>{
                let owners=self.owners.as_ref().unwrap();let text=owners.args.as_ref().and_then(|args|args.get("json")).and_then(DslValue::as_str).unwrap_or("[]");let quote=owners.parser.as_ref().ok_or_else(invalid)?.normal_step_demands(text);let d=match quote{Ok(d)=>d,Err(error)=>{let native=matches!(error,JsonError::Native(_));let retained=self.metadata(cx,|owner|{owner.owners.as_mut().unwrap().error=Some(error);if native{owner.phase=12}else{owner.empty=true;owner.phase=1;owner.position=0}})?;if retained&&native{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native host JSON quote refused its original source").into())}return Ok(false)}};if !funds(grant,d){return Ok(false)}
                let result=cx.with_native_receiving(|native|native.receive_construction_step(d,|control,actual|{
                    let owners=self.owners.as_mut().unwrap();let text=owners.args.as_ref().and_then(|args|args.get("json")).and_then(DslValue::as_str).unwrap_or("[]");let parser=owners.parser.as_mut().unwrap();let result=parser.step_source(text,1,control,actual,true);let progress=parser.normal_step_progress();
                    match result{Ok(Some(root))=>{owners.root=Some(root);self.phase=5;self.entry=0},Ok(None)=>{},Err(JsonError::Native(error))=>{self.phase=12;return Err(error.with_retained_progress(progress))},Err(error)=>{owners.error=Some(error);self.empty=true;self.phase=1;self.position=0}}
                    Ok(RetainedCloneStep::Complete(progress))
                }));
                let progress=result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());
                let reconciled=match cx.receiving_issued_grant(){Some(issued)=>cx.consume_receiving_retained(issued,progress),None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native host parser omitted its original issued receiving scope"))};cx.consume_fuel(progress.copied_items as u64);
                match(result,reconciled){(Ok(_),Ok(()))=>{},(Err(error),Ok(()))=>return Err(error.into()),(Ok(_),Err(error))=>return Err(error.into()),(Err(error),Err(receipt))=>{let mut failure=OriginalTurnFailure::from(error);failure.context_refusal=Some(receipt);return Err(failure)}}

            },
            5=>{
                let owners=self.owners.as_ref().unwrap();let Some(values)=owners.root.as_ref().and_then(entries)else{self.metadata(cx,|owner|{owner.empty=true;owner.phase=1})?;return Ok(false)};
                if self.entry<values.len(){let valid=valid_entry(&values[self.entry]);self.metadata(cx,|owner|{if valid{owner.entry+=1}else{owner.empty=true;owner.phase=1;owner.position=0}})?;return Ok(false)}
                let length=values.len();let d=RetirementDemand{capacity_bytes:length.checked_mul(size_of::<usize>()).ok_or_else(invalid)?,..fixed::<Vec<usize>>()};let mut selected=None;
                if admit_original_construction(cx,&mut selected,construction(d),|_|Ok((Vec::with_capacity(length),receipt(d))))?{self.owners.as_mut().unwrap().selected=selected.unwrap();self.phase=6;self.entry=0}
            },
            6=>{
                let policy=match self.declaration.projection{Projection::Contributions(policy)=>policy,_=>return Err(invalid().into())};let owners=self.owners.as_ref().unwrap();let values=owners.root.as_ref().and_then(entries).ok_or_else(invalid)?;
                if self.entry==values.len()||policy.excluded_module_ids.len()+owners.selected.len()>=policy.maximum_modules{self.metadata(cx,|owner|owner.phase=8)?;return Ok(false)}
                let candidate=&values[self.entry];let selected=addressed(candidate,policy)&&(!policy.deduplicate_modules||module(candidate).is_some_and(|id|!policy.excluded_module_ids.contains(&id)&&!owners.selected.iter().any(|index|module(&values[*index])==Some(id))));
                if !selected{self.metadata(cx,|owner|owner.entry+=1)?;return Ok(false)}
                if owners.writer.is_none(){let d=fixed::<JsonBorrowedWriteCursor>();let mut writer=None;if admit_original_construction(cx,&mut writer,construction(d),|_|Ok((JsonBorrowedWriteCursor::new(),receipt(d))))?{self.owners.as_mut().unwrap().writer=writer;self.phase=7}return Ok(false)}
                self.metadata(cx,|owner|owner.phase=7)?;
            },
            7|8=>{
                let owners=self.owners.as_ref().unwrap();
                if owners.writer.is_none(){let d=fixed::<JsonBorrowedWriteCursor>();let mut writer=None;if admit_original_construction(cx,&mut writer,construction(d),|_|Ok((JsonBorrowedWriteCursor::new(),receipt(d))))?{self.owners.as_mut().unwrap().writer=writer}return Ok(false)}
                if self.phase==7{if let Some(bytes)=owners.writer.as_ref().unwrap().measured_bytes(){
                    let additional=bytes.checked_sub(2).and_then(|bytes|bytes.checked_add(usize::from(!owners.selected.is_empty()))).ok_or_else(invalid)?;let accepted=self.output_bytes.checked_add(additional).is_some_and(|bytes|bytes<=self.declaration.maximum_output_bytes);let d=fixed::<Self>();let mut done=None;
                    admit_original_construction(cx,&mut done,construction(d),|_|{let owners=self.owners.as_mut().unwrap();if accepted{owners.selected.push(self.entry);self.output_bytes+=additional}let p=owners.writer.as_mut().unwrap().restart_measurement()?;self.entry+=1;self.phase=6;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes.max(p.copied_bytes),..Default::default()}))})?;return Ok(false)
                }}
                let candidate=(self.phase==7).then_some(self.entry);let view=View{root:owners.root.as_ref().ok_or_else(invalid)?,selected:&owners.selected,candidate};let d=owners.writer.as_ref().unwrap().normal_step_demands(&view)?;if !funds(grant,d){return Ok(false)}
                let cancel=cx.borrowed_original_cancel_token()?;let clock=cx.clock_source();let deadline=cx.deadline_us();let mut done=None;
                admit_original_construction(cx,&mut done,construction(d),|_|{
                    let owners=self.owners.as_mut().unwrap();let view=View{root:owners.root.as_ref().ok_or_else(invalid)?,selected:&owners.selected,candidate};let mut accepted=|_|!cancel.is_cancelled_now()&&clock().is_some_and(|now|now<deadline);let mut control=match owners.encoding.take(){Some(saved)=>NativeEncodeControl::resume(saved,&mut accepted)?,None=>NativeEncodeControl::new_retained(&mut accepted)};
                    let result=owners.writer.as_mut().unwrap().step(&view,1,&mut control,grant);let progress=owners.writer.as_ref().unwrap().normal_step_progress();owners.encoding=Some(control.pause()?);let text=result?;
                    if let Some(text)=text{if self.phase!=8{return Err(invalid().with_retained_progress(progress))}owners.text=Some(text);self.phase=9}Ok(((),progress))
                })?;
            },
            9=>{
                let d=fixed::<(M,M,String,Option<String>)>();let mut mutation=None;
                if admit_original_construction(cx,&mut mutation,construction(d),|_|Ok(((self.declaration.mutation)(self.owners.as_mut().unwrap().text.take().ok_or_else(invalid)?),receipt(d))))?{self.owners.as_mut().unwrap().mutation=mutation;self.phase=10}
            },
            12=>return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native host JSON source retains its refusal until original retirement").into()),
            _=>return Err(invalid().into()),
        }
        Ok(self.ready())
    }
    /// 📤️ The accepted mutation moves into its actual source slot while every parser and argument owner stays retained.
    pub fn take_into(&mut self,output:&mut Option<M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if !self.ready()||output.is_some(){return Err(invalid().into())}let d=fixed::<(Option<M>,Option<M>,u8)>();let mut done=None;admit_original_construction(cx,&mut done,construction(d),|_|{*output=self.owners.as_mut().unwrap().mutation.take();self.phase=11;Ok(((),receipt(d)))})}
    pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(self.body)}if self.owners.is_some(){return self.retirement.next_owner_demands::<Owners<M>>(true,&None,self.body)}Ok(Default::default())}
    /// ♻️ Cancellation and refusal retain all original fields until paid native physical closure.
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;self.closing=true;if self.terminal_is_empty(){return Ok(true)}let d=self.close_demands()?;close_original_retained_step(cx,d,|grant|if self.retirement.is_empty(){Ok(self.retirement.begin_granted(&mut self.owners,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}else{self.retirement.select(self.body)?.step(grant)})?;Ok(self.terminal_is_empty())}
    pub fn terminal_is_empty(&self)->bool{self.owners.is_none()&&self.retirement.is_empty()}
    pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),..fixed::<Option<Box<Self>>>()}}
}
impl<M:RetireOwned+Send+'static> Drop for OriginalHostConfiguration<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original host configuration requires paid source retirement")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(any(test,feature="testkit"))]
#[path="🧪️testkit/🦀️.rs"]
pub mod testkit;
#[cfg(any(test,feature="testkit"))]
pub use testkit::with_original_host_configuration;
