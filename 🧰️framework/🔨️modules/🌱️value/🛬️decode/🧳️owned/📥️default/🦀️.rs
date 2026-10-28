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
empty_default!(() => (),bool => false,u8 => 0,u16 => 0,u32 => 0,u64 => 0,usize => 0,i8 => 0,i16 => 0,i32 => 0,i64 => 0,isize => 0,f64 => 0.0,String => String::new());
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

impl<T:RetireOwned,const N:usize> NativeDefault for semio_framework_value_persistent::list::PagedList<T,N>{fn native_default_demand()->RetirementDemand{super::tree::fixed(2*std::mem::size_of::<Self>())}fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::native_default_demand();if !retained_clone_grant_admits(grant,d){return Err(super::tree::invalid())}Ok((Self::new(),super::tree::receipt(d)))}}
