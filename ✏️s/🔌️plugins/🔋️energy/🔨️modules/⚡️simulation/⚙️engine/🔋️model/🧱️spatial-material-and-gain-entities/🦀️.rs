//! 🧱️ The site, zone, surface, fenestration, material, construction and internal gain entities.

use super::*;

// #region 🔖️Site
/// 🌍️ Site location and orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Site {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub time_zone_hours: f64,
    pub north_axis_deg: f64,
}
// #endregion 🔖️Site

// #region 🔖️Zone
/// 🏠️ Thermal zone with volume and conditioning flags.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Zone {
    pub id: EntityId,
    pub name: String,
    pub volume_m3: f64,
    pub multiplier: u32,
    pub conditioned: bool,
    pub part_of_total_floor_area: bool,
}

/// 🪑️ Space within a zone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Space {
    pub id: EntityId,
    pub name: String,
    pub zone_id: EntityId,
    pub floor_area_m2: f64,
}
// #endregion 🔖️Zone

// #region 🔖️Surface
/// 🧱️ Surface boundary type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SurfaceClass {
    ExteriorWall,
    InteriorWall,
    Roof,
    Ceiling,
    Floor,
    Interzone,
    Adiabatic,
    Ground,
}

/// 📐️ Planar polygon surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Surface {
    pub id: EntityId,
    pub name: String,
    pub zone_id: EntityId,
    pub class: SurfaceClass,
    pub vertices_m: Vec<[f64; 3]>,
    pub construction_id: EntityId,
    pub outside_boundary_condition: OutsideBoundary,
    pub sun_exposed: bool,
    pub wind_exposed: bool,
    pub multiplier: u32,
}

/// 🌡️ Exterior boundary condition for surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum OutsideBoundary {
    OutdoorAir,
    Ground,
    OtherSideTemperature,
    Adiabatic,
    Interzone(EntityId),
}

/// 🚧️ Discriminator half of [`OutsideBoundary`]. The union's `Interzone` arm carries the partner
/// surface, and `dsl::DslScalar` binds unit variants only, so a mutation payload names the boundary
/// through this scalar and carries the partner in its own optional `EntityId` field — the same
/// parallel-field shape `replace-airflow-network` uses for its `(zone, node)` pairs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum OutsideBoundaryKind {
    OutdoorAir,
    Ground,
    OtherSideTemperature,
    Adiabatic,
    Interzone,
}

impl OutsideBoundary {
    /// 🚧️ The union's tag, without its payload.
    pub fn kind(self) -> OutsideBoundaryKind {
        match self {
            OutsideBoundary::OutdoorAir => OutsideBoundaryKind::OutdoorAir,
            OutsideBoundary::Ground => OutsideBoundaryKind::Ground,
            OutsideBoundary::OtherSideTemperature => OutsideBoundaryKind::OtherSideTemperature,
            OutsideBoundary::Adiabatic => OutsideBoundaryKind::Adiabatic,
            OutsideBoundary::Interzone(_) => OutsideBoundaryKind::Interzone,
        }
    }

    /// 🔗️ The partner surface an `Interzone` boundary is paired with, `None` for every other arm.
    pub fn interzone_partner(self) -> Option<EntityId> {
        match self {
            OutsideBoundary::Interzone(other) => Some(other),
            _ => None,
        }
    }

    /// 🏗️ Rebuilds the union from its two payload halves; `None` when the halves disagree — an
    /// `Interzone` without a partner, or a partner offered to an arm that has no room for one.
    pub fn from_parts(kind: OutsideBoundaryKind, partner: Option<EntityId>) -> Option<OutsideBoundary> {
        match (kind, partner) {
            (OutsideBoundaryKind::Interzone, Some(other)) => Some(OutsideBoundary::Interzone(other)),
            (OutsideBoundaryKind::Interzone, None) => None,
            (_, Some(_)) => None,
            (OutsideBoundaryKind::OutdoorAir, None) => Some(OutsideBoundary::OutdoorAir),
            (OutsideBoundaryKind::Ground, None) => Some(OutsideBoundary::Ground),
            (OutsideBoundaryKind::OtherSideTemperature, None) => Some(OutsideBoundary::OtherSideTemperature),
            (OutsideBoundaryKind::Adiabatic, None) => Some(OutsideBoundary::Adiabatic),
        }
    }
}

/// 🪟️ Fenestration (window, skylight, door) with its attached solar shading projections.
///
/// `height_m`/`sill_height_m` and the four shading fields are the window-attached analogue of
/// EnergyPlus `Shading:Overhang:Projection` and `Shading:Fin:Projection` — a horizontal projection
/// above the head and two vertical projections beside the jambs, both measured from the glazing
/// plane. They are zero for an unshaded window, which is why every existing model keeps its
/// behaviour. Free-standing site obstructions stay [`ShadingSurface`]'s job.
///
/// `glazing_construction_id` is the optional escape from the single-pane simplification the three
/// scalar optics fields impose: when it names a [`Construction`], that layered stack IS the glazing
/// and `u_value_w_m2k`/`shgc`/`vlt` are the fallback the kernel and every translator use only while
/// it is `None`. Ticket 26/09/06/ENERGY-PLUGIN-END-TO-END measured what the simplification costs —
/// a semio→EnergyPlus translation can only emit `WindowMaterial:SimpleGlazingSystem`, worth +5.7 to
/// +8.1 % of annual cooling on ANSI/ASHRAE 140 cases 600/900 against the standard's own two-pane
/// stack — so the slot exists to carry the real stack once a caller has one.
///
/// 🔶️ How far a [`Fenestration::vertices_m`] corner may sit off its host surface's plane [m]
/// before [`Model::validate`] calls the aperture misplaced. One millimetre — the same order as the
/// coordinates an epJSON document round-trips through.
pub const FENESTRATION_PLANE_TOLERANCE_M: f64 = 1e-3;

/// See ANSI/ASHRAE 140 §5.2 cases 610/630/910/930, whose overhang and fins are exactly this shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Fenestration {
    pub id: EntityId,
    pub name: String,
    pub surface_id: EntityId,
    pub u_value_w_m2k: f64,
    pub shgc: f64,
    pub vlt: f64,
    pub area_m2: f64,
    pub height_m: f64,
    pub sill_height_m: f64,
    pub frame_conductance_w_k: f64,
    pub divider_conductance_w_k: f64,
    pub overhang_depth_m: f64,
    pub overhang_offset_m: f64,
    pub fin_depth_m: f64,
    pub fin_offset_m: f64,
    pub glazing_construction_id: Option<EntityId>,
    /// 🔶️ The aperture's own polygon, in the host [`Surface`]'s plane, world metres, wound the
    /// same way the host is (counter-clockwise seen from outside).
    ///
    /// EMPTY — the zero value, which every document written before ticket
    /// 26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR carries — means "derive the rectangle from
    /// `area_m2`/`height_m`/`sill_height_m` on the host surface exactly as before"
    /// ([`crate::precompute::fenestration_polygon`]). NON-EMPTY means the polygon IS the
    /// aperture: an arbitrary planar ring of three or more vertices, and the three scalars become
    /// display/physics-only (the U-value and SHGC heat balance never needs a shape).
    #[serde(default)]
    #[value(default)]
    pub vertices_m: Vec<[f64; 3]>,
}
// #endregion 🔖️Surface

// #region 🔖️Material
/// 🪨️ Surface roughness class of an exterior face, which scales the forced part of its outside
/// convection (EnergyPlus `Material` roughness keys).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SurfaceRoughness {
    VeryRough,
    Rough,
    MediumRough,
    MediumSmooth,
    Smooth,
    VerySmooth,
}

impl SurfaceRoughness {
    /// 🌬️ Multiplier of the smooth-surface forced convection excess.
    pub fn forced_convection_multiplier(self) -> f64 {
        match self {
            Self::VeryRough => 2.17,
            Self::Rough => 1.67,
            Self::MediumRough => 1.52,
            Self::MediumSmooth => 1.13,
            Self::Smooth => 1.11,
            Self::VerySmooth => 1.0,
        }
    }
}

/// 🧱️ Opaque material layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Material {
    pub id: EntityId,
    pub name: String,
    pub roughness: SurfaceRoughness,
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub density_kg_m3: f64,
    pub specific_heat_j_kg_k: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
}

/// 🪟️ One glass pane of a layered glazing construction, by its normal-incidence spectral averages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct GlazingMaterial {
    pub id: EntityId,
    pub name: String,
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub solar_transmittance: f64,
    pub solar_reflectance_front: f64,
    pub solar_reflectance_back: f64,
    pub visible_transmittance: f64,
    pub visible_reflectance_front: f64,
    pub visible_reflectance_back: f64,
    pub infrared_transmittance: f64,
    pub infrared_emissivity_front: f64,
    pub infrared_emissivity_back: f64,
}

/// 🌫️ Fill gas of a glazing gap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum GasKind {
    Air,
    Argon,
    Krypton,
    Xenon,
}

/// 🌫️ Gas-filled gap between two panes of a layered glazing construction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct GasMaterial {
    pub id: EntityId,
    pub name: String,
    pub thickness_m: f64,
    pub gas: GasKind,
}

/// 🧱️ Layered construction. An opaque construction names [`Material`]s outside first; a glazing
/// construction alternates [`GlazingMaterial`] panes and [`GasMaterial`] gaps, outside pane first.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Construction {
    pub id: EntityId,
    pub name: String,
    pub layer_material_ids: Vec<EntityId>,
}
// #endregion 🔖️Material

// #region 🔖️Gains
/// 👤️ People internal gain object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct PeopleGain {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub activity_schedule_id: ScheduleId,
    pub people_per_area: f64,
    pub sensible_fraction: f64,
    pub latent_fraction: f64,
    pub radiant_fraction: f64,
}

/// 💡️ Lighting internal gain.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct LightingGain {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub visible_fraction: f64,
    pub return_air_fraction: f64,
}

/// 🔌️ Electric equipment gain.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct EquipmentGain {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub latent_fraction: f64,
}
// #endregion 🔖️Gains
