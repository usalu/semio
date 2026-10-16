//! 🚀️ Engine orchestration: Model + SimulationConfig → Results.

extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_error as error;
extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_meters as meters;
extern crate semio_s_plugin_energy_engine_metrics as metrics;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_output as output;
extern crate semio_s_plugin_energy_engine_precompute as precompute;
extern crate semio_s_plugin_energy_engine_results as results;
extern crate semio_s_plugin_energy_engine_site as site;
extern crate semio_s_plugin_energy_engine_sizing as sizing;
extern crate semio_s_plugin_energy_engine_units as units;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_plugin_energy_engine_sim_retirement as retirement;
extern crate semio_s_plugin_energy_engine_sim_wire as wire;

use crate::retirement::{ENERGY_RUNG_DEMAND, close_config_step, close_meter_table_step, close_model_step, close_results_step, close_sizing_step, close_string_step, close_string_vector_step, close_summaries_step, close_time_series_step, config_is_terminal_empty, energy_done, energy_grant_covers, energy_guarded, energy_idle, energy_one, energy_rung, energy_self_funded, energy_turn, model_is_terminal_empty, summaries_are_terminal_empty};
use crate::wire::{ENERGY_WIRE_HEADER_BYTES, ENERGY_WIRE_QUEUE_SLOTS, EnergyCommitReservation, EnergyJobCursor, EnergyJobPreview, EnergyJobStage, EnergyNumericalBounds, EnergyNumericalCensus, EnergyNumericalDimension, EnergyPublicationChannels, EnergyQualityTier, EnergyWireBuild, EnergyWireIdentity, EnergyWireKind, EnergyWireLease, EnergyWirePacket, EnergyWireRejection, close_retained_packet, decode_preview_packet, end_use_code, energy_spend, energy_wire_header, fuel_code, unit_code, weather_copy_target};
use crate::error::Error;
use crate::kernel::{HourWeather, SimulationConfig, SimulationEnvironment, SimulationKernel, SimulationModel, SurfaceState, TimestepBuilder, TimestepWork, WindowState, ZoneState};
use crate::meters::{EndUse, FuelType, Meter, MeterTable};
use crate::metrics::{EmissionFactors, EnvironmentalMetrics, ResilienceMetrics, SourceEnergyFactors};
use crate::model::{FixedTable, Model};
use crate::output::{TimeSeries, TimeSeriesAppendError, TimeSeriesTable};
use crate::precompute::{PrecomputeBuilder, PrecomputedModel};
use crate::results::{Results, RunMetadata, SizingTables, SummaryRow, SummaryTables};
use crate::site::WeatherRecord;
use crate::sizing::{SizingBuilder, SizingConfig};
use crate::units::Unit;
use semio_framework_job::{
    allocate_operation_id, default_now_us, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RevisionId,
    OriginalTurnFailure, StepBudget, StepContext,
};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use std::time::Instant;

#[path = "🧱️state/🦀️.rs"]
mod state;
pub use self::state::*;
#[path = "📮️packets/🦀️.rs"]
mod packets;
#[path = "🔍️pipeline/🦀️.rs"]
mod pipeline;
#[path = "🏃️run/🦀️.rs"]
mod run;
#[path = "🔚️close/🦀️.rs"]
mod close;
use self::close::*;
#[path = "🚀️engine/🦀️.rs"]
mod engine;
pub use self::engine::*;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
