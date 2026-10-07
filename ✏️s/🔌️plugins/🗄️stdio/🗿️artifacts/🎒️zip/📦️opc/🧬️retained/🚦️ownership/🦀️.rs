//! 💰️ Retained OPC adoption admits each actual page and copied literal before ownership.
use super::*;
use semio_framework_value::{NativeDecodeControl, paged::PAGED_UTF8_CHUNK_BYTES};

fn append<T, const N: usize>(target: &mut PagedList<T, N>, value: T, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    while !target.has_reserved_slot() {
        let bytes = target.next_allocation_bytes().map_err(ValueError::from)?;
        control.charge(bytes)?;
        let progress = target.reserve_one(bytes).map_err(|error| ValueError::from(error.refusal()))?;
        if !progress.progressed || progress.allocated_bytes != bytes {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "retained OPC page request differs from admitted backing"));
        }
    }
    target.push_reserved(value).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "retained OPC page rejected its admitted owner"))
}
fn text(value: &str, control: &mut NativeDecodeControl<'_>) -> Result<RetainedOpcText, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(value.len())?;
        let mut chunks = PagedList::default();
        let mut start = 0usize;
        while start < value.len() {
            let mut end = start.saturating_add(PAGED_UTF8_CHUNK_BYTES).min(value.len());
            while !value.is_char_boundary(end) { end -= 1; }
            let chunk = control.copy_text(&value[start..end])?;
            append(&mut chunks, chunk, control)?;
            control.advance(end - start)?;
            start = end;
        }
        RetainedOpcText::from_retained_chunks(chunks, value.len())
    })
}
fn bytes(value: &[u8], control: &mut NativeDecodeControl<'_>) -> Result<RetainedOpcBytes, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(value.len())?;
        let mut bytes = PagedList::default();
        for &byte in value {
            append(&mut bytes, byte, control)?;
            control.step()?;
        }
        Ok(RetainedOpcBytes::from_retained_bytes(bytes))
    })
}
fn metadata(entries: Vec<(String, String)>, control: &mut NativeDecodeControl<'_>) -> Result<RetainedOpcMetadataEntries, ValueError> {
    let mut output = RetainedOpcMetadataEntries::default();
    for (name, content_type) in entries {
        let entry = RetainedOpcMetadataEntry { name: text(&name, control)?, content_type: text(&content_type, control)? };
        append(&mut output, entry, control)?;
        control.step()?;
    }
    Ok(output)
}
impl RetainedOpcPackage {
    /// 🧬️ Constructs actual paged retained fields under one cumulative ownership ledger.
    pub fn try_from_package_controlled(package: OpcPackage, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| {
            let count = package.relationships.groups().try_fold(0usize, |total, (_,values)| total.checked_add(values.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "retained OPC relationship count overflow")))?;
            let total = package.parts.len().checked_add(package.content_types.defaults.len()).and_then(|n| n.checked_add(package.content_types.overrides.len())).and_then(|n| n.checked_add(package.relationships.owner_count())).and_then(|n| n.checked_add(count)).and_then(|n| n.checked_add(1)).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "retained OPC construction workload overflow"))?;
            control.begin_stage(total)?;
            let mut parts = RetainedOpcParts::default();
            for part in package.parts {
                let part = RetainedOpcPart { path: text(&part.path, control)?, content_type: text(&part.content_type, control)?, bytes: bytes(&part.bytes, control)? };
                append(&mut parts, part, control)?;
                control.step()?;
            }
            let content_types = RetainedOpcContentTypes { defaults: metadata(package.content_types.defaults, control)?, overrides: metadata(package.content_types.overrides, control)? };
            let mut owners = PagedList::default();
            for (owner, values) in package.relationships.into_groups() {
                let owner = PagedUtf8::<{usize::MAX}>::try_from_str_controlled(&owner, control)?;
                let mut relationships = RetainedOpcRelationships::default();
                for value in values {
                    let value = RetainedOpcRelationship { id: text(&value.id, control)?, rel_type: text(&value.rel_type, control)?, target: text(&value.target, control)?, target_mode: value.target_mode.into() };
                    append(&mut relationships, value, control)?;
                    control.step()?;
                }
                append(&mut owners, (owner, relationships), control)?;
                control.step()?;
            }
            let relationships = RetainedOpcRelationshipOwners::from_sorted_retained_entries_controlled(owners, control)?;
            let comment = text(&package.comment, control)?;
            control.step()?;
            Ok(Self { parts, content_types, relationships, comment })
        })
    }
}
