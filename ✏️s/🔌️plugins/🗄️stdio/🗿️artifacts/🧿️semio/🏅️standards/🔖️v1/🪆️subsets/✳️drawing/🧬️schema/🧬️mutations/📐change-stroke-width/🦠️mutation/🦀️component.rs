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

    async fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Change stroke width of style \"{}\"", self.style_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.style_name.clone()]
    }
}
//#endregion 🔖️Payload
