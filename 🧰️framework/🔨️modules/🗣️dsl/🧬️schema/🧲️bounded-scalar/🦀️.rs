//! 🧲️ Bounded fixed-shape leaves receive through one declaration: the macro adds both receiving families over a pure conversion.
#[macro_export]
macro_rules! dsl_owned_scalar_receiving{
 (copy $name:ty,$convert:expr)=>{
  impl $crate::receiving::owned::scalar_sealed::Sealed for $name{}
  impl $crate::receiving::owned::OwnedScalar for $name{
   fn creation_bytes()->usize{::core::mem::size_of::<Self>()}
   fn take_source(source:&mut Option<$crate::FieldValue>)->Result<Self,$crate::receiving::owned::ValueError>{let convert:fn(&$crate::FieldValue)->Result<Self,$crate::receiving::owned::ValueError>=$convert;convert(source.as_ref().ok_or_else($crate::receiving::owned::invalid)?)}
  }
  impl $crate::receiving::owned::DslOwnedReceivingField for $name{type Cursor=$crate::receiving::owned::ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl $crate::DslReceivingField for $name{fn receive_value(value:&$crate::FieldValue,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::owned::RetainedWorkOutcome<Self,$crate::receiving::NativeSnapshotFailure>{
   let convert:fn(&$crate::FieldValue)->Result<Self,$crate::receiving::owned::ValueError>=$convert;
   $crate::receiving::receiving_turn(|meter|{meter.admit(owner.native())?;let output=convert(value).map_err($crate::receiving::NativeSnapshotFailure::from)?;$crate::receiving::work(owner,::core::mem::size_of::<Self>(),1)?;Ok(output)})
  }}
 };
 (move $name:ty,$payload:expr,$convert:expr,$take:expr)=>{
  impl $crate::receiving::owned::scalar_sealed::Sealed for $name{}
  impl $crate::receiving::owned::OwnedScalar for $name{
   fn creation_bytes()->usize{0}
   fn take_source(source:&mut Option<$crate::FieldValue>)->Result<Self,$crate::receiving::owned::ValueError>{let take:fn(&mut Option<$crate::FieldValue>)->Result<Self,$crate::receiving::owned::ValueError>=$take;take(source)}
  }
  impl $crate::receiving::owned::DslOwnedReceivingField for $name{type Cursor=$crate::receiving::owned::ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl $crate::DslReceivingField for $name{fn receive_value(value:&$crate::FieldValue,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::owned::RetainedWorkOutcome<Self,$crate::receiving::NativeSnapshotFailure>{
   let payload:fn(&$crate::FieldValue)->usize=$payload;let convert:fn(&$crate::FieldValue)->Result<Self,$crate::receiving::owned::ValueError>=$convert;
   $crate::receiving::receiving_turn(|meter|{meter.admit(owner.native())?;$crate::receiving::work(owner,payload(value).max(::core::mem::size_of::<Self>()),1)?;let output=convert(value).map_err($crate::receiving::NativeSnapshotFailure::from)?;Ok(output)})
  }}
 };
}
