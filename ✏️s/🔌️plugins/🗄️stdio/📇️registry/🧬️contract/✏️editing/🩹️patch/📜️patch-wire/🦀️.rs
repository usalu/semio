//! 📜️ The snapshot patch value, its labels and text, hex and binary wire.

use super::*;

//#region 🩹️Patch
/// 🩹️ One path-scoped edit of a decoded snapshot. `path` (and `from` of a move) is an RFC 6901 pointer; `index` positions an
/// object member (only an exact inverse restoring an object member's place carries it); `key` is a rename's new member name.
/// A `splice` replaces `remove` units at `offset` of the container at `path` with the units of `value` (array items, octets,
/// UTF-8 bytes of a string, object members by position); `continued` marks a non-final part of one exact multi-part inverse,
/// whose whole-snapshot invariant the run's final part checks.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
pub enum SnapshotPatch {
    Set {
        path: String,
        value: DslValue,
    },
    Insert {
        path: String,
        value: DslValue,
        #[value(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    Remove {
        path: String,
    },
    Move {
        from: String,
        path: String,
        #[value(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    Rename {
        path: String,
        key: String,
    },
    Splice {
        path: String,
        offset: u64,
        remove: u64,
        value: DslValue,
        #[value(default, skip_serializing_if = "is_false")]
        continued: bool,
    },
}

impl SnapshotPatch {
    /// 📍️ The pointer this operation writes: a move's destination, every other operation's own path.
    pub fn path(&self) -> &str {
        match self {
            Self::Set { path, .. } | Self::Insert { path, .. } | Self::Remove { path } | Self::Move { path, .. } | Self::Rename { path, .. } | Self::Splice { path, .. } => path,
        }
    }

    /// ⏭️ Whether this operation is a non-final part of one exact multi-part inverse.
    pub fn continued(&self) -> bool {
        matches!(self, Self::Splice { continued: true, .. })
    }

    /// 🎯️ The decoded segments of [`Self::path`] — the conflict target the envelope carries (empty: the whole document).
    pub fn target(&self) -> Vec<String> {
        decode_pointer(self.path()).unwrap_or_default()
    }

    /// 🏷️ The wire name of this operation (`set`, `insert`, `remove`, `move`, `rename`).
    pub fn operation(&self) -> &'static str {
        match self {
            Self::Set { .. } => "set",
            Self::Insert { .. } => "insert",
            Self::Remove { .. } => "remove",
            Self::Move { .. } => "move",
            Self::Rename { .. } => "rename",
            Self::Splice { .. } => "splice",
        }
    }
}

/// 🏷️ The history label of one patch in every shell locale, naming the operation and the pointer it addresses.
pub fn snapshot_patch_label(patch: &SnapshotPatch) -> LocalizedLabel {
    let shown = |path: &str| if path.is_empty() { "/".to_string() } else { path.to_string() };
    match patch {
        SnapshotPatch::Set { path, .. } => LocalizedLabel::native(&format!("Set {}", shown(path)), &format!("{} setzen", shown(path))),
        SnapshotPatch::Insert { path, .. } => LocalizedLabel::native(&format!("Insert {}", shown(path)), &format!("{} einfügen", shown(path))),
        SnapshotPatch::Remove { path } => LocalizedLabel::native(&format!("Remove {}", shown(path)), &format!("{} entfernen", shown(path))),
        SnapshotPatch::Move { from, path, .. } => LocalizedLabel::native(&format!("Move {} to {}", shown(from), shown(path)), &format!("{} nach {} verschieben", shown(from), shown(path))),
        SnapshotPatch::Rename { path, key } => LocalizedLabel::native(&format!("Rename {} to {key}", shown(path)), &format!("{} in {key} umbenennen", shown(path))),
        SnapshotPatch::Splice { path, offset, .. } => LocalizedLabel::native(&format!("Replace part of {} at {offset}", shown(path)), &format!("Teil von {} ab {offset} ersetzen", shown(path))),
    }
}

impl semio_framework_dsl_record::BorrowedDslField for SnapshotPatch{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;}

impl semio_framework_dsl_record::DslField for SnapshotPatch {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(<Self as ToValue>::to_value(self))
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Value(value) = value else { return Err("snapshot patch requires a structured value".into()) };
        let patch = <Self as FromValue>::from_value(value.clone()).map_err(|error| error.to_string())?;
        validate_patch(&patch).map_err(|error| error.to_string())?;
        Ok(patch)
    }
}

impl kernel::OpText for SnapshotPatch {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if line.len() > SNAPSHOT_PATCH_MAX_BYTES {
            return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "snapshot patch exceeds the native publication item limit", semio_framework_diagnostic::TextSpan::at(1, 1)));
        }
        let patch: Self =
            semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        validate_patch(&patch).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(patch)
    }
}

/// 🔡️ The patch as lowercase hexadecimal of its canonical JSON — the one-physical-line payload every hand-written stdio text
/// codec embeds (`patch-snapshot patch=<hex>`, or a `<tag>:<hex>` envelope).
pub fn snapshot_patch_hex(patch: &SnapshotPatch) -> String {
    <SnapshotPatch as kernel::OpText>::print_op(patch).bytes().map(|byte| format!("{byte:02x}")).collect()
}

/// 🔡️ Parses a [`snapshot_patch_hex`] payload back into its validated patch, refusing malformed hexadecimal.
pub fn snapshot_patch_from_hex(hex: &str) -> Result<SnapshotPatch, String> {
    if !hex.len().is_multiple_of(2) || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) || hex.len() > SNAPSHOT_PATCH_MAX_BYTES * 2 {
        return Err("snapshot patch is not hexadecimal of even length within the patch budget".into());
    }
    let bytes = (0..hex.len()).step_by(2).map(|at| u8::from_str_radix(&hex[at..at + 2], 16).map_err(|error| error.to_string())).collect::<Result<Vec<u8>, String>>()?;
    let json = String::from_utf8(bytes).map_err(|error| error.to_string())?;
    <SnapshotPatch as kernel::OpText>::parse_op(&json).map_err(|error| error.to_string())
}

/// 📝️ The `patch-snapshot` op line of a hand-written aggregate text codec: `patch-snapshot patch=<hex of the patch JSON>` (the
/// grammar rule `patch-snapshot = "patch-snapshot" "patch" "=" hex` every stdio mutation grammar shares).
pub fn snapshot_patch_text(patch: &SnapshotPatch) -> String {
    format!("patch-snapshot patch={}", snapshot_patch_hex(patch))
}

/// 📝️ Parses a [`snapshot_patch_text`] line back into its patch, refusing any other opcode or malformed hex.
pub fn snapshot_patch_from_text(line: &str) -> Result<SnapshotPatch, String> {
    snapshot_patch_from_hex(line.strip_prefix("patch-snapshot patch=").ok_or_else(|| "expected `patch-snapshot patch=<hex>`".to_string())?)
}

/// 💾️ The patch carried by the rest of a hand-written aggregate binary frame (its canonical JSON bytes, as [`OpBinary::encode_op`]
/// writes them), with the refusal as text for decoders that report strings.
pub fn snapshot_patch_from_bytes(bytes: &[u8]) -> Result<SnapshotPatch, String> {
    SnapshotPatch::decode_op(bytes).map_err(|error| error.to_string())
}

impl OpBinary for SnapshotPatch {
    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        Ok(<Self as kernel::OpText>::print_op(self).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|error| kernel::ProtocolError::Malformed { what: "snapshot patch utf8", offset: 0, detail: error.to_string() })?;
        <Self as kernel::OpText>::parse_op(line).map_err(|error| kernel::ProtocolError::Malformed { what: "snapshot patch", offset: 0, detail: error.to_string() })
    }
}

//#endregion 🩹️Patch
