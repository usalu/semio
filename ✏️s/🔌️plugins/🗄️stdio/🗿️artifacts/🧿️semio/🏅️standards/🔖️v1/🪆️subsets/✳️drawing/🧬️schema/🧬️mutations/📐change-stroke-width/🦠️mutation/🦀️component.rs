//! 📐️️ `change-stroke-width` — sets one scalar field (`DrawStyle.stroke_width`, name-keyed) on
//! the named style, independently of `change-stroke-color` per SMO's stroke decomposition ruling.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeStrokeWidth {
    pub style_name: String,
    pub new_width: Option<f64>,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for ChangeStrokeWidth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stroke-width", kind: "change-stroke-width", record: "ChangedStrokeWidth" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> <SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change stroke width of style \"{}\"", self.style_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.style_name.clone()]
    }
}
//#endregion 🔖️Payload
