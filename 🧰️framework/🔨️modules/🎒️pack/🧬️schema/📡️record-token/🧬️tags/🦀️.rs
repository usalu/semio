//! 🧬️ Closed schema-aware record wire tag assignments.
//#region 🔖️Tags
/// 🕳️ `FieldValue::Absent` — never written at record-field granularity (canonical mode
/// omits it entirely) but valid as a decode target, e.g. inside a sparse `TableSoA` fallback
/// column or a hand-crafted non-canonical file.
pub const TAG_ABSENT: u8 = 0x00;
pub const TAG_FALSE: u8 = 0x01;
pub const TAG_TRUE: u8 = 0x02;
pub const TAG_INT: u8 = 0x03;
pub const TAG_UINT: u8 = 0x04;
pub const TAG_F64: u8 = 0x05;
pub const TAG_STR: u8 = 0x06;
pub const TAG_STR_INLINE: u8 = 0x07;
pub const TAG_BYTES: u8 = 0x08;
pub const TAG_BYTES_CHUNKED: u8 = 0x09;
pub const TAG_ENUM: u8 = 0x0A;
pub const TAG_TUPLE: u8 = 0x0B;
pub const TAG_LIST: u8 = 0x0C;
pub const TAG_RECORD: u8 = 0x0D;
pub const TAG_BLOCK: u8 = 0x0E;
pub const TAG_STATEMENTS: u8 = 0x0F;
pub const TAG_MAP: u8 = 0x10;
pub const TAG_VALUE: u8 = 0x11;
pub const TAG_NULL: u8 = 0x12;
pub const TAG_WIRE: u8 = 0x13;
pub const TAG_TABLE_SOA: u8 = 0x14;
pub const TAG_PACKED_F64: u8 = 0x15;
pub const TAG_PACKED_VARINT: u8 = 0x16;
pub const TAG_EXPR: u8 = 0x17;
//#endregion 🔖️Tags

