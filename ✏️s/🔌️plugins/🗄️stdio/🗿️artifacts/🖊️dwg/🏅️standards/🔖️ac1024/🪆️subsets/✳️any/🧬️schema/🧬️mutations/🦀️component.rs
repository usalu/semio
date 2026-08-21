//! 🧬️ Logical DWG document mutations.

use crate::artifacts::dwg::schema::diff::{self, DwgDiff};
use crate::artifacts::dwg::DwgSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region Mutations
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DwgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: DwgSnapshot,
    },
    SetVersionInfo {
        version: String,
        maintenance_version: u8,
        codepage: u16,
    },
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_dwg_mutation(snapshot: &mut DwgSnapshot, mutation: &DwgMutation) -> protocol::MutationOutcome<DwgDiff> {
    let outcome = mutation.diff(snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

impl Mutation<DwgSnapshot> for DwgMutation {
    type Diff = DwgDiff;

    async fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            DwgMutation::NoMutation => DwgDiff::default(),
            DwgMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            DwgMutation::SetVersionInfo { version, maintenance_version, codepage } => diff::diff_set_version_info(base, version, *maintenance_version, *codepage),
        }).await
    }

    async fn inverse(&self, base: &DwgSnapshot) -> Vec<Self> {
        match self {
            DwgMutation::NoMutation => vec![DwgMutation::NoMutation],
            DwgMutation::SetSnapshot { .. } => vec![DwgMutation::SetSnapshot { snapshot: base.clone() }],
            DwgMutation::SetVersionInfo { .. } => vec![DwgMutation::SetVersionInfo { version: base.version.clone(), maintenance_version: base.maintenance_version, codepage: base.codepage }],
        }
    }
}
//#endregion Mutations

//#region Codecs
impl OpText for DwgMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline }).await?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record).await;
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }

    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline).await
    }
}

impl OpBinary for DwgMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self).await
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes).await
    }
}
//#endregion Codecs

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DwgMutation> {
    let base = crate::artifacts::dwg::standards::v_ac1024::engine::demo_dwg_snapshot();
    vec![DwgMutation::NoMutation, DwgMutation::SetSnapshot { snapshot: base }, DwgMutation::SetVersionInfo { version: "AC1024".into(), maintenance_version: 9, codepage: 65001 }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    #[semio_framework_async_macros::async_test]
    async fn logical_mutations_obey_diff_and_inverse_laws() {
        let base = DwgSnapshot { version: "AC1024".into(), maintenance_version: 2, codepage: 30, ..Default::default() };
        for mutation in demo_mutation_cases() {
            let mut applied = base.clone();
            let diff = apply_dwg_mutation(&mut applied, &mutation);
            assert_eq!(diff.diff().apply(&base).expect("diff must apply to base"), applied);
            for inverse in mutation.inverse(&base) {
                apply_dwg_mutation(&mut applied, &inverse);
            }
            assert_eq!(applied, base);
        }
        assert!(DwgDiff::between(&base, &base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_codecs_retain_logical_mutations() {
        for mutation in demo_mutation_cases() {
            assert_eq!(DwgMutation::parse_op(&mutation.print_op()).expect("text mutation"), mutation);
            assert_eq!(DwgMutation::decode_op(&mutation.encode_op().expect("binary mutation")).expect("binary mutation"), mutation);
        }
    }
}

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `📦️glue.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/retitles-the-summary-and-records-the-last-editor/🦀️component.rs"]
mod set_snapshot_retitles_the_summary_and_records_the_last_editor;
//#endregion 🧪️FixtureCases
