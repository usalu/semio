//! 👤 Block5d mutation — `AddAuthor`: a credited author.

use crate::BlockAuthor;
use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 👤 `add-author` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-author")]
pub struct AddAuthor {
    #[dsl(block)]
    pub author: BlockAuthor,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_author(author: BlockAuthor) -> Block5dMutation {
    Block5dMutation::AddAuthor(AddAuthor { author, index: None })
}

/// 📍️ Builder — like [`add_author`] but inserts the row at `index`.
pub fn add_author_at(author: BlockAuthor, index: u32) -> Block5dMutation {
    Block5dMutation::AddAuthor(AddAuthor { author, index: Some(index) })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for AddAuthor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "author", kind: "add-author", record: "AddedAuthor" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add author \"{}\"", self.author.name), &format!("Autor \"{}\" hinzufügen", self.author.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.author.id.clone()]
    }
}
//#endregion 🔖️Mutation
