//! 🖌️ `set-style` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStyle {
    pub name: String,
    pub style: DxfStyle,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetStyle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "style", kind: "set-style", record: "SetStyle" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { name, style } = self;
        protocol::MutationOutcome::new({
            let old = base.tables.styles.iter().find(|s| &s.name == name).cloned().unwrap_or_default();
            diff_set_style(name, style_field_changes(&old, style))
        })
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { name, .. } = self;
        Ok({
            match base.tables.styles.iter().find(|s| &s.name == name) {
                Some(s) => vec![DxfMutation::SetStyle(set_style::SetStyle { name: name.clone(), style: s.clone() })],
                None => vec![DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: name.clone() })],
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set style", "Textstil setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
