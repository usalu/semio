//! 🟫️ Energy model mutation — `CreateSurface`: Adds one planar polygon surface to an existing zone with an existing construction. The exterior boundary arrives as its two halves — a `boundary` discriminator and the `interzoneSurfaceId` only the `Interzone` arm carries — because `dsl::DslScalar` binds unit variants only.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟫️ `create-surface` payload. Adds one planar polygon surface to an existing zone with an existing construction. The exterior boundary arrives as its two halves — a `boundary` discriminator and the `interzoneSurfaceId` only the `Interzone` arm carries — because `dsl::DslScalar` binds unit variants only.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-surface")]
pub struct CreateSurface {
    pub id: crate::model::EntityId,
    pub name: String,
    pub zone_id: crate::model::EntityId,
    pub class: crate::model::SurfaceClass,
    pub vertices_m: Vec<[f64; 3]>,
    pub construction_id: crate::model::EntityId,
    pub boundary: crate::model::OutsideBoundaryKind,
    pub interzone_surface_id: Option<crate::model::EntityId>,
    pub sun_exposed: bool,
    pub wind_exposed: bool,
    pub multiplier: u32,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_surface<Op: From<CreateSurface>>(
    id: crate::model::EntityId,
    name: String,
    zone_id: crate::model::EntityId,
    class: crate::model::SurfaceClass,
    vertices_m: Vec<[f64; 3]>,
    construction_id: crate::model::EntityId,
    boundary: crate::model::OutsideBoundaryKind,
    interzone_surface_id: Option<crate::model::EntityId>,
    sun_exposed: bool,
    wind_exposed: bool,
    multiplier: u32,
    index: Option<u32>,
) -> Op {
    CreateSurface { id, name, zone_id, class, vertices_m, construction_id, boundary, interzone_surface_id, sun_exposed, wind_exposed, multiplier, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateSurface
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateSurface> + From<crate::DeleteSurface>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create surface \"{}\"", self.name), &format!("Oberfläche \"{}\" erstellen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateSurface
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateSurface> + From<crate::DeleteSurface>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "surface", kind: "create-surface", record: "CreatedSurface" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
