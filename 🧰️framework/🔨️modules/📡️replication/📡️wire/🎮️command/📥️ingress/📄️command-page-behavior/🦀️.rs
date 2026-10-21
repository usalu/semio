//! 📄️ Command page behavior: the page block reservation, the fixed command page retirement and the fixed command page implementation with its value and serde codecs. See [`super`].

use super::*;

//#region 🔖️PagedCommandIngress
/// 🧱️ One command page's own 4 KiB block, reserved fallibly so an exhausted guest heap answers with
/// a `Fault` the host can display rather than `handle_alloc_error` → `unreachable`.
fn try_reserve_command_page_block() -> Result<Box<[u8; COMMAND_PAGE_MAXIMUM_BYTES]>, semio_framework_diagnostic::Fault> {
    let mut block = Vec::new();
    block
        .try_reserve_exact(COMMAND_PAGE_MAXIMUM_BYTES)
        .map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-allocation"), "a command page could not reserve its exact 4096-byte block"))?;
    block.resize(COMMAND_PAGE_MAXIMUM_BYTES, 0);
    block
        .into_boxed_slice()
        .try_into()
        .map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-allocation"), "a command page block is not its exact 4096-byte extent"))
}

impl semio_framework_value::retirement::RetireOwned for FixedCommandPage{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(FixedCommandPageRetirement{bytes:std::mem::ManuallyDrop::new(Some(self.bytes))})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<FixedCommandPageRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

impl semio_framework_value::retirement::RetirementCursor for FixedCommandPageRetirement{
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep as Step;if self.bytes.is_none(){return Step::Complete}if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<COMMAND_PAGE_MAXIMUM_BYTES{return Step::BudgetExhausted}drop(self.bytes.take());Step::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:COMMAND_PAGE_MAXIMUM_BYTES,..Default::default()})}
 fn terminal_is_empty(&self)->bool{self.bytes.is_none()}
 fn next_demand(&self,_:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{release_bytes:if self.bytes.is_some(){COMMAND_PAGE_MAXIMUM_BYTES}else{0},depth:usize::from(self.bytes.is_some()),..Default::default()})}
 fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(if self.bytes.is_some(){COMMAND_PAGE_MAXIMUM_BYTES}else{0})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl Drop for FixedCommandPageRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.bytes.is_none(),"original command page dropped before funded release");if self.bytes.is_none(){unsafe{std::mem::ManuallyDrop::drop(&mut self.bytes)}}}}

impl FixedCommandPage {
    pub fn try_from_array(bytes: [u8; COMMAND_PAGE_MAXIMUM_BYTES], len: u32) -> Result<Self, semio_framework_diagnostic::Fault> {
        let len = usize::try_from(len).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-length"), "command page length is not representable"))?;
        if len > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        if bytes[len..].iter().any(|byte| *byte != 0) {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-padding"), "command page carries nonzero bytes outside its declared authority"));
        }
        let mut block = try_reserve_command_page_block()?;
        block.copy_from_slice(&bytes);
        Ok(Self { bytes: block, len: len as u16 })
    }

    pub fn try_copy_from(bytes: &[u8]) -> Result<Self, semio_framework_diagnostic::Fault> {
        if bytes.len() > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        let mut block = try_reserve_command_page_block()?;
        block[..bytes.len()].copy_from_slice(bytes);
        Ok(Self { bytes: block, len: bytes.len() as u16 })
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 🌉️ Hand-written, not derived: `bytes` is a fixed `[u8; COMMAND_PAGE_MAXIMUM_BYTES]` (4096 slots,
/// most unused past `len`) — deriving would walk all 4096 via the blanket `[T; N]` impl instead of
/// just the live `len` prefix the old hand-rolled `serde::Serialize` tuple encoding took care to
/// emit only. Wire shape: a plain `DslValue::Array` of the `len` live bytes (no separate length
/// field — the array's own length IS the count, simpler than the old length-prefixed tuple).
impl semio_framework_value::ToValue for FixedCommandPage {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Array(self.as_slice().iter().map(semio_framework_value::ToValue::to_value).collect())
    }
}

impl semio_framework_value::FromValue for FixedCommandPage {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Array(items) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected an array for FixedCommandPage, found {value:?}")));
        };
        if items.len() > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"fixed command page exceeds 4096 bytes".to_string()));
        }
        let mut bytes = Vec::with_capacity(items.len());
        for (index, item) in items.into_iter().enumerate() {
            bytes.push(<u8 as semio_framework_value::FromValue>::from_value(item).map_err(|error| error.under(index))?);
        }
        FixedCommandPage::try_copy_from(&bytes).map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))
    }
}

/// 🪢️ `serde` kept alongside the hand-written `ToValue`/`FromValue` above for the same
/// wire-sharing reason as `CommandPageCursor`/`CommandIngressStatus`: mirrors the `DslValue::Array`
/// of just the `len` live bytes, no length prefix, no derive over the fixed 4096-slot backing array.
impl serde::Serialize for FixedCommandPage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.as_slice().iter().copied())
    }
}

impl<'de> Deserialize<'de> for FixedCommandPage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        FixedCommandPage::try_copy_from(&bytes).map_err(|fault| serde::de::Error::custom(fault.message))
    }
}
//#endregion 🔖️PagedCommandIngress
