//! 🔍️ `scale-elements` — a relative scale of a set of model elements in place by one factor per axis (ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §17.6, §20.15): every element's BASE scale is multiplied, so editing the
//! factors in history re-derives every scale from whatever base it replays on.

use super::*;

//#region 🔖️Payload
/// 🔍️ `scale-elements` payload — the element ids it scales and the factor per axis each scale is multiplied by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ScaleElements {
    pub targets: Vec<String>,
    pub factors: [f64; 3],
}

impl ScaleElements {
    /// 🟰️ Unit factors scale nothing.
    pub fn identity(&self) -> bool {
        self.factors == [1.0; 3]
    }

    /// 🚧️ Every factor is finite and greater than zero.
    pub fn admissible(&self) -> bool {
        self.factors.iter().all(|factor| factor.is_finite() && *factor > 0.0)
    }
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for ScaleElements {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "elements", kind: "scale-elements", record: "ScaledElements" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
        if !self.admissible() {
            return protocol::MutationOutcome::fatal("mutation.invariant", "scale factors must be finite and greater than 0", self.targets.clone());
        }
        let [fx, fy, fz] = self.factors;
        relative_placement_diff(&self.targets, self.identity(), base, |placement| SemioTransform { scale: SemioPoint3 { x: placement.scale.x * fx, y: placement.scale.y * fy, z: placement.scale.z * fz }, ..placement })
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
        Ok(relative_placement_inverse(&self.targets, self.identity() || !self.admissible(), base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (items_en, items_de) = element_count_label(self.targets.len());
        let (factors_en, factors_de) = vector_label(self.factors);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {items_en} by {factors_en}"), &format!("{items_de} um {factors_de} skalieren"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Payload
