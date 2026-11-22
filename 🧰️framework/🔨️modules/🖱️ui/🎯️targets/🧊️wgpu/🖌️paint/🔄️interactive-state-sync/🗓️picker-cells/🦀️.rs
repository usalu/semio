//! 🗓️ Picker cells: the retained nodes of an open `date` or `color` input's popup — React's native pickers drawn as ledgered composite children, their geometry from the shared picker models. See [`super`].

use semio_framework_value::DslValue;
use semio_ui_declaration::layout::ActionDescriptor;
use semio_ui_declaration::ui::{UiButtonNode, UiInputNode, UiNode, UiPresence, UiStackNode, UiState};
use semio_ui_declaration::IconName;
use semio_ui_locale::{Label, Locale};
use semio_ui_theme::geometry::Rect;
use semio_ui_theme::Theme;
use semio_ui_wgpu_layout::colour_picker::{colour_channels, colour_popup_layout, colour_value, COLOUR_SWATCHES};
use semio_ui_wgpu_layout::date_picker::{date_locale, date_popup_layout, focus_date, month_grid, CivilDate, DATE_GRID_COLUMNS, DATE_GRID_ROWS};

/// 🔑️ The key prefix of every picker cell; cell `i` is `picker.i`.
pub const PICKER_KEY_PREFIX: &str = "picker.";
pub const DATE_CELL_PANEL: usize = 0;
pub const DATE_CELL_PREVIOUS: usize = 1;
pub const DATE_CELL_NEXT: usize = 2;
pub const DATE_CELL_TITLE: usize = 3;
pub const DATE_CELL_FIRST_DAY: usize = 4;
const DATE_CELL_FIRST_WEEKDAY: usize = DATE_CELL_FIRST_DAY + DATE_GRID_ROWS * DATE_GRID_COLUMNS;

pub use semio_ui_wgpu_layout::date_picker::is_picker_kind;

/// 🔢️ How many cells the popup of `input_kind` holds.
pub fn picker_cell_count(input_kind: &str) -> usize {
    match input_kind {
        "date" => DATE_CELL_FIRST_WEEKDAY + DATE_GRID_COLUMNS,
        "color" => 2 + COLOUR_SWATCHES.len() + 4,
        _ => 0,
    }
}

pub fn picker_cell_key(index: usize) -> String {
    format!("{PICKER_KEY_PREFIX}{index}")
}

fn with_value(action: &ActionDescriptor, value: &str) -> ActionDescriptor {
    let mut fields = match &action.args {
        Some(DslValue::Object(fields)) => fields.clone(),
        _ => Vec::new(),
    };
    fields.retain(|(key, _)| key != "value");
    fields.push(("value".to_string(), DslValue::String(value.to_string())));
    ActionDescriptor { args: Some(DslValue::Object(fields)), ..action.clone() }
}

fn inert() -> ActionDescriptor {
    ActionDescriptor { controller_id: String::new(), action: String::new(), args: None }
}

fn button(index: usize, label: impl Into<String>, action: ActionDescriptor, selected: bool, enabled: bool) -> UiNode {
    let presence = UiPresence { selected, state: if enabled { UiState::Normal } else { UiState::Disabled }, ..UiPresence::default() };
    UiNode::Button(UiButtonNode { id: Some(picker_cell_key(index)), icon_id: IconName::from(""), label: Label::data(label.into()), action, style: None, presence, menu: None })
}

fn panel(index: usize) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some(picker_cell_key(index)), children: Vec::new(), presence: UiPresence::default(), activate: Some(inert()), drop_action: None, drop_overlay: None, menu: None })
}

fn field(parent: &UiInputNode, index: usize, kind: &str, value: String, commit: String) -> UiNode {
    UiNode::Input(UiInputNode { id: picker_cell_key(index), input_kind: kind.into(), value, commit: Some(commit), on_change: parent.on_change.clone(), ..Default::default() })
}

/// 🧩️ One minted popup cell: its node, its ABSOLUTE rect and whether it holds the popup's keyboard focus (the focused day, the highlighted swatch) — the typed flag paint reads as [`semio_ui_retained_tree::tree::NodeFlags::PICKER_FOCUS`].
pub struct PickerCell {
    pub node: UiNode,
    pub rect: Rect,
    pub focused: bool,
}

/// 🧩️ Cell `index` of the popup of `input`, below (or above) `anchor`. `None` past the last cell.
#[allow(clippy::too_many_arguments, reason = "one popup context: the field, its widget state, its place and the host's language")]
pub fn picker_cell(input: &UiInputNode, picker_day: Option<i64>, highlighted: Option<usize>, anchor: Rect, viewport_h: f32, theme: &Theme, locale: Option<Locale>, index: usize) -> Option<PickerCell> {
    let (cell, pad) = (theme.control_height, theme.padding_standard);
    match input.input_kind.as_str() {
        "date" => {
            let layout = date_popup_layout(anchor, viewport_h, cell, pad, false);
            let shown = focus_date(&input.value, picker_day)?;
            let names = locale.and_then(|locale| date_locale(locale.as_str()));
            let week_starts_monday = names.is_none_or(|names| names.week_starts_monday);
            let grid = month_grid(shown.year, shown.month, week_starts_monday)?;
            let value = CivilDate::parse_iso(&input.value);
            let (node, rect) = match index {
                DATE_CELL_PANEL => (panel(index), layout.panel),
                DATE_CELL_PREVIOUS => (button(index, "\u{2039}", inert(), false, true), layout.previous),
                DATE_CELL_NEXT => (button(index, "\u{203a}", inert(), false, true), layout.next),
                DATE_CELL_TITLE => (button(index, names.map_or_else(|| format!("{:04}-{:02}", shown.year, shown.month), |names| names.month_title(shown.year, shown.month)), inert(), false, false), layout.title),
                i if i < DATE_CELL_FIRST_WEEKDAY => {
                    let slot = i - DATE_CELL_FIRST_DAY;
                    let day = grid[slot];
                    (button(index, day.day.to_string(), with_value(&input.on_change, &day.iso()), value == Some(day), true), layout.cells[slot])
                }
                i if i < DATE_CELL_FIRST_WEEKDAY + DATE_GRID_COLUMNS => {
                    let column = i - DATE_CELL_FIRST_WEEKDAY;
                    (button(index, names.map_or("", |names| names.weekday_header()[column]), inert(), false, false), layout.weekdays[column])
                }
                _ => return None,
            };
            let focused = picker_day.is_some() && (DATE_CELL_FIRST_DAY..DATE_CELL_FIRST_WEEKDAY).contains(&index) && grid[index - DATE_CELL_FIRST_DAY] == shown;
            Some(PickerCell { node, rect, focused })
        }
        "color" => {
            let layout = colour_popup_layout(anchor, viewport_h, cell, pad, false);
            let current = colour_value(&input.value).unwrap_or_else(|| "#000000".into());
            let channels = colour_channels(&current).unwrap_or([0, 0, 0]);
            let swatches = COLOUR_SWATCHES.len();
            let (node, rect) = match index {
                0 => (panel(index), layout.panel),
                1 => (button(index, highlighted.and_then(|slot| COLOUR_SWATCHES.get(slot)).map_or_else(|| current.clone(), |hex| (*hex).to_string()), inert(), false, false), layout.preview),
                i if i < 2 + swatches => {
                    let hex = COLOUR_SWATCHES[i - 2];
                    (UiNode::Input(UiInputNode { id: picker_cell_key(index), input_kind: "colorSwatch".into(), value: hex.into(), on_change: with_value(&input.on_change, hex), ..Default::default() }), layout.swatches[i - 2])
                }
                i if i == 2 + swatches => (field(input, index, "colourHex", current.clone(), "blur".into()), layout.hex),
                i if i < 2 + swatches + 4 => {
                    let channel = i - 3 - swatches;
                    (field(input, index, "colourChannel", channels[channel].to_string(), format!("colour:{}:{current}", ["r", "g", "b"][channel])), layout.channels[channel])
                }
                _ => return None,
            };
            let focused = (2..2 + swatches).contains(&index) && highlighted == Some(index - 2);
            Some(PickerCell { node, rect, focused })
        }
        _ => None,
    }
}
