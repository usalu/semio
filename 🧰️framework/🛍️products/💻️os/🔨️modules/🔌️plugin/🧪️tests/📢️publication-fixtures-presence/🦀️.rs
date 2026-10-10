//! 👥️ Real shared-ephemeral publication snapshot fixture.

use crate::store::{ArtifactDsl, ArtifactPack, PackDecodeOptions, PackEncodeOptions, PackError, TextError, TextSpan};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicationPresence {
    pub revision: u64,
}

impl ArtifactDsl for PublicationPresence {
    const EXTENSION: &'static str = "publication-presence";

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        serde_json::from_str(text).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string(self).expect("publication presence serializes")
    }
}

impl ArtifactPack for PublicationPresence {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }

    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicationPresenceDiff {
    pub revision: Option<u64>,
}

impl protocol::DiffAlgebra<PublicationPresence> for PublicationPresenceDiff {
    fn inverse(&self, base: &PublicationPresence) -> Self {
        Self { revision: self.revision.map(|_| base.revision) }
    }
    fn is_empty(&self) -> bool {
        self.revision.is_none()
    }
}

impl protocol::MutationDiff<PublicationPresence> for PublicationPresenceDiff {
    fn apply(&self, base: &PublicationPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<PublicationPresence> {
        Ok(PublicationPresence { revision: self.revision.unwrap_or(base.revision) })
    }

    fn absorb(&mut self, other: Self) {
        if other.revision.is_some() {
            self.revision = other.revision;
        }
    }
}

#[path = "../../🧪️testing/📢️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

fn original_scalar_refusal() -> semio_framework_value::ValueError { semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "presence requires its one unsigned revision JSON field") }
struct RevisionReader<'a, 'control, 'callback> { bytes: &'a [u8], position: usize, native: &'control mut semio_framework_value::NativeDecodeControl<'callback> }
impl RevisionReader<'_, '_, '_> {
    fn peek(&self) -> Option<u8> { self.bytes.get(self.position).copied() }
    fn byte(&mut self) -> Result<u8, semio_framework_value::ValueError> {
        let value = self.peek().ok_or_else(original_scalar_refusal)?;
        self.position += 1;
        self.native.advance(1)?;
        Ok(value)
    }
    fn whitespace(&mut self) -> Result<(), semio_framework_value::ValueError> { while self.peek().is_some_and(|byte| matches!(byte, b' ' | b'\n' | b'\r' | b'\t')) { self.byte()?; } Ok(()) }
    fn punctuation(&mut self, expected: u8) -> Result<(), semio_framework_value::ValueError> { self.whitespace()?; if self.byte()? != expected { return Err(original_scalar_refusal()); } Ok(()) }
    fn key(&mut self) -> Result<(), semio_framework_value::ValueError> {
        self.punctuation(b'"')?;
        for expected in b"revision" {
            let mut actual = u16::from(self.byte()?);
            if actual == u16::from(b'\\') {
                if self.byte()? != b'u' { return Err(original_scalar_refusal()); }
                actual = 0;
                for _ in 0..4 {
                    let digit = self.byte()?;
                    let digit = match digit { b'0'..=b'9' => digit - b'0', b'a'..=b'f' => digit - b'a' + 10, b'A'..=b'F' => digit - b'A' + 10, _ => return Err(original_scalar_refusal()) };
                    actual = actual * 16 + u16::from(digit);
                }
            }
            if actual != u16::from(*expected) { return Err(original_scalar_refusal()); }
        }
        if self.byte()? != b'"' { return Err(original_scalar_refusal()); }
        Ok(())
    }
    fn revision(&mut self) -> Result<u64, semio_framework_value::ValueError> {
        self.whitespace()?;
        let first = self.byte()?;
        if !first.is_ascii_digit() { return Err(original_scalar_refusal()); }
        let mut value = u64::from(first - b'0');
        if first == b'0' && self.peek().is_some_and(|byte| byte.is_ascii_digit()) { return Err(original_scalar_refusal()); }
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            let digit = u64::from(self.byte()? - b'0');
            value = value.checked_mul(10).and_then(|value| value.checked_add(digit)).ok_or_else(original_scalar_refusal)?;
        }
        Ok(value)
    }
}
impl crate::store::ArtifactPresenceSnapshot for PublicationPresence {
    fn decode_presence_native(bytes: &[u8], original: &mut crate::store::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, semio_framework_value::ValueError> {
        use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
        let grant = original.remaining_grant();
        if grant.maximum_items == 0 { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "scalar presence requires original work admission")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "scalar presence requires original depth admission")); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<Self>(), ..Default::default() };
        if !progress.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "scalar presence requires original typed copy admission")); }
        let revision = original.native().scoped_stage(|native| {
            native.begin_stage(bytes.len())?;
            let mut reader = RevisionReader { bytes, position: 0, native };
            reader.punctuation(b'{')?; reader.key()?; reader.punctuation(b':')?;
            let revision = reader.revision()?;
            reader.punctuation(b'}')?; reader.whitespace()?;
            if reader.position != bytes.len() { return Err(original_scalar_refusal()); }
            Ok(revision)
        })?;
        original.record_progress(progress)?;
        Ok(Self { revision })
    }
    fn encode_presence_native(&self, original: &mut crate::store::NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, semio_framework_value::ValueError> {
        let mut buffer = [0u8; 33];
        let prefix = b"{\"revision\":";
        buffer[..prefix.len()].copy_from_slice(prefix);
        let mut digits = [0u8; 20];
        let mut cursor = digits.len(); let mut revision = self.revision;
        loop { cursor -= 1; digits[cursor] = b'0' + (revision % 10) as u8; revision /= 10; if revision == 0 { break; } }
        let end = prefix.len() + digits.len() - cursor;
        buffer[prefix.len()..end].copy_from_slice(&digits[cursor..]); buffer[end] = b'}';
        let text = std::str::from_utf8(&buffer[..end + 1]).expect("fixed ASCII revision JSON");
        original.receive::<String, Vec<u8>>(|slot, native, body| {
            *slot = Some(String::new());
            body.copy_encode_text_into(native, text, slot.as_mut().expect("original scalar JSON output"), 1)?;
            Ok(slot.take().expect("original scalar JSON backing").into_bytes())
        })
    }
}

#[cfg(test)]
mod original_native_laws {
    use super::*;
    use crate::store::ArtifactPresenceSnapshot;
    #[test]
    fn scalar_presence_original_native_matches_independent_serde() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️native.json")).unwrap();
        let axes = &fixture["grant"];
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: axes["items"].as_u64().unwrap() as usize, maximum_copy_bytes: axes["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: axes["capacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: axes["releaseBytes"].as_u64().unwrap() as usize, maximum_depth: axes["depth"].as_u64().unwrap() as usize };
        for row in fixture["accepted"].as_array().unwrap() {
            let bytes = row["input"].as_str().unwrap().as_bytes();
            let mut callback = |_| true;
            let mut native = semio_framework_value::NativeDecodeControl::new(8192, &mut callback);
            let mut owner = crate::store::NativeSnapshotDecodeOwner::new(&mut native, grant);
            let (value, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| PublicationPresence::decode_presence_native(bytes, &mut owner).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let third: PublicationPresence = serde_json::from_slice(bytes).unwrap();
            assert_eq!(value, third);
            assert_eq!(value.revision, row["revision"].as_u64().unwrap());
            let mut callback = |_| true;
            let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
            let mut native = semio_framework_value::NativeEncodeControl::new(8192, &mut callback);
            native.install_retirement_recipient(&mut recipient).unwrap();
            let mut owner = crate::store::NativeSnapshotEncodeOwner::new(&mut native, grant);
            let (encoded, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| value.encode_presence_native(&mut owner).unwrap());
            let progress = owner.progress();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            assert_eq!(encoded, serde_json::to_vec(&third).unwrap());
        }
        for row in fixture["refused"].as_array().unwrap() {
            let bytes = row.as_str().unwrap().as_bytes();
            let mut callback = |_| true;
            let mut native = semio_framework_value::NativeDecodeControl::new(8192, &mut callback);
            let mut owner = crate::store::NativeSnapshotDecodeOwner::new(&mut native, grant);
            assert!(PublicationPresence::decode_presence_native(bytes, &mut owner).is_err());
            assert!(serde_json::from_slice::<PublicationPresence>(bytes).is_err());
        }
        eprintln!("[DEBUG] scalar presence original native JSON agrees with independent Serde and neutral corpus");
    }
}
