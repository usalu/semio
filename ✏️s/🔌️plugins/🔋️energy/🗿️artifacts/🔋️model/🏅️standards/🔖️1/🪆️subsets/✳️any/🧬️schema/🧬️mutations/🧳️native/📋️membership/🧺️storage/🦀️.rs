//! 🧺️ Fixed copied items retain their actual vector or array builder until one paid final header transfer.
use super::*;
use semio_framework_value::retirement::RetireOwned;
/// 📐️ Collection storage quotes its genuine backing independently of the selected semantic intent.
pub trait MembershipValues<I:NativeScalar>:RetireOwned+Send+Sync+'static{
 type Builder:RetireOwned+Send+Sync;
 fn len(&self)->usize;
 fn item(&self,index:usize)->I;
 fn birth_demand(count:usize)->Result<RetirementDemand,ValueError>;
 fn begin(count:usize)->Result<Self::Builder,ValueError>;
 fn push(builder:&mut Self::Builder,index:usize,item:I)->Result<(),ValueError>;
 fn finish(builder:Self::Builder)->Result<Self,ValueError> where Self:Sized;
}
impl<I:NativeScalar+'static> MembershipValues<I> for Vec<I>{
 type Builder=Vec<I>;
 fn len(&self)->usize{Vec::len(self)}fn item(&self,index:usize)->I{self[index]}
 fn birth_demand(count:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<Self>(),capacity_bytes:count.checked_mul(size_of::<I>()).filter(|n|*n<=isize::MAX as usize).ok_or_else(invalid)?,depth:1,..Default::default()})}
 fn begin(count:usize)->Result<Self::Builder,ValueError>{Self::birth_demand(count)?;Ok(Vec::with_capacity(count))}
 fn push(builder:&mut Self::Builder,index:usize,item:I)->Result<(),ValueError>{if index!=builder.len()||size_of::<I>()>0&&builder.len()==builder.capacity(){return Err(invalid())}builder.push(item);Ok(())}
 fn finish(builder:Self::Builder)->Result<Self,ValueError>{Ok(builder)}
}
impl<I:NativeScalar+'static,const N:usize> MembershipValues<I> for [I;N]{
 type Builder=[Option<I>;N];
 fn len(&self)->usize{N}fn item(&self,index:usize)->I{self[index]}
 fn birth_demand(count:usize)->Result<RetirementDemand,ValueError>{if count!=N{return Err(invalid())}Ok(scalar(2*size_of::<Self::Builder>()))}
 fn begin(count:usize)->Result<Self::Builder,ValueError>{Self::birth_demand(count)?;Ok([None;N])}
 fn push(builder:&mut Self::Builder,index:usize,item:I)->Result<(),ValueError>{let slot=builder.get_mut(index).ok_or_else(invalid)?;if slot.is_some(){return Err(invalid())}*slot=Some(item);Ok(())}
 fn finish(builder:Self::Builder)->Result<Self,ValueError>{if builder.iter().any(Option::is_none){return Err(invalid())}Ok(builder.map(|item|item.expect("all original fixed item slots are initialized")))}
}
