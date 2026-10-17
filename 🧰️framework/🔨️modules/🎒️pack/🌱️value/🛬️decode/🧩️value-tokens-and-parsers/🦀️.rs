//! 🧩️ Value tokens and parsers of the retained record decoder: the role, container and token enums, the varint and UTF-8 readers and the continuation states. See [`super`].

use super::*;

//#region 🔖️RetainedValue
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedValueRole {
    Count,
    FieldId,
    Integer,
    Unsigned,
    Enum,
    Symbol,
    Chunk,
    StringLength,
    BytesLength,
    TableRows,
    TableField,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedValueContainer {
    Record,
    Tuple,
    List,
    Statements,
    Map,
    ChunkedBytes,
    Table,
    PackedF64,
    PackedVarint,
    Wire,
}

semio_framework_value::artifact_retire_leaf!(RetainedValueContainer);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedValueToken {
    Tag { offset: u64, value: u8 },
    Begin { kind: RetainedValueContainer, count: u64 },
    Unsigned { role: RetainedValueRole, value: u64 },
    Signed(i64),
    F64(u64),
    StringChar(char),
    Byte(u8),
    WirePresence(u8),
    WireNodePresence(u8),
    WireLabelPresence(u8),
    TablePresence { rows: u64, value: u8 },
    TableBitmap { first_row: u64, value: u8 },
    End(RetainedValueContainer),
    Complete { bytes: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RetainedContext {
    Field,
    Dsl,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct RetainedVarint {
    pub(super) value: u64,
    pub(super) bytes: u8,
}

impl RetainedVarint {
    pub(super) fn admit(&mut self, byte: u8, offset: u64) -> Result<Option<u64>, PackRefusal> {
        if self.bytes >= 10 || (self.bytes == 9 && ((byte & 0x80) != 0 || byte & 0x7f > 1)) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-varint", offset: offset - self.bytes as u64, detail: "overlong varint" });
        }
        let payload = (byte & 0x7f) as u64;
        self.value |= payload << (self.bytes as u32 * 7);
        self.bytes += 1;
        if byte & 0x80 != 0 {
            return Ok(None);
        }
        if self.bytes > 1 && payload == 0 {
            return Err(PackRefusal::NonCanonical("non-minimal retained varint"));
        }
        Ok(Some(self.value))
    }

    pub(super) fn preview(self, byte: u8, offset: u64) -> Result<Option<u64>, PackRefusal> {
        let mut cursor = self;
        cursor.admit(byte, offset)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct RetainedUtf8 {
    pub(super) value: u32,
    minimum: u32,
    pub(super) remaining: u8,
}

impl RetainedUtf8 {
    pub(super) fn admit(&mut self, byte: u8, offset: u64) -> Result<Option<char>, PackRefusal> {
        if self.remaining == 0 {
            match byte {
                0x00..=0x7f => return Ok(Some(byte as char)),
                0xc2..=0xdf => (self.value, self.minimum, self.remaining) = ((byte & 0x1f) as u32, 0x80, 1),
                0xe0..=0xef => (self.value, self.minimum, self.remaining) = ((byte & 0x0f) as u32, 0x800, 2),
                0xf0..=0xf4 => (self.value, self.minimum, self.remaining) = ((byte & 7) as u32, 0x10000, 3),
                _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid leading byte" }),
            }
            return Ok(None);
        }
        if byte & 0xc0 != 0x80 {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid continuation" });
        }
        self.value = (self.value << 6) | (byte & 0x3f) as u32;
        self.remaining -= 1;
        if self.remaining != 0 {
            return Ok(None);
        }
        if self.value < self.minimum || (0xd800..=0xdfff).contains(&self.value) || self.value > 0x10ffff {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid scalar" });
        }
        char::from_u32(self.value).map(Some).ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid scalar" })
    }

    pub(super) fn preview(self, byte: u8, offset: u64) -> Result<Option<char>, PackRefusal> {
        let mut cursor = self;
        cursor.admit(byte, offset)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AfterVarint {
    Record(u16),
    Field,
    Scalar(RetainedValueRole),
    Sequence(RetainedValueContainer, u16, RetainedContext),
    Statements(u16),
    Map(u16, RetainedContext),
    String,
    Bytes,
    Chunks,
    TableRows(u16),
    TableColumns(u64, u16),
    TableField(u64, u64, u16),
    Packed(RetainedValueContainer),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Expect {
    Finish,
    End(RetainedValueContainer),
    Value(u16, RetainedContext),
    StringTag,
    Varint(RetainedVarint, AfterVarint),
    F64([u8; 8], usize),
    Utf8(u64, RetainedUtf8),
    Bytes(u64),
    Record(u64, u16),
    Values(u64, u16, RetainedContext),
    Varints(u64, RetainedValueRole),
    F64s(u64),
    Statements(u64, u16),
    Map(u64, u16, RetainedContext),
    Wire(u16),
    WireNode,
    WireLabel,
    TableColumns(u64, u64, u16),
    TablePresence(u64, u64, u16),
    TableBitmap(u64, u64, u64, u64, u64, u16),
    TableElem(u64, u64, u64, u16),
}
//#endregion 🔖️RetainedValue
