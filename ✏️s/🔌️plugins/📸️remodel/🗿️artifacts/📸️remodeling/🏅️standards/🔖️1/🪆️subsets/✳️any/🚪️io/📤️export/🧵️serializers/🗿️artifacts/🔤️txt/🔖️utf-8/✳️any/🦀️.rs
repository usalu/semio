use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
/// 🎯️ The foreign dialect this leaf writes.
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.txt@utf-8/*` — the txt rendition of a remodeling scene IS
/// its own `.remodeling` DSL text, the exact bytes `📚️examples/**/🗣️.dsl.semio` carry, so this hop is
/// `IoFidelity::Exact`. (The pre-W6 leaf here was a stray copy-paste of stdio's internal json↔txt
/// bridge returning `Err("not yet implemented")`.)
pub struct RemodelingIntoTxt;

impl Serializer<RemodelingSnapshot> for RemodelingIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(<RemodelingSnapshot as store::ArtifactDsl>::print_dsl(from))))
    }
}
