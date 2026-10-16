//! 🔬️ Headless example-geometry lane — every bundled generation3d example's real
//! `🖼️assets/<name>/🗣️.dsl.semio` is parsed into its `FlowHostSnapshot`, evaluated through the same
//! `FlowHost` the editor's `flow-eval-tick` drives with the packaged `brep`/`math` operator sets
//! installed, and the resulting preview handle is tessellated through the same
//! `tessellate_geometry` bridge the preview path calls. Every number the run produces is held to
//! the committed expected-stats fixture beside the example (`🧫️fixtures/🧩️example/🔣️.json`), which
//! the TypeScript twin reads too.
//!
//! ⚖️ `parry3d` recomputes volume, centre of mass and bounding box from the same triangle soup, so
//! no committed expectation rests on our own arithmetic alone.
//!
//! @see ../../../../../../📓️kernel-and-preview-audit-2026-09-09.md — the op chain per example.
//! @see ../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs — `FlowHost::evaluate`.

use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

use semio_framework_os_flow::neural::Registry;
use semio_framework_job::{allocate_operation_id, CancelToken, Generation, StepBudget, StepContext};
use semio_framework_os_flow::{flow_neuron_kind_info_map, install_flow_extension, FlowExtensionSpec, FlowHost, FlowHostRetirement};
use semio_s_artifact_procedural_3d_io_text::snapshot::parse_dsl;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use serde::Deserialize;

include!("📇️fixture/🦀️.rs");
include!("⏱️calibration/🦀️.rs");
include!("🧰️harness/🦀️.rs");
include!("🔬️examples/🦀️.rs");
include!("🌉️scene-bridge/🦀️.rs");
include!("🚚️mesh-delivery/🦀️.rs");

pub(crate) fn geometry_session() -> &'static semio_s_spatial_kernel_semio_session::Session {
    static SESSION: std::sync::OnceLock<semio_s_spatial_kernel_semio_session::Session> = std::sync::OnceLock::new();
    SESSION.get_or_init(semio_s_spatial_kernel_semio_session::Session::new)
}
