//! 👤️ Block2d mutation — `AddAuthor`: a credited author.

use crate::BlockAuthor;
use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 👤️ `add-author` payload.
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
pub fn add_author(author: BlockAuthor) -> Block2dMutation {
    Block2dMutation::AddAuthor(AddAuthor { author, index: None })
}

/// 📍️ Builder — like [`add_author`] but inserts the row at `index`.
pub fn add_author_at(author: BlockAuthor, index: u32) -> Block2dMutation {
    Block2dMutation::AddAuthor(AddAuthor { author, index: Some(index) })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for AddAuthor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "author", kind: "add-author", record: "AddedAuthor" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
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
