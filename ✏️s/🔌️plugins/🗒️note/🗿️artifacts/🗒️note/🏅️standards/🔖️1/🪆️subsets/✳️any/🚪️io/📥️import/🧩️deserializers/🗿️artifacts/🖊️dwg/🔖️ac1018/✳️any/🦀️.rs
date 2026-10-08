//! 🚪️ note <- dwg — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Maps `Line`/`LwPolyline`
//! entities onto ink blocks and `Text` entities onto text blocks — a real, honest domain mapping
//! over typed `DwgGeometry` fields (not hand-rolled byte manipulation), but not full CAD fidelity,
//! so this hop is `IoFidelity::Lossy`.

use crate::NoteSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_dwg::dwg_from_bytes;

pub const DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };

pub struct DwgIntoNote;

impl Deserializer<NoteSnapshot> for DwgIntoNote {
    const FROM: Dialect = DWG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DwgIntoNote: expected a binary dwg payload".to_string())));
        };
        let drawing = dwg_from_bytes(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DwgIntoNote: {error}"))))?;
        let value = crate::standards::v1::subsets::any::io::note_document_json_from_dwg(&drawing).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DwgIntoNote: {error}"))))?;
        let snapshot: NoteSnapshot = semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DwgIntoNote: {error}"))))?;
        Ok(IoOutcome::clean(snapshot))
    }
}
