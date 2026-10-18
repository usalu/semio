use super::*;
use semio_framework_os_flow_vcs::forms_bridge;
use canvas::camera::{world_to_screen, Camera, Viewport};
use canvas::Point;
use dag::{computation_node_width, slider_widget_height, HandleRole};
use semio_framework_graph_dsl::{WireEdge, WireNode};
use semio_framework_graph_manifest::PropertyBag;
use neural::{ChannelSpec as InputSpec, OperatorInfo as NeuronKindInfo};
use semio_framework_os_infinite_directed_presentation::DagPreviewContent;
// 🌿️ The flow ARTIFACT crate's own vcs surface — `crate::vcs` glob-imports it privately, so the
// undo/redo law names it at its source.
use semio_framework_artifact_flow_flow::FlowEnvelope;
use std::sync::{Mutex, OnceLock};

include!("🧰️fixture-session-operators-and-registry-host/🦀️.rs");
include!("🔭️kind-infos-and-test-fixture-hosts/🦀️.rs");
include!("🌊️evaluation-and-baseline-laws/🦀️.rs");
include!("🔢️chain-ledger-and-registry-generation-baseline/🦀️.rs");
include!("🔌️connect-ports-geometry-handles-and-slider-drag-laws/🦀️.rs");
include!("🖱️gesture-journal-and-canvas-slider-hits/🦀️.rs");
include!("📷️fixture-round-trip-theme-camera-and-catalogue/🦀️.rs");
include!("🕸️node-graph-surface-and-dag-edges/🦀️.rs");
include!("⏪️undo-redo-and-gesture-journal-laws/🦀️.rs");
include!("🧩️variadic-merge-widget-add-and-wheel-laws/🦀️.rs");
include!("👻️ghost-selection-and-drag-laws/🦀️.rs");
include!("🧲️port-insertion-cluster-and-collapse-laws/🦀️.rs");
include!("🔁️extension-bridge-and-chain-census-laws/🦀️.rs");
