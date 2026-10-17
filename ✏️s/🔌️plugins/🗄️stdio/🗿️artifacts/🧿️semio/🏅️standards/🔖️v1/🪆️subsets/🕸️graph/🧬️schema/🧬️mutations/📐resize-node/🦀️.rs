//! 📐 `resize-node` — a node takes an absolute canvas width and height (taxonomy's `resize` verb: the final size, not a
//! scale factor). Composed editors (dag, reasoning/wires) keep node sizes on the graph's native `width`/`height`.

use crate::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Payload
/// 📐 `resize-node` payload — the node and the size it takes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ResizeNode {
    pub id: GraphNodeId,
    pub width: f64,
    pub height: f64,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for ResizeNode
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<ResizeNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "node", kind: "resize-node", record: "ResizedNode" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let number = |value: f64| {
            let en = format!("{}", (value * 100.0).round() / 100.0);
            let de = en.replace('.', ",");
            (en, de)
        };
        let ((width_en, width_de), (height_en, height_de)) = (number(self.width), number(self.height));
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize node \"{}\" to {width_en} × {height_en}", self.id.value), &format!("Knoten \"{}\" auf {width_de} × {height_de} skalieren", self.id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
