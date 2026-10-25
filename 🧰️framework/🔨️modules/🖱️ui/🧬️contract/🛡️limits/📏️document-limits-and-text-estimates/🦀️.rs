//! 📖️ Document limits and text estimates: the `UiDocumentLimits`, the intake step constants, the patch byte estimate, the text byte counts and the component, tree toolbar and row target validity checks. See [`super`].

use super::*;

//#region 🔖️Limits
/// 🛡️ Quotas a [`crate::UiSnapshot`]/[`crate::UiPatch`] must stay within. `max_nodes`/`max_depth` bound
/// the document shape and are enforced by [`validate_snapshot`] (surfaced as
/// [`UiContractViolation::NodeQuota`]/[`UiContractViolation::DepthQuota`]); `max_children`/
/// `max_text_bytes`/`max_patch_ops`/`max_patch_bytes` bound one incoming [`crate::UiPatch`] and are
/// enforced directly by [`apply_patch`] (surfaced as [`PatchRejection::QuotaExceeded`]) — rejecting a
/// patch before it is even applied to the shadow draft is cheaper than discovering the violation after.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct UiDocumentLimits {
    /// 📦️ Total live nodes in one surface. 20 000 comfortably covers the largest known tree (a fully
    /// expanded product tree view or timeline) with headroom, while still bounding a malicious
    /// plugin's flood well below where a `HashMap<UiNodeId, UiNodeRecord>` becomes a memory concern.
    pub max_nodes: usize,
    /// 🏠️ Deepest legal node-to-root chain. 128 is far beyond any legitimate UI nesting (the deepest
    /// real shape, a `Tree`/`TreeSection`/`TreeItem` chain, rarely exceeds a few dozen) and doubles as
    /// the traversal's own recursion-depth bound, so it is also the security property that keeps
    /// `validate_snapshot`'s stack-free walk cheap even under adversarial input.
    pub max_depth: usize,
    /// 👶️ Direct children on one node. 4 096 covers the largest legitimate flat list (an unpaginated
    /// tree section or a large `Select`-like listing rendered as children) without letting one node
    /// alone approach `max_nodes`.
    pub max_children: usize,
    /// 📝️ UTF-8 bytes in one component's own text-bearing fields (label/description/value/…). 64 KiB
    /// is generous for authored UI copy (far beyond a label or even a long description) while refusing
    /// to let a single component smuggle an arbitrarily large string through the contract.
    pub max_text_bytes: usize,
    /// 🩹️ Ops in one [`crate::UiPatch`]. 4 096 mirrors `max_children`'s order of magnitude — no
    /// legitimate single reconciliation pass should need more ops than the largest single-node fan-out
    /// this crate already permits.
    pub max_patch_ops: usize,
    /// 📮️ Estimated wire bytes for one [`crate::UiPatch`] (see [`patch_byte_estimate`]). 1 MiB matches
    /// a conservative single-frame transport budget — large enough for a full-surface `Upsert` burst,
    /// small enough that a malicious patch cannot exhaust an actor mailbox in one message.
    pub max_patch_bytes: usize,
}

impl Default for UiDocumentLimits {
    fn default() -> Self {
        Self { max_nodes: 20_000, max_depth: 128, max_children: 4_096, max_text_bytes: 65_536, max_patch_ops: 4_096, max_patch_bytes: 1_048_576 }
    }
}

/// 📏️ Intake steps ONE retained node costs at its first publication, measured with ~1.8× headroom.
///
/// Every renderer drives the retained intake one PHASE per `advance` — a LEB128 byte, a text body, an
/// attach — so a fixed step count caps a surface patch at a few KiB no matter how large the grant is.
/// Both the React and the wgpu targets therefore price an intake through
/// [`UiDocumentLimits::retained_ui_intake_step_ceiling`] instead. The numbers are declared once,
/// language-agnostically, in `🧵️retained/🧪️tests/🧫️fixtures/📥️intake/🔣️.json`'s `budget` and mirrored by the
/// TypeScript twin `RETAINED_UI_INTAKE_STEPS_PER_NODE`
/// (`📃️UiDocumentStore/📥️intake/🟦️.ts`).
pub const RETAINED_UI_INTAKE_STEPS_PER_NODE: usize = 8_192;

/// 🎞️ Steps one drive may spend before it MUST hand the frame back. Exhausting a slice is a YIELD, not
/// a fault: the caller parks the intake with its retained cursor and resumes it on the next frame, so a
/// document larger than one slice publishes across several frames. Only the whole-document ceiling is
/// terminal. Twin of `RETAINED_UI_INTAKE_SLICE_STEPS`.
pub const RETAINED_UI_INTAKE_SLICE_STEPS: usize = 4_096;

impl UiDocumentLimits {
    /// 🪜️ Liveness backstop for one surface patch: the steps a document at this quota may cost to mint.
    /// It is NOT the progress guarantee — the intake rejects a phase reporting 32 consecutive steps that
    /// carry neither an item nor a byte, and that byte-aware rule is what catches a genuine stall.
    pub fn retained_ui_intake_step_ceiling(&self) -> usize {
        self.max_nodes.saturating_mul(RETAINED_UI_INTAKE_STEPS_PER_NODE)
    }
}

/// 🧮️ A rough, dependency-free proxy for a patch's wire cost — this crate has no `pack`/serde-json
/// runtime dependency (see `🦀️.rs`'s dependency-free guarantee), so this sums UTF-8 byte lengths
/// of the ops' own text-bearing payloads plus a small fixed per-op overhead, rather than actually
/// encoding the patch.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn patch_byte_estimate(patch: &crate::UiPatch) -> usize {
    const OP_OVERHEAD_BYTES: usize = 16;
    patch.ops.iter().map(|op| OP_OVERHEAD_BYTES + op_text_bytes(op)).sum()
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn op_text_bytes(op: &crate::UiPatchOp) -> usize {
    match op {
        crate::UiPatchOp::Upsert(record) => record.key.len() + component_text_bytes(&record.component) + accessibility_text_bytes(&record.accessibility) + bindings_text_bytes(&record.bindings) + menu_text_bytes(&record.menu),
        crate::UiPatchOp::SetComponent { component, .. } => component_text_bytes(component),
        crate::UiPatchOp::SetChildren { children, .. } => children.len() * size_of::<crate::UiNodeId>(),
        crate::UiPatchOp::SetAccessibility { accessibility, .. } => accessibility_text_bytes(accessibility),
        crate::UiPatchOp::SetBindings { bindings, .. } => bindings_text_bytes(bindings),
        crate::UiPatchOp::SetMenu { menu, .. } => menu_text_bytes(menu),
        crate::UiPatchOp::SetLayout { .. } | crate::UiPatchOp::SetActivity { .. } | crate::UiPatchOp::SetStyle { .. } | crate::UiPatchOp::Remove { .. } | crate::UiPatchOp::SetRoot { .. } => 0,
    }
}

/// ♿️ UTF-8 bytes of `spec`'s own text-bearing fields (`label`/`description`/`shortcut`) — mirrors
/// [`component_text_bytes`]'s own scope-limited accounting for the same reason: these are the fields
/// most likely to carry a large user-authored string.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn accessibility_text_bytes(spec: &crate::AccessibilitySpec) -> usize {
    label_bytes(&spec.label) + label_bytes(&spec.description) + spec.shortcut.as_deref().map_or(0, str::len)
}

/// 🪢️ UTF-8 bytes of every binding's own `action` identity — `scope`/`name` are the only text an
/// [`crate::ActionBinding`] carries outside its opaque, non-text-bearing `args`/`capability` payload.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn bindings_text_bytes(bindings: &crate::UiNodeBindings) -> usize {
    bindings.iter().map(|binding| binding.action.scope.len() + binding.action.name.len() + binding.capability.as_deref().map_or(0, str::len)).sum()
}

/// 📋️ UTF-8 bytes of `menu`'s own `id` — its only text-bearing field outside the opaque `args`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn menu_text_bytes(menu: &Option<crate::MenuRef>) -> usize {
    menu.as_ref().map_or(0, |menu_ref| menu_ref.id.len())
}

/// 🧱️ UTF-8 bytes of `component`'s own text-bearing fields — the fields most likely to carry large
/// user-authored strings (labels, descriptions, values, placeholders). Not exhaustive over every field
/// of every variant; icon keys and other short identifiers are not text content this quota guards.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn component_text_bytes(component: &crate::Component) -> usize {
    use crate::Component::*;
    match component {
        Container(props) => label_bytes(&props.label) + props.description.as_deref().map_or(0, str::len) + props.error.as_deref().map_or(0, str::len),
        Text(props) => props.value.0.len(),
        Button(props) => props.label.0.len(),
        Input(props) => props.value.len() + label_bytes(&props.placeholder),
        Select(props) => props.items.iter().map(|item| item.label.0.len()).sum::<usize>() + label_bytes(&props.placeholder),
        Toggle(props) => label_bytes(&props.text),
        KeyValueList(props) => props.entries.iter().map(|entry| entry.label.0.len() + entry.value.len()).sum(),
        TreeSection(props) => label_bytes(&props.label),
        TreeItem(props) => props.label.0.len() + props.description.as_deref().map_or(0, str::len),
        Image(props) => label_bytes(&props.alt),
        Progress(props) => props.value_text.0.len(),
        Extension(props) => props.extension.len(),
        Table(props) => props.label.0.len() + props.columns.iter().map(|column| column.0.len()).sum::<usize>() + label_bytes(&props.row_label) + label_bytes(&props.column_label) + label_bytes(&props.actions_label),
        TableRow(props) => props.cells.iter().map(|cell| cell.len()).sum(),
        Separator(_) | Slider(_) | NumberStepper(_) | Ring(_) | IconSelect(_) | Tree(_) | Surface(_) => 0,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn label_bytes(label: &Option<crate::Label>) -> usize {
    label.as_ref().map_or(0, |label| label.0.len())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn component_is_finite(component: &crate::Component) -> bool {
    match component {
        crate::Component::Slider(props) => [props.value, props.min, props.max, props.step].into_iter().chain(props.snaps.iter().copied()).chain(props.display_factor).chain(limit_values(props.limits.as_ref())).all(f64::is_finite),
        crate::Component::NumberStepper(props) => [Some(props.value), Some(props.step), props.min, props.max, props.display_factor].into_iter().flatten().chain(props.snaps.iter().copied()).chain(limit_values(props.limits.as_ref())).all(f64::is_finite) && props.min.zip(props.max).is_none_or(|(min, max)| min <= max),
        crate::Component::Ring(props) => props.t.is_finite(),
        crate::Component::Progress(props) => props.completed.is_finite() && props.total.is_none_or(f64::is_finite),
        crate::Component::Input(props) => [props.min, props.max, props.step, props.display_factor].into_iter().flatten().chain(props.snaps.iter().copied()).chain(limit_values(props.limits.as_ref())).all(|value| value.is_finite()),
        _ => true,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn limit_values(limits: Option<&crate::UiNumberLimits>) -> impl Iterator<Item = f64> + '_ {
    limits.into_iter().flat_map(|limits| [limits.min.as_ref(), limits.max.as_ref()]).flatten().map(|bound| bound.value)
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn component_snaps_are_valid(component: &crate::Component) -> bool {
    match component {
        crate::Component::Slider(props) => props.snaps_are_valid(),
        crate::Component::Input(props) => props.snaps_are_valid(),
        crate::Component::NumberStepper(props) => props.snaps_are_valid(),
        _ => true,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn component_number_range_is_valid(component: &crate::Component) -> bool {
    match component {
        crate::Component::Slider(props) => props.number_range_is_valid(),
        crate::Component::Input(props) => props.number_range_is_valid(),
        crate::Component::NumberStepper(props) => props.number_range_is_valid(),
        _ => true,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn is_section(component: &crate::Component) -> bool {
    matches!(component, crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Section, .. }))
}

/// 🎛️ Maximum independently actionable Buttons one inline Tree toolbar may own.
pub const TREE_INLINE_TOOLBAR_BUTTONS: usize = 4;

fn tree_toolbar_is_valid<'a>(record: &crate::UiNodeRecord, toolbar_id: crate::UiNodeId, mut get: impl FnMut(crate::UiNodeId) -> Option<&'a crate::UiNodeRecord>) -> bool {
    if !record.children.iter().any(|child| *child == toolbar_id) {
        return false;
    }
    let Some(toolbar) = get(toolbar_id) else { return false };
    if !matches!(&toolbar.component, crate::Component::Container(props) if props.role == crate::ContainerRole::Toolbar)
        || !matches!(&toolbar.layout, crate::LayoutSpec::Stack(layout) if layout.axis == crate::Axis::Horizontal)
        || toolbar.children.is_empty()
        || toolbar.children.len() > TREE_INLINE_TOOLBAR_BUTTONS
    {
        return false;
    }
    toolbar.children.iter().all(|button_id| get(*button_id).is_some_and(|button| matches!(&button.component, crate::Component::Button(_)) && button.children.is_empty()))
}

pub(super) fn tree_inline_toolbar_is_valid<'a>(record: &crate::UiNodeRecord, get: impl FnMut(crate::UiNodeId) -> Option<&'a crate::UiNodeRecord>) -> bool {
    let crate::Component::TreeItem(props) = &record.component else { return true };
    props.inline_toolbar.is_none_or(|toolbar| tree_toolbar_is_valid(record, toolbar, get))
}

pub(super) fn tree_section_header_toolbar_is_valid<'a>(record: &crate::UiNodeRecord, get: impl FnMut(crate::UiNodeId) -> Option<&'a crate::UiNodeRecord>) -> bool {
    let crate::Component::TreeSection(props) = &record.component else { return true };
    props.header_toolbar.is_none_or(|toolbar| tree_toolbar_is_valid(record, toolbar, get))
}

pub(super) fn tree_detail_is_valid<'a>(record: &crate::UiNodeRecord, mut get: impl FnMut(crate::UiNodeId) -> Option<&'a crate::UiNodeRecord>) -> bool {
    let crate::Component::TreeItem(props) = &record.component else { return true };
    let Some(detail_id) = props.detail else { return true };
    record.children.iter().any(|child| *child == detail_id) && get(detail_id).is_some_and(|detail| matches!(detail.component, crate::Component::Surface(_)))
}

/// 🎯️ A row's verbs and its [`crate::RowTarget`] come together: a TreeItem or TableRow carries `target`
/// exactly when it carries row actions or a target `activation`, and never a record `Trigger::Activate`
/// binding — its activation is the verb on its target.
pub(super) fn row_target_is_valid(record: &crate::UiNodeRecord) -> bool {
    let (row_actions, target) = match &record.component {
        crate::Component::TreeItem(props) => (&props.row_actions, &props.target),
        crate::Component::TableRow(props) => (&props.row_actions, &props.target),
        _ => return true,
    };
    let verbs = !row_actions.is_empty() || target.as_ref().is_some_and(|target| target.activation.is_some());
    target.is_some() == verbs && !record.bindings.iter().any(|binding| binding.trigger == crate::Trigger::Activate)
}
//#endregion 🔖️Limits
