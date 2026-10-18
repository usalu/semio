//! 🧬️ Board schema: layout contracts, palette, dag input records, layout inference algorithms and the text io codecs.
//!
//! Crate `semio-framework-os-infinite-board-schema`; a downward producer of the infinite board family.

use std::collections::{BTreeMap, HashMap, HashSet};
use semio_framework_canvas::Circle;
use semio_framework_graph_drawing::force::ForceLayoutOptions as CoreForceLayoutOptions;
use serde::{Deserialize, Serialize};
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, Number, ToValue, ValueError, ValueRefusalKind};
use graph::{EdgeId, HandleId, NodeId};
use semio_framework_pack_json::{JsonGrammarCursor, JsonMemberPolicy, JsonWriteNode, JsonWriteSource};
use semio_framework_value::ordered::{OrderedMap, SharedOwner, UpdateCursor};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence, sequence_birth_bytes};
use geometry::Vec2;
use semio_framework_graph_drawing::force;
use semio_framework_value::retirement::*;

#[path = "📸️board-snapshot-records/🦀️.rs"]
mod board_snapshot_records;
pub use board_snapshot_records::{BoardEdgeSnapshot, BoardHandleSnapshot, BoardNodeShape, BoardNodeSnapshot, BoardRegionSnapshot, BoardSnapshot, BoardSnapshotSchema};

#[path = "🧾️dag-output-sources/🦀️.rs"]
mod dag_output_sources;
pub use dag_output_sources::{absent, channel, decode_board_palette_overlay_json, decode_dag_computing_progress_json, refusal, DagJsonSource, DagOutputKind, DagOutputPreparation, DagPreparedOutput};

#[path = "🔖️dag-selection-json-cursor/🦀️.rs"]
mod dag_selection_json_cursor;
pub use dag_selection_json_cursor::{DagSelectionJsonCursor, DagSelectionTextFault, DagSelectionTextGrant, DagSelectionTextStep, DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES};

#[path = "📡️dag-json-codecs/🦀️.rs"]
mod dag_json_codecs;
pub use dag_json_codecs::{closed, decode_dag_channels_json, decode_dag_hover_json, decode_dag_node_statuses_json, decode_dag_selection_json, encode_dag_channels_json, encode_dag_hover_json, encode_dag_node_ids_json, encode_dag_selection_json};
use dag_json_codecs::{identifier, identifiers, input_channel, refuse};

#[path = "🔌️dag-input-cursor-and-field-helpers/🦀️.rs"]
mod dag_input_cursor_and_field_helpers;
pub use dag_input_cursor_and_field_helpers::{array, field, DagInputCursor, DagInputFacts, DagInputKind};

#[path = "🚧️layout-io-error-and-admissions/🦀️.rs"]
mod layout_io_error_and_admissions;
pub use layout_io_error_and_admissions::{decode_dag_options_json, decode_redraw_options_json, ForceAdmission, LayoutIoError};

#[path = "🖍️selection-and-palette/🦀️.rs"]
mod selection_and_palette;
pub use selection_and_palette::{apply_board_palette_overlay, BoardPalette, BoardPaletteOverlay, Selection};

#[path = "🛂️palette-overlay-value-codec/🦀️.rs"]
mod palette_overlay_value_codec;

#[path = "🪟️dag-views-traits-and-facts/🦀️.rs"]
mod dag_views_traits_and_facts;
pub use dag_views_traits_and_facts::{
    DagChannelDirection, DagChannelRef, DagChannelView, DagComputingProgress, DagHoverFacts, DagNodeEvaluationStatus, DagNodeStatuses, DagOutputCandidates, DagOutputSource, DagRefusalView, DagScreenHit, DagScreenHitJson, DagSelectionDomain,
    DagSelectionDomains, DagSelectionSource, DagWireTypeRefusal,
};

#[path = "🌐️visibility-and-force-layout/🦀️.rs"]
mod visibility_and_force_layout;
pub use visibility_and_force_layout::{apply_force_layout_resolved, board_locked_option, board_visible_option, board_visible_or_true, resolve_node_id_endpoint, BoardVisibility};

#[path = "🌳️buchheim-tree-layout/🦀️.rs"]
mod buchheim_tree_layout;
use buchheim_tree_layout::{half_extent, run_buchheim_layout, TreeDirection};

#[path = "🏗️hierarchical-layout-entry/🦀️.rs"]
mod hierarchical_layout_entry;
pub use hierarchical_layout_entry::{apply_hierarchical_inner, buchheim_positions};

#[path = "🪜️layered-buchheim-layout/🦀️.rs"]
mod layered_buchheim_layout;
pub use layered_buchheim_layout::layered_positions;
use layered_buchheim_layout::{layered_buchheim_first_walk, layered_buchheim_second_walk, layered_buchheim_third_walk, LayeredBuchheimNode};

#[path = "⚙️layout-options/🦀️.rs"]
mod layout_options;
pub(crate) use layout_options::{default_direction, default_tree_layer_spacing, default_tree_sibling_gap};
pub use layout_options::{ForceGraphLayoutOptions, HierarchicalTreeLayoutOptions, RedrawLayoutOptions};

#[path = "📊️layout-progress-error-and-dag-options/🦀️.rs"]
mod layout_progress_error_and_dag_options;
pub use layout_progress_error_and_dag_options::{DagLayoutOptions, DagLayoutOrientation, LayoutControl, LayoutError, LayoutProgress};

#[path = "🏷️edge-tips-and-descriptors/🦀️.rs"]
mod edge_tips_and_descriptors;
pub use edge_tips_and_descriptors::{board_edge_handle_ids, builtin_edge_tips, CameraDescriptor, EdgeTipCatalogEntry, EdgeTipDef, EdgeTipGeometry, NodeDescriptor};
