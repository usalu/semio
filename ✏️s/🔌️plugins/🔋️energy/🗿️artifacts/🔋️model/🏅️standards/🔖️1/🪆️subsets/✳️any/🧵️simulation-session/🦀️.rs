//! ⚡️ Energy simulation tool run (`📋️tool-run-contract.md` §2.4, §3.7): the `ToolRunDefinition` vocabulary
//! (stages of the tier/timestep pipeline, counters, reasons) and the run job that captures the base model
//! in bounded grants, drives the numerical `EnergyJob` inline and publishes progress ticks. One unit of
//! semantic progress is one computed timestep (warmup or run); physical work remains charged to the original caller.
//! The run is read-only: it holds no provisional document ops, so finalize publishes nothing.
//! Source of record: `✏️editor/🧵️simulation-session/🔣️.json`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_results as results;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_plugin_energy_engine_sim as sim;
extern crate semio_s_plugin_energy_engine_sim_retirement as sim_retirement;
extern crate semio_s_plugin_energy_engine_sim_wire as sim_wire;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
#[cfg(test)]
extern crate semio_s_artifact_energy_model_examples as examples;

use crate::kernel::{SimulationConfig};
use crate::model::{Model};
use crate::sim::{EnergyAdmissionRejected, EnergyJob};
use crate::sim_retirement::{EnergyModelCloseCursor};
use crate::sim_wire::{EnergyJobCursor, EnergyJobStage, EnergyNumericalBounds, EnergyQualityTier};
use crate::snapshot::{EnergyModelSnapshot};
use semio_framework_job::RetainedPayloadBuilder;
use semio_framework_job::JobPayloadStagedSource;
use semio_framework_job::JobPayloadAdmissionFault;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::JobOutcomeDescriptor;
use semio_framework_job::JobOutcomeKind;
use semio_framework_job::JobOutcomeView;
use semio_framework_job::JobPayloadStream;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_job::RetainedCloneProgress;
use semio_framework_job::RetainedCloneStep;
use semio_framework_job::RetainedJobPayload;
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunStageDefinition, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunSettingsReads, ToolRunStepRing,
    ToolRunTickWriter, ToolRunTickWireCursor, ToolRunTickPayloadSource, ToolRunTraceKind, ToolRunVerdict,
};
use std::sync::Arc;
use std::mem::{size_of,MaybeUninit};
use semio_framework_value::retirement::controlled::ControlledRetirement;

//#region 🔖️Contract
pub const ENERGY_SIMULATION_RUN_JOB_KIND: &str = "energy.simulation.run";
pub const ENERGY_SIMULATION_RUN_SCHEMA: &str = "energy.simulation.run.v1";
/// 🎚️ The editor config fields a run reads; a publication changing one of them reconfigures a live run.
pub const ENERGY_SIMULATION_RUN_SETTINGS: [&str; 3] = ["/zoneTimestepMinutes", "/systemTimestepMinutes", "/warmupDays"];

/// 🪟️ Window kind ids whose bodies every tick of this run marks dirty (`ToolRunDefinition::windows`
/// → `ToolRunLedger::dirty_scope`). Spelled as literals here rather than imported from
/// `semio_s_artifact_energy_model_editor_edit::windows::*`: this crate is the run contract and
/// the editor builds ON it (the simulation window reads `ENERGY_SIMULATION_RUN_SCHEMA`), so an
/// import the other way would make the layering circular for a pair of string constants. The two
/// `semio_s_artifact_energy_model_editor_edit` simulation window laws assert the simulation literal still equals the
/// window's own kind id and body key.
pub const ENERGY_MODEL_3D_WINDOW_KIND_ID: &str = "energy.model.3d";
pub const ENERGY_SIMULATION_WINDOW_KIND_ID: &str = "energy.simulation";
const MAXIMUM_CAPTURE_ITEMS: usize = 4_194_304;
const MAXIMUM_CAPTURE_BYTES: usize = 512 * 1_024 * 1_024;
const CAPTURE_LAST_LANE: u8 = 47;

/// 🧭️ Stages of the run as the panel names them: the plugin capture, then the numerical pipeline grouped by tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationRunStage {
    Capture,
    Prepare,
    Warmup,
    Run,
    Finalize,
    Encode,
}

impl EnergySimulationRunStage {
    pub const ALL: [Self; 6] = [Self::Capture, Self::Prepare, Self::Warmup, Self::Run, Self::Finalize, Self::Encode];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Prepare => "prepare",
            Self::Warmup => "warmup",
            Self::Run => "run",
            Self::Finalize => "finalize",
            Self::Encode => "encode",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Capture => LocalizedLabel::native("Capturing the model", "Modell wird erfasst"),
            Self::Prepare => LocalizedLabel::native("Preparing weather, geometry and zones", "Wetter, Geometrie und Zonen werden vorbereitet"),
            Self::Warmup => LocalizedLabel::native("Warming up", "Einschwingen"),
            Self::Run => LocalizedLabel::native("Solving timesteps", "Zeitschritte werden gelöst"),
            Self::Finalize => LocalizedLabel::native("Sizing and summarizing", "Auslegung und Zusammenfassung"),
            Self::Encode => LocalizedLabel::native("Encoding the final result", "Endergebnis wird kodiert"),
        }
    }

    /// 🧭️ The run stage a numerical pipeline stage belongs to.
    pub fn of(stage: EnergyJobStage) -> Self {
        match stage {
            EnergyJobStage::Validate | EnergyJobStage::ResolveWeather | EnergyJobStage::Precompute | EnergyJobStage::InitializeZones | EnergyJobStage::InitializeSurfaces | EnergyJobStage::InitializeWarmupHistory => Self::Prepare,
            EnergyJobStage::WarmupTimestep | EnergyJobStage::WarmupConvergence => Self::Warmup,
            EnergyJobStage::StartRun | EnergyJobStage::RunZoneTimestep | EnergyJobStage::AggregateZone | EnergyJobStage::AggregateFacility | EnergyJobStage::PublishTimestep => Self::Run,
            EnergyJobStage::Finalize | EnergyJobStage::Size | EnergyJobStage::FinalizeSummaries | EnergyJobStage::FinalizeMetrics | EnergyJobStage::FinalizeEconomics | EnergyJobStage::BuildResults | EnergyJobStage::PublishFinal => Self::Finalize,
            EnergyJobStage::EncodeOutput | EnergyJobStage::Complete => Self::Encode,
        }
    }
}

/// 🔢️ Counters every progress snapshot carries, in this order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationRunCounter {
    WarmupTimesteps,
    RunTimesteps,
    TiersPublished,
    FacilityElectricityWh,
    InputIdentityBytes,
}

impl EnergySimulationRunCounter {
    pub const ALL: [Self; 5] = [Self::WarmupTimesteps, Self::RunTimesteps, Self::TiersPublished, Self::FacilityElectricityWh, Self::InputIdentityBytes];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::WarmupTimesteps => "warmupTimesteps",
            Self::RunTimesteps => "runTimesteps",
            Self::TiersPublished => "tiersPublished",
            Self::FacilityElectricityWh => "facilityElectricityWh",
            Self::InputIdentityBytes => "energy.input-identity-bytes",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::WarmupTimesteps => LocalizedLabel::native("Warmup timesteps", "Einschwing-Zeitschritte"),
            Self::RunTimesteps => LocalizedLabel::native("Run timesteps", "Lauf-Zeitschritte"),
            Self::TiersPublished => LocalizedLabel::native("Quality tiers published", "Veröffentlichte Qualitätsstufen"),
            Self::FacilityElectricityWh => LocalizedLabel::native("Facility electricity (Wh)", "Anlagenelektrizität (Wh)"),
            Self::InputIdentityBytes => LocalizedLabel::native("Input bytes verified", "Geprüfte Eingabebytes"),
        }
    }
}

/// 🗒️ Step reasons. The four tier reasons carry `[kWh, run timesteps, total run timesteps]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationRunReason {
    SteadyStateEstimate,
    DesignDay,
    CoarseTimestep,
    Final,
    WarmupConverged,
    CaptureRejected,
    AdmissionRejected,
    SimulationFaulted,
}

impl EnergySimulationRunReason {
    pub const ALL: [Self; 8] = [Self::SteadyStateEstimate, Self::DesignDay, Self::CoarseTimestep, Self::Final, Self::WarmupConverged, Self::CaptureRejected, Self::AdmissionRejected, Self::SimulationFaulted];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn from_code(code: u16) -> Option<Self> {
        Self::ALL.get(usize::from(code)).copied()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::SteadyStateEstimate => "steadyStateEstimate",
            Self::DesignDay => "designDay",
            Self::CoarseTimestep => "coarseTimestep",
            Self::Final => "final",
            Self::WarmupConverged => "warmupConverged",
            Self::CaptureRejected => "captureRejected",
            Self::AdmissionRejected => "admissionRejected",
            Self::SimulationFaulted => "simulationFaulted",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::SteadyStateEstimate | Self::DesignDay | Self::CoarseTimestep | Self::Final | Self::WarmupConverged => ToolRunVerdict::Success,
            Self::CaptureRejected | Self::AdmissionRejected | Self::SimulationFaulted => ToolRunVerdict::Danger,
        }
    }

    pub fn template(self) -> LocalizedLabel {
        match self {
            Self::SteadyStateEstimate => LocalizedLabel::native("Steady-state estimate (provisional): {0} kWh facility electricity", "Stationäre Schätzung (vorläufig): {0} kWh Anlagenelektrizität"),
            Self::DesignDay => LocalizedLabel::native("Design day (provisional): {0} kWh after {1} of {2} timesteps", "Auslegungstag (vorläufig): {0} kWh nach {1} von {2} Zeitschritten"),
            Self::CoarseTimestep => LocalizedLabel::native("Coarse timestep (provisional): {0} kWh after {1} of {2} timesteps", "Grober Zeitschritt (vorläufig): {0} kWh nach {1} von {2} Zeitschritten"),
            Self::Final => LocalizedLabel::native("Final: {0} kWh after {1} of {2} timesteps", "Endgültig: {0} kWh nach {1} von {2} Zeitschritten"),
            Self::WarmupConverged => LocalizedLabel::native("Warmup converged after {0} timesteps", "Einschwingen nach {0} Zeitschritten konvergiert"),
            Self::CaptureRejected => LocalizedLabel::native("The model exceeds the capture capacity", "Das Modell überschreitet die Erfassungskapazität"),
            Self::AdmissionRejected => LocalizedLabel::native("The model exceeds the simulation capacity", "Das Modell überschreitet die Simulationskapazität"),
            Self::SimulationFaulted => LocalizedLabel::native("The simulation failed after {0} timesteps", "Die Simulation ist nach {0} Zeitschritten fehlgeschlagen"),
        }
    }

    /// 🏅️ The reason a published quality tier reports under.
    pub fn of_tier(tier: EnergyQualityTier) -> Self {
        match tier {
            EnergyQualityTier::SteadyStateEstimate => Self::SteadyStateEstimate,
            EnergyQualityTier::DesignDay => Self::DesignDay,
            EnergyQualityTier::CoarseTimestep => Self::CoarseTimestep,
            EnergyQualityTier::Final => Self::Final,
        }
    }
}

/// ⏯️ The simulation run declaration: read-only, restarted on any base or settings change, no trace subjects,
/// reading the three run settings of the editor config.
pub fn energy_simulation_run_definition() -> ToolRunDefinition {
    ToolRunDefinition { receiving: semio_framework_tool_run::ToolRunReceivingLimits::REPORTS,
        mutating: false,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Restart,
        unit: LocalizedLabel::native("timesteps", "Zeitschritte"),
        stages: EnergySimulationRunStage::ALL.iter().map(|stage| ToolRunStageDefinition { id: stage.id().into(), label: stage.label() }).collect(),
        counters: EnergySimulationRunCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().into(), label: counter.label() }).collect(),
        reasons: EnergySimulationRunReason::ALL.iter().map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: reason.template() }).collect(),
        trace: ToolRunTraceKind::None,
        run_job: JobKindId::new(ENERGY_SIMULATION_RUN_JOB_KIND),
        revalidate_job: None,
        settings: ToolRunSettingsReads { config: ENERGY_SIMULATION_RUN_SETTINGS.iter().map(|pointer| pointer.to_string()).collect(), window_config: Default::default() },
        windows: vec![ENERGY_MODEL_3D_WINDOW_KIND_ID.into(), ENERGY_SIMULATION_WINDOW_KIND_ID.into()],
        member: None,
    }
}
//#endregion 🔖️Contract

//#region 🧱️SurfacePayload
/// 🧱️ The per-surface energy map a tick carries in `ToolRunTick::payload`, read back by a window
/// through `ToolRunView::payload`. This is the ONLY channel per-surface physics reaches a window on:
/// the framework tool-run driver closes the settled `Complete` candidate's payload pages unread, so
/// the numerical `Results` never leaves the job.
///
/// Wire format, little-endian throughout:
/// `"ESF1"` (4 B) · row count `u32` (4 B) · then one 20-byte row per surface —
/// `id: u32`, `conduction_loss_kwh: f32`, `conduction_gain_kwh: f32`, `solar_transmitted_kwh: f32`,
/// `solar_absorbed_kwh: f32`. `f32` halves the byte cost and is far finer than the colour ramp's 8
/// bands. Hand-rolled rather than `pack::encode_json_value`, which builds a whole `.spk` container
/// for what is a fixed-width array here.
pub const ENERGY_SURFACE_PAYLOAD_MAGIC: [u8; 4] = *b"ESF1";
pub const ENERGY_SURFACE_PAYLOAD_HEADER_BYTES: usize = 8;
pub const ENERGY_SURFACE_PAYLOAD_ROW_BYTES: usize = 20;
/// 🧱️ Hard ceiling on published rows. 8 192 rows = 163 848 B, which leaves ~96 KiB of the
/// `TOOL_RUN_TICK_BYTES_MAX` (262 144 B) tick budget for the progress record and the step ring. A
/// model with more surfaces than this publishes its first 8 192 rows rather than failing the tick.
pub const ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS: usize = 8_192;

/// 🧱️ How many computed timesteps may pass before the map is republished, on top of the tier
/// boundaries and the completing tick.
///
/// 🌱️ Tier boundaries alone are NOT enough and this constant is the fix for why: the first (and for a
/// design-day run the only intermediate) tier fires at `EnergyJobStage::StartRun`, i.e. BEFORE a single
/// run timestep has been integrated, so the map published there is all zeros — every surface equal,
/// every surface on the same colour band, frozen for the rest of the run. That is exactly what the
/// browser timeline recorded (48 vertices in one bucket, stable for the whole run). Republishing every
/// 16 timesteps keeps the map live at ~40 KB per refresh for a 2 000-surface model.
pub const ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL: u64 = 16;

/// 🧱️ Byte length of a payload carrying `rows` rows.
pub const fn energy_surface_payload_bytes(rows: usize) -> usize {
    ENERGY_SURFACE_PAYLOAD_HEADER_BYTES + rows * ENERGY_SURFACE_PAYLOAD_ROW_BYTES
}

/// 🧱️ Encodes the live per-surface table into the tick payload's wire format.
pub fn encode_surface_energy_payload(rows: impl Iterator<Item = crate::results::SurfaceEnergySummary>) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut count: u32 = 0;
    bytes.extend_from_slice(&ENERGY_SURFACE_PAYLOAD_MAGIC);
    bytes.extend_from_slice(&0u32.to_le_bytes());
    for row in rows.take(ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS) {
        bytes.extend_from_slice(&row.id.0.to_le_bytes());
        for value in [row.conduction_loss_kwh, row.conduction_gain_kwh, row.solar_transmitted_kwh, row.solar_absorbed_kwh] {
            bytes.extend_from_slice(&(value as f32).to_le_bytes());
        }
        count += 1;
    }
    bytes[4..8].copy_from_slice(&count.to_le_bytes());
    bytes
}
//#endregion 🧱️SurfacePayload

//#region 🔖️ContractHelpers

/// ⚙️ Folds the captured model's persisted run period and schedule tables into the settings template.
pub fn simulation_config_for(template: &SimulationConfig, model: &Model) -> SimulationConfig {
    SimulationConfig {
        run_period_year: model.run_period.year,
        run_period_start_month: model.run_period.start_month,
        run_period_start_day: model.run_period.start_day,
        run_period_end_month: model.run_period.end_month,
        run_period_end_day: model.run_period.end_day,
        schedules: model.schedules.clone(),
        ..template.clone()
    }
}
//#endregion 🔖️ContractHelpers

#[path = "🧮️retained-input/🦀️.rs"]
mod retained_input;
use retained_input::{CaptureCensus, ModelCapture};

#[path="📮️publication/🦀️.rs"]
mod publication;
use publication::EnergyTickPublication;

#[path = "🧵️run-job/🦀️.rs"]
mod run_job;
use run_job::EnergyRunCursor;
pub use run_job::EnergySimulationRunJob;

//#region 🧪️Laws
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Laws

#[cfg(test)]
#[global_allocator]
static ENERGY_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
