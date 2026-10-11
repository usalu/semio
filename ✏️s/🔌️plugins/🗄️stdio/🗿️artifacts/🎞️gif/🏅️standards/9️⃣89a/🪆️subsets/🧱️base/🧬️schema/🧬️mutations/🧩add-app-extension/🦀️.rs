//! 🧩️ `add-app-extension` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "add-app-extension")]
pub struct AddAppExtension {
    pub(crate) index: usize,
    #[dsl(block)]
    pub(crate) extension: GifAppExtension,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for AddAppExtension {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "app-extension", kind: "add-app-extension", record: "AddAppExtension" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index, extension } = self;
        protocol::MutationOutcome::new({
            GifDiff { app_extensions: Some(GifAppExtensionsDiff { added: vec![GifAppExtensionAdded { index: (*index).min(base.app_extensions.len()), extension: extension.clone() }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(vec![GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: (*index).min(base.app_extensions.len()) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add app extension", "Anwendungserweiterung hinzufügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
