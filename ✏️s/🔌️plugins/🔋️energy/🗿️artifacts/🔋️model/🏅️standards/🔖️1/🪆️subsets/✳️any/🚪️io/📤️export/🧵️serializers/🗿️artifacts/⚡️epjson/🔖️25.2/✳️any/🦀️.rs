//! 🧾️ model -> epJSON
//!
//! ⚡️ REAL codec, not a bridge: a hand-written EnergyPlus 25.2 `epJSON` writer over
//! [`pack::json`], with no third-party translator in the path. Every document it emits validates
//! against EnergyPlus's own bundled `Energy+.schema.epJSON` (draft-04-shaped JSON Schema) and runs
//! unmodified under `energyplus -a -w <epw> -d <dir> <file.epJSON>` — both are asserted by
//! `🧪️tests/🏛️export-epjson-runs-in-energyplus`, the second (honeybee-free) physics oracle route.
//!
//! Object types written: `Version`, `SimulationControl`, `Building`, `ShadowCalculation`, `Site:Location`,
//! `Site:GroundTemperature:BuildingSurface`, `GlobalGeometryRules`, `Timestep`, `RunPeriod`,
//! `ScheduleTypeLimits`, `Schedule:Constant`, `Schedule:Compact`, `Material`, `Material:NoMass`,
//! `WindowMaterial:Glazing`, `WindowMaterial:Gas`, `WindowMaterial:SimpleGlazingSystem`, `Construction`, `Zone`, `BuildingSurface:Detailed`,
//! `FenestrationSurface:Detailed`, `Shading:Overhang:Projection`, `Shading:Fin:Projection`,
//! `ZoneInfiltration:DesignFlowRate`, `People`, `Lights`, `ElectricEquipment`,
//! `ThermostatSetpoint:DualSetpoint`, `ZoneControl:Thermostat`, `ZoneHVAC:IdealLoadsAirSystem`,
//! `ZoneHVAC:EquipmentList`, `ZoneHVAC:EquipmentConnections`, `NodeList`, `Sizing:Zone`,
//! `Output:Variable`, `Output:Meter`, `Output:SQLite`.
//!
//! # Three conventions this file DECLARES, because the semio schema does not state them
//! 1. **Terrain, solar distribution and shadow updates.** [`Model`] carries none of them.
//!    `Building.terrain` is written as `Country`, `solar_distribution` as `FullInteriorAndExterior`
//!    and `ShadowCalculation` as polygon clipping re-run every day — the ANSI/ASHRAE 140 §5.2
//!    settings of NREL's BESTEST-GSR encoding, and byte-identical to what the honeybee→OpenStudio
//!    route emits, so the two producers stay comparable (`📓️w2-oracle-toolchain.md` §5).
//!    EnergyPlus's own default (every 20 days) moves ASHRAE 140 case 900FF's peak by 0.4 K.
//! 2. **Aperture geometry.** [`Fenestration`] carries an area (plus an optional height/sill) and
//!    no vertices. Windows on one host surface are laid out one per equal horizontal bay, centred,
//!    with `height_m`/`sill_height_m` when non-zero and otherwise a 1.5 width/height aspect at a
//!    0.2 m sill — the same rule the honeybee translator declares, which reproduces ASHRAE 140's
//!    own two 3.0 m × 2.0 m south windows at x 0.5–3.5 / 4.5–7.5 exactly.
//! 3. **First vertex.** `GlobalGeometryRules` is written `UpperLeftCorner` / `Counterclockwise` /
//!    `World`. The model's own winding is already "counter-clockwise seen from OUTSIDE" (see
//!    `⚙️engine/🏛️bestest/🦀️.rs`'s `surfaces()`), which is exactly what `Counterclockwise` means to
//!    EnergyPlus, but a model ring states no first corner — and EnergyPlus reads the first corner:
//!    it takes a surface's height, width and in-plane frame (the frame its windows are placed and
//!    shaded in) from vertices 1–3 under the declared `UpperLeftCorner`. Every
//!    `BuildingSurface:Detailed` and `FenestrationSurface:Detailed` ring is therefore rotated to
//!    start at its upper-left corner seen from outside ([`upper_left_first`]), winding kept. Written
//!    from the lower-left instead, ASHRAE 140 case 600FF peaks 1.3 K below EnergyPlus's own
//!    encoding of the same building.
//!
//! # Two things the model DOES state, written the way EnergyPlus reads them
//! * **Infiltration.** `ScheduledAch` and `PerExteriorArea` are constant design flows in the
//!   engine (`air_exchange::infiltration_flow_m3_s`: schedule × rate; the four coefficient fields
//!   belong to the wind/stack methods). EnergyPlus scales every `ZoneInfiltration:DesignFlowRate`
//!   by `A + B·|ΔT| + C·v + D·v²`, so both are written with [`CONSTANT_DESIGN_FLOW_COEFFICIENTS`] —
//!   copying the model's unused zeros would tell EnergyPlus there is no infiltration at all.
//! * **Glazing.** A [`Fenestration`] whose `glazing_construction_id` names a construction IS that
//!   layered `WindowMaterial:Glazing`/`WindowMaterial:Gas` stack, and its `u_value_w_m2k`/`shgc`/
//!   `vlt` are only the fallback while the id is `None`. Such an aperture references the stack; the
//!   fallback still travels as the window's `WindowMaterial:SimpleGlazingSystem`, so the import leaf
//!   restores the whole fenestration.
//!
//! With these, EnergyPlus run directly on this codec's ASHRAE 140 cases 600/600FF/900/900FF
//! reproduces EnergyPlus run on NREL's own encoding of the standard (honeybee `native`) to the
//! fourth decimal of every judged metric.
//!
//! Anything the model can express but epJSON's covered subset cannot is reported as an
//! [`EpJsonDiagnostic`], never silently dropped.
//!
//! @see https://energyplus.readthedocs.io/en/latest/schema.html
//! @see ../../../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️w6-epjson-io.md
use crate::air_exchange::InfiltrationMethod;
use crate::model::{Construction, EntityId, Fenestration, GasKind, Material, Model, OutsideBoundary, Surface, SurfaceClass};
use crate::schedule::ScheduleId;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_pack_json::{Object, Value};

#[path = "🏷️names/🦀️.rs"]
mod constants_and_names;
pub use constants_and_names::*;

#[path = "📐️geometry/🦀️.rs"]
mod geometry_and_helpers;
pub use geometry_and_helpers::*;

#[path = "🧬️encoder/🦀️.rs"]
mod encoder;
pub use encoder::*;

#[path = "🌉️bridge/🦀️.rs"]
mod leaf_and_bridge;
pub use leaf_and_bridge::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
