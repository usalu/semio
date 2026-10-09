//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use crate::RemodelingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{serialize_dsl_txt, ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct RemodelingIntoTxt;

impl Serializer<RemodelingSnapshot> for RemodelingIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &RemodelingSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}
