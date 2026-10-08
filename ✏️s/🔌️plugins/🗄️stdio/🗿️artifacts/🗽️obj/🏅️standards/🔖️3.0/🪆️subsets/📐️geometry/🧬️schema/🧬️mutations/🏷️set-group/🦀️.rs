//! 🏷️ `set-group` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when these fields lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record (`✨️derive/🦀️.rs`'s
//! `dsl_variants_codegen`, "single-field tuple variant" branch), keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-group")]
pub struct SetGroup {
    pub name: String,
    pub faces: Vec<u64>,
    pub index: Option<usize>,
}

impl protocol::MutationKind<ObjSnapshot, ObjMutation> for SetGroup {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "group", kind: "set-group", record: "SetGroup" };

    fn diff(&self, base: &ObjSnapshot) -> protocol::MutationOutcome<<ObjMutation as Mutation<ObjSnapshot>>::Diff> {
        let Self { name, faces, index } = self;
        protocol::MutationOutcome::new({
            let current = base.groups.iter().find(|entry| &entry.name == name);
            match current {
                Some(entry) if entry.faces == *faces => ObjDiff::default(),
                Some(_) => diff_set_group(0, name, faces.clone(), true),
                None => diff_set_group(index.unwrap_or(base.groups.len()).min(base.groups.len()), name, faces.clone(), false),
            }
        })
    }
    fn inverse(&self, base: &ObjSnapshot) -> Result<Vec<ObjMutation>, semio_framework_value::ValueError> {
        let Self { name, .. } = self;
        Ok({
            match base.groups.iter().find(|g| &g.name == name) {
                Some(g) => vec![ObjMutation::SetGroup(set_group::SetGroup { name: name.clone(), faces: g.faces.clone(), index: None })],
                None => vec![ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: name.clone() })],
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set group", "Gruppe setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
