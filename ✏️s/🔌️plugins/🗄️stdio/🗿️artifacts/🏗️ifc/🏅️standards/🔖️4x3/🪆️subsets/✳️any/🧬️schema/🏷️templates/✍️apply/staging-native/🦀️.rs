//! 🧭️ Private prepared template pages retain original command carriers and never publish model mutations.
use super::*;
use semio_framework_plugin::action_staging::*;
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,OriginalSourceBinding};
use semio_framework_plugin::app::staged_form::*;
use semio_framework_ui_locale::Locale;
use semio_framework_value::retained_clone::{RetainedCloneOwner,RetainedCloneSource};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use std::mem::ManuallyDrop;

pub const NATIVE_PREPARED_TEMPLATE_MOUNTED:bool=false;
const DEPTH:usize=16;
const TEXT:usize=512;
type RawSource=RetainedCloneSource<DslValue,StagedFormRequest>;
#[path="drafts.rs"]
mod drafts;
use drafts::IdentityDraftState;
#[path="identity-controls.rs"]
mod identity_controls;
use identity_controls::IdentityControlState;
fn refused()->ValueError{invalid("staging.native-template-representation")}
fn stale()->ValueError{invalid("staging.stale")}
fn empty_token()->StagedFormToken{StagedFormToken{action:String::new(),generation:String::new(),revision:String::new(),window:String::new()}}
fn token_part(token:&StagedFormToken,part:usize)->&str{match part{0=>&token.action,1=>&token.generation,2=>&token.revision,_=>&token.window}}
fn token_part_mut(token:&mut StagedFormToken,part:usize)->&mut String{match part{0=>&mut token.action,1=>&mut token.generation,2=>&mut token.revision,_=>&mut token.window}}
fn editable(path:&[String])->bool{!matches!(path.last().map(String::as_str),Some("kind"|"key"|"name"|"template"))}
fn label(path:&[String],locale:Locale)->&'static str{
    match(path.last().map(String::as_str),locale){
        (Some("id"),Locale::En)=>"Entity identifier",(Some("id"),Locale::De)=>"Entitätskennung",
        (Some("guid"),_)=>"GlobalId",(Some("target"),Locale::En)=>"Target",(Some("target"),Locale::De)=>"Ziel",
        (Some("key"),Locale::En)=>"Identity role",(Some("key"),Locale::De)=>"Identitätsrolle",
        (Some("template"),Locale::En)=>"Released template",(Some("template"),Locale::De)=>"Veröffentlichtes Template",
        (Some("name"),Locale::En)=>"Declared name",(Some("name"),Locale::De)=>"Deklarierter Name",
        (Some("kind"),Locale::En)=>"Declared representation",(Some("kind"),Locale::De)=>"Deklarierte Darstellung",
        (_,Locale::En)=>"Explicit value",(_,Locale::De)=>"Expliziter Wert",
    }
}
fn required(path:&[String])->bool{matches!(path.last().map(String::as_str),Some("id"|"guid"|"target"|"value"|"bits"))}
fn description<'a>(root:&'a DslValue,path:&[String])->Option<&'a str>{if path.len()<3{return None;}let name=match path[0].as_str(){"identities"=>"key","entries"=>"name",_=>return None};let DslValue::Object(fields)=at(root,&path[..2]).ok()?else{return None};if fields.len()>8{return None;}fields.iter().find(|(key,_)|key==name)?.1.as_str()}
fn at<'a>(mut value:&'a DslValue,path:&[String])->Result<&'a DslValue,ValueError>{
    if path.len()>DEPTH{return Err(refused());}
    for part in path{value=match value{DslValue::Object(fields)if fields.len()<=8=>fields.iter().find(|(key,_)|key==part).map(|(_,value)|value),DslValue::Array(values)=>part.parse::<usize>().ok().and_then(|index|values.get(index)),_=>None}.ok_or_else(refused)?;}Ok(value)
}
#[derive(Default,Clone,Copy)]
struct Walk{slots:[usize;DEPTH],next:[usize;DEPTH+1],depth:usize,done:bool}
enum Visit{Progress,Leaf,Done}
impl Walk{
    fn node<'a>(&self,mut value:&'a DslValue)->Result<&'a DslValue,ValueError>{for slot in &self.slots[..self.depth]{value=match value{DslValue::Object(fields)=>fields.get(*slot).map(|(_,value)|value),DslValue::Array(values)=>values.get(*slot),_=>None}.ok_or_else(refused)?;}Ok(value)}
    fn segment<'a>(&self,root:&'a DslValue,index:usize,number:&'a mut [u8;20])->Result<&'a str,ValueError>{
        let mut parent=*self;parent.depth=index;let slot=self.slots[index];match parent.node(root)?{DslValue::Object(fields)=>Ok(&fields.get(slot).ok_or_else(refused)?.0),DslValue::Array(_)=>{let mut value=slot;let mut position=20;loop{position-=1;number[position]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}std::str::from_utf8(&number[position..]).map_err(|_|refused())},_=>Err(refused())}
    }
    fn consumed(&mut self){self.next[self.depth]=1;}
    fn next(&mut self,root:&DslValue)->Result<Visit,ValueError>{
        if self.done{return Ok(Visit::Done);}let node=self.node(root)?;let count=match node{DslValue::Object(fields)if fields.len()<=8=>fields.len(),DslValue::Array(values)=>values.len(),DslValue::String(text)if text.len()<=TEXT=>{if self.next[self.depth]==0{return Ok(Visit::Leaf);}0},_=>return Err(refused())};
        if self.next[self.depth]<count{if self.depth==DEPTH{return Err(refused());}let slot=self.next[self.depth];self.next[self.depth]+=1;self.slots[self.depth]=slot;self.depth+=1;self.next[self.depth]=0;}else if self.depth==0{self.done=true;}else{self.depth-=1;}Ok(Visit::Progress)
    }
}
#[derive(semio_framework_value::RetireOwned)]
enum Control{Content{address:u64,binding:StagedFormBinding},Identity{address:u64,global_id:bool,role:String,binding:StagedFormBinding},Removal{address:u64,row:usize,binding:StagedFormBinding},Page{address:u64,forward:bool,binding:StagedFormPageBinding}}
impl Control{
    fn content(&self)->Result<&StagedFormBinding,ValueError>{match self{Self::Content{binding,..}|Self::Identity{binding,..}|Self::Removal{binding,..}=>Ok(binding),_=>Err(refused())}}
    fn content_mut(&mut self)->Result<&mut StagedFormBinding,ValueError>{match self{Self::Content{binding,..}|Self::Identity{binding,..}|Self::Removal{binding,..}=>Ok(binding),_=>Err(refused())}}
}
#[derive(semio_framework_value::RetireOwned)]
struct Page{source:Option<RawSource>,status:StagedFormStatus,controls:Vec<Control>}
/// 🪟️ Only completed original-owned metadata is visible to the artifact-neutral body builder.
pub struct IfcPreparedTemplateSource{page:ManuallyDrop<Option<Page>>}
impl StagedBodySource for IfcPreparedTemplateSource{
    fn identity(&self)->usize{self as*const _ as usize}
    fn title(&self)->&str{match self.page.as_ref().unwrap().source.as_ref().unwrap().try_authority().unwrap().locale{Locale::En=>"Apply released template",Locale::De=>"Veröffentlichtes Template anwenden"}}
    fn status(&self)->&StagedFormStatus{&self.page.as_ref().unwrap().status}
    fn controls(&self)->usize{self.page.as_ref().unwrap().controls.len()}
    fn control(&self,index:usize)->Result<StagedBodyControl<'_>,ValueError>{
        let page=self.page.as_ref().ok_or_else(stale)?;let source=page.source.as_ref().ok_or_else(stale)?.try_authority()?;let field=page.controls.get(index).ok_or_else(refused)?;
        match field{
            Control::Removal{address,row,binding}=>{let rows=drafts::rows(&source.args)?;let description=drafts::fields(rows.get(*row).ok_or_else(refused)?)?.0;Ok(StagedBodyControl{scalar:StagedScalarControl{owner:self.identity(),address:*address,controller:crate::editor::IFC4X3_CONTROLLER,label:match source.locale{Locale::En=>"Remove identity",Locale::De=>"Identität entfernen"},description:Some(description),disabled:false,kind:StagedScalarKind::Button{page:false}},required:false,error:None,binding:Some(StagedBodyBinding::Action(binding))})},
            Control::Identity{address,global_id,role,binding}=>Ok(StagedBodyControl{scalar:StagedScalarControl{owner:self.identity(),address:*address,controller:crate::editor::IFC4X3_CONTROLLER,label:match(*global_id,source.locale){(true,_)=>"GlobalId",(false,Locale::En)=>"Entity identifier",(false,Locale::De)=>"Entitätskennung"},description:Some(role),disabled:false,kind:StagedScalarKind::Text{value:""}},required:true,error:None,binding:Some(StagedBodyBinding::Action(binding))}),
            Control::Page{address,forward,binding}=>Ok(StagedBodyControl{scalar:StagedScalarControl{owner:self.identity(),address:*address,controller:crate::editor::IFC4X3_CONTROLLER,label:match(*forward,source.locale){(true,Locale::En)=>"Next page",(true,Locale::De)=>"Nächste Seite",(false,Locale::En)=>"Previous page",(false,Locale::De)=>"Vorherige Seite"},description:None,disabled:false,kind:StagedScalarKind::Button{page:true}},required:false,error:None,binding:Some(StagedBodyBinding::Page(binding))}),
            Control::Content{address,binding}=>{let [StagedFormBindingEdit::Text{path}]=binding.edits.as_slice()else{return Err(refused())};let DslValue::String(value)=at(&source.args,path)?else{return Err(refused())};let edit=editable(path);Ok(StagedBodyControl{scalar:StagedScalarControl{owner:self.identity(),address:*address,controller:crate::editor::IFC4X3_CONTROLLER,label:label(path,source.locale),description:description(&source.args,path),disabled:false,kind:if edit{StagedScalarKind::Text{value}}else{StagedScalarKind::ReadOnly{value}}},required:required(path),error:None,binding:edit.then_some(StagedBodyBinding::Action(binding))})},
        }
    }
}
impl IfcPreparedTemplateSource{
    pub fn context_matches(&self,window:Option<&str>,locale:Locale)->Result<bool,ValueError>{Ok(self.page.as_ref().ok_or_else(stale)?.source.as_ref().ok_or_else(stale)?.try_authority()?.context_matches(window,locale))}
}
impl Drop for IfcPreparedTemplateSource{fn drop(&mut self){assert!(std::thread::panicking()||self.page.is_none(),"prepared IFC page requires original paid retirement");}}
impl RetireOwned for IfcPreparedTemplateSource{
    fn retirement(mut self)->Box<dyn RetirementCursor>{self.page.take().expect("private prepared source has one owner").retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.page.as_ref()?.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
}
/// 🧵️ The caller retains its original snapshot lease and supplies the full canonical source hash on every turn.
pub struct IfcTemplateSourceWork{
    request:ManuallyDrop<Option<StagedFormRequest>>,source:ManuallyDrop<Option<RawSource>>,page:ManuallyDrop<Option<Page>>,copy:ManuallyDrop<Option<DslValue>>,discarded:ManuallyDrop<Option<DslValue>>,
    clone:RetainedCloneOwner<DslValue>,query:IfcTemplateWork,full:IfcTemplateWork,retirement:RetainedCloneClose,identities:IdentityDraftState,identity_control:IdentityControlState,
    binding:Option<(OriginalSourceBinding,usize,[u8;32])>,walk:Walk,ordinal:u64,offset:u64,phase:u8,part:usize,path_part:usize,navigation:u8,closing:bool,failed:bool,
}
impl Default for IfcTemplateSourceWork{fn default()->Self{Self{request:ManuallyDrop::new(None),source:ManuallyDrop::new(None),page:ManuallyDrop::new(None),copy:ManuallyDrop::new(None),discarded:ManuallyDrop::new(None),clone:Default::default(),query:Default::default(),full:Default::default(),retirement:Default::default(),identities:Default::default(),identity_control:Default::default(),binding:None,walk:Default::default(),ordinal:0,offset:0,phase:0,part:0,path_part:0,navigation:0,closing:false,failed:false}}}
fn copy_text(target:&mut String,text:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    if !target.is_empty()||text.len()>TEXT{return Err(refused().into());}let mut candidate=None;let demand=OriginalConstructionDemand{items:1,copy_bytes:text.len()+size_of::<String>(),capacity_bytes:text.len(),release_bytes:0,depth:1};
    admit_original_construction(cx,&mut candidate,demand,|_|{target.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"staging text backing"))?;target.push_str(text);Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:target.capacity(),..Default::default()}))})
}
fn reserve<T>(target:&mut Vec<T>,count:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    if !target.is_empty()||target.capacity()!=0{return Err(refused().into());}let bytes=count.checked_mul(size_of::<T>()).ok_or_else(refused)?;let mut candidate=None;
    admit_original_construction(cx,&mut candidate,OriginalConstructionDemand{items:1,copy_bytes:size_of::<Vec<T>>(),capacity_bytes:bytes,release_bytes:0,depth:1},|_|{target.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"staging page backing"))?;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),retained_capacity_bytes:target.capacity()*size_of::<T>(),..Default::default()}))})
}
fn metadata(cx:&mut StepContext<'_>,bytes:usize,change:impl FnOnce())->Result<bool,OriginalTurnFailure>{let demand=fixed(bytes);if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging cancelled").into());}if cx.should_yield()||!fits(cx.retained_grant(),demand){return Ok(false);}change();cx.consume_retained(receipt(demand))?;cx.consume_fuel(1);Ok(true)}
fn post_clone(result:Result<RetainedCloneStep,ValueError>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let posted=cx.consume_retained(progress);cx.consume_fuel(progress.copied_items as u64);
    match(result,posted){(Ok(step),Ok(()))=>Ok(matches!(step,RetainedCloneStep::Complete(_))),(Err(error),posted)=>{let mut failure=OriginalTurnFailure::from(error);if let Err(error)=posted{failure.context_refusal=Some(error);}Err(failure)},(Ok(_),Err(error))=>Err(error.into())}
}
impl IfcTemplateSourceWork{
    pub fn original_request_received(&self)->bool{self.binding.is_some()}
    pub fn context_matches(&self,window:Option<&str>,locale:Locale)->Result<bool,ValueError>{
        if let Some(request)=self.request.as_ref(){return Ok(request.context_matches(window,locale));}if let Some(source)=self.source.as_ref(){return Ok(source.try_authority()?.context_matches(window,locale));}if let Some(page)=self.page.as_ref(){return Ok(page.source.as_ref().ok_or_else(stale)?.try_authority()?.context_matches(window,locale));}Err(stale())
    }
    /// 📨️ A denied header grant leaves the admitted request with its original donor.
    pub fn receive(&mut self,donor:&mut Option<StagedFormRequest>,snapshot:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if donor.is_none()||self.binding.is_some()||self.closing||self.failed{return Err(stale().into());}let authority=OriginalSourceBinding::borrow(cx);metadata(cx,size_of::<Self>()+2*size_of::<StagedFormRequest>(),||{*self.request=donor.take();self.binding=Some((authority,snapshot as*const _ as usize,revision));})
    }
    fn matches(&self,snapshot:&IfcSnapshot,revision:[u8;32],cx:&StepContext<'_>)->bool{self.binding.is_some_and(|(binding,pointer,hash)|binding.matches(cx)&&pointer==snapshot as*const _ as usize&&hash==revision)}
    pub fn advance(&mut self,snapshot:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.closing||self.failed||!self.matches(snapshot,revision,cx){return Err(stale().into());}if cx.is_cancelled(){self.failed=true;return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging cancelled").into());}if !metadata(cx,size_of::<Self>(),||{})?{return Ok(false);}let result=self.advance_inner(snapshot,revision,cx);if result.is_err(){self.failed=true;}result
    }
    fn advance_inner(&mut self,snapshot:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        match self.phase{
            0=>{let request=self.request.as_ref().ok_or_else(refused)?;if !request.token.admitted()||request.token.action!=APPLY_TEMPLATE_ACTION||!revision_matches(&request.token.revision,revision){return Err(stale().into());}if let Some(page)=&request.page{if !page.admitted()||page.choice.is_some()||page.diagnostic_offset!="0"{return Err(refused().into());}self.offset=page.field_offset.parse().map_err(|_|refused())?;if self.offset%STAGED_FORM_BODY_LIMITS.content as u64!=0{return Err(refused().into());}}let demand=RawSource::borrowed_constructor_demand::<StagedFormRequest>();let physical=RetirementDemand{capacity_bytes:demand.capacity_bytes,copy_bytes:2*size_of::<StagedFormRequest>(),depth:demand.depth,..Default::default()};if !fits(cx.retained_grant(),physical){return Ok(false);}let request=self.request.take().unwrap();match RawSource::admit_borrowed(request,|request|&request.args,cx.retained_grant()){Ok((source,mut progress))=>{*self.source=Some(source);self.phase=1;progress.copied_bytes+=physical.copy_bytes;cx.consume_retained(progress)?;cx.consume_fuel(1);},Err((error,request))=>{*self.request=Some(request);return Err(error.into());}}},
            1=>{let source=self.source.as_ref().unwrap();let demand=self.clone.advance_demands(source.try_borrow()?,4096)?;if !fits(cx.retained_grant(),demand){return Ok(false);}let result=self.clone.advance(source.try_borrow()?,4096,cx.retained_grant());if post_clone(result,cx)?{self.phase=2;}},
            2=>{metadata(cx,2*size_of::<DslValue>(),||{*self.copy=self.clone.take();self.phase=if self.identities.full{29}else{25};})?;},
            3=>{if self.query.receiver.receive_owned(&mut self.copy,cx)?{self.phase=4;}},
            4=>{if self.query.receiver.advance(cx)?{self.phase=5;}},
            5=>{if self.query.receiver.take(&mut self.query.planner.application,cx)?{self.phase=6;}},
            6=>{if self.query.planner.identity_frontier()&&!self.identities.queried{self.phase=26;return Ok(false);}if self.query.planner.garbage.is_some()||!self.query.retirement.is_empty(){let demand=self.query.retirement.next_owner_demands::<IfcValue>(self.query.planner.garbage.is_some(),&None,4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|if self.query.retirement.is_empty(){self.query.retirement.begin_granted(&mut self.query.planner.garbage,grant)?.ok_or_else(refused)}else{self.query.retirement.step_granted(grant)})?;return Ok(false);}let demand=self.query.planner.demand(snapshot)?;if !fits(cx.retained_grant(),demand){return Ok(false);}if let Err(error)=self.query.planner.advance(snapshot){if self.identities.full&&matches!(error.code,"identityCollision"|"duplicateGuid"|"identityMissing"|"identityUnexpected"){self.identities.draft=true;self.phase=7;return Ok(false);}return Err(invalid(error.code).into());}cx.consume_retained(receipt(demand))?;cx.consume_fuel(1);if self.query.planner.done(){self.phase=7;}},
            7=>{metadata(cx,size_of::<Page>()+size_of::<RawSource>(),||{*self.page=Some(Page{source:self.source.take(),status:StagedFormStatus{token:empty_token(),ready:false,diagnostics:Vec::new(),diagnostic_offset:String::new(),diagnostic_total:String::new()},controls:Vec::new()});self.part=0;self.phase=8;})?;},
            8=>{let page=self.page.as_mut().unwrap();let token=&page.source.as_ref().unwrap().try_authority()?.token;if copy_text(token_part_mut(&mut page.status.token,self.part),token_part(token,self.part),cx)?{self.part+=1;if self.part==4{self.phase=9;}}},
            9=>{let page=self.page.as_mut().unwrap();if copy_text(&mut page.status.diagnostic_offset,"0",cx)?{self.phase=10;}},
            10=>{let page=self.page.as_mut().unwrap();if copy_text(&mut page.status.diagnostic_total,if self.identities.draft{"1"}else{"0"},cx)?{self.phase=if self.identities.draft{40}else{11};}},
            11=>{if reserve(&mut self.page.as_mut().unwrap().controls,STAGED_FORM_BODY_LIMITS.controls,cx)?{self.phase=12;}},
            12=>{let page=self.page.as_mut().unwrap();let raw=&page.source.as_ref().unwrap().try_authority()?.args;let mut failure=None;metadata(cx,2*size_of::<Walk>()+2*size_of::<u64>()+size_of::<Self>(),||{match self.walk.next(raw){Err(error)=>failure=Some(error),Ok(Visit::Done)=>{self.phase=33;},Ok(Visit::Leaf)=>{if self.ordinal>=self.offset&&self.ordinal-self.offset<STAGED_FORM_BODY_LIMITS.content as u64{self.phase=13;}else if let Some(next)=self.ordinal.checked_add(1){self.walk.consumed();self.ordinal=next;}else{failure=Some(refused());}},Ok(Visit::Progress)=>{}}})?;if let Some(error)=failure{return Err(error.into());}},
            13=>{let page=self.page.as_mut().unwrap();metadata(cx,size_of::<Control>()+size_of::<usize>(),||{page.controls.push(Control::Content{address:self.ordinal,binding:StagedFormBinding{token:empty_token(),layout:StagedFormLayout::Preserve,edits:Vec::new()}});self.part=0;self.phase=14;})?;},
            14=>{let page=self.page.as_mut().unwrap();let token=&page.source.as_ref().unwrap().try_authority()?.token;let binding=page.controls.last_mut().unwrap().content_mut()?;if copy_text(token_part_mut(&mut binding.token,self.part),token_part(token,self.part),cx)?{self.part+=1;if self.part==4{self.phase=15;}}},
            15=>{let binding=self.page.as_mut().unwrap().controls.last_mut().unwrap().content_mut()?;if reserve(&mut binding.edits,1,cx)?{self.phase=16;}},
            16=>{let binding=self.page.as_mut().unwrap().controls.last_mut().unwrap().content_mut()?;metadata(cx,size_of::<StagedFormBindingEdit>(),||{binding.edits.push(StagedFormBindingEdit::Text{path:Vec::new()});self.path_part=0;self.phase=17;})?;},
            17=>{let page=self.page.as_mut().unwrap();let binding=page.controls.last_mut().unwrap().content_mut()?;let StagedFormBindingEdit::Text{path}=&mut binding.edits[0]else{return Err(refused().into())};if path.capacity()==0{reserve(path,self.walk.depth,cx)?;return Ok(false);}if path.len()==self.path_part{metadata(cx,size_of::<String>(),||path.push(String::new()))?;return Ok(false);}let mut number=[0;20];let segment=self.walk.segment(&page.source.as_ref().unwrap().try_authority()?.args,self.path_part,&mut number)?;if segment.is_empty()||matches!(segment,"__proto__"|"prototype"|"constructor"){return Err(refused().into());}if copy_text(&mut path[self.path_part],segment,cx)?{self.path_part+=1;if self.path_part==self.walk.depth{self.walk.consumed();self.ordinal=self.ordinal.checked_add(1).ok_or_else(refused)?;self.phase=12;}}},
            18=>{let forward=self.navigation==1;let destination=if forward{self.offset.checked_add(STAGED_FORM_BODY_LIMITS.content as u64).filter(|offset|*offset<self.ordinal)}else{self.offset.checked_sub(STAGED_FORM_BODY_LIMITS.content as u64)};if let Some(_)=destination{let page=self.page.as_mut().unwrap();metadata(cx,size_of::<Control>()+size_of::<Self>(),||{page.controls.push(Control::Page{address:u64::MAX-2+self.navigation as u64,forward,binding:StagedFormPageBinding{token:empty_token(),page:StagedFormPage{field_offset:String::new(),diagnostic_offset:String::new(),choice:None}}});self.part=0;self.phase=22;})?;}else{metadata(cx,size_of::<Self>(),||{self.navigation+=1;if self.navigation==2{self.phase=24;}})?;}},
            22=>{let page=self.page.as_mut().unwrap();let token=&page.source.as_ref().unwrap().try_authority()?.token;let Control::Page{binding,..}=page.controls.last_mut().unwrap()else{return Err(refused().into())};if copy_text(token_part_mut(&mut binding.token,self.part),token_part(token,self.part),cx)?{self.part+=1;if self.part==4{self.phase=23;}}},
            23=>{let Control::Page{binding,forward,..}=self.page.as_mut().unwrap().controls.last_mut().unwrap()else{return Err(refused().into())};if self.part==4{let destination=if *forward{self.offset.checked_add(STAGED_FORM_BODY_LIMITS.content as u64)}else{self.offset.checked_sub(STAGED_FORM_BODY_LIMITS.content as u64)}.ok_or_else(refused)?;let mut number=[0;20];let text=decimal(destination,&mut number);if copy_text(&mut binding.page.field_offset,text,cx)?{self.part=5;}}else if copy_text(&mut binding.page.diagnostic_offset,"0",cx)?{self.navigation+=1;self.phase=if self.navigation==2{24}else{18};}},
            24=>{if !self.clone.terminal_is_empty(){let demand=self.clone.selected_close_demand(4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.clone.close_step(grant))?;}else{metadata(cx,size_of::<Self>(),||{self.query.begin_close();self.full.begin_close();self.phase=19;})?;}},
            19=>{if !self.query.terminal_is_empty(){self.query.close_step(cx)?;}else if !self.full.terminal_is_empty(){self.full.close_step(cx)?;}else if self.discarded.is_some()||!self.retirement.is_empty(){let demand=self.retirement.next_owner_demands::<DslValue>(self.discarded.is_some(),&None,4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|if self.retirement.is_empty(){self.retirement.begin_granted(&mut self.discarded,grant)?.ok_or_else(refused)}else{self.retirement.step_granted(grant)})?;}else{metadata(cx,size_of::<bool>(),||{self.page.as_mut().unwrap().status.ready=!self.identities.draft;self.phase=20;})?;}},
            25..=32|40..=46=>self.advance_identity(cx)?,
            33|34=>self.advance_identity_control(cx)?,
            _=>{},
        }Ok(self.phase==20)
    }
    pub fn take(&mut self,output:&mut Option<IfcPreparedTemplateSource>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.phase!=20||self.failed||self.closing||output.is_some()||self.binding.is_none_or(|(binding,_,_)|!binding.matches(cx)){return Err(stale().into());}metadata(cx,size_of::<Page>()+size_of::<IfcPreparedTemplateSource>(),||{*output=Some(IfcPreparedTemplateSource{page:ManuallyDrop::new(self.page.take())});self.phase=21;})
    }
    pub fn begin_close(&mut self){self.closing=true;self.query.begin_close();self.full.begin_close();}
    pub fn terminal_is_empty(&self)->bool{self.request.is_none()&&self.source.is_none()&&self.page.is_none()&&self.copy.is_none()&&self.discarded.is_none()&&self.clone.terminal_is_empty()&&self.query.terminal_is_empty()&&self.full.terminal_is_empty()&&self.retirement.is_empty()}
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{
        if self.binding.is_some_and(|(binding,_,_)|!binding.matches(cx)){return Err(stale().into());}
        let step=self.close_one(cx)?;Ok(match step{semio_framework_job::InteractiveJobCloseStep::Complete{progress}if !self.terminal_is_empty()=>semio_framework_job::InteractiveJobCloseStep::Pending{progress},other=>other})
    }
    fn close_one(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{
        if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);}if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});}
        let demand=self.close_demand(4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_owned(grant))
    }
}
impl IfcTemplateSourceWork{
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.query.terminal_is_empty(){return self.query.close_demand(body);}if !self.full.terminal_is_empty(){return self.full.close_demand(body);}if !self.clone.terminal_is_empty(){return self.clone.selected_close_demand(body);}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body);}
        macro_rules! demand{($owner:expr,$kind:ty)=>{if ($owner).is_some(){return self.retirement.next_owner_demands::<$kind>(true,&None,body);}};}
        demand!(&self.copy,DslValue);demand!(&self.discarded,DslValue);demand!(&self.page,Page);demand!(&self.source,RawSource);demand!(&self.request,StagedFormRequest);Ok(Default::default())
    }
    fn close_owned(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !fits(grant,self.close_demand(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let step=if !self.query.terminal_is_empty(){self.query.close_owned(grant)?}else if !self.full.terminal_is_empty(){self.full.close_owned(grant)?}else if !self.clone.terminal_is_empty(){self.clone.close_step(grant)?}else if !self.retirement.is_empty(){self.retirement.step_granted(grant)?}else{
            macro_rules! close{($owner:expr)=>{self.retirement.begin_granted($owner,grant)?.ok_or_else(refused)};}
            if self.copy.is_some(){close!(&mut self.copy)?}else if self.discarded.is_some(){close!(&mut self.discarded)?}else if self.page.is_some(){close!(&mut self.page)?}else if self.source.is_some(){close!(&mut self.source)?}else if self.request.is_some(){close!(&mut self.request)?}else{return Err(refused());}
        };Ok(RetainedCloneStep::Progress(step.progress()))
    }
}
impl RetireOwned for IfcTemplateSourceWork{
    fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for IfcTemplateSourceWork{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{if self.terminal_is_empty(){return semio_framework_value::retirement::RetirementStep::Complete;}match self.close_owned(grant){Ok(step)=>semio_framework_value::retirement::RetirementStep::Progress(step.progress()),Err(error)=>semio_framework_value::retirement::RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool{IfcTemplateSourceWork::terminal_is_empty(self)}
    fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(usize::MAX)?.copy_bytes)}
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.copy_bytes)}
}
fn revision_matches(text:&str,revision:[u8;32])->bool{text.len()==64&&text.bytes().enumerate().all(|(index,byte)|byte==b"0123456789abcdef"[if index%2==0{(revision[index/2]>>4)as usize}else{(revision[index/2]&15)as usize}])}
fn decimal(mut value:u64,buffer:&mut[u8;20])->&str{let mut index=20;loop{index-=1;buffer[index]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}std::str::from_utf8(&buffer[index..]).expect("decimal digits are UTF8")}
impl Drop for IfcTemplateSourceWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"IFC staged preparation requires original paid retirement");}}
#[cfg(test)]
#[path="tests.rs"]
mod tests;
