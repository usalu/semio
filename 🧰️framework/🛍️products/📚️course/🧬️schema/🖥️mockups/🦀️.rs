//! 🖥️ Solution-free software workspace previews, given reference data and highlighted interface parts.
use serde::{Deserialize, Serialize};
use super::{Slug, Text};

/// 🧾️ Localized cell content; empty cells are meaningful.
pub type MockupText = Text;

/// 🔲️ Zero-based selected rectangle in the reference grid.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MockupSelection {
    pub row: usize,
    pub column: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_end: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_end: Option<usize>,
}

/// 🎯️ The highlighted Excel interface part.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExcelTarget { Cell, Range, Row, Column, Worksheet, Workbook, Formula, NameBox }

/// 📗️ A workbook showing the input data of an exercise.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExcelMockup {
    pub caption: Text,
    pub workbook: Text,
    pub sheets: Vec<Text>,
    pub active_sheet: usize,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<MockupText>>,
    pub selection: MockupSelection,
    pub formula: MockupText,
    pub target: ExcelTarget,
}

/// 🔌️ A labelled node port.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MockupPort { pub id: Slug, pub label: Text }

/// 🧩️ A computation or parameter with separately identified ports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MockupNode {
    pub id: Slug, pub label: Text, pub inputs: Vec<MockupPort>, pub outputs: Vec<MockupPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget: Option<MockupWidget>,
}

/// 📍️ A port reference within a workspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MockupEndpoint { pub node: Slug, pub port: Slug }

/// 🧵️ A directed output-to-input connection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MockupWire { pub from: MockupEndpoint, pub to: MockupEndpoint }

/// 🌿️ A Grasshopper tree path and its ordered input values.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MockupBranch { pub path: String, pub values: Vec<MockupText> }

/// 🔦️ The highlighted Grasshopper interface part.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrasshopperTargetKind { Node, Input, Output, Wire, Tree, Branch, Item }

/// 🪧️ The node/port or zero-based wire, branch or first-branch item to highlight.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MockupTarget {
    pub kind: GrasshopperTargetKind,
    pub index: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<Slug>,
}

/// 🦗️ A bounded node graph with accessible directed relations and tree data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrasshopperMockup {
    pub caption: Text,
    pub nodes: Vec<MockupNode>,
    pub wires: Vec<MockupWire>,
    pub tree: Vec<MockupBranch>,
    pub target: MockupTarget,
}

/// 🪟️ A workspace preview containing only given data, independent of grading truth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Mockup { Excel(ExcelMockup), Grasshopper(GrasshopperMockup) }

/// 🎛️ Static native widget appearance, independent of grading truth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum MockupWidget {
    Parameter,
    Component,
    Slider { min: f64, max: f64, value: f64 },
    Panel { lines: Vec<MockupText> },
    Plugin { label: Text },
}
