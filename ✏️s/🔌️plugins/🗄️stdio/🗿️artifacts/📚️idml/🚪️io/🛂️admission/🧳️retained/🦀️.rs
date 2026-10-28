//! 🧳 Genuine immutable source custody around canonical borrowed IDML admission.
use super::{IdmlAdmissionCursor,IdmlAdmissionSource,IdmlAdmissionGrant,IdmlVersionView,IdmlPartReplacement};
use crate::IdmlSnapshot;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
const FIXED:usize=512;
const UNIT:usize=64;
fn fits(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items!=0&&demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes&&demand.depth<=grant.maximum_depth}
/// 🚦 Immutable package admission yields no mutable post or event publication authority.
#[derive(Clone,Copy,Debug)]
pub enum RetainedIdmlAdmissionAdvance {Progress(RetainedCloneProgress),Ready(RetainedCloneProgress,bool)}
impl RetainedIdmlAdmissionAdvance {pub fn progress(self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Ready(p,_)=>p}}}
/// 🧷 The exact original source alias remains live through every admission and cancellation frontier.
pub struct RetainedIdmlAdmission {source:Option<RetainedCloneBinding>,replacement_source:Option<RetainedCloneBinding>,replacement_index:Option<usize>,cursor:IdmlAdmissionCursor,ready:bool,error:Option<&'static str>,version:Option<IdmlVersionView>,closing:bool}
impl Default for RetainedIdmlAdmission {fn default()->Self{Self::new()}}
impl RetainedIdmlAdmission {
    /// 🌱 Fixed inline state is born inside the enclosing funded preparation body.
    pub fn new()->Self{Self{source:None,replacement_source:None,replacement_index:None,cursor:IdmlAdmissionCursor::new(),ready:false,error:None,version:None,closing:false}}
    /// 🩹 Captures only the resolved index before borrowing actual replacement bytes.
    pub fn replacement(index:usize)->Self{let mut owner=Self::new();owner.replacement_index=Some(index);owner}
    fn inputs(&self,original:RetainedCloneRef<'_,IdmlSnapshot>,replacement:Option<RetainedCloneRef<'_,Vec<u8>>>)->Result<(),ValueError>{
        if self.replacement_index.is_some()!=replacement.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML replacement profile changed"));}
        original.check_binding(&self.source)?;if let Some(replacement)=replacement{replacement.check_binding(&self.replacement_source)?;}Ok(())
    }
    /// 📏 Quotes an original lease, one page, or a bounded syntax/name work slice before access.
    pub fn demand(&self,original:RetainedCloneRef<'_,IdmlSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{
        self.read_demand(original,None,body)
    }
    /// 📏 Includes the genuine original mutation-byte lease before any replacement read.
    pub fn replacement_demand(&self,original:RetainedCloneRef<'_,IdmlSnapshot>,replacement:RetainedCloneRef<'_,Vec<u8>>,body:usize)->Result<RetirementDemand,ValueError>{self.read_demand(original,Some(replacement),body)}
    fn read_demand(&self,original:RetainedCloneRef<'_,IdmlSnapshot>,replacement:Option<RetainedCloneRef<'_,Vec<u8>>>,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing||self.ready{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML retained admission already settled"));}
        self.inputs(original,replacement)?;
        if !original.check_binding(&self.source)?{return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
        if let Some(replacement)=replacement{if !replacement.check_binding(&self.replacement_source)?{return Ok(RetirementDemand{copy_bytes:replacement.binding_copy_bytes(),depth:1,..Default::default()});}}
        let capacity_bytes=self.cursor.next_capacity_bytes();
        let copy_bytes=if capacity_bytes!=0{FIXED+UNIT+self.cursor.next_copy_bytes()}else{if body<FIXED+UNIT{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"IDML admission body cannot fit one bounded syntax unit"));}FIXED+((body-FIXED)/UNIT).min(1024)*UNIT};
        Ok(RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()})
    }
    /// ▶ Reads the genuine original source only after all selected grant axes fit.
    pub fn advance(&mut self,original:RetainedCloneRef<'_,IdmlSnapshot>,body:usize,grant:RetainedCloneGrant)->Result<RetainedIdmlAdmissionAdvance,ValueError>{
        self.read_advance(original,None,body,grant)
    }
    /// ▶ Reads both genuine immutable original sources under the selected quote.
    pub fn advance_replacement(&mut self,original:RetainedCloneRef<'_,IdmlSnapshot>,replacement:RetainedCloneRef<'_,Vec<u8>>,body:usize,grant:RetainedCloneGrant)->Result<RetainedIdmlAdmissionAdvance,ValueError>{self.read_advance(original,Some(replacement),body,grant)}
    fn read_advance(&mut self,original:RetainedCloneRef<'_,IdmlSnapshot>,replacement:Option<RetainedCloneRef<'_,Vec<u8>>>,body:usize,grant:RetainedCloneGrant)->Result<RetainedIdmlAdmissionAdvance,ValueError>{
        let demand=self.read_demand(original,replacement,body)?;
        if !fits(demand,grant){return Ok(RetainedIdmlAdmissionAdvance::Progress(Default::default()));}
        if let Some(progress)=original.bind(&mut self.source,grant)?{return Ok(RetainedIdmlAdmissionAdvance::Progress(progress));}
        if let Some(replacement)=replacement{if let Some(progress)=replacement.bind(&mut self.replacement_source,grant)?{return Ok(RetainedIdmlAdmissionAdvance::Progress(progress));}}
        let capacity=demand.capacity_bytes;let maximum_work=if capacity!=0{1}else{(demand.copy_bytes-FIXED)/UNIT};
        let step=self.cursor.advance(IdmlAdmissionSource{snapshot:original.get(),replacement:self.replacement_index.zip(replacement).map(|(index,source)|IdmlPartReplacement{index,bytes:source.get().as_slice()}),compare_version:self.replacement_index.is_none()},IdmlAdmissionGrant{maximum_work,maximum_copy_bytes:if capacity!=0{self.cursor.next_copy_bytes()}else{0},maximum_capacity_bytes:capacity,maximum_release_bytes:0});
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:FIXED+step.work*UNIT+step.copied_bytes,retained_capacity_bytes:step.retained_capacity_bytes,..Default::default()};
        if step.done||step.error.is_some(){self.ready=true;self.error=step.error;self.version=step.version;return Ok(RetainedIdmlAdmissionAdvance::Ready(progress,self.error.is_none()));}
        Ok(RetainedIdmlAdmissionAdvance::Progress(progress))
    }
    /// 🧷 Borrows only the exact admitted source pair while both genuine aliases stay retained.
    pub fn admitted_source<'a>(&'a self,original:RetainedCloneRef<'a,IdmlSnapshot>,replacement:Option<RetainedCloneRef<'a,Vec<u8>>>)->Result<IdmlAdmissionSource<'a>,ValueError>{
        if self.closing||!self.ready||self.error.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML source is not admitted"));}self.inputs(original,replacement)?;
        if !original.check_binding(&self.source)?{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML admitted original alias changed"));}
        if let Some(source)=replacement{if !source.check_binding(&self.replacement_source)?{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML admitted byte alias changed"));}}
        Ok(IdmlAdmissionSource{snapshot:original.get(),replacement:self.replacement_index.zip(replacement).map(|(index,source)|IdmlPartReplacement{index,bytes:source.get().as_slice()}),compare_version:self.replacement_index.is_none()})
    }
    /// 🔎 Lends its original numeric lookup index while admission and source custody remain live.
    pub fn admitted_index(&self)->Option<&IdmlAdmissionCursor>{if self.ready&&self.error.is_none()&&!self.closing{Some(&self.cursor)}else{None}}
    /// 🏷 Numeric evidence is available only after complete original package admission.
    pub fn version(&self)->Option<IdmlVersionView>{if self.ready&&self.error.is_none(){self.version}else{None}}
    /// ⚠ Semantic refusal is a fixed original code, with no unquoted diagnostic allocation.
    pub fn refusal(&self)->Option<&'static str>{self.error}
    /// 🛑 Keeps the genuine source alias while revoking lexical work.
    pub fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
    /// ♻ Numeric metadata closes before the original immutable lease.
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML admission closure not started"));}
        let release=self.cursor.next_release_bytes();if release!=0{return Ok(RetirementDemand{copy_bytes:FIXED,release_bytes:release,depth:1,..Default::default()});}
        RetainedCloneBinding::close_demand(if self.replacement_source.is_some(){&self.replacement_source}else{&self.source},body)
    }
    /// 🪜 Releases at most one exact page or one original source binding action.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let demand=self.close_demand(grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.cursor.next_release_bytes()!=0{let step=self.cursor.close_one(grant.maximum_release_bytes);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:FIXED,released_bytes:step.released_bytes,..Default::default()}));}
        let step=RetainedCloneBinding::close_one(if self.replacement_source.is_some(){&mut self.replacement_source}else{&mut self.source},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    /// 🪹 No numeric page or genuine source alias remains retained.
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.terminal_is_empty()&&self.source.is_none()&&self.replacement_source.is_none()}
}
impl Drop for RetainedIdmlAdmission {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"IDML admission requires original source and metadata closure");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
