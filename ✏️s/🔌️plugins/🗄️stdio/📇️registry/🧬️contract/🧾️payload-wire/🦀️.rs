//! 🧾️ Wire helpers every stdio artifact shares: operation payload JSON, draft splices, ordered rows, the double-option decoder, Base64, semantic fingerprints and the serde operation codec.

pub use semio_framework_diagnostic as diagnostic;
pub use semio_framework_os_kernel as kernel;
pub use semio_framework_value as value;

use semio_framework_value::ToValue;

//#region 🧾️PayloadWire
/// 🧾️ Builds the operation of semantic kind `kind` from its leaf wire payload — JSON text in the exact `payload_value()`
/// shape — through the derive-generated [`kernel::Mutation::from_payload_value`]: the one decoder a wire-form `🥒️.feature`
/// row goes through, so no case adapter maps parameters onto an operation by hand. An artifact crate re-exports it for the
/// adapters that link only that crate.
pub fn mutation_from_payload_json<P, M: kernel::Mutation<P>>(kind: &str, payload: &str) -> Result<M, String> {
    let value = semio_framework_pack_json::parse(payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("{kind} payload is not JSON: {error}"))?;
    M::from_payload_value(kind, semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| format!("{kind} payload does not decode: {error}"))
}

/// 🧾️ The leaf wire payload of `operation` as JSON text — the derive-generated [`kernel::Mutation::payload_value`] the
/// decoder above reads back, so a case adapter can hold a row's `params` against what the subject itself would emit.
pub fn mutation_payload_json<P, M: kernel::Mutation<P>>(operation: &M) -> String {
    semio_framework_pack_json::to_json_string(&operation.payload_value())
}

/// 🚫️ An operation dispatch refused: the outcome's first message, by its frozen `mutation.*` code and its sentence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MutationRefusal {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for MutationRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[{}] {}", self.code, self.message)
    }
}

impl From<MutationRefusal> for String {
    fn from(refusal: MutationRefusal) -> String {
        refusal.to_string()
    }
}

/// ▶️ Applies `operation` to `snapshot` through its own diff and refuses — leaving `snapshot` untouched — when the
/// outcome carries a message, so a case adapter sees a rejection, by code, instead of encoding an unchanged model.
pub fn apply_mutation_checked<P, M: kernel::Mutation<P>>(snapshot: &mut P, operation: &M) -> Result<(), MutationRefusal> {
    let outcome = operation.diff(snapshot);
    if let Some(message) = outcome.messages().first() {
        return Err(MutationRefusal { code: message.code.0.to_string(), message: message.message.to_string() });
    }
    *snapshot = kernel::apply_diff(outcome.diff(), snapshot).map_err(|error| MutationRefusal { code: error.code.to_string(), message: error.message.to_string() })?;
    Ok(())
}

/// ▶️ The central apply for a caller that holds a snapshot (an io drain, an editor, an oracle, a test): the mutation's sparse diff goes through
/// [`kernel::apply_diff`], the only applier, and a rejected apply leaves `snapshot` untouched and answers a fatal outcome carrying the apply error.
/// Artifacts re-export it under their own `apply_<artifact>_mutation` name; no schema module applies a diff itself.
pub fn apply_mutation<P, M: kernel::Mutation<P>>(snapshot: &mut P, mutation: &M) -> kernel::MutationOutcome<M::Diff> {
    let outcome = mutation.diff(snapshot);
    match kernel::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => kernel::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#region ✂️DraftSplices
/// ✂️ One range of an explicit text draft's change set as the window publishes it: `delete` scalars at `offset` of the text the draft
/// started from were replaced by `insert`. Ranges ascend and never overlap; offsets and lengths count Unicode scalar values (`char`s).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DraftSplice {
    pub offset: usize,
    pub delete: usize,
    pub insert: String,
}

fn splice_fault(code: &'static str, message: impl Into<String>) -> semio_framework_plugin::Fault {
    semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
/// 📥️ Reads a change set from its JSON text (the same list `window_kit_splices_argument` reads from the staged arguments).
pub fn draft_splices_from_json(text: &str) -> Result<Vec<DraftSplice>, semio_framework_plugin::Fault> {
    let parsed = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| splice_fault("stdio.window-kit.splices-json", format!("the change set is not JSON: {error}")))?;
    let semio_framework_value::DslValue::Array(rows) = semio_framework_pack_json::to_dsl_value(&parsed) else {
        return Err(splice_fault("stdio.window-kit.splices-shape", "the change set must be a list of ranges"));
    };
    let number = |value: &semio_framework_value::DslValue| match value {
        semio_framework_value::DslValue::Number(number) => number.as_u64().and_then(|value| usize::try_from(value).ok()),
        _ => None,
    };
    let mut splices = Vec::with_capacity(rows.len());
    for row in &rows {
        let semio_framework_value::DslValue::Object(members) = row else { return Err(splice_fault("stdio.window-kit.splices-shape", "every range must be an object")) };
        let field = |name: &str| members.iter().find(|(key, _)| key == name).map(|(_, value)| value);
        if members.iter().any(|(key, _)| !matches!(key.as_str(), "offset" | "delete" | "insert")) {
            return Err(splice_fault("stdio.window-kit.splices-shape", "a range names only offset, delete and insert"));
        }
        let (Some(offset), Some(delete), Some(semio_framework_value::DslValue::String(insert))) = (field("offset").and_then(number), field("delete").and_then(number), field("insert")) else {
            return Err(splice_fault("stdio.window-kit.splices-shape", "a range needs integer offset and delete and text insert"));
        };
        splices.push(DraftSplice { offset, delete, insert: insert.clone() });
    }
    if splices.windows(2).any(|pair| pair[1].offset < pair[0].offset.saturating_add(pair[0].delete)) {
        return Err(splice_fault("stdio.window-kit.splices-order", "the ranges of a change set ascend and never overlap"));
    }
    Ok(splices)
}

/// ✂️ Applies a change set to `text` (ranges in the coordinates of `text`, counted in scalars); `None` when a range leaves the text.
pub fn apply_draft_splices(text: &str, splices: &[DraftSplice]) -> Option<String> {
    let scalars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len());
    let mut position = 0usize;
    for splice in splices {
        let end = splice.offset.checked_add(splice.delete)?;
        if splice.offset < position || end > scalars.len() {
            return None;
        }
        result.extend(&scalars[position..splice.offset]);
        result.push_str(&splice.insert);
        position = end;
    }
    result.extend(&scalars[position..]);
    Some(result)
}

/// ↩️ The change set that undoes `splices` on the text they were taken from: every range puts back the scalars its forward range
/// deleted (read from `text`) in place of the ones it inserted, at its place in the edited text. `None` when a range leaves `text`.
pub fn invert_draft_splices(text: &str, splices: &[DraftSplice]) -> Option<Vec<DraftSplice>> {
    let scalars: Vec<char> = text.chars().collect();
    let mut shift: isize = 0;
    let mut inverse = Vec::with_capacity(splices.len());
    for splice in splices {
        let end = splice.offset.checked_add(splice.delete)?;
        if end > scalars.len() {
            return None;
        }
        let offset = usize::try_from(isize::try_from(splice.offset).ok()?.checked_add(shift)?).ok()?;
        inverse.push(DraftSplice { offset, delete: splice.insert.chars().count(), insert: scalars[splice.offset..end].iter().collect() });
        shift = shift.checked_add(isize::try_from(splice.insert.chars().count()).ok()?)?.checked_sub(isize::try_from(splice.delete).ok()?)?;
    }
    Some(inverse)
}
//#endregion ✂️DraftSplices

//#region 🔢️OrderedRows
/// 🔢️ The index/key rows of a diff in ascending order — a new ordered list built from the read-only input, never an edited copy of a snapshot.
pub fn ordered<T: Clone + Ord>(rows: &[T]) -> Vec<T> {
    let mut sorted = Vec::with_capacity(rows.len());
    sorted.extend_from_slice(rows);
    sorted.sort_unstable();
    sorted
}

/// 🔢️ The rows ascending with duplicates dropped.
pub fn ordered_unique<T: Clone + Ord>(rows: &[T]) -> Vec<T> {
    let mut sorted = ordered(rows);
    sorted.dedup();
    sorted
}

/// 🔢️ The rows descending with duplicates dropped (the order removals replay in so earlier indices stay valid).
pub fn ordered_unique_descending<T: Clone + Ord>(rows: &[T]) -> Vec<T> {
    let mut sorted = ordered_unique(rows);
    sorted.reverse();
    sorted
}

/// 🔢️ The rows stably ordered by `key` (the insertion order of positional additions).
pub fn ordered_by_key<T: Clone, K: Ord>(rows: &[T], key: impl Fn(&T) -> K) -> Vec<T> {
    let mut sorted = Vec::with_capacity(rows.len());
    sorted.extend_from_slice(rows);
    sorted.sort_by_key(key);
    sorted
}
//#endregion 🔢️OrderedRows

/// ↩️ [`kernel::Mutation::inverse`] against `base`, reachable from a crate that cannot name the kernel trait — the
/// production inverse itself, never a copy of its rules.
pub fn mutation_inverse<P, M: kernel::Mutation<P>>(operation: &M, base: &P) -> Result<Vec<M>, semio_framework_value::ValueError> {
    Ok(operation.inverse(base)?)
}
//#endregion 🧾️PayloadWire

//#region 🪆️DoubleOption
/// 🪆️ Decodes a PRESENT key of an `Option<Option<T>>` field as `Some(inner)`, so a present `null` is `Some(None)` (cleared,
/// removed, valueless) and only an absent key (the field's bare `#[value(default)]`) is `None` (untouched): the value derive's
/// blanket `Option<T>` impl reads `Null` as absence at any depth and would collapse both. Paired with
/// `skip_serializing_if = "Option::is_none"`, `payload_value()` and `with_payload_value()` round-trip — the
/// `deserialize_double_option` shape the value derive's own docs name (`🧰️framework/🔨️modules/🌱️value/✨️derive`).
pub fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}
//#endregion 🪆️DoubleOption

//#region 🧾️SnapshotSource
/// 🧾️ Prints every typed snapshot detail without passing through a native file encoder.
pub fn snapshot_edit_source<S: ToValue>(snapshot: &S) -> String {
    semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&snapshot.to_value()))
}
//#endregion 🧾️SnapshotSource

/// 🔡 Encodes bytes with the padded RFC 4648 standard Base64 alphabet.
pub fn base64_standard(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3).saturating_mul(4));
    let (chunks, remainder) = bytes.as_chunks::<3>();
    for chunk in chunks {
        let value = u32::from_be_bytes([0, chunk[0], chunk[1], chunk[2]]);
        output.push(ALPHABET[((value >> 18) & 63) as usize] as char);
        output.push(ALPHABET[((value >> 12) & 63) as usize] as char);
        output.push(ALPHABET[((value >> 6) & 63) as usize] as char);
        output.push(ALPHABET[(value & 63) as usize] as char);
    }
    match remainder {
        [first] => {
            output.push(ALPHABET[(first >> 2) as usize] as char);
            output.push(ALPHABET[((first & 3) << 4) as usize] as char);
            output.push_str("==");
        }
        [first, second] => {
            output.push(ALPHABET[(first >> 2) as usize] as char);
            output.push(ALPHABET[(((first & 3) << 4) | (second >> 4)) as usize] as char);
            output.push(ALPHABET[((second & 15) << 2) as usize] as char);
            output.push('=');
        }
        _ => {}
    }
    output
}

fn hash_hex_bytes(hash: &str) -> Vec<u8> {
    fn nibble(byte: u8) -> u8 {
        match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => unreachable!("framework hash must be lowercase hexadecimal"),
        }
    }
    hash.as_bytes().as_chunks::<2>().0.iter().map(|pair| nibble(pair[0]) << 4 | nibble(pair[1])).collect()
}

/// 🪪 Computes the stable BLAKE3 identity of a semantic projection.
pub fn semantic_fingerprint<T: semio_framework_value::ToValue>(projection: &T) -> Result<Vec<u8>, String> {
    let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(projection))).into_bytes();
    Ok(hash_hex_bytes(&semio_framework_hash::hash_bytes(&encoded)))
}

/// 📡 Implements the canonical JSON text mutation codec, and the binary one: tagged by the vocabulary's
/// `📡️.protocol.semio` records (`kernel::tagged_value_binary`) when a protocol is given, JSON bytes otherwise.
#[macro_export]
macro_rules! impl_serde_op_codec {
    ($mutation:ty, $what:literal, protocol = $protocol:expr) => {
        impl $crate::kernel::OpText for $mutation {
            fn print_op(&self) -> String {
                ::semio_framework_pack_json::to_string(&::semio_framework_pack_json::from_dsl_value(&$crate::value::ToValue::to_value(self)))
            }

            fn parse_op(line: &str) -> Result<Self, $crate::diagnostic::TextError> {
                let parsed = ::semio_framework_pack_json::parse(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::diagnostic::TextError::from_value_error(error.into_value_error(), $crate::diagnostic::TextSpan::at(1, 1)))?;
                <Self as $crate::value::FromValue>::from_value(::semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| $crate::diagnostic::TextError::from_value_error(error, $crate::diagnostic::TextSpan::at(1, 1)))
            }
        }

        impl $crate::kernel::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, $crate::kernel::ProtocolError> {
                $crate::kernel::tagged_value_binary::encode_op($protocol, $crate::kernel::tagged_value_binary::VariantTag::Field("mutation"), self)
            }

            fn decode_op(bytes: &[u8]) -> Result<Self, $crate::kernel::ProtocolError> {
                $crate::kernel::tagged_value_binary::decode_op($protocol, $crate::kernel::tagged_value_binary::VariantTag::Field("mutation"), bytes)
            }
        }
    };
    ($mutation:ty, $what:literal) => {
        impl $crate::kernel::OpText for $mutation {
            fn print_op(&self) -> String {
                ::semio_framework_pack_json::to_string(&::semio_framework_pack_json::from_dsl_value(&$crate::value::ToValue::to_value(self)))
            }

            fn parse_op(line: &str) -> Result<Self, $crate::diagnostic::TextError> {
                let parsed = ::semio_framework_pack_json::parse(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::diagnostic::TextError::from_value_error(error.into_value_error(), $crate::diagnostic::TextSpan::at(1, 1)))?;
                <Self as $crate::value::FromValue>::from_value(::semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| $crate::diagnostic::TextError::from_value_error(error, $crate::diagnostic::TextSpan::at(1, 1)))
            }
        }

        impl $crate::kernel::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, $crate::kernel::ProtocolError> {
                Ok(::semio_framework_pack_json::to_string(&::semio_framework_pack_json::from_dsl_value(&$crate::value::ToValue::to_value(self))).into_bytes())
            }

            fn decode_op(bytes: &[u8]) -> Result<Self, $crate::kernel::ProtocolError> {
                let parsed = ::semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::kernel::ProtocolError::Malformed { what: $what, offset: 0, detail: error.to_string() })?;
                <Self as $crate::value::FromValue>::from_value(::semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| $crate::kernel::ProtocolError::Malformed { what: $what, offset: 0, detail: error.to_string() })
            }
        }
    };
}

#[cfg(test)]
#[path = "../🧪️tests/🧾️payload-wire/🦀️.rs"]
mod tests;
