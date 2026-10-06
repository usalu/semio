//! 🧩️ Puzzle-owned projection into the framework's neutral board scene contract.

use semio_framework_value::{DslValue, ErasedSnapshotRetirement, NativeDecodeControl, ValueError, ValueRefusalKind};

/// 🧭️ Endpoint semantics selected by the owning Puzzle presentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode { Normal, Ported }

/// 🧺️ Retains every partially constructed record until transfer or explicit retirement.
pub struct Projection {
    nodes: Vec<DslValue>, handles: Vec<DslValue>, edges: Vec<DslValue>, regions: Vec<DslValue>,
    fields: Vec<(String, DslValue)>, scene: Option<DslValue>, started: bool,
}

impl Projection {
    /// 🈳️ Starts without heap allocation or domain dependencies on individual artifacts.
    pub fn new() -> Self { Self { nodes: Vec::new(), handles: Vec::new(), edges: Vec::new(), regions: Vec::new(), fields: Vec::new(), scene: None, started: false } }

    /// 📤️ Transfers a completed neutral scene; a refused projection retains its partial owner.
    pub fn take_scene(&mut self) -> Option<DslValue> { self.scene.take() }

    /// ♻️ Transfers both completed and partial owners to the framework's explicit retirement cursor.
    pub fn take_retirement(&mut self) -> Box<dyn ErasedSnapshotRetirement> {
        let owners = ((std::mem::take(&mut self.nodes), std::mem::take(&mut self.handles), std::mem::take(&mut self.edges), std::mem::take(&mut self.regions)), std::mem::take(&mut self.fields), self.scene.take());
        semio_framework_value::retirement::owned_retirement(owners)
    }

    /// 🚦️ Projects with cumulative allocation admission, progress and cancellation.
    pub fn project(&mut self, input: &DslValue, mode: Mode, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        if self.started { return Err(invalid("projection has already started")); }
        self.started = true;
        let nodes = array(input, "nodes")?;
        let edges = array(input, "edges")?;
        let regions = match input.get("targetRegions") { None => &[][..], Some(value) => value.as_array().ok_or_else(|| invalid("targetRegions must be an array"))? };
        let mut handle_count = 0usize;
        control.scoped_stage(|control| {
            control.begin_stage(nodes.len())?;
            for node in nodes {
                match (mode, node.get("handles")) {
                    (Mode::Ported, Some(value)) => handle_count = handle_count.checked_add(value.as_array().ok_or_else(|| invalid("ported nodes require handles"))?.len()).ok_or_else(|| invalid("handle count overflow"))?,
                    (Mode::Ported, None) => return Err(invalid("ported nodes require handles")),
                    (Mode::Normal, Some(_)) => return Err(invalid("normal nodes cannot contain handles")),
                    (Mode::Normal, None) => (),
                }
                control.step()?;
            }
            Ok::<_, ValueError>(())
        })?;
        self.nodes = control.allocate_vec(nodes.len())?;
        self.handles = control.allocate_vec(handle_count)?;
        self.edges = control.allocate_vec(edges.len())?;
        self.regions = control.allocate_vec(regions.len())?;
        control.scoped_stage(|control| {
            control.begin_stage(nodes.len().checked_add(handle_count).ok_or_else(|| invalid("scene workload overflow"))?)?;
            for node in nodes {
                self.begin_record(14, control)?;
                let id = required_text(node, "id")?;
                self.push_text("id", id, control)?;
                self.push_number("x", required_number(node, "x")?, control)?;
                self.push_number("y", required_number(node, "y")?, control)?;
                let rectangle = node.get("shape").and_then(DslValue::as_str) == Some("rectangle");
                self.push_text("shape", if rectangle { "rectangle" } else { "circle" }, control)?;
                for key in if rectangle { &["width", "height"][..] } else { &["radius"][..] } {
                    let value = required_number(node, key)?;
                    if value <= 0.0 { return Err(invalid("node dimensions must be positive")); }
                    self.push_number(key, value, control)?;
                }
                self.optional_text(node, &["text", "iconKind", "nodeKind"], control)?;
                self.optional_positive(node, &["scale"], control)?;
                self.visibility(node, control)?;
                self.optional_bool(node, &["locked", "root"], control)?;
                self.nodes.push(DslValue::Object(std::mem::take(&mut self.fields)));
                control.step()?;
                if mode == Mode::Ported {
                    for handle in array(node, "handles")? {
                        self.begin_record(10, control)?;
                        self.push_text("id", required_text(handle, "id")?, control)?;
                        self.push_text("nodeId", id, control)?;
                        self.push_number("angle", required_number(handle, "angle")?, control)?;
                        self.push_text("handleKind", trimmed_text(handle, "handleKind").unwrap_or("port"), control)?;
                        self.optional_text(handle, &["color", "iconKind"], control)?;
                        self.optional_positive(handle, &["scale", "radius"], control)?;
                        self.visibility(handle, control)?;
                        self.optional_bool(handle, &["locked"], control)?;
                        self.handles.push(DslValue::Object(std::mem::take(&mut self.fields)));
                        control.step()?;
                    }
                }
            }
            Ok::<_, ValueError>(())
        })?;
        let ordered = if mode == Mode::Normal { ordered_nodes(nodes, control)? } else { Vec::new() };
        control.scoped_stage(|control| {
            control.begin_stage(edges.len())?;
            for edge in edges {
                self.begin_record(8, control)?;
                self.push_text("id", required_text(edge, "id")?, control)?;
                for key in ["source", "target"] {
                    let endpoint = required_text(edge, key)?;
                    if mode == Mode::Normal && !contains_node(nodes, &ordered, endpoint, control)? { return Err(invalid("edge endpoint does not identify a node")); }
                    self.push_text(key, endpoint, control)?;
                }
                self.optional_text(edge, &["edgeKind", "sourceTip", "targetTip"], control)?;
                self.visibility(edge, control)?;
                self.optional_bool(edge, &["locked"], control)?;
                self.edges.push(DslValue::Object(std::mem::take(&mut self.fields)));
                control.step()?;
            }
            Ok::<_, ValueError>(())
        })?;
        control.scoped_stage(|control| {
            control.begin_stage(regions.len())?;
            for region in regions {
                if let Some(id) = trimmed_text(region, "id") {
                    self.begin_record(9, control)?;
                    self.push_text("id", id, control)?;
                    for key in ["x", "y", "width", "height"] { self.push_number(key, finite_number(region.get(key)).unwrap_or(0.0), control)?; }
                    self.optional_text(region, &["label"], control)?;
                    self.optional_bool(region, &["hidden", "locked", "selected"], control)?;
                    self.regions.push(DslValue::Object(std::mem::take(&mut self.fields)));
                }
                control.step()?;
            }
            Ok::<_, ValueError>(())
        })?;
        self.begin_record(6, control)?;
        self.push_text("mode", match mode { Mode::Normal => "normal", Mode::Ported => "ported" }, control)?;
        if let Some(camera) = input.get("camera") {
            let mut fields = control.allocate_vec(3)?;
            for key in ["x", "y", "zoom"] {
                let value = required_number(camera, key)?;
                fields.push((control.copy_text(key)?, DslValue::json_number(value)));
            }
            self.push("camera", DslValue::Object(fields), control)?;
        }
        for (key, lane) in [("nodes", 0), ("handles", 1), ("edges", 2), ("regions", 3)] {
            let key = control.copy_text(key)?;
            let rows = match lane { 0 => &mut self.nodes, 1 => &mut self.handles, 2 => &mut self.edges, _ => &mut self.regions };
            self.fields.push((key, DslValue::Array(std::mem::take(rows))));
        }
        control.checkpoint()?;
        self.scene = Some(DslValue::Object(std::mem::take(&mut self.fields)));
        Ok(())
    }

    fn begin_record(&mut self, capacity: usize, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { self.fields = control.allocate_vec(capacity)?; Ok(()) }
    fn push(&mut self, key: &str, value: DslValue, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { let key = control.copy_text(key)?; self.fields.push((key, value)); Ok(()) }
    fn push_text(&mut self, key: &str, text: &str, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { let key = control.copy_text(key)?; let text = control.copy_text(text)?; self.fields.push((key, DslValue::String(text))); Ok(()) }
    fn push_number(&mut self, key: &str, value: f64, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { self.push(key, DslValue::json_number(value), control) }
    fn optional_text(&mut self, row: &DslValue, keys: &[&str], control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { for key in keys { if let Some(value) = trimmed_text(row, key) { self.push_text(key, value, control)?; } } Ok(()) }
    fn optional_positive(&mut self, row: &DslValue, keys: &[&str], control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { for key in keys { if let Some(value) = finite_number(row.get(key)).filter(|value| *value > 0.0) { self.push_number(key, value, control)?; } } Ok(()) }
    fn optional_bool(&mut self, row: &DslValue, keys: &[&str], control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { for key in keys { if let Some(value) = row.get(key).and_then(DslValue::as_bool) { self.push(key, DslValue::Bool(value), control)?; } } Ok(()) }
    fn visibility(&mut self, row: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { if let Some(value) = row.get("hidden").and_then(DslValue::as_bool).map(|hidden| !hidden).or_else(|| row.get("visible").and_then(DslValue::as_bool)) { self.push("visible", DslValue::Bool(value), control)?; } Ok(()) }
}

impl Default for Projection { fn default() -> Self { Self::new() } }
impl Drop for Projection { fn drop(&mut self) { assert!(std::thread::panicking() || (self.nodes.is_empty() && self.handles.is_empty() && self.edges.is_empty() && self.regions.is_empty() && self.fields.is_empty() && self.scene.is_none()), "Puzzle projection retains ownership requiring explicit transfer or retirement"); } }

fn invalid(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
fn array<'a>(row: &'a DslValue, key: &str) -> Result<&'a [DslValue], ValueError> { row.get(key).and_then(DslValue::as_array).ok_or_else(|| invalid("required scene array is absent")) }
fn required_text<'a>(row: &'a DslValue, key: &str) -> Result<&'a str, ValueError> { row.get(key).and_then(DslValue::as_str).ok_or_else(|| invalid("required scene identifier is absent")) }
fn trimmed_text<'a>(row: &'a DslValue, key: &str) -> Option<&'a str> { row.get(key).and_then(DslValue::as_str).map(str::trim).filter(|text| !text.is_empty()) }
fn finite_number(value: Option<&DslValue>) -> Option<f64> { value.and_then(DslValue::as_f64).filter(|value| value.is_finite()) }
fn required_number(row: &DslValue, key: &str) -> Result<f64, ValueError> { finite_number(row.get(key)).ok_or_else(|| invalid("required finite scene coordinate is absent")) }

fn ordered_nodes(nodes: &[DslValue], control: &mut NativeDecodeControl<'_>) -> Result<Vec<usize>, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        let mut ordered = control.allocate_vec(nodes.len())?;
        let mut scratch = control.allocate_vec(nodes.len())?;
        for index in 0..nodes.len() { ordered.push(index); scratch.push(0); control.step()?; }
        let mut width = 1usize;
        while width < nodes.len() {
            let mut start = 0usize;
            while start < nodes.len() {
                let middle = start.saturating_add(width).min(nodes.len());
                let end = middle.saturating_add(width).min(nodes.len());
                let (mut left, mut right) = (start, middle);
                for destination in &mut scratch[start..end] {
                    if right == end || (left < middle && required_text(&nodes[ordered[left]], "id")? <= required_text(&nodes[ordered[right]], "id")?) { *destination = ordered[left]; left += 1; } else { *destination = ordered[right]; right += 1; }
                    control.step()?;
                }
                start = end;
            }
            std::mem::swap(&mut ordered, &mut scratch);
            width = width.saturating_mul(2);
        }
        Ok(ordered)
    })
}

fn contains_node(nodes: &[DslValue], ordered: &[usize], id: &str, control: &mut NativeDecodeControl<'_>) -> Result<bool, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        let (mut left, mut right) = (0, ordered.len());
        while left < right {
            let middle = left + (right - left) / 2;
            let found = match required_text(&nodes[ordered[middle]], "id")?.cmp(id) { std::cmp::Ordering::Less => { left = middle + 1; false }, std::cmp::Ordering::Greater => { right = middle; false }, std::cmp::Ordering::Equal => true };
            control.step()?;
            if found { return Ok(true); }
        }
        Ok(false)
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
