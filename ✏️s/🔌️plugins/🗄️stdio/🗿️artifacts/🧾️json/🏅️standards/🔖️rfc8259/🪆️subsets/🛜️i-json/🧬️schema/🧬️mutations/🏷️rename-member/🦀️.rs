//! 🏷 `rename-member` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameMember {
    pub(crate) path: JsonPath,
    pub(crate) from: String,
    pub(crate) to: String,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for RenameMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "member", kind: "rename-member", record: "RenameMember" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { path, from, to } = self;
        let Some(JsonValue::Object { members }) = resolve(&base.value, path) else {
            return protocol::MutationOutcome::error(CODE_TARGET_MISSING, format!("rename-member: no object at the addressed path, so member {from:?} cannot be renamed"), target_of(path));
        };
        let Some(position) = members.iter().position(|member| &member.key == from) else {
            return protocol::MutationOutcome::error(CODE_TARGET_MISSING, format!("rename-member: the object carries no member named {from:?}"), target_of(path));
        };
        if from == to {
            return protocol::MutationOutcome::new(JsonDiff::default());
        }
        if members.iter().any(|member| &member.key == to) {
            return protocol::MutationOutcome::fatal(
                CODE_INVARIANT,
                format!("rename-member: the object already carries a member named {to:?} -- RFC 7493 §2.3 requires member names to be unique within one object, so this rename would create the duplicate the clause forbids"),
                target_of(path),
            );
        }
        let renamed = JsonObjectDiff { removed: vec![from.clone()], modified: Vec::new(), added: vec![JsonObjectAdded { index: position, key: to.clone(), item: members[position].value.clone() }] };
        protocol::MutationOutcome::new(diff_at_path(path, Some(JsonValueDiff::Object { diff: renamed })))
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        let Self { path, from, to } = self;
        Ok(vec![JsonIJsonMutation::RenameMember(Self { path: path.clone(), from: to.clone(), to: from.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Rename member", "Eigenschaft umbenennen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
