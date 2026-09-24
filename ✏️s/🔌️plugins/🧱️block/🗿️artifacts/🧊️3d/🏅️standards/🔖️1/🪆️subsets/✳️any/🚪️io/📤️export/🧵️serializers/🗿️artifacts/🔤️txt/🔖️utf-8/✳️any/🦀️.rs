//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use store::ArtifactDsl;
use crate::Block3dSnapshot;
use semio_framework::io::io_mechanism::{serialize_dsl_txt, Serializer};
use semio_framework::io_schema::{Dialect, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct Block3dIntoTxt;

/// 🏗 This subset snapshot as UTF-8 DSL text — also the body the zip container leaf embeds.
pub fn dsl_text(from: &Block3dSnapshot) -> String {
    <Block3dSnapshot as store::ArtifactDsl>::print_dsl(from)
}

impl Serializer<Block3dSnapshot> for Block3dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Block3dSnapshot) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}
