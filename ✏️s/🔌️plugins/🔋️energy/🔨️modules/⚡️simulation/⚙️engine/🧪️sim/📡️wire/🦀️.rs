//! 📡️ Energy engine retained wire: checkpoint, preview and commit packets with their lease and lossless queue, the job
//! stage, quality and preview vocabulary, and the numerical admission census they carry.

#[cfg(test)]
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_meters as meters;
extern crate semio_s_plugin_energy_engine_precompute as precompute;
extern crate semio_s_plugin_energy_engine_site as site;
extern crate semio_s_plugin_energy_engine_units as units;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_sim_retirement as retirement;

use crate::retirement::{energy_grant_covers, energy_self_funded, energy_turn};
use crate::kernel::{SimulationConfig, SimulationEnvironment, SimulationKernel, TimestepWork};
use crate::meters::{EndUse, FuelType};
use crate::model::Model;
use crate::precompute::PrecomputedModel;
use crate::site::WeatherRecord;
use crate::units::Unit;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, StepContext};
use semio_framework_job_payload::{RetainedJobPayload};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[path = "📬️packets-and-queues/🦀️.rs"]
mod packets_and_queues;
pub use packets_and_queues::*;

#[path = "🛡️job-vocabulary-and-census/🦀️.rs"]
mod job_vocabulary_and_census;
pub use job_vocabulary_and_census::*;

#[path = "🔎️observation-and-decode/🦀️.rs"]
mod observation_and_decode;
pub use observation_and_decode::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
