//! 📝️ Borrowed ordered domain text with one structural event or sixty-four copied bytes per turn.
use super::{ValueError, ValueRefusalKind, ArtifactPreparedOperationError};
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneGrant};

/// 🌳️ Exact domain text leaves and ordered delimiters without intermediate ownership.
pub enum ArtifactOperationTextNode<'a> {
    Bytes(&'a [u8]),
    Hex(&'a [u8]),
    Scalar,
    Sequence { length: usize, open: &'a [u8], separator: &'a [u8], close: &'a [u8] },
}

/// 🫳️ Ordinal lookup borrows the original typed operation; scalar formatting stays inline.
pub trait ArtifactOperationText: Sync {
    fn operation_text_node(&self, path: &[usize]) -> Result<ArtifactOperationTextNode<'_>, ValueError>;
    fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), ArtifactPreparedOperationError>;
}

#[derive(Clone, Copy, Default)]
struct Frame { phase: u8, offset: usize, child: usize }

/// 🧭️ Fixed depth traversal retains offsets alone and reborrows source authority each turn.
pub struct ArtifactOperationTextCursor {
    frames: [Frame; 64],
    path: [usize; 64],
    depth: usize,
    complete: bool,
}

impl Default for ArtifactOperationTextCursor {
    fn default() -> Self { Self { frames: [Frame::default(); 64], path: [0; 64], depth: 0, complete: false } }
}

impl ArtifactOperationTextCursor {
    pub fn is_complete(&self) -> bool { self.complete }

    fn pop(&mut self) { if self.depth == 0 { self.complete = true; } else { self.depth -= 1; } }

    pub fn advance(&mut self, source: &dyn ArtifactOperationText, output: &mut [u8], grant: RetainedCloneGrant) -> Result<usize, ArtifactPreparedOperationError> {
        if self.complete || output.is_empty() || grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 { return Ok(0); }
        if grant.maximum_depth < self.depth + 1 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "operation text current node exceeds supplied depth").into()); }
        let maximum = output.len().min(64).min(grant.maximum_copy_bytes);
        let output = &mut output[..maximum];
        let path = &self.path[..self.depth];
        let node = source.operation_text_node(path)?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            self.frames[self.depth].phase = if matches!(node, ArtifactOperationTextNode::Sequence { .. }) { 2 } else { 1 };
            return Ok(0);
        }
        if let ArtifactOperationTextNode::Sequence { length, open, separator, close } = node {
            match frame.phase {
                2 | 5 | 6 => {
                    let bytes = match frame.phase { 2 => open, 5 => separator, _ => close };
                    let count = output.len().min(64).min(bytes.len() - frame.offset);
                    output[..count].copy_from_slice(&bytes[frame.offset..frame.offset + count]);
                    self.frames[self.depth].offset += count;
                    if self.frames[self.depth].offset == bytes.len() {
                        self.frames[self.depth].offset = 0;
                        if frame.phase == 6 { self.pop(); } else { self.frames[self.depth].phase = 3; }
                    }
                    return Ok(count);
                }
                3 => {
                    if frame.child == length { self.frames[self.depth].phase = 6; return Ok(0); }
                    if self.depth + 1 == 64 || grant.maximum_depth < self.depth + 2 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "operation text child exceeds supplied or inline depth").into()); }
                    self.frames[self.depth].phase = 4;
                    self.path[self.depth] = frame.child;
                    self.depth += 1;
                    self.frames[self.depth] = Frame::default();
                    return Ok(0);
                }
                4 => {
                    self.frames[self.depth].child += 1;
                    self.frames[self.depth].phase = if self.frames[self.depth].child == length { 6 } else { 5 };
                    return Ok(0);
                }
                _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "operation text changed its node shape").into()),
            }
        }
        if frame.phase != 1 { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "operation text changed its leaf shape").into()); }
        if matches!(node, ArtifactOperationTextNode::Scalar) {
            let maximum = output.len().min(64);
            let (count, complete) = source.operation_text_scalar(path, frame.offset, &mut output[..maximum]).map_err(|error| {
                assert!(error.written_bytes <= maximum, "original text source refusal prefix exceeds supplied output");
                let progress=error.reason.retained_progress().checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:error.written_bytes,..Default::default()}).expect("one bounded scalar turn has representable physical receipt");
                ArtifactPreparedOperationError{written_bytes:error.written_bytes,reason:error.reason.with_retained_progress(progress)}
            })?;
            if count > maximum { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "operation scalar exceeded admitted output").into()); }
            self.frames[self.depth].offset += count;
            if complete { self.pop(); }
            return Ok(count);
        }
        let (bytes, hex) = match node { ArtifactOperationTextNode::Bytes(bytes) => (bytes, false), ArtifactOperationTextNode::Hex(bytes) => (bytes, true), _ => unreachable!() };
        let length = bytes.len().checked_mul(if hex { 2 } else { 1 }).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "operation text extent overflow"))?;
        let count = output.len().min(64).min(length - frame.offset);
        if hex { for (index, target) in output[..count].iter_mut().enumerate() { let ordinal = frame.offset + index; let byte = bytes[ordinal / 2]; *target = b"0123456789abcdef"[usize::from(if ordinal % 2 == 0 { byte >> 4 } else { byte & 15 })]; } }
        else { output[..count].copy_from_slice(&bytes[frame.offset..frame.offset + count]); }
        self.frames[self.depth].offset += count;
        if self.frames[self.depth].offset == length { self.pop(); }
        Ok(count)
    }
}
