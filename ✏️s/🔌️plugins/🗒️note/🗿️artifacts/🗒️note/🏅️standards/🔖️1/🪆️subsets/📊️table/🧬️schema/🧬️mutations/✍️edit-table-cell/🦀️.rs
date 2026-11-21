//! ✍️ Replaces the content of one addressed table cell.
use crate::schema::diff::{NoteBlockPatch, NoteTableEdit};
use crate::{NoteBlockNode, NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔡️ The table block and zero-based row and column identify the cell.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-table-cell")]
pub struct EditTableCell {
    pub id: String,
    pub row: usize,
    pub column: usize,
    pub content: String,
}

impl EditTableCell {
    fn previous_content<'a>(&self, base: &'a NoteSnapshot) -> Option<&'a str> {
        let NoteBlockNode::Table { columns, rows, .. } = crate::schema::find_block(&base.blocks, &self.id)? else {
            return None;
        };
        columns.get(self.column)?;
        rows.get(self.row)?.get(self.column).map(|cell| cell.content.as_str())
    }
}

impl<Op> MutationKind<NoteSnapshot, Op> for EditTableCell
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<EditTableCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor {
        verb: "edit",
        entity: "table-cell",
        kind: "edit-table-cell",
        record: "EditedTableCell",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Edit table cell", "Tabellenzelle bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for EditTableCell
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<EditTableCell>,
{
    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        let Some(before) = self.previous_content(base) else {
            return protocol::MutationOutcome::error(
                "mutation.target-missing",
                format!("Table {} has no cell at row {}, column {}.", self.id, self.row, self.column),
                [self.id.clone(), self.row.to_string(), self.column.to_string()],
            );
        };
        let edit = NoteTableEdit::SetCell {
            row: self.row,
            column: self.column,
            before: before.to_owned(),
            after: self.content.clone(),
        };
        let patch = NoteBlockPatch { table: Some(vec![edit]), ..Default::default() };
        protocol::MutationOutcome::new(NoteDiff::block_patches([(self.id.clone(), patch)]))
    }

    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(self.previous_content(base).into_iter().map(|content| {
            Op::from(Self { id: self.id.clone(), row: self.row, column: self.column, content: content.to_owned() })
        }).collect())
    }
}
