//! 📥️ Missing authored fields construct only declared native defaults under their actual grant.
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,retirement::RetireOwned,retained_clone::retained_clone_grant_admits};

/// 🌱️ Required default construction declares its physical birth independently of arbitrary `Default` implementations.
pub trait NativeDefault:RetireOwned+Sized {
    fn native_default_demand()->RetirementDemand;
    fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>;
}

macro_rules! empty_default {
    ($($ty:ty=>$value:expr),*$(,)?)=>{$(
        impl NativeDefault for $ty {
            fn native_default_demand()->RetirementDemand { super::tree::fixed(2*std::mem::size_of::<Self>()) }
            fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError> {
                let demand=Self::native_default_demand();
                if !retained_clone_grant_admits(grant,demand) { return Err(super::tree::invalid()); }
                Ok(($value,super::tree::receipt(demand)))
            }
        }
    )*};
}
empty_default!(() => (),bool => false,u8 => 0,u16 => 0,u32 => 0,u64 => 0,usize => 0,i8 => 0,i16 => 0,i32 => 0,i64 => 0,isize => 0,f32 => 0.0,f64 => 0.0,String => String::new());
impl<T:RetireOwned> NativeDefault for Vec<T> {
    fn native_default_demand()->RetirementDemand { super::tree::fixed(2*std::mem::size_of::<Self>()) }
    fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError> {
        let demand=Self::native_default_demand();
        if !retained_clone_grant_admits(grant,demand) { return Err(super::tree::invalid()); }
        Ok((Vec::new(),super::tree::receipt(demand)))
    }
}
impl<T:RetireOwned> NativeDefault for Option<T> {
    fn native_default_demand()->RetirementDemand { super::tree::fixed(2*std::mem::size_of::<Self>()) }
    fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError> {
        let demand=Self::native_default_demand();
        if !retained_clone_grant_admits(grant,demand) { return Err(super::tree::invalid()); }
        Ok((None,super::tree::receipt(demand)))
    }
}

/// 👯️ A pair receives each item's own native default under one inline pair header.
impl<A:NativeDefault,B:NativeDefault> NativeDefault for (A,B) {
    fn native_default_demand()->RetirementDemand {
        let(first,second)=(A::native_default_demand(),B::native_default_demand());
        RetirementDemand{copy_bytes:2*std::mem::size_of::<Self>()+first.copy_bytes+second.copy_bytes,capacity_bytes:first.capacity_bytes+second.capacity_bytes,release_bytes:first.release_bytes+second.release_bytes,depth:1+first.depth.max(second.depth),..Default::default()}
    }
    fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError> {
        let demand=Self::native_default_demand();
        if !retained_clone_grant_admits(grant,demand) { return Err(super::tree::invalid()); }
        let header=2*std::mem::size_of::<Self>();
        let(first,first_progress)=A::native_default(super::tree::child_grant(grant,header))?;
        let(second,second_progress)=B::native_default(super::tree::child_grant(grant,header+first_progress.copied_bytes))?;
        Ok(((first,second),RetainedCloneProgress{copied_items:1,copied_bytes:header+first_progress.copied_bytes+second_progress.copied_bytes,retained_capacity_bytes:first_progress.retained_capacity_bytes+second_progress.retained_capacity_bytes,released_bytes:first_progress.released_bytes+second_progress.released_bytes}))
    }
}
impl<T:RetireOwned,const N:usize> NativeDefault for semio_framework_value_persistent::list::PagedList<T,N>{fn native_default_demand()->RetirementDemand{super::tree::fixed(2*std::mem::size_of::<Self>())}fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::native_default_demand();if !retained_clone_grant_admits(grant,d){return Err(super::tree::invalid())}Ok((Self::new(),super::tree::receipt(d)))}}
