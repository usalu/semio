//! 📚️ Energy model examples: the BESTEST cases and the demo as bundled documents.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_s_plugin_energy_engine_bestest as bestest;
extern crate semio_s_plugin_energy_engine_model as model;
#[cfg(test)]
extern crate semio_s_plugin_energy_epw as epw;
#[cfg(test)]
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
#[path = "🏛️bestest-600/🦀️.rs"]
pub mod bestest_600;
#[path = "🏛️bestest-600ff/🦀️.rs"]
pub mod bestest_600ff;
#[path = "🏛️bestest-610/🦀️.rs"]
pub mod bestest_610;
#[path = "🏛️bestest-620/🦀️.rs"]
pub mod bestest_620;
#[path = "🏛️bestest-630/🦀️.rs"]
pub mod bestest_630;
#[path = "🏛️bestest-640/🦀️.rs"]
pub mod bestest_640;
#[path = "🏛️bestest-650/🦀️.rs"]
pub mod bestest_650;
#[path = "🏛️bestest-900/🦀️.rs"]
pub mod bestest_900;
#[path = "🏛️bestest-900ff/🦀️.rs"]
pub mod bestest_900ff;
#[path = "🏛️bestest-910/🦀️.rs"]
pub mod bestest_910;
#[path = "🏛️bestest-920/🦀️.rs"]
pub mod bestest_920;
#[path = "🏛️bestest-930/🦀️.rs"]
pub mod bestest_930;
#[path = "🏛️bestest-940/🦀️.rs"]
pub mod bestest_940;
#[path = "🏛️bestest-950/🦀️.rs"]
pub mod bestest_950;

#[cfg(test)]
#[path = "🧪️tests/🔬️bestest-catalog/🦀️.rs"]
mod bestest_catalog_tests;
