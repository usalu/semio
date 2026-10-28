//! 🫴️ Original source-owned STEP receiving retains lexer, graph and reference frontiers without joined text or cold parsing.
use super::{IfcSnapshot, IfcValue, MAX_DEPTH, MAX_RECORD_BYTES, MAX_SOURCE_BYTES, TxtSnapshot};
use semio_framework_os_kernel::{NativeSnapshotBodyWallet, NativeSnapshotDecodeOwner};
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, RetireOwned, ValueError, ValueRefusalKind};
use std::{cell::Cell, mem::size_of};
#[path = "native-step-lex.rs"]
mod lex;
#[path = "native-step-parse.rs"]
mod parse;
pub(super) fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
pub(super) fn limit(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::WorkLimit, message)
}
/// 📍️ Errors retain the authentic native cause and original UTF-8 byte offset without allocating an origin mirror.
#[derive(Debug, RetireOwned)]
pub struct NativeStepReadError {
    pub cause: ValueError,
    pub offset: usize,
}
impl std::fmt::Display for NativeStepReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STEP at original UTF-8 byte {}: {}", self.offset, self.cause)
    }
}
impl std::error::Error for NativeStepReadError {}
pub(super) struct Port<'a, 'b, 'c> {
    native: &'a mut NativeDecodeControl<'b>,
    body: &'c mut NativeSnapshotBodyWallet,
    offset: &'c Cell<usize>,
}
impl Port<'_, '_, '_> {
    pub(super) fn effect<T: Copy>(&mut self, bytes: usize, operation: impl FnOnce() -> T) -> Result<T, ValueError> {
        self.native.checkpoint()?;
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_depth: 2, ..Default::default() })?;
        let value = operation();
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        self.native.step()?;
        Ok(value)
    }
    pub(super) fn vector<T>(&mut self, count: usize, out: &mut Vec<T>) -> Result<(), ValueError> {
        self.body.allocate_vec_into(self.native, count, out, 2)?;
        self.native.step()
    }
    pub(super) fn text(&mut self, count: usize, out: &mut String) -> Result<(), ValueError> {
        if out.capacity() != 0 {
            return Err(invalid("STEP token receiving text was not retired or transferred"));
        }
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<String>(), maximum_capacity_bytes: count, maximum_depth: 2, ..Default::default() })?;
        self.native.charge(count)?;
        out.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "STEP token text allocation failed"))?;
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<String>(), retained_capacity_bytes: out.capacity(), ..Default::default() })?;
        self.native.step()
    }
    pub(super) fn release_text(&mut self, out: &mut String) -> Result<(), ValueError> {
        let capacity = out.capacity();
        self.native.checkpoint()?;
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<String>(), maximum_release_bytes: capacity, maximum_depth: 2, ..Default::default() })?;
        drop(std::mem::take(out));
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<String>(), released_bytes: capacity, ..Default::default() })?;
        self.native.step()
    }
    pub(super) fn at(&self, offset: usize) {
        self.offset.set(offset);
    }
}
/// 📥️ Reads actual TXT line storage under the caller's original decoder and five-axis physical wallet.
pub fn read(source: &TxtSnapshot, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<IfcSnapshot, NativeStepReadError> {
    let offset = Cell::new(0);
    let result = owner.receive::<parse::Receiving, IfcSnapshot>(|slot, native, body| {
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<parse::Receiving>(), maximum_depth: 2, ..Default::default() })?;
            *slot = Some(parse::Receiving::default());
            body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<parse::Receiving>(), ..Default::default() })?;
            let mut port = Port { native, body, offset: &offset };
            let state = slot.as_mut().unwrap();
            port.text(256, &mut state.error_text)?;
            let mut extent = 0usize;
            if !port.effect(size_of::<&str>(), || source.schema == "stdio.txt")? {
                return Err(invalid("STEP import requires actual stdio.txt source storage"));
            }
            for (i, line) in source.lines.iter().enumerate() {
                port.effect(size_of::<usize>(), || ())?;
                extent = extent
                    .checked_add(line.len())
                    .and_then(|n| n.checked_add(if i + 1 < source.lines.len() || source.trailing_newline { source.line_ending.as_str().len() } else { 0 }))
                    .filter(|n| *n <= MAX_SOURCE_BYTES)
                    .ok_or_else(|| limit("STEP source exceeds byte admission"))?;
            }
            if let Err(mut error) = parse::receive(source, state, &mut port) {
                if error.kind == ValueRefusalKind::InvalidValue && matches!(error.message, std::borrow::Cow::Borrowed(_)) {
                    use std::fmt::Write;
                    let at = offset.get();
                    let digits = if at == 0 { 1 } else { at.ilog10() as usize + 1 };
                    let count = "STEP at original UTF-8 byte ".len() + digits + 2 + error.message.len();
                    if count <= state.error_text.capacity() {
                        let demand = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: count, maximum_depth: 2, ..Default::default() };
                        if port.body.admit_frontier(demand).is_ok() {
                            port.native.checkpoint()?;
                            write!(&mut state.error_text, "STEP at original UTF-8 byte {at}: {}", error.message).map_err(|_| invalid("STEP error text exceeds its admitted destination"))?;
                            port.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: state.error_text.len(), ..Default::default() })?;
                            port.native.step()?;
                            error.message = std::borrow::Cow::Owned(std::mem::take(&mut state.error_text));
                        }
                    }
                }
                return Err(error);
            }
            port.effect(size_of::<IfcSnapshot>(), || ())?;
            Ok(state.snapshot.take().unwrap())
        })
    });
    result.map_err(|cause| NativeStepReadError { cause, offset: offset.get() })
}
#[cfg(test)]
#[path = "native-step-read-tests.rs"]
mod tests;
