//! 🚮️ `remove-texcoord` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when these fields lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record (`✨️derive/🦀️.rs`'s
//! `dsl_variants_codegen`, "single-field tuple variant" branch), keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-texcoord")]
pub struct RemoveTexcoord {
    pub index: usize,
}

impl protocol::MutationKind<ObjSnapshot, ObjMutation> for RemoveTexcoord {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "texcoord", kind: "remove-texcoord", record: "RemoveTexcoord" };

    fn diff(&self, base: &ObjSnapshot) -> protocol::MutationOutcome<<ObjMutation as Mutation<ObjSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(diff_remove_texcoord(*index))
    }
    fn inverse(&self, base: &ObjSnapshot) -> Result<Vec<ObjMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.texcoords.get(*index) {
                Some(v) => vec![ObjMutation::InsertTexcoord(insert_tex_coord::InsertTexcoord { index: *index, texcoord: v.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove texcoord", "Texturkoordinate entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
