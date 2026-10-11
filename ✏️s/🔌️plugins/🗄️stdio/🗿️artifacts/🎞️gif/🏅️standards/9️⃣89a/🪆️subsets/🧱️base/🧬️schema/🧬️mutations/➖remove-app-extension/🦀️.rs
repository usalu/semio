//! ➖️ `remove-app-extension` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-app-extension")]
pub struct RemoveAppExtension {
    pub(crate) index: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for RemoveAppExtension {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "app-extension", kind: "remove-app-extension", record: "RemoveAppExtension" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(GifDiff { app_extensions: Some(GifAppExtensionsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok(match base.app_extensions.get(*index) {
            Some(ext) => vec![GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: *index, extension: ext.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove app extension", "Anwendungserweiterung entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
