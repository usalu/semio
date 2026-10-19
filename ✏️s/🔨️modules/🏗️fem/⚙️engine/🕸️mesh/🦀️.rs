//! the incremental, checkpointed `MeshJob`. The deterministic constrained Bowyer-Watson
//! 🕸️ Meshing: 2D constrained Delaunay triangulation with holes (`PlanarDomain` → `TriMesh2`) as a one-shot call and as
//! kernel is first-party; every public type is plain data composed of `f64` and `u32`.

use semio_framework_job::InteractiveJob;

use semio_framework_job::JobOutcomeBorrow;

use semio_framework_job::JobOutcomeDescriptor;

use semio_framework_job::JobOutcomeKind;

use semio_framework_job::JobOutcomeView;

use semio_framework_job::JobPayloadAdmissionFault;

use semio_framework_job::JobPayloadStream;

use semio_framework_job::Operation;

use semio_framework_job::RetainedJobPayload;

use semio_framework_job::RetainedJobPayloadWriter;

use semio_framework_job::StepContext;
use std::collections::BTreeMap;
use semio_framework_job::close_original_retained_step;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_job::RetainedCloneProgress;
use semio_framework_job::RetainedCloneStep;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::RetirementDemand;

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
