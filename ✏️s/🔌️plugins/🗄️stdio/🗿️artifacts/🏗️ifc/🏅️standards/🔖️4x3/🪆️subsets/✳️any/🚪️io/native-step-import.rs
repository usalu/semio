//! 📥️ Normal TXT import receives actual source storage and publishes an owned graph through the existing original IFC route.
use super::{IfcSnapshot, TXT_DIALECT, TxtSnapshot, reading};
use semio_framework_os_kernel::{
    ArtifactPackReceiving,
    io::{
        io_mechanism::{Deserializer, IoRunControl},
        io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult},
    },
};
use semio_framework_value::{ValueError, ValueRefusalKind};
/// 🔌️ Only an actual UTF-8 artifact Pack is accepted; its line storage remains in the original receiving frame.
pub struct TxtIntoIfc4x3Native;
impl Deserializer<IfcSnapshot> for TxtIntoIfc4x3Native {
    const FROM: semio_framework_artifact_reference::Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossless;
    async fn deserialize(input: &IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IfcSnapshot> {
        let IoPayload::Binary(bytes) = input else {
            return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue, "STEP import requires the actual native UTF-8 artifact Pack")));
        };
        if bytes.len() > super::MAX_SOURCE_BYTES + 64 {
            return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::WorkLimit, "STEP UTF-8 artifact Pack exceeds source byte admission")));
        }
        control
            .receive_nested::<TxtSnapshot, IoResult<IfcSnapshot>>(|source, original| {
                *source = Some(<TxtSnapshot as ArtifactPackReceiving>::receive_pack(bytes, &mut original.snapshot_decode()?)?);
                Ok(reading::read(source.as_ref().unwrap(), &mut original.snapshot_decode()?).map(|value| IoOutcome { value, diagnostics: Vec::new() }).map_err(|error| IoError::from_value_error(error.cause)))
            })
            .map_err(IoError::from_value_error)?
    }
}
