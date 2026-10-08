//! 🧬️ Authoritative change-jfif-header mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeJfifHeaderMutation {
    pub version: (u8, u8),
    pub density_units: JfifDensityUnits,
    pub x_density: u16,
    pub y_density: u16,
    pub thumbnail: Option<JfifThumbnail>,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ChangeJfifHeaderMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "jfif-header", kind: "change-jfif-header", record: "ChangeJfifHeader" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { version, density_units, x_density, y_density, thumbnail } = self;
        protocol::MutationOutcome::new(contribute(base, *version, *density_units, *x_density, *y_density, thumbnail.clone()))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Result<Vec<JpgMutation>, semio_framework_value::ValueError> {
        let image = &base.image;
        let unchanged = image.jfif_version == self.version && image.jfif_density_units == self.density_units && image.jfif_x_density == self.x_density && image.jfif_y_density == self.y_density && image.jfif_thumbnail == self.thumbnail;
        Ok((!unchanged).then(|| JpgMutation::ChangeJfifHeader(ChangeJfifHeaderMutation { version: image.jfif_version, density_units: image.jfif_density_units, x_density: image.jfif_x_density, y_density: image.jfif_y_density, thumbnail: image.jfif_thumbnail.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change JFIF header", "JFIF-Header ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-jfif-header".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, version: (u8, u8), density_units: JfifDensityUnits, x_density: u16, y_density: u16, thumbnail: Option<JfifThumbnail>) -> JpgDiff {
    JpgDiff {
        jfif_version: (base.image.jfif_version != version).then_some(version),
        jfif_density_units: (base.image.jfif_density_units != density_units).then_some(density_units),
        jfif_x_density: (base.image.jfif_x_density != x_density).then_some(x_density),
        jfif_y_density: (base.image.jfif_y_density != y_density).then_some(y_density),
        jfif_thumbnail: (base.image.jfif_thumbnail != thumbnail).then_some(thumbnail),
        ..Default::default()
    }
}
//#endregion Semantics
