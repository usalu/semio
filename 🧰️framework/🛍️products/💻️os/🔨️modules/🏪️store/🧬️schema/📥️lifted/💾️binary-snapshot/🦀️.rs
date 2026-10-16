//! 📥️ Trait impls of `💾️binary-snapshot` whose trait or type belongs to `os-store-space-schema` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🚪️io/💾️binary/📸️snapshot`.
use pack::snapshot::ArtifactPack;
use semio_framework_os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, pack_rt};
use semio_framework_os_pack::pack_rt::renormalize_whole_number_floats;
use semio_framework_value::{FromValue, ToValue};
use crate::space_history_document::SpaceHistorySnapshot;

impl ArtifactPack for SpaceHistorySnapshot {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let value = ToValue::to_value(self);
        Ok(pack_rt::encode_pack_value(&value))
    }
    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        let value = pack_rt::decode_pack_value(bytes)?;
        FromValue::from_value(renormalize_whole_number_floats(value)).map_err(PackError::from)
    }
}
