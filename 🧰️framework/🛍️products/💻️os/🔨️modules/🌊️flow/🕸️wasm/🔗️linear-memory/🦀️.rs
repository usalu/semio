//! 🔗️ The primitive linear-memory exports of the Flow bridge.

use super::*;

//#region 🌉️LinearMemory

fn flow_bridge_clock_ready() -> bool {
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    { semio_framework_job::install_browser_monotonic_clock().is_ok() }
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    { semio_framework_job::default_now_us().is_some() }
}

struct RetainedMessage {
    _message: AbiMessage,
    pub(super) bytes: Vec<u8>,
}

thread_local! {
    pub(super) static BRIDGE: RefCell<FlowBridge<FlowDomainAdapter>> = RefCell::new(FlowBridge::new(FlowDomainAdapter::default));
    pub(super) static RETAINED: RefCell<Option<RetainedMessage>> = const { RefCell::new(None) };
}

/// 🌉️ The native host receives one original feature body before any linear-memory envelope copy.
pub fn flow_bridge_poll_on_original_caller(cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(semio_framework::abi::AbiHandle,FlowFeatureStep)>,FlowFailure>{BRIDGE.with(|bridge|match bridge.try_borrow_mut(){Ok(mut bridge)=>bridge.poll_on_original_caller(cx),Err(_)=>Ok(None)})}

#[unsafe(no_mangle)]
pub extern "C" fn flow_bridge_allocate(length: usize) -> *mut u8 {
    if length == 0 || length > protocol::FLOW_MAX_REQUEST_BYTES + 32 || !flow_bridge_clock_ready() {
        return std::ptr::null_mut();
    }
    let mut bytes = Vec::<u8>::with_capacity(length);
    let pointer = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    pointer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn flow_bridge_release(pointer: *mut u8, capacity: usize) {
    if !pointer.is_null() && capacity != 0 && capacity <= protocol::FLOW_MAX_REQUEST_BYTES + 32 {
        drop(unsafe { Vec::from_raw_parts(pointer, 0, capacity) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn flow_bridge_send(pointer: *const u8, length: usize, byte_credit: usize, maximum_items: usize, maximum_copy_bytes: usize, maximum_capacity_bytes: usize, maximum_release_bytes: usize, maximum_depth: usize, now_ms: u64, deadline_ms: u64) -> i32 {
    if pointer.is_null() || length == 0 || length > protocol::FLOW_MAX_REQUEST_BYTES + 32 || !flow_bridge_clock_ready() {
        return -1;
    }
    let Ok(message) = decode_abi_message(unsafe { std::slice::from_raw_parts(pointer, length) }) else {
        return -1;
    };
    let budget = AbiWorkBudget { byte_credit, retained:RetainedCloneGrant {maximum_items,maximum_copy_bytes,maximum_capacity_bytes,maximum_release_bytes,maximum_depth}, now_ms, deadline_ms: (deadline_ms != 0).then_some(deadline_ms), cancelled: false, interrupted: false };
    BRIDGE.with(|bridge| bridge.borrow_mut().try_send(message, budget).map(|_| 1).unwrap_or(-1))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn flow_bridge_poll(pointer: *mut u8, capacity: usize, byte_credit: usize, maximum_items: usize, maximum_copy_bytes: usize, maximum_capacity_bytes: usize, maximum_release_bytes: usize, maximum_depth: usize, now_ms: u64, deadline_ms: u64) -> i32 {
    BRIDGE.with(|bridge|bridge.borrow_mut().clear_step_progress());
    if pointer.is_null() || capacity == 0 || capacity > protocol::FLOW_MAX_REQUEST_BYTES + 32 || !flow_bridge_clock_ready() {
        return -1;
    }
    RETAINED.with(|retained| {
        if retained.borrow().is_none() {
            let budget = AbiWorkBudget { byte_credit, retained:RetainedCloneGrant {maximum_items,maximum_copy_bytes,maximum_capacity_bytes,maximum_release_bytes,maximum_depth}, now_ms, deadline_ms: (deadline_ms != 0).then_some(deadline_ms), cancelled: false, interrupted: false };
            match BRIDGE.with(|bridge| bridge.borrow_mut().poll(budget)) {
                Ok(AbiPortPoll::Message(message)) => *retained.borrow_mut() = Some(RetainedMessage { bytes: encode_abi_message(&message), _message: message }),
                Ok(AbiPortPoll::Pending) => return 0,
                Ok(AbiPortPoll::Closed) | Err(_) => return -1,
            }
        }
        let mut retained = retained.borrow_mut();
        let value = retained.as_ref().expect("retained Flow message");
        if value.bytes.len() > capacity {
            return i32::try_from(value.bytes.len()).unwrap_or(-1);
        }
        unsafe { std::ptr::copy_nonoverlapping(value.bytes.as_ptr(), pointer, value.bytes.len()) };
        let length = value.bytes.len();
        retained.take();
        i32::try_from(length).unwrap_or(-1)
    })
}

/// 🧾️ Reads one exact original Bridge receipt axis from the latest poll, including refusal.
#[unsafe(no_mangle)]
pub extern "C" fn flow_bridge_step_progress(axis:u32)->u64{
    if axis>3{return u64::MAX;}
    BRIDGE.with(|bridge|{let progress=bridge.borrow().step_progress();match axis{0=>progress.copied_items as u64,1=>progress.copied_bytes as u64,2=>progress.retained_capacity_bytes as u64,3=>progress.released_bytes as u64,_=>unreachable!()}})
}

#[unsafe(no_mangle)]
pub extern "C" fn flow_bridge_begin_close() {
    if !flow_bridge_clock_ready() { return; }
    BRIDGE.with(|bridge| bridge.borrow_mut().begin_close());
}

#[unsafe(no_mangle)]
pub extern "C" fn flow_bridge_terminal_is_empty() -> i32 {
    if !flow_bridge_clock_ready() { return 0; }
    i32::from(RETAINED.with(|retained| retained.borrow().is_none()) && BRIDGE.with(|bridge| bridge.borrow().terminal_is_empty()))
}

//#endregion 🌉️LinearMemory
