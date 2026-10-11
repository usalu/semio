//! 🏷️ `set-row-property` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetRowProperty {
    pub(crate) element_name: String,
    pub(crate) row_index: usize,
    pub(crate) property_name: String,
    pub(crate) value: PlyValue,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for SetRowProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "row-property", kind: "set-row-property", record: "SetRowProperty" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        let Self { element_name, row_index, property_name, value } = self;
        protocol::MutationOutcome::new(diff_set_row_property(element_name, *row_index, property_name, value.clone()))
    }
    fn inverse(&self, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
        let Self { element_name, row_index, property_name, .. } = self;
        Ok({
            {
                let prior = base.elements.iter().find(|e| &e.name == element_name).and_then(|el| {
                    let prop_idx = el.properties.iter().position(|p| p.name() == property_name)?;
                    el.rows.get(*row_index)?.values.get(prop_idx).cloned()
                });
                match prior {
                    Some(value) => vec![PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: element_name.clone(), row_index: *row_index, property_name: property_name.clone(), value })],
                    None => Vec::new(),
                }
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set row property", "Zeileneigenschaft setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
