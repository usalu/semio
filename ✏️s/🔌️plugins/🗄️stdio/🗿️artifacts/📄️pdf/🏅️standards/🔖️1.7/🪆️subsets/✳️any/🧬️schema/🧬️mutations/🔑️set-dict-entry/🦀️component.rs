//! 🔑️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-dict-entry`.

use super::remove_dict_entry::RemoveDictEntry;
use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::{self, PdfDiff, PdfPathSegment}, snapshot::{ObjRef, PdfDictEntry, PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDictEntry {
    pub id: ObjRef,
    pub path: Vec<PdfPathSegment>,
    pub key: String,
    pub value: PdfObject,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetDictEntry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dict-entry", kind: "set-dict-entry", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_dict_entry(base, self.id, &self.path, &self.key, self.value.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        match original_dict_value(base, self.id, &self.path, &self.key) { Some(value) => vec![PdfMutation::SetDictEntry(SetDictEntry { id: self.id, path: self.path.clone(), key: self.key.clone(), value })], None => vec![PdfMutation::RemoveDictEntry(RemoveDictEntry { id: self.id, path: self.path.clone(), key: self.key.clone() })] }
    }

    fn label(&self) -> String {
        format!("Set dictionary entry {}", self.key)
    }

    fn target(&self) -> Vec<String> {
        vec![format!("{} {}", self.id.num, self.id.gen), self.key.clone()]
    }
}

fn original_dict_value(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str) -> Option<PdfObject> {
    let object = base.objects.iter().find(|object| object.id == id)?;
    let mut current = &object.value;
    for segment in path {
        current = match (segment, current) {
            (PdfPathSegment::ArrayIndex { index }, PdfObject::Array(items)) => items.get(*index)?,
            (PdfPathSegment::DictKey { key }, PdfObject::Dict(entries)) => &entries.iter().find(|entry| &entry.key == key)?.value,
            (PdfPathSegment::DictKey { key }, PdfObject::Stream { dict, .. }) => &dict.iter().find(|entry| &entry.key == key)?.value,
            _ => return None,
        };
    }
    let entries: &[PdfDictEntry] = match current {
        PdfObject::Dict(entries) => entries,
        PdfObject::Stream { dict, .. } => dict,
        _ => return None,
    };
    entries.iter().find(|entry| entry.key == key).map(|entry| entry.value.clone())
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<SetDictEntry as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-dict-entry");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
