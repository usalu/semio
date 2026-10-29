//! 🫴️ The native Energy Pack uses the original context-bound Record receiver for its authored intrinsic document.
use crate::original::EnergyModelSnapshot;
use store::os_store::codec_registry::native_snapshot::{OriginalArtifactPackReceiving,OriginalSnapshotRecordReceiving};
impl OriginalArtifactPackReceiving for EnergyModelSnapshot{
 type Receiving=OriginalSnapshotRecordReceiving<Self>;
 fn original_pack_birth_demand()->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Self::Receiving>(),depth:1,..Default::default()}}
 fn admit_original_pack(source:&[u8],limits:protocol::PackLimits,native:&semio_framework_value::NativeDecodeControl<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Self::Receiving,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{Self::Receiving::admit(source,"energy.model",limits,native,grant)}
}
