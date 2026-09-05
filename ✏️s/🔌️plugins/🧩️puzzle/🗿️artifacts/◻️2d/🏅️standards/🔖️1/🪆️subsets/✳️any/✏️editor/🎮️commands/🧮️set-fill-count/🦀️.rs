//! 🖌️ `set-fill-count` command.

use crate::artifacts::puzzle2d::op::Puzzle2dPlaySnapshot;
use crate::editor::puzzle2d::config::Puzzle2dFillLifecycle;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use semio_framework_plugin::kernel::Effect;
use serde_json::Value;
use std::sync::atomic::{AtomicPtr, Ordering};

const FILL_SESSION_CAPACITY: usize = 8;
const FILL_SESSION_LOCKED: *mut FillSessionNode = usize::MAX as *mut FillSessionNode;

/// 🪪️ Owns only the mounted fill continuation authority and its bounded publications.
pub struct Puzzle2dFillActionCtx<'a> {
    pub runtime: &'a mut crate::editor::puzzle2d::config::Puzzle2dFillRuntime,
    pub effects: &'a mut Vec<Effect>,
    pub artifact_mutations: &'a mut Vec<crate::artifacts::puzzle2d::mutations::Puzzle2dMutation>,
    pub operation: Option<semio_framework_plugin::AppOperationContext>,
    pub boundary_fault: &'a mut Option<&'static str>,
}

enum FillTerminal {
    Completed(infinite_canvas::BoardFillResult),
    Cancelled,
    Fault(&'static str),
}

enum FillWork {
    AwaitingSnapshot,
    Session(semio_framework_job::MountedWorkerJobSession<ArtifactBoardFillJob>),
    Rejected(semio_framework_job::WorkerJobSessionAdmissionRejected<ArtifactBoardFillJob>),
    Detached(infinite_canvas::BoardFillJob),
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArtifactFillCaptureStage {
    Nodes,
    Handles,
    Kinds,
    Rules,
    Complete,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArtifactNodeCaptureField {
    Begin,
    Id,
    X,
    Y,
    Scale,
    Shape,
    ExtentX,
    ExtentY,
    Bound0,
    Bound1,
    Bound2,
    Bound3,
    Publish,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArtifactHandleCaptureField {
    Begin,
    Id,
    ScanEdges,
    NodeKind,
    HandleKind,
    WireKind,
    EdgeKind,
    X,
    Y,
    Angle,
    Shape,
    ExtentX,
    ExtentY,
    Radius,
    NodeVisible,
    HandleVisible,
    Visible,
    Connected,
    SlotX,
    SlotY,
    Weight,
    Publish,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArtifactKindCaptureField {
    Begin,
    Id,
    Shape,
    Scale,
    Radius,
    Width,
    Height,
    IconBegin,
    Icon,
    Weight,
    TemplateBegin,
    TemplateHandleKind,
    TemplateWireKind,
    TemplateEdgeKind,
    TemplateAngle,
    TemplateRadius,
    TemplateWeight,
    TemplatePublish,
    Publish,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArtifactRuleCaptureField {
    Begin,
    Source,
    Target,
    Bidirectional,
    Specificity,
    Publish,
}

struct ArtifactFillCaptureCursor {
    stage: ArtifactFillCaptureStage,
    node: usize,
    node_field: ArtifactNodeCaptureField,
    node_x: f64,
    node_y: f64,
    node_scale: f64,
    node_rectangle: bool,
    node_extent_x: f64,
    node_extent_y: f64,
    handle_node: usize,
    handle: usize,
    handle_field: ArtifactHandleCaptureField,
    handle_edge: usize,
    handle_connected: bool,
    handle_x: f64,
    handle_y: f64,
    handle_angle: f64,
    handle_rectangle: bool,
    handle_extent_x: f64,
    handle_extent_y: f64,
    handle_radius: f64,
    handle_node_visible: bool,
    handle_visible: bool,
    kind: usize,
    template: usize,
    kind_field: ArtifactKindCaptureField,
    kind_size: f64,
    rule: usize,
    rule_field: ArtifactRuleCaptureField,
    byte: usize,
}

struct ArtifactBoardFillJob {
    operation: semio_framework_job::Operation,
    render_generation: u64,
    canonical_base_revision: [u8; 32],
    maximum_count: u32,
    suggestion_offset: f64,
    snapshot: Option<store::SnapshotRead<Puzzle2dPlaySnapshot>>,
    snapshot_return: Option<store::SnapshotReadReturn>,
    ingress: Option<infinite_canvas::BoardFillSnapshotIngress>,
    capture: ArtifactFillCaptureCursor,
    inner: Option<infinite_canvas::BoardFillJob>,
    fault: Option<&'static str>,
    closing: bool,
}

impl ArtifactFillCaptureCursor {
    fn new() -> Self {
        Self {
            stage: ArtifactFillCaptureStage::Nodes,
            node: 0,
            node_field: ArtifactNodeCaptureField::Begin,
            node_x: 0.0,
            node_y: 0.0,
            node_scale: 1.0,
            node_rectangle: false,
            node_extent_x: 0.0,
            node_extent_y: 0.0,
            handle_node: 0,
            handle: 0,
            handle_field: ArtifactHandleCaptureField::Begin,
            handle_edge: 0,
            handle_connected: false,
            handle_x: 0.0,
            handle_y: 0.0,
            handle_angle: 0.0,
            handle_rectangle: false,
            handle_extent_x: 0.0,
            handle_extent_y: 0.0,
            handle_radius: 0.0,
            handle_node_visible: true,
            handle_visible: true,
            kind: 0,
            template: 0,
            kind_field: ArtifactKindCaptureField::Begin,
            kind_size: 0.0,
            rule: 0,
            rule_field: ArtifactRuleCaptureField::Begin,
            byte: 0,
        }
    }
}

impl ArtifactBoardFillJob {
    fn new(operation: semio_framework_job::Operation, render_generation: u64, canonical_base_revision: [u8; 32], maximum_count: u32, suggestion_offset: f64, snapshot: store::SnapshotRead<Puzzle2dPlaySnapshot>) -> Self {
        Self {
            operation,
            render_generation,
            canonical_base_revision,
            maximum_count,
            suggestion_offset,
            snapshot: Some(snapshot),
            snapshot_return: None,
            ingress: Some(infinite_canvas::BoardFillSnapshotIngress::new(suggestion_offset)),
            capture: ArtifactFillCaptureCursor::new(),
            inner: None,
            fault: None,
            closing: false,
        }
    }

    fn nodes(document: &Value) -> Result<&[Value], &'static str> {
        document.get("nodes").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-nodes")
    }

    fn edges(document: &Value) -> Result<&[Value], &'static str> {
        document.get("edges").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-edges")
    }

    fn node_kinds(document: &Value) -> Result<&[Value], &'static str> {
        document.get("meta").and_then(|meta| meta.get("kindCatalogs")).and_then(|catalogs| catalogs.get("nodes")).and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-node-kinds")
    }

    fn rules(document: &Value) -> Option<&[Value]> {
        document.get("meta").and_then(|meta| meta.get("kindCompatibility")).or_else(|| document.get("kindCompatibility")).and_then(Value::as_array).map(Vec::as_slice)
    }

    fn finite(value: Option<f64>, default: f64) -> f64 {
        value.filter(|value| value.is_finite()).unwrap_or(default)
    }

    fn capture_node_one(&mut self) -> Result<(), &'static str> {
        let document = &self.snapshot.as_ref().ok_or("puzzle2d-fill-snapshot-owner")?.get().0;
        let nodes = Self::nodes(document)?;
        let Some(node) = nodes.get(self.capture.node) else {
            if self.capture.node_field != ArtifactNodeCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-node");
            }
            self.capture.stage = ArtifactFillCaptureStage::Handles;
            return Ok(());
        };
        match self.capture.node_field {
            ArtifactNodeCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_node().map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Id;
            }
            ArtifactNodeCaptureField::Id => {
                let value = node.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-node-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_node_id_byte(byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.node_field = ArtifactNodeCaptureField::X;
                }
            }
            ArtifactNodeCaptureField::X => {
                self.capture.node_x = Self::finite(node.get("x").and_then(Value::as_f64), 0.0);
                self.capture.node_field = ArtifactNodeCaptureField::Y;
            }
            ArtifactNodeCaptureField::Y => {
                self.capture.node_y = Self::finite(node.get("y").and_then(Value::as_f64), 0.0);
                self.capture.node_field = ArtifactNodeCaptureField::Scale;
            }
            ArtifactNodeCaptureField::Scale => {
                self.capture.node_scale = Self::finite(node.get("scale").and_then(Value::as_f64), 1.0).max(f64::EPSILON);
                self.capture.node_field = ArtifactNodeCaptureField::Shape;
            }
            ArtifactNodeCaptureField::Shape => {
                self.capture.node_rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
                self.capture.node_field = ArtifactNodeCaptureField::ExtentX;
            }
            ArtifactNodeCaptureField::ExtentX => {
                let field = if self.capture.node_rectangle { "width" } else { "radius" };
                let extent = Self::finite(node.get(field).and_then(Value::as_f64), 1.0).max(f64::EPSILON) * self.capture.node_scale;
                self.capture.node_extent_x = if self.capture.node_rectangle { extent * 0.5 } else { extent };
                self.capture.node_field = ArtifactNodeCaptureField::ExtentY;
            }
            ArtifactNodeCaptureField::ExtentY => {
                self.capture.node_extent_y = if self.capture.node_rectangle { Self::finite(node.get("height").and_then(Value::as_f64), 1.0).max(f64::EPSILON) * self.capture.node_scale * 0.5 } else { self.capture.node_extent_x };
                self.capture.node_field = ArtifactNodeCaptureField::Bound0;
            }
            ArtifactNodeCaptureField::Bound0 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(0, self.capture.node_x - self.capture.node_extent_x).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Bound1;
            }
            ArtifactNodeCaptureField::Bound1 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(1, self.capture.node_y - self.capture.node_extent_y).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Bound2;
            }
            ArtifactNodeCaptureField::Bound2 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(2, self.capture.node_x + self.capture.node_extent_x).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Bound3;
            }
            ArtifactNodeCaptureField::Bound3 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(3, self.capture.node_y + self.capture.node_extent_y).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Publish;
            }
            ArtifactNodeCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_node().map_err(capture_fault_code)?;
                self.capture.node += 1;
                self.capture.node_field = ArtifactNodeCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_handle_one(&mut self) -> Result<(), &'static str> {
        let document = &self.snapshot.as_ref().ok_or("puzzle2d-fill-snapshot-owner")?.get().0;
        let nodes = Self::nodes(document)?;
        let Some(node) = nodes.get(self.capture.handle_node) else {
            self.capture.stage = ArtifactFillCaptureStage::Kinds;
            return Ok(());
        };
        let handles = node.get("handles").and_then(Value::as_array).ok_or("puzzle2d-fill-capture-handles")?;
        let Some(handle) = handles.get(self.capture.handle) else {
            if self.capture.handle_field != ArtifactHandleCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-handle");
            }
            self.capture.handle_node += 1;
            self.capture.handle = 0;
            self.capture.handle_edge = 0;
            self.capture.handle_connected = false;
            return Ok(());
        };
        match self.capture.handle_field {
            ArtifactHandleCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_handle().map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Id;
            }
            ArtifactHandleCaptureField::Id => {
                let value = handle.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-handle-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_handle_text_byte(infinite_canvas::BoardFillIngressHandleText::Id, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.handle_field = ArtifactHandleCaptureField::ScanEdges;
                }
            }
            ArtifactHandleCaptureField::ScanEdges => {
                let id = handle.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-handle-id")?;
                let edges = Self::edges(document)?;
                if let Some(edge) = edges.get(self.capture.handle_edge) {
                    self.capture.handle_edge += 1;
                    if edge.get("source").and_then(Value::as_str) == Some(id) || edge.get("target").and_then(Value::as_str) == Some(id) {
                        self.capture.handle_connected = true;
                    }
                } else {
                    self.capture.handle_field = ArtifactHandleCaptureField::NodeKind;
                }
            }
            ArtifactHandleCaptureField::NodeKind | ArtifactHandleCaptureField::HandleKind | ArtifactHandleCaptureField::WireKind | ArtifactHandleCaptureField::EdgeKind => {
                let (value, field, next) = match self.capture.handle_field {
                    ArtifactHandleCaptureField::NodeKind => (node.get("nodeKind").and_then(Value::as_str).unwrap_or(""), infinite_canvas::BoardFillIngressHandleText::NodeKind, ArtifactHandleCaptureField::HandleKind),
                    ArtifactHandleCaptureField::HandleKind => (handle.get("handleKind").and_then(Value::as_str).unwrap_or("port"), infinite_canvas::BoardFillIngressHandleText::HandleKind, ArtifactHandleCaptureField::WireKind),
                    ArtifactHandleCaptureField::WireKind => (handle.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), infinite_canvas::BoardFillIngressHandleText::WireKind, ArtifactHandleCaptureField::EdgeKind),
                    ArtifactHandleCaptureField::EdgeKind => (handle.get("edgeKind").and_then(Value::as_str).unwrap_or(""), infinite_canvas::BoardFillIngressHandleText::EdgeKind, ArtifactHandleCaptureField::X),
                    _ => return Err("puzzle2d-fill-capture-handle-field"),
                };
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_handle_text_byte(field, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.handle_field = next;
                }
            }
            ArtifactHandleCaptureField::X => {
                self.capture.handle_x = Self::finite(node.get("x").and_then(Value::as_f64), 0.0);
                self.capture.handle_field = ArtifactHandleCaptureField::Y;
            }
            ArtifactHandleCaptureField::Y => {
                self.capture.handle_y = Self::finite(node.get("y").and_then(Value::as_f64), 0.0);
                self.capture.handle_field = ArtifactHandleCaptureField::Angle;
            }
            ArtifactHandleCaptureField::Angle => {
                self.capture.handle_angle = Self::finite(handle.get("angle").and_then(Value::as_f64), 0.0);
                self.capture.handle_field = ArtifactHandleCaptureField::Shape;
            }
            ArtifactHandleCaptureField::Shape => {
                self.capture.handle_rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
                self.capture.handle_field = ArtifactHandleCaptureField::ExtentX;
            }
            ArtifactHandleCaptureField::ExtentX => {
                let field = if self.capture.handle_rectangle { "width" } else { "radius" };
                self.capture.handle_extent_x = Self::finite(node.get(field).and_then(Value::as_f64), 1.0).max(f64::EPSILON);
                self.capture.handle_field = ArtifactHandleCaptureField::ExtentY;
            }
            ArtifactHandleCaptureField::ExtentY => {
                self.capture.handle_extent_y = if self.capture.handle_rectangle { Self::finite(node.get("height").and_then(Value::as_f64), 1.0).max(f64::EPSILON) } else { self.capture.handle_extent_x };
                self.capture.handle_field = ArtifactHandleCaptureField::Radius;
            }
            ArtifactHandleCaptureField::Radius => {
                self.capture.handle_radius = if self.capture.handle_rectangle { self.capture.handle_extent_x.max(self.capture.handle_extent_y) * 0.5 } else { self.capture.handle_extent_x };
                self.capture.handle_field = ArtifactHandleCaptureField::NodeVisible;
            }
            ArtifactHandleCaptureField::NodeVisible => {
                self.capture.handle_node_visible = node.get("visible").and_then(Value::as_bool) != Some(false);
                self.capture.handle_field = ArtifactHandleCaptureField::HandleVisible;
            }
            ArtifactHandleCaptureField::HandleVisible => {
                self.capture.handle_visible = handle.get("visible").and_then(Value::as_bool) != Some(false);
                self.capture.handle_field = ArtifactHandleCaptureField::Visible;
            }
            ArtifactHandleCaptureField::Visible => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_visible(self.capture.handle_node_visible && self.capture.handle_visible).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Connected;
            }
            ArtifactHandleCaptureField::Connected => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_connected(self.capture.handle_connected).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::SlotX;
            }
            ArtifactHandleCaptureField::SlotX => {
                let distance = self.capture.handle_radius + self.suggestion_offset;
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_slot(0, self.capture.handle_x + self.capture.handle_angle.cos() * distance).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::SlotY;
            }
            ArtifactHandleCaptureField::SlotY => {
                let distance = self.capture.handle_radius + self.suggestion_offset;
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_slot(1, self.capture.handle_y + self.capture.handle_angle.sin() * distance).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Weight;
            }
            ArtifactHandleCaptureField::Weight => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_weight(1.0).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Publish;
            }
            ArtifactHandleCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_handle().map_err(capture_fault_code)?;
                self.capture.handle += 1;
                self.capture.handle_edge = 0;
                self.capture.handle_connected = false;
                self.capture.handle_field = ArtifactHandleCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_kind_one(&mut self) -> Result<(), &'static str> {
        let document = &self.snapshot.as_ref().ok_or("puzzle2d-fill-snapshot-owner")?.get().0;
        let kinds = Self::node_kinds(document)?;
        let Some(kind) = kinds.get(self.capture.kind) else {
            if self.capture.kind_field != ArtifactKindCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-kind");
            }
            self.capture.stage = ArtifactFillCaptureStage::Rules;
            return Ok(());
        };
        let templates = kind.get("handles").and_then(Value::as_array).ok_or("puzzle2d-fill-capture-kind-handles")?;
        let template = templates.get(self.capture.template);
        match self.capture.kind_field {
            ArtifactKindCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind().map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Id;
            }
            ArtifactKindCaptureField::Id => {
                let value = kind.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-kind-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(infinite_canvas::BoardFillIngressKindText::Id, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.kind_field = ArtifactKindCaptureField::Shape;
                }
            }
            ArtifactKindCaptureField::Shape => {
                let rectangle = kind.get("shape").and_then(Value::as_str) == Some("rectangle");
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_rectangle(rectangle).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Scale;
            }
            ArtifactKindCaptureField::Scale => {
                self.capture.kind_size = 96.0 * Self::finite(kind.get("scale").and_then(Value::as_f64), 1.0).max(f64::EPSILON);
                self.capture.kind_field = ArtifactKindCaptureField::Radius;
            }
            ArtifactKindCaptureField::Radius => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_radius(self.capture.kind_size * 0.5).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Width;
            }
            ArtifactKindCaptureField::Width => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_width(self.capture.kind_size).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Height;
            }
            ArtifactKindCaptureField::Height => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_height(self.capture.kind_size).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::IconBegin;
            }
            ArtifactKindCaptureField::IconBegin => {
                if kind.get("icon").and_then(Value::as_str).is_some() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind_icon().map_err(capture_fault_code)?;
                    self.capture.kind_field = ArtifactKindCaptureField::Icon;
                } else {
                    self.capture.kind_field = ArtifactKindCaptureField::Weight;
                }
            }
            ArtifactKindCaptureField::Icon => {
                let value = kind.get("icon").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-kind-icon")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(infinite_canvas::BoardFillIngressKindText::Icon, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.kind_field = ArtifactKindCaptureField::Weight;
                }
            }
            ArtifactKindCaptureField::Weight => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_weight(1.0).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateBegin;
            }
            ArtifactKindCaptureField::TemplateBegin => {
                if template.is_some() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind_handle().map_err(capture_fault_code)?;
                    self.capture.kind_field = ArtifactKindCaptureField::TemplateHandleKind;
                } else {
                    self.capture.kind_field = ArtifactKindCaptureField::Publish;
                }
            }
            ArtifactKindCaptureField::TemplateHandleKind | ArtifactKindCaptureField::TemplateWireKind | ArtifactKindCaptureField::TemplateEdgeKind => {
                let template = template.ok_or("puzzle2d-fill-capture-stale-template")?;
                let (value, field, next) = match self.capture.kind_field {
                    ArtifactKindCaptureField::TemplateHandleKind => {
                        (template.get("handleKind").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-template-kind")?, infinite_canvas::BoardFillIngressTemplateText::HandleKind, ArtifactKindCaptureField::TemplateWireKind)
                    }
                    ArtifactKindCaptureField::TemplateWireKind => (template.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), infinite_canvas::BoardFillIngressTemplateText::WireKind, ArtifactKindCaptureField::TemplateEdgeKind),
                    ArtifactKindCaptureField::TemplateEdgeKind => (template.get("edgeKind").and_then(Value::as_str).unwrap_or(""), infinite_canvas::BoardFillIngressTemplateText::EdgeKind, ArtifactKindCaptureField::TemplateAngle),
                    _ => return Err("puzzle2d-fill-capture-template-field"),
                };
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_handle_text_byte(field, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.kind_field = next;
                }
            }
            ArtifactKindCaptureField::TemplateAngle => {
                let template = template.ok_or("puzzle2d-fill-capture-stale-template")?;
                let angle = Self::finite(template.get("angle").and_then(Value::as_f64), 0.0);
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_handle_angle(angle).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateRadius;
            }
            ArtifactKindCaptureField::TemplateRadius => {
                let template = template.ok_or("puzzle2d-fill-capture-stale-template")?;
                let radius = template.get("radius").and_then(Value::as_f64).filter(|value| value.is_finite() && *value > 0.0);
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_handle_radius(radius).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateWeight;
            }
            ArtifactKindCaptureField::TemplateWeight => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_handle_weight(1.0).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplatePublish;
            }
            ArtifactKindCaptureField::TemplatePublish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_kind_handle().map_err(capture_fault_code)?;
                self.capture.template += 1;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateBegin;
            }
            ArtifactKindCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_kind().map_err(capture_fault_code)?;
                self.capture.kind += 1;
                self.capture.template = 0;
                self.capture.kind_field = ArtifactKindCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_rule_one(&mut self) -> Result<(), &'static str> {
        let document = &self.snapshot.as_ref().ok_or("puzzle2d-fill-snapshot-owner")?.get().0;
        let Some(rules) = Self::rules(document) else {
            if self.capture.rule_field != ArtifactRuleCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-rule");
            }
            self.capture.stage = ArtifactFillCaptureStage::Complete;
            return Ok(());
        };
        let Some(rule) = rules.get(self.capture.rule) else {
            if self.capture.rule_field != ArtifactRuleCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-rule");
            }
            self.capture.stage = ArtifactFillCaptureStage::Complete;
            return Ok(());
        };
        match self.capture.rule_field {
            ArtifactRuleCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_rule().map_err(capture_fault_code)?;
                self.capture.rule_field = ArtifactRuleCaptureField::Source;
            }
            ArtifactRuleCaptureField::Source | ArtifactRuleCaptureField::Target => {
                let (value, field, next) = match self.capture.rule_field {
                    ArtifactRuleCaptureField::Source => (rule.get("source").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-source")?, infinite_canvas::BoardFillIngressRuleText::Source, ArtifactRuleCaptureField::Target),
                    ArtifactRuleCaptureField::Target => (rule.get("target").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-target")?, infinite_canvas::BoardFillIngressRuleText::Target, ArtifactRuleCaptureField::Bidirectional),
                    _ => return Err("puzzle2d-fill-capture-rule-field"),
                };
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_rule_text_byte(field, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.rule_field = next;
                }
            }
            ArtifactRuleCaptureField::Bidirectional => {
                let bidirectional = rule.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_rule_bidirectional(bidirectional).map_err(capture_fault_code)?;
                self.capture.rule_field = ArtifactRuleCaptureField::Specificity;
            }
            ArtifactRuleCaptureField::Specificity => {
                let specificity = rule.get("specificity").and_then(Value::as_str).unwrap_or("handle");
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_rule_specificity(specificity).map_err(capture_fault_code)?;
                self.capture.rule_field = ArtifactRuleCaptureField::Publish;
            }
            ArtifactRuleCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_rule().map_err(capture_fault_code)?;
                self.capture.rule += 1;
                self.capture.rule_field = ArtifactRuleCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_one(&mut self) -> Result<(), &'static str> {
        match self.capture.stage {
            ArtifactFillCaptureStage::Nodes => self.capture_node_one(),
            ArtifactFillCaptureStage::Handles => self.capture_handle_one(),
            ArtifactFillCaptureStage::Kinds => self.capture_kind_one(),
            ArtifactFillCaptureStage::Rules => self.capture_rule_one(),
            ArtifactFillCaptureStage::Complete => {
                let snapshot = self.ingress.as_mut().and_then(infinite_canvas::BoardFillSnapshotIngress::take_snapshot).ok_or("puzzle2d-fill-capture-snapshot")?;
                self.ingress = None;
                self.inner = Some(infinite_canvas::BoardFillJob::with_operation(snapshot, self.maximum_count, self.operation));
                Ok(())
            }
        }
    }

    fn take_preview(&mut self) -> Option<infinite_canvas::BoardFillPreview> {
        self.inner.as_mut().and_then(infinite_canvas::BoardFillJob::take_preview)
    }

    fn take_checkpoint(&mut self) -> Option<infinite_canvas::BoardFillCheckpoint> {
        self.inner.as_mut().and_then(infinite_canvas::BoardFillJob::take_checkpoint)
    }

    fn adopt_checkpoint(&mut self, checkpoint: infinite_canvas::BoardFillCheckpoint) -> Result<(), infinite_canvas::BoardFillCheckpoint> {
        let Some(inner) = self.inner.as_mut() else { return Err(checkpoint) };
        inner.adopt_checkpoint(checkpoint)
    }

    fn take_fault(&mut self) -> Option<&'static str> {
        self.fault.take().or_else(|| self.inner.as_mut().and_then(infinite_canvas::BoardFillJob::take_fault))
    }

    fn fault_outcome(&mut self, code: &'static str) -> semio_framework_job::StepOutcome {
        self.fault = Some(code);
        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) })
    }
}

impl semio_framework_job::InteractiveJob for ArtifactBoardFillJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if context.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return self.fault_outcome("stale-puzzle2d-artifact-fill-operation");
        }
        if self.snapshot.as_ref().is_none_or(|snapshot| !snapshot.commit_authority_matches(self.render_generation, self.canonical_base_revision)) {
            return self.fault_outcome("stale-puzzle2d-artifact-fill-snapshot");
        }
        if context.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        if let Some(inner) = self.inner.as_mut() {
            return semio_framework_job::InteractiveJob::step(inner, context);
        }
        context.set_stage("puzzle2d-fill-artifact-capture");
        if let Err(code) = self.capture_one() {
            return self.fault_outcome(code);
        }
        context.consume_fuel(1);
        if context.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return self.fault_outcome("stale-puzzle2d-artifact-fill-operation");
        }
        if self.snapshot.as_ref().is_none_or(|snapshot| !snapshot.commit_authority_matches(self.render_generation, self.canonical_base_revision)) {
            return self.fault_outcome("stale-puzzle2d-artifact-fill-snapshot");
        }
        if context.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        semio_framework_job::StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(inner) = self.inner.as_mut() {
            semio_framework_job::InteractiveJob::begin_close(inner);
            let step = semio_framework_job::InteractiveJob::close_step(inner, 1, maximum_bytes);
            if matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete) && semio_framework_job::InteractiveJob::terminal_is_empty(inner) {
                self.inner = None;
            }
            return step;
        }
        if let Some(ingress) = self.ingress.as_mut() {
            ingress.begin_close();
            let step = ingress.close_step(1, maximum_bytes);
            if matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete) && ingress.terminal_is_empty() {
                self.ingress = None;
            }
            return step;
        }
        if let Some(snapshot) = self.snapshot.take() {
            let Some(witness) = snapshot.return_to_registry_witness() else {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            };
            self.snapshot_return = Some(witness);
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<store::SnapshotRead<Puzzle2dPlaySnapshot>>() };
        }
        if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.snapshot_return.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<store::SnapshotReadReturn>() };
        }
        if self.fault.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.snapshot.is_none() && self.snapshot_return.is_none() && self.ingress.is_none() && self.inner.is_none() && self.fault.is_none()
    }
}

impl Drop for ArtifactBoardFillJob {
    fn drop(&mut self) {
        assert!(semio_framework_job::InteractiveJob::terminal_is_empty(self), "Puzzle2d artifact fill job must reach exact terminal-empty before Drop");
    }
}

struct FillSessionNode {
    app_instance_id: u32,
    operation: semio_framework_job::Operation,
    canonical_base_revision: [u8; 32],
    maximum_count: u32,
    cancel: semio_framework_async::CancelToken,
    work: FillWork,
    retained_outcome: Option<semio_framework_job::StepOutcome>,
    outcome_terminal: bool,
    terminal_published: bool,
    checkpoint: Option<infinite_canvas::BoardFillCheckpoint>,
    apply: Option<FillPlacementApplyCursor>,
    checkpoint_sequence: u64,
    terminal: Option<FillTerminal>,
    closing: bool,
}

struct FillPlacementApplyCursor {
    placement: Option<infinite_canvas::BoardFillPlacement>,
    handles: [Option<FillPlacementHandleOwner>; infinite_canvas::BOARD_FILL_KIND_HANDLE_CAPACITY],
    handle: Option<FillPlacementHandleOwner>,
    node: Option<FillPlacementNodeOwner>,
    edge: Option<FillPlacementEdgeOwner>,
    handle_cursor: usize,
    text_byte: usize,
    stage: FillPlacementApplyStage,
}

#[derive(Clone, Copy)]
struct FillPlacementHandleOwner {
    id: infinite_canvas::BoardFillText,
    handle_kind: infinite_canvas::BoardFillText,
    angle: f64,
    radius: Option<f64>,
}

#[derive(Clone, Copy)]
struct FillPlacementNodeOwner {
    id: infinite_canvas::BoardFillText,
    node_kind: infinite_canvas::BoardFillText,
    target_handle_index: usize,
    shape: infinite_canvas::BoardFillCommitShape,
    x: f64,
    y: f64,
    radius: f64,
    width: f64,
    height: f64,
    icon_kind: Option<infinite_canvas::BoardFillText>,
}

#[derive(Clone, Copy)]
struct FillPlacementEdgeOwner {
    id: infinite_canvas::BoardFillText,
    source: infinite_canvas::BoardFillText,
    target: infinite_canvas::BoardFillText,
    edge_kind: infinite_canvas::BoardFillText,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FillPlacementApplyStage {
    BeginHandle,
    HandleId,
    HandleKind,
    HandleAngle,
    HandleRadius,
    HandlePublish,
    NodeBegin,
    NodeId,
    NodeKind,
    NodeTarget,
    NodeShape,
    NodeX,
    NodeY,
    NodeText,
    NodeAnchor,
    NodeGeometry,
    NodeHeight,
    NodeIconBegin,
    NodeIcon,
    EdgeBegin,
    EdgeId,
    EdgeKind,
    EdgeSource,
    EdgeTarget,
    Publish,
    Close,
}

enum FillPlacementApplyStep {
    Pending,
    Complete,
}

fn copy_fill_text_one(source: &infinite_canvas::BoardFillText, destination: &mut infinite_canvas::BoardFillText, byte: &mut usize) -> Result<bool, &'static str> {
    let source = source.as_str().as_bytes();
    let Some(value) = source.get(*byte).copied() else {
        *byte = 0;
        return Ok(true);
    };
    destination.try_push_byte(value).map_err(|_| "puzzle2d-fill-fixed-text")?;
    *byte += 1;
    if *byte == source.len() {
        *byte = 0;
        return Ok(true);
    }
    Ok(false)
}

fn try_document_str(text: &str) -> Result<String, &'static str> {
    let mut output = String::new();
    output.try_reserve_exact(text.len()).map_err(|_| "puzzle2d-fill-document-text")?;
    output.push_str(text);
    Ok(output)
}

fn try_document_text(text: infinite_canvas::BoardFillText) -> Result<String, &'static str> {
    try_document_str(text.as_str())
}

#[derive(Clone, Copy)]
enum FillPlacementPublishHandles<'a> {
    Commit(&'a [Option<infinite_canvas::BoardFillCommitHandle>; infinite_canvas::BOARD_FILL_KIND_HANDLE_CAPACITY]),
    Cursor(&'a [Option<FillPlacementHandleOwner>; infinite_canvas::BOARD_FILL_KIND_HANDLE_CAPACITY]),
}

struct FillPlacementPublishView<'a> {
    node_id: &'a infinite_canvas::BoardFillText,
    edge_id: &'a infinite_canvas::BoardFillText,
    edge_kind: &'a infinite_canvas::BoardFillText,
    node_kind: &'a infinite_canvas::BoardFillText,
    source_handle_id: &'a infinite_canvas::BoardFillText,
    target_handle_id: &'a infinite_canvas::BoardFillText,
    target_handle_index: usize,
    x: f64,
    y: f64,
    shape: infinite_canvas::BoardFillCommitShape,
    radius: f64,
    width: f64,
    height: f64,
    icon_kind: Option<&'a infinite_canvas::BoardFillText>,
    handles: FillPlacementPublishHandles<'a>,
    handle_count: usize,
}

impl<'a> FillPlacementPublishView<'a> {
    fn from_commit(placement: &'a infinite_canvas::BoardFillCommitPlacement) -> Self {
        Self {
            node_id: &placement.node_id,
            edge_id: &placement.edge_id,
            edge_kind: &placement.edge_kind,
            node_kind: &placement.node_kind,
            source_handle_id: &placement.source_handle_id,
            target_handle_id: &placement.target_handle_id,
            target_handle_index: placement.target_handle_index,
            x: placement.x,
            y: placement.y,
            shape: placement.shape,
            radius: placement.radius,
            width: placement.width,
            height: placement.height,
            icon_kind: placement.icon_kind.as_ref(),
            handles: FillPlacementPublishHandles::Commit(&placement.handles),
            handle_count: placement.handle_count,
        }
    }

    fn from_cursor(node: &'a FillPlacementNodeOwner, edge: &'a FillPlacementEdgeOwner, handles: &'a [Option<FillPlacementHandleOwner>; infinite_canvas::BOARD_FILL_KIND_HANDLE_CAPACITY], handle_count: usize) -> Self {
        Self {
            node_id: &node.id,
            edge_id: &edge.id,
            edge_kind: &edge.edge_kind,
            node_kind: &node.node_kind,
            source_handle_id: &edge.source,
            target_handle_id: &edge.target,
            target_handle_index: node.target_handle_index,
            x: node.x,
            y: node.y,
            shape: node.shape,
            radius: node.radius,
            width: node.width,
            height: node.height,
            icon_kind: node.icon_kind.as_ref(),
            handles: FillPlacementPublishHandles::Cursor(handles),
            handle_count,
        }
    }

    fn handle(&self, index: usize) -> Option<infinite_canvas::BoardFillCommitHandle> {
        match self.handles {
            FillPlacementPublishHandles::Commit(handles) => *handles.get(index)?,
            FillPlacementPublishHandles::Cursor(handles) => handles.get(index)?.map(|handle| infinite_canvas::BoardFillCommitHandle { id: handle.id, handle_kind: handle.handle_kind, angle: handle.angle, radius: handle.radius }),
        }
    }
}

/// 📤️ Pre-credits the final event destination before materializing the fixed terminal owner.
fn publish_fixed_placement(placement: FillPlacementPublishView<'_>, mutations: &mut Vec<crate::artifacts::puzzle2d::mutations::Puzzle2dMutation>) -> Result<(), &'static str> {
    if placement.handle_count > infinite_canvas::BOARD_FILL_KIND_HANDLE_CAPACITY {
        return Err("puzzle2d-fill-apply-handle-capacity");
    }
    let shape = match placement.shape {
        infinite_canvas::BoardFillCommitShape::Circle => "circle",
        infinite_canvas::BoardFillCommitShape::Rectangle => "rectangle",
    };
    let mut required_bytes = std::mem::size_of::<crate::artifacts::puzzle2d::mutations::Puzzle2dMutation>()
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(std::mem::size_of::<crate::artifacts::puzzle2d::Puzzle2dHandle>().checked_mul(placement.handle_count)?))
        .and_then(|bytes| bytes.checked_add(placement.node_id.as_str().len().checked_mul(2)?))
        .and_then(|bytes| bytes.checked_add(placement.edge_id.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.edge_kind.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.node_kind.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.source_handle_id.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.target_handle_id.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.icon_kind.map_or(0, |icon| icon.as_str().len())))
        .and_then(|bytes| bytes.checked_add(shape.len()))
        .ok_or("puzzle2d-fill-apply-backing")?;
    for index in 0..placement.handle_count {
        let handle = placement.handle(index).ok_or("puzzle2d-fill-apply-handle-owner")?;
        required_bytes = required_bytes.checked_add(handle.id.as_str().len()).and_then(|bytes| bytes.checked_add(handle.handle_kind.as_str().len())).ok_or("puzzle2d-fill-apply-backing")?;
    }
    if required_bytes > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES * 2 {
        return Err("puzzle2d-fill-apply-backing");
    }
    mutations.try_reserve_exact(2).map_err(|_| "puzzle2d-fill-mutation-page")?;
    let mut handles = Vec::new();
    handles.try_reserve_exact(placement.handle_count).map_err(|_| "puzzle2d-fill-apply-backing")?;
    for index in 0..placement.handle_count {
        let handle = placement.handle(index).ok_or("puzzle2d-fill-apply-handle-owner")?;
        handles.push(crate::artifacts::puzzle2d::Puzzle2dHandle { id: try_document_text(handle.id)?, handle_kind: Some(try_document_text(handle.handle_kind)?), angle: handle.angle, radius: handle.radius, ..Default::default() });
    }
    let node = crate::artifacts::puzzle2d::Puzzle2dNode {
        id: try_document_text(*placement.node_id)?,
        node_kind: Some(try_document_text(*placement.node_kind)?),
        shape: Some(try_document_str(shape)?),
        x: placement.x,
        y: placement.y,
        radius: matches!(placement.shape, infinite_canvas::BoardFillCommitShape::Circle).then_some(placement.radius),
        width: matches!(placement.shape, infinite_canvas::BoardFillCommitShape::Rectangle).then_some(placement.width),
        height: matches!(placement.shape, infinite_canvas::BoardFillCommitShape::Rectangle).then_some(placement.height),
        text: Some(try_document_text(*placement.node_id)?),
        icon_kind: placement.icon_kind.copied().map(try_document_text).transpose()?,
        anchor: crate::artifacts::puzzle2d::Puzzle2dNodeAnchor::Fixed,
        handles,
        ..Default::default()
    };
    let edge = crate::artifacts::puzzle2d::mutations::connect_handles(
        try_document_text(*placement.edge_id)?,
        try_document_text(*placement.source_handle_id)?,
        try_document_text(*placement.target_handle_id)?,
        Some(try_document_text(*placement.edge_kind)?),
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        None,
        None,
    );
    mutations.push(crate::artifacts::puzzle2d::mutations::create_node(node, None));
    mutations.push(edge);
    Ok(())
}

fn publish_commit_candidate(candidate: &semio_framework_job::CommitCandidate, mutations: &mut Vec<crate::artifacts::puzzle2d::mutations::Puzzle2dMutation>) -> Option<Result<infinite_canvas::BoardFillResult, &'static str>> {
    let candidate = infinite_canvas::BoardFillCommitCandidate::from_commit_candidate(candidate)?;
    if let Some(placement) = candidate.placement.as_ref() {
        if let Err(code) = publish_fixed_placement(FillPlacementPublishView::from_commit(placement), mutations) {
            return Some(Err(code));
        }
    }
    Some(Ok(candidate.result))
}

impl FillPlacementApplyCursor {
    fn new(placement: infinite_canvas::BoardFillPlacement) -> Self {
        Self { placement: Some(placement), handles: std::array::from_fn(|_| None), handle: None, node: None, edge: None, handle_cursor: 0, text_byte: 0, stage: FillPlacementApplyStage::BeginHandle }
    }

    fn step(&mut self, mutations: &mut Vec<crate::artifacts::puzzle2d::mutations::Puzzle2dMutation>) -> Result<FillPlacementApplyStep, &'static str> {
        let placement = self.placement.as_ref().ok_or("puzzle2d-fill-apply-owner")?;
        match self.stage {
            FillPlacementApplyStage::BeginHandle => {
                if self.handle_cursor < placement.handle_count() {
                    self.handle = Some(FillPlacementHandleOwner { id: infinite_canvas::BoardFillText::empty(), handle_kind: infinite_canvas::BoardFillText::empty(), angle: 0.0, radius: None });
                    self.stage = FillPlacementApplyStage::HandleId;
                } else {
                    self.stage = FillPlacementApplyStage::NodeBegin;
                }
            }
            FillPlacementApplyStage::HandleId => {
                let source = placement.fixed_handle_id(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                let destination = &mut self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.id;
                if copy_fill_text_one(source, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::HandleKind;
                }
            }
            FillPlacementApplyStage::HandleKind => {
                let source = placement.fixed_handle_kind(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                let destination = &mut self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.handle_kind;
                if copy_fill_text_one(source, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::HandleAngle;
                }
            }
            FillPlacementApplyStage::HandleAngle => {
                self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.angle = placement.fixed_handle_angle(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                self.stage = FillPlacementApplyStage::HandleRadius;
            }
            FillPlacementApplyStage::HandleRadius => {
                self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.radius = placement.fixed_handle_radius(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                self.stage = FillPlacementApplyStage::HandlePublish;
            }
            FillPlacementApplyStage::HandlePublish => {
                let handle = self.handle.take().ok_or("puzzle2d-fill-apply-handle-owner")?;
                let slot = self.handles.get_mut(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle-capacity")?;
                if slot.is_some() {
                    self.handle = Some(handle);
                    return Err("puzzle2d-fill-apply-handle-owner");
                }
                *slot = Some(handle);
                self.handle_cursor += 1;
                self.stage = FillPlacementApplyStage::BeginHandle;
            }
            FillPlacementApplyStage::NodeBegin => {
                self.node = Some(FillPlacementNodeOwner {
                    id: infinite_canvas::BoardFillText::empty(),
                    node_kind: infinite_canvas::BoardFillText::empty(),
                    target_handle_index: 0,
                    shape: infinite_canvas::BoardFillCommitShape::Circle,
                    x: 0.0,
                    y: 0.0,
                    radius: 0.0,
                    width: 0.0,
                    height: 0.0,
                    icon_kind: None,
                });
                self.stage = FillPlacementApplyStage::NodeId;
            }
            FillPlacementApplyStage::NodeId => {
                let destination = &mut self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.id;
                if copy_fill_text_one(&placement.node_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::NodeKind;
                }
            }
            FillPlacementApplyStage::NodeKind => {
                let destination = &mut self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.node_kind;
                if copy_fill_text_one(&placement.node_kind, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::NodeTarget;
                }
            }
            FillPlacementApplyStage::NodeTarget => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.target_handle_index = placement.target_handle_index;
                self.stage = FillPlacementApplyStage::NodeShape;
            }
            FillPlacementApplyStage::NodeShape => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.shape = match placement.shape {
                    "circle" => infinite_canvas::BoardFillCommitShape::Circle,
                    "rectangle" => infinite_canvas::BoardFillCommitShape::Rectangle,
                    _ => return Err("puzzle2d-fill-node-shape"),
                };
                self.stage = FillPlacementApplyStage::NodeX;
            }
            FillPlacementApplyStage::NodeX => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.x = placement.x;
                self.stage = FillPlacementApplyStage::NodeY;
            }
            FillPlacementApplyStage::NodeY => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.y = placement.y;
                self.stage = FillPlacementApplyStage::NodeText;
            }
            FillPlacementApplyStage::NodeText => {
                self.stage = FillPlacementApplyStage::NodeAnchor;
            }
            FillPlacementApplyStage::NodeAnchor => {
                self.stage = FillPlacementApplyStage::NodeGeometry;
            }
            FillPlacementApplyStage::NodeGeometry => {
                let node = self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?;
                if placement.shape == "rectangle" {
                    node.width = placement.width;
                    self.stage = FillPlacementApplyStage::NodeHeight;
                } else {
                    node.radius = placement.radius;
                    self.stage = FillPlacementApplyStage::NodeIconBegin;
                }
            }
            FillPlacementApplyStage::NodeHeight => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.height = placement.height;
                self.stage = FillPlacementApplyStage::NodeIconBegin;
            }
            FillPlacementApplyStage::NodeIconBegin => {
                if placement.icon_kind.as_ref().is_some() {
                    self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.icon_kind = Some(infinite_canvas::BoardFillText::empty());
                    self.stage = FillPlacementApplyStage::NodeIcon;
                } else {
                    self.stage = FillPlacementApplyStage::EdgeBegin;
                }
            }
            FillPlacementApplyStage::NodeIcon => {
                let source = placement.icon_kind.as_ref().ok_or("puzzle2d-fill-node-icon")?;
                let destination = self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.icon_kind.as_mut().ok_or("puzzle2d-fill-node-icon-owner")?;
                if copy_fill_text_one(source, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeBegin;
                }
            }
            FillPlacementApplyStage::EdgeBegin => {
                self.edge =
                    Some(FillPlacementEdgeOwner { id: infinite_canvas::BoardFillText::empty(), source: infinite_canvas::BoardFillText::empty(), target: infinite_canvas::BoardFillText::empty(), edge_kind: infinite_canvas::BoardFillText::empty() });
                self.stage = FillPlacementApplyStage::EdgeId;
            }
            FillPlacementApplyStage::EdgeId => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.id;
                if copy_fill_text_one(&placement.edge_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeKind;
                }
            }
            FillPlacementApplyStage::EdgeKind => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.edge_kind;
                if copy_fill_text_one(&placement.edge_kind, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeSource;
                }
            }
            FillPlacementApplyStage::EdgeSource => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.source;
                if copy_fill_text_one(&placement.source_handle_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeTarget;
                }
            }
            FillPlacementApplyStage::EdgeTarget => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.target;
                if copy_fill_text_one(&placement.target_handle_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::Publish;
                }
            }
            FillPlacementApplyStage::Publish => {
                let node = self.node.as_ref().ok_or("puzzle2d-fill-node-owner")?;
                let edge = self.edge.as_ref().ok_or("puzzle2d-fill-edge-owner")?;
                publish_fixed_placement(FillPlacementPublishView::from_cursor(node, edge, &self.handles, self.handle_cursor), mutations)?;
                self.node = None;
                self.edge = None;
                self.handles = std::array::from_fn(|_| None);
                self.stage = FillPlacementApplyStage::Close;
            }
            FillPlacementApplyStage::Close => {
                let placement = self.placement.as_mut().ok_or("puzzle2d-fill-apply-owner")?;
                if placement.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                    self.placement = None;
                    return Ok(FillPlacementApplyStep::Complete);
                }
            }
        }
        Ok(FillPlacementApplyStep::Pending)
    }

    fn close_step(&mut self) -> bool {
        if self.handle.take().is_some() {
            return false;
        }
        if let Some(slot) = self.handles.iter_mut().rev().find(|slot| slot.is_some()) {
            *slot = None;
            return false;
        }
        if self.node.take().is_some() {
            return false;
        }
        if self.edge.take().is_some() {
            return false;
        }
        if let Some(placement) = self.placement.as_mut() {
            if placement.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                self.placement = None;
            }
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.placement.is_none() && self.handles.iter().all(Option::is_none) && self.handle.is_none() && self.node.is_none() && self.edge.is_none()
    }
}

impl Drop for FillPlacementApplyCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Puzzle2d placement apply must reach exact terminal-empty before Drop");
    }
}

impl FillSessionNode {
    fn matches(&self, app_instance_id: u32, operation: u64, generation: u64) -> bool {
        self.app_instance_id == app_instance_id && self.operation.operation.0 == operation && self.operation.generation.0 == generation
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.cancel.cancel_now();
    }

    fn close_step(&mut self) -> bool {
        if self.terminal.take().is_some() {
            return false;
        }
        if let Some(outcome) = self.retained_outcome.as_mut() {
            if !outcome.terminal_is_empty() {
                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                return false;
            }
            self.retained_outcome = None;
            return false;
        }
        if let Some(apply) = self.apply.as_mut() {
            if apply.close_step() {
                self.apply = None;
            }
            return false;
        }
        if let Some(checkpoint) = self.checkpoint.take() {
            match &mut self.work {
                FillWork::Session(session) => {
                    let Some(job) = session.checked_out_job_mut() else {
                        session.begin_close();
                        self.work = FillWork::Detached(checkpoint.into_closing_job());
                        return false;
                    };
                    if let Err(checkpoint) = job.adopt_checkpoint(checkpoint) {
                        session.begin_close();
                        self.work = FillWork::Detached(checkpoint.into_closing_job());
                        return false;
                    }
                    session.begin_close();
                    return false;
                }
                _ => {
                    self.work = FillWork::Detached(checkpoint.into_closing_job());
                    return false;
                }
            }
        }
        match &mut self.work {
            FillWork::AwaitingSnapshot => self.work = FillWork::Empty,
            FillWork::Session(session) => {
                session.begin_close();
                if matches!(session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::WorkerJobCloseStep::Complete) && session.terminal_is_empty() {
                    self.work = FillWork::Empty;
                }
            }
            FillWork::Rejected(rejected) => {
                rejected.begin_close();
                if matches!(rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && rejected.terminal_is_empty() {
                    self.work = FillWork::Empty;
                }
            }
            FillWork::Detached(job) => {
                semio_framework_job::InteractiveJob::begin_close(job);
                if matches!(semio_framework_job::InteractiveJob::close_step(job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && semio_framework_job::InteractiveJob::terminal_is_empty(job) {
                    self.work = FillWork::Empty;
                }
            }
            FillWork::Empty => {}
        }
        self.terminal_is_empty()
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.work, FillWork::Empty) && self.retained_outcome.is_none() && self.checkpoint.is_none() && self.apply.is_none() && self.terminal.is_none()
    }
}

impl Drop for FillSessionNode {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Puzzle2d fill node must reach exact terminal-empty before Drop");
    }
}

struct FillSessionBacking {
    pointer: std::ptr::NonNull<std::mem::MaybeUninit<FillSessionNode>>,
    initialized: bool,
}

impl FillSessionBacking {
    fn try_new() -> Option<Self> {
        let layout = std::alloc::Layout::new::<FillSessionNode>();
        let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<std::mem::MaybeUninit<FillSessionNode>>())?;
        Some(Self { pointer, initialized: false })
    }

    fn write(mut self, node: FillSessionNode) -> Box<FillSessionNode> {
        let pointer = self.pointer.as_ptr().cast::<FillSessionNode>();
        unsafe { pointer.write(node) };
        self.initialized = true;
        unsafe { Box::from_raw(pointer) }
    }
}

impl Drop for FillSessionBacking {
    fn drop(&mut self) {
        if !self.initialized {
            unsafe { std::alloc::dealloc(self.pointer.as_ptr().cast::<u8>(), std::alloc::Layout::new::<FillSessionNode>()) };
        }
    }
}

const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<ArtifactBoardFillJob>();
    assert_send::<FillSessionNode>();
};

struct FillRegistrySlot {
    node: AtomicPtr<FillSessionNode>,
}

impl FillRegistrySlot {
    const fn new() -> Self {
        Self { node: AtomicPtr::new(std::ptr::null_mut()) }
    }
}

static FILL_SESSION_SLOTS: [FillRegistrySlot; FILL_SESSION_CAPACITY] = [const { FillRegistrySlot::new() }; FILL_SESSION_CAPACITY];

struct FillSessionGuard {
    slot: usize,
    node: Option<Box<FillSessionNode>>,
}

impl FillSessionGuard {
    fn node_mut(&mut self) -> Option<&mut FillSessionNode> {
        self.node.as_deref_mut()
    }

    fn retire(mut self) {
        let Some(node) = self.node.take() else { return };
        if !node.terminal_is_empty() {
            self.node = Some(node);
            return;
        }
        FILL_SESSION_SLOTS[self.slot].node.store(std::ptr::null_mut(), Ordering::Release);
        drop(node);
    }
}

impl Drop for FillSessionGuard {
    fn drop(&mut self) {
        let Some(node) = self.node.take() else { return };
        FILL_SESSION_SLOTS[self.slot].node.store(Box::into_raw(node), Ordering::Release);
    }
}

struct FillSessionReservation {
    slot: usize,
    published: bool,
}

impl Drop for FillSessionReservation {
    fn drop(&mut self) {
        if self.published {
            return;
        }
        let _ = FILL_SESSION_SLOTS[self.slot].node.compare_exchange(FILL_SESSION_LOCKED, std::ptr::null_mut(), Ordering::AcqRel, Ordering::Acquire);
    }
}

fn reserve_session_slot() -> Option<FillSessionReservation> {
    for (index, slot) in FILL_SESSION_SLOTS.iter().enumerate() {
        if slot.node.compare_exchange(std::ptr::null_mut(), FILL_SESSION_LOCKED, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            return Some(FillSessionReservation { slot: index, published: false });
        }
    }
    None
}

fn publish_session(mut reservation: FillSessionReservation, node: Box<FillSessionNode>) {
    FILL_SESSION_SLOTS[reservation.slot].node.store(Box::into_raw(node), Ordering::Release);
    reservation.published = true;
}

fn take_matching_session(app_instance_id: u32, operation: u64, generation: u64) -> Option<FillSessionGuard> {
    for (index, slot) in FILL_SESSION_SLOTS.iter().enumerate() {
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() || pointer == FILL_SESSION_LOCKED || slot.node.compare_exchange(pointer, FILL_SESSION_LOCKED, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let node = unsafe { Box::from_raw(pointer) };
        if node.matches(app_instance_id, operation, generation) {
            return Some(FillSessionGuard { slot: index, node: Some(node) });
        }
        slot.node.store(Box::into_raw(node), Ordering::Release);
    }
    None
}

fn take_snapshot_pending_session(render: semio_framework_plugin::AppRenderOperationContext) -> Option<FillSessionGuard> {
    for (index, slot) in FILL_SESSION_SLOTS.iter().enumerate() {
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() || pointer == FILL_SESSION_LOCKED || slot.node.compare_exchange(pointer, FILL_SESSION_LOCKED, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let node = unsafe { Box::from_raw(pointer) };
        if node.app_instance_id == render.app_instance_id && node.canonical_base_revision == render.canonical_base_revision && matches!(&node.work, FillWork::AwaitingSnapshot) && !node.closing {
            return Some(FillSessionGuard { slot: index, node: Some(node) });
        }
        slot.node.store(Box::into_raw(node), Ordering::Release);
    }
    None
}

fn pump_abandoned_session(active: Option<(u32, u64, u64)>) -> bool {
    for (index, slot) in FILL_SESSION_SLOTS.iter().enumerate() {
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() || pointer == FILL_SESSION_LOCKED || slot.node.compare_exchange(pointer, FILL_SESSION_LOCKED, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let mut guard = FillSessionGuard { slot: index, node: Some(unsafe { Box::from_raw(pointer) }) };
        let is_active = guard.node.as_ref().is_some_and(|node| active.is_some_and(|key| node.matches(key.0, key.1, key.2)));
        if is_active {
            continue;
        }
        if let Some(node) = guard.node_mut() {
            node.begin_close();
            if node.close_step() {
                guard.retire();
                return true;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
fn registry_has_sessions() -> bool {
    FILL_SESSION_SLOTS.iter().any(|slot| !slot.node.load(Ordering::Acquire).is_null())
}

fn fill_action_effect(generation: u64, action: &'static str) -> Effect {
    Effect::DispatchAction {
        req: semio_framework_plugin::kernel::RequestId(semio_framework_job::allocate_operation_id().0),
        action: action.into(),
        args: semio_framework::optional_json_to_dsl(Some(serde_json::json!({ "generation": generation }))),
        delay_ms: 1,
    }
}

fn queue_fill_action(ctx: &mut Puzzle2dFillActionCtx<'_>, action: &'static str) {
    ctx.effects.push(fill_action_effect(ctx.runtime.fill_job_generation, action));
}

fn fill_worker_pool() -> semio_framework_async::WorkerPool {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
}

/// 🔍️ Requests one immutable store lease only for an exact awaiting fill authority.
pub fn prepare_snapshot_read(render: semio_framework_plugin::AppRenderOperationContext, _snapshot: &Puzzle2dPlaySnapshot) -> bool {
    take_snapshot_pending_session(render).is_some()
}

/// 🧵️ Transfers the exact immutable document lease into the shared-worker capture session.
pub fn reconcile_snapshot_read(doc: &semio_framework_plugin::ArtifactView<'_, Puzzle2dPlaySnapshot>, config: &semio_framework_plugin::ConfigView<'_, crate::editor::puzzle2d::config::Puzzle2dConfig>) -> Vec<Effect> {
    let Some(render) = doc.render_operation() else { return Vec::new() };
    let Some(mut guard) = take_snapshot_pending_session(render) else { return Vec::new() };
    let snapshot = match doc.take_snapshot_read() {
        Ok(snapshot) => snapshot,
        Err(_) => return Vec::new(),
    };
    let Some(node) = guard.node_mut() else { return Vec::new() };
    let generation = node.operation.generation.0;
    let job = ArtifactBoardFillJob::new(node.operation, render.generation.0, render.canonical_base_revision, node.maximum_count, config.snapshot.suggestion_offset, snapshot);
    let params = semio_framework_job::BatchJobParams {
        operation: node.operation.operation,
        generation: node.operation.generation,
        cancel: node.cancel.clone(),
        config: semio_framework_job::BatchDriveConfig { site: "puzzle2d.fill", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 7000 },
        now_us: semio_framework_job::default_now_us,
    };
    match semio_framework_job::MountedWorkerJobSession::try_new(job, params) {
        Ok(session) => node.work = FillWork::Session(session),
        Err(rejected) => {
            node.work = FillWork::Rejected(rejected);
            node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-admission-rejected"));
            node.begin_close();
        }
    }
    vec![fill_action_effect(generation, "brushFillSessionStep")]
}

fn base_revision(authority: &semio_framework_plugin::AppOperationContext) -> u64 {
    u64::from_le_bytes([
        authority.canonical_base_revision[0],
        authority.canonical_base_revision[1],
        authority.canonical_base_revision[2],
        authority.canonical_base_revision[3],
        authority.canonical_base_revision[4],
        authority.canonical_base_revision[5],
        authority.canonical_base_revision[6],
        authority.canonical_base_revision[7],
    ])
}

fn fill_fault(ctx: &mut Puzzle2dFillActionCtx<'_>, code: &'static str) {
    match crate::editor::puzzle2d::config::Puzzle2dFillText::try_from_str(code) {
        Some(code) => ctx.runtime.fill_job_fault_code = Some(code),
        None => *ctx.boundary_fault = Some("puzzle2d-fill-runtime-text-capacity"),
    }
    ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Faulted;
}

pub fn reject_fill_request(ctx: &mut Puzzle2dFillActionCtx<'_>, code: &'static str) {
    fill_fault(ctx, code);
}

fn capture_fault_code(fault: infinite_canvas::BoardFillCaptureFault) -> &'static str {
    match fault {
        infinite_canvas::BoardFillCaptureFault::TextCapacity => "puzzle2d-fill-capture-text-capacity",
        infinite_canvas::BoardFillCaptureFault::NodeCapacity => "puzzle2d-fill-capture-node-capacity",
        infinite_canvas::BoardFillCaptureFault::HandleCapacity => "puzzle2d-fill-capture-handle-capacity",
        infinite_canvas::BoardFillCaptureFault::KindCapacity => "puzzle2d-fill-capture-kind-capacity",
        infinite_canvas::BoardFillCaptureFault::KindHandleCapacity => "puzzle2d-fill-capture-kind-handle-capacity",
        infinite_canvas::BoardFillCaptureFault::RuleCapacity => "puzzle2d-fill-capture-rule-capacity",
        infinite_canvas::BoardFillCaptureFault::StaleNode => "puzzle2d-fill-capture-stale-node",
        infinite_canvas::BoardFillCaptureFault::StaleHandle => "puzzle2d-fill-capture-stale-handle",
        infinite_canvas::BoardFillCaptureFault::StaleKind => "puzzle2d-fill-capture-stale-kind",
        infinite_canvas::BoardFillCaptureFault::StaleRule => "puzzle2d-fill-capture-stale-rule",
        infinite_canvas::BoardFillCaptureFault::GenerationExhausted => "puzzle2d-fill-capture-generation-exhausted",
    }
}

fn action_authority<'a>(ctx: &'a Puzzle2dFillActionCtx<'_>) -> Option<&'a semio_framework_plugin::AppOperationContext> {
    ctx.operation.as_ref()
}

fn is_fresh(ctx: &Puzzle2dFillActionCtx<'_>, node: &FillSessionNode) -> bool {
    action_authority(ctx).is_some_and(|authority| {
        authority.app_instance_id == node.app_instance_id
            && authority.operation_id == node.operation.operation.0
            && authority.generation == node.operation.generation.0
            && authority.canonical_base_revision == node.canonical_base_revision
            && ctx.runtime.fill_job_operation == node.operation.operation.0
            && ctx.runtime.fill_job_generation == node.operation.generation.0
            && ctx.runtime.fill_job_base_revision == node.operation.base_revision.0
    })
}

fn queue_fill_step(ctx: &mut Puzzle2dFillActionCtx<'_>) {
    queue_fill_action(ctx, "brushFillSessionStep");
}

fn queue_fill_adopt(ctx: &mut Puzzle2dFillActionCtx<'_>) {
    queue_fill_action(ctx, "brushFillSessionAdopt");
}

fn queue_fill_discard(ctx: &mut Puzzle2dFillActionCtx<'_>) {
    queue_fill_action(ctx, "brushFillSessionDiscard");
}

fn publish_preview(ctx: &mut Puzzle2dFillActionCtx<'_>, preview: infinite_canvas::BoardFillPreview) {
    if let Some(stage) = crate::editor::puzzle2d::config::Puzzle2dFillText::try_from_str(preview.stage.id()) {
        ctx.runtime.fill_job_stage = stage;
    } else {
        fill_fault(ctx, "puzzle2d-fill-stage-capacity");
        return;
    }
    ctx.runtime.fill_job_accepted_count = u64::from(preview.accepted_count);
    ctx.runtime.fill_job_search_count = preview.search_count;
    ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
}

fn close_session_work_one(node: &mut FillSessionNode) {
    match &mut node.work {
        FillWork::Session(session) => {
            session.begin_close();
            if matches!(session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::WorkerJobCloseStep::Complete) && session.terminal_is_empty() {
                node.work = FillWork::Empty;
            }
        }
        FillWork::Rejected(rejected) => {
            rejected.begin_close();
            if matches!(rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && rejected.terminal_is_empty() {
                node.work = FillWork::Empty;
            }
        }
        FillWork::Detached(job) => {
            semio_framework_job::InteractiveJob::begin_close(job);
            if matches!(semio_framework_job::InteractiveJob::close_step(job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && semio_framework_job::InteractiveJob::terminal_is_empty(job) {
                node.work = FillWork::Empty;
            }
        }
        FillWork::AwaitingSnapshot | FillWork::Empty => {}
    }
}

pub fn begin_fill_job(ctx: &mut Puzzle2dFillActionCtx<'_>, count: u32, seed: u64) {
    let abandonment_pending = pump_abandoned_session(None);
    if count > fill::PUZZLE2D_FILL_COUNT_MAX {
        fill_fault(ctx, "puzzle2d-fill-count-capacity");
        if abandonment_pending {
            queue_fill_discard(ctx);
        }
        return;
    }
    let Some(authority) = action_authority(ctx).cloned() else {
        fill_fault(ctx, "puzzle2d-fill-operation-authority");
        return;
    };
    let Some(slot) = reserve_session_slot() else {
        fill_fault(ctx, "puzzle2d-fill-session-capacity");
        if abandonment_pending {
            queue_fill_discard(ctx);
        }
        return;
    };
    let Some(backing) = FillSessionBacking::try_new() else {
        fill_fault(ctx, "puzzle2d-fill-session-backing");
        return;
    };
    let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(authority.operation_id), semio_framework_job::RevisionId(base_revision(&authority)), semio_framework_job::Generation(authority.generation), seed);
    let cancel = semio_framework_job::root_cancel_token();
    let node = backing.write(FillSessionNode {
        app_instance_id: authority.app_instance_id,
        operation,
        canonical_base_revision: authority.canonical_base_revision,
        maximum_count: count,
        cancel,
        work: FillWork::AwaitingSnapshot,
        retained_outcome: None,
        outcome_terminal: false,
        terminal_published: false,
        checkpoint: None,
        apply: None,
        checkpoint_sequence: 0,
        terminal: None,
        closing: false,
    });
    publish_session(slot, node);
    ctx.runtime.fill_job_operation = operation.operation.0;
    ctx.runtime.fill_job_generation = operation.generation.0;
    ctx.runtime.fill_job_seed = seed;
    ctx.runtime.fill_job_base_revision = operation.base_revision.0;
    ctx.runtime.fill_job_checkpoint_sequence = 0;
    ctx.runtime.fill_job_accepted_count = 0;
    ctx.runtime.fill_job_search_count = 0;
    let Some(stage) = crate::editor::puzzle2d::config::Puzzle2dFillText::try_from_str("capture") else {
        fill_fault(ctx, "puzzle2d-fill-stage-capacity");
        return;
    };
    ctx.runtime.fill_job_stage = stage;
    ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Capturing;
    ctx.runtime.fill_job_fault_code = None;
    queue_fill_step(ctx);
}

fn apply_checkpoint_step(ctx: &mut Puzzle2dFillActionCtx<'_>, node: &mut FillSessionNode) {
    if node.apply.is_none() {
        let Some(checkpoint) = node.checkpoint.as_mut() else {
            fill_fault(ctx, "puzzle2d-fill-checkpoint-owner");
            node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-checkpoint-owner"));
            node.begin_close();
            queue_fill_discard(ctx);
            return;
        };
        if let Some(placement) = checkpoint.take_pending_placement() {
            node.apply = Some(FillPlacementApplyCursor::new(placement));
        }
    }
    if let Some(apply) = node.apply.as_mut() {
        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Applying;
        match apply.step(ctx.artifact_mutations) {
            Ok(FillPlacementApplyStep::Pending) => {
                queue_fill_adopt(ctx);
                return;
            }
            Ok(FillPlacementApplyStep::Complete) => node.apply = None,
            Err(code) => {
                node.terminal = Some(FillTerminal::Fault(code));
                fill_fault(ctx, code);
                node.begin_close();
                queue_fill_discard(ctx);
                return;
            }
        }
    }
    let Some(checkpoint) = node.checkpoint.take() else {
        node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-checkpoint-owner"));
        fill_fault(ctx, "puzzle2d-fill-checkpoint-owner");
        node.begin_close();
        queue_fill_discard(ctx);
        return;
    };
    let FillWork::Session(session) = &mut node.work else {
        node.checkpoint = Some(checkpoint);
        node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-checkpoint-session"));
        fill_fault(ctx, "puzzle2d-fill-checkpoint-session");
        node.begin_close();
        queue_fill_discard(ctx);
        return;
    };
    let Some(job) = session.checked_out_job_mut() else {
        node.checkpoint = Some(checkpoint);
        queue_fill_adopt(ctx);
        return;
    };
    if let Err(checkpoint) = job.adopt_checkpoint(checkpoint) {
        node.checkpoint = Some(checkpoint);
        node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-checkpoint-stale"));
        fill_fault(ctx, "puzzle2d-fill-checkpoint-stale");
        node.begin_close();
        queue_fill_discard(ctx);
        return;
    }
    if session.resume().is_err() {
        node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-checkpoint-resume"));
        fill_fault(ctx, "puzzle2d-fill-checkpoint-resume");
        node.begin_close();
        queue_fill_discard(ctx);
        return;
    }
    ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
    queue_fill_step(ctx);
}

fn pump_fill_worker(ctx: &mut Puzzle2dFillActionCtx<'_>, node: &mut FillSessionNode) {
    let FillWork::Session(session) = &mut node.work else { return };
    let pool = fill_worker_pool();
    let poll = session.pump_one(&pool, semio_framework_async::Lane::Interactive);
    match poll {
        Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => {
            let Some(outcome) = session.take_checked_out_outcome() else {
                node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-outcome-owner"));
                fill_fault(ctx, "puzzle2d-fill-outcome-owner");
                node.begin_close();
                queue_fill_discard(ctx);
                return;
            };
            node.outcome_terminal = outcome.is_terminal();
            match &outcome {
                semio_framework_job::StepOutcome::PreviewReady(_) => {
                    if let Some(preview) = session.checked_out_job_mut().and_then(ArtifactBoardFillJob::take_preview) {
                        publish_preview(ctx, preview);
                    }
                }
                semio_framework_job::StepOutcome::CheckpointReady(_) => match session.checked_out_job_mut().and_then(ArtifactBoardFillJob::take_checkpoint) {
                    Some(checkpoint) => match node.checkpoint_sequence.checked_add(1) {
                        Some(sequence) => {
                            node.checkpoint_sequence = sequence;
                            ctx.runtime.fill_job_checkpoint_sequence = sequence;
                            ctx.runtime.fill_job_accepted_count = u64::from(checkpoint.accepted_count());
                            node.checkpoint = Some(checkpoint);
                            ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::CheckpointReady;
                        }
                        None => {
                            node.checkpoint = Some(checkpoint);
                            node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-checkpoint-sequence"));
                            fill_fault(ctx, "puzzle2d-fill-checkpoint-sequence");
                            node.begin_close();
                        }
                    },
                    None => {
                        node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-checkpoint-missing"));
                        fill_fault(ctx, "puzzle2d-fill-checkpoint-missing");
                        node.begin_close();
                    }
                },
                semio_framework_job::StepOutcome::Complete(candidate) => match publish_commit_candidate(candidate, ctx.artifact_mutations) {
                    Some(Ok(result)) => {
                        ctx.runtime.fill_job_accepted_count = u64::from(result.accepted_count);
                        ctx.runtime.fill_job_search_count = result.search_count;
                        node.terminal = Some(FillTerminal::Completed(result));
                        node.terminal_published = true;
                        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::AwaitingAdoption;
                    }
                    Some(Err(_)) => {
                        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Applying;
                    }
                    None => {
                        node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-result-missing"));
                        node.terminal_published = true;
                        fill_fault(ctx, "puzzle2d-fill-result-missing");
                    }
                },
                semio_framework_job::StepOutcome::Cancelled => {
                    node.terminal = Some(FillTerminal::Cancelled);
                    ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::AwaitingAdoption;
                }
                semio_framework_job::StepOutcome::Fault(_) => {
                    let code = match session.checked_out_job_mut().and_then(ArtifactBoardFillJob::take_fault) {
                        Some(code) => code,
                        None => "puzzle2d-fill-worker-fault",
                    };
                    node.terminal = Some(FillTerminal::Fault(code));
                    fill_fault(ctx, code);
                }
                semio_framework_job::StepOutcome::Yield => {
                    ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
                }
            }
            node.retained_outcome = Some(outcome);
            if node.outcome_terminal || node.checkpoint.is_some() {
                queue_fill_adopt(ctx);
            } else {
                queue_fill_step(ctx);
            }
        }
        Ok(semio_framework_job::WorkerJobPoll::Submitted | semio_framework_job::WorkerJobPoll::Rejected) => {
            ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Queued;
            queue_fill_step(ctx);
        }
        Ok(semio_framework_job::WorkerJobPoll::Closing) => {
            close_session_work_one(node);
            queue_fill_discard(ctx);
        }
        Ok(semio_framework_job::WorkerJobPoll::TerminalEmpty) => {
            node.work = FillWork::Empty;
            queue_fill_adopt(ctx);
        }
        Ok(_) => queue_fill_step(ctx),
        Err(semio_framework_job::MountedWorkerJobPumpFault::Submit(_)) => {
            ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Queued;
            queue_fill_step(ctx);
        }
        Err(_) => {
            node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-worker-contention"));
            fill_fault(ctx, "puzzle2d-fill-worker-contention");
            node.begin_close();
            queue_fill_discard(ctx);
        }
    }
}

pub fn step_fill_job(ctx: &mut Puzzle2dFillActionCtx<'_>, expected_generation: Option<u64>) {
    let active = action_authority(ctx).map(|authority| (authority.app_instance_id, authority.operation_id, authority.generation));
    if expected_generation.is_some_and(|generation| generation != ctx.runtime.fill_job_generation) {
        pump_abandoned_session(active);
        return;
    }
    let Some(authority) = action_authority(ctx).cloned() else {
        pump_abandoned_session(None);
        fill_fault(ctx, "puzzle2d-fill-operation-authority");
        return;
    };
    pump_abandoned_session(Some((authority.app_instance_id, authority.operation_id, authority.generation)));
    let Some(mut guard) = take_matching_session(authority.app_instance_id, authority.operation_id, authority.generation) else { return };
    let Some(node) = guard.node_mut() else { return };
    if !is_fresh(ctx, node) {
        node.begin_close();
        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Closing;
        if node.close_step() {
            guard.retire();
        } else {
            queue_fill_discard(ctx);
        }
        return;
    }
    if node.closing {
        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Closing;
        if node.close_step() {
            guard.retire();
            ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Discarded;
        } else {
            queue_fill_discard(ctx);
        }
        return;
    }
    if let Some(outcome) = node.retained_outcome.as_mut() {
        if node.outcome_terminal && !node.terminal_published {
            if let semio_framework_job::StepOutcome::Complete(candidate) = outcome {
                match publish_commit_candidate(candidate, ctx.artifact_mutations) {
                    Some(Ok(result)) => {
                        ctx.runtime.fill_job_accepted_count = u64::from(result.accepted_count);
                        ctx.runtime.fill_job_search_count = result.search_count;
                        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::AwaitingAdoption;
                        node.terminal = Some(FillTerminal::Completed(result));
                        node.terminal_published = true;
                    }
                    Some(Err(_)) => {
                        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Applying;
                        queue_fill_adopt(ctx);
                        return;
                    }
                    None => {
                        node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-result-missing"));
                        node.terminal_published = true;
                        fill_fault(ctx, "puzzle2d-fill-result-missing");
                    }
                }
            }
        }
        if !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            queue_fill_adopt(ctx);
            return;
        }
        node.retained_outcome = None;
        if node.outcome_terminal {
            node.outcome_terminal = false;
            node.terminal_published = false;
            close_session_work_one(node);
            queue_fill_adopt(ctx);
            return;
        }
        if node.checkpoint.is_some() {
            queue_fill_adopt(ctx);
            return;
        }
        let FillWork::Session(session) = &mut node.work else {
            node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-resume-session"));
            fill_fault(ctx, "puzzle2d-fill-resume-session");
            node.begin_close();
            queue_fill_discard(ctx);
            return;
        };
        if session.resume().is_err() {
            node.terminal = Some(FillTerminal::Fault("puzzle2d-fill-resume-contention"));
            fill_fault(ctx, "puzzle2d-fill-resume-contention");
            node.begin_close();
            queue_fill_discard(ctx);
        } else {
            queue_fill_step(ctx);
        }
        return;
    }
    if node.checkpoint.is_some() || node.apply.is_some() {
        apply_checkpoint_step(ctx, node);
        return;
    }
    match &mut node.work {
        FillWork::AwaitingSnapshot => ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Capturing,
        FillWork::Session(_) => pump_fill_worker(ctx, node),
        FillWork::Rejected(_) => {
            node.begin_close();
            queue_fill_discard(ctx);
        }
        FillWork::Detached(_) => {
            node.begin_close();
            queue_fill_discard(ctx);
        }
        FillWork::Empty => queue_fill_adopt(ctx),
    }
}

pub fn adopt_fill_job(ctx: &mut Puzzle2dFillActionCtx<'_>, expected_generation: Option<u64>) {
    if expected_generation.is_some_and(|generation| generation != ctx.runtime.fill_job_generation) {
        pump_abandoned_session(None);
        return;
    }
    let Some(authority) = action_authority(ctx).cloned() else { return };
    let Some(mut guard) = take_matching_session(authority.app_instance_id, ctx.runtime.fill_job_operation, ctx.runtime.fill_job_generation) else { return };
    let Some(node) = guard.node_mut() else { return };
    if node.checkpoint.is_some() || node.apply.is_some() || node.retained_outcome.is_some() {
        drop(guard);
        step_fill_job(ctx, expected_generation);
        return;
    }
    let Some(terminal) = node.terminal.take() else {
        close_session_work_one(node);
        return;
    };
    match terminal {
        FillTerminal::Completed(result) => {
            ctx.runtime.fill_job_accepted_count = u64::from(result.accepted_count);
            ctx.runtime.fill_job_search_count = result.search_count;
            ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Completed;
            ctx.runtime.fill_job_fault_code = None;
        }
        FillTerminal::Cancelled => {
            ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Cancelled;
            ctx.runtime.fill_job_fault_code = None;
        }
        FillTerminal::Fault(code) => fill_fault(ctx, code),
    }
    node.begin_close();
    queue_fill_discard(ctx);
}

pub fn cancel_fill_job(ctx: &mut Puzzle2dFillActionCtx<'_>, expected_generation: Option<u64>) {
    if expected_generation.is_some_and(|generation| generation != ctx.runtime.fill_job_generation) {
        pump_abandoned_session(None);
        return;
    }
    let Some(authority) = action_authority(ctx).cloned() else { return };
    let Some(mut guard) = take_matching_session(authority.app_instance_id, ctx.runtime.fill_job_operation, ctx.runtime.fill_job_generation) else { return };
    let Some(node) = guard.node_mut() else { return };
    node.cancel.cancel_now();
    node.terminal = Some(FillTerminal::Cancelled);
    node.begin_close();
    ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Closing;
    queue_fill_discard(ctx);
}

pub fn discard_fill_job(ctx: &mut Puzzle2dFillActionCtx<'_>, expected_generation: Option<u64>) {
    let Some(authority) = action_authority(ctx).cloned() else {
        if pump_abandoned_session(None) {
            queue_fill_discard(ctx);
        }
        return;
    };
    let generation = match expected_generation {
        Some(generation) => generation,
        None => ctx.runtime.fill_job_generation,
    };
    let Some(mut guard) = take_matching_session(authority.app_instance_id, ctx.runtime.fill_job_operation, generation) else {
        if pump_abandoned_session(None) {
            queue_fill_discard(ctx);
        }
        return;
    };
    let Some(node) = guard.node_mut() else { return };
    node.begin_close();
    if node.close_step() {
        guard.retire();
        if generation == ctx.runtime.fill_job_generation {
            ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Discarded;
        }
    } else {
        ctx.runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Closing;
        queue_fill_discard(ctx);
    }
}

pub fn retry_fill_job(ctx: &mut Puzzle2dFillActionCtx<'_>) {
    let count = ctx.runtime.fill_count;
    let seed = ctx.runtime.fill_job_seed;
    begin_fill_job(ctx, count, seed);
}

/// 🪣️ Activates fill and starts one persistent, generation-tagged job session.
pub fn set_fill_count(ctx: &mut Puzzle2dFillActionCtx<'_>, args: Option<&Value>) {
    let Some(value) = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()) else {
        fill_fault(ctx, "puzzle2d-fill-count");
        return;
    };
    if !value.is_finite() || value < 0.0 || value.round() > f64::from(fill::PUZZLE2D_FILL_COUNT_MAX) {
        fill_fault(ctx, "puzzle2d-fill-count-capacity");
        return;
    }
    let count = value.round() as u32;
    ctx.runtime.fill_count = count;
    ctx.effects.push(Effect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
    begin_fill_job(ctx, count, 1);
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    static REGISTRY_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn empty_node(operation: u64, generation: u64) -> Box<FillSessionNode> {
        FillSessionBacking::try_new().expect("registry node backing").write(FillSessionNode {
            app_instance_id: 7,
            operation: semio_framework_job::Operation::new(semio_framework_job::OperationId(operation), semio_framework_job::RevisionId(11), semio_framework_job::Generation(generation), 3),
            canonical_base_revision: [9; 32],
            maximum_count: 0,
            cancel: semio_framework_job::root_cancel_token(),
            work: FillWork::Empty,
            retained_outcome: None,
            outcome_terminal: false,
            terminal_published: false,
            checkpoint: None,
            apply: None,
            checkpoint_sequence: 0,
            terminal: None,
            closing: true,
        })
    }

    /// 🧮️ The fixed session authority admits MAX exact reservations and refuses MAX+1 before owner construction.
    #[test]
    fn fill_registry_max_and_max_plus_one_are_exact() {
        let _serial = REGISTRY_TEST_LOCK.lock().expect("registry test lock");
        assert!(!registry_has_sessions());
        let mut reservations: [Option<FillSessionReservation>; FILL_SESSION_CAPACITY] = std::array::from_fn(|_| None);
        for reservation in &mut reservations {
            *reservation = reserve_session_slot();
            assert!(reservation.is_some());
        }
        assert!(reserve_session_slot().is_none());
        drop(reservations);
        assert!(!registry_has_sessions());
        let unwind = std::panic::catch_unwind(|| {
            let _reservation = reserve_session_slot().expect("panic reservation");
            panic!("hostile pre-publication panic");
        });
        assert!(unwind.is_err());
        assert!(!registry_has_sessions());
    }

    /// 🛟️ Panic-unwound and lost guards republish the exact generation-qualified owner for later retirement.
    #[test]
    fn fill_registry_guard_drop_recovers_exact_owner() {
        let _serial = REGISTRY_TEST_LOCK.lock().expect("registry test lock");
        assert!(!registry_has_sessions());
        let slot = reserve_session_slot().expect("registry reservation");
        publish_session(slot, empty_node(31, 5));
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = take_matching_session(7, 31, 5).expect("exact checked-out owner");
            panic!("hostile lost handle");
        }));
        assert!(unwind.is_err());
        assert!(take_matching_session(7, 31, 6).is_none());
        let guard = take_matching_session(7, 31, 5).expect("republished exact owner");
        guard.retire();
        assert!(!registry_has_sessions());
    }

    /// 🪪️ The terminal generation is an exact identity and cannot alias generation zero.
    #[test]
    fn fill_registry_terminal_generation_does_not_wrap_or_alias() {
        let _serial = REGISTRY_TEST_LOCK.lock().expect("registry test lock");
        assert!(!registry_has_sessions());
        let slot = reserve_session_slot().expect("registry reservation");
        publish_session(slot, empty_node(41, u64::MAX));
        assert!(take_matching_session(7, 41, 0).is_none());
        let guard = take_matching_session(7, 41, u64::MAX).expect("terminal generation owner");
        guard.retire();
        assert!(!registry_has_sessions());
    }

    fn mounted_fill_source_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(context_start) = production.find("pub struct Puzzle2dFillActionCtx") else { return false };
        let Some(context_end_relative) = production[context_start..].find("enum FillTerminal") else { return false };
        let context = &production[context_start..context_start + context_end_relative];
        let Some(worker_start) = production.find("impl semio_framework_job::InteractiveJob for ArtifactBoardFillJob") else { return false };
        let Some(worker_end) = production[worker_start..].find("impl Drop for ArtifactBoardFillJob") else { return false };
        let worker = &production[worker_start..worker_start + worker_end];
        context.contains("Puzzle2dFillRuntime")
            && context.contains("boundary_fault")
            && !context.contains("Puzzle2dConfig")
            && !context.contains("Vec<Value>")
            && !context.contains("BTreeMap")
            && !context.contains("String")
            && production.contains("snapshot: Option<store::SnapshotRead<Puzzle2dPlaySnapshot>>")
            && worker.contains("context.consume_fuel(1)")
            && worker.contains("commit_authority_matches(self.render_generation, self.canonical_base_revision)")
            && production.contains("semio_framework_job::MountedWorkerJobSession::try_new(job, params)")
            && production.contains("semio_framework_async::process_worker_pool")
            && production.contains("semio_framework_job::StepOutcome::Complete(candidate)")
            && production.contains("infinite_canvas::BoardFillCommitCandidate::from_commit_candidate(candidate)")
            && production.matches("publish_commit_candidate(candidate, ctx.artifact_mutations)").count() == 2
            && production.contains("terminal_published")
            && !production.contains("BoardFillJob::take_result")
            && !production.contains("FillWork::Capture")
    }

    fn fixed_placement_owner_source_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(start) = production.find("struct FillSessionNode") else { return false };
        let Some(end) = production[start..].find("fn try_document_str") else { return false };
        let owners = &production[start..start + end];
        let Some(apply_start) = production.find("impl FillPlacementApplyCursor") else { return false };
        let Some(apply_end) = production[apply_start..].find("impl Drop for FillPlacementApplyCursor") else { return false };
        let apply = &production[apply_start..apply_start + apply_end];
        let Some(view_start) = production.find("enum FillPlacementPublishHandles") else { return false };
        let Some(view_end) = production[view_start..].find("fn publish_fixed_placement") else { return false };
        let view = &production[view_start..view_start + view_end];
        owners.contains("edge_kind: infinite_canvas::BoardFillText")
            && owners.contains("text_byte: usize")
            && owners.contains("fn copy_fill_text_one")
            && owners.contains("destination.try_push_byte(value)")
            && !owners.contains("String")
            && !owners.contains("Vec<")
            && !owners.contains("BTreeMap")
            && !owners.contains("Puzzle2dNode")
            && !owners.contains("Puzzle2dHandle")
            && apply.matches("copy_fill_text_one(").count() == 9
            && apply.contains("fixed_handle_id(self.handle_cursor)")
            && apply.contains("fixed_handle_kind(self.handle_cursor)")
            && apply.contains("FillPlacementPublishView::from_cursor")
            && !apply.contains("let placement = infinite_canvas::BoardFillCommitPlacement")
            && !apply.contains("handles: std::array::from_fn(|index|")
            && !apply.contains("= placement.node_id;")
            && !apply.contains("= placement.edge_kind;")
            && !apply.contains(".to_string(")
            && view.contains("edge_kind: &'a infinite_canvas::BoardFillText")
            && view.contains("FillPlacementPublishHandles::Commit(&placement.handles)")
            && view.contains("FillPlacementPublishHandles::Cursor(handles)")
            && !view.contains("String")
            && !view.contains("Vec<")
            && !view.contains("BTreeMap")
    }

    fn full_terminal_candidate_source_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(start) = production.find("fn publish_commit_candidate") else { return false };
        let Some(end) = production[start..].find("impl FillPlacementApplyCursor") else { return false };
        let publish = &production[start..start + end];
        publish.contains("BoardFillCommitCandidate::from_commit_candidate(candidate)")
            && publish.contains("if let Some(placement) = candidate.placement.as_ref()")
            && publish.contains("publish_fixed_placement(FillPlacementPublishView::from_commit(placement), mutations)")
            && publish.contains("Some(Ok(candidate.result))")
            && !publish.contains("BoardFillResult::from_commit_candidate")
            && !publish.contains("take_result")
    }

    fn granular_capture_source_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(start) = production.find("fn capture_node_one") else { return false };
        let Some(end) = production[start..].find("fn take_preview") else { return false };
        let capture = &production[start..start + end];
        capture.matches(".get(").count() == 57
            && capture.matches("as_bytes().get(self.capture.byte)").count() == 7
            && capture.matches("self.ingress").count() == 39
            && capture.matches(".push_node_id_byte(").count() == 1
            && capture.matches(".push_handle_text_byte(").count() == 2
            && capture.matches(".push_kind_text_byte(").count() == 2
            && capture.matches(".push_kind_handle_text_byte(").count() == 1
            && capture.matches(".push_rule_text_byte(").count() == 1
            && !capture.contains("for ")
            && !capture.contains(".iter(")
            && !capture.contains(".clone(")
            && !capture.contains(".to_string(")
            && !capture.contains(".push_node(")
            && !capture.contains(".push_handle(")
            && !capture.contains(".push_rule(")
    }

    fn placement_publish_source_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(start) = production.find("fn publish_fixed_placement") else { return false };
        let Some(end) = production[start..].find("fn publish_commit_candidate") else { return false };
        let publish = &production[start..start + end];
        let Some(reserve) = publish.find("mutations.try_reserve_exact(2)") else { return false };
        let Some(handles) = publish.find("let mut handles = Vec::new()") else { return false };
        let Some(node) = publish.find("let node = crate::artifacts::puzzle2d::Puzzle2dNode") else { return false };
        reserve < handles && handles < node && publish.contains("Some(try_document_text(*placement.edge_kind)?)") && publish.matches("mutations.push(").count() == 2 && !production.contains("ReserveMutations")
    }

    /// 🧵️ Removing worker-owned capture or restoring mutable terminal rereads fails the live source law.
    #[test]
    fn mounted_fill_worker_and_terminal_mutations_are_rejected() {
        let source = include_str!("🦀️.rs");
        assert!(mounted_fill_source_contract(source));
        let ui_capture = source.replacen("impl semio_framework_job::InteractiveJob for ArtifactBoardFillJob", "impl semio_framework_job::InteractiveJob for RemovedArtifactBoardFillJob", 1);
        assert!(!mounted_fill_source_contract(&ui_capture));
        let mutable_terminal = source.replacen("infinite_canvas::BoardFillCommitCandidate::from_commit_candidate(candidate)", "job.take_result()", 1);
        assert!(!mounted_fill_source_contract(&mutable_terminal));
        let dynamic_runtime = source.replacen("pub runtime: &'a mut crate::editor::puzzle2d::config::Puzzle2dFillRuntime,", "pub runtime: &'a mut crate::editor::puzzle2d::config::Puzzle2dFillRuntime, pub dynamic: Vec<Value>,", 1);
        assert!(!mounted_fill_source_contract(&dynamic_runtime));
    }

    /// 🧷️ Injected dynamic retained placement text and re-coalesced fixed text both fail the mounted ownership law.
    #[test]
    fn mounted_fill_fixed_placement_owner_mutations_are_rejected() {
        let source = include_str!("🦀️.rs");
        assert!(fixed_placement_owner_source_contract(source));
        let dynamic = source.replacen("edge_kind: infinite_canvas::BoardFillText,", "edge_kind: String,", 1);
        assert!(!fixed_placement_owner_source_contract(&dynamic));
        let dynamic_view = source.replacen("edge_kind: &'a infinite_canvas::BoardFillText,", "edge_kind: String,", 1);
        assert!(!fixed_placement_owner_source_contract(&dynamic_view));
        let whole_text = source.replacen("if copy_fill_text_one(&placement.edge_kind, destination, &mut self.text_byte)? {", "if { *destination = placement.edge_kind; true } {", 1);
        assert!(!fixed_placement_owner_source_contract(&whole_text));
        let whole_candidate =
            source.replacen("FillPlacementPublishView::from_cursor(node, edge, &self.handles, self.handle_cursor)", "infinite_canvas::BoardFillCommitPlacement { handles: std::array::from_fn(|index| self.handles[index]), ..Default::default() }", 1);
        assert!(!fixed_placement_owner_source_contract(&whole_candidate));
    }

    /// 🔡️ A MAX fixed label advances by exactly one character byte per retained apply opportunity.
    #[test]
    fn mounted_fill_fixed_placement_text_cursor_is_one_byte_per_turn() {
        let source = "x".repeat(infinite_canvas::BOARD_FILL_TEXT_BYTES);
        let source = infinite_canvas::BoardFillText::try_from_str(&source).expect("MAX fixed placement text");
        let mut destination = infinite_canvas::BoardFillText::empty();
        let mut byte = 0usize;
        for index in 0..infinite_canvas::BOARD_FILL_TEXT_BYTES {
            let complete = copy_fill_text_one(&source, &mut destination, &mut byte).expect("fixed byte copy");
            assert_eq!(destination.as_str().len(), index + 1);
            assert_eq!(complete, index + 1 == infinite_canvas::BOARD_FILL_TEXT_BYTES);
        }
        assert_eq!(destination, source);
        let over = "x".repeat(infinite_canvas::BOARD_FILL_TEXT_BYTES + 1);
        assert!(infinite_canvas::BoardFillText::try_from_str(&over).is_err());
    }

    /// 📦️ Replacing the full exact terminal placement with a summary decoder fails the live terminal law.
    #[test]
    fn mounted_fill_summary_only_terminal_mutation_is_rejected() {
        let source = include_str!("🦀️.rs");
        assert!(full_terminal_candidate_source_contract(source));
        let summary = source.replacen("BoardFillCommitCandidate::from_commit_candidate(candidate)", "BoardFillResult::from_commit_candidate(candidate)", 1);
        assert!(!full_terminal_candidate_source_contract(&summary));
        let discarded = source.replacen("if let Some(placement) = candidate.placement.as_ref() {", "if let Some(placement) = None {", 1);
        assert!(!full_terminal_candidate_source_contract(&discarded));
    }

    /// 🔬️ Re-coalescing source fields or whole text into one worker grant fails the live capture law.
    #[test]
    fn mounted_fill_capture_granularity_mutations_are_rejected() {
        let source = include_str!("🦀️.rs");
        assert!(granular_capture_source_contract(source));
        let fields = source.replacen(
            "self.capture.node_x = Self::finite(node.get(\"x\").and_then(Value::as_f64), 0.0);",
            "self.capture.node_x = Self::finite(node.get(\"x\").and_then(Value::as_f64), 0.0); self.capture.node_y = Self::finite(node.get(\"y\").and_then(Value::as_f64), 0.0);",
            1,
        );
        assert!(!granular_capture_source_contract(&fields));
        let text = source.replacen("if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {", "for byte in value.as_bytes().iter().copied() {", 1);
        assert!(!granular_capture_source_contract(&text));
    }

    /// 🪪️ Placement publication pre-credits its live destination in the same bounded continuation.
    #[test]
    fn placement_publish_credit_mutation_is_rejected() {
        let source = include_str!("🦀️.rs");
        assert!(placement_publish_source_contract(source));
        let uncredited = source.replacen("mutations.try_reserve_exact(2)", "mutations.capacity().checked_add(2)", 1);
        assert!(!placement_publish_source_contract(&uncredited));
    }

    /// 🗂️ Fill's kind capture reads the document's own `meta.kindCatalogs.nodes` slice. It used to read
    /// `nodeKinds` — the board *engine's* spelling, which the puzzle2d document schema forbids
    /// (`additionalProperties: false` over `nodes`/`🐙️handles`/`edges`/`wires`) — so capture failed on
    /// every run with `puzzle2d-fill-capture-node-kinds` and the job went straight to `Faulted`.
    #[test]
    fn fill_capture_reads_the_document_node_kind_slice() {
        let document = serde_json::json!({
            "meta": {
                "kindCatalogs": {
                    "nodes": [{ "id": "seed", "name": "Seed", "label": "Seed", "handles": [] }],
                    "handles": [],
                    "edges": [],
                    "wires": []
                }
            }
        });
        let kinds = ArtifactBoardFillJob::node_kinds(&document).expect("document node-kind slice");
        assert_eq!(kinds.len(), 1);
        assert_eq!(kinds[0].get("id").and_then(Value::as_str), Some("seed"));

        let engine_shaped = serde_json::json!({ "meta": { "kindCatalogs": { "nodeKinds": [{ "id": "seed" }] } } });
        assert_eq!(
            ArtifactBoardFillJob::node_kinds(&engine_shaped).err(),
            Some("puzzle2d-fill-capture-node-kinds"),
            "the engine's `nodeKinds` spelling must not satisfy fill's document read, else this guard proves nothing"
        );
    }
}
//#endregion 🧪️Tests
