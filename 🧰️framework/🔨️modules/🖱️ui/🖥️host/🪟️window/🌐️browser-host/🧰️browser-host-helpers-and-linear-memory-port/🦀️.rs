//! 🧰️ Browser host helpers and linear memory port: the request, payload and listener helpers, the budget and progress helpers and the linear memory browser host port. See [`super`].

use super::*;

pub(super) fn request(operation: u16, request_id: AbiRequestId, generation: u32, bytes: Vec<u8>) -> Result<AbiRequest, BrowserHostError> {
    Ok(AbiRequest { operation: AbiOperation::try_new(operation)?, request_id, generation, bytes: AbiBytes::try_new(bytes).map_err(|rejected| rejected.code)? })
}

pub(super) fn canvas_payload(canvas: CanvasId) -> Vec<u8> {
    let mut bytes = vec![1];
    bytes.extend_from_slice(&canvas.get().to_le_bytes());
    bytes
}

pub(super) fn parse_listener(bytes: &[u8]) -> Result<ListenerId, BrowserHostError> {
    if bytes.len() != 9 || bytes[0] != 1 {
        return Err(BrowserHostError::MalformedHostReply);
    }
    ListenerId::try_new(u32::from_le_bytes(bytes[1..5].try_into().expect("fixed")), u32::from_le_bytes(bytes[5..9].try_into().expect("fixed"))).map_err(BrowserHostError::Abi)
}

pub(super) fn validate_listener_handle(listener: ListenerId, actual: semio_framework_abi::AbiHandle) -> Result<(), BrowserHostError> {
    let expected = listener.handle();
    if actual.slot() != expected.slot() {
        Err(BrowserHostError::Abi(AbiErrorCode::UnknownHandle))
    } else if actual.generation() < expected.generation() {
        Err(BrowserHostError::Abi(AbiErrorCode::AbaHandle))
    } else if actual.generation() > expected.generation() {
        Err(BrowserHostError::Abi(AbiErrorCode::StaleGeneration))
    } else {
        Ok(())
    }
}

pub(super) fn unavailable_from_reply(reply: &AbiReply) -> BrowserHostUnavailable {
    match reply.bytes.as_slice().first().copied() {
        Some(1) => BrowserHostUnavailable::Window,
        Some(2) => BrowserHostUnavailable::Document,
        Some(3) => BrowserHostUnavailable::Canvas,
        Some(4) => BrowserHostUnavailable::Clipboard,
        Some(5) => BrowserHostUnavailable::Listener,
        _ => BrowserHostUnavailable::Callback,
    }
}

pub(super) fn cursor_tag(cursor: CursorRequest) -> u8 {
    match cursor {
        CursorRequest::Default => 0,
        CursorRequest::Pointer => 1,
        CursorRequest::Text => 2,
        CursorRequest::Grab => 3,
        CursorRequest::Grabbing => 4,
    }
}

pub(super) fn validate_budget(budget: AbiWorkBudget) -> Result<(), BrowserHostError> {
    if budget.cancelled {
        Err(BrowserHostError::Abi(AbiErrorCode::Cancelled))
    } else if budget.interrupted {
        Err(BrowserHostError::Abi(AbiErrorCode::Interrupted))
    } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
        Err(BrowserHostError::Abi(AbiErrorCode::DeadlineExceeded))
    } else if budget.byte_credit == 0 {
        Err(BrowserHostError::Abi(AbiErrorCode::NoCredit))
    } else {
        Ok(())
    }
}

pub(super) fn one_credit(budget: AbiWorkBudget) -> AbiWorkBudget {
    AbiWorkBudget { byte_credit: 1, ..budget }
}

pub(super) fn progress(completed_units: u32, total_units: u32) -> BrowserHostStep {
    BrowserHostStep::Progress(BrowserHostProgress { completed_units, total_units })
}

pub(super) fn message_body_bytes(message: &AbiMessage) -> usize {
    match message {
        AbiMessage::Request(value) => value.bytes.len(),
        AbiMessage::Reply(value) => value.bytes.len(),
        AbiMessage::Event(value) => value.bytes.len(),
        AbiMessage::Page(value) => value.bytes.len(),
        AbiMessage::Control(_) => 0,
    }
}

/// 🧠️ Low-level Wasm import adapter with an owned public surface.
#[derive(Default)]
pub struct LinearMemoryBrowserHostPort;

#[cfg(all(target_arch = "wasm32", not(feature = "testkit")))]
#[link(wasm_import_module = "semio_browser_host")]
unsafe extern "C" {
    #[link_name = "send"]
    fn browser_host_send(message_pointer: *const u8, message_length: usize) -> i32;
    #[link_name = "poll"]
    fn browser_host_poll(message_pointer: *mut u8, message_capacity: usize) -> i32;
}

#[cfg(feature = "testkit")]
unsafe fn browser_host_send(message_pointer: *const u8, message_length: usize) -> i32 {
    linear_memory_test_import::send(message_pointer, message_length)
}

#[cfg(feature = "testkit")]
unsafe fn browser_host_poll(message_pointer: *mut u8, message_capacity: usize) -> i32 {
    linear_memory_test_import::poll(message_pointer, message_capacity)
}

#[cfg(feature = "testkit")]
include!("../../../🧪️tests/🔬️window-browser-linear-memory-test-import/🦀️.rs");

impl AbiPort for LinearMemoryBrowserHostPort {
    fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), semio_framework_abi::AbiPortRejection> {
        if let Err(error) = validate_budget(budget) {
            let BrowserHostError::Abi(code) = error else { unreachable!() };
            return Err(semio_framework_abi::AbiPortRejection { code, message });
        }
        #[cfg(any(target_arch = "wasm32", feature = "testkit"))]
        {
            let bytes = semio_framework_abi::encode_abi_message(&message);
            if bytes.len() > BROWSER_HOST_MAX_ENCODED_EVENT_BYTES {
                return Err(semio_framework_abi::AbiPortRejection { code: AbiErrorCode::LimitExceeded, message });
            }
            if unsafe { browser_host_send(bytes.as_ptr(), bytes.len()) } == 1 {
                Ok(())
            } else {
                Err(semio_framework_abi::AbiPortRejection { code: AbiErrorCode::Interrupted, message })
            }
        }
        #[cfg(all(not(target_arch = "wasm32"), not(feature = "testkit")))]
        Err(semio_framework_abi::AbiPortRejection { code: AbiErrorCode::Closed, message })
    }

    fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
        validate_budget(budget).map_err(|error| match error {
            BrowserHostError::Abi(code) => code,
            _ => AbiErrorCode::Interrupted,
        })?;
        #[cfg(any(target_arch = "wasm32", feature = "testkit"))]
        {
            let mut bytes = vec![0; BROWSER_HOST_INITIAL_POLL_BYTES];
            let mut length = unsafe { browser_host_poll(bytes.as_mut_ptr(), bytes.len()) };
            if length == 0 {
                return Ok(AbiPortPoll::Pending);
            }
            if length < 0 {
                return Ok(AbiPortPoll::Closed);
            }
            let required = length as usize;
            if required > BROWSER_HOST_MAX_ENCODED_EVENT_BYTES {
                return Err(AbiErrorCode::LimitExceeded);
            }
            if required > bytes.len() {
                bytes.resize(required, 0);
                length = unsafe { browser_host_poll(bytes.as_mut_ptr(), bytes.len()) };
                if length == 0 {
                    return Ok(AbiPortPoll::Pending);
                }
                if length < 0 {
                    return Ok(AbiPortPoll::Closed);
                }
                if length as usize > BROWSER_HOST_MAX_ENCODED_EVENT_BYTES {
                    return Err(AbiErrorCode::LimitExceeded);
                }
                if length as usize != required {
                    return Err(AbiErrorCode::MalformedLength);
                }
            }
            bytes.truncate(length as usize);
            let message = semio_framework_abi::decode_abi_message(&bytes)?;
            match &message {
                AbiMessage::Event(event) if event.bytes.len() > BROWSER_HOST_MAX_EVENT_BODY_BYTES => return Err(AbiErrorCode::LimitExceeded),
                AbiMessage::Page(page) if page.bytes.len() > BROWSER_HOST_MAX_PAGE_BODY_BYTES => return Err(AbiErrorCode::LimitExceeded),
                _ => {}
            }
            Ok(AbiPortPoll::Message(message))
        }
        #[cfg(all(not(target_arch = "wasm32"), not(feature = "testkit")))]
        Ok(AbiPortPoll::Closed)
    }
}
