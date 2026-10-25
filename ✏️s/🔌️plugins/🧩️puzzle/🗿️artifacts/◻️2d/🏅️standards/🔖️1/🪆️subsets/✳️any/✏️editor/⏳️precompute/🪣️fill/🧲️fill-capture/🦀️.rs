//! 🧲️ Fill capture: the fill capture and its streaming of the document into the board fill ingress. See [`super`].

use super::*;

/// 🔬️ The staged document capture of one run: the field cursor, the engine ingress it feeds, the suggestion
/// offset every open handle's slot is pushed out by, the first engine serial past the document's fill ids and the
/// number of open handles it streamed.
pub(super) struct FillCapture {
    pub(super) capture: ArtifactFillCaptureCursor,
    pub(super) ingress: Option<BoardFillSnapshotIngress>,
    pub(super) suggestion_offset: f64,
    pub(super) serial: u64,
    pub(super) open_handles: u64,
}

impl FillCapture {
    pub(super) fn new(suggestion_offset: f64) -> Self {
        Self { capture: ArtifactFillCaptureCursor::new(), ingress: None, suggestion_offset, serial: 1, open_handles: 0 }
    }

    /// 🔬️ Up to `units` capture units; the captured snapshot once the document is fully streamed.
    pub(super) fn advance(&mut self, document: &Value, kinds: &[Value], units: usize) -> Result<Option<BoardFillSnapshot>, &'static str> {
        for _ in 0..units {
            if let Some(snapshot) = self.capture_one(document, kinds)? {
                return Ok(Some(snapshot));
            }
        }
        Ok(None)
    }

    /// 🚰️ One granted close turn of the ingress.
    pub(super) fn close_one(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        let Some(ingress) = self.ingress.as_mut() else { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }) };
        ingress.begin_close();
        let demand = self.close_demands()?;
        let ingress = self.ingress.as_mut().expect("original fill ingress remains staged");
        let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(match ingress.close_step(grant) {
            InteractiveJobCloseStep::Complete { progress } => semio_framework_value::RetainedCloneStep::Complete(progress),
            step => semio_framework_value::RetainedCloneStep::Progress(step.progress()),
        }))?;
        if ingress.terminal_is_empty() { self.ingress = None; }
        Ok(if self.ingress.is_none() { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
    }

    pub(super) fn is_closed(&self) -> bool {
        self.ingress.is_none()
    }

    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let Some(ingress) = self.ingress.as_ref() else { return Ok(Default::default()) };
        Ok(semio_framework_value::RetirementDemand { release_bytes: ingress.next_close_release_byte_demand(), depth: 1, ..Default::default() })
    }
}

//#region 🔬️Capture
/// 🔬️ Streams the document into the engine's fixed-capacity fill ingress one field — and, for text,
/// one byte — per call, so a capture never allocates and never outruns its step budget.
impl FillCapture {
    pub(super) fn nodes(document: &Value) -> Result<&[Value], &'static str> {
        document.get("nodes").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-nodes")
    }

    fn edges(document: &Value) -> Result<&[Value], &'static str> {
        document.get("edges").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-edges")
    }

    fn rules(document: &Value) -> Option<&[Value]> {
        document.get("meta").and_then(|meta| meta.get("kindCompatibility")).or_else(|| document.get("kindCompatibility")).and_then(Value::as_array).map(Vec::as_slice)
    }

    pub(super) fn finite(value: Option<f64>, default: f64) -> f64 {
        value.filter(|value| value.is_finite()).unwrap_or(default)
    }

    fn capture_node_one(&mut self, document: &Value) -> Result<(), &'static str> {
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
                    self.serial = self.serial.max(fill_serial_after(value));
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

    fn capture_handle_one(&mut self, document: &Value) -> Result<(), &'static str> {
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
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_handle_text_byte(BoardFillIngressHandleText::Id, byte).map_err(capture_fault_code)?;
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
                    ArtifactHandleCaptureField::NodeKind => (node.get("nodeKind").and_then(Value::as_str).unwrap_or(""), BoardFillIngressHandleText::NodeKind, ArtifactHandleCaptureField::HandleKind),
                    ArtifactHandleCaptureField::HandleKind => (handle.get("handleKind").and_then(Value::as_str).unwrap_or("port"), BoardFillIngressHandleText::HandleKind, ArtifactHandleCaptureField::WireKind),
                    ArtifactHandleCaptureField::WireKind => (handle.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), BoardFillIngressHandleText::WireKind, ArtifactHandleCaptureField::EdgeKind),
                    ArtifactHandleCaptureField::EdgeKind => (handle.get("edgeKind").and_then(Value::as_str).unwrap_or(""), BoardFillIngressHandleText::EdgeKind, ArtifactHandleCaptureField::X),
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
                self.open_handles += u64::from(self.capture.handle_node_visible && self.capture.handle_visible && !self.capture.handle_connected);
                self.capture.handle += 1;
                self.capture.handle_edge = 0;
                self.capture.handle_connected = false;
                self.capture.handle_field = ArtifactHandleCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_kind_one(&mut self, kinds: &[Value]) -> Result<(), &'static str> {
        let Some(kind) = kinds.get(self.capture.kind) else {
            if self.capture.kind_field != ArtifactKindCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-kind");
            }
            self.capture.stage = ArtifactFillCaptureStage::Rules;
            return Ok(());
        };
        let templates = kind.get("handles").and_then(Value::as_array).map_or(&[][..], Vec::as_slice);
        let template = templates.get(self.capture.template);
        match self.capture.kind_field {
            ArtifactKindCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind().map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Id;
            }
            ArtifactKindCaptureField::Id => {
                let value = kind.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-kind-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(BoardFillIngressKindText::Id, byte).map_err(capture_fault_code)?;
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
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(BoardFillIngressKindText::Icon, byte).map_err(capture_fault_code)?;
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
                        (template.get("handleKind").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-template-kind")?, BoardFillIngressTemplateText::HandleKind, ArtifactKindCaptureField::TemplateWireKind)
                    }
                    ArtifactKindCaptureField::TemplateWireKind => {
                        (template.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), BoardFillIngressTemplateText::WireKind, ArtifactKindCaptureField::TemplateEdgeKind)
                    }
                    ArtifactKindCaptureField::TemplateEdgeKind => (template.get("edgeKind").and_then(Value::as_str).unwrap_or(""), BoardFillIngressTemplateText::EdgeKind, ArtifactKindCaptureField::TemplateAngle),
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

    fn capture_rule_one(&mut self, document: &Value) -> Result<(), &'static str> {
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
                    ArtifactRuleCaptureField::Source => (rule.get("source").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-source")?, BoardFillIngressRuleText::Source, ArtifactRuleCaptureField::Target),
                    ArtifactRuleCaptureField::Target => {
                        (rule.get("target").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-target")?, BoardFillIngressRuleText::Target, ArtifactRuleCaptureField::Bidirectional)
                    }
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

    fn capture_one(&mut self, document: &Value, kinds: &[Value]) -> Result<Option<BoardFillSnapshot>, &'static str> {
        match self.capture.stage {
            ArtifactFillCaptureStage::Nodes => self.capture_node_one(document).map(|()| None),
            ArtifactFillCaptureStage::Handles => self.capture_handle_one(document).map(|()| None),
            ArtifactFillCaptureStage::Kinds => self.capture_kind_one(kinds).map(|()| None),
            ArtifactFillCaptureStage::Rules => self.capture_rule_one(document).map(|()| None),
            ArtifactFillCaptureStage::Complete => self.ingress.as_mut().and_then(BoardFillSnapshotIngress::take_snapshot).map(Some).ok_or("puzzle2d-fill-capture-snapshot"),
        }
    }
}
//#endregion 🔬️Capture
