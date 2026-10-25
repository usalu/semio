//! 📊️ Semantic ownership census — charges the complete semantic ownership of one presented node in bounded pages, so a producer can price a subtree before it reconciles it.

use std::mem::{size_of, take};
use semio_framework_ui_runtime_presentation::TreeNode;
use crate::{SURFACE_RECONCILE_PAGE_BYTES, SURFACE_RECONCILE_SEMANTIC_COPIES, SURFACE_RECONCILE_VALUE_DEPTH, SurfaceReconcileFault};

//#region ⏭️ResumableReconcile

pub struct SurfaceSemanticMapPage {
    cursor: ui_contract::UiMapCursor,
    value: Option<ui_contract::UiValue>,
}

#[expect(clippy::large_enum_variant, reason = "The fixed traversal stack owns each map page and value within its admitted depth and byte budget.")]
pub enum SurfaceSemanticValueFrame {
    Value(ui_contract::UiValue),
    List { cursor: ui_contract::UiListCursor },
    Map { page: SurfaceSemanticMapPage },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceSemanticUsage {
    pub items: usize,
    pub bytes: usize,
}

pub enum SurfaceSemanticCensusStep {
    Progress(SurfaceSemanticUsage),
    Complete,
    Fault(SurfaceReconcileFault),
}

/// 📏️ Charges complete semantic ownership in bounded pages; a node may span several pages.
pub struct SurfaceSemanticCensusCursor {
    field: u8,
    container: u8,
    entry: usize,
    binding: usize,
    action: u8,
    data_attribute: u8,
    string_byte: usize,
    pub(crate) depth: usize,
    value_stack: Box<[Option<SurfaceSemanticValueFrame>]>,
    oversized_pages: usize,
}

impl Default for SurfaceSemanticCensusCursor {
    fn default() -> Self {
        let mut value_stack = Vec::with_capacity(SURFACE_RECONCILE_VALUE_DEPTH);
        value_stack.resize_with(SURFACE_RECONCILE_VALUE_DEPTH, || None);
        Self { field: 0, container: 0, entry: 0, binding: 0, action: 0, data_attribute: 0, string_byte: 0, depth: 0, value_stack: value_stack.into_boxed_slice(), oversized_pages: 0 }
    }
}

/// 🆕️ Allocation items a fresh reconciliation charges per census item, as `numerator / denominator` — measured
/// 0.17–0.35 on tree pick rows, bound tree rows, Home-shaped table rows and their containers; priced at 1/2.
pub const SURFACE_RECONCILE_ALLOCATION_ITEMS_PER_CENSUS_ITEM: (usize, usize) = (1, 2);

/// 🌳️ An upper bound of the items `node` and its whole subtree charge a fresh reconciliation against
/// [`SurfaceReconcileLimits::max_items`]. Per record: the reconciler's own semantic census, plus
/// [`SURFACE_RECONCILE_ALLOCATION_ITEMS_PER_CENSUS_ITEM`] of it for the allocation items the copy, bindings and
/// assembly stages charge on top (one per allocated page of what the census priced), plus one per page an
/// owner spans beyond its first. A producer that stops materialising rows once this would overdraw its item
/// budget shortens a window instead of having the whole surface refused with `Credits { items }` (ticket
/// 26/09/23 U5 — 138 Home rows faulted at `items 4098 > 4097`). Pinned against the real reconciler by
/// `subtree_item_census_bounds_the_items_a_fresh_reconciliation_consumes`.
pub fn surface_subtree_items(node: &TreeNode) -> Result<usize, SurfaceReconcileFault> {
    let mut items = 0usize;
    let mut pending = vec![node];
    let mut census = SurfaceSemanticCensusCursor::default();
    while let Some(next) = pending.pop() {
        census.restart();
        let mut record = 0usize;
        loop {
            match census.step(next) {
                SurfaceSemanticCensusStep::Progress(delta) => record = record.checked_add(delta.items).ok_or(SurfaceReconcileFault::CounterOverflow)?,
                SurfaceSemanticCensusStep::Complete => break,
                SurfaceSemanticCensusStep::Fault(fault) => return Err(fault),
            }
        }
        items = items.checked_add(record.div_ceil(SURFACE_RECONCILE_ALLOCATION_ITEMS_PER_CENSUS_ITEM.1).saturating_mul(SURFACE_RECONCILE_ALLOCATION_ITEMS_PER_CENSUS_ITEM.0)).and_then(|items| items.checked_add(record)).and_then(|items| items.checked_add(census.oversized_pages)).ok_or(SurfaceReconcileFault::CounterOverflow)?;
        pending.extend(next.children.iter());
    }
    Ok(items)
}

impl SurfaceSemanticCensusCursor {
    /// 🔁️ Readies a completed census for the next record, keeping its value stack's backing.
    fn restart(&mut self) {
        let value_stack = take(&mut self.value_stack);
        *self = Self { field: 0, container: 0, entry: 0, binding: 0, action: 0, data_attribute: 0, string_byte: 0, depth: 0, value_stack, oversized_pages: 0 };
    }

    /// 🔠️ UiText storage is inline in the already-counted enclosing payload; traversal remains work.
    fn inline_text(&self, _value: &ui_contract::UiText) -> SurfaceSemanticUsage {
        SurfaceSemanticUsage { items: 1, bytes: 0 }
    }

    pub fn owner(&mut self, bytes: usize) -> SurfaceSemanticUsage {
        self.oversized_pages = self.oversized_pages.saturating_add(bytes.div_ceil(SURFACE_RECONCILE_PAGE_BYTES).saturating_sub(1));
        self.string_byte = bytes.saturating_mul(SURFACE_RECONCILE_SEMANTIC_COPIES);
        SurfaceSemanticUsage { items: SURFACE_RECONCILE_SEMANTIC_COPIES, bytes: 0 }
    }

    fn backing<T>(&mut self, capacity: usize) -> SurfaceSemanticUsage {
        self.owner(capacity.saturating_mul(size_of::<T>()))
    }

    pub fn push_value(&mut self, value: &ui_contract::UiValue) -> Result<(), SurfaceReconcileFault> {
        if self.depth == SURFACE_RECONCILE_VALUE_DEPTH {
            return Err(SurfaceReconcileFault::ValueDepth { actual: self.depth.checked_add(1).ok_or(SurfaceReconcileFault::CounterOverflow)?, max: SURFACE_RECONCILE_VALUE_DEPTH });
        }
        let value = value.credited_clone().ok_or(SurfaceReconcileFault::AliasCapacity)?;
        self.push_owned_value(value)
    }

    fn push_owned_value(&mut self, value: ui_contract::UiValue) -> Result<(), SurfaceReconcileFault> {
        if self.depth == SURFACE_RECONCILE_VALUE_DEPTH {
            return Err(SurfaceReconcileFault::ValueDepth { actual: self.depth.checked_add(1).ok_or(SurfaceReconcileFault::CounterOverflow)?, max: SURFACE_RECONCILE_VALUE_DEPTH });
        }
        let next = self.depth.checked_add(1).ok_or(SurfaceReconcileFault::CounterOverflow)?;
        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::Value(value));
        self.depth = next;
        Ok(())
    }

    pub fn value_step(&mut self) -> Option<SurfaceSemanticCensusStep> {
        let frame = self.value_stack.get_mut(self.depth.checked_sub(1)?)?.take()?;
        match frame {
            SurfaceSemanticValueFrame::Value(value) => {
                self.depth -= 1;
                match value {
                    ui_contract::UiValue::Null | ui_contract::UiValue::Bool(_) | ui_contract::UiValue::Number(_) => Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())),
                    ui_contract::UiValue::Text(value) => Some(SurfaceSemanticCensusStep::Progress(self.owner(value.len()))),
                    ui_contract::UiValue::List(values) => {
                        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::List { cursor: values.cursor() });
                        self.depth += 1;
                        Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
                    }
                    ui_contract::UiValue::Map(values) => {
                        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::Map { page: SurfaceSemanticMapPage { cursor: values.cursor(), value: None } });
                        self.depth += 1;
                        Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
                    }
                }
            }
            SurfaceSemanticValueFrame::List { mut cursor } => {
                let Some(value) = cursor.next() else {
                    self.depth -= 1;
                    return Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()));
                };
                self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::List { cursor });
                Some(match self.push_owned_value(value) {
                    Ok(()) => SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()),
                    Err(fault) => SurfaceSemanticCensusStep::Fault(fault),
                })
            }
            SurfaceSemanticValueFrame::Map { mut page } => {
                if page.value.is_none() {
                    let Some(key_bytes) = page.cursor.advance().map(|(key, _)| key.len()) else {
                        self.depth -= 1;
                        return Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()));
                    };
                    page.value = page.cursor.take_current().map(|(_, value)| value);
                    self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { page });
                    return Some(SurfaceSemanticCensusStep::Progress(self.owner(key_bytes)));
                }
                let Some(value) = page.value.take() else {
                    self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { page });
                    return Some(SurfaceSemanticCensusStep::Fault(SurfaceReconcileFault::CounterOverflow));
                };
                self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { page });
                if let Err(fault) = self.push_owned_value(value) {
                    return Some(SurfaceSemanticCensusStep::Fault(fault));
                }
                Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
            }
        }
    }

    fn bindings_step(&mut self, bindings: &ui_contract::UiNodeBindings) -> SurfaceSemanticCensusStep {
        let Some(binding) = bindings.get(self.binding) else {
            self.binding = 0;
            self.action = 0;
            return SurfaceSemanticCensusStep::Complete;
        };
        let usage = match self.action {
            0 => self.inline_text(&binding.action.scope),
            1 => self.inline_text(&binding.action.name),
            2 => {
                if let Some(args) = &binding.args {
                    if let Err(fault) = self.push_value(args) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                SurfaceSemanticUsage::default()
            }
            3 => binding.capability.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)),
            _ => {
                self.binding += 1;
                self.action = 0;
                return SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default());
            }
        };
        self.action += 1;
        SurfaceSemanticCensusStep::Progress(usage)
    }

    /// 🪜️ A row's verbs from component stage `first` on: its row-action backing, each action's icon, label and verb, then
    /// its ONE target — scope, argument map and activation verb. A row names N verbs, never N argument maps.
    fn row_verbs_step(&mut self, row_actions: &ui_contract::UiFixedList<ui_contract::RowAction>, target: Option<&ui_contract::RowTarget>, first: u8) -> SurfaceSemanticCensusStep {
        let progress = |usage| SurfaceSemanticCensusStep::Progress(usage);
        match self.container - first {
            0 => {
                self.container += 1;
                progress(self.backing::<ui_contract::RowAction>(row_actions.capacity()))
            }
            1 => {
                let Some(action) = row_actions.get(self.entry) else {
                    self.container += 1;
                    self.entry = 0;
                    return progress(SurfaceSemanticUsage::default());
                };
                let usage = match self.data_attribute {
                    0 => self.inline_text(&action.icon),
                    1 => action.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    _ => {
                        self.data_attribute = 0;
                        self.entry += 1;
                        return progress(self.inline_text(&action.verb));
                    }
                };
                self.data_attribute += 1;
                progress(usage)
            }
            2 => {
                self.container += 1;
                progress(target.map_or_else(SurfaceSemanticUsage::default, |target| self.inline_text(&target.scope)))
            }
            3 => {
                self.container += 1;
                if let Some(args) = target.and_then(|target| target.args.as_ref()) {
                    let Some(args) = args.credited_clone() else { return SurfaceSemanticCensusStep::Fault(SurfaceReconcileFault::AliasCapacity) };
                    if let Err(fault) = self.push_owned_value(ui_contract::UiValue::Map(args)) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                progress(SurfaceSemanticUsage::default())
            }
            4 => {
                self.container += 1;
                progress(target.and_then(|target| target.activation.as_ref()).map_or_else(SurfaceSemanticUsage::default, |verb| self.inline_text(verb)))
            }
            _ => SurfaceSemanticCensusStep::Complete,
        }
    }

    fn component_step(&mut self, component: &ui_contract::Component) -> SurfaceSemanticCensusStep {
        use ui_contract::Component::*;
        let progress = |usage| SurfaceSemanticCensusStep::Progress(usage);
        match component {
            Container(props) => {
                let usage = match self.container {
                    0 => props.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    1 => props.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)),
                    2 => props.error.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)),
                    3 => props.drop_overlay.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.title.0)),
                    4 => props.drop_overlay.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.hint.0)),
                    5 => props.drop_overlay.as_ref().and_then(|value| value.accept.as_ref()).map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Text(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.value.0))
                }
                1 => {
                    self.container = 2;
                    progress(props.data_attributes.as_ref().map_or_else(SurfaceSemanticUsage::default, |values| self.backing::<(ui_contract::UiText, ui_contract::UiText)>(values.capacity())))
                }
                2 => {
                    let Some((key, value)) = props.data_attributes.as_ref().and_then(|values| values.get(self.entry)) else { return SurfaceSemanticCensusStep::Complete };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(key)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(value)
                    };
                    progress(usage)
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Button(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.icon),
                    1 => self.inline_text(&props.label.0),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Separator(_) | NumberStepper(_) => SurfaceSemanticCensusStep::Complete,
            Input(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.value),
                    1 => props.placeholder.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    2 => props.commit.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)),
                    3 => props.accept.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Select(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.value))
                }
                1 => {
                    self.container = 2;
                    progress(self.backing::<ui_contract::SelectItem>(props.items.capacity()))
                }
                2 => {
                    let Some(item) = props.items.get(self.entry) else {
                        self.container = 3;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(&item.value)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(&item.label.0)
                    };
                    progress(usage)
                }
                3 => {
                    self.container = 4;
                    progress(props.placeholder.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Toggle(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.icon),
                    1 => props.text.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            KeyValueList(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.backing::<ui_contract::KeyValueEntry>(props.entries.capacity()))
                }
                1 => {
                    let Some(entry) = props.entries.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(&entry.label.0)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(&entry.value)
                    };
                    progress(usage)
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Slider(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.unit.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            Ring(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(self.inline_text(&props.orb_id))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            Progress(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(self.inline_text(&props.value_text.0))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            IconSelect(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.value),
                    1 => self.inline_text(&props.classifier_kind),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Tree(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.interaction_domain.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            TreeSection(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            TreeItem(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.label.0))
                }
                1 => {
                    self.container = 2;
                    progress(props.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)))
                }
                2 => {
                    self.container = 3;
                    progress(props.icon.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)))
                }
                3 => {
                    self.container = 4;
                    progress(props.drag_data.as_ref().map_or_else(SurfaceSemanticUsage::default, |values| self.backing::<(ui_contract::UiText, ui_contract::UiText)>(values.capacity())))
                }
                4 => {
                    let Some((key, value)) = props.drag_data.as_ref().and_then(|values| values.get(self.entry)) else {
                        self.container = 5;
                        self.entry = 0;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.inline_text(key)
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.inline_text(value)
                    };
                    progress(usage)
                }
                _ => self.row_verbs_step(&props.row_actions, props.target.as_ref(), 5),
            },
            Image(props) => {
                let usage = match self.container {
                    0 => self.inline_text(&props.src),
                    1 => props.alt.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Surface(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.doc_schema))
                }
                1 => {
                    self.container = 2;
                    progress(self.backing::<u8>(props.doc.bytes.capacity()))
                }
                2 => {
                    self.container = 3;
                    progress(self.backing::<ui_contract::ActionBinding>(props.bindings.capacity()))
                }
                3 => self.bindings_step(&props.bindings),
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Table(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.label.0))
                }
                1 => {
                    self.container = 2;
                    progress(props.row_label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                }
                2 => {
                    self.container = 3;
                    progress(props.column_label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                }
                3 => {
                    self.container = 4;
                    progress(props.actions_label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                }
                4 => {
                    self.container = 5;
                    progress(self.backing::<ui_contract::Label>(props.columns.capacity()))
                }
                5 => {
                    let Some(column) = props.columns.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    self.entry += 1;
                    progress(self.inline_text(&column.0))
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            TableRow(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.backing::<ui_contract::UiText>(props.cells.capacity()))
                }
                1 => {
                    let Some(cell) = props.cells.get(self.entry) else {
                        self.container = 2;
                        self.entry = 0;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    self.entry += 1;
                    progress(self.inline_text(cell))
                }
                _ => self.row_verbs_step(&props.row_actions, props.target.as_ref(), 2),
            },
            Extension(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.extension))
                }
                1 => {
                    self.container = 2;
                    match self.push_value(&props.props) {
                        Ok(()) => progress(SurfaceSemanticUsage::default()),
                        Err(fault) => SurfaceSemanticCensusStep::Fault(fault),
                    }
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
        }
    }

    pub fn step(&mut self, node: &TreeNode) -> SurfaceSemanticCensusStep {
        if self.string_byte > 0 {
            let bytes = self.string_byte.min(SURFACE_RECONCILE_PAGE_BYTES);
            self.string_byte -= bytes;
            return SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage { items: 0, bytes });
        }
        if let Some(step) = self.value_step() {
            return step;
        }
        match self.field {
            0 => {
                self.field = 1;
                let bytes = size_of::<TreeNode>().saturating_mul(SURFACE_RECONCILE_SEMANTIC_COPIES);
                SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage { items: SURFACE_RECONCILE_SEMANTIC_COPIES, bytes })
            }
            1 => {
                self.field = 2;
                SurfaceSemanticCensusStep::Progress(self.inline_text(&node.key))
            }
            2 => match self.component_step(&node.component) {
                SurfaceSemanticCensusStep::Complete => {
                    self.field = 3;
                    self.container = 0;
                    self.entry = 0;
                    self.binding = 0;
                    self.action = 0;
                    self.data_attribute = 0;
                    SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
                }
                step => step,
            },
            3 => {
                self.field = 4;
                SurfaceSemanticCensusStep::Progress(node.accessibility.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
            }
            4 => {
                self.field = 5;
                SurfaceSemanticCensusStep::Progress(node.accessibility.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
            }
            5 => {
                self.field = 6;
                SurfaceSemanticCensusStep::Progress(node.accessibility.shortcut.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(value)))
            }
            6 => {
                self.field = 7;
                SurfaceSemanticCensusStep::Progress(self.backing::<ui_contract::ActionBinding>(node.bindings.capacity()))
            }
            7 => match self.bindings_step(&node.bindings) {
                SurfaceSemanticCensusStep::Complete => {
                    self.field = 8;
                    SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
                }
                step => step,
            },
            8 => {
                self.field = 9;
                SurfaceSemanticCensusStep::Progress(node.menu.as_ref().map_or_else(SurfaceSemanticUsage::default, |menu| self.inline_text(&menu.id)))
            }
            9 => {
                self.field = 10;
                if let Some(args) = node.menu.as_ref().and_then(|menu| menu.args.as_ref()) {
                    if let Err(fault) = self.push_value(args) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
            }
            10 => {
                self.field = 11;
                SurfaceSemanticCensusStep::Progress(self.backing::<Option<Box<TreeNode>>>(node.children.capacity()))
            }
            _ => SurfaceSemanticCensusStep::Complete,
        }
    }
}

//#endregion ⏭️ResumableReconcile
