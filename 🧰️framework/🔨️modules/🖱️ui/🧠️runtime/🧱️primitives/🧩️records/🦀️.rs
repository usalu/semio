//! 🧩️ Record diff primitives — the bounded copy, comparison and retirement cursors one node record is diffed and retired with, and the record builders that assemble it.

use std::mem::take;
use semio_framework_ui_runtime_presentation::TreeNode;
use crate::{SURFACE_COMPONENT_COPY_WORK_BYTES, SURFACE_RECONCILE_PAGE_BYTES, SurfaceReconcileFault};

//#region ⏭️ResumableReconcile

pub struct FlatPresentedNode {
    pub parent: Option<usize>,
    pub node: TreeNode,
    pub child_ids: ui_contract::UiNodeChildren,
}

pub struct PresentationFrame {
    pub index: usize,
    pub children: ui_contract::BuiltChildrenIntoIter,
}

pub struct RemovalFrame {
    pub id: ui_contract::UiNodeId,
    pub next_child: usize,
}

#[expect(clippy::large_enum_variant, reason = "Each variant retains a bounded copy cursor in the already admitted record slot; boxing would allocate while copying.")]
pub enum RecordOwnedCopy {
    Bindings(ui_contract::UiBindingsCopy),
    Component(ui_contract::UiComponentCopy),
    Comparison(ExistingComponentComparison),
    Changed(bool),
}

#[derive(Default)]
pub struct ExistingComponentComparison {
    pub cursor: ui_contract::UiComponentComparisonCursor,
    pub lease: Option<ui_contract::UiDocumentLease>,
    pub changed: Option<bool>,
}

impl ExistingComponentComparison {
    pub fn close_step(&mut self) -> bool {
        self.cursor.release_reads();
        if let Some(lease) = self.lease.as_mut() {
            if lease.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES).is_ok_and(|step| step.complete) { self.lease = None; }
            return false;
        }
        true
    }
}

impl RecordOwnedCopy {
    #[cfg(any(test, feature = "testkit"))]
    pub fn bindings(&self) -> Option<&ui_contract::UiBindingsCopy> { if let Self::Bindings(value) = self { Some(value) } else { None } }
    pub fn bindings_mut(&mut self) -> Option<&mut ui_contract::UiBindingsCopy> { if let Self::Bindings(value) = self { Some(value) } else { None } }
    pub fn component(&self) -> Option<&ui_contract::UiComponentCopy> { if let Self::Component(value) = self { Some(value) } else { None } }
    pub fn component_mut(&mut self) -> Option<&mut ui_contract::UiComponentCopy> { if let Self::Component(value) = self { Some(value) } else { None } }
    pub fn close_step(&mut self) -> bool {
        match self {
            Self::Bindings(value) => value.close_step(1, SURFACE_RECONCILE_PAGE_BYTES).is_ok_and(|step| step.complete),
            Self::Component(value) => value.close_step(1, SURFACE_COMPONENT_COPY_WORK_BYTES).is_ok_and(|step| step.complete),
            Self::Comparison(value) => value.close_step(),
            Self::Changed(_) => true,
        }
    }
}

pub struct RecordDiffCursor {
    pub id: ui_contract::UiNodeId,
    pub record: RecordSource,
    pub field: u8,
    pub fresh: Option<FreshRecordClone>,
    pub owned_copy: Option<RecordOwnedCopy>,
}

pub struct RecordSource(Option<ui_contract::UiNodeRecord>);
impl From<ui_contract::UiNodeRecord> for RecordSource { fn from(record: ui_contract::UiNodeRecord) -> Self { Self(Some(record)) } }
impl std::ops::Deref for RecordSource {
    type Target = ui_contract::UiNodeRecord;
    fn deref(&self) -> &Self::Target { self.0.as_ref().expect("unplaced record remains in its structural source slot") }
}
impl std::ops::DerefMut for RecordSource {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0.as_mut().expect("unplaced record remains in its structural source slot") }
}

#[derive(Default)]
pub struct FreshRecordClone {
    pub key: Option<ui_contract::UiText>,
    pub component: Option<ui_contract::Component>,
    pub layout: Option<ui_contract::LayoutSpec>,
    pub children: Option<ui_contract::UiNodeChildren>,
    pub accessibility: Option<ui_contract::AccessibilitySpec>,
    pub bindings: Option<ui_contract::UiNodeBindings>,
    pub menu: Option<Option<ui_contract::MenuRef>>,
}

pub fn diff_record_field(old: &ui_contract::UiNodeRecord, new: &ui_contract::UiNodeRecord, field: u8) -> Result<Option<ui_contract::UiPatchOp>, SurfaceReconcileFault> {
    let id = new.id;
    Ok(match field {
        0 if old.component != new.component => Some(ui_contract::UiPatchOp::SetComponent { id, component: new.component.credited_clone().ok_or(SurfaceReconcileFault::AliasCapacity)? }),
        1 if old.layout != new.layout => Some(ui_contract::UiPatchOp::SetLayout { id, layout: new.layout.clone() }),
        2 if old.activity != new.activity || old.disabled != new.disabled => Some(ui_contract::UiPatchOp::SetActivity { id, activity: new.activity, disabled: new.disabled }),
        3 if old.children != new.children => Some(ui_contract::UiPatchOp::SetChildren { id, children: new.children.clone() }),
        4 if old.style != new.style => Some(ui_contract::UiPatchOp::SetStyle { id, style: new.style }),
        5 if old.accessibility != new.accessibility => Some(ui_contract::UiPatchOp::SetAccessibility { id, accessibility: new.accessibility.clone() }),
        6 if old.bindings != new.bindings => Some(ui_contract::UiPatchOp::SetBindings { id, bindings: ui_contract::credited_bindings(&new.bindings).ok_or(SurfaceReconcileFault::AliasCapacity)? }),
        7 if old.menu != new.menu => Some(ui_contract::UiPatchOp::SetMenu {
            id,
            menu: match new.menu.as_ref() {
                Some(menu) => Some(menu.credited_clone().ok_or(SurfaceReconcileFault::AliasCapacity)?),
                None => None,
            },
        }),
        _ => None,
    })
}

pub fn retire_record_one(record: &mut ui_contract::UiNodeRecord, field: &mut u8, owned_copy: &mut Option<RecordOwnedCopy>) -> bool {
    match *field {
        0 => record.key = ui_contract::UiText::default(),
        1 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if !matches!(record.component, ui_contract::Component::Separator(_)) {
                let source = std::mem::replace(&mut record.component, ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
                *owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(source)));
                return false;
            }
        }
        2 => record.layout = ui_contract::LayoutSpec::default(),
        3 => {
            if record.children.pop().is_some() { return false; }
            if !record.children.terminal_is_empty() {
                let _ = record.children.release_empty_page(SURFACE_RECONCILE_PAGE_BYTES);
                return false;
            }
        }
        4 => record.accessibility = ui_contract::AccessibilitySpec::default(),
        5 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if !record.bindings.terminal_is_empty() {
                *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut record.bindings))));
                return false;
            }
        }
        6 => record.menu = None,
        7 => return true,
        _ => return true,
    }
    *field += 1;
    false
}

pub fn retire_fresh_record_one(record: &mut FreshRecordClone, field: &mut u8, owned_copy: &mut Option<RecordOwnedCopy>) -> bool {
    match *field {
        0 => record.key = None,
        1 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if let Some(component) = record.component.take() {
                *owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(component)));
                return false;
            }
        }
        2 => record.layout = None,
        3 => record.children = None,
        4 => record.accessibility = None,
        5 => {
            if let Some(copy) = owned_copy.as_mut() {
                if !copy.close_step() { return false; }
                *owned_copy = None;
            }
            if let Some(bindings) = record.bindings.take() {
                *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(bindings)));
                return false;
            }
        }
        6 => record.menu = None,
        7 => return true,
        _ => return true,
    }
    *field += 1;
    false
}

//#endregion ⏭️ResumableReconcile

//#region 🔖️Diff

pub fn build_record_owned(id: ui_contract::UiNodeId, node: TreeNode, children: ui_contract::UiNodeChildren, transition: Option<ui_contract::TransitionHint>) -> ui_contract::UiNodeRecord {
    let TreeNode { key, component, layout, style, activity, disabled, accessibility, bindings, menu, children: _, rejected_children: _ } = node;
    ui_contract::UiNodeRecord { id, key, component, layout, style, activity, disabled, transition, accessibility, bindings, menu, children }
}

/// 🏗️ Assembles a complete [`ui_contract::UiNodeRecord`] for `node` at `id` with `children` already
/// resolved to ids and `transition` carried over verbatim — [`TreeNode`] has no `transition`
/// field of its own (see `🦀️present.rs`'s module doc: it is builder-side and never diffs against a
/// previous tree), so this reconciler is the one place a record's `transition` is set, and it never
/// invents one: `None` for a freshly seen node, whatever the retained record already carried for an
/// existing one. Driving `Introducing`/`Celebrating` from presence data is out of this packet's scope.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "testkit"))]
pub fn build_record(id: ui_contract::UiNodeId, node: &TreeNode, children: ui_contract::UiNodeChildren, transition: Option<ui_contract::TransitionHint>) -> ui_contract::UiNodeRecord {
    ui_contract::UiNodeRecord {
        id,
        key: node.key.clone(),
        component: node.component.credited_clone().expect("test component alias credit"),
        layout: node.layout.clone(),
        style: node.style,
        activity: node.activity,
        disabled: node.disabled,
        transition,
        accessibility: node.accessibility.clone(),
        bindings: ui_contract::credited_bindings(&node.bindings).expect("test binding alias credit"),
        menu: node.menu.as_ref().map(|menu| menu.credited_clone().expect("test menu alias credit")),
        children,
    }
}

//#endregion 🔖️Diff
