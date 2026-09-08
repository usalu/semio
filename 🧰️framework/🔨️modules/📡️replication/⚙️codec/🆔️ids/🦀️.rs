//! 🆔 Pack identity types and segment kind constants.

//#region 🔖️Ids
/// @emoji 🔑️ A blake3 content hash (32 bytes).await, formatted as lowercase hex via `Display`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentHash(pub [u8; 32]);

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContentHash({self})")
    }
}

/// @emoji 🧩️ Identity of a chunk within a pack file's chunk table.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkId(pub u32);

/// @emoji 🏷️ The one-byte kind tag stamped on every segment; see `KIND_*` constants below.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SegmentKind(pub u8);

/// @emoji 🗜️ The one-byte compression codec identifier stamped on segment flags.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CodecId(pub u8);

/// @emoji 📏️ An absolute byte offset paired with a length, used for spans into a pack file.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ByteRange {
    pub offset: u64,
    pub len: u64,
}
//#endregion 🔖️Ids

//#region 🔖️SegmentKinds
/// @emoji 🔚️ Marks the end of the segment stream.
pub const KIND_END: u8 = 0x00;
/// @emoji 🗺️ The manifest segment: spans + counts describing the rest of the file.
pub const KIND_MANIFEST: u8 = 0x01;
/// @emoji 🧬️ An embedded schema description segment.
pub const KIND_SCHEMA: u8 = 0x02;
/// @emoji 🔤️ The interned string table segment.
pub const KIND_SYMBOLS: u8 = 0x03;
/// @emoji 📄️ The encoded document body segment.
pub const KIND_DOCUMENT: u8 = 0x04;
/// @emoji 🧱️ One chunk of blob data, framed like any other segment.
pub const KIND_CHUNK: u8 = 0x05;
/// @emoji 📇️ The chunk table segment: offset/len/crc/hash per chunk.
pub const KIND_CHUNK_TABLE: u8 = 0x06;
/// @emoji 📸️ A snapshot segment.
pub const KIND_SNAPSHOT: u8 = 0x07;
/// @emoji 🔎️ A field index segment.
pub const KIND_FIELD_INDEX: u8 = 0x08;
/// @emoji ⬜️ Padding, skipped on read.
pub const KIND_PADDING: u8 = 0x7F;
//#endregion 🔖️SegmentKinds

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
