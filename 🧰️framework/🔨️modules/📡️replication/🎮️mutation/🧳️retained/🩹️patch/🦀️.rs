//! 🩹️ Typed sparse inversion and field-header ownership exchange never clone untouched rows.
use super::super::ApplyCapability;
#[path="🚚️apply/🦀️.rs"] mod apply;
pub use apply::RetainedRowPatchApply;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

/// 🧭️ Authored fields copy the immutable event; inverse fields copy the corresponding original row.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedPatchSource{Authored,Inverse}

/// 🧬️ Each sparse patch declares its original inverse fields and validated owned field exchange.
pub trait RetainedRowPatch<R>:Default+semio_framework_value::retirement::RetireOwned+Send+'static{
 type Fields:RetainedRowPatchFields<R,Self>;
 const FIELD_COUNT:usize;
 fn exchange_demand(&self,index:usize)->Result<RetirementDemand,ValueError>;
 fn exchange_field(&mut self,row:&mut R,index:usize,capability:ApplyCapability,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
}

/// 🪡️ Inverse preparation retains genuine original row/patch sources until typed closure.
pub trait RetainedRowPatchFields<R,P:RetainedRowPatch<R>>:Send+'static{
 fn new(source:RetainedPatchSource)->Self;
 fn demand(&self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<P>,RetainedCloneProgress),ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}

/// 🏗️ Declares retained setters beside each technology's sparse row schema without semantic cold adapters.
#[macro_export]
macro_rules! retained_row_patch{
 ($vis:vis $owner:ident for $patch:ty => $row:ty {$($field:ident:$field_ty:ty),+$(,)?})=>{
  $vis mod $owner{
   use super::*;
   use $crate::retained::{RetainedFieldPreparation,retained_mutation_demand_fits};
   use $crate::retained::patch::{RetainedRowPatch,RetainedRowPatchFields,RetainedPatchSource};
   use $crate::value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
   /// 🧳️ Original sparse inverse fields and source aliases remain typed until complete settlement.
   pub struct Fields{
    $($field:RetainedFieldPreparation<$field_ty>,)+
    source:RetainedPatchSource,result:std::mem::ManuallyDrop<Option<$patch>>,retirement:RetainedCloneClose,phase:usize,payload:Option<RetainedCloneBinding>,row:Option<RetainedCloneBinding>,closing:bool,
   }
   impl RetainedRowPatchFields<$row,$patch> for Fields{
    fn new(source:RetainedPatchSource)->Self{Self{source,$($field:Default::default(),)+result:std::mem::ManuallyDrop::new(Some(Default::default())),retirement:Default::default(),phase:0,payload:None,row:None,closing:false}}
    fn demand(&self,patch:RetainedCloneRef<'_,$patch>,row:RetainedCloneRef<'_,$row>,body:usize)->Result<RetirementDemand,ValueError>{
     if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse already closing"));}
     if self.payload.is_none(){return Ok(RetirementDemand{copy_bytes:patch.binding_copy_bytes(),depth:1,..Default::default()});}
     if self.row.is_none(){return Ok(RetirementDemand{copy_bytes:row.binding_copy_bytes(),depth:1,..Default::default()});}
     let mut index=0;
     $(if self.phase==index{
      if patch.get().$field.is_none(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<usize>(),depth:1,..Default::default()});}
      if self.$field.value().is_some(){let mut demand=self.$field.exchange_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(std::mem::size_of::<Option<$field_ty>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"sparse inverse header extent overflow"))?;return Ok(demand);}
      let source=if self.source==RetainedPatchSource::Authored{patch.project(index,|patch|patch.$field.as_ref().expect("authored schema field"))}else{row.project(index,|row|&row.$field)};return self.$field.demand(source,body);
     }index+=1;)+
     let _=index;Ok(RetirementDemand{copy_bytes:std::mem::size_of::<$patch>()*2,depth:1,..Default::default()})
    }
    fn advance(&mut self,patch:RetainedCloneRef<'_,$patch>,row:RetainedCloneRef<'_,$row>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
     let demand=self.demand(patch,row,body)?;
     if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
     if let Some(progress)=patch.bind(&mut self.payload,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
     if let Some(progress)=row.bind(&mut self.row,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
     let mut index=0;
     $(if self.phase==index{
      if patch.get().$field.is_none(){self.phase+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
      if self.$field.value().is_some(){
       let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-std::mem::size_of::<Option<$field_ty>>(),..grant};
       let(value,mut progress)=self.$field.take(child)?;self.result.as_mut().expect("untransferred sparse inverse").$field=value;
       progress.copied_bytes+=std::mem::size_of::<Option<$field_ty>>();self.phase+=1;return Ok(RetainedCloneStep::Progress(progress));
      }
      let source=if self.source==RetainedPatchSource::Authored{patch.project(index,|patch|patch.$field.as_ref().expect("authored schema field"))}else{row.project(index,|row|&row.$field)};let step=self.$field.advance(source,body,grant)?;return Ok(RetainedCloneStep::Progress(step.progress()));
     }index+=1;)+
     let _=index;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
    fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<$patch>,RetainedCloneProgress),ValueError>{
     if self.closing||self.phase!=<$patch as RetainedRowPatch<$row>>::FIELD_COUNT{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse not ready for ownership transfer"));}
     let copy_bytes=std::mem::size_of::<$patch>()*2;
     if !retained_mutation_demand_fits(RetirementDemand{copy_bytes,depth:1,..Default::default()},grant){return Ok((None,Default::default()));}
     let value=self.result.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse ownership already transferred"))?;
     Ok((Some(value),RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}))
    }
    fn begin_close(&mut self){self.closing=true;$(self.$field.begin_close();)+}
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
     $(if !self.$field.terminal_is_empty(){return self.$field.close_demand(body);})+
     if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<$patch>(false,&None,body);}
     if self.result.is_some(){return Ok(self.retirement.next_owner_demands::<$patch>(true,&None,body)?);}
     let binding=if self.row.is_some(){&self.row}else{&self.payload};
     RetainedCloneBinding::close_demand(binding,body)
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
     if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse closure not started"));}
     if !retained_mutation_demand_fits(self.close_demand(grant.maximum_copy_bytes)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
     $(if !self.$field.terminal_is_empty(){return self.$field.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));})+
     if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
     if let Some(step)=self.retirement.begin_granted(&mut self.result,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
     let step=RetainedCloneBinding::close_one(if self.row.is_some(){&mut self.row}else{&mut self.payload},grant)?;
     Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn terminal_is_empty(&self)->bool{self.result.is_none()&&self.retirement.is_empty()&&self.payload.is_none()&&self.row.is_none()$(&&self.$field.terminal_is_empty())+}
   }
   impl Drop for Fields{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"sparse inverse requires original field and source closure");if empty{unsafe{std::mem::ManuallyDrop::drop(&mut self.result);}}}}
  }
  impl $crate::retained::patch::RetainedRowPatch<$row> for $patch{
   type Fields=$owner::Fields;
   const FIELD_COUNT:usize=[$(stringify!($field)),+].len();
   fn exchange_demand(&self,index:usize)->Result<$crate::value::RetirementDemand,$crate::value::ValueError>{
    let mut field_index=0;
    $(if index==field_index{return Ok($crate::value::RetirementDemand{copy_bytes:if self.$field.is_some(){std::mem::size_of::<$field_ty>()*2}else{std::mem::size_of::<usize>()},depth:1,..Default::default()});}field_index+=1;)+
    let _=field_index;Err($crate::value::ValueError::literal($crate::value::ValueRefusalKind::InvariantViolated,"sparse exchange field index outside declared schema"))
   }
   fn exchange_field(&mut self,row:&mut $row,index:usize,_capability:$crate::ApplyCapability,grant:$crate::value::retained_clone::RetainedCloneGrant)->Result<$crate::value::retained_clone::RetainedCloneProgress,$crate::value::ValueError>{
    let demand=<Self as $crate::retained::patch::RetainedRowPatch<$row>>::exchange_demand(self,index)?;
    if !$crate::retained::retained_mutation_demand_fits(demand,grant){return Ok(Default::default());}
    let mut field_index=0;
    $(if index==field_index{if let Some(value)=self.$field.as_mut(){std::mem::swap(value,&mut row.$field);}return Ok($crate::value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});}field_index+=1;)+
    let _=field_index;unreachable!("quoted schema field index")
   }
  }
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
