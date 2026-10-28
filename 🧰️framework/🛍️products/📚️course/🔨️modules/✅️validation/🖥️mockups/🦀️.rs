//! 🖥️ Bounded software preview data and checked graph/grid references.
use super::{Issues, IssueCode};
use crate::schema::{GrasshopperTargetKind, Mockup, MockupEndpoint, MockupNode, MockupWidget, Text};

impl Issues {
    fn bounded(&mut self, path: String, size: usize, min: usize, max: usize) {
        self.at_least(path.clone(), size, min, IssueCode::ItemsTooFew);
        if size > max { self.push(path, IssueCode::ValueInvalid); }
    }

    fn cell(&mut self, base: &str, text: &Text) {
        self.length(format!("{base}/en"), &text.en, 0, 256);
        self.length(format!("{base}/de"), &text.de, 0, 256);
    }

    fn endpoint(&mut self, base: &str, endpoint: &MockupEndpoint, nodes: &[MockupNode], input: bool) {
        self.slug(format!("{base}/node"), &endpoint.node);
        self.slug(format!("{base}/port"), &endpoint.port);
        let exists = nodes.iter().find(|node| node.id == endpoint.node).is_some_and(|node| {
            let ports = if input { &node.inputs } else { &node.outputs };
            ports.iter().any(|port| port.id == endpoint.port)
        });
        if !exists { self.push(base.to_string(), IssueCode::ValueInvalid); }
    }

    pub(super) fn mockup(&mut self, base: &str, mockup: &Mockup) {
        match mockup {
            Mockup::Excel(value) => {
                self.text(&format!("{base}/caption"), &value.caption);
                self.text(&format!("{base}/workbook"), &value.workbook);
                self.bounded(format!("{base}/sheets"), value.sheets.len(), 1, 8);
                for (index, sheet) in value.sheets.iter().enumerate() { self.text(&format!("{base}/sheets/{index}"), sheet); }
                if value.active_sheet >= value.sheets.len() || value.active_sheet > 31 { self.push(format!("{base}/activeSheet"), IssueCode::ValueInvalid); }
                self.bounded(format!("{base}/columns"), value.columns.len(), 1, 16);
                for (index, column) in value.columns.iter().enumerate() {
                    let path = format!("{base}/columns/{index}");
                    if self.length(path.clone(), column, 1, 2) && !column.bytes().all(|byte| byte.is_ascii_uppercase()) { self.push(path, IssueCode::ValueInvalid); }
                }
                self.bounded(format!("{base}/rows"), value.rows.len(), 1, 32);
                for (row, cells) in value.rows.iter().enumerate() {
                    let path = format!("{base}/rows/{row}");
                    self.bounded(path.clone(), cells.len(), 1, 16);
                    if cells.len() != value.columns.len() { self.push(path.clone(), IssueCode::ValueInvalid); }
                    for (column, cell) in cells.iter().enumerate() { self.cell(&format!("{path}/{column}"), cell); }
                }
                let selection = &value.selection;
                for (key, index, size) in [("row", Some(selection.row), value.rows.len()), ("rowEnd", selection.row_end, value.rows.len()), ("column", Some(selection.column), value.columns.len()), ("columnEnd", selection.column_end, value.columns.len())] {
                    if index.is_some_and(|index| index >= size || index > 31) { self.push(format!("{base}/selection/{key}"), IssueCode::ValueInvalid); }
                }
                if selection.row_end.is_some_and(|index| index < selection.row) { self.push(format!("{base}/selection/rowEnd"), IssueCode::ValueInvalid); }
                if selection.column_end.is_some_and(|index| index < selection.column) { self.push(format!("{base}/selection/columnEnd"), IssueCode::ValueInvalid); }
                self.cell(&format!("{base}/formula"), &value.formula);
            }
            Mockup::Grasshopper(value) => {
                self.text(&format!("{base}/caption"), &value.caption);
                self.bounded(format!("{base}/nodes"), value.nodes.len(), 1, 8);
                self.unique(&format!("{base}/nodes"), value.nodes.iter().map(|node| node.id.as_str()));
                for (index, node) in value.nodes.iter().enumerate() {
                    let path = format!("{base}/nodes/{index}");
                    self.slug(format!("{path}/id"), &node.id);
                    self.text(&format!("{path}/label"), &node.label);
                    if let Some(widget) = &node.widget {
                        let widget_path = format!("{path}/widget");
                        match widget {
                            MockupWidget::Slider { min, max, value } => {
                                if !min.is_finite() || !max.is_finite() || !value.is_finite() || max <= min || value < min || value > max { self.push(widget_path, IssueCode::ValueInvalid); }
                            }
                            MockupWidget::Panel { lines } => {
                                self.bounded(format!("{widget_path}/lines"), lines.len(), 1, 8);
                                for (index, line) in lines.iter().enumerate() { self.cell(&format!("{widget_path}/lines/{index}"), line); }
                            }
                            MockupWidget::Plugin { label } => self.text(&format!("{widget_path}/label"), label),
                            _ => {}
                        }
                    }
                    for (side, ports) in [("inputs", &node.inputs), ("outputs", &node.outputs)] {
                        let path = format!("{path}/{side}");
                        self.bounded(path.clone(), ports.len(), 0, 12);
                        self.unique(&path, ports.iter().map(|port| port.id.as_str()));
                        for (index, port) in ports.iter().enumerate() {
                            self.slug(format!("{path}/{index}/id"), &port.id);
                            self.text(&format!("{path}/{index}/label"), &port.label);
                        }
                    }
                }
                self.bounded(format!("{base}/wires"), value.wires.len(), 0, 24);
                for (index, wire) in value.wires.iter().enumerate() {
                    self.endpoint(&format!("{base}/wires/{index}/from"), &wire.from, &value.nodes, false);
                    self.endpoint(&format!("{base}/wires/{index}/to"), &wire.to, &value.nodes, true);
                }
                self.bounded(format!("{base}/tree"), value.tree.len(), 0, 12);
                for (index, branch) in value.tree.iter().enumerate() {
                    let path = format!("{base}/tree/{index}");
                    let valid = branch.path.strip_prefix('{').and_then(|path| path.strip_suffix('}')).is_some_and(|path| path.split(';').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit())));
                    if !valid { self.push(format!("{path}/path"), IssueCode::ValueInvalid); }
                    self.length(format!("{path}/path"), &branch.path, 1, 64);
                    self.bounded(format!("{path}/values"), branch.values.len(), 0, 16);
                    for (index, cell) in branch.values.iter().enumerate() { self.cell(&format!("{path}/values/{index}"), cell); }
                }
                let target = &value.target;
                let path = format!("{base}/target");
                if target.index > 31 { self.push(format!("{path}/index"), IssueCode::ValueInvalid); }
                if let Some(node) = &target.node { self.slug(format!("{path}/node"), node); }
                if let Some(port) = &target.port { self.slug(format!("{path}/port"), port); }
                match target.kind {
                    GrasshopperTargetKind::Node | GrasshopperTargetKind::Input | GrasshopperTargetKind::Output => {
                        if !value.nodes.iter().any(|node| Some(&node.id) == target.node.as_ref()) { self.push(format!("{path}/node"), IssueCode::ValueInvalid); }
                        else if target.kind != GrasshopperTargetKind::Node {
                            self.endpoint(&path, &MockupEndpoint { node: target.node.clone().unwrap_or_default(), port: target.port.clone().unwrap_or_default() }, &value.nodes, target.kind == GrasshopperTargetKind::Input);
                        }
                    }
                    GrasshopperTargetKind::Wire | GrasshopperTargetKind::Branch | GrasshopperTargetKind::Item => {
                        let count = match target.kind {
                            GrasshopperTargetKind::Wire => value.wires.len(),
                            GrasshopperTargetKind::Branch => value.tree.len(),
                            _ => value.tree.first().map_or(0, |branch| branch.values.len()),
                        };
                        if target.index >= count { self.push(format!("{path}/index"), IssueCode::ValueInvalid); }
                    }
                    GrasshopperTargetKind::Tree => {}
                }
            }
        }
    }
}
