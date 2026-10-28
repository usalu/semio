//! 📤 Original-owned STEP publication writes directly into the native UTF-8 artifact envelope.
use super::{emit::IfcStepEmitCursor, snapshot, IfcSnapshot, IfcValue, IFC4X3_STEP_SCHEMA, MAX_RECORD_BYTES, MAX_SOURCE_BYTES};
use semio_framework_os_kernel::{NativeSnapshotBodyWallet, NativeSnapshotEncodeOwner};
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant, RetainedCloneProgress, RetireOwned, ValueError, ValueRefusalKind};
use std::mem::size_of;

const TOKEN: &[u8] = b"stdio.txt.pack v1";
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
fn limit(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::OwnershipLimit, message) }

struct Port<'a, 'b, 'c> { native: &'a mut NativeEncodeControl<'b>, body: &'c mut NativeSnapshotBodyWallet }
impl Port<'_, '_, '_> {
    fn effect<T>(&mut self, bytes: usize, operation: impl FnOnce() -> T) -> Result<T, ValueError> {
        self.native.checkpoint()?;
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_depth: 2, ..Default::default() })?;
        let value = operation();
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        self.native.step()?;
        Ok(value)
    }
    fn append(&mut self, output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), ValueError> {
        if bytes.len() > output.capacity().saturating_sub(output.len()) { return Err(invalid("STEP output exceeded its original admitted backing")); }
        self.effect(bytes.len(), || output.extend_from_slice(bytes))?;
        self.native.advance(bytes.len())
    }
}

#[derive(Default, RetireOwned)]
struct Meter { bytes: usize, record: usize, quote: u8, pending_quote: bool }
impl Meter {
    fn inspect(&mut self, bytes: &[u8]) -> Result<(), ValueError> {
        self.bytes = self.bytes.checked_add(bytes.len()).filter(|bytes| *bytes <= MAX_SOURCE_BYTES).ok_or_else(|| limit("STEP output exceeds original source byte limit"))?;
        for &byte in bytes {
            self.record = self.record.checked_add(1).filter(|bytes| *bytes <= MAX_RECORD_BYTES).ok_or_else(|| limit("STEP output exceeds original record byte limit"))?;
            if self.quote == b'\'' && self.pending_quote { self.pending_quote = false; if byte == b'\'' { continue; } self.quote = 0; }
            if self.quote == b'\'' { if byte == b'\'' { self.pending_quote = true; } continue; }
            if self.quote == b'"' { if byte == b'"' { self.quote = 0; } continue; }
            match byte { b'\'' | b'"' => self.quote = byte, b';' => self.record = 0, _ => {} }
        }
        Ok(())
    }
}

#[derive(RetireOwned)]
struct Receiving { cursor: IfcStepEmitCursor, ids: Vec<u64>, output: Vec<u8>, meter: Meter }
impl Receiving { fn new() -> Self { Self { cursor: IfcStepEmitCursor::new(), ids: Vec::new(), output: Vec::new(), meter: Meter::default() } } }

fn text_list(values: &[IfcValue], port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if values.is_empty() { return Err(invalid("STEP header requires a nonempty string list")); }
    for value in values { if !port.effect(size_of::<&IfcValue>(), || matches!(value, IfcValue::String(_)))? { return Err(invalid("STEP header list requires only strings")); } }
    Ok(())
}
fn header(source: &IfcSnapshot, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if !port.effect(3 * size_of::<&[IfcValue]>(), || source.schema == snapshot::STDIO_IFC_DOCUMENT_SCHEMA)? { return Err(invalid("STEP source graph must identify stdio.ifc")); }
    match source.header.file_description.as_slice() { [IfcValue::Aggregate(values), IfcValue::String(_)] => text_list(values, port)?, _ => return Err(invalid("STEP FILE_DESCRIPTION requires its two declared attributes")) }
    match source.header.file_name.as_slice() { [IfcValue::String(_), IfcValue::String(_), IfcValue::Aggregate(authors), IfcValue::Aggregate(organizations), IfcValue::String(_), IfcValue::String(_), IfcValue::String(_)] => { text_list(authors, port)?; text_list(organizations, port)?; }, _ => return Err(invalid("STEP FILE_NAME requires its seven declared attributes")) }
    if !port.effect(size_of::<&[IfcValue]>(), || matches!(source.header.file_schema.as_slice(), [IfcValue::Aggregate(values)] if matches!(values.as_slice(), [IfcValue::String(value)] if value == IFC4X3_STEP_SCHEMA)))? { return Err(invalid("STEP FILE_SCHEMA must contain exactly IFC4X3_ADD2")); }
    Ok(())
}

fn sort(ids: &mut [u64], port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    fn sift(ids: &mut [u64], mut root: usize, end: usize, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        loop {
            let left = root.saturating_mul(2).saturating_add(1); if left >= end { return Ok(()); }
            let mut child = left;
            if left + 1 < end && port.effect(2 * size_of::<u64>(), || ids[left] < ids[left + 1])? { child += 1; }
            if !port.effect(2 * size_of::<u64>(), || ids[root] < ids[child])? { return Ok(()); }
            port.effect(2 * size_of::<u64>(), || ids.swap(root, child))?; root = child;
        }
    }
    for root in (0..ids.len() / 2).rev() { sift(ids, root, ids.len(), port)?; }
    for end in (1..ids.len()).rev() { port.effect(2 * size_of::<u64>(), || ids.swap(0, end))?; sift(ids, 0, end, port)?; }
    Ok(())
}
fn reference(ids: &[u64], id: u64, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let (mut low, mut high) = (0, ids.len());
    while low < high { let middle = low + (high - low) / 2; match port.effect(size_of::<u64>(), || ids[middle].cmp(&id))? { std::cmp::Ordering::Less => low = middle + 1, std::cmp::Ordering::Greater => high = middle, std::cmp::Ordering::Equal => return Ok(()) } }
    Err(invalid("STEP source reference has no entity target"))
}

/// 🫴️ The caller's original encoder retains all partial source indexes and output until final publication.
pub(super) fn encode(source: &IfcSnapshot, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
    owner.receive::<Receiving, Vec<u8>>(|slot, native, body| native.scoped_stage(|native| {
        native.begin_stage(0)?;
        body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<Receiving>(), maximum_depth: 2, ..Default::default() })?;
        *slot = Some(Receiving::new());
        body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Receiving>(), ..Default::default() })?;
        let state = slot.as_mut().unwrap();
        let mut port = Port { native, body };
        header(source, &mut port)?;
        if source.entities.len() > MAX_SOURCE_BYTES / 8 { return Err(limit("STEP source identity count cannot fit its physical source bound")); }
        port.body.allocate_encode_vec_into(port.native, source.entities.len(), &mut state.ids, 2)?;
        for entity in &source.entities { if entity.id == 0 { return Err(invalid("STEP instance identifiers must be positive")); } port.effect(size_of::<u64>(), || state.ids.push(entity.id))?; }
        sort(&mut state.ids, &mut port)?;
        for pair in state.ids.windows(2) { if port.effect(2 * size_of::<u64>(), || pair[0] == pair[1])? { return Err(invalid("STEP instance identifiers must be unique")); } }
        loop {
            let fragment = port.effect(size_of::<super::emit::Fragment>(), || state.cursor.next(source))??;
            let Some(fragment) = fragment else { break; };
            if let Some(id) = fragment.reference() { reference(&state.ids, id, &mut port)?; }
            port.effect(size_of::<Meter>(), || state.meter.inspect(fragment.as_bytes()))??;
        }
        let physical = state.meter.bytes.checked_add(12 + TOKEN.len()).ok_or_else(|| limit("STEP UTF-8 artifact extent overflow"))?;
        port.body.allocate_encode_vec_into(port.native, physical, &mut state.output, 2)?;
        port.effect(size_of::<IfcStepEmitCursor>(), || state.cursor = IfcStepEmitCursor::new())?;
        port.append(&mut state.output, &semio_framework_os_kernel::semio_format::BINARY_MAGIC)?;
        port.append(&mut state.output, &(TOKEN.len() as u32).to_le_bytes())?;
        port.append(&mut state.output, TOKEN)?;
        loop { let fragment = port.effect(size_of::<super::emit::Fragment>(), || state.cursor.next(source))??; let Some(fragment) = fragment else { break; }; port.append(&mut state.output, fragment.as_bytes())?; }
        if state.output.len() != physical { return Err(invalid("STEP UTF-8 artifact output differs from its admitted census")); }
        port.native.checkpoint()?;
        port.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<Vec<u8>>(), maximum_depth: 2, ..Default::default() })?;
        port.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Vec<u8>>(), ..Default::default() })?;
        Ok(std::mem::take(&mut state.output))
    }))
}
