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

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;


//#region ⚙️Engine
#[path = "../../⚙️engine/air_exchange/🦀️component.rs"]
pub mod air_exchange;
#[path = "../../⚙️engine/air_system/🦀️component.rs"]
pub mod air_system;
#[path = "../../⚙️engine/airflow_network/🦀️component.rs"]
pub mod airflow_network;
#[path = "../../⚙️engine/calendar/🦀️component.rs"]
pub mod calendar;
#[path = "../../⚙️engine/coils/🦀️component.rs"]
pub mod coils;
#[path = "../../⚙️engine/comfort/🦀️component.rs"]
pub mod comfort;
#[path = "../../⚙️engine/controls/🦀️component.rs"]
pub mod controls;
#[path = "../../⚙️engine/curves/🦀️component.rs"]
pub mod curves;
#[path = "../../⚙️engine/daylight/🦀️component.rs"]
pub mod daylight;
#[path = "../../⚙️engine/dispatch/🦀️component.rs"]
pub mod dispatch;
#[path = "../../⚙️engine/economics/🦀️component.rs"]
pub mod economics;
#[path = "../../⚙️engine/electrical/🦀️component.rs"]
pub mod electrical;
#[path = "../../⚙️engine/envelope/🦀️component.rs"]
pub mod envelope;
#[path = "../../⚙️engine/error/🦀️component.rs"]
pub mod error;
#[path = "../../⚙️engine/evaporative/🦀️component.rs"]
pub mod evaporative;
#[path = "../../⚙️engine/fans/🦀️component.rs"]
pub mod fans;
#[path = "../../⚙️engine/faults/🦀️component.rs"]
pub mod faults;
#[path = "../../⚙️engine/fenestration/🦀️component.rs"]
pub mod fenestration;
#[path = "../../⚙️engine/gains/🦀️component.rs"]
pub mod gains;
#[path = "../../⚙️engine/geometry/🦀️component.rs"]
pub mod geometry;
#[path = "../../⚙️engine/heat_recovery/🦀️component.rs"]
pub mod heat_recovery;
#[path = "../../⚙️engine/humidity_eq/🦀️component.rs"]
pub mod humidity_eq;
#[path = "../../⚙️engine/hvac_topo/🦀️component.rs"]
pub mod hvac_topo;
#[path = "../../⚙️engine/iaq/🦀️component.rs"]
pub mod iaq;
#[path = "../../⚙️engine/ideal_hvac/🦀️component.rs"]
pub mod ideal_hvac;
#[path = "../../⚙️engine/kernel/🦀️component.rs"]
pub mod kernel;
#[path = "../../⚙️engine/material/🦀️component.rs"]
pub mod material;
#[path = "../../⚙️engine/meters/🦀️component.rs"]
pub mod meters;
#[path = "../../⚙️engine/metrics/🦀️component.rs"]
pub mod metrics;
#[path = "../../⚙️engine/model/🦀️component.rs"]
pub mod model;
#[path = "../../⚙️engine/num/🦀️component.rs"]
pub mod num;
#[path = "../../⚙️engine/output/🦀️component.rs"]
pub mod output;
#[path = "../../⚙️engine/plant/🦀️component.rs"]
pub mod plant;
#[path = "../../⚙️engine/precompute/🦀️component.rs"]
pub mod precompute;
#[path = "../../⚙️engine/props/🦀️component.rs"]
pub mod props;
#[path = "../../⚙️engine/refrigeration/🦀️component.rs"]
pub mod refrigeration;
#[path = "../../⚙️engine/results/🦀️component.rs"]
pub mod results;
#[path = "../../⚙️engine/room_air/🦀️component.rs"]
pub mod room_air;
#[path = "../../⚙️engine/schedule/🦀️component.rs"]
pub mod schedule;
#[path = "../../⚙️engine/shw/🦀️component.rs"]
pub mod shw;
#[path = "../../⚙️engine/sim/🦀️component.rs"]
pub mod sim;
#[path = "../../⚙️engine/site/🦀️component.rs"]
pub mod site;
#[path = "../../⚙️engine/sizing/🦀️component.rs"]
pub mod sizing;
#[path = "../../⚙️engine/solar/🦀️component.rs"]
pub mod solar;
#[path = "../../⚙️engine/solar_thermal/🦀️component.rs"]
pub mod solar_thermal;
#[path = "../../⚙️engine/terminal/🦀️component.rs"]
pub mod terminal;
#[path = "../../⚙️engine/units/🦀️component.rs"]
pub mod units;
#[path = "../../⚙️engine/water/🦀️component.rs"]
pub mod water;
#[path = "../../⚙️engine/zone_air/🦀️component.rs"]
pub mod zone_air;
#[path = "../../⚙️engine/zone_hvac/🦀️component.rs"]
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


//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod model {
        #[path = "../../🗿️artifacts/🔋️model/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::model::schema::snapshot::EnergyModelSnapshot;
        pub use crate::artifacts::model::schema::mutations::EnergyModelMutation;
        pub use crate::artifacts::model::schema::diff::EnergyModelDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod no_mutation {
                    #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/🫙no-mutation/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/🫙no-mutation/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔋️model/🧬️schema/🧬️mutations/🫙no-mutation/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::model::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::model::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::model::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::model::schema::diff::*; pub use crate::artifacts::model::schema::diff::text::*; pub mod schema { pub use crate::artifacts::model::schema::diff::*; } pub mod text { pub use crate::artifacts::model::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::model::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::model::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::model::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🔋️model/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🔋️model/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🔋️model/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod export {
                #[path = "."]
                pub mod serializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🔋️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::model::io::export::serializers::artifacts::csv::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::model::io::import::deserializers::artifacts::csv::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::model::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::model::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::model::io::export::serializers::artifacts::xlsx::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::model::io::import::deserializers::artifacts::xlsx::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::model::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::model::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🔋️model/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
pub mod plugin;
#[path = "../../🔌️plugin/🎛️apps/🦀️component.rs"]
pub mod plugin_apps;
#[path = "../../🔌️plugin/🛂️manifest/🦀️component.rs"]
pub mod plugin_manifest;
#[path = "../../🔌️plugin/🔧️setup/🦀️component.rs"]
pub mod plugin_setup;
#[path = "../../🔌️plugin/🎟️capabilities/🦀️component.rs"]
pub mod plugin_capabilities;

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🔋️model/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_model_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
