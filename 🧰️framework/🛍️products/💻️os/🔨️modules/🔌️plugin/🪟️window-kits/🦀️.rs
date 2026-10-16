//! 🪟️ Window kit contract, form panels, label axes and the UI command value producer.
extern crate semio_framework_os_kernel as dsl;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::AppDefinition;
use semio_framework::CommandDefinition;
use semio_framework::InteractionDefinition;
use semio_framework::InteractionVerb;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use semio_framework_value::RetirementDemand;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_value::DslValue;
use semio_framework::ActionDefinition;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::{ActionKind, TreeWindowRequest};
use semio_framework::{Fault, FaultCode, FaultOrigin, IconName, ViewModel, WindowKindDefinition};
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use serde_json::Value;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_ui_layout::layout::ContextMenuItemSpec;
use semio_framework_ui_layout::layout::ContextMenuSurfaceTarget;
/// 🧾️ The body-wide node budget and its fixed-node reserve live ONCE, in the UI contract, because
/// the React host spends the same number from the other side of the wire
/// (`🌳️Tree/🟦️.tsx`'s `capTreeWindowRequests`). See
/// [`semio_framework_ui_contract::TREE_WINDOW_BODY_NODE_BUDGET`] for the cost model both sides
/// implement; [`TreeWindows`] is this side of it.
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_ui_contract::TREE_WINDOW_BODY_NODE_BUDGET;
// 🗣️ Shared two-axis (locale × terminology) label resolution — replaces the ~25x hand-rolled
// `struct XLabels { .. }` + `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels`
// pattern duplicated per app, AND the 4-crate hand-rolled `NATIVE_EN/NATIVE_DE/REUSE_EN/REUSE_DE` +
// non-exhaustive `match (terminology, is_de) { ..., (_, true) => ... }` terminology resolvers this
// SDK never covered. `app_labels!` now declares all four cells per field and resolves them via an
// exhaustive match on the generated `Locale`/`Terminology` enums — no catch-all arm is possible, so
// adding a locale or terminology to `🖱️ui/🎚️axes/🔣️.json` breaks every invocation until it supplies that
// cell. See ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.
use semio_framework_ui_locale::AppLabels;
#[cfg(test)]
use semio_framework_ui_locale::app_labels;

use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_interaction::ui_assembly_error;
use semio_framework_plugin_window_transient::ui_label;
use semio_framework_plugin_window_transient::ui_text;
#[cfg(test)]
use crate::image_kit::IMAGE_WINDOW_PAGED_NODE_KEY;
#[cfg(test)]
use crate::image_kit::IMAGE_WINDOW_PIXELS_LANE_KEY;
#[cfg(test)]
use crate::image_kit::ImageView;
#[cfg(test)]
use crate::image_kit::ImageWindowKit;
#[cfg(test)]
use crate::media_kit::MEDIA_PLAYBACK_OUTPUT_PORT;
#[cfg(test)]
use crate::media_kit::MEDIA_TRANSPORT_EXTENSION_ID;
#[cfg(test)]
use crate::media_kit::MediaCapabilityStatus;
#[cfg(test)]
use crate::media_kit::MediaKind;
#[cfg(test)]
use crate::media_kit::MediaResource;
#[cfg(test)]
use crate::media_kit::MediaView;
#[cfg(test)]
use crate::media_kit::MediaWindowKit;
#[cfg(test)]
use crate::mesh_kit::MeshView;
#[cfg(test)]
use crate::mesh_kit::MeshWindowKit;
#[cfg(test)]
use crate::text_kit::TextEditView;
#[cfg(test)]
use crate::text_kit::TextView;
#[cfg(test)]
use crate::text_kit::TextWindowKit;



// 📋️ Shared form-panel builder — lifts the `Section > labeled Field rows > submit Button` skeleton
// (and the sibling `entity_detail` read-only `KeyValue` summary block) duplicated across plugin crates
// that render declarative forms/detail panels, mirroring `PanelTreeBuilder`'s namespaced builder-pattern
// shape above (`namespace` prefixes every id, method chaining ends in `.build() -> BuiltNode`).

/// 📋️ Fluent builder for a `Section` of labeled `Field` rows ending in an optional submit `Button` —
/// same namespaced-id / method-chaining shape as `PanelTreeBuilder`.
pub struct FormPanelBuilder {
    namespace: UiText,
    fields: UiFixedList<BuiltNode>,
    submit: Option<BuiltNode>,
}

impl FormPanelBuilder {
    /// 📋️ `namespace` prefixes every field id built via `.field_id()`/`.field()`/`.from_dictionary()`.
    pub async fn new(namespace: impl TryInto<UiText>) -> UiAssemblyResult<Self> {
        Ok(Self { namespace: ui_text(namespace, "form-panel.namespace")?, fields: UiFixedList::default(), submit: None })
    }

    /// 📋️ Builds a namespaced field id: `"{namespace}.field.{id}"` — mirrors `PanelTreeBuilder::item_id`.
    pub async fn field_id(&self, id: &str) -> UiAssemblyResult<UiText> {
        UiText::try_format(format_args!("{}.field.{id}", self.namespace)).ok_or_else(|| ui_assembly_error("form-panel.field-id"))
    }

    /// 📋️ Adds one labeled field row: `control` is any already-built child node. The old
    /// `UiControlNode` wrapper enum this took is retired outright (`📓️recipe-plugin.md`'s own
    /// call: every one of its variants is already a `Component` variant in its own right, so a
    /// control-as-child needs no separate wrapper type) — callers build `control` with the plain
    /// `ui::*` DSL (`ui::input(..).build()`, `ui::toggle(..).build()`, …) instead.
    pub async fn field<L: TryInto<Label>>(mut self, id: &str, label: L, description: Option<String>, control: BuiltNode) -> UiAssemblyResult<Self> {
        let field_id = self.field_id(id).await?;
        let builder = field(ui_label(label, "form-panel.field-label")?);
        let builder = builder.try_id(&field_id).map_err(|_| ui_assembly_error("form-panel.field-id"))?;
        let mut builder = builder.try_child(control).map_err(|_| ui_assembly_error("form-panel.field-control"))?;
        if let Some(description) = description {
            builder = builder.description(ui_text(description, "form-panel.field-description")?);
        }
        let field = builder.try_build().map_err(|_| ui_assembly_error("form-panel.field-build"))?;
        self.fields.try_push(field).map_err(|_| ui_assembly_error("form-panel.fields"))?;
        Ok(self)
    }

    /// 📋️ Routes the OS `form.dictionary` resource shape (see the `ArtifactKindSpec { id:
    /// "form.dictionary", source_format: "forms.dictionary", .. }` registered by `forms/plugin`'s
    /// `create_forms_app`) into a sequence of text-input field rows: each top-level entry in the
    /// `dictionary_json` array — `{ "id", "label"?, "description"?, "value"? }` — becomes one field
    /// dispatching the shared `on_change` action (its `args` are left to the caller; the emitted input's
    /// own id already carries which field changed).
    pub async fn from_dictionary(mut self, dictionary_json: &Value, on_change: ActionId) -> UiAssemblyResult<Self> {
        let Some(entries) = dictionary_json.as_array() else { return Ok(self) };
        for entry in entries {
            let Some(id) = entry.get("id").and_then(Value::as_str) else { continue };
            let label = ui_label(entry.get("label").and_then(Value::as_str).unwrap_or(id), "form-panel.dictionary-label")?;
            let description = entry.get("description").and_then(Value::as_str).map(str::to_string);
            let value = ui_text(entry.get("value").and_then(Value::as_str).unwrap_or_default(), "form-panel.dictionary-value")?;
            let field_id = self.field_id(id).await?;
            let builder = input(InputKind::Text).value(value);
            let builder = builder.try_id(&field_id).map_err(|_| ui_assembly_error("form-panel.dictionary-id"))?;
            let builder = builder.try_on(Trigger::Change, on_change.clone()).map_err(|_| ui_assembly_error("form-panel.dictionary-binding"))?;
            let control = builder.try_build().map_err(|_| ui_assembly_error("form-panel.dictionary-control"))?;
            self = self.field(id, label, description, control).await?;
        }
        Ok(self)
    }

    /// 📋️ Sets the trailing submit `Button` row. ⚠️ Decision: narrowed from `action: ActionDescriptor`
    /// to a bare [`ActionId`] bound as [`Trigger::Activate`] — same call as `tree_item_with_action`'s,
    /// same reasoning (see that function's doc).
    pub async fn submit<L: TryInto<Label>>(mut self, label: L, action: ActionId) -> UiAssemblyResult<Self> {
        let id = self.field_id("submit").await?;
        let icon = ui_text(IconName::CircleDot.as_str(), "form-panel.submit-icon")?;
        let builder = button(ui_label(label, "form-panel.submit-label")?).icon(icon);
        let builder = builder.try_id(&id).map_err(|_| ui_assembly_error("form-panel.submit-id"))?;
        let builder = builder.try_on(Trigger::Activate, action).map_err(|_| ui_assembly_error("form-panel.submit-binding"))?;
        self.submit = Some(builder.try_build().map_err(|_| ui_assembly_error("form-panel.submit-build"))?);
        Ok(self)
    }

    /// 📋️ Builds `Fields -> optional submit Button` under one wrapper, id'd by the builder's
    /// namespace. ⚠️ Decision: the old outer wrapper was `UiSectionNode { label: None, .. }` — mapped
    /// here to `ui::column()` (`ContainerRole::Plain`), not `ui::section("")`: a genuinely unlabeled
    /// box is exactly what `Plain` means, whereas `Section`'s whole point is a real header, so forcing
    /// an empty label through it would be the wrong component, not a faithful rename. `default_open`
    /// is dropped for the same reason — it only means something on a collapsible header this wrapper
    /// no longer has.
    pub async fn build(self) -> UiAssemblyResult<BuiltNode> {
        let builder = column().try_id(&self.namespace).map_err(|_| ui_assembly_error("form-panel.root-id"))?;
        let mut builder = builder.try_children(self.fields).map_err(|_| ui_assembly_error("form-panel.fields"))?;
        if let Some(submit) = self.submit {
            builder = builder.try_child(submit).map_err(|_| ui_assembly_error("form-panel.submit"))?;
        }
        builder.try_build().map_err(|_| ui_assembly_error("form-panel.root-build"))
    }
}

/// 📋️ A read-only entity-detail panel: `title`/`subtitle` header text, a `KeyValue` summary block built
/// from `entries`, and trailing action buttons. Mirrors `📓️recipe-plugin.md` §7's own worked example
/// verbatim: `Component::KeyValueList` has no `ui::*` builder yet (§4), so its `BuiltNode` is
/// hand-assembled; `actions` takes already-built nodes (`Vec<BuiltNode>`) rather than a bare props
/// struct, since a `Component::Button`'s `action`/`style` now live on the node, not `ButtonProps`.
pub async fn entity_detail<L: TryInto<Label>, E: IntoIterator<Item = KeyValueEntry>, A: IntoIterator<Item = BuiltNode>>(title: L, subtitle: Option<Label>, entries: E, actions: A) -> UiAssemblyResult<BuiltNode> {
    let mut children = BuiltChildren::default();
    let title = text(ui_label(title, "entity-detail.title")?).try_build().map_err(|_| ui_assembly_error("entity-detail.title-build"))?;
    children.try_push(title).map_err(|_| ui_assembly_error("entity-detail.children"))?;
    if let Some(subtitle) = subtitle {
        let subtitle = text(subtitle).try_build().map_err(|_| ui_assembly_error("entity-detail.subtitle-build"))?;
        children.try_push(subtitle).map_err(|_| ui_assembly_error("entity-detail.children"))?;
    }
    let mut fixed_entries = UiFixedList::default();
    for entry in entries {
        fixed_entries.try_push(entry).map_err(|_| ui_assembly_error("entity-detail.entries"))?;
    }
    let summary = BuiltNode::try_new("summary", Component::KeyValueList(KeyValueListProps { entries: fixed_entries })).map_err(|_| ui_assembly_error("entity-detail.summary"))?;
    children.try_push(summary).map_err(|_| ui_assembly_error("entity-detail.children"))?;
    for action in actions {
        children.try_push(action).map_err(|_| ui_assembly_error("entity-detail.actions"))?;
    }
    let builder = column().try_children(children).map_err(|_| ui_assembly_error("entity-detail.children"))?;
    builder.try_build().map_err(|_| ui_assembly_error("entity-detail.build"))
}

#[cfg(test)]
include!("../🧪️tests/🔬️app-form-kit/🦀️.rs");

/// 🗣️ Anything `resolve_labels` can resolve a label set from.
pub trait LabelAxes {
    fn locale(&self) -> Locale;
    fn terminology(&self) -> Terminology {
        Terminology::Native
    }
}

impl LabelAxes for ViewModel {
    fn locale(&self) -> Locale {
        self.locale
    }
    fn terminology(&self) -> Terminology {
        self.terminology
    }
}

/// 🗣️ Resolves the active label set for the shell-provided locale/terminology axes.
pub fn resolve_labels<L: AppLabels>(axes: &impl LabelAxes) -> &'static L {
    L::labels(axes.locale(), axes.terminology())
}

#[cfg(test)]
include!("../🧪️tests/🔬️app-terminology/🦀️.rs");

// 🎯️ Shared ~30x hand-rolled `fn x_action(action: &str, args: Option<DslValue>) -> ActionDescriptor {
// ActionDescriptor { controller_id: X_CONTROLLER_ID.into(), action: action.into(), args } }` body —
// every app keeps its own locally-named wrapper (so call
// sites never change), delegating to `ActionFactory::new(X_CONTROLLER_ID).action(action, args)`.

pub const UI_COMMAND_VALUE_DEPTH: usize = 64;

pub const UI_COMMAND_VALUE_ITEMS: usize = 256;

#[expect(clippy::large_enum_variant, reason = "Each bounded traversal frame retains its fixed map cursor inline; boxing would add a separate allocation for every nested map.")]
enum UiCommandJsonFrame {
    List { cursor: UiListCursor, output: Vec<DslValue> },
    Map { cursor: UiMapCursor, output: Vec<(String, DslValue)>, key: Option<String> },
}

/// 🔀️ Map-insert semantics (replace-if-present, else append) on `DslValue::Object`'s
/// insertion-ordered `Vec<(String, DslValue)>` — mirrors what `serde_json::Map::insert` gave the
/// old JSON-backed producer for free.
fn dsl_object_insert(entries: &mut Vec<(String, DslValue)>, key: String, value: DslValue) {
    match entries.iter_mut().find(|(existing, _)| *existing == key) {
        Some(entry) => entry.1 = value,
        None => entries.push((key, value)),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc(hidden)]
pub enum UiCommandJsonStep {
    MoreWork,
    Ready,
    Cancelled,
    Fault,
}

#[doc(hidden)]
pub struct UiCommandJsonProducer {
    stack: UiFixedList<UiCommandJsonFrame, UI_COMMAND_VALUE_DEPTH>,
    pending: Option<UiValue>,
    ready: Option<DslValue>,
    fault: Option<Fault>,
    items: usize,
}

impl UiCommandJsonProducer {
    fn try_new(value: &UiValue) -> Result<Self, Fault> {
        let pending = value.credited_clone().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("app.intent.bridge-admission"), "typed intent value alias admission refused before conversion"))?;
        Ok(Self { stack: UiFixedList::default(), pending: Some(pending), ready: None, fault: None, items: 0 })
    }

    fn reject(&mut self, code: &'static str, detail: &'static str) -> UiCommandJsonStep {
        self.pending = None;
        self.stack = UiFixedList::default();
        self.fault = Some(Fault::new(FaultOrigin::Framework, FaultCode::new(code), detail));
        UiCommandJsonStep::Fault
    }

    fn accept(&mut self, value: DslValue) -> UiCommandJsonStep {
        let Some(items) = self.items.checked_add(1).filter(|items| *items <= UI_COMMAND_VALUE_ITEMS) else {
            return self.reject("app.intent.bridge-items", "typed intent value exceeds the fixed command item census");
        };
        self.items = items;
        let Some(frame) = self.stack.last_mut() else {
            self.ready = Some(value);
            return UiCommandJsonStep::Ready;
        };
        match frame {
            UiCommandJsonFrame::List { output, .. } => {
                if output.try_reserve(1).is_err() {
                    return self.reject("app.intent.bridge-allocation", "typed intent list candidate allocation was refused");
                }
                output.push(value);
            }
            UiCommandJsonFrame::Map { output, key, .. } => {
                let Some(key) = key.take() else {
                    return self.reject("app.intent.bridge-map-key", "typed intent map child lost its retained key");
                };
                dsl_object_insert(output, key, value);
            }
        }
        UiCommandJsonStep::MoreWork
    }

    fn drive_one(&mut self, cancelled: bool, deadline_expired: bool) -> UiCommandJsonStep {
        if self.ready.is_some() {
            return UiCommandJsonStep::Ready;
        }
        if self.fault.is_some() {
            return UiCommandJsonStep::Fault;
        }
        if cancelled {
            self.pending = None;
            self.stack = UiFixedList::default();
            self.fault = Some(Fault::new(FaultOrigin::Framework, FaultCode::new("app.intent.bridge-cancelled"), "typed intent value conversion was cancelled"));
            return UiCommandJsonStep::Cancelled;
        }
        if deadline_expired {
            return UiCommandJsonStep::MoreWork;
        }
        if let Some(value) = self.pending.take() {
            return match value {
                UiValue::Null => self.accept(DslValue::Null),
                UiValue::Bool(value) => self.accept(DslValue::Bool(value)),
                UiValue::Number(value) => self.accept(DslValue::json_number(value)),
                UiValue::Text(value) => self.accept(DslValue::String(value.to_string())),
                UiValue::List(values) => {
                    let frame = UiCommandJsonFrame::List { cursor: values.cursor(), output: Vec::new() };
                    match self.stack.try_push(frame) {
                        Ok(()) => UiCommandJsonStep::MoreWork,
                        Err(_) => self.reject("app.intent.bridge-depth", "typed intent list exceeds the fixed command depth"),
                    }
                }
                UiValue::Map(values) => {
                    let frame = UiCommandJsonFrame::Map { cursor: values.cursor(), output: Vec::new(), key: None };
                    match self.stack.try_push(frame) {
                        Ok(()) => UiCommandJsonStep::MoreWork,
                        Err(_) => self.reject("app.intent.bridge-depth", "typed intent map exceeds the fixed command depth"),
                    }
                }
            };
        }
        let Some(frame) = self.stack.last_mut() else {
            return self.reject("app.intent.bridge-state", "typed intent conversion lost its retained frame");
        };
        let completed = match frame {
            UiCommandJsonFrame::List { cursor, output } => match cursor.next() {
                Some(value) => {
                    self.pending = Some(value);
                    return UiCommandJsonStep::MoreWork;
                }
                None => DslValue::Array(std::mem::take(output)),
            },
            UiCommandJsonFrame::Map { cursor, output, key } => match cursor.next() {
                Some((next_key, value)) => {
                    *key = Some(next_key.to_string());
                    self.pending = Some(value);
                    return UiCommandJsonStep::MoreWork;
                }
                None => DslValue::Object(std::mem::take(output)),
            },
        };
        let _ = self.stack.pop();
        self.accept(completed)
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    fn take_ready(mut self) -> Result<DslValue, Self> {
        self.ready.take().ok_or(self)
    }

    fn take_fault(&mut self) -> Option<Fault> {
        self.fault.take()
    }
}

#[doc(hidden)]
pub async fn ui_value_to_dsl_retained(value: &UiValue) -> Result<DslValue, Fault> {
    let mut producer = UiCommandJsonProducer::try_new(value)?;
    loop {
        match producer.drive_one(false, false) {
            UiCommandJsonStep::MoreWork => semio_framework_async::yield_once().await,
            UiCommandJsonStep::Ready => return producer.take_ready().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("app.intent.bridge-publication"), "typed intent candidate was not complete")),
            UiCommandJsonStep::Cancelled | UiCommandJsonStep::Fault => {
                return Err(producer.take_fault().unwrap_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("app.intent.bridge-fault"), "typed intent conversion faulted")));
            }
        }
    }
}

/// 🔀️ M1: advances a firing `ActionBinding`'s echoed `args` and trigger `input` through
/// retained fixed-depth cursors into one complete typed-command `DslValue` candidate — the
/// `DslValue` `ArtifactApp::command_from_action` takes as `Option<&DslValue>` — `input` wins on
/// key collision when both are objects (the trigger-specific payload is the more specific,
/// more recent datum); otherwise a present `input` replaces `args` wholesale (a scalar `input`,
/// e.g. `Trigger::Delta`'s signed step count, has no field to merge INTO). `None` when neither is
/// set, matching `command_from_action`'s existing `None` (no-args) call shape.
// 🚫️async: E1-adjacent pure structural transform consumed by typed intent dispatch.
#[doc(hidden)]
pub async fn merge_ui_values(args: Option<&UiValue>, input: Option<&UiValue>) -> Result<Option<DslValue>, Fault> {
    let args = match args {
        Some(value) => Some(ui_value_to_dsl_retained(value).await?),
        None => None,
    };
    let input = match input {
        Some(value) => Some(ui_value_to_dsl_retained(value).await?),
        None => None,
    };
    Ok(match (args, input) {
        (None, None) => None,
        (Some(args), None) => Some(args),
        (None, Some(input)) => Some(input),
        (Some(DslValue::Object(mut args)), Some(DslValue::Object(input))) => {
            for (key, value) in input {
                dsl_object_insert(&mut args, key, value);
            }
            Some(DslValue::Object(args))
        }
        (Some(_), Some(input)) => Some(input),
    })
}

#[path = "../🖱️context-menu/🦀️.rs"]
pub mod context_menu;

pub use context_menu::{delete_selection, node_graph_delete_selection_spec, nothing_selected, selection_count_phrase, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch, SelectionKind};

/// 🪟️ A framework window kit: declares its read-only/editable `WindowKindDefinition` pair and
/// renders its plain owned `ViewModel` into a semantic contract node. Product surfaces encode
/// their typed scene payload through `semio_framework_ui_scene`; ordinary widgets use the
/// contract builder DSL directly, so no renderer-owned `UiNode` crosses this SDK boundary.
pub trait WindowKit {
    type ViewModel;
    const KIND_ID: &'static str;
    fn window_kind() -> WindowKindDefinition;
    fn editable_window_kind() -> WindowKindDefinition;
    fn render(view: &Self::ViewModel) -> UiAssemblyResult<BuiltNode>;
}


/// 🎯️ Constructs [`ActionId`]s bound to one controller id (renamed `scope`, per
/// `📓️recipe-plugin.md` §1's direct `ActionId::v1(scope, name)` mapping). ⚠️ Decision: the old
/// `ActionDescriptor` bundled a controller-scoped id AND its optional args into one value a caller
/// assigned straight to a node's sole `action` field; the new model splits "what to call"
/// (`ActionId`) from "when" (`Trigger`, chosen wherever `.on`/`.on_with` is actually called on a
/// node builder) — a factory scoped only to one controller cannot know the trigger, so trigger
/// selection stays out of scope here too. `args` is still accepted and converted
/// (`UiValue`), just returned alongside the id rather than folded into one struct — call
/// `.on_with(trigger, id, args)` when `args.is_some()`, `.on(trigger, id)` otherwise.
pub struct ActionFactory {
    #[doc(hidden)]
    pub controller_id: &'static str,
}

impl ActionFactory {
    pub const fn new(controller_id: &'static str) -> Self {
        Self { controller_id }
    }

    pub fn action(&self, action: &str, args: Option<UiValue>) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
        let id = ActionId::try_v1(self.controller_id, action).ok_or_else(|| ui_assembly_error("action-factory.id"))?;
        Ok((id, args))
    }
}

/// 📇️ An app's action declarations indexed by id, built from its {@link AppDefinition}. Threaded
/// into {@link VcsArtifactApp} at registration time so the wrapper can enforce the actions contract
/// (default materialization, required-arg validation, kind discipline) without the plugin re-checking.
/// An empty registry is a store-only construction path; command dispatch rejects every key.
#[derive(Clone, Default)]
pub struct AppActionRegistry {
    #[doc(hidden)]
    pub actions: HashMap<String, ActionDefinition>,
    window_actions: HashMap<String, HashMap<String, ActionDefinition>>,
    app_commands: HashMap<String, CommandDefinition>,
    mode_commands: HashMap<String, HashMap<String, CommandDefinition>>,
    /// 📇️ The app's `controller_id` — addresses `ActionDescriptor.controller_id` for the
    /// framework-built history panel. Empty for the registry-less test path.
    #[doc(hidden)]
    pub controller_id: String,
    /// 🕹️ The app's declared `InteractionDefinition`s indexed by domain id — `dispatch_interaction_action`'s
    /// sole source of domain declarations (granularities, hierarchy, hover/selection specs). Empty
    /// for the registry-less test path, exactly like `actions`/`commands` — an app constructed via
    /// `VcsArtifactApp::new` therefore has no interaction domains to intercept, matching the fact
    /// that `AppBuilder::interaction(...)` calls never reach a registry built that way either.
    interactions: HashMap<String, InteractionDefinition>,
    /// ⏯️ Every tool and utility that declares a `ToolRunDefinition`, with its label, keyed by id —
    /// the manifest is the run's source of record (tool run contract §2.4).
    tool_runs: HashMap<String, (LocalizedLabel, semio_framework_tool_run::ToolRunDefinition)>,
    /// 🪟️ Every declared window kind's body key — how a run's declared `windows` resolve to the bodies a tick dirties.
    window_body_keys: HashMap<String, String>,
    /// 🕹️ Every declared window body key indexed by the interaction domain that window kind
    /// declares (`semio_framework::interaction_window_bodies_by_domain`) — the schema-first input
    /// of `interaction_declared_refresh_scope`, read once here instead of per dispatched hover.
    interaction_window_bodies: BTreeMap<String, Vec<String>>,
    /// 🌳️ Every declared panel leaf body key, the panel half of the same derivation.
    panel_body_keys: Vec<String>,
    /// 🧹️ Identifiers of catalog rows already detached by close, paged down one grant at a time.
    retiring: Vec<Vec<u8>>,
}

#[doc(hidden)]
pub fn validate_ui_dispatch_classification(owner: &str, id: &str, classification: semio_framework::InteractiveJobClassification) -> Result<(), Fault> {
    if classification == semio_framework::InteractiveJobClassification::Migrated {
        Ok(())
    } else {
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.not-ui-safe"), format!("UI dispatch rejected {owner}:{id} with interactive-job classification {classification:?}")))
    }
}

impl AppActionRegistry {
    #[cfg(test)]
    #[doc(hidden)]
    pub fn test_with_controller_id(controller_id: String) -> Self {
        Self { controller_id, ..Self::default() }
    }

    /// 📇️ Indexes an app definition's declared actions and commands (including
    /// framework-injected ones) by id.
    ///
    /// 🪪️ `actions` is the app-level index every dispatch and every tool-proof join
    /// (`migrated_tool_ids`) reads, so it must hold EVERY action the app declares, not only the ones
    /// some window happens to carry. Two sources feed it, in this order:
    ///
    /// 1. the per-window rosters, each resolved through `semio_framework::window_kind_actions` — the
    ///    window's own declarations plus every app-level action no window kind claims;
    /// 2. `semio_framework::interaction_action_definitions`, the six verbs an app's declared
    ///    `InteractionDefinition` domains inject. An `AppDefinition` assembled as a plain struct
    ///    literal (legal — every field is `pub`) never went through the builder's injection pass, so
    ///    without this second source its interaction verbs would be absent from the registry and every
    ///    bounded tool proof keyed on them would fail closed with an EMPTY `migrated` set
    ///    (`interactive-job.catalog-authority`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.3).
    ///
    /// A window roster wins over the injected twin for the same id: it is the authored declaration.
    pub fn from_definition(definition: &AppDefinition) -> Self {
        let app_commands = definition.commands.iter().map(|command| (command.id.clone(), command.clone())).collect();
        let mode_commands = definition.modes.iter().map(|mode| (mode.id.clone(), mode.commands.iter().map(|command| (command.id.clone(), command.clone())).collect())).collect();
        let window_actions: HashMap<String, HashMap<String, ActionDefinition>> =
            definition.window_kinds.iter().map(|window| (window.id.clone(), semio_framework::window_kind_actions(definition, window).into_iter().map(|action| (action.id.clone(), action.clone())).collect())).collect();
        let mut actions: HashMap<String, ActionDefinition> = semio_framework::interaction_action_definitions(definition).into_iter().map(|action| (action.id.clone(), action)).collect();
        actions.extend(window_actions.values().flat_map(|window| window.iter().map(|(id, action)| (id.clone(), action.clone()))));
        let tool_runs = definition
            .tools
            .iter()
            .filter_map(|tool| tool.run.clone().map(|run| (tool.id.clone(), (tool.label.clone(), run))))
            .chain(definition.utilities.iter().filter_map(|utility| utility.run.clone().map(|run| (utility.id.clone(), (utility.label.clone(), run)))))
            .collect();
        let window_body_keys = definition.window_kinds.iter().map(|window| (window.id.clone(), window.body_key.clone())).collect();
        let interaction_window_bodies = semio_framework::interaction_window_bodies_by_domain(definition);
        let panel_body_keys = semio_framework::panel_leaf_body_keys(definition);
        Self {
            actions,
            window_actions,
            app_commands,
            mode_commands,
            controller_id: definition.controller_id.clone(),
            interactions: definition.interactions.iter().map(|interaction| (interaction.id.clone(), interaction.clone())).collect(),
            tool_runs,
            window_body_keys,
            interaction_window_bodies,
            panel_body_keys,
            retiring: Vec::new(),
        }
    }

    /// 📏️ Quotes the next catalog close turn: releasing the newest detached identifier allocation whole, or detaching one row.
    #[doc(hidden)]
    pub fn retirement_demands(&self) -> RetirementDemand {
        if self.terminal_is_empty() {
            return RetirementDemand::default();
        }
        RetirementDemand { release_bytes: self.retiring.last().map_or(0, Vec::capacity), depth: 1, ..Default::default() }
    }

    /// 🧹 Detaches one catalog row or one empty nested catalog owner whole (one handed-off item), then
    /// releases each detached identifier allocation whole on the release axis; a turn granted below the quoted
    /// release demand yields instead of refusing the row for ever.
    #[doc(hidden)]
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let demand = self.retirement_demands();
        if grant.maximum_depth < demand.depth {
            return Err(plugin_sdk_fault("action registry close exceeds its admitted depth"));
        }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(identifier) = self.retiring.pop() {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: identifier.capacity(), ..Default::default() }));
        }
        let detached = self.detach_one_row();
        self.retiring.extend(detached.into_iter().filter(|identifier| !identifier.is_empty()).map(String::into_bytes));
        let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    fn detach_one_row(&mut self) -> Vec<String> {
        fn first<V>(map: &mut HashMap<String, V>) -> Option<String> {
            let key = map.keys().next().cloned()?;
            map.remove_entry(&key).map(|(key, _)| key)
        }
        if let Some(key) = first(&mut self.actions) {
            return vec![key];
        }
        if let Some(row) = self.window_actions.values_mut().find_map(first) {
            return vec![row];
        }
        if let Some(owner) = first(&mut self.window_actions) {
            return vec![owner];
        }
        if let Some(row) = self.mode_commands.values_mut().find_map(first) {
            return vec![row];
        }
        if let Some(owner) = first(&mut self.mode_commands) {
            return vec![owner];
        }
        if let Some(key) = first(&mut self.app_commands) {
            return vec![key];
        }
        if let Some(key) = first(&mut self.tool_runs) {
            return vec![key];
        }
        if let Some(key) = first(&mut self.interactions) {
            return vec![key];
        }
        if let Some(key) = self.window_body_keys.keys().next().cloned() {
            return self.window_body_keys.remove_entry(&key).map_or_else(Vec::new, |(key, body)| vec![key, body]);
        }
        if let Some((domain, bodies)) = self.interaction_window_bodies.pop_first() {
            return std::iter::once(domain).chain(bodies).collect();
        }
        if let Some(body) = self.panel_body_keys.pop() {
            return vec![body];
        }
        if !self.controller_id.is_empty() {
            return vec![std::mem::take(&mut self.controller_id)];
        }
        Vec::new()
    }

    /// 🧺 Proves that every catalog row and nested catalog owner was retired.
    #[doc(hidden)]
    pub fn terminal_is_empty(&self) -> bool {
        self.actions.is_empty()
            && self.window_actions.is_empty()
            && self.app_commands.is_empty()
            && self.mode_commands.is_empty()
            && self.controller_id.is_empty()
            && self.interactions.is_empty()
            && self.tool_runs.is_empty()
            && self.window_body_keys.is_empty()
            && self.interaction_window_bodies.is_empty()
            && self.panel_body_keys.is_empty()
            && self.retiring.is_empty()
    }

    /// 🪟️ The body key of a declared window kind.
    #[doc(hidden)]
    pub fn window_body_key(&self, window_kind_id: &str) -> Option<&str> {
        self.window_body_keys.get(window_kind_id).map(String::as_str)
    }

    /// 🐢️ The refresh scope the app's OWN surface declarations owe one reserved interaction verb —
    /// `None` when no declared window paints a touched domain, which is the framework's signal to
    /// keep its widest `UiDirtyScope::Full`.
    #[doc(hidden)]
    pub fn interaction_declared_refresh_scope(&self, verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        semio_framework::interaction_declared_refresh_scope(verb, domains, &self.interaction_window_bodies, &self.panel_body_keys)
    }

    /// ⏯️ The declared label and `ToolRunDefinition` of one tool or utility.
    #[doc(hidden)]
    pub fn tool_run(&self, id: &str) -> Option<&(LocalizedLabel, semio_framework_tool_run::ToolRunDefinition)> {
        self.tool_runs.get(id)
    }

    #[doc(hidden)]
    pub fn get(&self, id: &str) -> Option<&ActionDefinition> {
        self.actions.get(id)
    }

    #[doc(hidden)]
    pub async fn window_action(&self, window_kind_id: &str, action_id: &str) -> Option<&ActionDefinition> {
        self.window_actions.get(window_kind_id)?.get(action_id)
    }

    #[doc(hidden)]
    pub async fn has_mode(&self, mode_id: &str) -> bool {
        self.mode_commands.contains_key(mode_id)
    }

    #[doc(hidden)]
    pub fn get_command(&self, id: &str) -> Option<&CommandDefinition> {
        self.app_commands.get(id).or_else(|| self.mode_commands.values().find_map(|commands| commands.get(id)))
    }

    #[doc(hidden)]
    pub async fn app_command(&self, id: &str) -> Option<&CommandDefinition> {
        self.app_commands.get(id)
    }

    #[doc(hidden)]
    pub async fn mode_command(&self, mode_id: &str, id: &str) -> Option<&CommandDefinition> {
        self.mode_commands.get(mode_id)?.get(id)
    }

    /// 🫙️ Whether this registry declares NOTHING — the registry-less path `AppActionRegistry::default()`
    /// builds (see `controller_id`'s doc). It has no controller, no action and no command, so it joins no
    /// tool to any factory and every dispatch through it fails closed at its own key lookup.
    #[doc(hidden)]
    pub fn is_registry_less(&self) -> bool {
        self.controller_id.is_empty() && self.actions.is_empty() && self.window_actions.is_empty() && self.app_commands.is_empty() && self.mode_commands.is_empty()
    }

    #[doc(hidden)]
    pub fn migrated_tool_ids(&self) -> BTreeSet<String> {
        let mut ids = BTreeSet::new();
        ids.extend(self.actions.iter().filter(|(_, definition)| definition.semantics.execution.interactive_job == semio_framework::InteractiveJobClassification::Migrated).map(|(id, _)| id.clone()));
        ids.extend(self.app_commands.iter().filter(|(_, definition)| definition.semantics.execution.interactive_job == semio_framework::InteractiveJobClassification::Migrated).map(|(id, _)| id.clone()));
        for commands in self.mode_commands.values() {
            ids.extend(commands.iter().filter(|(_, definition)| definition.semantics.execution.interactive_job == semio_framework::InteractiveJobClassification::Migrated).map(|(id, _)| id.clone()));
        }
        ids
    }

    #[cfg(test)]
    fn test_migrated_tool_ids(&self) -> BTreeSet<String> {
        self.migrated_tool_ids()
    }

    #[cfg(test)]
    fn test_controller_id(&self) -> &str {
        &self.controller_id
    }

    #[cfg(test)]
    #[doc(hidden)]
    pub fn test_set_action_classification(&mut self, id: &str, classification: semio_framework::InteractiveJobClassification) {
        self.actions.get_mut(id).expect("test action declaration").semantics.execution.interactive_job = classification;
    }

    #[cfg(test)]
    #[doc(hidden)]
    pub fn test_set_app_command_classification(&mut self, id: &str, classification: semio_framework::InteractiveJobClassification) {
        self.app_commands.get_mut(id).expect("test app command declaration").semantics.execution.interactive_job = classification;
    }

    /// 🕹️ `domain_id`'s declared `InteractionDefinition`, `None` when undeclared.
    #[doc(hidden)]
    pub async fn interaction(&self, domain_id: &str) -> Option<&InteractionDefinition> {
        self.interactions.get(domain_id)
    }

    /// 🕹️ Every declared interaction domain, in no particular order — `clearSelection`/`selectAll`
    /// (which carry no `domainId` arg, applying to every declared domain at once) iterate this.
    #[doc(hidden)]
    pub async fn interactions(&self) -> impl Iterator<Item = &InteractionDefinition> {
        self.interactions.values()
    }

    /// 🗂️ Ribbon-parent-taxonomy category (a `ui_wgpu::wgpu::RIBBON_PARENT_CATEGORIES` id) for a declared
    /// action id — the `organize_context_menu` `category_of` lookup at the `VcsArtifactApp::context_menu`
    /// funnel. `None` for a command id (`CommandDefinition.category` is an unrelated footer-tab
    /// grouping, not this taxonomy) and for any action that never called `.with_category(...)`.
    /// 🚫️async: E1-adjacent pure lookup consumed by `organize_context_menu`'s sync `Fn(&str) ->
    /// Option<String>` category closure — see R9.
    pub fn category_of(&self, id: &str) -> Option<String> {
        self.actions.get(id).and_then(|action| action.category.clone())
    }
}

#[cfg(test)]
#[path = "🧪️tests/📤️json-bridge/🦀️.rs"]
mod command_json_bridge_tests;

#[path = "🎬️media/🦀️.rs"]
pub mod media_kit;
#[path = "📝️text/🦀️.rs"]
pub mod text_kit;
#[path = "🖼️image/🦀️.rs"]
pub mod image_kit;
#[path = "🧊️mesh/🦀️.rs"]
pub mod mesh_kit;
#[path = "🌍️world3d/☀️sun/🦀️.rs"]
pub mod world3d_sun;
