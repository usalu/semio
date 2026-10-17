//! 🏗️ Typed building energy model entities, validation, and cross-references.

extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_error as error;
extern crate semio_s_plugin_energy_engine_geometry as geometry;
extern crate semio_s_plugin_energy_engine_schedule as schedule;

use crate::error::{Diagnostics, Error, Severity};
use crate::schedule::ScheduleId;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[path = "🏷️ids-and-fixed-table/🦀️.rs"]
mod ids_and_fixed_table;
pub use ids_and_fixed_table::*;

#[path = "🧱️spatial-material-and-gain-entities/🦀️.rs"]
mod spatial_material_and_gain_entities;
pub use spatial_material_and_gain_entities::*;

#[path = "⚙️hvac-entities/🦀️.rs"]
mod hvac_entities;
pub use hvac_entities::*;

#[path = "🧬️model-aggregate-and-fixtures/🦀️.rs"]
mod model_aggregate_and_fixtures;
pub use model_aggregate_and_fixtures::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
