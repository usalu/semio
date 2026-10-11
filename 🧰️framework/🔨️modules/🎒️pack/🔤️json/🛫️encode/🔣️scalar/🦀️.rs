//! 🔣️ Fixed native scalar bytes and borrowed typed JSON roles share one canonical owner.
use crate::ArtifactCanonicalJsonText;
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🧬️ A borrowed typed JSON node; callers cannot supply encoded bytes or a digest.
#[derive(Clone, Copy, Debug)]
pub enum ArtifactCanonicalJsonNode<'a> {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    /// 🔤️ An unsigned 64-bit integer spelled as its canonical decimal JSON string, for wires that carry ids beyond 2^53.
    U64Text(u64),
    /// 🔤️ A signed 64-bit integer spelled as its canonical decimal JSON string, for wires that carry exact signed64 values.
    I64Text(i64),
    I128(i128),
    U128(u128),
    F32(f32),
    /// 🔢️ A binary32 spelled as its IEEE-754 bit pattern in eight lowercase hexadecimal digits inside a JSON string, for wires that must carry exact floats.
    F32HexWord(f32),
    F64(f64),
    String(&'a str),
    Text(ArtifactCanonicalJsonText<'a>),
    Array(usize),
    Object(usize),
}

pub struct ArtifactCanonicalJsonScalarBytes {
    pub bytes: [u8; 64],
    pub length: usize,
}

impl ArtifactCanonicalJsonScalarBytes {
    /// 🔓️ Every arm but `F32` is now serde-free: `null`/`bool`/plain-decimal integers have a
    /// single unambiguous JSON spelling (no shortest-round-trip question the way floats have), so
    /// they are written directly; `F64` routes through the allocation-free `pack::json::write_float_to`, proven
    /// byte-identical to `serde_json`'s own `f64` writer for every value (`.🧬semio/🦑️repo/
    /// 🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
    /// 🔍️research/📓️float-format-parity.md`). `F32` stays on `serde_json` — that proof covers only
    /// `f64`, and `zmij`'s `f32` path uses a materially different threshold/precision budget this
    /// ticket did not verify.
    pub fn from_node(node: ArtifactCanonicalJsonNode<'_>) -> Result<Self, ValueError> {
        let mut scalar = Self { bytes: [0; 64], length: 0 };
        scalar.write_node(node)?;
        Ok(scalar)
    }

    /// ✍️ Initializes only the real scalar prefix inside its already retained inline backing.
    pub fn write_node(&mut self, node: ArtifactCanonicalJsonNode<'_>) -> Result<semio_framework_value::RetainedCloneProgress, ValueError> {
        self.length = 0;
        use std::io::Write as _;
        let result=match node {
            ArtifactCanonicalJsonNode::Null => self.write_all(b"null").map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::Bool(value) => self.write_all(if value { b"true" } else { b"false" }).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::I64(value) => write!(self, "{value}").map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::U64(value) => write!(self, "{value}").map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::U64Text(value) => write!(self, "\"{value}\"").map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::I64Text(value) => write!(self, "\"{value}\"").map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::I128(value) => write!(self, "{value}").map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::U128(value) => write!(self, "{value}").map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::F32HexWord(value) => write!(self, "\"{:08x}\"", value.to_bits()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::F32(value) => semio_framework_value::serde_json::to_writer(&mut *self, &value).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            ArtifactCanonicalJsonNode::F64(value) => crate::write_float_to(value, &mut *self).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue,"canonical scalar fixed output refused")),
            _ => return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"canonical-edit.invalid-typed-path")),
        };
        result.map_err(|error|error.with_retained_progress(semio_framework_value::RetainedCloneProgress{copied_items:usize::from(self.length!=0),copied_bytes:self.length,..Default::default()}))?;
        Ok(semio_framework_value::RetainedCloneProgress {copied_items:1,copied_bytes:self.length,..Default::default()})
    }
}

impl std::fmt::Write for ArtifactCanonicalJsonScalarBytes {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.length.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(std::fmt::Error)?;
        self.bytes[self.length..end].copy_from_slice(text.as_bytes());
        self.length = end;
        Ok(())
    }
}

impl std::io::Write for ArtifactCanonicalJsonScalarBytes {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let end = self.length.checked_add(bytes.len()).filter(|end| *end <= self.bytes.len()).ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        self.bytes[self.length..end].copy_from_slice(bytes);
        self.length = end;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

