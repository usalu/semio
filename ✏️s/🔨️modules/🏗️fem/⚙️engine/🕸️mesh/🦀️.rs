//! 🕸️ Meshing: 2D constrained Delaunay triangulation with holes (`PlanarDomain` → `TriMesh2`) as a one-shot call and as
//! the incremental, checkpointed `MeshJob`. The deterministic constrained Bowyer-Watson
//! kernel is first-party; every public type is plain data composed of `f64` and `u32`.

use semio_framework_job::{InteractiveJob, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedJobPayload, RetainedJobPayloadWriter, StepContext};
use std::collections::BTreeMap;
use semio_framework_job::{close_original_retained_step, InteractiveJobCloseStep, OriginalTurnFailure, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand};

#[path = "🗺️planar-domain/🦀️.rs"]
mod planar_domain;
pub use planar_domain::*;

#[path = "🔷️triangulation/🦀️.rs"]
mod triangulation;
pub use triangulation::*;

#[path = "🧵️mesh-job/🦀️.rs"]
mod mesh_job;
pub use mesh_job::*;

#[path = "🕹️mesh-job-run/🦀️.rs"]
mod mesh_job_run;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
