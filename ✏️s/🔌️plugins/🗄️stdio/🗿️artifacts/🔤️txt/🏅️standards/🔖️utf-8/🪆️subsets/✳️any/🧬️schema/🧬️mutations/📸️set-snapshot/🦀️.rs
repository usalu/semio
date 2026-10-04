//! 📸️ Whole-document replacement for natural file ingestion.

use crate::schema::diff::{TxtDiff, TxtLinesDiff};
use crate::schema::mutation_support::native_snapshot_error;
use crate::TxtSnapshot;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetSnapshotMutation {
    pub snapshot: TxtSnapshot,
}

pub type SetSnapshotPayload = SetSnapshotMutation;

pub fn decode_set_snapshot_payload(value: &semio_framework_value::DslValue) -> Result<SetSnapshotPayload, String> {
    let fields = crate::schema::mutation_support::txt_required_object(value, &["snapshot"])?;
    let snapshot = semio_framework_value::FromValue::from_value(fields[0].1.clone()).map_err(|error| error.to_string())?;
    Ok(SetSnapshotPayload { snapshot })
}

impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SetSnapshotMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        if self.snapshot.schema != crate::STDIO_TXT_DOCUMENT_SCHEMA {
            return protocol::MutationOutcome::fatal("mutation.invariant", "replacement snapshot carries another document schema", vec!["schema".to_string()]);
        }
        if let Some(reason) = native_snapshot_error(&self.snapshot) {
            return protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new());
        }
        let lines = TxtLinesDiff::between(&base.lines, &self.snapshot.lines);
        protocol::MutationOutcome::new(TxtDiff {
            trailing_newline: (base.trailing_newline != self.snapshot.trailing_newline).then_some(self.snapshot.trailing_newline),
            line_ending: (base.line_ending != self.snapshot.line_ending).then_some(self.snapshot.line_ending),
            lines: (!lines.is_empty()).then_some(lines),
        })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Result<Vec<super::TxtMutation>, semio_framework_value::ValueError> {
        Ok(vec![super::TxtMutation::SetSnapshot(Self { snapshot: base.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
