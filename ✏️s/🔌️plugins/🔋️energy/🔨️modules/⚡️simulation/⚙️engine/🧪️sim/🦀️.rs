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
use semio_framework_job::{allocate_operation_id, default_now_us, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress, RevisionId, OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_job_payload::{JobPayloadStream, RetainedJobPayload};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use std::time::Instant;

#[path = "🪪️input-identity/🦀️.rs"]
pub mod input_identity;
use input_identity::EnergyInputIdentity;
#[path="♻️close/🦀️.rs"]
mod original_terminal_retirement;
use original_terminal_retirement::EnergyAuthorityRetirement;

#[path="🧺️batch/🦀️.rs"]
pub mod batch;
pub use batch::EnergyBatchRun;
pub use semio_framework_job::Operation;
#[cfg(any(test,feature="testing"))]
#[path="🧪️testing/🦀️.rs"]
pub mod testing;

#[path="🎟️admission/🦀️.rs"]
mod admission;
pub use admission::EnergyJobAdmission;

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
#[cfg(any(test, feature = "testkit"))]
pub mod testkit {
    pub use super::{close::*, engine::*, packets::*, pipeline::*, run::*, state::*};
    pub use super::retirement::energy_self_funded;
    pub fn finalization_stage_index(job: &super::EnergyJob) -> usize { super::P7C1_FINALIZATION_STAGES.iter().position(|stage| *stage == job.finalization.stage).unwrap() }
    pub fn finalization_stage_count() -> usize { super::P7C1_FINALIZATION_STAGES.len() }
    pub fn set_finalization_stage_index(job: &mut super::EnergyJob, index: usize) { job.finalization.stage = super::P7C1_FINALIZATION_STAGES[index]; }
    pub fn result_build_stage_index(job: &super::EnergyJob) -> usize { super::P7C1_RESULT_BUILD_STAGES.iter().position(|stage| *stage == job.result_build.stage).unwrap() }
    pub fn result_build_stage_count() -> usize { super::P7C1_RESULT_BUILD_STAGES.len() }
    pub fn set_result_build_stage_index(job: &mut super::EnergyJob, index: usize) { job.result_build.stage = super::P7C1_RESULT_BUILD_STAGES[index]; }
    pub fn warmup_convergence_stage_index(job: &super::EnergyJob) -> usize { super::P7C1_WARMUP_CONVERGENCE_STAGES.iter().position(|stage| *stage == job.warmup_convergence.as_ref().unwrap().stage).unwrap() }
    pub fn warmup_convergence_stage_count() -> usize { super::P7C1_WARMUP_CONVERGENCE_STAGES.len() }
    pub fn set_warmup_convergence_stage_index(job: &mut super::EnergyJob, index: usize) { job.warmup_convergence.as_mut().unwrap().stage = super::P7C1_WARMUP_CONVERGENCE_STAGES[index]; }
    pub fn has_warmup_convergence(job: &super::EnergyJob) -> bool { job.warmup_convergence.is_some() }
    pub fn aggregate_zone_stage_index(job: &super::EnergyJob) -> usize { super::P7C1_AGGREGATE_STAGES.iter().position(|stage| *stage == job.aggregate_zone_work.as_ref().unwrap().stage).unwrap() }
    pub fn aggregate_zone_stage_count() -> usize { super::P7C1_AGGREGATE_STAGES.len() }
    pub fn set_aggregate_zone_stage_index(job: &mut super::EnergyJob, index: usize) { job.aggregate_zone_work.as_mut().unwrap().stage = super::P7C1_AGGREGATE_STAGES[index]; }
    pub fn has_aggregate_zone(job: &super::EnergyJob) -> bool { job.aggregate_zone_work.is_some() }
    pub fn aggregate_facility_stage_index(job: &super::EnergyJob) -> usize { super::P7C1_AGGREGATE_STAGES.iter().position(|stage| *stage == job.aggregate_facility_work.as_ref().unwrap().stage).unwrap() }
    pub fn aggregate_facility_stage_count() -> usize { super::P7C1_AGGREGATE_STAGES.len() }
    pub fn set_aggregate_facility_stage_index(job: &mut super::EnergyJob, index: usize) { job.aggregate_facility_work.as_mut().unwrap().stage = super::P7C1_AGGREGATE_STAGES[index]; }
    pub fn has_aggregate_facility(job: &super::EnergyJob) -> bool { job.aggregate_facility_work.is_some() }

    pub fn configuration(job: &super::EnergyJob) -> &super::SimulationConfig { &job.config }
    pub fn validation_stage_index(job: &super::EnergyJob) -> usize { super::P7C1_VALIDATION_STAGES.iter().position(|stage| *stage == job.validation.stage).unwrap() }
    pub fn validation_stage_count() -> usize { super::P7C1_VALIDATION_STAGES.len() }
    pub fn set_validation_stage_index(job: &mut super::EnergyJob, index: usize) { job.validation.stage = super::P7C1_VALIDATION_STAGES[index]; }
    pub fn install_precomputed_model(job: &mut super::EnergyJob, model: super::PrecomputedModel) { job.pre = Some(model); }
    pub fn has_precomputed_model(job: &super::EnergyJob) -> bool { job.pre.is_some() }
    pub fn original_close_demands(job: &super::EnergyJob) -> Result<super::RetirementDemand, super::ValueError> { job.original_close_demands() }
    pub fn precompute_close_demands(job: &super::EnergyJob, body: usize) -> Option<Result<super::RetirementDemand, super::ValueError>> { job.precompute_close_demands(body) }
    pub fn original_weather_record(job:&super::EnergyJob,index:usize)->super::WeatherRecord{job.weather_record(index)}
    pub fn weather_index(job: &super::EnergyJob, date: super::calendar::SimDate, hour: u8) -> Option<usize> { job.weather_index(date, hour) }
    pub fn hour_weather(job: &super::EnergyJob, index: usize, repeat_day: bool) -> Option<super::HourWeather> { job.hour_weather(index, repeat_day) }
    pub fn advance_input_identity(job: &mut super::EnergyJob, grant: super::RetainedCloneGrant) -> Result<super::RetainedCloneProgress, super::ValueError> {
        let authority = job.authority.as_mut().unwrap();
        authority.input_identity.advance(&authority.model, &authority.config, grant)
    }

}

#[cfg(test)]
#[global_allocator]
static ENERGY_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
