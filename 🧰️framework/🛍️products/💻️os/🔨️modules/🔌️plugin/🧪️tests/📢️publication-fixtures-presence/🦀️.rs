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

impl crate::store::ArtifactPresenceSnapshot for PublicationPresence {}

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
            let mut recipient = semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
            let mut native = semio_framework_value::NativeDecodeControl::new(1 << 20, &mut callback);
            native.install_retirement_recipient(&mut recipient).unwrap();
            let mut owner = crate::store::NativeSnapshotDecodeOwner::new(&mut native, grant);
            let (value, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| PublicationPresence::decode_presence_native(bytes, &mut owner).unwrap());
            let progress = owner.progress();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            let third: PublicationPresence = serde_json::from_slice(bytes).unwrap();
            assert_eq!(value, third);
            assert_eq!(value.revision, row["revision"].as_u64().unwrap());
            let mut callback = |_| true;
            let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
            let mut native = semio_framework_value::NativeEncodeControl::new(1 << 20, &mut callback);
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
            let mut recipient = semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
            let mut native = semio_framework_value::NativeDecodeControl::new(1 << 20, &mut callback);
            native.install_retirement_recipient(&mut recipient).unwrap();
            let mut owner = crate::store::NativeSnapshotDecodeOwner::new(&mut native, grant);
            assert!(PublicationPresence::decode_presence_native(bytes, &mut owner).is_err());
            assert!(serde_json::from_slice::<PublicationPresence>(bytes).is_err());
        }
        eprintln!("[DEBUG] scalar presence generic JSON native agrees with independent Serde and neutral corpus");
    }
}
