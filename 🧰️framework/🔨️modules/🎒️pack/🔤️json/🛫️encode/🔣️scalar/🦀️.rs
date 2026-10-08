//! 🔣️ Fixed native scalar bytes and borrowed typed JSON roles share one canonical owner.
use crate::ArtifactCanonicalJsonText;

/// 🧬️ A borrowed typed JSON node; callers cannot supply encoded bytes or a digest.
#[derive(Clone, Copy, Debug)]
pub enum ArtifactCanonicalJsonNode<'a> {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
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
    pub fn from_node(node: ArtifactCanonicalJsonNode<'_>) -> Result<Self, String> {
        let mut scalar = Self { bytes: [0; 64], length: 0 };
        use std::io::Write as _;
        match node {
            ArtifactCanonicalJsonNode::Null => scalar.write_all(b"null").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::Bool(value) => scalar.write_all(if value { b"true" } else { b"false" }).map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::I64(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::U64(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::I128(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::U128(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::F32(value) => semio_framework_value::serde_json::to_writer(&mut scalar, &value).map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::F64(value) => crate::write_float_to(value, &mut scalar).map_err(|error| error.to_string()),
            _ => return Err("canonical-edit.invalid-typed-path".into()),
        }?;
        Ok(scalar)
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
        let end = self.length.checked_add(bytes.len()).filter(|end| *end <= self.bytes.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "canonical scalar exceeds fixed encoding"))?;
        self.bytes[self.length..end].copy_from_slice(bytes);
        self.length = end;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

