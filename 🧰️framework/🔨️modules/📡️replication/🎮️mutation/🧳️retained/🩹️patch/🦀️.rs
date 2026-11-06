//! 🩹️ Typed sparse inversion and field-header ownership exchange never clone untouched rows.
use super::super::ApplyCapability;
#[path="🚚️apply/🦀️.rs"] mod apply;
pub use apply::RetainedRowPatchApply;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

/// 🧭️ Authored fields copy the immutable event; inverse fields copy the corresponding original row.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedPatchSource{Authored,Inverse}

/// 🧬️ Each sparse patch declares its original inverse fields and validated owned field exchange.
pub trait RetainedRowPatch<R>:Default+RetainedPatchEmpty+semio_framework_value::retirement::RetireOwned+Send+'static{
 type Fields:RetainedRowPatchFields<R,Self>;
 const FIELD_COUNT:usize;
 fn exchange_demand(&self,index:usize)->Result<RetirementDemand,ValueError>;
 fn exchange_field(&mut self,row:&mut R,index:usize,capability:ApplyCapability,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
}
/// 🌱️ The defining patch owner accounts for constructing every original empty field.
pub trait RetainedPatchEmpty:Sized{fn empty_patch_work_demand()->u64;fn empty_patch()->(Self,u64);}
/// 🌱️ Declares an empty patch whose complete fields are all absent options.
#[macro_export]
macro_rules! retained_empty_option_patch{
 ($patch:path {$($field:ident),* $(,)?})=>{
  impl $crate::retained::patch::RetainedPatchEmpty for $patch{
   fn empty_patch_work_demand()->u64{[$(stringify!($field)),*].len() as u64}
   fn empty_patch()->(Self,u64){(Self{$($field: ::core::option::Option::None),*},Self::empty_patch_work_demand())}
  }
 };
}

/// 🪡️ Inverse preparation retains genuine original row/patch sources until typed closure.
pub trait RetainedRowPatchFields<R,P:RetainedRowPatch<R>>:Send+'static{
 fn new_work_demand()->Result<u64,ValueError>;
 fn new(source:RetainedPatchSource)->(Self,u64);
 fn demand(&self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn advance_work_demand(&self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn advance(&mut self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome;
 fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<P>,RetainedCloneProgress),ValueError>;
 fn begin_close(&mut self);
 fn close_started(&self)->bool;
 fn begin_close_demand(&self)->RetirementDemand;
 fn begin_close_work_demand(&self)->u64;
 fn begin_close_paid(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,ValueError>;
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome;
 fn terminal_is_empty(&self)->bool;
}

/// 🏗️ Declares retained setters beside each technology's sparse row schema without semantic cold adapters.
#[macro_export]
macro_rules! retained_row_patch{
 ($vis:vis $owner:ident for $patch:ty => $row:ty {$($field:ident:$field_ty:ty),+$(,)?})=>{
  $vis mod $owner{
   use super::*;
   use $crate::retained::{RetainedFieldPreparation,retained_mutation_demand_fits};
    use $crate::retained::patch::{RetainedRowPatch,RetainedRowPatchFields,RetainedPatchSource,RetainedPatchEmpty};
   use $crate::value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
   /// 🧳️ Original sparse inverse fields and source aliases remain typed until complete settlement.
   pub struct Fields{
    $($field:Option<Box<RetainedFieldPreparation<$field_ty>>>,)+
    source:RetainedPatchSource,result:std::mem::ManuallyDrop<Option<$patch>>,retirement:RetainedCloneClose,phase:usize,payload:Option<RetainedCloneBinding>,row:Option<RetainedCloneBinding>,closing:bool,
   }
   impl RetainedRowPatchFields<$row,$patch> for Fields{
    fn new_work_demand()->Result<u64,ValueError>{<$patch as RetainedPatchEmpty>::empty_patch_work_demand().checked_add(<$patch as RetainedRowPatch<$row>>::FIELD_COUNT as u64+1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"sparse owner birth work overflow"))}
    fn new(source:RetainedPatchSource)->(Self,u64){let(result,work)=<$patch as RetainedPatchEmpty>::empty_patch();(Self{source,$($field:None,)+result:std::mem::ManuallyDrop::new(Some(result)),retirement:Default::default(),phase:0,payload:None,row:None,closing:false},work+<$patch as RetainedRowPatch<$row>>::FIELD_COUNT as u64+1)}
    fn demand(&self,patch:RetainedCloneRef<'_,$patch>,row:RetainedCloneRef<'_,$row>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
     if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse already closing"));}
     if maximum_work_units==0{return Ok(Default::default())}
     if self.payload.is_none(){return Ok(RetirementDemand{copy_bytes:patch.binding_copy_bytes(),depth:1,..Default::default()});}
     if self.row.is_none(){return Ok(RetirementDemand{copy_bytes:row.binding_copy_bytes(),depth:1,..Default::default()});}
     let mut index=0;
     $(if self.phase==index{
      if patch.get().$field.is_none(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<usize>(),depth:1,..Default::default()});}
      let Some(field)=self.$field.as_ref()else{return Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<RetainedFieldPreparation<$field_ty>>()+2*std::mem::size_of::<Option<Box<RetainedFieldPreparation<$field_ty>>>>(),capacity_bytes:std::mem::size_of::<RetainedFieldPreparation<$field_ty>>(),depth:1,..Default::default()});};
      if field.value().is_some(){let mut demand=field.exchange_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(std::mem::size_of::<Option<$field_ty>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"sparse inverse header extent overflow"))?;return Ok(demand);}
       let source=if self.source==RetainedPatchSource::Authored{patch.project(index,|patch|patch.$field.as_ref().expect("authored schema field"))}else{row.project(index,|row|&row.$field)};return field.demand(source,body,maximum_work_units);
     }index+=1;)+
     let _=index;Ok(RetirementDemand{copy_bytes:std::mem::size_of::<$patch>()*2,depth:1,..Default::default()})
    }
    fn advance_work_demand(&self,patch:RetainedCloneRef<'_,$patch>,row:RetainedCloneRef<'_,$row>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
     self.demand(patch,row,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.payload.is_none()||self.row.is_none(){return Ok(1)}let mut index=0;$(if self.phase==index{if patch.get().$field.is_none(){return Ok(1)}let Some(field)=self.$field.as_ref()else{return Ok(1)};if field.value().is_some(){return Ok(1)}let source=if self.source==RetainedPatchSource::Authored{patch.project(index,|patch|patch.$field.as_ref().expect("authored schema field"))}else{row.project(index,|row|&row.$field)};return field.advance_work_demand(source,body,maximum_work_units)}index+=1;)+let _=index;Ok(1)
    }
    fn advance(&mut self,patch:RetainedCloneRef<'_,$patch>,row:RetainedCloneRef<'_,$row>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->$crate::value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,$crate::value::OriginalRetirementFailure>=(||{
     let selected=self.advance_work_demand(patch,row,body,maximum_work_units)?;if selected==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.demand(patch,row,body,maximum_work_units)?;
     if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
     let turn=patch.bind(&mut self.payload,grant);work+=turn.work;if let Some(progress)=turn.result?{return Ok(RetainedCloneStep::Progress(progress));}
     let turn=row.bind(&mut self.row,grant);work+=turn.work;if let Some(progress)=turn.result?{return Ok(RetainedCloneStep::Progress(progress));}
     let mut index=0;
     $(if self.phase==index{
      if patch.get().$field.is_none(){work+=1;self.phase+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
      if self.$field.is_none(){work+=1;self.$field=Some(Box::new(RetainedFieldPreparation::default()));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}));}
      let field=self.$field.as_mut().expect("funded sparse field owner");
      if field.value().is_some(){
       let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-std::mem::size_of::<Option<$field_ty>>(),..grant};
       let(value,mut progress)=field.take(child)?;work+=1;self.result.as_mut().expect("untransferred sparse inverse").$field=value;
       progress.copied_bytes+=std::mem::size_of::<Option<$field_ty>>();self.phase+=1;return Ok(RetainedCloneStep::Progress(progress));
      }
       let source=if self.source==RetainedPatchSource::Authored{patch.project(index,|patch|patch.$field.as_ref().expect("authored schema field"))}else{row.project(index,|row|&row.$field)};let turn=field.advance(source,body,maximum_work_units,grant);work+=turn.work;let step=turn.result?;return Ok(RetainedCloneStep::Progress(step.progress()));
     }index+=1;)+
     let _=index;work+=1;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
     })();$crate::value::RetainedWorkOutcome{result,work}
    }
    fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<$patch>,RetainedCloneProgress),ValueError>{
     if self.closing||self.phase!=<$patch as RetainedRowPatch<$row>>::FIELD_COUNT{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse not ready for ownership transfer"));}
     let copy_bytes=std::mem::size_of::<$patch>()*2;
     if !retained_mutation_demand_fits(RetirementDemand{copy_bytes,depth:1,..Default::default()},grant){return Ok((None,Default::default()));}
     let value=self.result.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse ownership already transferred"))?;
     Ok((Some(value),RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}))
    }
    fn begin_close(&mut self){self.closing=true;}
    fn close_started(&self)->bool{self.closing}
    fn begin_close_demand(&self)->RetirementDemand{if self.closing{Default::default()}else{RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()}}}
    fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
    fn begin_close_paid(&mut self,grant:RetainedCloneGrant)->$crate::value::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{let demand=self.begin_close_demand();if self.closing||!retained_mutation_demand_fits(demand,grant){return $crate::value::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}self.closing=true;$crate::value::RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work:1}}
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
     $(if let Some(field)=self.$field.as_ref(){return if field.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<Box<RetainedFieldPreparation<$field_ty>>>>(),release_bytes:std::mem::size_of::<RetainedFieldPreparation<$field_ty>>(),depth:1,..Default::default()})}else if !field.close_started(){Ok(field.begin_close_demand())}else{field.close_demand(body)};})+
     if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<$patch>(false,&None,body);}
     if self.result.is_some(){return Ok(self.retirement.next_owner_demands::<$patch>(true,&None,body)?);}
     let binding=if self.row.is_some(){&self.row}else{&self.payload};
     RetainedCloneBinding::close_demand(binding,body)
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demand(body)?;$(if let Some(field)=self.$field.as_ref(){return if field.terminal_is_empty(){Ok(1)}else if !field.close_started(){Ok(field.begin_close_work_demand())}else{field.close_work_demand(body)}})+if !self.retirement.is_empty()||self.result.is_some(){return self.retirement.next_owner_work::<$patch>(self.result.is_some(),&None,body)}let binding=if self.row.is_some(){&self.row}else{&self.payload};RetainedCloneBinding::close_work_demand(binding,body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->$crate::value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,$crate::value::OriginalRetirementFailure>=(||{
     if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse inverse closure not started").into());}
     if !retained_mutation_demand_fits(self.close_demand(grant.maximum_copy_bytes)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
     $(if let Some(field)=self.$field.as_mut(){if field.terminal_is_empty(){work+=1;drop(self.$field.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*std::mem::size_of::<Option<Box<RetainedFieldPreparation<$field_ty>>>>(),released_bytes:std::mem::size_of::<RetainedFieldPreparation<$field_ty>>(),..Default::default()}));}if !field.close_started(){let turn=field.begin_close_paid(grant);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?));}let turn=field.close_step(grant);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));})+
     if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
     let turn=self.retirement.begin_granted(&mut self.result,grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(step.progress()));}
     let turn=RetainedCloneBinding::close_one(if self.row.is_some(){&mut self.row}else{&mut self.payload},grant);work+=turn.work;let step=turn.result?;
     Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
     })();$crate::value::RetainedWorkOutcome{result,work}
    }
    fn terminal_is_empty(&self)->bool{self.result.is_none()&&self.retirement.is_empty()&&self.payload.is_none()&&self.row.is_none()$(&&self.$field.is_none())+}
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
