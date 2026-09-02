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
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region ⚡️SimulationEngine
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔄️air_exchange/🦀️.rs"]
pub mod air_exchange;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌬️air_system/🦀️.rs"]
pub mod air_system;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🫧️airflow_network/🦀️.rs"]
pub mod airflow_network;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📅️calendar/🦀️.rs"]
pub mod calendar;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌀️coils/🦀️.rs"]
pub mod coils;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🛋️comfort/🦀️.rs"]
pub mod comfort;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🎛️controls/🦀️.rs"]
pub mod controls;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📉️curves/🦀️.rs"]
pub mod curves;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌞️daylight/🦀️.rs"]
pub mod daylight;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚦️dispatch/🦀️.rs"]
pub mod dispatch;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💰️economics/🦀️.rs"]
pub mod economics;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🦀️.rs"]
pub mod electrical;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏢️envelope/🦀️.rs"]
pub mod envelope;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚨️error/🦀️.rs"]
pub mod error;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌫️evaporative/🦀️.rs"]
pub mod evaporative;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🪭️fans/🦀️.rs"]
pub mod fans;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚠️faults/🦀️.rs"]
pub mod faults;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🪟️fenestration/🦀️.rs"]
pub mod fenestration;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📈️gains/🦀️.rs"]
pub mod gains;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📐️geometry/🦀️.rs"]
pub mod geometry;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/♻️heat_recovery/🦀️.rs"]
pub mod heat_recovery;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💦️humidity_eq/🦀️.rs"]
pub mod humidity_eq;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🕸️hvac_topo/🦀️.rs"]
pub mod hvac_topo;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🫁️iaq/🦀️.rs"]
pub mod iaq;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💯️ideal_hvac/🦀️.rs"]
pub mod ideal_hvac;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs"]
pub mod kernel;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧱️material/🦀️.rs"]
pub mod material;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧮️meters/🦀️.rs"]
pub mod meters;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📊️metrics/🦀️.rs"]
pub mod metrics;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs"]
pub mod model;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔢️num/🦀️.rs"]
pub mod num;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📤️output/🦀️.rs"]
pub mod output;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏭️plant/🦀️.rs"]
pub mod plant;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️.rs"]
pub mod precompute;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧰️props/🦀️.rs"]
pub mod props;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/❄️refrigeration/🦀️.rs"]
pub mod refrigeration;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️.rs"]
pub mod results;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🛏️room_air/🦀️.rs"]
pub mod room_air;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🦀️.rs"]
pub mod schedule;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚿️shw/🦀️.rs"]
pub mod shw;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs"]
pub mod sim;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️.rs"]
pub mod site;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📏️sizing/🦀️.rs"]
pub mod sizing;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/☀️solar/🦀️.rs"]
pub mod solar;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔥️solar_thermal/🦀️.rs"]
pub mod solar_thermal;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔚️terminal/🦀️.rs"]
pub mod terminal;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚖️units/🦀️.rs"]
pub mod units;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💧️water/🦀️.rs"]
pub mod water;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏠️zone_air/🦀️.rs"]
pub mod zone_air;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌡️zone_hvac/🦀️.rs"]
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
        #[path = "../../🗿️artifacts/🔋️model/🦀️.rs"]
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
                            #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod entries {
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃entries/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod replace_model {
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/🧪️tests/degrades-an-empty-model-payload-to-a-no-op/🦀️.rs"]
                                    mod tests_degrades_an_empty_model_payload_to_a_no_op;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
        pub mod op {
            pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::EnergyModelDiff;
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::EnergyModelSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
                mod tests;
            }
        }
    }
}
//#endregion 🗿️Artifacts

#[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs"]
pub mod energy_simulation_session;

//#region ✏️👁️Surfaces
// 🎭️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: energy's FIRST authored editor+viewer
// surfaces for `s.energy.model@1/*` — energy had zero document apps, so there was no app tree to
// migrate here (contrast the pilot's `📐️cad`, which moved an existing `🎛️apps/📐️cad/` tree). Two
// independent `#[path = "."]` trees, mirroring `🔖️Artifacts` above: `editor` mounts real
// mutation-capable content, `viewer` mounts an independently-authored read-only twin that never
// imports through `editor` (`policyViewerPurityBreaches`). Facet dirs that hold only
// `📌️.empty.md` (`🎚️config`/`🎮️commands`/`👥️presence`/`🫧️transient` at every surface/mode level) need
// no mount — nothing real lives there yet (`Config`/`Presence`/`Transient` = `NoConfig`/`NoPresence`/
// `NoTransient`).
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod model {
        #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs"]
                    pub mod simulation;
                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs"]
                    pub mod structure;
                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️zones/🦀️.rs"]
                    pub mod zones;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod model {
        #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/⚡️simulation/🦀️.rs"]
                    pub mod simulation;
                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️.rs"]
                    pub mod structure;
                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️zones/🦀️.rs"]
                    pub mod zones;
                }
            }
        }
    }
}
//#endregion ✏️👁️Surfaces

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
pub mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::EnergyApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    pub use crate::artifacts::model::examples::demo as art_model_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
