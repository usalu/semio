//! 📐️ `set-triangle-vertices` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
/// 📐️ Replaces one triangle's 3 vertices (whole-value replace).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTriangleVertices {
    pub(crate) index: usize,
    pub(crate) vertices: [[f64; 3]; 3],
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for SetTriangleVertices {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "triangle-vertices", kind: "set-triangle-vertices", record: "SetTriangleVertices" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as Mutation<StlSnapshot>>::Diff> {
        let Self { index, vertices } = self;
        protocol::MutationOutcome::new(diff::diff_set_triangle_vertices(*index, *vertices))
    }
    fn inverse(&self, base: &StlSnapshot) -> Result<Vec<StlMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            match base.triangles.get(*index) {
                Some(t) => vec![StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index: *index, vertices: t.vertices })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set triangle vertices", "Eckpunkte des Dreiecks setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
