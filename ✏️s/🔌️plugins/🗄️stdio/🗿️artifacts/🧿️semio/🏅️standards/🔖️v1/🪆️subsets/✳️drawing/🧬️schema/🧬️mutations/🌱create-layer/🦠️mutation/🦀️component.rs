//! 🌱️ `create-layer` — brings a new id-keyed `DrawLayer` into existence at a FINAL-state z-order
//! index (`layers` is real id-keyed per `DrawLayer.id`, positioned per `taxonomy.md`'s "create:
//! full initial payload (+ optional index)").

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, SemioDrawingSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateLayer {
    pub index: usize,
    pub layer: DrawLayer,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for CreateLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> <SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create layer \"{}\"", self.layer.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer.id.clone()]
    }
}
//#endregion 🔖️Payload
