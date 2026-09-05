//! 🧬️ Authoritative change-jfif-header mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
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
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ChangeJfifHeaderMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "jfif-header", kind: "change-jfif-header", record: "ChangeJfifHeader" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { version, density_units, x_density, y_density, thumbnail } = self;
        protocol::MutationOutcome::new(contribute(base, *version, *density_units, *x_density, *y_density, thumbnail.clone()))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let Self { version, density_units, x_density, y_density, thumbnail } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        {
            vec![JpgMutation::ChangeJfifHeader(crate::artifacts::jpg::schema::mutations::ChangeJfifHeaderMutation {
                version: base.jfif_version,
                density_units: base.jfif_density_units,
                x_density: base.jfif_x_density,
                y_density: base.jfif_y_density,
                thumbnail: base.jfif_thumbnail.clone(),
            })]
        }
    }
    fn label(&self) -> String {
        "change jfif header".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-jfif-header".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, version: (u8, u8), density_units: JfifDensityUnits, x_density: u16, y_density: u16, thumbnail: Option<JfifThumbnail>) -> JpgDiff {
    JpgDiff {
        jfif_version: (base.jfif_version != version).then_some(version),
        jfif_density_units: (base.jfif_density_units != density_units).then_some(density_units),
        jfif_x_density: (base.jfif_x_density != x_density).then_some(x_density),
        jfif_y_density: (base.jfif_y_density != y_density).then_some(y_density),
        jfif_thumbnail: (base.jfif_thumbnail != thumbnail).then_some(thumbnail),
        ..Default::default()
    }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    serde_json::from_str(include_str!("🧪️tests/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-jfif-header payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
