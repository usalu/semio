//! 🌳️ Safe typed borrowed nodes and Store-private rooted iterator lifetime authority.

use super::*;

//#region 🧬️BorrowedValues
/// 🌿️ Borrowed JSON structure; no encoded bytes, caller digest, raw pointer, or unsafe contract.
pub enum ArtifactCanonicalJsonValue<'a> {
    Scalar(ArtifactCanonicalJsonNode<'a>),
    Source(&'a dyn ArtifactCanonicalJson),
    Array(ArtifactCanonicalJsonArray<'a>),
    Object(ArtifactCanonicalJsonObject<'a>),
}

/// 📚️ A retained typed sequence. Construction, next, and drop must stay bounded and borrow payloads.
pub struct ArtifactCanonicalJsonArray<'a> {
    values: Box<dyn Iterator<Item = ArtifactCanonicalJsonValue<'a>> + Send + 'a>,
}

impl<'a> ArtifactCanonicalJsonArray<'a> {
    pub fn new(values: impl Iterator<Item = ArtifactCanonicalJsonValue<'a>> + Send + 'a) -> Self { Self { values: Box::new(values) } }
}

impl<'a> Iterator for ArtifactCanonicalJsonArray<'a> {
    type Item = ArtifactCanonicalJsonValue<'a>;
    fn next(&mut self) -> Option<Self::Item> { self.values.next() }
}

/// 🗂️ A retained ordered map/field iterator. Native borrowed iterators preserve exact serde order.
pub struct ArtifactCanonicalJsonObject<'a> {
    values: Box<dyn Iterator<Item = (&'a str, ArtifactCanonicalJsonValue<'a>)> + Send + 'a>,
}

impl<'a> ArtifactCanonicalJsonObject<'a> {
    pub fn new(values: impl Iterator<Item = (&'a str, ArtifactCanonicalJsonValue<'a>)> + Send + 'a) -> Self { Self { values: Box::new(values) } }
}

impl<'a> Iterator for ArtifactCanonicalJsonObject<'a> {
    type Item = (&'a str, ArtifactCanonicalJsonValue<'a>);
    fn next(&mut self) -> Option<Self::Item> { self.values.next() }
}
//#endregion 🧬️BorrowedValues

//#region 🔒️RootedEncoding
struct BorrowedString<'a> {
    text: &'a str,
    offset: usize,
    phase: u8,
    escape: [u8; 6],
    escape_length: usize,
    escape_offset: usize,
}

impl<'a> BorrowedString<'a> {
    fn new(text: &'a str) -> Self { Self { text, offset: 0, phase: 0, escape: [0; 6], escape_length: 0, escape_offset: 0 } }

    fn next_byte(&mut self) -> Option<u8> {
        if self.phase == 0 { self.phase = 1; return Some(b'"'); }
        if self.phase == 2 { return None; }
        if self.escape_offset < self.escape_length {
            let byte = self.escape[self.escape_offset];
            self.escape_offset += 1;
            return Some(byte);
        }
        let Some(byte) = self.text.as_bytes().get(self.offset).copied() else { self.phase = 2; return Some(b'"'); };
        self.offset += 1;
        self.escape_length = canonical_escape(byte, &mut self.escape);
        self.escape_offset = 1;
        Some(self.escape[0])
    }
}

enum BorrowedFrame<'a> {
    Pending(ArtifactCanonicalJsonValue<'a>),
    String(BorrowedString<'a>),
    Scalar { bytes: ScalarBytes, offset: usize },
    Array { values: ArtifactCanonicalJsonArray<'a>, started: bool, emitted: bool },
    Object { values: ArtifactCanonicalJsonObject<'a>, phase: u8, emitted: bool, current: Option<ArtifactCanonicalJsonValue<'a>> },
    Indexed { source: &'a dyn ArtifactCanonicalJson, cursor: Box<ArtifactCanonicalJsonCursor> },
}

/// 🔒️ Private lifetime projection anchored exclusively to a frozen Store-owned Box or Arc root.
/// Both sealer and reader declare this field before their root, so unwind retires frames first.
/// No frame or projected reference can leave this module; public checkpoints carry only replay data.
pub(super) struct ArtifactCanonicalEditEncoder {
    frames: Vec<Option<BorrowedFrame<'static>>>,
    depth: usize,
    root_address: usize,
    started: bool,
}

impl Default for ArtifactCanonicalEditEncoder {
    fn default() -> Self { Self { frames: Vec::with_capacity(ARTIFACT_CANONICAL_JSON_DEPTH), depth: 0, root_address: 0, started: false } }
}

impl ArtifactCanonicalEditEncoder {
    /// 🪪️ Extends only references obtained from the exact privately owned immutable root.
    /// Private callers retain its Box or Arc until every frame is empty, including unwind.
    fn bind<T: ArtifactCanonicalJson>(&mut self, root: &T) -> Result<(), String> {
        let address = root as *const T as usize;
        if self.started {
            if self.root_address != address { return Err("canonical-edit.borrowed-root-rebound".into()); }
            return Ok(());
        }
        let value = root.canonical_json_borrowed_root()?.unwrap_or(ArtifactCanonicalJsonValue::Source(root));
        let value: ArtifactCanonicalJsonValue<'static> = unsafe { std::mem::transmute(value) };
        if self.frames.is_empty() { self.frames.push(Some(BorrowedFrame::Pending(value))); } else { self.frames[0] = Some(BorrowedFrame::Pending(value)); }
        self.depth = 1;
        self.root_address = address;
        self.started = true;
        Ok(())
    }

    fn push(&mut self, value: BorrowedFrame<'static>) -> Result<(), String> {
        if self.depth == ARTIFACT_CANONICAL_JSON_DEPTH { return Err("canonical-edit.depth-limit".into()); }
        if self.depth == self.frames.len() { self.frames.push(Some(value)); } else { self.frames[self.depth] = Some(value); }
        self.depth += 1;
        Ok(())
    }

    fn next_byte(&mut self) -> Result<Option<u8>, String> {
        for _ in 0..ARTIFACT_CANONICAL_JSON_DEPTH * 8 {
            if self.depth == 0 { return Ok(None); }
            let top = self.depth - 1;
            let frame = self.frames[top].take().ok_or_else(|| "canonical-edit.borrowed-frame-missing".to_string())?;
            match frame {
                BorrowedFrame::Pending(value) => {
                    self.frames[top] = Some(match value {
                        ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String(text)) => BorrowedFrame::String(BorrowedString::new(text)),
                        ArtifactCanonicalJsonValue::Scalar(node) => BorrowedFrame::Scalar { bytes: ScalarBytes::from_node(node)?, offset: 0 },
                        ArtifactCanonicalJsonValue::Source(source) => match source.canonical_json_borrowed_root()? {
                            Some(value) => BorrowedFrame::Pending(value),
                            None => BorrowedFrame::Indexed { source, cursor: Box::new(ArtifactCanonicalJsonCursor { maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH - top, ..ArtifactCanonicalJsonCursor::default() }) },
                        },
                        ArtifactCanonicalJsonValue::Array(values) => BorrowedFrame::Array { values, started: false, emitted: false },
                        ArtifactCanonicalJsonValue::Object(values) => BorrowedFrame::Object { values, phase: 0, emitted: false, current: None },
                    });
                }
                BorrowedFrame::String(mut value) => {
                    if let Some(byte) = value.next_byte() { self.frames[top] = Some(BorrowedFrame::String(value)); return Ok(Some(byte)); }
                    self.depth -= 1;
                }
                BorrowedFrame::Scalar { bytes, offset } => {
                    if offset == bytes.length { self.depth -= 1; continue; }
                    let byte = bytes.bytes[offset];
                    self.frames[top] = Some(BorrowedFrame::Scalar { bytes, offset: offset + 1 });
                    return Ok(Some(byte));
                }
                BorrowedFrame::Indexed { source, mut cursor } => {
                    if let Some(byte) = cursor.next_byte(source)? { self.frames[top] = Some(BorrowedFrame::Indexed { source, cursor }); return Ok(Some(byte)); }
                    self.depth -= 1;
                }
                BorrowedFrame::Array { mut values, started, emitted } => {
                    if !started { self.frames[top] = Some(BorrowedFrame::Array { values, started: true, emitted }); return Ok(Some(b'[')); }
                    let Some(value) = values.values.next() else { self.depth -= 1; return Ok(Some(b']')); };
                    self.frames[top] = Some(BorrowedFrame::Array { values, started, emitted: true });
                    self.push(BorrowedFrame::Pending(value))?;
                    if emitted { return Ok(Some(b',')); }
                }
                BorrowedFrame::Object { mut values, phase, emitted, mut current } => {
                    match phase {
                        0 => { self.frames[top] = Some(BorrowedFrame::Object { values, phase: 1, emitted, current }); return Ok(Some(b'{')); }
                        1 => {
                            let Some((key, value)) = values.values.next() else { self.depth -= 1; return Ok(Some(b'}')); };
                            self.frames[top] = Some(BorrowedFrame::Object { values, phase: 2, emitted: true, current: Some(value) });
                            self.push(BorrowedFrame::String(BorrowedString::new(key)))?;
                            if emitted { return Ok(Some(b',')); }
                        }
                        2 => { self.frames[top] = Some(BorrowedFrame::Object { values, phase: 3, emitted, current }); return Ok(Some(b':')); }
                        3 => {
                            let value = current.take().ok_or_else(|| "canonical-edit.borrowed-value-missing".to_string())?;
                            self.frames[top] = Some(BorrowedFrame::Object { values, phase: 1, emitted, current });
                            self.push(BorrowedFrame::Pending(value))?;
                        }
                        _ => return Err("canonical-edit.borrowed-object-phase".into()),
                    }
                }
            }
        }
        Err("canonical-edit.borrowed-expansion-limit".into())
    }

    pub(super) fn encode_chunk<T: ArtifactCanonicalJson>(&mut self, root: &T, output: &mut [u8]) -> Result<usize, ArtifactCanonicalJsonEncodeError> {
        self.bind(root).map_err(|reason| ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason })?;
        let mut written = 0;
        while written < output.len().min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES) {
            let Some(byte) = self.next_byte().map_err(|reason| ArtifactCanonicalJsonEncodeError { written_bytes: written, reason })? else { break };
            output[written] = byte;
            written += 1;
        }
        Ok(written)
    }

    pub(super) fn is_complete(&self) -> bool { self.started && self.depth == 0 }

    pub(super) fn reset(&mut self) -> Result<(), String> {
        if self.depth != 0 { return Err("canonical-edit.borrowed-reset-before-retirement".into()); }
        self.root_address = 0;
        self.started = false;
        Ok(())
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.depth == 0 && self.root_address == 0 && !self.started }

    pub(super) fn close_step(&mut self) -> Result<SnapshotRetirementStep, String> {
        if self.depth != 0 {
            self.depth -= 1;
            self.frames[self.depth] = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.reset()?;
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }
}
//#endregion 🔒️RootedEncoding
