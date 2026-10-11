//! 📦️ `set-bounds` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 📦️ Sets the X/Y/Z max/min bounding box.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetBounds {
    pub max: (f64, f64, f64),
    pub min: (f64, f64, f64),
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetBounds {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "bounds", kind: "set-bounds", record: "SetBounds" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { max, min } = self;
        protocol::MutationOutcome::new(diff::diff_set_bounds(*max, *min))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        Ok(vec![LasMutation::SetBounds(set_bounds::SetBounds { max: (base.header.max_x, base.header.max_y, base.header.max_z), min: (base.header.min_x, base.header.min_y, base.header.min_z) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set bounds", "Grenzen setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
