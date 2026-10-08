//! 🌱️ Retained canonical genesis framing over borrowed initial bytes and exact member identities.

use crate::os_store::OwnerRef;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;

#[derive(Clone, Copy)]
pub struct MemberGenesisEnvelopeSource<'a> {
    pub schema: &'a str,
    pub expected: &'a ArtifactRef,
    pub owner: &'a OwnerRef,
    pub initial_pack: &'a [u8],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemberGenesisEnvelopeProgress {
    pub processed_bytes: usize,
    pub written_bytes: usize,
    pub retained_capacity_bytes: usize,
    pub complete: bool,
}

#[derive(Clone, Copy, Default)]
struct IdPlan { dictionary: usize, prefix_len: usize, uuid: Option<[u8; 16]>, ready: bool }

pub struct MemberGenesisEnvelopeEncoder {
    uri: ManuallyDrop<Option<Vec<u8>>>,
    source_seal: Option<[(usize, usize); 13]>,
    ids: [IdPlan; 8],
    dictionary: [usize; 8],
    dictionary_len: usize,
    initial_dictionary_len: usize,
    field: usize,
    comparison: usize,
    comparison_byte: usize,
    output_position: usize,
    record: usize,
    frame_position: usize,
    records_len: u64,
    record_count: u32,
    crc: protocol::codec::Crc32cCursor,
    frame_hash: semio_framework_hash::Hasher,
    chain_hash: semio_framework_hash::Hasher,
    closed: bool,
}

fn fault(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }
fn width(mut value: usize) -> usize { let mut count = 1; while value >= 128 { count += 1; value >>= 7; } count }
fn varint(value: usize, index: usize) -> u8 { let rest = value >> (index * 7); (rest as u8 & 127) | if rest >= 128 { 128 } else { 0 } }

impl MemberGenesisEnvelopeSource<'_> {
    fn parent_parts(&self) -> [&[u8]; 7] { [self.owner.parent.artifact_id.as_bytes(), b"!", self.owner.parent.dialect.artifact_kind.as_bytes(), b"@", self.owner.parent.dialect.standard.as_bytes(), b"/", self.owner.parent.dialect.subset.as_bytes()] }
    fn uri_len(&self) -> Option<usize> { self.parent_parts().iter().try_fold(0usize, |length, part| length.checked_add(part.len())) }
    fn seal(&self) -> [(usize, usize); 13] {
        let fields = [self.schema.as_bytes(), self.expected.artifact_id.as_bytes(), self.expected.dialect.artifact_kind.as_bytes(), self.expected.dialect.standard.as_bytes(), self.expected.dialect.subset.as_bytes(), self.owner.slot.as_bytes(), self.owner.child_id.as_bytes(), self.owner.parent.artifact_id.as_bytes(), self.owner.parent.dialect.artifact_kind.as_bytes(), self.owner.parent.dialect.standard.as_bytes(), self.owner.parent.dialect.subset.as_bytes(), self.initial_pack, &[]];
        fields.map(|field| (field.as_ptr() as usize, field.len()))
    }
}

impl Default for MemberGenesisEnvelopeEncoder { fn default() -> Self { Self::new() } }

impl MemberGenesisEnvelopeEncoder {
    pub fn new() -> Self {
        Self { uri: ManuallyDrop::new(None), source_seal: None, ids: [IdPlan::default(); 8], dictionary: [0; 8], dictionary_len: 0, initial_dictionary_len: 0, field: 0, comparison: 0, comparison_byte: 0, output_position: 0, record: 0, frame_position: 0, records_len: 0, record_count: 0, crc: protocol::codec::Crc32cCursor::new(), frame_hash: semio_framework_hash::Hasher::new(), chain_hash: semio_framework_hash::Hasher::new(), closed: false }
    }

    /// 📐️ Names the one URI allocation before the original source may be copied.
    pub fn next_capacity_byte_demand(&self, source: MemberGenesisEnvelopeSource<'_>) -> Option<usize> { if self.closed || self.uri.is_some() { Some(0) } else { source.uri_len() } }

    /// 🧬️ Borrows the complete URI assembled only from valid UTF-8 source strings and ASCII delimiters.
    fn fields<'a>(&'a self, source: MemberGenesisEnvelopeSource<'a>) -> Result<[&'a str; 8], ValueError> {
        let bytes = self.uri.as_ref().ok_or_else(|| fault("genesis URI capacity remains unadmitted"))?;
        if Some(bytes.len()) != source.uri_len() { return Err(fault("genesis URI bytes are not complete")); }
        let parent = unsafe { std::str::from_utf8_unchecked(bytes) };
        Ok([&source.expected.artifact_id, source.schema, parent, &source.owner.slot, &source.owner.child_id, &source.expected.dialect.artifact_kind, &source.expected.dialect.standard, &source.expected.dialect.subset])
    }

    /// ⛽️ Names indivisible fixed identifier parsing work separately from streaming bytes.
    pub fn next_copy_byte_demand(&self, source: MemberGenesisEnvelopeSource<'_>) -> Result<usize, ValueError> {
        if self.closed { return Ok(0); }
        if self.uri.as_ref().is_none_or(|uri| Some(uri.len()) != source.uri_len()) { return Ok(1); }
        if self.field < 8 && !self.ids[self.field].ready { return Ok(self.fields(source)?[self.field].len().min(38).max(1)); }
        Ok(usize::from(self.record < 5))
    }

    fn dictionary_payload_len(&self, source: MemberGenesisEnvelopeSource<'_>, start: usize, end: usize) -> Result<usize, ValueError> {
        let mut length = 1 + width(start) + width(end - start);
        for index in start..end { let size = self.ids[self.dictionary[index]].prefix_len; length += width(size) + size; }
        let _ = source;
        Ok(length)
    }

    fn payload_len(&self, source: MemberGenesisEnvelopeSource<'_>, record: usize) -> Result<usize, ValueError> {
        Ok(match record {
            0 => self.dictionary_payload_len(source, 0, self.initial_dictionary_len)?,
            1 => 1 + (0..2).map(|field| 1 + width(self.ids[field].dictionary) + usize::from(self.ids[field].uuid.is_some()) * 16).sum::<usize>(),
            2 => self.dictionary_payload_len(source, self.initial_dictionary_len, self.dictionary_len)?,
            3 => 2 + (2..8).map(|field| 1 + width(self.ids[field].dictionary) + usize::from(self.ids[field].uuid.is_some()) * 16).sum::<usize>(),
            4 => crate::os_spr::format::COMMIT_PAYLOAD_LEN,
            _ => return Err(fault("genesis record cursor exceeded its canonical plan")),
        })
    }

    fn frame_len(&self, source: MemberGenesisEnvelopeSource<'_>, record: usize) -> Result<usize, ValueError> { let body = self.payload_len(source, record)? + 2; Ok(width(body) + body + 8) }

    /// 📏️ Exposes exact output capacity after the borrowed dictionary comparisons finish.
    pub fn encoded_length(&self, source: MemberGenesisEnvelopeSource<'_>) -> Result<Option<usize>, ValueError> {
        if self.field != 8 { return Ok(None); }
        let mut length = width(source.initial_pack.len()).checked_add(source.initial_pack.len()).and_then(|length| length.checked_add(crate::os_spr::format::HEADER_SIZE)).ok_or_else(|| fault("genesis output length overflow"))?;
        for record in 0..5 { if record != 2 || self.initial_dictionary_len != self.dictionary_len { length = length.checked_add(self.frame_len(source, record)?).ok_or_else(|| fault("genesis output length overflow"))?; } }
        Ok(Some(length))
    }

    fn payload_byte(&self, source: MemberGenesisEnvelopeSource<'_>, record: usize, mut position: usize) -> Result<u8, ValueError> {
        if record == 4 {
            return Ok(crate::os_spr::format::write_commit_payload(1, 0, self.records_len, self.record_count, self.chain_hash.finalize().as_bytes())[position]);
        }
        let fields = self.fields(source)?;
        if record == 0 || record == 2 {
            let (start, end) = if record == 0 { (0, self.initial_dictionary_len) } else { (self.initial_dictionary_len, self.dictionary_len) };
            if position == 0 { return Ok(1); } position -= 1;
            for value in [start, end - start] { let size = width(value); if position < size { return Ok(varint(value, position)); } position -= size; }
            for index in start..end {
                let field = self.dictionary[index];
                let length = self.ids[field].prefix_len;
                if position < width(length) { return Ok(varint(length, position)); } position -= width(length);
                if position < length { return Ok(fields[field].as_bytes()[position]); } position -= length;
            }
        } else {
            if position == 0 { return Ok(1); } position -= 1;
            if record == 3 { if position == 0 { return Ok(3); } position -= 1; }
            for field in if record == 1 { 0..2 } else { 2..8 } {
                let id = self.ids[field];
                if position == 0 { return Ok(if id.uuid.is_some() { 2 } else { 1 }); } position -= 1;
                if position < width(id.dictionary) { return Ok(varint(id.dictionary, position)); } position -= width(id.dictionary);
                if let Some(uuid) = id.uuid { if position < 16 { return Ok(uuid[position]); } position -= 16; }
            }
        }
        Err(fault("genesis payload cursor exceeded its exact borrowed record"))
    }

    fn write_byte(&mut self, source: MemberGenesisEnvelopeSource<'_>) -> Result<u8, ValueError> {
        let pack_prefix = width(source.initial_pack.len());
        let spr_start = pack_prefix + source.initial_pack.len();
        let position = self.output_position;
        let byte = if position < pack_prefix { varint(source.initial_pack.len(), position) }
        else if position < spr_start { source.initial_pack[position - pack_prefix] }
        else if position < spr_start + crate::os_spr::format::HEADER_SIZE {
            let header = crate::os_spr::format::build_header_bytes(crate::os_spr::REQUIRED_HASH_CHAIN, crate::os_spr::OPTIONAL_CANONICAL);
            if position == spr_start { self.chain_hash.update(semio_framework_hash::hash(&header).as_bytes()); }
            header[position - spr_start]
        } else {
            if self.record == 2 && self.dictionary_len == self.initial_dictionary_len { self.record += 1; }
            let payload = self.payload_len(source, self.record)?;
            let body = payload + 2;
            let prefix = width(body);
            let length = prefix + body + 8;
            if self.frame_position == 0 { self.crc = protocol::codec::Crc32cCursor::new(); self.frame_hash = semio_framework_hash::Hasher::new(); }
            let relative = self.frame_position;
            let byte = if relative < prefix { varint(body, relative) }
            else if relative == prefix { [crate::os_spr::REC_STR_DICT, crate::os_spr::REC_DOC, crate::os_spr::REC_STR_DICT, crate::os_spr::REC_COMPOSITION, crate::os_spr::REC_COMMIT][self.record] }
            else if relative == prefix + 1 { if self.record == 3 { 0 } else { crate::os_spr::wire::FRAME_FLAG_CRITICAL } }
            else if relative < prefix + body { self.payload_byte(source, self.record, relative - prefix - 2)? }
            else if relative < prefix + body + 4 { self.crc.finish().to_le_bytes()[relative - prefix - body] }
            else { u32::try_from(length).map_err(|_| fault("genesis frame exceeds canonical back length"))?.to_le_bytes()[relative - prefix - body - 4] };
            self.frame_hash.update(&[byte]);
            if relative >= prefix && relative < prefix + body { self.crc.update_page(&[byte]); }
            self.frame_position += 1;
            if self.frame_position == length {
                if self.record != 4 { self.chain_hash.update(self.frame_hash.finalize().as_bytes()); self.records_len += length as u64; self.record_count += 1; }
                self.frame_position = 0;
                self.record += 1;
            }
            byte
        };
        self.output_position += 1;
        Ok(byte)
    }

    /// 🧵️ Copies or compares only the granted work prefix and streams into already admitted caller storage.
    pub fn encode_step(&mut self, source: MemberGenesisEnvelopeSource<'_>, output: &mut [u8], grant: RetainedCloneGrant) -> Result<MemberGenesisEnvelopeProgress, ValueError> {
        let mut progress = MemberGenesisEnvelopeProgress::default();
        if self.closed { return Err(fault("genesis encoder is already retired")); }
        if grant.maximum_items == 0 { return Ok(progress); }
        if source.initial_pack.is_empty() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "member genesis requires the original nonempty initial pack")); }
        let seal = source.seal();
        if self.source_seal.is_some_and(|prior| prior != seal) { return Err(fault("genesis encoder requires its exact original borrowed source")); }
        let uri_length = source.uri_len().ok_or_else(|| fault("genesis parent URI length overflow"))?;
        if self.uri.is_none() {
            if grant.maximum_capacity_bytes < uri_length { return Ok(progress); }
            *self.uri = Some(Vec::with_capacity(uri_length));
            self.source_seal = Some(seal);
            progress.retained_capacity_bytes = uri_length;
            return Ok(progress);
        }
        let mut remaining = grant.maximum_copy_bytes;
        while self.uri.as_ref().unwrap().len() < uri_length && remaining != 0 {
            let mut position = self.uri.as_ref().unwrap().len();
            for part in source.parent_parts() { if position < part.len() { self.uri.as_mut().unwrap().push(part[position]); break; } position -= part.len(); }
            remaining -= 1;
            progress.processed_bytes += 1;
        }
        if self.uri.as_ref().unwrap().len() != uri_length { return Ok(progress); }
        while self.field < 8 && remaining != 0 {
            let field = self.field;
            if !self.ids[field].ready {
                let fields = self.fields(source)?;
                let text = fields[field];
                let cost = text.len().min(38).max(1);
                if remaining < cost { return Ok(progress); }
                let parsed = crate::os_spr::scalar::split_prefix_uuid(text);
                let (prefix_len, uuid) = parsed.map_or((text.len(), None), |(prefix, uuid)| (prefix.len(), Some(uuid)));
                self.ids[field] = IdPlan { prefix_len, uuid, ready: true, dictionary: 0 };
                remaining -= cost;
                progress.processed_bytes += cost;
                continue;
            }
            if self.comparison == self.dictionary_len {
                self.dictionary[self.dictionary_len] = field;
                self.ids[field].dictionary = self.dictionary_len;
                self.dictionary_len += 1;
                self.field += 1;
                self.comparison = 0;
                self.comparison_byte = 0;
                if self.field == 2 { self.initial_dictionary_len = self.dictionary_len; }
            } else {
                let other = self.dictionary[self.comparison];
                let length = self.ids[field].prefix_len;
                if length != self.ids[other].prefix_len { self.comparison += 1; self.comparison_byte = 0; }
                else if self.comparison_byte == length {
                    self.ids[field].dictionary = self.comparison;
                    self.field += 1;
                    self.comparison = 0;
                    self.comparison_byte = 0;
                    if self.field == 2 { self.initial_dictionary_len = self.dictionary_len; }
                } else {
                    let fields = self.fields(source)?;
                    if fields[field].as_bytes()[self.comparison_byte] == fields[other].as_bytes()[self.comparison_byte] { self.comparison_byte += 1; }
                    else { self.comparison += 1; self.comparison_byte = 0; }
                }
            }
            remaining -= 1;
            progress.processed_bytes += 1;
        }
        if self.field != 8 { return Ok(progress); }
        let length = self.encoded_length(source)?.unwrap();
        while self.output_position < length && progress.written_bytes < output.len() && remaining != 0 {
            output[progress.written_bytes] = self.write_byte(source)?;
            progress.written_bytes += 1;
            progress.processed_bytes += 1;
            remaining -= 1;
        }
        progress.complete = self.output_position == length;
        Ok(progress)
    }

    pub fn next_close_byte_demand(&self) -> usize { self.uri.as_ref().map_or(0, Vec::capacity) }

    /// ♻️ Releases the URI backing in one wholly funded turn after output transfer or cancellation.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let bytes = self.next_close_byte_demand();
        if self.uri.is_none() { self.closed = true; return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        drop(self.uri.take());
        self.closed = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }))
    }

    pub fn terminal_is_empty(&self) -> bool { self.uri.is_none() }
}

impl Drop for MemberGenesisEnvelopeEncoder {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "member genesis encoder retains its URI until exact bounded close"); }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
