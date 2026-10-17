//! 🚪️ Direct service of the codec ABI: the public codec service over the registered resolver and the request metadata decoder. See [`super`].

use super::*;

//#region ⏳️RetainedOperation
/// 🌉️ Owned retained OS-host codec service; UI callers can submit only A1 requests and pages.
pub struct OsHostCodecService {
    retained: RetainedOsHostCodecService<RegisteredOsHostFormatResolver>,
}

impl Default for OsHostCodecService {
    fn default() -> Self {
        Self::new()
    }
}

impl OsHostCodecService {
    pub fn new() -> Self {
        Self { retained: RetainedOsHostCodecService::new(RegisteredOsHostFormatResolver) }
    }

    pub fn begin(&mut self, request: AbiRequest) -> Result<AbiHandle, (AbiErrorCode, AbiRequest)> {
        self.retained.begin(request)
    }

    pub fn offer(&mut self, handle: AbiHandle, page: AbiPage) -> Result<(), AbiRejectedPage> {
        self.retained.offer(handle, page)
    }

    pub fn seal(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        self.retained.seal(handle)
    }

    pub fn step(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
        self.retained.step(handle, budget)
    }

    pub fn control(&mut self, control: AbiControl) -> Result<Option<OsHostCodecCancelOutcome>, AbiErrorCode> {
        self.retained.control(control)
    }

    pub fn close_step(&mut self, control: AbiControl, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
        self.retained.close_step(control, budget)
    }

    pub fn lose(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        self.retained.lose(handle)
    }
}

pub(super) fn decode_request_metadata(bytes: &[u8]) -> Result<usize, AbiErrorCode> {
    if bytes.len() != 5 || bytes[0] != 1 {
        return Err(AbiErrorCode::MalformedTag);
    }
    let declared = u32::from_le_bytes(bytes[1..5].try_into().expect("fixed metadata width")) as usize;
    if declared > OS_HOST_CODEC_MAX_INPUT_BYTES {
        Err(AbiErrorCode::LimitExceeded)
    } else {
        Ok(declared)
    }
}
//#endregion ⏳️RetainedOperation
