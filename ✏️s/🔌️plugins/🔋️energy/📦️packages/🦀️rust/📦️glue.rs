//! ⚡️ Energy plugin — headless building energy model (BEM) engine: typed Rust API for transient
//! whole-building simulation (EnergyPlus-class predictor-corrector kernel), no IDF/epJSON, templates,
//! scripting, or language bindings. See `AGENTS.md` for the domain overview.
//!
//! WIRING ONLY. Every `pub mod` below points at exactly one taxonomy/module component file with a
//! `#[path]` written in full, relative to the owner root (this file itself lives two levels deeper,
//! in `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out to the owner root) —
//! do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! 🔄️ 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: these 50 mounts moved out of
//! `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/<domain>/` into a plugin-level module,
//! `🔨️modules/⚡️simulation/⚙️engine/<domain>/` — a `💡️inferences` family member must satisfy
//! `Inference<Snapshot>` (total, deterministic, pure over a snapshot); this fallible, on-demand BEM
//! simulation kernel (air/HVAC/plant solves, curve fits, sizing, economics, …) does not, so it was
//! never a legitimate inference. An artifact is a schema + io system, never an engine; a MODULE may
//! still have one (`taxonomyLeafParentDirs` already lists `⚙️engine` globally — see the
//! `🏗️fem`/`✏️s/🔨️modules/🏗️fem/⚙️engine/` precedent under this same ticket). Energy has no document
//! app (see the "Shape note" below), so this is a module engine, not an app engine. Mount NAMEs are
//! unchanged (`crate::air_exchange`, `crate::kernel`, …), only the `#[path]` TARGET moved, so every
//! existing `crate::<domain>::X`-style call site elsewhere in this crate is unaffected — declared flat
//! at the crate root, one file per domain, with the flat `pub use` re-export surface preserved so
//! `crate::props::…`/`crate::units::…`-style internal references and any external
//! `semio_s_plugin_energy::<Type>` usage both keep working unchanged.
//!
//! 🧭️ Shape note: energy is a headless library plugin — no document app, no DSL/pack/spr wire
//! codec of its own, no command surface. There is no app to receive "behaviour", which is why the
//! 50 domain modules below relocated to a plugin-level `🔨️modules/` engine rather than an app engine.

#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;


//#region ⚡️SimulationEngine
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔄️air_exchange/🦀️component.rs"]
pub mod air_exchange;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌬️air_system/🦀️component.rs"]
pub mod air_system;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🫧️airflow_network/🦀️component.rs"]
pub mod airflow_network;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📅️calendar/🦀️component.rs"]
pub mod calendar;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌀️coils/🦀️component.rs"]
pub mod coils;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🛋️comfort/🦀️component.rs"]
pub mod comfort;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🎛️controls/🦀️component.rs"]
pub mod controls;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📉️curves/🦀️component.rs"]
pub mod curves;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌞️daylight/🦀️component.rs"]
pub mod daylight;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚦️dispatch/🦀️component.rs"]
pub mod dispatch;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💰️economics/🦀️component.rs"]
pub mod economics;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🦀️component.rs"]
pub mod electrical;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏢️envelope/🦀️component.rs"]
pub mod envelope;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚨️error/🦀️component.rs"]
pub mod error;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌫️evaporative/🦀️component.rs"]
pub mod evaporative;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🪭️fans/🦀️component.rs"]
pub mod fans;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚠️faults/🦀️component.rs"]
pub mod faults;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🪟️fenestration/🦀️component.rs"]
pub mod fenestration;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📈️gains/🦀️component.rs"]
pub mod gains;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📐️geometry/🦀️component.rs"]
pub mod geometry;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/♻️heat_recovery/🦀️component.rs"]
pub mod heat_recovery;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💦️humidity_eq/🦀️component.rs"]
pub mod humidity_eq;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🕸️hvac_topo/🦀️component.rs"]
pub mod hvac_topo;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🫁️iaq/🦀️component.rs"]
pub mod iaq;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💯️ideal_hvac/🦀️component.rs"]
pub mod ideal_hvac;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️component.rs"]
pub mod kernel;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧱️material/🦀️component.rs"]
pub mod material;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧮️meters/🦀️component.rs"]
pub mod meters;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📊️metrics/🦀️component.rs"]
pub mod metrics;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️component.rs"]
pub mod model;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔢️num/🦀️component.rs"]
pub mod num;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📤️output/🦀️component.rs"]
pub mod output;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏭️plant/🦀️component.rs"]
pub mod plant;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️component.rs"]
pub mod precompute;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧰️props/🦀️component.rs"]
pub mod props;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/❄️refrigeration/🦀️component.rs"]
pub mod refrigeration;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️component.rs"]
pub mod results;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🛏️room_air/🦀️component.rs"]
pub mod room_air;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🦀️component.rs"]
pub mod schedule;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚿️shw/🦀️component.rs"]
pub mod shw;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs"]
pub mod sim;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️component.rs"]
pub mod site;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📏️sizing/🦀️component.rs"]
pub mod sizing;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/☀️solar/🦀️component.rs"]
pub mod solar;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔥️solar_thermal/🦀️component.rs"]
pub mod solar_thermal;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔚️terminal/🦀️component.rs"]
pub mod terminal;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚖️units/🦀️component.rs"]
pub mod units;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💧️water/🦀️component.rs"]
pub mod water;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏠️zone_air/🦀️component.rs"]
pub mod zone_air;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌡️zone_hvac/🦀️component.rs"]
pub mod zone_hvac;
//#endregion ⚡️SimulationEngine

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

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod entries {
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃entries/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod replace_model {
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::EnergyModelSnapshot;
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::EnergyModelDiff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
                mod tests;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🔖️Plugin
#[path = "../../🦀️component.rs"]
pub mod plugin;
#[path = "../../🎛️apps/🦀️component.rs"]
pub mod plugin_apps;

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_model_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
