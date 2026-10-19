use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::mem::ManuallyDrop;

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct MountedChildGroupIdentitySource<'a> { pub parent: &'a str, pub actor: &'a str, pub revision: [u8; 32], pub instance: u32, pub operation: u64 }

impl<'a> MountedChildGroupIdentitySource<'a> {
    /// 🫳️ Borrows the mounted parent's original coordinates for the issuer to validate and seal.
    pub fn borrowed(parent: &'a str, actor: &'a str, revision: [u8; 32], instance: u32, operation: u64) -> Self { Self { parent, actor, revision, instance, operation } }
}

#[doc(hidden)]
pub struct MountedChildGroupIdentityIssuer {
    hash: Option<semio_framework_hash::Sha256>,
    pub seal: Option<(usize, usize, usize, usize, [u8; 32], u32, u64)>,
    pub field: usize,
    pub offset: usize,
    pub encoded: [u8; 70],
    pub output: ManuallyDrop<Option<Vec<u8>>>,
    pub finished: bool,
    pub closing: bool,
}

impl MountedChildGroupIdentityIssuer {
    pub fn new() -> Self { Self { hash: Some(semio_framework_hash::Sha256::new()), seal: None, field: 0, offset: 0, encoded: [0; 70], output: ManuallyDrop::new(None), finished: false, closing: false } }
    pub fn next_capacity_byte_demand(&self) -> usize { if !self.finished && !self.closing && self.field == 8 && self.output.is_none() { 70 } else { 0 } }
    pub fn ready(&self) -> bool { !self.finished && !self.closing && self.output.as_ref().is_some_and(|output| output.len() == 70) }
    /// 🔏️ Hashes one original source prefix and admits the full fixed output before copying its bytes.
    pub fn advance(&mut self, source: MountedChildGroupIdentitySource<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted group identity is closing")); }
        if self.finished || self.ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if source.parent.len() > store::MEMBER_OPEN_IDENTITY_BYTES || source.actor.len() > store::MEMBER_OPEN_IDENTITY_BYTES { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "mounted group identity exceeds original identity admission")); }
        let seal = (source.parent.as_ptr() as usize, source.parent.len(), source.actor.as_ptr() as usize, source.actor.len(), source.revision, source.instance, source.operation);
        if self.seal.is_some_and(|original| original != seal) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "mounted group identity requires its exact borrowed originals")); }
        if self.field < 8 {
            let parent_length = (source.parent.len() as u64).to_le_bytes();
            let actor_length = (source.actor.len() as u64).to_le_bytes();
            let instance = source.instance.to_le_bytes();
            let operation = source.operation.to_le_bytes();
            let fields: [&[u8]; 8] = [b"semio.owned-child-group/v1\0", &parent_length, source.parent.as_bytes(), &actor_length, source.actor.as_bytes(), &source.revision, &instance, &operation];
            let field = fields[self.field];
            let count = (field.len() - self.offset).min(grant.maximum_copy_bytes).min(64);
            if count == 0 && self.offset != field.len() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.seal = Some(seal);
            self.hash.as_mut().unwrap().update(&field[self.offset..self.offset + count]);
            self.offset += count;
            if self.offset == field.len() { self.field += 1; self.offset = 0; }
            if self.field == 8 {
                self.encoded[..6].copy_from_slice(b"group:");
                let digest = self.hash.take().unwrap().finalize();
                for (index, byte) in digest.into_iter().enumerate() { self.encoded[6 + index * 2] = b"0123456789abcdef"[usize::from(byte >> 4)]; self.encoded[7 + index * 2] = b"0123456789abcdef"[usize::from(byte & 15)]; }
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() }));
        }
        if self.output.is_none() {
            if grant.maximum_capacity_bytes < 70 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            *self.output = Some(Vec::with_capacity(70));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: 70, ..Default::default() }));
        }
        let output = self.output.as_mut().unwrap();
        let copied = (70 - output.len()).min(grant.maximum_copy_bytes).min(64);
        if copied == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        output.extend_from_slice(&self.encoded[output.len()..output.len() + copied]);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() }))
    }
    /// 🧳️ Moves one completed ASCII identity without a second allocation or implicit source clone.
    pub fn take_ready(&mut self, grant: RetainedCloneGrant) -> Option<String> { if grant.maximum_items == 0 || !self.ready() { return None; } self.finished = true; Some(unsafe { String::from_utf8_unchecked(self.output.take().unwrap()) }) }
    pub fn next_close_byte_demand(&self) -> usize { self.output.as_ref().map_or(0, Vec::capacity) }
    /// 🪵️ Retires the original full output allocation or its inline unfinished hash state.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        if self.terminal_is_empty() { return RetainedCloneStep::Complete(Default::default()); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < self.next_close_byte_demand() { return RetainedCloneStep::Progress(Default::default()); }
        self.closing = true;
        if self.output.is_some() { let bytes = self.next_close_byte_demand(); drop(self.output.take()); return RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }); }
        self.hash = None; self.finished = true;
        RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() })
    }
    pub fn terminal_is_empty(&self) -> bool { self.output.is_none() && (self.finished || self.closing) }
}

impl Drop for MountedChildGroupIdentityIssuer { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "mounted group identity requires exact source transfer or funded output retirement"); } }

#[cfg(test)]
include!("🧪️tests/🦀️.rs");
