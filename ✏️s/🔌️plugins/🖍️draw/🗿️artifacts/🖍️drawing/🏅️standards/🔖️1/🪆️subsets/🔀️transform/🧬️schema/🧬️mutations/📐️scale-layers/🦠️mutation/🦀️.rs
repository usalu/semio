//! 📐️ Drawing mutation — `ScaleLayers`: a relative, parametric scaling of a set of layers by two axis factors about one
//! world-space pivot — a resize handle's gesture, replayable on any base.
use crate::mutations::{drawing_label_layers, drawing_label_number, DrawingMutation};
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 📐️ `scale-layers` payload — the scaled layer ids, the world-space pivot that stays in place and the axis factors.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-layers")]
pub struct ScaleLayers {
    pub targets: semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_layers(targets: semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>, pivot_x: f64, pivot_y: f64, scale_x: f64, scale_y: f64) -> DrawingMutation {
    DrawingMutation::ScaleLayers(ScaleLayers { targets, pivot_x, pivot_y, scale_x, scale_y })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for ScaleLayers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "layers", kind: "scale-layers", record: "ScaledLayers" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((x_en, x_de), (y_en, y_de)) = (drawing_label_number(self.scale_x), drawing_label_number(self.scale_y));
        let (en, de) = drawing_label_layers(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by ({x_en}, {y_en})"), &format!("{de} um ({x_de}; {y_de}) skalieren"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.iter().map(|target| target.to_string_owner()).collect()
    }
}
//#endregion 🔖️Mutation
