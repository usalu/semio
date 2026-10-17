//! 🧪️ Layout interpretation is tested with neutral visual fixtures and the authored native composition.
#[allow(unused_imports)]
use semio_framework_graph_layout_run::{layout_run_definition, layout_run_entity, layout_run_job, layout_run_overlay_positions, LayoutRunConfig, LayoutRunEdge, LayoutRunEncodeError, LayoutRunGraph, LayoutRunNode, LayoutRunOpEncoder, LayoutRunPoint, LayoutRunResume};
#[allow(unused_imports)]
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin, ToolDefinition, ToolRunJob};
#[allow(unused_imports)]
use semio_framework_tool_run::{JobKindId, ToolRunIdentity};
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap};
use crate::editor::jack::modes::edit::tools::reorganize::*;
#[test]fn reorganize_declares_the_content_member(){assert_eq!(LAYOUT_RUN_MEMBER,"content");assert_eq!(layout_config().max_iterations,120);}
