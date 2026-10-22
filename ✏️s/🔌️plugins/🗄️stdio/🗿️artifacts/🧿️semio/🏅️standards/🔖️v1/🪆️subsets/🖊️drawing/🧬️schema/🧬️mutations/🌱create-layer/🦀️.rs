//! 🌱️ `create-layer` — brings a new id-keyed `DrawLayer` into existence at a FINAL-state z-order
//! index (`layers` is real id-keyed per `DrawLayer.id`, positioned per `taxonomy.md`'s "create:
//! full initial payload (+ optional index)").

use crate::schema::snapshot::{DrawLayer, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLayer {
    pub index: usize,
    pub layer: DrawLayer,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for CreateLayer
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<CreateLayer> + From<crate::schema::mutations::delete_layer::DeleteLayer>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create layer \"{}\"", self.layer.name), &format!("Ebene \"{}\" erstellen", self.layer.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for CreateLayer
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<CreateLayer> + From<crate::schema::mutations::delete_layer::DeleteLayer>,
{
    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioDrawingSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
