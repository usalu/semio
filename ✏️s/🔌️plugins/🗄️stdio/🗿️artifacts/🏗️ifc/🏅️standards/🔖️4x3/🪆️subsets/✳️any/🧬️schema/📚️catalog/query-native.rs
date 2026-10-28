//! 🫴️ Scoped ADD2 queries reuse the shared finite kernel inside the original native receiving owner.
use super::{Ifc4x3AttributeMetadata,IfcAttributeQueryResult,IfcAttributeQueryWork,IfcSnapshot,IfcValue,metadata};
use semio_framework_os_kernel::NativeSnapshotDecodeOwner;
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
fn admit(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,demand:RetirementDemand)->Result<(),ValueError>{
 owner.native().checkpoint()?;
 let remaining=owner.remaining_grant();
 if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original native attribute query exhausted its work grant"));}
 if demand.depth>remaining.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original native attribute query exceeds its depth grant"));}
 if !retained_clone_grant_admits(remaining,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native attribute query exceeds its physical grant"));}
 Ok(())
}
fn performed(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,demand:RetirementDemand)->Result<(),ValueError>{owner.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}
/// 🔎️ Lends exact source slots for one callback while query metadata and owned output retain original custody.
pub fn with_ifc4x3_native_attribute<O:RetireOwned+'static>(
 source:&IfcSnapshot,row:usize,name:&str,qualified:Option<&str>,epoch:u64,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,
 visit:impl for<'value> FnOnce(IfcAttributeQueryResult<'value,IfcValue,metadata::StaticAttribute,metadata::StaticTypeExpression>,&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<O,ValueError>
)->Result<O,ValueError>{
 let demand=IfcAttributeQueryWork::core_demands();admit(owner,demand)?;
 owner.scoped_stage(|original|{original.native().begin_stage(0)?;original.receive_nested::<IfcAttributeQueryWork,O>(|slot,original|{
  admit(original,demand)?;*slot=Some(IfcAttributeQueryWork::default());performed(original,demand)?;original.native().step()?;
  loop{
   let work=slot.as_mut().unwrap();work.check_core::<Ifc4x3AttributeMetadata,_>(source,row,name,qualified,epoch)?;let demand=work.demands();admit(original,demand)?;
   let result=work.advance_core::<Ifc4x3AttributeMetadata,_>(source,row,name,qualified,epoch);performed(original,demand)?;let done=result?;original.native().step()?;if done{break;}
  }
  let work=slot.as_ref().unwrap();let demand=work.demands();admit(original,demand)?;
  let result=work.completed_core::<Ifc4x3AttributeMetadata,_>(source,row,name,qualified,epoch);performed(original,demand)?;
  let found=result?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original native query completed without its borrowed result"))?;
  original.native().step()?;visit(found,original)
 })})
}

