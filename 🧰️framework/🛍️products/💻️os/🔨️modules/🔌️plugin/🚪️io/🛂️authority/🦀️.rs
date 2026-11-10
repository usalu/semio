//! 🛂️ Guest producers borrow original operation progress and preallocation admission imports.
use semio_framework_value::{ValueError, ValueRefusalKind};
#[path="📤️result/🦀️.rs"]
pub mod owned_result;

#[cfg(all(target_arch="wasm32",target_env="p2"))]
mod return_types{pub use crate::component::wasip2::semio::framework::{effects,ui,types,pure};pub use crate::component::wasip2::exports::semio::framework::{reactor,codec};}
#[path="📤️return/🦀️.rs"]
mod return_walk_template;
#[cfg(all(target_arch="wasm32",target_env="p2"))]
pub mod return_walk{crate::return_walk_template!();}

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
type IdentityObserver<'a> = semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'a>;

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
type Identity<'a> = semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'a>;

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
/// 🛂️ The guest forwards each original allocation admission or quote to the host import and settles posts and cancels without a second host call.
pub struct HostAllocation { rejected: std::rc::Rc<std::cell::Cell<u32>>, reserving: Option<(std::rc::Rc<std::cell::Cell<usize>>, usize)> }

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
impl HostAllocation {
    /// 🎟️ Forwards each request with the numbers its own ledger quoted.
    pub fn new(rejected: std::rc::Rc<std::cell::Cell<u32>>) -> Self { Self { rejected, reserving: None } }
    /// 🧮️ Charges every request against one shared operation ledger bounded by the original maximum.
    pub fn reserving(rejected: std::rc::Rc<std::cell::Cell<u32>>, owned: std::rc::Rc<std::cell::Cell<usize>>, maximum: usize) -> Self { Self { rejected, reserving: Some((owned, maximum)) } }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
impl semio_framework_value_native_owned::native_decoding::NativeAllocationAuthorityPort for HostAllocation {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn receive<'reply, 'turn, 'source>(&mut self, _: &mut (), event: semio_framework_value_native_owned::native_decoding::NativeAllocationEvent, reply: semio_framework_value_native_owned::native_decoding::NativeAllocationReply<'reply>) -> Result<semio_framework_value_native_owned::native_decoding::NativeAllocationReplyComplete<'reply>, ValueError> where 'source: 'turn {
        use semio_framework_value_native_owned::native_decoding::NativeAllocationEvent::{Admit, Quote};
        let (Admit(next) | Quote(next)) = event else { return Ok(reply.complete(Ok(()))) };
        let (previous, after, maximum) = match &self.reserving {
            Some((owned, maximum)) => {
                let previous = owned.get();
                let Some(after) = previous.checked_add(next.bytes).filter(|after| after <= maximum) else { return Ok(reply.complete(Err(refusal(2).into()))) };
                (previous, after, *maximum)
            }
            None => (next.owned_bytes, next.next_owned_bytes, next.maximum_bytes),
        };
        let code = crate::component::wasip2::semio::framework::pure::operation_allocation(next.bytes as u64, previous as u64, after as u64, maximum as u64);
        if code != 0 { self.rejected.set(code); } else if let Some((owned, _)) = &self.reserving { owned.set(after); }
        Ok(reply.complete(admitted(code).map_err(Into::into)))
    }
    fn recover<'reply, 'turn, 'source>(&mut self, _: &mut (), _: semio_framework_value_native_owned::native_decoding::NativeAllocationEvent, _: semio_framework_value_native_owned::native_decoding::NativeAllocationReply<'reply>) -> Result<semio_framework_value_native_owned::native_decoding::NativeAllocationReplyComplete<'reply>, ValueError> where 'source: 'turn { Err(refusal(4)) }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
type Authority = semio_framework_value_native_owned::native_decoding::NativeAllocationAuthority<HostAllocation>;

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
thread_local! { static PARKED: std::cell::RefCell<Vec<std::mem::ManuallyDrop<Authority>>> = const { std::cell::RefCell::new(Vec::new()) }; }

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
/// 🅿️ Retries every authority whose transaction was still open at release; the next host call settles what the earlier one could not.
pub(crate) fn settle_parked() {
    let parked = PARKED.with(|slot| std::mem::take(&mut *slot.borrow_mut()));
    for authority in parked {
        if let Err(owner) = std::mem::ManuallyDrop::into_inner(authority).into_port() { PARKED.with(|slot| slot.borrow_mut().push(std::mem::ManuallyDrop::new(owner))); }
    }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
/// 🧾️ Returns the host port once no transaction remains; an open transaction keeps its state parked and is retried by the next host call.
pub(crate) fn release(authority: Authority) {
    if let Err(owner) = authority.into_port() { PARKED.with(|slot| slot.borrow_mut().push(std::mem::ManuallyDrop::new(owner))); }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
/// 🧭️ Runs one operation under an identity authority whose allocations the host admits through its original import.
fn forwarded<R>(maximum: usize, rejected: &std::rc::Rc<std::cell::Cell<u32>>, observer: &mut IdentityObserver<'_>, operation: impl FnOnce(Identity<'_>) -> R) -> Result<R, ValueError> {
    settle_parked();
    let mut authority = Authority::new(HostAllocation::new(rejected.clone()));
    let result = authority.with_send_loan(&mut (), |loan| Ok::<R, ValueError>(operation(Identity::new_forwarded(maximum, observer, loan)?)));
    release(authority);
    result.and_then(|inner| inner)
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
/// 🌉️ Carries one original host allocation port through all synchronous retained authoring hops.
pub fn with_operation_authority<T>(operation: impl FnOnce(&mut Identity<'_>) -> Result<T, crate::Fault>) -> Result<T, crate::Fault> {
    use crate::component::wasip2::semio::framework::pure;
    use semio_framework_value::native_encoding::NativeEncodeProgress;
    let fault = |error: ValueError| crate::Fault::from(error.to_string());
    let maximum = usize::try_from(pure::operation_begin().map_err(refusal).map_err(fault)?).map_err(|_| fault(refusal(2)))?;
    let mut observer = |next: NativeEncodeProgress| pure::operation_progress(next.completed as u64, next.total as u64, next.owned_bytes as u64) == 0;
    let rejected = std::rc::Rc::new(std::cell::Cell::new(0));
    forwarded(maximum, &rejected, &mut observer as &mut IdentityObserver<'_>, |mut identity| {
        let result = operation(&mut identity);
        let receipt = identity.pause_forwarded().map_err(fault)?;
        let returned = admitted(pure::operation_finish(receipt.owned_bytes() as u64)).map_err(fault);
        match result { Err(error) => Err(error), Ok(output) => { returned?; Ok(output) } }
    }).map_err(fault)?
}

#[cfg(all(target_arch="wasm32",target_env="p2"))]
/// 📤️ Returns original refused scalars before constructing any diagnostic or owned JSON result.
pub fn with_owned_operation_authority<T>(operation:impl FnOnce(&mut Identity<'_>)->Result<T,crate::Fault>)->Result<Result<T,crate::Fault>,ValueError>{
    use crate::component::wasip2::semio::framework::pure;
    use semio_framework_value::native_encoding::NativeEncodeProgress;
    let maximum=usize::try_from(pure::operation_begin().map_err(refusal)?).map_err(|_|refusal(2))?;
    let rejected=std::rc::Rc::new(std::cell::Cell::new(0));
    let mut observer=|next:NativeEncodeProgress|{let code=pure::operation_progress(next.completed as u64,next.total as u64,next.owned_bytes as u64);if code!=0{rejected.set(code);}code==0};
    forwarded(maximum,&rejected,&mut observer as &mut IdentityObserver<'_>,|mut identity|{
        let result=operation(&mut identity);
        if rejected.get()!=0{return Err(refusal(rejected.get()));}
        let receipt=identity.pause_forwarded()?;
        admitted(pure::operation_finish(receipt.owned_bytes()as u64))?;
        Ok(result)
    }).and_then(|inner|inner)
}

#[cfg(all(target_arch="wasm32",target_env="p2"))]
/// 📤️ Reserves actual native return fields before finishing the original guest authoring phase.
pub fn with_wit_operation_authority<T>(operation:impl FnOnce(&mut Identity<'_>)->Result<T,crate::Fault>,walk:impl FnOnce(&T,&mut dyn FnMut(return_types::pure::OperationReturnAllocation,usize)->Result<(),ValueError>)->Result<(),ValueError>)->Result<T,return_types::types::PluginError>{
    use return_types::{pure,types::PluginError};
    use semio_framework_value::native_encoding::NativeEncodeProgress;
    let rejected=|error:ValueError|PluginError::OperationRefusal(refusal_code(error.kind));
    let maximum=usize::try_from(pure::operation_begin().map_err(|code|rejected(refusal(code)))?).map_err(|_|rejected(refusal(2)))?;
    let rejected_code=std::rc::Rc::new(std::cell::Cell::new(0));
    let mut observer=|next:NativeEncodeProgress|{let code=pure::operation_progress(next.completed as u64,next.total as u64,next.owned_bytes as u64);if code!=0{rejected_code.set(code);}code==0};
    forwarded(maximum,&rejected_code,&mut observer as &mut IdentityObserver<'_>,|mut identity|{
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
    }).map_err(rejected)?
}

#[cfg(all(target_arch="wasm32",target_env="p2"))]
/// 🎭️ The actor caller retains its original semantic output before this scalar-only identity return.
pub fn with_actor_operation_authority(operation:impl FnOnce(&mut Identity<'_>)->Result<(),ValueError>)->Result<(),ValueError>{
    use return_types::pure;
    use semio_framework_value::native_encoding::NativeEncodeProgress;
    let maximum=usize::try_from(pure::operation_begin().map_err(refusal)?).map_err(|_|refusal(2))?;
    let rejected=std::rc::Rc::new(std::cell::Cell::new(0));
    let mut observer=|next:NativeEncodeProgress|{let code=pure::operation_progress(next.completed as u64,next.total as u64,next.owned_bytes as u64);if code!=0{rejected.set(code);}code==0};
    forwarded(maximum,&rejected,&mut observer as &mut IdentityObserver<'_>,|mut identity|{
        let result=operation(&mut identity);
        let receipt=identity.pause_forwarded()?;
        let returned=admitted(pure::operation_finish(receipt.owned_bytes()as u64));
        result?;
        if rejected.get()!=0{return Err(refusal(rejected.get()))}
        returned
    }).and_then(|inner|inner)
}

#[cfg(all(target_arch="wasm32",target_env="p2"))]
#[path="🪶️snapshot/🦀️.rs"]
mod snapshot;
#[cfg(all(target_arch="wasm32",target_env="p2"))]
pub use snapshot::{with_snapshot_authority,snapshot_retirement,snapshot_original_progress,snapshot_close,snapshot_take,SnapshotReturn};
