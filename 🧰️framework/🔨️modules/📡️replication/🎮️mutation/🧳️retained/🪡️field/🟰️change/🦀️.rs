//! 🎚️ Typed fields share original typed equality, authored cloning, and exact inverse ownership.
use crate::{field::RetainedFieldPreparation,retained_mutation_demand_fits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value_persistent::{retained_equality::RetainedEquality, retained_equality::RetainedEqualityOwner};
use std::mem::size_of;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}

/// 🪡️ None means unchanged; Some(value) transfers the original funded authored field without a synthetic source wrapper.
pub struct RetainedFieldChange<T:RetainedEquality+RetainedClone>{equality:RetainedEqualityOwner<T>,authored:RetainedFieldPreparation<T>,original:RetainedFieldPreparation<T>,phase:u8,closing:bool}
impl<T:RetainedEquality+RetainedClone> Default for RetainedFieldChange<T>{fn default()->Self{Self{equality:RetainedEqualityOwner::new(),authored:Default::default(),original:Default::default(),phase:0,closing:false}}}
impl<T:RetainedEquality+RetainedClone> RetainedFieldChange<T>{
 /// 🧭️ Typed wrappers keep the same original source pair after child ownership transfer.
 pub fn verify_sources(&self,a:RetainedCloneRef<'_,T>,o:RetainedCloneRef<'_,T>)->Result<(),ValueError>{self.equality.verify(a,o)}
 fn fixed()->usize{size_of::<u8>()*2}
 fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=Self::fixed();d.depth+=1;d}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=Self::fixed();p}
 pub fn ready(&self)->bool{self.phase==2&&!self.closing}
 pub fn inverse_ready(&self)->bool{self.phase==4&&!self.closing}
 pub fn decision_demand(&self,authored:RetainedCloneRef<'_,T>,original:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid("typed field decision is closing"));}self.equality.verify(authored,original)?;Ok(match self.phase{0=>Self::nested(self.equality.demand(authored,original,body)?),1=>Self::nested(self.authored.demand(authored,body)?),2=>self.change_demand()?,_=>return Err(invalid("typed field field decision is spent"))})}
 pub fn decide(&mut self,authored:RetainedCloneRef<'_,T>,original:RetainedCloneRef<'_,T>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.decision_demand(authored,original,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let p=match self.phase{0=>{let step=self.equality.advance(authored,original,Self::child(g))?;if step.equal.is_some(){self.phase=if self.equality.result()==Some(true){2}else{1};}Self::receipt(step.progress)},1=>{let step=self.authored.advance(authored,body,Self::child(g))?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;}Self::receipt(step.progress())},2=>return Ok(RetainedCloneStep::Complete(Default::default())),_=>unreachable!()};Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 /// 📤️ Quotes both the original authored child transfer and the exact outer tri-state result header.
 pub fn change_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("typed field authored change is not ready"));}let fixed=size_of::<Option<T>>()*2+Self::fixed();if self.equality.result()==Some(true){return Ok(scalar(fixed));}let mut d=self.authored.exchange_demand()?;d.copy_bytes+=fixed;d.depth+=1;Ok(d)}
 pub fn take_change(&mut self,g:RetainedCloneGrant)->Result<Option<(Option<T>,RetainedCloneProgress)>,ValueError>{let d=self.change_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}let fixed=size_of::<Option<T>>()*2+Self::fixed();let(value,progress)=if self.equality.result()==Some(true){(None,RetainedCloneProgress{copied_items:1,copied_bytes:fixed,..Default::default()})}else{let(value,mut p)=self.authored.take(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g})?;p.copied_bytes+=fixed;(Some(value.ok_or_else(||invalid("typed field authored field disappeared"))?),p)};self.phase=3;Ok(Some((value,progress)))}
 pub fn inverse_demand(&self,original:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=3{return Err(invalid("typed field inverse lacks original decision transfer"));}self.equality.verify_original(original)?;Ok(Self::nested(self.original.demand(original,body)?))}
 pub fn inverse(&mut self,original:RetainedCloneRef<'_,T>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.inverse_demand(original,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let step=self.original.advance(original,body,Self::child(g))?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=4;}Ok(match step{RetainedCloneStep::Complete(p)=>RetainedCloneStep::Complete(Self::receipt(p)),RetainedCloneStep::Progress(p)=>RetainedCloneStep::Progress(Self::receipt(p))})}
 pub fn inverse_take_demand(&self)->Result<RetirementDemand,ValueError>{if self.phase!=4||self.closing{return Err(invalid("typed field inverse is not ready"));}Ok(Self::nested(self.original.exchange_demand()?))}
 pub fn take_inverse(&mut self,g:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{let d=self.inverse_take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}let(value,p)=self.original.take(Self::child(g))?;self.phase=5;Ok(Some((value.ok_or_else(||invalid("typed field original inverse disappeared"))?,Self::receipt(p))))}
 pub fn begin_close(&mut self){self.closing=true;self.authored.begin_close();self.original.begin_close();self.equality.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("typed field field close has not begun"));}let body=body.saturating_sub(Self::fixed());Ok(Self::nested(if !self.authored.terminal_is_empty(){self.authored.close_demand(body)?}else if !self.original.terminal_is_empty(){self.original.close_demand(body)?}else{self.equality.close_demand(body)?}))}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let step=if !self.authored.terminal_is_empty(){self.authored.close_step(Self::child(g))?}else if !self.original.terminal_is_empty(){self.original.close_step(Self::child(g))?}else{self.equality.close_step(Self::child(g))?};let p=Self::receipt(step.progress());Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.authored.terminal_is_empty()&&self.original.terminal_is_empty()&&self.equality.terminal_is_empty()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🎚️record/🦀️.rs"] pub mod record;

#[path="📋️indexed/🦀️.rs"] pub mod indexed;
