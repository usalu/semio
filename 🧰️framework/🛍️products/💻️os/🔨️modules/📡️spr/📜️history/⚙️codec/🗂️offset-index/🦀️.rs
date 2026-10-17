//! 🗂️ Offset index of the history codec: the section identifiers, the pair and offset section writers and the index builder and reader. See [`super`].

use super::*;

//#region 🔖️Codec
pub const SEC_EDIT_OFFSETS: u8 = 0x01;

pub const SEC_CHECKPOINT_OFFSETS: u8 = 0x02;

pub const SEC_DICT_OFFSETS: u8 = 0x03;

pub const SEC_SNAPSHOT_OFFSETS: u8 = 0x04;

pub const SEC_SEALED_OFFSETS: u8 = 0x05;

async fn write_pair_section(out: &mut ByteWriter, kind: u8, entries: &[(u64, u64)]) {
    out.write_u8(kind);
    out.write_varint_u64(entries.len() as u64);
    for (a, b) in entries {
        out.write_varint_u64(*a);
        out.write_varint_u64(*b);
    }
}

async fn write_offsets_section(out: &mut ByteWriter, kind: u8, offsets: &[u64]) {
    out.write_u8(kind);
    out.write_varint_u64(offsets.len() as u64);
    for offset in offsets {
        out.write_varint_u64(*offset);
    }
}

#[derive(Clone, Debug, Default)]
pub struct IndexBuilder {
    pub(super) edits: Vec<(u64, u64)>,
    pub(super) checkpoints: Vec<(String, u64, u64)>,
    dict_offsets: Vec<u64>,
    snapshots: Vec<(u64, u64)>,
    sealed: Vec<u64>,
}

impl IndexBuilder {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn record_edit(&mut self, ordinal: u64, offset: u64) {
        self.edits.push((ordinal, offset));
    }

    pub async fn record_checkpoint(&mut self, id: &str, offset: u64, edit_ordinal: u64) {
        self.checkpoints.push((id.to_string(), offset, edit_ordinal));
    }

    pub async fn record_dict(&mut self, offset: u64) {
        self.dict_offsets.push(offset);
    }

    pub async fn record_snapshot(&mut self, edit_ordinal: u64, offset: u64) {
        self.snapshots.push((edit_ordinal, offset));
    }

    pub async fn record_sealed(&mut self, offset: u64) {
        self.sealed.push(offset);
    }

    pub async fn build(&self) -> Vec<u8> {
        let mut out = ByteWriter::new();
        out.write_u8(1);
        write_pair_section(&mut out, SEC_EDIT_OFFSETS, &self.edits).await;
        out.write_u8(SEC_CHECKPOINT_OFFSETS);
        out.write_varint_u64(self.checkpoints.len() as u64);
        for (id, offset, edit_ordinal) in &self.checkpoints {
            write_str_field(&mut out, id).await;
            out.write_varint_u64(*offset);
            out.write_varint_u64(*edit_ordinal);
        }
        write_offsets_section(&mut out, SEC_DICT_OFFSETS, &self.dict_offsets).await;
        write_pair_section(&mut out, SEC_SNAPSHOT_OFFSETS, &self.snapshots).await;
        write_offsets_section(&mut out, SEC_SEALED_OFFSETS, &self.sealed).await;
        out.into_bytes()
    }
}

pub struct IndexReader<'a> {
    pub(super) edits: Vec<(u64, u64)>,
    pub(super) checkpoints: Vec<(&'a str, u64, u64)>,
    snapshots: Vec<(u64, u64)>,
}

impl<'a> IndexReader<'a> {
    pub async fn open(payload: &'a [u8]) -> Result<Self, ProtocolError> {
        let mut input = ByteReader::new(payload);
        let format = input.read_u8()?;
        if format > 1 {
            return Err(malformed_fmt("index", format).await);
        }
        let mut edits = Vec::new();
        let mut checkpoints = Vec::new();
        let mut snapshots = Vec::new();
        while input.remaining() > 0 {
            let kind = input.read_u8()?;
            let count = input.read_varint_u64()?;
            match kind {
                SEC_EDIT_OFFSETS => {
                    for _ in 0..count {
                        let ordinal = input.read_varint_u64()?;
                        let offset = input.read_varint_u64()?;
                        edits.push((ordinal, offset));
                    }
                }
                SEC_CHECKPOINT_OFFSETS => {
                    for _ in 0..count {
                        let len = input.read_varint_u64()? as usize;
                        let bytes = input.read_bytes(len)?;
                        let id = std::str::from_utf8(bytes).map_err(|_| ProtocolError::Malformed { what: "index checkpoint id utf8", offset: 0, detail: "invalid utf-8".to_string() })?;
                        let offset = input.read_varint_u64()?;
                        let edit_ordinal = input.read_varint_u64()?;
                        checkpoints.push((id, offset, edit_ordinal));
                    }
                }
                SEC_DICT_OFFSETS => {
                    for _ in 0..count {
                        input.read_varint_u64()?;
                    }
                }
                SEC_SNAPSHOT_OFFSETS => {
                    for _ in 0..count {
                        let ordinal = input.read_varint_u64()?;
                        let offset = input.read_varint_u64()?;
                        snapshots.push((ordinal, offset));
                    }
                }
                SEC_SEALED_OFFSETS => {
                    for _ in 0..count {
                        input.read_varint_u64()?;
                    }
                }
                other => return Err(ProtocolError::Malformed { what: "index section kind", offset: 0, detail: format!("unknown section {other:#x}") }),
            }
        }
        Ok(Self { edits, checkpoints, snapshots })
    }

    pub async fn edit_offset_at_or_before(&self, ordinal: u64) -> Option<u64> {
        self.edits.iter().filter(|(o, _)| *o <= ordinal).max_by_key(|(o, _)| *o).map(|(_, offset)| *offset)
    }

    pub async fn checkpoint_offset(&self, checkpoint_id: &str) -> Option<(u64, u64)> {
        self.checkpoints.iter().find(|(id, _, _)| *id == checkpoint_id).map(|(_, offset, edit_ordinal)| (*offset, *edit_ordinal))
    }

    pub async fn latest_snapshot_offset_at_or_before(&self, ordinal: u64) -> Option<u64> {
        self.snapshots.iter().filter(|(o, _)| *o <= ordinal).max_by_key(|(o, _)| *o).map(|(_, offset)| *offset)
    }
}
//#endregion 🔖️Index
