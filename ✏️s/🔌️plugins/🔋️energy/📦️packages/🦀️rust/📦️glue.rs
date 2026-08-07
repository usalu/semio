//! ⚡️ Energy plugin — headless building energy model (BEM) engine: typed Rust API for transient
//! whole-building simulation (EnergyPlus-class predictor-corrector kernel), no IDF/epJSON, templates,
//! scripting, or language bindings. See `AGENTS.md` for the domain overview.
//!
//! WIRING ONLY. Every `pub mod` below points at exactly one `⚙️engine/🦀️<domain>.rs` taxonomy
//! component file with a `#[path]` written in full, relative to the owner root (this file itself now
//! lives two levels deeper, in `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out
//! to the owner root) — do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! 🧭️ Shape note (read before "fixing" the missing `🗿️artifacts`/`🎛️apps`): unlike the play-app
//! plugins this taxonomy was designed around, energy has no document/projection, no DSL/pack/spr wire
//! codec, and no command surface to decompose — it is a pure computation library consumed by Rust
//! callers, not a WASM component with a `semio_plugin!`-registered app. The 50 domain modules below
//! are therefore declared flat at the crate root (exactly as the pre-migration bundle crate declared
//! them), just relocated from one 11,663-line file into one file per domain under `⚙️engine/`, with the
//! flat `pub use` re-export surface preserved so `crate::props::…`/`crate::units::…`-style internal
//! references and any external `semio_s_plugin_energy::<Type>` usage both keep working unchanged.

#![allow(clippy::too_many_arguments)]

//#region ⚙️Engine
#[path = "../../⚙️engine/🦀️air_exchange.rs"]
pub mod air_exchange;
#[path = "../../⚙️engine/🦀️air_system.rs"]
pub mod air_system;
#[path = "../../⚙️engine/🦀️airflow_network.rs"]
pub mod airflow_network;
#[path = "../../⚙️engine/🦀️calendar.rs"]
pub mod calendar;
#[path = "../../⚙️engine/🦀️coils.rs"]
pub mod coils;
#[path = "../../⚙️engine/🦀️comfort.rs"]
pub mod comfort;
#[path = "../../⚙️engine/🦀️controls.rs"]
pub mod controls;
#[path = "../../⚙️engine/🦀️curves.rs"]
pub mod curves;
#[path = "../../⚙️engine/🦀️daylight.rs"]
pub mod daylight;
#[path = "../../⚙️engine/🦀️dispatch.rs"]
pub mod dispatch;
#[path = "../../⚙️engine/🦀️economics.rs"]
pub mod economics;
#[path = "../../⚙️engine/🦀️electrical.rs"]
pub mod electrical;
#[path = "../../⚙️engine/🦀️envelope.rs"]
pub mod envelope;
#[path = "../../⚙️engine/🦀️error.rs"]
pub mod error;
#[path = "../../⚙️engine/🦀️evaporative.rs"]
pub mod evaporative;
#[path = "../../⚙️engine/🦀️fans.rs"]
pub mod fans;
#[path = "../../⚙️engine/🦀️faults.rs"]
pub mod faults;
#[path = "../../⚙️engine/🦀️fenestration.rs"]
pub mod fenestration;
#[path = "../../⚙️engine/🦀️gains.rs"]
pub mod gains;
#[path = "../../⚙️engine/🦀️geometry.rs"]
pub mod geometry;
#[path = "../../⚙️engine/🦀️heat_recovery.rs"]
pub mod heat_recovery;
#[path = "../../⚙️engine/🦀️humidity_eq.rs"]
pub mod humidity_eq;
#[path = "../../⚙️engine/🦀️hvac_topo.rs"]
pub mod hvac_topo;
#[path = "../../⚙️engine/🦀️iaq.rs"]
pub mod iaq;
#[path = "../../⚙️engine/🦀️ideal_hvac.rs"]
pub mod ideal_hvac;
#[path = "../../⚙️engine/🦀️kernel.rs"]
pub mod kernel;
#[path = "../../⚙️engine/🦀️material.rs"]
pub mod material;
#[path = "../../⚙️engine/🦀️meters.rs"]
pub mod meters;
#[path = "../../⚙️engine/🦀️metrics.rs"]
pub mod metrics;
#[path = "../../⚙️engine/🦀️model.rs"]
pub mod model;
#[path = "../../⚙️engine/🦀️num.rs"]
pub mod num;
#[path = "../../⚙️engine/🦀️output.rs"]
pub mod output;
#[path = "../../⚙️engine/🦀️plant.rs"]
pub mod plant;
#[path = "../../⚙️engine/🦀️precompute.rs"]
pub mod precompute;
#[path = "../../⚙️engine/🦀️props.rs"]
pub mod props;
#[path = "../../⚙️engine/🦀️refrigeration.rs"]
pub mod refrigeration;
#[path = "../../⚙️engine/🦀️results.rs"]
pub mod results;
#[path = "../../⚙️engine/🦀️room_air.rs"]
pub mod room_air;
#[path = "../../⚙️engine/🦀️schedule.rs"]
pub mod schedule;
#[path = "../../⚙️engine/🦀️shw.rs"]
pub mod shw;
#[path = "../../⚙️engine/🦀️sim.rs"]
pub mod sim;
#[path = "../../⚙️engine/🦀️site.rs"]
pub mod site;
#[path = "../../⚙️engine/🦀️sizing.rs"]
pub mod sizing;
#[path = "../../⚙️engine/🦀️solar.rs"]
pub mod solar;
#[path = "../../⚙️engine/🦀️solar_thermal.rs"]
pub mod solar_thermal;
#[path = "../../⚙️engine/🦀️terminal.rs"]
pub mod terminal;
#[path = "../../⚙️engine/🦀️units.rs"]
pub mod units;
#[path = "../../⚙️engine/🦀️water.rs"]
pub mod water;
#[path = "../../⚙️engine/🦀️zone_air.rs"]
pub mod zone_air;
#[path = "../../⚙️engine/🦀️zone_hvac.rs"]
pub mod zone_hvac;
//#endregion ⚙️Engine

//#region 🔖️FlatReExports
pub use air_exchange::*;
pub use air_system::*;
pub use airflow_network::*;
pub use calendar::*;
pub use coils::*;
pub use comfort::*;
pub use controls::*;
pub use curves::*;
pub use daylight::*;
pub use dispatch::*;
pub use economics::*;
pub use electrical::*;
pub use envelope::*;
pub use error::*;
pub use evaporative::*;
pub use fans::*;
pub use faults::*;
pub use fenestration::*;
pub use gains::*;
pub use geometry::*;
pub use heat_recovery::*;
pub use humidity_eq::*;
pub use hvac_topo::*;
pub use iaq::*;
pub use ideal_hvac::{ideal_loads_deliver, ideal_loads_deliver_with_controls, EconomizerControl, HumidityControl, IdealLoadsConfig, IdealLoadsInput, IdealLoadsOutput, IdealLoadsRequest};
pub use kernel::*;
pub use material::*;
pub use meters::*;
pub use metrics::*;
pub use model::*;
pub use num::*;
pub use output::*;
pub use plant::*;
pub use precompute::*;
pub use props::*;
pub use refrigeration::*;
pub use results::*;
pub use room_air::*;
pub use schedule::*;
pub use shw::*;
pub use sim::*;
pub use site::*;
pub use sizing::*;
pub use solar::*;
pub use solar_thermal::*;
pub use terminal::*;
pub use units::*;
pub use water::*;
pub use zone_air::*;
pub use zone_hvac::*;
//#endregion 🔖️FlatReExports


//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
pub mod plugin;
//#endregion 🔖️Plugin
