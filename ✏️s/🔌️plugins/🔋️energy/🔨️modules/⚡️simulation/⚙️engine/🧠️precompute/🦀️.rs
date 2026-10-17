//! 🧮️ Precomputed geometry, finite-difference constructions, glazing optics, shading casters and
//! per-zone radiant enclosures for one simulation run.
//!
//! Everything here is invariant for the whole run and built one model record per step, so a
//! bounded interactive job never blocks on a large model: surfaces in world coordinates (the model's
//! north axis applied), window rectangles placed on their host walls, obstruction polygons with the
//! list of receivers each can shade, the approximate view factors and gray-body exchange factors of
//! every zone enclosure, and the sky-diffuse shading ratios of every sunlit receiver.

extern crate semio_s_plugin_energy_engine_envelope as envelope;
extern crate semio_s_plugin_energy_engine_fenestration as fenestration;
extern crate semio_s_plugin_energy_engine_geometry as geometry;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_num as num;
extern crate semio_s_plugin_energy_engine_solar as solar;
extern crate semio_s_plugin_energy_engine_units as units;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_schedule as schedule;

use crate::envelope::{ConductionLayer, NodeChain};
use crate::fenestration::{rated_coefficient_adjustment, Gap, GlazingSystem, Pane};
use crate::geometry::{polygon_normal, surface_area_m2, surface_tilt_azimuth};
use crate::model::{EntityId, FixedTable, Model, OutsideBoundary, SurfaceClass};
use crate::solar::{dot, sky_diffuse_shading_ratios};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

#[path = "🧾️records-and-precomputed-model/🦀️.rs"]
mod records_and_precomputed_model;
pub use records_and_precomputed_model::*;

#[path = "🏗️precompute-builder/🦀️.rs"]
mod precompute_builder;
pub use precompute_builder::*;

#[path = "🔺️geometry-and-enclosure-factors/🦀️.rs"]
mod geometry_and_enclosure_factors;
pub use geometry_and_enclosure_factors::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
