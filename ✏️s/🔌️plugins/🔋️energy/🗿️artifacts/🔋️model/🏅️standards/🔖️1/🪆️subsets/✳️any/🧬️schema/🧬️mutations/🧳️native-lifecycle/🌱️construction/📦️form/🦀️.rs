//! 📦️ Immutable rows and owned inverse records share typed original field assembly with separate actual output births.
use super::*;
/// 🧩️ An output form admits its exact defining owner before any prepared fields move.
pub trait RowBirthForm<R:RetireOwned+Send+Sync+'static>:Send+Sync+'static{
 type Output:RetireOwned+Send+Sync;type Admission;
 fn demand()->RetirementDemand;
 fn prepare(g:RetainedCloneGrant)->Result<Self::Admission,ValueError>;
 fn materialize(admission:Self::Admission,row:R)->(Self::Output,RetainedCloneProgress);
}
/// 🌱️ Persisted rows receive a genuine separately admitted immutable allocation.
pub struct SealedRowForm;
impl<R:RetireOwned+Send+Sync+'static> RowBirthForm<R> for SealedRowForm{
 type Output=SealedShared<R>;type Admission=SealedSharedBirth<R>;
 fn demand()->RetirementDemand{RetirementDemand{copy_bytes:4*size_of::<R>()+4*size_of::<Option<Self::Output>>()+2*size_of::<SealedSharedBirth<R>>()+1,capacity_bytes:SealedShared::<R>::birth_bytes(),depth:1,..Default::default()}}
 fn prepare(g:RetainedCloneGrant)->Result<Self::Admission,ValueError>{SealedShared::<R>::prepare_birth(g)}
 fn materialize(b:Self::Admission,row:R)->(Self::Output,RetainedCloneProgress){b.materialize(row,SharedIssuer::owned())}
}
/// ↩️ An inverse payload transfers its already funded field owners as the actual owned record.
pub struct OwnedRecordForm;
impl<R:RetireOwned+Send+Sync+'static> RowBirthForm<R> for OwnedRecordForm{
 type Output=R;type Admission=();
 fn demand()->RetirementDemand{scalar(4*size_of::<R>()+4*size_of::<Option<R>>()+1)}
 fn prepare(g:RetainedCloneGrant)->Result<(),ValueError>{if !retained_mutation_demand_fits(<Self as RowBirthForm<R>>::demand(),g){return Err(invalid());}Ok(())}
 fn materialize(_:(),row:R)->(R,RetainedCloneProgress){(row,Default::default())}
}

/// 🔢️ A bounded native array inverse funds its exact vector backing before fixed field construction.
pub struct OwnedFixedVectorForm<T:crate::scalar::NativeScalar+'static,const N:usize>(std::marker::PhantomData<T>);
impl<T:crate::scalar::NativeScalar+'static,const N:usize> RowBirthForm<Vec<T>> for OwnedFixedVectorForm<T,N>{
 type Output=Vec<T>;type Admission=();
 fn demand()->RetirementDemand{RetirementDemand{copy_bytes:4*size_of::<Vec<T>>()+4*size_of::<Option<Vec<T>>>()+2*N*size_of::<T>()+1,capacity_bytes:N*size_of::<T>(),depth:1,..Default::default()}}
 fn prepare(g:RetainedCloneGrant)->Result<(),ValueError>{if !retained_mutation_demand_fits(Self::demand(),g){return Err(invalid());}Ok(())}
 fn materialize(_:(),row:Vec<T>)->(Vec<T>,RetainedCloneProgress){assert_eq!(row.len(),N);assert_eq!(row.capacity(),N);(row,RetainedCloneProgress{retained_capacity_bytes:N*size_of::<T>(),..Default::default()})}
}
