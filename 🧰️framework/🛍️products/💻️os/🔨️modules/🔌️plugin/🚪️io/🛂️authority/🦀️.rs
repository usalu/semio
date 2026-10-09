//! 🛂️ Guest producers borrow original operation progress and preallocation admission imports.
use semio_framework_value::{ValueError, ValueRefusalKind};
#[path="📤️result/🦀️.rs"]
pub mod owned_result;

#[cfg(all(target_arch="wasm32",target_env="p2"))]
mod return_types{pub use crate::component::wasip2::semio::framework::{effects,ui,types,pure};pub use crate::component::wasip2::exports::semio::framework::{reactor,codec};}
#[cfg(all(target_arch="wasm32",target_env="p2"))]
#[path="📤️return/🦀️.rs"]
pub mod return_walk;

/// 🔢️ Carries typed native refusal kinds through the neutral fixed-width host protocol.
pub fn refusal_code(kind: ValueRefusalKind) -> u32 {
    match kind { ValueRefusalKind::Canceled => 1, ValueRefusalKind::OwnershipLimit => 2, ValueRefusalKind::WorkLimit => 3, ValueRefusalKind::InvariantViolated => 4, ValueRefusalKind::AllocationFailed => 5, ValueRefusalKind::UnsupportedOwner => 6, ValueRefusalKind::DepthLimit => 7, ValueRefusalKind::InvalidValue => 8 }
}

/// 🧾️ Refuses an invalid or rejected original host admission without synthesizing a guest grant.
pub fn refusal(code: u32) -> ValueError {
    let kind = match code { 1 => ValueRefusalKind::Canceled, 2 => ValueRefusalKind::OwnershipLimit, 3 => ValueRefusalKind::WorkLimit, 4 => ValueRefusalKind::InvariantViolated, 5 => ValueRefusalKind::AllocationFailed, 6 => ValueRefusalKind::UnsupportedOwner, 7 => ValueRefusalKind::DepthLimit, 8 => ValueRefusalKind::InvalidValue, _ => ValueRefusalKind::InvariantViolated };
    ValueError::literal(kind, "original host operation admission refused the guest producer")
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
fn admitted(code: u32) -> Result<(), ValueError> { if code == 0 { Ok(()) } else { Err(refusal(code)) } }

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
/// 🌉️ Carries one original host allocation port through all synchronous retained authoring hops.
pub fn with_operation_authority<T>(operation: impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<T, crate::Fault>) -> Result<T, crate::Fault> {
    use crate::component::wasip2::semio::framework::pure;
    use semio_framework_value::native_encoding::{NativeEncodeAllocation, NativeEncodeProgress};
    let fault = |error: ValueError| crate::Fault::from(error.to_string());
    let maximum = usize::try_from(pure::operation_begin().map_err(refusal).map_err(fault)?).map_err(|_| fault(refusal(2)))?;
    let mut observer = |next: NativeEncodeProgress| pure::operation_progress(next.completed as u64, next.total as u64, next.owned_bytes as u64) == 0;
    let mut allocate = |next: NativeEncodeAllocation| admitted(pure::operation_allocation(next.bytes as u64, next.owned_bytes as u64, next.next_owned_bytes as u64, next.maximum_bytes as u64));
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new_forwarded(maximum, &mut observer, &mut allocate).map_err(fault)?;
    let result = operation(&mut identity);
    let receipt = identity.pause_forwarded().map_err(fault)?;
    let returned = admitted(pure::operation_finish(receipt.owned_bytes() as u64)).map_err(fault);
    match result { Err(error) => Err(error), Ok(output) => { returned?; Ok(output) } }
}

#[cfg(all(target_arch="wasm32",target_env="p2"))]
/// 📤️ Returns original refused scalars before constructing any diagnostic or owned JSON result.
pub fn with_owned_operation_authority<T>(operation:impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<T,crate::Fault>)->Result<Result<T,crate::Fault>,ValueError>{
    use crate::component::wasip2::semio::framework::pure;
    use semio_framework_value::native_encoding::{NativeEncodeAllocation,NativeEncodeProgress};
    let maximum=usize::try_from(pure::operation_begin().map_err(refusal)?).map_err(|_|refusal(2))?;
    let rejected=std::cell::Cell::new(0);
    let mut observer=|next:NativeEncodeProgress|{let code=pure::operation_progress(next.completed as u64,next.total as u64,next.owned_bytes as u64);if code!=0{rejected.set(code);}code==0};
    let mut allocate=|next:NativeEncodeAllocation|{let code=pure::operation_allocation(next.bytes as u64,next.owned_bytes as u64,next.next_owned_bytes as u64,next.maximum_bytes as u64);if code!=0{rejected.set(code);}admitted(code)};
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new_forwarded(maximum,&mut observer,&mut allocate)?;
    let result=operation(&mut identity);
    if rejected.get()!=0{return Err(refusal(rejected.get()));}
    let receipt=identity.pause_forwarded()?;
    admitted(pure::operation_finish(receipt.owned_bytes()as u64))?;
    Ok(result)
}

#[cfg(all(target_arch="wasm32",target_env="p2"))]
/// 📤️ Reserves actual native return fields before finishing the original guest authoring phase.
pub fn with_wit_operation_authority<T>(operation:impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<T,crate::Fault>,walk:impl FnOnce(&T,&mut dyn FnMut(return_types::pure::OperationReturnAllocation,usize)->Result<(),ValueError>)->Result<(),ValueError>)->Result<T,return_types::types::PluginError>{
    use return_types::{pure,types::PluginError};
    use semio_framework_value::native_encoding::{NativeEncodeAllocation,NativeEncodeProgress};
    let rejected=|error:ValueError|PluginError::OperationRefusal(refusal_code(error.kind));
    let maximum=usize::try_from(pure::operation_begin().map_err(|code|rejected(refusal(code)))?).map_err(|_|rejected(refusal(2)))?;
    let rejected_code=std::cell::Cell::new(0);
    let mut observer=|next:NativeEncodeProgress|{let code=pure::operation_progress(next.completed as u64,next.total as u64,next.owned_bytes as u64);if code!=0{rejected_code.set(code);}code==0};
    let mut allocate=|next:NativeEncodeAllocation|{let code=pure::operation_allocation(next.bytes as u64,next.owned_bytes as u64,next.next_owned_bytes as u64,next.maximum_bytes as u64);if code!=0{rejected_code.set(code);}admitted(code)};
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new_forwarded(maximum,&mut observer,&mut allocate).map_err(rejected)?;
    let result=operation(&mut identity);
    if rejected_code.get()!=0{return Err(rejected(refusal(rejected_code.get())));}
    let result=result.map_err(|fault|crate::component::wasip2::plugin_error(&fault));
    let mut reserve=|kind,count:usize|admitted(pure::operation_reserve_return(kind,count as u64));
    let reservation=match &result{Ok(output)=>walk(output,&mut reserve),Err(error)=>return_walk::walk_error(error,&mut reserve)};
    let receipt=identity.pause_forwarded().map_err(rejected)?;
    let finished=admitted(pure::operation_finish(receipt.owned_bytes()as u64));
    reservation.map_err(rejected)?;
    finished.map_err(rejected)?;
    result
}

#[cfg(all(target_arch="wasm32",target_env="p2"))]
#[path="🪶️snapshot/🦀️.rs"]
mod snapshot;
#[cfg(all(target_arch="wasm32",target_env="p2"))]
pub use snapshot::{with_snapshot_authority,snapshot_retirement,snapshot_close,snapshot_take,SnapshotReturn};
