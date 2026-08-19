//! 🚪️ curate -> txt — foreign `Serializer<CurateSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3). See the sibling `Deserializer`'s doc comment: an
//! honest not-yet-implemented stub, `IoFidelity::Lossy`.
use crate::artifacts::curate::CurateSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub async fn serialize(_from: &CurateSnapshot) -> Result<semio_s_plugin_stdio::artifacts::txt::TxtSnapshot, String> {
    Err("txt export not yet implemented".into())
}

pub struct CurateIntoTxt;

impl Serializer<CurateSnapshot> for CurateIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(_from: &CurateSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "CurateIntoTxt: txt export not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
