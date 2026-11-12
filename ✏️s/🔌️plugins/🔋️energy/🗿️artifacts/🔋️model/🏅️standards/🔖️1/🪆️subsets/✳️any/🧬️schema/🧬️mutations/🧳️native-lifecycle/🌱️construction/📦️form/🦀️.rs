//! 📦️ Immutable rows and owned inverse records share typed original field assembly with separate actual output births.
use super::*;
/// 🧩️ An output form admits its exact defining owner before any prepared fields move.
pub trait RowBirthForm<R:RetireOwned+Send+Sync+'static>:Send+Sync+'static{
 type Output:RetireOwned+Send+Sync;
 fn quote()->Result<Quote,ValueError>;
 fn materialize(raw:&mut Option<R>,output:&mut Option<Self::Output>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>;
}
/// 🌱️ Persisted rows receive a genuine separately admitted immutable allocation.
pub struct SealedRowForm;
impl<R:RetireOwned+Send+Sync+'static> RowBirthForm<R> for SealedRowForm{
 type Output=SealedShared<R>;
 fn quote()->Result<Quote,ValueError>{let mut d=SealedShared::<R>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<R>>()+1).ok_or_else(invalid)?;Ok((d,SealedShared::<R>::source_admission_work_demand()))}
 fn materialize(raw:&mut Option<R>,output:&mut Option<Self::Output>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{SealedShared::admit_from(raw,SharedIssuer::owned(),output,effect)}
}
/// ↩️ An inverse payload transfers its already funded field owners as the actual owned record.
pub struct OwnedRecordForm;
impl<R:RetireOwned+Send+Sync+'static> RowBirthForm<R> for OwnedRecordForm{
 type Output=R;
 fn quote()->Result<Quote,ValueError>{Ok((scalar(4*size_of::<R>()+4*size_of::<Option<R>>()+1),1))}
 fn materialize(raw:&mut Option<R>,output:&mut Option<R>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{let d=scalar(4*size_of::<R>()+4*size_of::<Option<R>>()+1);if !retained_mutation_demand_fits(d,effect.grant()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=raw.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}
}

/// 🔢️ A bounded native array inverse funds its exact vector backing before fixed field construction.
pub struct OwnedFixedVectorForm<T:crate::scalar::NativeScalar+'static,const N:usize>(std::marker::PhantomData<T>);
impl<T:crate::scalar::NativeScalar+'static,const N:usize> OwnedFixedVectorForm<T,N>{fn demand()->RetirementDemand{RetirementDemand{copy_bytes:4*size_of::<Vec<T>>()+4*size_of::<Option<Vec<T>>>()+2*N*size_of::<T>()+1,capacity_bytes:N*size_of::<T>(),depth:1,..Default::default()}}}
impl<T:crate::scalar::NativeScalar+'static,const N:usize> RowBirthForm<Vec<T>> for OwnedFixedVectorForm<T,N>{
 type Output=Vec<T>;
 fn quote()->Result<Quote,ValueError>{Ok((Self::demand(),1))}
 fn materialize(raw:&mut Option<Vec<T>>,output:&mut Option<Vec<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{if !retained_mutation_demand_fits(Self::demand(),effect.grant()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let row=raw.take();if let Some(row)=&row{assert_eq!(row.len(),N);assert_eq!(row.capacity(),N);}*output=row;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{retained_capacity_bytes:N*size_of::<T>(),..receipt(scalar(1))})),work:1}}
}
