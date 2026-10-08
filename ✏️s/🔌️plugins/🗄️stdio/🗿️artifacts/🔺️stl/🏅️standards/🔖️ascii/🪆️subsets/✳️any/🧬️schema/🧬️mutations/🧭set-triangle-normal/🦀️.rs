//! 🧭️ `set-triangle-normal` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
/// 🧭️ Replaces one triangle's facet normal.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTriangleNormal {
    pub(crate) index: usize,
    pub(crate) normal: [f64; 3],
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for SetTriangleNormal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "triangle-normal", kind: "set-triangle-normal", record: "SetTriangleNormal" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as Mutation<StlSnapshot>>::Diff> {
        let Self { index, normal } = self;
        protocol::MutationOutcome::new(diff::diff_set_triangle_normal(*index, *normal))
    }
    fn inverse(&self, base: &StlSnapshot) -> Result<Vec<StlMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            match base.triangles.get(*index) {
                Some(t) => vec![StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index: *index, normal: t.normal })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set triangle normal", "Normale des Dreiecks setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
