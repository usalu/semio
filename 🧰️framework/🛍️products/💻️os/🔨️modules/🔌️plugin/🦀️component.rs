//! 🔌️ Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM plugins.

/// 🚧️ A wasm32-wasip2 component exports either `plugin-world` or `extension-world`, never both —
/// `component-guest` and `component-extension-guest` are mutually exclusive for that target.
#[cfg(all(feature = "component-guest", feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2"))]
compile_error!("`component-guest` and `component-extension-guest` are mutually exclusive for wasm32-wasip2 targets");

#[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
pub mod component {
    //! 🧩️ WASI P2 component exports for the plugin world contract.
    #![allow(unsafe_op_in_unsafe_fn)]

    use crate::plugin_runtime::{ensure_plugin_initialized, plugin_clear_instance_guard, plugin_create_app, plugin_exchange, plugin_manifest};
    use wit_bindgen::generate;

    generate!({
        world: "plugin-world",
        path: "📜️wit",
    });

    use exports::semio::framework::plugin::Guest;
    use semio::framework::types::{MigrateArtifactInput as MigrateDocumentInput, MigrateArtifactOutput as MigrateDocumentOutput, PluginError};
    use semio_framework::{Fault, FaultCode, FaultOrigin};

    pub struct ComponentGuest;

    impl Guest for ComponentGuest {
        fn manifest() -> Vec<u8> {
            ensure_plugin_initialized();
            store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&plugin_manifest()).unwrap_or(dsl::DslValue::Null))
        }

        fn instantiate_app(app_id: String, _instance_id: String) -> Result<u32, PluginError> {
            ensure_plugin_initialized();
            plugin_create_app(&app_id).map_err(|fault| PluginError::Fault(dsl::encode_fault_bytes(&fault)))
        }

        fn exchange(instance_id: u32, commands: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, PluginError> {
            ensure_plugin_initialized();
            plugin_exchange(instance_id, &commands).map_err(|fault| PluginError::Fault(dsl::encode_fault_bytes(&fault)))
        }

        fn migrate_artifact(_input: MigrateDocumentInput) -> Result<MigrateDocumentOutput, PluginError> {
            Err(PluginError::Fault(dsl::encode_fault_bytes(&Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.migrate-document"), "migrate-document not implemented"))))
        }

        fn clear_instance_guard() {
            plugin_clear_instance_guard();
        }

        fn list_artifact_dialects() -> Vec<u8> {
            ensure_plugin_initialized();
            crate::wire_list_composer_entries()
        }

        fn artifact_compose(key: Vec<u8>, sources: Vec<u8>) -> Result<Vec<u8>, PluginError> {
            ensure_plugin_initialized();
            crate::wire_artifact_compose(&key, &sources)
                .map_err(|message| PluginError::Fault(dsl::encode_fault_bytes(&Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.artifact-compose"), message))))
        }
    }

    export!(ComponentGuest);

    pub fn component_export_anchor() {}

    pub fn host_backbone_send(uri: &str, message: &[u8]) -> Result<(), String> {
        semio::framework::host::backbone_send(uri, message).map_err(|fault| dsl::decode_fault_bytes(&fault).message)
    }

    pub fn host_backbone_poll(uri: &str) -> Result<Vec<Vec<u8>>, String> {
        semio::framework::host::backbone_poll(uri).map_err(|fault| dsl::decode_fault_bytes(&fault).message)
    }

    pub fn host_backbone_status(uri: &str) -> Result<String, String> {
        semio::framework::host::backbone_status(uri).map_err(|fault| dsl::decode_fault_bytes(&fault).message)
    }

    pub fn host_now_ms() -> i64 {
        semio::framework::host::now_ms()
    }

    pub fn host_read_asset(handle: u64) -> Result<Vec<u8>, String> {
        semio::framework::host::read_asset(handle).map_err(|fault| dsl::decode_fault_bytes(&fault).message)
    }

    /// 📦️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION (D3): `kind`/
    /// `direction` ("import"|"export") in, JSON `Vec<io::ArtifactDialect>` bytes out.
    pub fn host_io_dialects(kind: &str, direction: &str) -> Result<Vec<u8>, String> {
        semio::framework::host::io_dialects(kind, direction).map_err(|bytes| dsl::decode_fault_bytes(&bytes).message)
    }

    /// 📦️ Routes a compose request to whichever OTHER plugin the host's `IoRouter` says owns `key`
    /// — the guest-side half of cross-plugin reuse. `key`/`sources`/result are the same JSON shapes
    /// `wire_artifact_compose`/`wire_list_composer_entries` use.
    pub fn host_io_compose(key: &[u8], sources: &[u8]) -> Result<Vec<u8>, String> {
        semio::framework::host::io_compose(key, sources).map_err(|bytes| dsl::decode_fault_bytes(&bytes).message)
    }

    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): resolves an `io::ArtifactLink`'s encoded bytes
    /// into the linked artifact's current pack via the host `resolve-artifact-link` import — the
    /// ONE seam a WASI guest ever touches for link resolution; it must never resolve a link itself
    /// (no guest has host-store access). The WIT error is already a plain `string` (unlike every
    /// other `host::` import here), so no `decode_fault_bytes` step is needed.
    pub fn host_resolve_artifact_link(link: &[u8]) -> Result<Vec<u8>, String> {
        semio::framework::host::resolve_artifact_link(link)
    }

    /// 🌉️ Installs the guest-side `io_dispatch` fallback hook: a local registry miss is retried via
    /// the host's `io-compose` import, which routes to whichever OTHER loaded plugin owns the key.
    /// Called once from `ensure_plugin_initialized` — see that function's own doc comment for why
    /// this can't live in the (non-wasm-gated) `plugin_runtime` module directly.
    pub fn install_io_fallback_dispatcher() {
        crate::set_io_fallback_dispatcher(|key, sources| {
            let key_bytes = serde_json::to_vec(key).ok()?;
            let wire_sources: Vec<crate::WireComposeSource> = sources
                .iter()
                .map(|source| crate::WireComposeSource { dialect: crate::ArtifactDialect::from(source.dialect), payload: source.payload.clone() })
                .collect();
            let sources_bytes = serde_json::to_vec(&wire_sources).ok()?;
            match host_io_compose(&key_bytes, &sources_bytes) {
                Ok(result_bytes) => Some(crate::wire_decode_composed_artifact(&result_bytes).map_err(|message| crate::ComposeError { message, diagnostics: Vec::new() })),
                Err(message) => Some(Err(crate::ComposeError { message, diagnostics: Vec::new() })),
            }
        });
    }
}

#[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
pub use component::component_export_anchor;

#[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
pub fn component_export_anchor() {}

#[cfg(all(feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2"))]
pub mod extension_component {
    //! 🧩️ WASI P2 component exports for the `extension-world` contract — a standalone wasm
    //! component surface for runtime-installable extensions, instantiated on their own instead of
    //! only ever piggybacking on a `plugin-world` component (the previous workaround).
    #![allow(unsafe_op_in_unsafe_fn)]

    use crate::plugin_runtime::{extension_activate, extension_deactivate, extension_invoke, extension_manifest};
    use wit_bindgen::generate;

    generate!({
        world: "extension-world",
        path: "📜️wit",
    });

    use exports::semio::framework::extension::Guest;
    use semio::framework::types::PluginError;

    pub struct ExtensionComponentGuest;

    impl Guest for ExtensionComponentGuest {
        fn manifest() -> Vec<u8> {
            store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&extension_manifest()).unwrap_or(dsl::DslValue::Null))
        }

        fn activate() -> Result<(), PluginError> {
            extension_activate().map_err(|fault| PluginError::Fault(dsl::encode_fault_bytes(&fault)))
        }

        fn deactivate() {
            extension_deactivate();
        }

        fn invoke(capability: String, request: Vec<u8>) -> Result<Vec<u8>, PluginError> {
            extension_invoke(&capability, &request).map_err(|fault| PluginError::Fault(dsl::encode_fault_bytes(&fault)))
        }
    }

    export!(ExtensionComponentGuest);

    pub fn extension_component_export_anchor() {}
}

#[cfg(all(feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2"))]
pub use extension_component::extension_component_export_anchor;

#[cfg(not(all(feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2")))]
pub fn extension_component_export_anchor() {}

#[path = "🏗️builder/🦀️component.rs"]
mod builder;

pub mod app {
    // #region app
    //! 🧩️ Declarative app builder and plugin trait.

    use dsl::{to_dsl_value, DslValue};
    use protocol::OpText;
    use semio_framework::{
        clipboard_action_definitions, element_id_segment, history_action_definitions, is_element_id,
        kernel::{
            ActorId, AppEvent, ArtifactKind, CapabilityRequirement, ClipboardError, ClipboardFragment, ArtifactDiff, ArtifactHandle, ArtifactVersion, EditRef, HostEffect, HybridLogicalTimestamp, InverseMutation, InvocationId, InvocationResult,
            KernelMutation, MutationId, PastePlacement, Rights, SchemaId, Scope, UndoGroup, UndoPolicy,
        },
        note_shell_command_action_definition, record_tutorial_action_definition, set_active_tool_action_definition, set_active_utility_action_definition, set_history_command_filter_action_definition, start_introduction_action_definition,
        start_tutorial_action_definition, ActionArgDef, ActionDefinition, ActionKind, ActionRef, AppDefinition, AppIo, CommandDefinition, CommandGrammar, CommandRef, CommandScope, ConfigSpec, DialogDefinition, ExampleDefinition,
        IconName, IntroductionDefinition, IntroductionInteractionKind, Keybinding, MediaForm, MediaPortDirection, MediaPortSpec, ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, PluginManifest, ToolDefinition, ToolRef,
        TutorialDefinition, UtilityDefinition, UtilityRef, ViewModel, WindowKindDefinition, WindowKinds, Fault, FaultCode, FaultFrom, FaultOrigin, NOTE_SHELL_COMMAND_ACTION_ID, RECORD_TUTORIAL_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID,
        SET_HISTORY_COMMAND_FILTER_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID,
    };
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::collections::{HashMap, HashSet};
    use store::{
        build_history_columns, child_store_factory, create_config_envelope, create_document_envelope, ArtifactCommand, ArtifactPack, ArtifactStore, ChildDispatch, ChildGenesis, ChildStoreFactory, CompositionCoordinator, ConfigStore, EngineHandles,
        GroupMeta, GroupReceipt, HistoryColumn, OwnerRef, SpaceConflict, SpaceMember,
    };
    /// 🚪️ `os_io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
    /// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
    /// the `os_io` mount), so it is named through the `store::os_io::` path everywhere in this file,
    /// exactly like the sibling `🎞️gif` migration leaf (`store::os_io::ArtifactDialect`) already does.
    use store::os_io::{ArtifactKindId, ArtifactRef};
    use ui_wgpu::wgpu::{
        collect_window_kind_ids_from_layout, ui_control_to_node, ui_stack_vertical, ui_text, ui_tree_stamp_presence, ActionDescriptor, ContextMenuItemSpec, ContextMenuRequest, ContextMenuSurfaceTarget, Label, Locale, LocalizedLabel, NamedLayout,
        SurfaceKind, Terminology, UiButtonNode, UiControlNode, UiFieldNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiNode, UiPresence, UiSectionNode, UiSelectItem, UiSelectNode, UiState, UiTreeActionPlacement, UiTreeItemAction,
        UiTreeItemNode, UiTreeNode, UiTreeSectionNode, WindowEngagement, WindowEngagementSlot, WindowLayout, WindowMeasure, WindowOptions, FRAMEWORK_HISTORY_BODY_KEY,
    };

    fn plugin_sdk_fault(message: impl Into<String>) -> Fault {
        Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), message)
    }

    pub struct ModeSpec {
        pub id: String,
        pub label: LocalizedLabel,
        pub icon_id: IconName,
        pub tools: Vec<ToolRef>,
        pub layout_id: Option<String>,
        pub commands: Vec<CommandRef>,
    }

    pub struct WindowKindSpec {
        pub id: String,
        pub label: LocalizedLabel,
        pub body_key: String,
        pub surface_kind: SurfaceKind,
        pub icon_id: IconName,
        pub measures: Vec<WindowMeasure>,
        pub engagement: Option<WindowEngagement>,
        pub actions: Vec<ActionRef>,
        pub utilities: Vec<UtilityRef>,
        /// 🧱️ Carried verbatim from `.window_kind_def(WindowKindDefinition)`; the scalar-arg constructors
        /// (`.window_kind()`/`.window_kind_with_engagement()`) always leave these `None`/empty, matching
        /// `build_definition`'s prior hardcoded defaults for those paths.
        pub params_schema: Option<String>,
        pub artifact_snapshot_schema: Option<String>,
        pub input_event_schema: Option<String>,
        pub output_schema: Option<String>,
        pub capabilities: Vec<CapabilityRequirement>,
    }

    /// 🌳️ A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it) — exactly one of the two.
    pub struct PanelTabSpec {
        pub kind: PanelTabKind,
        pub label: LocalizedLabel,
        pub group: PanelGroup,
        pub body_key: Option<String>,
        pub children: Vec<PanelTabSpec>,
    }

    impl PanelTabSpec {
        /// 🍃️ An app-declared leaf tab; `group` is only meaningful on the root entry passed to `.panel_tab_tree`.
        pub fn leaf(id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: impl Into<String>) -> Self {
            Self { kind: PanelTabKind::App(id.into()), label: label.into(), group, body_key: Some(body_key.into()), children: Vec::new() }
        }

        /// 🌳️ An app-declared branch tab; its `children` render as the tab row below it when active.
        pub fn group(id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, children: Vec<PanelTabSpec>) -> Self {
            Self { kind: PanelTabKind::App(id.into()), label: label.into(), group, body_key: None, children }
        }

        /// 🏛️ A framework-predefined tab — only the framework shell itself should ever pass a
        /// non-`App` `PanelTabKind` here; plugins must go through `leaf`/`group`.
        pub fn framework(kind: PanelTabKind, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: Option<String>, children: Vec<PanelTabSpec>) -> Self {
            Self { kind, label: label.into(), group, body_key, children }
        }
    }

    /// 🌳️ Asserts every tab in the tree has a non-empty, unique id and sets exactly one of `body_key`/`children`.
    fn validate_panel_tab_spec(app_id: &str, tab: &PanelTabSpec, seen_ids: &mut HashSet<String>) {
        let id = tab.kind.id_str();
        assert!(!id.trim().is_empty(), "app {} panel tab id must be non-empty", app_id);
        assert!(seen_ids.insert(id.to_string()), "app {} duplicate panel tab id {}", app_id, id);
        assert!(tab.body_key.is_some() != !tab.children.is_empty(), "app {} panel tab {} must set exactly one of body_key or children", app_id, id);
        if let Some(body_key) = &tab.body_key {
            assert!(!body_key.trim().is_empty(), "app {} panel tab {} body_key must be non-empty", app_id, id);
        }
        for child in &tab.children {
            validate_panel_tab_spec(app_id, child, seen_ids);
        }
    }

    /// 🌳️ Converts one plugin-declared `PanelTabSpec` (recursively) into a `PanelTabDefinition`.
    fn panel_tab_spec_to_definition(tab: PanelTabSpec) -> PanelTabDefinition {
        PanelTabDefinition { kind: tab.kind, label: tab.label, group: tab.group, body_key: tab.body_key, children: tab.children.into_iter().map(panel_tab_spec_to_definition).collect() }
    }

    /// 🔁️ Inverse of `ModeSpec` -> `ModeDefinition` in `build_definition` — lets `.mode_def()` accept an
    /// already-built `ModeDefinition` (e.g. from a taxonomy `🎭️modes/<mode>/🦀️component.rs` file) and
    /// store it through the same `ModeSpec` pipeline as the scalar `.mode(...)` args. Fields line up 1:1.
    fn mode_definition_to_spec(def: ModeDefinition) -> ModeSpec {
        ModeSpec { id: def.id, label: def.label, icon_id: def.icon_id, tools: def.tools, layout_id: def.layout_id, commands: def.commands }
    }

    /// 🔁️ Inverse of `WindowKindSpec` -> `WindowKindDefinition` in `build_definition` — unpacks
    /// `WindowOptions` back into `measures`/`engagement` so a full `WindowKindDefinition` can be pushed
    /// through the same `WindowKindSpec` pipeline as `.window_kind()`/`.window_kind_with_engagement()`.
    fn window_kind_definition_to_spec(def: WindowKindDefinition) -> WindowKindSpec {
        WindowKindSpec {
            id: def.id,
            label: def.label,
            body_key: def.body_key,
            surface_kind: def.surface_kind,
            icon_id: def.icon_id,
            measures: def.options.measures,
            engagement: def.options.engagement.as_option().cloned(),
            actions: def.actions,
            utilities: def.utilities,
            params_schema: def.params_schema,
            artifact_snapshot_schema: def.artifact_snapshot_schema,
            input_event_schema: def.input_event_schema,
            output_schema: def.output_schema,
            capabilities: def.capabilities,
        }
    }

    /// 🔁️ Inverse of `panel_tab_spec_to_definition` — lets `.panel_tab_def()` accept an already-built
    /// (possibly nested) `PanelTabDefinition` and store it through the same `PanelTabSpec` pipeline as
    /// `.panel_tab()`/`.panel_tab_tree()`.
    fn panel_tab_definition_to_spec(def: PanelTabDefinition) -> PanelTabSpec {
        PanelTabSpec { kind: def.kind, label: def.label, group: def.group, body_key: def.body_key, children: def.children.into_iter().map(panel_tab_definition_to_spec).collect() }
    }

    /// 📝️ Asserts every `ActionArgDef` in `args` (belonging to `owner`, e.g. an action or dialog id) has
    /// a non-empty, unique id and that any `Select` control declares at least one option — shared by
    /// per-action arg validation and dialog arg validation so both stay in lockstep.
    fn validate_arg_defs(app_id: &str, owner: &str, args: &[ActionArgDef]) {
        let mut arg_ids = HashSet::new();
        for arg in args {
            assert!(arg_ids.insert(arg.id.clone()), "app {} {} declares duplicate arg id {}", app_id, owner, arg.id);
            if let semio_framework::ActionArgControl::Select { options } = &arg.control {
                assert!(!options.is_empty(), "app {} {} arg {} is a Select with no options", app_id, owner, arg.id);
            }
        }
    }

    /// 🆔️ Shared by introduction- and tutorial-step validation: every referenced element id is
    /// grammar-checked always, plus a best-effort semantic check for the id shapes this app itself declares
    /// (utility ids, navbar/footer, panel tabs, window bodies — matched via `element_id_segment` since window
    /// kind ids are camelCased at the stamp site, never compared raw). Anything else grammar-valid is an
    /// escape hatch — an app may legitimately reference a plugin- or framework-owned element it doesn't
    /// declare.
    fn validate_referenced_element_id(app_id: &str, owner: &str, role: &str, id: &str, declared_utility_ids: &HashSet<String>, panel_tab_ids: &HashSet<String>, window_kind_ids: &HashSet<String>) {
        assert!(is_element_id(id), "app {} {} {} element id {} does not match the UI element id grammar", app_id, owner, role, id);
        if declared_utility_ids.contains(id) || id == UI_NAVBAR_ELEMENT_ID || id == UI_FOOTER_ELEMENT_ID {
            return;
        }
        if let Some(rest) = id.strip_prefix("framework.panelTab.") {
            let tab_id = rest.strip_suffix(".firstDraggable").unwrap_or(rest);
            assert!(panel_tab_ids.contains(tab_id), "app {} {} {} undeclared panel tab {}", app_id, owner, role, tab_id);
            return;
        }
        if let Some(rest) = id.strip_prefix("framework.window.") {
            let segment = rest.split('.').next().unwrap_or(rest);
            let declared = window_kind_ids.iter().any(|kind_id| element_id_segment(kind_id) == segment);
            assert!(declared, "app {} {} {} undeclared window kind for element id {}", app_id, owner, role, id);
        }
    }

    /// 👻️ Every `IntroductionPoint` a gesture references (one for click/scroll kinds, two for drag/orbit) —
    /// shared by tutorial gesture-cue validation; only `Element` points are grammar-checked (the other
    /// addressing schemes name windows/entities/curves, not the `ui.*` element-id vocabulary).
    fn introduction_gesture_points(gesture: &semio_framework::IntroductionGesture) -> Vec<&semio_framework::IntroductionPoint> {
        use semio_framework::IntroductionGesture;
        match gesture {
            IntroductionGesture::LeftClick { at } | IntroductionGesture::RightClick { at } | IntroductionGesture::DoubleClick { at } | IntroductionGesture::Scroll { at, .. } => {
                vec![at]
            }
            IntroductionGesture::Drag { from, to, .. } | IntroductionGesture::Orbit { from, to, .. } => vec![from, to],
        }
    }

    pub struct KeybindingSpec {
        pub keys: String,
        pub controller_id: String,
        pub action: String,
    }

    //#region 🔖️ArtifactKind
    /// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
    /// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
    /// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
    /// `semio_framework_os`'s artifact catalog registry share one definition.
    pub use semio_framework::{ArtifactKindSpec, OsMediaCapability};
    //#endregion 🔖️ArtifactKind

    //#region 🔖️MediaPort
    /// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
    /// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
    /// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
    pub use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};
    /// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
    /// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
    /// without a direct `semio-framework-core` dependency.
    pub use semio_framework::{MediaClass, MediaType};
    /// 🎞️ `MediaWireFormat` backs `MediaArtifactDescriptor::wire` (see `🔖️DocumentContract`
    /// below) — the plugin ABI's `consume-media`/`produce-media` payload framing, separate from
    /// `MediaPayload` above (which pairs with `Media`'s per-port `MediaType` projection).
    /// The legacy format enum was retired in ticket 26/08/11/
    /// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — `MediaWireFormat::
    /// Binary` now carries a plain format kind id string.
    pub use semio_framework::MediaWireFormat;
    //#endregion 🔖️MediaPort

    //#region 🔖️Dialect
    /// ⚠️ IO error alias from the framework media stack.
    pub use semio_framework::IoError;

    /// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
    /// 2). Defined in `semio_framework` so plugins and the OS product
    /// share one definition without an inverted dependency; re-exported here verbatim.
    pub use semio_framework::{
        StandardId, SubsetId, Dialect, ArtifactDialect,
        AnalyzeSource, IoConfidence, Analysis, ComposeSource, Composition, ComposeError,
        IoPayload, ErasedComposeSource, ComposedArtifact, ComposerEntry,
        IoDirection, IoKey, IoResolveError,
        register_composer_entries, io_resolve, io_dialects_for,
        io_keys_for, list_composer_entries, io_dispatch, set_io_fallback_dispatcher,
        WireComposeSource, WireComposedArtifact, wire_list_composer_entries, wire_artifact_compose, wire_decode_composed_artifact,
        SubsetValidator, SubsetValidatorEntry, subset_validator_entry_of, register_subset_validator,
    };

    /// 🧵️ Directed snapshot conversion out of this dialect into a foreign dialect. One unit
    /// struct per `🚪️io/📤️export/🧵️serializers/…` leaf.
    pub trait ArtifactSerializer {
        type From;
        type Into;
        const FROM: Dialect;
        const INTO: Dialect;
        fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError>;
    }

    /// 🧩️ Directed snapshot conversion from a foreign dialect into this dialect. One unit struct
    /// per `🚪️io/📥️import/🧩️deserializers/…` leaf.
    pub trait ArtifactDeserializer {
        type From;
        type Into;
        const FROM: Dialect;
        const INTO: Dialect;
        fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError>;
    }

    /// 🎹️ Subset-level composer: analyze foreign/native sources, build one snapshot in `WRITES`.
    /// Standard- and artifact-level (final) composers aggregate `ComposerEntry` rows value-level
    /// (see `ComposerEntry::of` convention on the framework side) rather than via this trait.
    pub trait ArtifactComposer: Sized {
        type Snapshot;
        const WRITES: Dialect;
        fn reads() -> &'static [Dialect];
        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError>;
    }

    /// 🎹️ Erases a typed `ArtifactComposer` into a `ComposerEntry` row for `register_composer_entries`.
    /// Lives here (not in `semio_framework_io`) because it needs the `ArtifactComposer` trait, which
    /// the lower-layer io module can't see. Erasure round-trips the snapshot through the same
    /// `store::ArtifactPack` binary codec `ArtifactBuilder::from_binary` already uses.
    pub fn composer_entry_of<C: ArtifactComposer>() -> ComposerEntry
    where
        C::Snapshot: store::ArtifactPack,
    {
        fn erased_compose<C: ArtifactComposer>(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError>
        where
            C::Snapshot: store::ArtifactPack,
        {
            let typed_sources: Vec<ComposeSource> = sources
                .iter()
                .map(|s| ComposeSource {
                    dialect: s.dialect,
                    payload: match &s.payload {
                        IoPayload::Text(t) => AnalyzeSource::Text(t.as_str()),
                        IoPayload::Binary(b) => AnalyzeSource::Binary(b.as_slice()),
                    },
                })
                .collect();
            let composed = C::compose(&typed_sources)?;
            let bytes = store::ArtifactPack::encode_pack(&composed.snapshot);
            Ok(ComposedArtifact {
                dialect: C::WRITES,
                payload: IoPayload::Binary(bytes),
                diagnostics: composed.diagnostics,
                confidence: composed.confidence,
            })
        }
        ComposerEntry { writes: C::WRITES, reads: C::reads(), compose: erased_compose::<C> }
    }

    /// 🧩️ Erases a typed `ArtifactDeserializer` into a `ComposerEntry` row, same shape as
    /// `composer_entry_of` but single-read (exactly one source — unlike a composer's multi-source
    /// union, a directed conversion has exactly one origin dialect) and via `Dialect`-typed
    /// snapshots rather than raw text/binary: the source payload is decoded as `D::From` (its own
    /// `store::ArtifactPack` codec), `D::deserialize` runs the typed conversion, and the result is
    /// re-packed as `D::Into` the same way `composer_entry_of` re-packs its `Composition::snapshot`.
    /// Registered through a subset composer's `register()` into the same `IoKey → ComposerEntry`
    /// registry composer entries already use.
    pub fn deserializer_entry_of<D: ArtifactDeserializer>() -> ComposerEntry
    where
        D::From: store::ArtifactPack,
        D::Into: store::ArtifactPack,
    {
        fn erased_compose<D: ArtifactDeserializer>(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError>
        where
            D::From: store::ArtifactPack,
            D::Into: store::ArtifactPack,
        {
            let source = match sources {
                [one] => one,
                other => {
                    return Err(ComposeError {
                        message: format!("deserializer {}->{} needs exactly 1 source, got {}", D::FROM.artifact_kind, D::INTO.artifact_kind, other.len()),
                        diagnostics: Vec::new(),
                    });
                }
            };
            let bytes = match &source.payload {
                IoPayload::Binary(b) => b.as_slice(),
                IoPayload::Text(_) => {
                    return Err(ComposeError {
                        message: format!("deserializer {}->{} source must be Binary (ArtifactPack-encoded)", D::FROM.artifact_kind, D::INTO.artifact_kind),
                        diagnostics: Vec::new(),
                    });
                }
            };
            let from = <D::From as store::ArtifactPack>::decode_pack(bytes).map_err(|e| ComposeError {
                message: format!("deserializer {}->{} failed to decode source: {e:?}", D::FROM.artifact_kind, D::INTO.artifact_kind),
                diagnostics: Vec::new(),
            })?;
            let into = D::deserialize(&from).map_err(|e| ComposeError {
                message: format!("deserializer {}->{} failed: {e:?}", D::FROM.artifact_kind, D::INTO.artifact_kind),
                diagnostics: Vec::new(),
            })?;
            let bytes = <D::Into as store::ArtifactPack>::encode_pack(&into);
            Ok(ComposedArtifact { dialect: D::INTO, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::High })
        }
        ComposerEntry { writes: D::INTO, reads: &[D::FROM], compose: erased_compose::<D> }
    }

    /// 🧵️ Erases a typed `ArtifactSerializer` into a `ComposerEntry` row — mirror image of
    /// `deserializer_entry_of`: writes `S::INTO`, reads exactly `[S::FROM]`, decodes the single
    /// source as `S::From`, runs `S::serialize`, re-packs the result as `S::Into`.
    pub fn serializer_entry_of<S: ArtifactSerializer>() -> ComposerEntry
    where
        S::From: store::ArtifactPack,
        S::Into: store::ArtifactPack,
    {
        fn erased_compose<S: ArtifactSerializer>(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError>
        where
            S::From: store::ArtifactPack,
            S::Into: store::ArtifactPack,
        {
            let source = match sources {
                [one] => one,
                other => {
                    return Err(ComposeError {
                        message: format!("serializer {}->{} needs exactly 1 source, got {}", S::FROM.artifact_kind, S::INTO.artifact_kind, other.len()),
                        diagnostics: Vec::new(),
                    });
                }
            };
            let bytes = match &source.payload {
                IoPayload::Binary(b) => b.as_slice(),
                IoPayload::Text(_) => {
                    return Err(ComposeError {
                        message: format!("serializer {}->{} source must be Binary (ArtifactPack-encoded)", S::FROM.artifact_kind, S::INTO.artifact_kind),
                        diagnostics: Vec::new(),
                    });
                }
            };
            let from = <S::From as store::ArtifactPack>::decode_pack(bytes).map_err(|e| ComposeError {
                message: format!("serializer {}->{} failed to decode source: {e:?}", S::FROM.artifact_kind, S::INTO.artifact_kind),
                diagnostics: Vec::new(),
            })?;
            let into = S::serialize(&from).map_err(|e| ComposeError {
                message: format!("serializer {}->{} failed: {e:?}", S::FROM.artifact_kind, S::INTO.artifact_kind),
                diagnostics: Vec::new(),
            })?;
            let bytes = <S::Into as store::ArtifactPack>::encode_pack(&into);
            Ok(ComposedArtifact { dialect: S::INTO, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::High })
        }
        ComposerEntry { writes: S::INTO, reads: &[S::FROM], compose: erased_compose::<S> }
    }
    //#endregion 🔖️Dialect

    //#region 🔖️ArtifactBuilder
    /// 🏗️ Incremental artifact materializer — snapshot/text/binary in, soft `Diagnostic`s out.
    /// `mutate` returns the handcrafted diff alongside the mutated builder (spine change S-1,
    /// `.claude/plans/the-current-schemas-are-scalable-journal.md`) — the diff is the single
    /// semantics source (`let d = mutation.diff(&snapshot); *snapshot = d.apply(snapshot); d`),
    /// not a separate recomputation.
    pub trait ArtifactBuilder: Sized {
        type Snapshot;
        type Mutation: protocol::Mutation<Self::Snapshot, Diff = Self::Diff>;
        type Diff;
        fn empty() -> Self;
        fn from_snapshot(snapshot: Self::Snapshot) -> Self;
        fn from_text(text: &str) -> Result<Self, store::TextError>;
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError>;
        fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff);
        fn absorb(self, diff: Self::Diff) -> Self;
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>>;
    }

    /// 🎚 Soft confidence for partial decomposition success.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Confidence {
        High,
        Medium,
        Low,
    }

    /// 📥 One decomposition source blob.
    #[derive(Clone, Debug)]
    pub enum DecomposeSource<'a> {
        Text(&'a str),
        Binary(&'a [u8]),
    }

    /// 📦 Decomposition result carrying soft diagnostics (never `Fault` for partial success).
    #[derive(Clone, Debug)]
    pub struct Decomposition<T> {
        pub parts: T,
        pub confidence: Confidence,
        pub diagnostics: Vec<dsl::Diagnostic>,
    }

    /// 📑️ Splits heterogeneous sources into typed parts with soft diagnostics.
    pub trait ArtifactDecomposer: Sized {
        type Snapshot;
        type Parts;
        fn decompose(sources: &[DecomposeSource]) -> Decomposition<Self::Parts>;
    }

    /// 🧐️ Standards/subsets successor to `ArtifactDecomposer` (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO
    /// phase 2): read-only analysis that also reports which `Dialect` it recognized, so aggregate
    /// composers can route a payload to the right standard (e.g. GIF87a vs GIF89a, `%PDF-1.x`).
    /// New artifacts implement this directly; migrated artifacts keep `ArtifactDecomposer` until
    /// their own migration wave swaps it -- both traits coexist until the global W16 strict flip.
    pub trait ArtifactAnalyzer: Sized {
        type Parts;
        const DIALECT: Dialect;
        /// 👃️ Cheap recognizability probe -- no allocation, no full parse.
        fn sniff(source: &AnalyzeSource) -> IoConfidence;
        fn analyze(sources: &[AnalyzeSource]) -> Analysis<Self::Parts>;
    }

    //#region 🧬️DerivedArtifactFacets
    /// 🧐️ Schema-owned analysis hook used by the derived analyzer.
    pub trait ArtifactAnalysis: 'static {
        type Parts;

        const DIALECT: Dialect;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence;

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts>;
    }

    /// 🎹️ IO-owned composition hook used by the derived composer.
    pub trait ArtifactComposition: 'static {
        type Snapshot;

        const WRITES: Dialect;

        fn reads() -> &'static [Dialect];

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError>;
    }

    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5): schema-owned child-composition hook —
    /// the derived analogue of `ArtifactComposition` for an artifact whose snapshot is assembled
    /// from OTHER artifacts' packs via declared `#[child(kind = "…")]` slots
    /// (`semio_framework_schema::ArtifactCompositionFields`), rather than only foreign-dialect IO
    /// sources. `Snapshot` deliberately repeats `DerivedArtifactSpec::Snapshot` instead of being
    /// read off it — a bare `type Children: ArtifactChildren` on `DerivedArtifactSpec` cannot itself
    /// constrain `Children::Snapshot == Self::Snapshot` without a where-clause at every use site, so
    /// `DerivedArtifactSpec::Children`'s own bound (below) restates the equality once, centrally.
    pub trait ArtifactChildren {
        type Snapshot;
        /// 🪆️ This artifact's declared child slots — `&[]` for a leaf with no `#[child(...)]`
        /// fields (see `NoChildren`).
        fn slots() -> &'static [::semio_framework_schema::ChildSlotSpec];
        /// 🏗️ Assembles `Self::Snapshot` from resolver-supplied child packs, one `(dialect, pack
        /// bytes)` pair per slot instance (`many` slots contribute more than one pair for the SAME
        /// `ChildSlotSpec.kind`) — the composition-side counterpart to `decompose_to_children`.
        fn compose_from_children(parts: &[(ArtifactDialect, Vec<u8>)]) -> Result<Self::Snapshot, ComposeError>;
        /// 📤️ Inverse of `compose_from_children`: the child packs `Self::Snapshot` currently owns,
        /// keyed by their own dialect — the seam `ArtifactChildren`'s own decompose side of the
        /// contract needs so a child's pack can be re-derived/re-exported without re-deriving the
        /// whole parent.
        fn decompose_to_children(snapshot: &Self::Snapshot) -> Vec<(ArtifactDialect, Vec<u8>)>;
    }

    /// 🍃️ `DerivedArtifactSpec::Children` default for a leaf artifact with zero composition slots —
    /// `slots()` is empty, `decompose_to_children` is always empty, and `compose_from_children`
    /// unconditionally errors (a leaf has nothing to compose FROM children; the derived composer
    /// below never even calls it when `slots()` is empty — see `DerivedArtifactComposer::compose`).
    pub struct NoChildren<S>(std::marker::PhantomData<S>);

    impl<S> ArtifactChildren for NoChildren<S> {
        type Snapshot = S;

        fn slots() -> &'static [::semio_framework_schema::ChildSlotSpec] {
            &[]
        }

        fn compose_from_children(_parts: &[(ArtifactDialect, Vec<u8>)]) -> Result<Self::Snapshot, ComposeError> {
            Err(ComposeError { message: "this artifact declares no #[child(...)] slots — NoChildren::compose_from_children is unreachable via the derived composer".into(), diagnostics: Vec::new() })
        }

        fn decompose_to_children(_snapshot: &Self::Snapshot) -> Vec<(ArtifactDialect, Vec<u8>)> {
            Vec::new()
        }
    }

    /// 🧬️ Hook bundle from which all public artifact lifecycle types are derived.
    pub trait DerivedArtifactSpec: Sized + 'static {
        type Snapshot;
        type Mutation: protocol::Mutation<Self::Snapshot, Diff = Self::Diff>;
        type Diff: protocol::MutationDiff<Self::Snapshot>;
        type Construction: ArtifactBuilder<Snapshot = Self::Snapshot, Mutation = Self::Mutation, Diff = Self::Diff>;
        type Analysis: ArtifactAnalysis;
        type Composition: ArtifactComposition<Snapshot = Self::Snapshot>;
        /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5): defaults to `NoChildren<Self::Snapshot>`
        /// for every spec built through `derive_artifact_facets!` without a `children: $ty` arm — see
        /// that macro's own doc comment. Stable Rust has no language-level default associated type, so
        /// the "default" lives at the MACRO level (it always emits a concrete `type Children = …;`),
        /// not here; a hand-written `impl DerivedArtifactSpec` must supply one explicitly.
        type Children: ArtifactChildren<Snapshot = Self::Snapshot>;
    }

    /// 📦️ Analyzer output derived from the artifact snapshot codec.
    #[derive(Clone, Debug, Default)]
    pub struct DerivedArtifactParts<Snapshot> {
        pub snapshot: Option<Snapshot>,
    }

    /// 🏗️ Generic materializer derived from a snapshot, semantic mutation, diff, and optional
    /// subset validation hook.
    pub struct DerivedArtifactBuilder<Spec: DerivedArtifactSpec> {
        construction: Spec::Construction,
    }

    impl<Spec: DerivedArtifactSpec> Clone for DerivedArtifactBuilder<Spec>
    where
        Spec::Construction: Clone,
    {
        fn clone(&self) -> Self {
            Self { construction: self.construction.clone() }
        }
    }

    impl<Spec: DerivedArtifactSpec> std::fmt::Debug for DerivedArtifactBuilder<Spec>
    where
        Spec::Construction: std::fmt::Debug,
    {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.debug_tuple("DerivedArtifactBuilder").field(&self.construction).finish()
        }
    }

    impl<Spec: DerivedArtifactSpec> Default for DerivedArtifactBuilder<Spec> {
        fn default() -> Self {
            Self::empty()
        }
    }

    impl<Spec: DerivedArtifactSpec> ArtifactBuilder for DerivedArtifactBuilder<Spec> {
        type Snapshot = Spec::Snapshot;
        type Mutation = Spec::Mutation;
        type Diff = Spec::Diff;

        fn empty() -> Self {
            Self { construction: Spec::Construction::empty() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { construction: Spec::Construction::from_snapshot(snapshot) }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self { construction: Spec::Construction::from_text(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { construction: Spec::Construction::from_binary(bytes)? })
        }

        fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let (construction, diff) = self.construction.mutate(mutation);
            (Self { construction }, diff)
        }

        fn absorb(self, diff: Self::Diff) -> Self {
            Self { construction: self.construction.absorb(diff) }
        }

        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            self.construction.build()
        }
    }

    impl<Spec: DerivedArtifactSpec> std::ops::Deref for DerivedArtifactBuilder<Spec> {
        type Target = Spec::Construction;

        fn deref(&self) -> &Self::Target {
            &self.construction
        }
    }

    impl<Spec: DerivedArtifactSpec> std::ops::DerefMut for DerivedArtifactBuilder<Spec> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.construction
        }
    }

    /// 🧐️ Codec-backed analyzer derived from the artifact snapshot and dialect coordinate.
    pub struct DerivedArtifactAnalyzer<Spec: DerivedArtifactSpec>(std::marker::PhantomData<Spec>);

    impl<Spec: DerivedArtifactSpec> ArtifactAnalyzer for DerivedArtifactAnalyzer<Spec> {
        type Parts = <Spec::Analysis as ArtifactAnalysis>::Parts;
        const DIALECT: Dialect = <Spec::Analysis as ArtifactAnalysis>::DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            <Spec::Analysis as ArtifactAnalysis>::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            <Spec::Analysis as ArtifactAnalysis>::analyze(sources)
        }
    }

    /// 🎹️ Composer derived from native snapshot codecs plus directed foreign IO hooks — and, when
    /// `Spec::Children` declares real slots (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM C1, Task 5), from
    /// this artifact's own composed children too. Living HERE (rather than duplicated per macro arm
    /// in `derive_artifact_facets!`) means every `DerivedArtifactSpec` — hand-written or
    /// macro-generated — gets child-slot-aware `reads()`/`compose()` automatically, for free, the
    /// moment its `Children` associated type names real slots.
    pub struct DerivedArtifactComposer<Spec: DerivedArtifactSpec>(std::marker::PhantomData<Spec>);

    impl<Spec: DerivedArtifactSpec> ArtifactComposer for DerivedArtifactComposer<Spec> {
        type Snapshot = Spec::Snapshot;
        const WRITES: Dialect = <Spec::Composition as ArtifactComposition>::WRITES;

        /// 🪆️ `Spec::Composition`'s own native/foreign reads, UNION each child slot's kind as an
        /// unconstrained-standard/unconstrained-subset `Dialect` (`StandardId("*")`/`SubsetId::ANY`
        /// — a child slot names only a `kind`, per `ChildSlotSpec.kind: &'static str`, never a
        /// specific standard/subset, so "any standard, any subset of this kind" is the literal
        /// reading of "this slot accepts the kind's dialect family").
        ///
        /// `ArtifactComposer::reads()`'s signature is fixed to `&'static [Dialect]` by the
        /// pre-existing (non-composition) trait, while `Spec::Children::slots()` is only knowable at
        /// runtime (a trait associated fn call, not a `const`) — so the union must be computed once
        /// and leaked into a `'static` lifetime rather than built as a `const`.
        ///
        /// ⚠️ Memoized in a `TypeId`-keyed table, NOT in a function-local `static`: a `static`
        /// declared inside a generic function is NOT monomorphized per type parameter — Rust gives
        /// every instantiation of `reads()` the SAME storage. With a plain `OnceLock` there, the
        /// first artifact kind to call `reads()` anywhere in the process would win and hand its
        /// answer to every other artifact kind forever (a composing artifact silently reporting a
        /// leaf's empty reads, or vice versa, depending purely on call order).
        ///
        /// A leaf `Spec::Children = NoChildren<_>` (`slots()` = `&[]`) still degrades to exactly
        /// `Spec::Composition::reads()`.
        fn reads() -> &'static [Dialect] {
            static UNIONS: std::sync::OnceLock<std::sync::Mutex<HashMap<std::any::TypeId, &'static [Dialect]>>> = std::sync::OnceLock::new();
            let unions = UNIONS.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
            let key = std::any::TypeId::of::<Spec>();
            let mut unions = unions.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            unions.entry(key).or_insert_with(|| {
                let mut reads: Vec<Dialect> = <Spec::Composition as ArtifactComposition>::reads().to_vec();
                for slot in <Spec::Children as ArtifactChildren>::slots() {
                    reads.push(Dialect { artifact_kind: slot.kind, standard: StandardId("*"), subset: SubsetId::ANY });
                }
                // 🎯️ Leaked deliberately: one small `Vec` per artifact-kind monomorphization, minted
                // once for the process lifetime — the `&'static [Dialect]` return type admits no
                // other option, and the count is bounded by the number of artifact kinds.
                Box::leak(reads.into_boxed_slice()) as &'static [Dialect]
            })
        }

        /// 🪆️ Routes through `Spec::Children::compose_from_children` when `Spec::Children` declares
        /// real slots AND every source's dialect kind matches one of them (never for a leaf artifact
        /// — `slots()` empty short-circuits straight to the unchanged `Spec::Composition::compose`
        /// path, so nothing about a non-composing artifact's behavior changes); falls through to
        /// `Spec::Composition::compose` otherwise (a mixed/foreign-dialect source set, or no
        /// declared slots at all).
        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let slots = <Spec::Children as ArtifactChildren>::slots();
            let all_child_sources = !slots.is_empty() && !sources.is_empty() && sources.iter().all(|source| slots.iter().any(|slot| slot.kind == source.dialect.artifact_kind));
            if all_child_sources {
                let parts: Vec<(ArtifactDialect, Vec<u8>)> = sources
                    .iter()
                    .map(|source| {
                        let bytes = match &source.payload {
                            AnalyzeSource::Binary(bytes) => bytes.to_vec(),
                            AnalyzeSource::Text(text) => text.as_bytes().to_vec(),
                        };
                        (ArtifactDialect::from(source.dialect), bytes)
                    })
                    .collect();
                let snapshot = <Spec::Children as ArtifactChildren>::compose_from_children(&parts)?;
                return Ok(Composition { snapshot, confidence: IoConfidence::High, diagnostics: Vec::new() });
            }
            <Spec::Composition as ArtifactComposition>::compose(sources)
        }
    }
    //#endregion 🧬️DerivedArtifactFacets

    /// 💡️ Read-side inference surface — one per artifact standard, sibling of `ArtifactAnalyzer`
    /// (not a widening of `ArtifactBuilder`: inference is read-only and cache-aware, unlike the
    /// authoring lifecycle `ArtifactBuilder` models). `infer`/`infer_cached` must stay
    /// observationally equal — `infer_cached`'s default is an honest passthrough for artifacts
    /// with no `InferredField`s yet (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    pub trait ArtifactInferrer: Sized {
        type Snapshot;
        type Inference: protocol::Inference<Self::Snapshot>;

        fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
            protocol::Inference::infer(snapshot)
        }

        /// 🧠️ Cache-aware variant; the default passthrough ignores `cache`/`session` and just calls
        /// `infer` — override once the standard registers real `InferredField`s.
        fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
            let _ = (cache, session);
            Self::infer(snapshot)
        }
    }
    //#endregion 🔖️ArtifactBuilder

    //#region 🔖️ArtifactDeclaration
    /// 🔖️ Everything an artifact currently registers by CALLING free functions, expressed instead as
    /// DATA the framework walks in `PluginBuilder::build()` — see
    /// `.🦑️repo/🎫️tickets/26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w1-mechanism-design.md`. Every
    /// field mirrors exactly one global registration function reachable from plugin code (census:
    /// `📓️w0-d-sdk-surface.md` §6); the two §6 functions that are NOT artifact-scoped
    /// (`register_app_schema_descriptor` — app config/presence schema, not an artifact concern;
    /// `register_linked_flow_extension_installer` — flow's own extension registry) have no field
    /// here on purpose, called out loudly rather than silently dropped — see the W1 report. Built
    /// only through `ArtifactDeclaration::builder(kind)`, a consuming typestate builder mirroring
    /// `PluginBuilder`'s own shape, so a declaration missing its mandatory `schema` is a compile
    /// error rather than a runtime panic. Every field is module-private — a plugin crate can
    /// describe a declaration through the builder's methods but never read or hand-assemble one;
    /// only `register_all` (called exactly once, from `PluginBuilder::build()`) ever walks it.
    pub struct ArtifactDeclaration {
        // 🏷️ Deliberately a raw `String`, not `ArtifactKindId`: `ArtifactKindId::parse` enforces the
        // canonical `s.<plugin>.<artifact>` grammar, and renaming EXISTING artifact kind strings
        // (today's note is `"s.note"`, raster's own `ArtifactKindSpec.id` is `"2d.raster"` — neither
        // canonical) to that grammar is its own later wave, not this one's (see `ArtifactKindId`'s
        // own doc at `🚪️io/🦀️component.rs:91-93`). `register_all` upgrades to the strict
        // plugin-segment check automatically the moment a given `kind` DOES parse as canonical, so
        // this tightens itself as that migration lands — no second pass needed here.
        kind: String,
        schema: Option<::semio_framework_schema::ArtifactSchemaDescriptor>,
        inferences: Vec<::semio_framework_schema::ArtifactInferenceDescriptor>,
        composers: &'static [ComposerEntry],
        formats: Vec<semio_framework::FormatDescriptor>,
        subset_validators: &'static [SubsetValidatorEntry],
        languages: &'static [dsl::LanguageSpec],
        document_codec: Option<DocumentCodecSpec>,
        migrations: Vec<store::DialectMigration>,
        // 🧒️🔗️ Pulled from `<Snapshot as ArtifactCompositionFields>::{child_slots,link_slots}` via
        // `.composition::<Snapshot>()` — never settable directly (UCAS review, 2026-08-12): the
        // derive-emitted trait impl IS the truth, so a hand-written list would be unwritable rather
        // than merely discouraged. No registration function consumes these yet (UCAS's composition
        // runtime reads `ArtifactCompositionFields` straight off the snapshot type on demand); they
        // are captured here so the declaration is a complete, single-source manifest of the
        // artifact's shape, not because `register_all` calls anything with them today.
        child_slots: &'static [::semio_framework_schema::ChildSlotSpec],
        link_slots: &'static [::semio_framework_schema::LinkSlotSpec],
        capabilities: Vec<CapabilityRequirement>,
    }

    /// 🏷️ Declaration builder has a `kind` only — next call must be `.schema(...)`.
    pub struct NeedsSchema;
    /// ✅️ Declaration builder has `kind` + `schema` — ready for every other facet plus `.build()`.
    pub struct DeclarationReady;

    /// 🏗️ Consuming typestate builder for [`ArtifactDeclaration`] — mirrors [`PluginBuilder`]'s own
    /// missing-mandatory-field-is-a-compile-error shape.
    pub struct ArtifactDeclarationBuilder<State> {
        kind: String,
        schema: Option<::semio_framework_schema::ArtifactSchemaDescriptor>,
        inferences: Vec<::semio_framework_schema::ArtifactInferenceDescriptor>,
        composers: &'static [ComposerEntry],
        formats: Vec<semio_framework::FormatDescriptor>,
        subset_validators: &'static [SubsetValidatorEntry],
        languages: &'static [dsl::LanguageSpec],
        document_codec: Option<DocumentCodecSpec>,
        migrations: Vec<store::DialectMigration>,
        child_slots: &'static [::semio_framework_schema::ChildSlotSpec],
        link_slots: &'static [::semio_framework_schema::LinkSlotSpec],
        capabilities: Vec<CapabilityRequirement>,
        _state: std::marker::PhantomData<State>,
    }

    impl ArtifactDeclaration {
        /// 🪪️ Starts a declaration from this artifact's kind id — canonical `s.<plugin>.<artifact>`
        /// where that migration has already landed, today's pre-migration grammar otherwise (see the
        /// field doc on `ArtifactDeclaration::kind`).
        pub fn builder(kind: impl Into<String>) -> ArtifactDeclarationBuilder<NeedsSchema> {
            ArtifactDeclarationBuilder {
                kind: kind.into(),
                schema: None,
                inferences: Vec::new(),
                composers: &[],
                formats: Vec::new(),
                subset_validators: &[],
                languages: &[],
                document_codec: None,
                migrations: Vec::new(),
                child_slots: &[],
                link_slots: &[],
                capabilities: Vec::new(),
                _state: std::marker::PhantomData,
            }
        }
    }

    impl ArtifactDeclarationBuilder<NeedsSchema> {
        /// 🧬️ Sets the artifact's four-facet schema descriptor — mandatory, so this is the one call
        /// that unlocks every other declaration method.
        pub fn schema(self, descriptor: ::semio_framework_schema::ArtifactSchemaDescriptor) -> ArtifactDeclarationBuilder<DeclarationReady> {
            ArtifactDeclarationBuilder {
                kind: self.kind,
                schema: Some(descriptor),
                inferences: self.inferences,
                composers: self.composers,
                formats: self.formats,
                subset_validators: self.subset_validators,
                languages: self.languages,
                document_codec: self.document_codec,
                migrations: self.migrations,
                child_slots: self.child_slots,
                link_slots: self.link_slots,
                capabilities: self.capabilities,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl ArtifactDeclarationBuilder<DeclarationReady> {
        /// 💡️ Appends inference descriptors (`register_artifact_inference_descriptor`, one call per
        /// item at `build()` time). Repeatable.
        pub fn inferences(mut self, items: impl IntoIterator<Item = ::semio_framework_schema::ArtifactInferenceDescriptor>) -> Self {
            self.inferences.extend(items);
            self
        }

        /// 🎹️ Sets this artifact's composer table (`register_composer_entries`). Every entry's
        /// `writes.artifact_kind` must equal this declaration's `kind` — checked at `build()` time,
        /// not here, since only `PluginBuilder::build()` knows the plugin id to check `kind` itself against.
        pub fn composers(mut self, entries: &'static [ComposerEntry]) -> Self {
            self.composers = entries;
            self
        }

        /// 🗂️ Appends format rows (`register_format_descriptors`).
        pub fn formats(mut self, rows: impl IntoIterator<Item = semio_framework::FormatDescriptor>) -> Self {
            self.formats.extend(rows);
            self
        }

        /// 🧾️ Sets this artifact's subset-validator table (`register_subset_validator`, one call per entry).
        pub fn subset_validators(mut self, entries: &'static [SubsetValidatorEntry]) -> Self {
            self.subset_validators = entries;
            self
        }

        /// 📖️ Sets this artifact's grammar table (`register_language`, one call per entry).
        pub fn languages(mut self, specs: &'static [dsl::LanguageSpec]) -> Self {
            self.languages = specs;
            self
        }

        /// 🗂️ Declares the document codec for one document-owning `ArtifactApp`
        /// (`register_document_codec_for_app::<A>`, keyed by `A::DOCUMENT_SCHEMA`). At most one per
        /// artifact — a second call overwrites the first, matching every other declaration field's
        /// last-write-wins builder convention.
        pub fn document_codec<A: ArtifactApp>(mut self) -> Self {
            self.document_codec = Some(DocumentCodecSpec::of::<A>());
            self
        }

        /// 🗂️ Sibling of `.document_codec::<A>()` for a library artifact with ZERO `ArtifactApp`s to
        /// bind to (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d, gap A — see
        /// `📓️w1d-declaration-gaps-report.md`). `.document_codec::<A>()` is keyed off `A::DOCUMENT_SCHEMA`
        /// and calls `register_document_codec_for_app::<A>`, which requires a real `ArtifactApp`; a
        /// headless library plugin (energy's `EnergyModelSnapshot`/`EnergyModelMutation` is the
        /// motivating case) has no such type. Same bounds as `store::ArtifactCodec::of`, same
        /// last-write-wins convention as `document_codec` above — the two share one `Option` slot since
        /// an artifact has exactly one document codec either way it's expressed.
        pub fn document_codec_bare<Snapshot, Mutation>(mut self, schema: impl Into<String>) -> Self
        where
            Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack + 'static,
            Mutation: ::protocol::Mutation<Snapshot> + PartialEq + Serialize + DeserializeOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static,
        {
            self.document_codec = Some(DocumentCodecSpec::bare::<Snapshot, Mutation>(schema));
            self
        }

        /// 🧭️ Appends dialect migrations (`register_dialect_migration`). Both `from.artifact_kind`
        /// and `to.artifact_kind` must equal this declaration's `kind` — checked at `build()` time.
        pub fn migrations(mut self, items: impl IntoIterator<Item = store::DialectMigration>) -> Self {
            self.migrations.extend(items);
            self
        }

        /// 🧒️🔗️ Pulls `child_slots`/`link_slots` from `<Snapshot as ArtifactCompositionFields>` — the
        /// ONLY way to set them (UCAS review, 2026-08-12: a hand-written list could silently disagree
        /// with the `#[derive(ArtifactSchema)]`-emitted impl the composition runtime actually reads,
        /// so there is deliberately no other setter).
        pub fn composition<Snapshot: ::semio_framework_schema::ArtifactCompositionFields>(mut self) -> Self {
            self.child_slots = Snapshot::child_slots();
            self.link_slots = Snapshot::link_slots();
            self
        }

        /// 🔒️ Declares a capability requirement owned by this artifact (unioned into
        /// `PluginManifest.capabilities` at `build()` time) — IO is artifact-owned, so an artifact
        /// that reads assets declares this on itself, not on the plugin.
        pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
            if !self.capabilities.contains(&capability) {
                self.capabilities.push(capability);
            }
            self
        }

        /// ✅️ Finishes the declaration.
        pub fn build(self) -> ArtifactDeclaration {
            ArtifactDeclaration {
                kind: self.kind,
                schema: self.schema,
                inferences: self.inferences,
                composers: self.composers,
                formats: self.formats,
                subset_validators: self.subset_validators,
                languages: self.languages,
                document_codec: self.document_codec,
                migrations: self.migrations,
                child_slots: self.child_slots,
                link_slots: self.link_slots,
                capabilities: self.capabilities,
            }
        }
    }

    /// 🗂️ A monomorphized, non-capturing thunk pairing a `schema` string with the fn pointer that
    /// registers a document codec under it — lets `.document_codec::<A>()`/`.document_codec_bare()`
    /// store the registration as inert data instead of performing it immediately, matching every
    /// other declaration field's "described now, run once at `PluginBuilder::build()`" contract. The
    /// `schema` sits alongside the thunk (not baked into it) because `bare`'s schema is a runtime
    /// `impl Into<String>`, not a type-level const like `A::DOCUMENT_SCHEMA` — a `fn()` thunk cannot
    /// close over it without capturing, which would break the "plain fn pointer" contract this type
    /// exists to keep.
    pub struct DocumentCodecSpec {
        schema: String,
        register: fn(String),
    }

    impl DocumentCodecSpec {
        fn of<A: ArtifactApp>() -> Self {
            fn register_thunk<A: ArtifactApp>(schema: String) {
                super::plugin_runtime::register_document_codec_for_app::<A>(schema);
            }
            DocumentCodecSpec { schema: A::DOCUMENT_SCHEMA.to_string(), register: register_thunk::<A> }
        }

        /// 🗂️ `of::<A>()`'s app-less sibling — see `.document_codec_bare()`'s own doc for why this
        /// exists. Registers straight against `store::register_document_codec`, bypassing
        /// `register_document_codec_for_app`'s `A::` indirection since there is no `A` to name.
        fn bare<Snapshot, Mutation>(schema: impl Into<String>) -> Self
        where
            Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack + 'static,
            Mutation: ::protocol::Mutation<Snapshot> + PartialEq + Serialize + DeserializeOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static,
        {
            fn register_thunk<Snapshot, Mutation>(schema: String)
            where
                Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack + 'static,
                Mutation: ::protocol::Mutation<Snapshot> + PartialEq + Serialize + DeserializeOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static,
            {
                store::register_document_codec(store::ArtifactCodec::of::<Snapshot, Mutation>(schema));
            }
            DocumentCodecSpec { schema: schema.into(), register: register_thunk::<Snapshot, Mutation> }
        }
    }

    impl ArtifactDeclaration {
        /// 🏗️ Performs every registration this declaration carries, in the fixed deterministic order
        /// schema → inferences → formats → subset validators → composers → languages → document
        /// codec → migrations (ordering was implicit in call order inside 33 hand-written `setup`
        /// functions; this is where it becomes explicit and uniform). Called exactly once per
        /// declared artifact, from `PluginBuilder::build()` — never `pub`, so a plugin crate can
        /// describe a declaration but never trigger its own registration.
        ///
        /// **Ownership check** — the single most important line in this change, and the direct
        /// countermeasure to the named violation this ticket opened against (lowpoly's
        /// `register_mesh_exporter("3d.mesh", …)`: an IO registration naming a kind that call had no
        /// connection to at all). Two layers, since today's on-disk kind strings are pre-migration
        /// (see the `kind` field doc):
        ///   1. **Always enforced**: every composer must ACTUALLY BE ABOUT this declaration's `kind`
        ///      — either producing it (`writes.artifact_kind == kind`, the import direction) or
        ///      consuming it (`kind` appears in `reads`, the export direction — an artifact's own
        ///      composer legitimately writes a FOREIGN format when exporting, e.g. note→svg, so only
        ///      `writes`-must-equal would reject every real export entry). Every subset validator's
        ///      `dialect.artifact_kind` and every migration's `from`/`to` `.artifact_kind` must equal
        ///      `kind` exactly — those are always about this artifact's own dialect, never a foreign one.
        ///   2. **Enforced once `kind` is canonical**: if `kind` parses as `s.<plugin>.<artifact>`,
        ///      its plugin segment must equal the builder's `plugin_id` — the precise, structural
        ///      form of "a plugin may only declare artifacts it owns." This tightens itself
        ///      automatically as kind strings migrate to the canonical grammar; no second pass here.
        pub(crate) fn register_all(self, plugin_id: &str, plugin: Plugin) -> Plugin {
            if let Ok(canonical) = ArtifactKindId::parse(&self.kind) {
                assert!(
                    canonical.plugin() == plugin_id,
                    "plugin {plugin_id:?} declared artifact {:?} but its canonical kind names owning plugin {:?} — a plugin may only declare artifacts it owns",
                    self.kind,
                    canonical.plugin()
                );
            }
            for entry in self.composers {
                let writes_it = entry.writes.artifact_kind == self.kind;
                let reads_it = entry.reads.iter().any(|dialect| dialect.artifact_kind == self.kind);
                assert!(
                    writes_it || reads_it,
                    "plugin {plugin_id:?}'s composer for artifact {:?} writes {} reading {:?} — neither touches the kind this declaration owns; a declaration's composers must produce or consume the kind it declares",
                    self.kind,
                    entry.writes.artifact_kind,
                    entry.reads.iter().map(|dialect| dialect.artifact_kind).collect::<Vec<_>>()
                );
            }
            for entry in self.subset_validators {
                assert!(
                    entry.dialect.artifact_kind == self.kind,
                    "plugin {plugin_id:?}'s subset validator for artifact {:?} validates {} — ownership mismatch",
                    self.kind,
                    entry.dialect.artifact_kind
                );
            }
            for migration in &self.migrations {
                assert!(
                    migration.from.artifact_kind == self.kind && migration.to.artifact_kind == self.kind,
                    "plugin {plugin_id:?}'s dialect migration for artifact {:?} names {}→{} — ownership mismatch",
                    self.kind,
                    migration.from.artifact_kind,
                    migration.to.artifact_kind
                );
            }

            if let Some(schema) = self.schema {
                ::semio_framework_schema::register_artifact_schema_descriptor(schema);
            }
            for inference in self.inferences {
                ::semio_framework_schema::register_artifact_inference_descriptor(inference);
            }
            if !self.formats.is_empty() {
                semio_framework::register_format_descriptors(self.formats);
            }
            for entry in self.subset_validators {
                semio_framework::register_subset_validator(entry);
            }
            if !self.composers.is_empty() {
                semio_framework::register_composer_entries(self.composers);
            }
            for spec in self.languages {
                dsl::register_language(*spec);
            }
            if let Some(codec) = self.document_codec {
                (codec.register)(codec.schema);
            }
            for migration in self.migrations {
                store::register_dialect_migration(migration);
            }

            let mut plugin = plugin;
            for capability in self.capabilities {
                plugin = plugin.capability(capability);
            }
            plugin
        }
    }
    //#endregion 🔖️ArtifactDeclaration

    pub struct AppBuilder {
        id: String,
        label: LocalizedLabel,
        document: Vec<String>,
        icon_id: Option<IconName>,
        controller_id: String,
        modes: Vec<ModeSpec>,
        default_mode_id: Option<String>,
        window_kinds: Vec<WindowKindSpec>,
        panel_tabs: Vec<PanelTabSpec>,
        keybindings: Vec<KeybindingSpec>,
        actions: Vec<ActionDefinition>,
        utilities: Vec<UtilityDefinition>,
        tools: Vec<ToolDefinition>,
        commands: Vec<CommandDefinition>,
        named_layouts: Vec<NamedLayout>,
        default_layout: Option<WindowLayout>,
        terminologies: Vec<String>,
        terminology_breadcrumbs: HashMap<String, Vec<String>>,
        introduction: Option<IntroductionDefinition>,
        tutorials: Vec<TutorialDefinition>,
        dialogs: Vec<DialogDefinition>,
        artifact_kinds: Vec<ArtifactKindSpec>,
        media_inputs: Vec<MediaPortSpec>,
        media_outputs: Vec<MediaPortSpec>,
        config: ConfigSpec,
        command_grammar: CommandGrammar,
        io: AppIo,
    }

    impl AppBuilder {
        pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
            let id = id.into();
            Self {
                controller_id: id.clone(),
                id,
                label: label.into(),
                document: Vec::new(),
                icon_id: None,
                modes: Vec::new(),
                default_mode_id: None,
                window_kinds: Vec::new(),
                panel_tabs: Vec::new(),
                keybindings: Vec::new(),
                actions: Vec::new(),
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_breadcrumbs: HashMap::new(),
                introduction: None,
                tutorials: Vec::new(),
                dialogs: Vec::new(),
                artifact_kinds: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                config: ConfigSpec::empty(),
                command_grammar: CommandGrammar::empty(),
                io: AppIo::default(),
            }
        }

        /// 🗂️ Declares one resource kind this app produces/consumes (see `ArtifactKindSpec`). Repeatable.
        pub fn artifact_kind(mut self, spec: ArtifactKindSpec) -> Self {
            self.artifact_kinds.push(spec);
            self
        }

        /// 🧮️ Declares this app's typed configuration record (see `crate::ConfigSpec`).
        pub fn config(mut self, spec: ConfigSpec) -> Self {
            self.config = spec;
            self
        }

        /// 🔌️ Declares this app's typed media I/O surface (see `crate::AppIo`).
        pub fn io(mut self, io: AppIo) -> Self {
            self.io = io;
            self
        }

        /// 🎛️ Declares this app's typed binary command grammar (see `crate::CommandGrammar`).
        pub fn command_grammar(mut self, grammar: CommandGrammar) -> Self {
            self.command_grammar = grammar;
            self
        }

        /// 🔌️ Declares one workflow input port this app accepts (see `MediaPortSpec`). Repeatable;
        /// validated in `build_definition` (non-empty/unique id, `direction` must be `In`).
        pub fn media_input(mut self, spec: MediaPortSpec) -> Self {
            self.media_inputs.push(spec);
            self
        }

        /// 🔌️ Declares one workflow output port this app produces (see `MediaPortSpec`). Repeatable;
        /// validated in `build_definition` (non-empty/unique id, `direction` must be `Out`, `MediaForm::Any`
        /// is rejected — `Any` is only ever legal on the accepting/input side, see `media_types_compatible`).
        pub fn media_output(mut self, spec: MediaPortSpec) -> Self {
            self.media_outputs.push(spec);
            self
        }

        /// 🗣️ Declares an alternative terminology id this app supports beyond the implicit "native" default.
        pub fn terminology(mut self, id: impl Into<String>) -> Self {
            self.terminologies.push(id.into());
            self
        }

        /// 🗺️ Replaces the full document path (product + app segments) while terminology `id` is active;
        /// `id` must also be declared via `terminology` — validated in `build_definition`.
        pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
            self.terminology_breadcrumbs.insert(id.into(), document.into_iter().map(Into::into).collect());
            self
        }

        /// @emoji 🎓️ Declares this app's first-run introduction walkthrough. Step anchors/advance
        /// conditions are validated against declared window kinds/utilities/actions/panel tabs in
        /// `build_definition`; declaring one auto-injects the `startIntroduction` action.
        pub fn introduction(mut self, introduction: IntroductionDefinition) -> Self {
            self.introduction = Some(introduction);
            self
        }

        /// @emoji 🎬️ Declares one recorded, timed tutorial (repeatable — an app may offer several). Every
        /// track is validated in `build_definition` (`validate_tutorial` plus referenced action/command/
        /// utility/tool/element ids); declaring at least one auto-injects the `startTutorial` action. The
        /// `recordTutorial` action is injected unconditionally (see `record_tutorial_action_definition`).
        pub fn tutorial(mut self, tutorial: TutorialDefinition) -> Self {
            self.tutorials.push(tutorial);
            self
        }

        /// @emoji 🗨️ Declares a modal form dialog (repeatable). `submit_action`/`cancel_action` and its
        /// `args` are validated in `build_definition`; opened only via `HostEffect::OpenDialog`.
        pub fn dialog(mut self, dialog: DialogDefinition) -> Self {
            self.dialogs.push(dialog);
            self
        }

        pub fn icon_id(mut self, icon_id: impl Into<IconName>) -> Self {
            self.icon_id = Some(icon_id.into());
            self
        }

        pub fn document<I, S>(mut self, document: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: Into<String>,
        {
            self.document = document.into_iter().map(Into::into).collect();
            self
        }

        pub fn mode(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
            self.modes.push(ModeSpec { id: id.into(), label: label.into(), icon_id: icon_id.into(), tools: Vec::new(), layout_id: None, commands: Vec::new() });
            self
        }

        /// 🎛️ Scopes commands to a mode — references ids declared via `.mode_command()`/`.command()`
        /// (each of which must be `CommandScope::Mode`).
        pub fn mode_commands(mut self, mode_id: impl AsRef<str>, command_ids: Vec<CommandRef>) -> Self {
            let mode_id = mode_id.as_ref();
            if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
                mode.commands = command_ids;
            }
            self
        }

        pub fn mode_layout(mut self, mode_id: impl AsRef<str>, layout_id: impl Into<String>) -> Self {
            let mode_id = mode_id.as_ref();
            if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
                mode.layout_id = Some(layout_id.into());
            }
            self
        }

        /// 🛠️ Scopes tools to a mode — references ids declared via `.tool()`/`.tool_simple()`. A tool is a
        /// mode-level activatable capability (e.g. puzzle3d fill); distinct from a window-scoped utility.
        pub fn mode_tools(mut self, mode_id: impl AsRef<str>, tool_ids: Vec<ToolRef>) -> Self {
            let mode_id = mode_id.as_ref();
            if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
                mode.tools = tool_ids;
            }
            self
        }

        pub fn default_mode_id(mut self, id: impl Into<String>) -> Self {
            self.default_mode_id = Some(id.into());
            self
        }

        pub fn window_kind(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, icon_id: impl Into<IconName>) -> Self {
            self.window_kinds.push(WindowKindSpec {
                id: id.into(),
                label: label.into(),
                body_key: body_key.into(),
                surface_kind,
                icon_id: icon_id.into(),
                measures: Vec::new(),
                engagement: None,
                actions: Vec::new(),
                utilities: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            });
            self
        }

        pub fn window_kind_with_engagement(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, engagement: WindowEngagement, icon_id: impl Into<IconName>) -> Self {
            self.window_kinds.push(WindowKindSpec {
                id: id.into(),
                label: label.into(),
                body_key: body_key.into(),
                surface_kind,
                icon_id: icon_id.into(),
                measures: Vec::new(),
                engagement: Some(engagement),
                actions: Vec::new(),
                utilities: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            });
            self
        }

        /// @emoji 🧱️ Declares a mode from an already-built `ModeDefinition` — e.g. assembled by a taxonomy
        /// `🎭️modes/<mode>/🦀️component.rs` component file instead of the scalar `.mode(...)` args. Stored
        /// through the same `ModeSpec` pipeline as `.mode()`, so `.mode_commands()`/`.mode_tools()`/
        /// `.mode_layout()` still apply post-hoc and `build_definition`'s validation runs unchanged.
        pub fn mode_def(mut self, def: ModeDefinition) -> Self {
            self.modes.push(mode_definition_to_spec(def));
            self
        }

        /// @emoji 🧱️ Declares a window kind from an already-built `WindowKindDefinition` — mirrors
        /// `.mode_def()`. `.window_kind_measures()`/`.window_kind_actions()`/`.window_kind_utilities()`
        /// still apply post-hoc.
        pub fn window_kind_def(mut self, def: WindowKindDefinition) -> Self {
            self.window_kinds.push(window_kind_definition_to_spec(def));
            self
        }

        /// @emoji 🧱️ Declares a (possibly nested) panel tab tree from an already-built `PanelTabDefinition`
        /// — mirrors `.panel_tab_tree()`, converting recursively through the same `PanelTabSpec` pipeline.
        pub fn panel_tab_def(mut self, def: PanelTabDefinition) -> Self {
            self.panel_tabs.push(panel_tab_definition_to_spec(def));
            self
        }

        /// 🎛️ Attaches measure controls (sliders/selects/toggles/groups) to an already-declared window kind.
        pub fn window_kind_measures(mut self, window_kind_id: impl AsRef<str>, measures: Vec<WindowMeasure>) -> Self {
            let window_kind_id = window_kind_id.as_ref();
            if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
                window.measures = measures;
            }
            self
        }

        /// 📇️ Scopes actions to a window kind — references ids declared via `.mutation()/.view_action()/.shell_action()`.
        pub fn window_kind_actions(mut self, window_kind_id: impl AsRef<str>, action_ids: Vec<ActionRef>) -> Self {
            let window_kind_id = window_kind_id.as_ref();
            if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
                window.actions = action_ids;
            }
            self
        }

        /// 🧰️ Scopes utilities to a window kind — references ids declared via `.utility()`/`.utility_simple()`. Mirrors
        /// `window_kind_actions`: the referenced utility ids are validated to resolve in `build_definition`.
        pub fn window_kind_utilities(mut self, window_kind_id: impl AsRef<str>, utility_ids: Vec<UtilityRef>) -> Self {
            let window_kind_id = window_kind_id.as_ref();
            if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
                window.utilities = utility_ids;
            }
            self
        }

        pub fn named_layout(mut self, layout: NamedLayout) -> Self {
            self.named_layouts.push(layout);
            self
        }

        pub fn default_layout(mut self, layout: WindowLayout) -> Self {
            self.default_layout = Some(layout);
            self
        }

        pub fn panel_tab(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: impl Into<String>) -> Self {
            self.panel_tabs.push(PanelTabSpec::leaf(id, label, group, body_key));
            self
        }

        /// 🌳️ Declares a root panel tab that may itself be a nested tree — build `tab` via `PanelTabSpec::leaf`/`PanelTabSpec::group`.
        pub fn panel_tab_tree(mut self, tab: PanelTabSpec) -> Self {
            self.panel_tabs.push(tab);
            self
        }

        /// 🏛️ Declares a framework-predefined panel tab (workbench/display/details/settings category or
        /// leaf) — only the framework shell itself should call this; plugins must use `.panel_tab()`/`.panel_tab_tree()`.
        pub fn panel_tab_framework(mut self, tab: PanelTabSpec) -> Self {
            self.panel_tabs.push(tab);
            self
        }

        pub fn keybinding(mut self, keys: impl Into<String>, action: impl Into<String>) -> Self {
            self.keybindings.push(KeybindingSpec { keys: keys.into(), controller_id: self.controller_id.clone(), action: action.into() });
            self
        }

        /// @emoji ✏️ Declares a document-mutating action — dispatched as VCS operations with a true inverse.
        pub fn mutation(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
            self.action_with(ActionDefinition::new_catalog(id, label, ActionKind::Mutation))
        }

        /// @emoji 👁️ Declares an ephemeral view action (camera, selection, hover, active utility) — not recorded in history.
        pub fn view_action(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
            self.action_with(ActionDefinition::new_catalog(id, label, ActionKind::View))
        }

        /// @emoji 🐚️ Declares a shell-only effect action (navigate, export, spawn) — no document mutation.
        pub fn shell_action(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
            self.action_with(ActionDefinition::new_catalog(id, label, ActionKind::Shell))
        }

        /// @emoji 📇️ Declares a fully specified action (icon, args, keybinding, palette visibility, category).
        pub fn action_with(mut self, action: ActionDefinition) -> Self {
            self.actions.push(action);
            self
        }

        /// @emoji 📝️ Attaches typed argument declarations to an already-declared action (post-hoc, mirroring
        /// `window_kind_actions`). If the id isn't declared yet at call time the args are dropped; the
        /// mismatch surfaces in `build_definition`, which asserts every declared action's args are consistent.
        pub fn action_args(mut self, action_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
            let action_id = action_id.as_ref();
            if let Some(action) = self.actions.iter_mut().find(|entry| entry.id == action_id) {
                action.args = args;
            }
            self
        }

        /// @emoji 🎛️ Declares a fully specified command. There are no window-level commands — only
        /// `CommandScope::App`/`CommandScope::Mode` may be declared here (`Os`/`Plugin` are rejected in
        /// `build_definition`); `Mode`-scope commands must additionally be referenced via `.mode_commands()`.
        pub fn command(mut self, command: CommandDefinition) -> Self {
            self.commands.push(command);
            self
        }

        /// @emoji 🎛️ Declares an app-scope command (applies whenever this app is focused, in any mode).
        pub fn app_command(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>) -> Self {
            self.command(CommandDefinition::new_catalog(id, label, CommandScope::App, category))
        }

        /// @emoji 🎛️ Declares a mode-scope command definition — still requires `.mode_commands(mode_id, ..)`
        /// to actually scope it to the mode(s) it applies to.
        pub fn mode_command(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>) -> Self {
            self.command(CommandDefinition::new_catalog(id, label, CommandScope::Mode, category))
        }

        /// @emoji 📝️ Attaches typed argument declarations to an already-declared command (post-hoc,
        /// mirroring `action_args`).
        pub fn command_args(mut self, command_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
            let command_id = command_id.as_ref();
            if let Some(command) = self.commands.iter_mut().find(|entry| entry.id == command_id) {
                command.args = args;
            }
            self
        }

        /// @emoji 🧰️ Declares an interactive utility this app exposes (referenced by `window_kind_utilities`).
        pub fn utility(mut self, utility: UtilityDefinition) -> Self {
            self.utilities.push(utility);
            self
        }

        /// @emoji 🧰️ Declares a utility with default settings (no group/keys/cursor/category, gates actions while active).
        pub fn utility_simple(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
            self.utility(UtilityDefinition::new(id, label, icon_id))
        }

        /// @emoji 🛠️ Declares a mode-level tool this app exposes (referenced by `.mode_tools()`). Distinct
        /// from `.utility()`: a tool is scoped to a whole mode, not a window kind, and its live options are
        /// supplied dynamically via `ArtifactApp::tool_measures`/`PluginApp::tool_measures`.
        pub fn tool(mut self, tool: ToolDefinition) -> Self {
            self.tools.push(tool);
            self
        }

        /// @emoji 🛠️ Declares a tool with default settings (no keybinding).
        pub fn tool_simple(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
            self.tool(ToolDefinition::new(id, label, icon_id))
        }

        /// @emoji 🧷️ Keybinding-vs-action-registry consistency is only enforced for apps that declare
        /// actions via `.mutation()`/`.view_action()`/`.shell_action()` — apps with an empty action
        /// registry keybind directly against controller actions instead, so there is nothing to check.
        pub fn build_definition(mut self) -> AppDefinition {
            assert!(!self.document.is_empty() && self.document.iter().all(|segment| !segment.trim().is_empty()), "app {} document must contain non-empty segments", self.id);
            for (terminology_id, document) in &self.terminology_breadcrumbs {
                assert!(self.terminologies.iter().any(|id| id == terminology_id), "app {} declares terminology_document for undeclared terminology {}", self.id, terminology_id);
                assert!(!document.is_empty() && document.iter().all(|segment| !segment.trim().is_empty()), "app {} terminology_document for {} must contain non-empty segments", self.id, terminology_id);
            }
            assert!(!self.window_kinds.is_empty(), "app {} must declare at least one window kind", self.id);
            assert!(!self.modes.is_empty(), "app {} must declare at least one mode", self.id);
            let mut window_kind_ids = HashSet::new();
            for window in &self.window_kinds {
                assert!(!window.id.trim().is_empty(), "app {} window kind id must be non-empty", self.id);
                assert!(!window.body_key.trim().is_empty(), "app {} window kind {} body_key must be non-empty", self.id, window.id);
                assert!(window_kind_ids.insert(window.id.clone()), "app {} duplicate window kind id {}", self.id, window.id);
            }
            let mut panel_tab_ids = HashSet::new();
            for tab in &self.panel_tabs {
                validate_panel_tab_spec(&self.id, tab, &mut panel_tab_ids);
            }
            // 🕰️ Unlike Document/Catalogue/Inspection/Parameters (per-app content, opt-in via
            // `.panel_tab(...)`), the history panel's content is framework-generic (`HistoryView`), so
            // every app gets it unconditionally — unless it already declared the reserved id itself.
            if panel_tab_ids.insert(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()) {
                self.panel_tabs.push(PanelTabSpec::framework(
                    PanelTabKind::App(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()),
                    LocalizedLabel::native(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_LABEL, "Verlauf"),
                    PanelGroup::Settings,
                    Some(FRAMEWORK_HISTORY_BODY_KEY.to_string()),
                    Vec::new(),
                ));
            }
            let mut layout_window_ids = Vec::new();
            if let Some(layout) = &self.default_layout {
                layout_window_ids.extend(collect_window_kind_ids_from_layout(layout));
            }
            for named in &self.named_layouts {
                layout_window_ids.extend(collect_window_kind_ids_from_layout(&named.layout));
            }
            for window_kind_id in layout_window_ids {
                assert!(window_kind_ids.contains(&window_kind_id), "app {} layout references undeclared window kind {}", self.id, window_kind_id);
            }
            let default_mode_id = self.default_mode_id.clone().unwrap_or_else(|| self.modes[0].id.clone());
            assert!(self.modes.iter().any(|mode| mode.id == default_mode_id), "app {} default_mode_id {} does not reference a declared mode", self.id, default_mode_id);
            let mut declared_action_ids = HashSet::new();
            for action in &self.actions {
                assert!(declared_action_ids.insert(action.id.clone()), "app {} duplicate action id {}", self.id, action.id);
                validate_arg_defs(&self.id, &format!("action {}", action.id), &action.args);
            }
            let mut declared_utility_ids = HashSet::new();
            for utility in &self.utilities {
                assert!(!utility.id.trim().is_empty(), "app {} utility id must be non-empty", self.id);
                assert!(declared_utility_ids.insert(utility.id.clone()), "app {} duplicate utility id {}", self.id, utility.id);
            }
            let mut declared_tool_ids = HashSet::new();
            for tool in &self.tools {
                assert!(!tool.id.trim().is_empty(), "app {} tool id must be non-empty", self.id);
                assert!(declared_tool_ids.insert(tool.id.clone()), "app {} duplicate tool id {}", self.id, tool.id);
            }
            let mut declared_command_scopes: HashMap<String, CommandScope> = HashMap::new();
            for command in &self.commands {
                assert!(matches!(command.scope, CommandScope::App | CommandScope::Mode), "app {} command {} must be declared CommandScope::App or CommandScope::Mode (Os/Plugin commands are not declared via AppBuilder)", self.id, command.id);
                assert!(declared_command_scopes.insert(command.id.clone(), command.scope).is_none(), "app {} duplicate command id {}", self.id, command.id);
                validate_arg_defs(&self.id, &format!("command {}", command.id), &command.args);
            }
            let app_declared_actions = !self.actions.is_empty();
            let mut actions = self.actions;
            for history_action in history_action_definitions() {
                if declared_action_ids.insert(history_action.id.clone()) {
                    actions.push(history_action);
                }
            }
            for clipboard_action in clipboard_action_definitions() {
                if declared_action_ids.insert(clipboard_action.id.clone()) {
                    actions.push(clipboard_action);
                }
            }
            if !self.utilities.is_empty() && declared_action_ids.insert(SET_ACTIVE_UTILITY_ACTION_ID.to_string()) {
                actions.push(set_active_utility_action_definition());
            }
            if !self.tools.is_empty() && declared_action_ids.insert(SET_ACTIVE_TOOL_ACTION_ID.to_string()) {
                actions.push(set_active_tool_action_definition());
            }
            if self.introduction.is_some() && declared_action_ids.insert(START_INTRODUCTION_ACTION_ID.to_string()) {
                actions.push(start_introduction_action_definition());
            }
            if !self.tutorials.is_empty() && declared_action_ids.insert(START_TUTORIAL_ACTION_ID.to_string()) {
                actions.push(start_tutorial_action_definition(&self.tutorials));
            }
            if declared_action_ids.insert(RECORD_TUTORIAL_ACTION_ID.to_string()) {
                actions.push(record_tutorial_action_definition());
            }
            if declared_action_ids.insert(SET_HISTORY_COMMAND_FILTER_ACTION_ID.to_string()) {
                actions.push(set_history_command_filter_action_definition());
            }
            if declared_action_ids.insert(NOTE_SHELL_COMMAND_ACTION_ID.to_string()) {
                actions.push(note_shell_command_action_definition());
            }
            let mut bound_keys: HashSet<String> = self.keybindings.iter().map(|binding| binding.keys.clone()).collect();
            let mut keybindings: Vec<Keybinding> = self.keybindings.into_iter().map(|binding| Keybinding { keys: binding.keys, action: ActionDescriptor { controller_id: binding.controller_id, action: binding.action, args: None } }).collect();
            for history_action in actions.iter().filter(|action| matches!(action.kind, ActionKind::History | ActionKind::Clipboard)) {
                if let Some(keys) = &history_action.keys {
                    if bound_keys.insert(keys.clone()) {
                        keybindings.push(Keybinding { keys: keys.clone(), action: ActionDescriptor { controller_id: self.controller_id.clone(), action: history_action.id.clone(), args: None } });
                    }
                }
            }
            for utility in &self.utilities {
                if let Some(keys) = &utility.keys {
                    if bound_keys.insert(keys.clone()) {
                        keybindings.push(Keybinding {
                            keys: keys.clone(),
                            action: ActionDescriptor { controller_id: self.controller_id.clone(), action: SET_ACTIVE_UTILITY_ACTION_ID.to_string(), args: Some(DslValue::Object(vec![("utilityId".into(), DslValue::String(utility.id.clone()))])) },
                        });
                    }
                }
            }
            for tool in &self.tools {
                if let Some(keys) = &tool.keys {
                    if bound_keys.insert(keys.clone()) {
                        keybindings.push(Keybinding {
                            keys: keys.clone(),
                            action: ActionDescriptor { controller_id: self.controller_id.clone(), action: SET_ACTIVE_TOOL_ACTION_ID.to_string(), args: Some(DslValue::Object(vec![("toolId".into(), DslValue::String(tool.id.clone()))])) },
                        });
                    }
                }
            }
            if app_declared_actions {
                for binding in &keybindings {
                    assert!(declared_action_ids.contains(&binding.action.action), "app {} keybinding {} references undeclared action {}", self.id, binding.keys, binding.action.action);
                }
            }
            for window in &self.window_kinds {
                for action_ref in &window.actions {
                    assert!(declared_action_ids.contains(action_ref.as_str()), "app {} window kind {} references undeclared action {}", self.id, window.id, action_ref.as_str());
                }
                for utility_ref in &window.utilities {
                    assert!(declared_utility_ids.contains(utility_ref.as_str()), "app {} window kind {} references undeclared utility {}", self.id, window.id, utility_ref.as_str());
                }
            }
            for mode in &self.modes {
                for command_ref in &mode.commands {
                    assert!(declared_command_scopes.get(command_ref.as_str()).copied() == Some(CommandScope::Mode), "app {} mode {} references undeclared or non-Mode-scope command {}", self.id, mode.id, command_ref.as_str());
                }
                for tool_ref in &mode.tools {
                    assert!(declared_tool_ids.contains(tool_ref.as_str()), "app {} mode {} references undeclared tool {}", self.id, mode.id, tool_ref.as_str());
                }
            }
            let mode_referenced_commands: HashSet<&str> = self.modes.iter().flat_map(|mode| mode.commands.iter().map(|command_ref| command_ref.as_str())).collect();
            for (id, scope) in &declared_command_scopes {
                assert!(*scope != CommandScope::Mode || mode_referenced_commands.contains(id.as_str()), "app {} mode-scope command {} is not referenced by any mode", self.id, id);
            }
            let mode_referenced_tools: HashSet<&str> = self.modes.iter().flat_map(|mode| mode.tools.iter().map(|tool_ref| tool_ref.as_str())).collect();
            for tool in &self.tools {
                assert!(mode_referenced_tools.contains(tool.id.as_str()), "app {} tool {} is not referenced by any mode", self.id, tool.id);
            }
            if let Some(introduction) = &self.introduction {
                assert!(!introduction.steps.is_empty(), "app {} introduction must declare at least one step", self.id);
                let mut step_ids = HashSet::new();
                for step in &introduction.steps {
                    assert!(!step.id.trim().is_empty(), "app {} introduction step id must be non-empty", self.id);
                    assert!(step_ids.insert(step.id.clone()), "app {} duplicate introduction step id {}", self.id, step.id);
                    let validate_element_id = |id: &str, role: &str| validate_referenced_element_id(&self.id, &format!("introduction step {}", step.id), role, id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                    if let Some(id) = &step.introduce {
                        validate_element_id(id, "introduce");
                    }
                    for id in &step.show {
                        validate_element_id(id, "show");
                    }
                    for interaction in &step.interactions {
                        assert!(!interaction.label.trim().is_empty(), "app {} introduction step {} interaction has an empty label", self.id, step.id);
                        match &interaction.on {
                            IntroductionInteractionKind::Action(action_ref) => {
                                assert!(declared_action_ids.contains(action_ref.as_str()), "app {} introduction step {} interaction references undeclared action {}", self.id, step.id, action_ref.as_str())
                            }
                            IntroductionInteractionKind::Utility(utility_ref) => {
                                assert!(declared_utility_ids.contains(utility_ref.as_str()), "app {} introduction step {} interaction references undeclared utility {}", self.id, step.id, utility_ref.as_str())
                            }
                            IntroductionInteractionKind::Tool(tool_ref) => assert!(declared_tool_ids.contains(tool_ref.as_str()), "app {} introduction step {} interaction references undeclared tool {}", self.id, step.id, tool_ref.as_str()),
                            IntroductionInteractionKind::Panel(panel_tab_id) => validate_element_id(panel_tab_id, "interaction.panel"),
                            IntroductionInteractionKind::Expand(tree_id) => validate_element_id(tree_id, "interaction.expand"),
                            IntroductionInteractionKind::Pan(window_kind_id) | IntroductionInteractionKind::Zoom(window_kind_id) | IntroductionInteractionKind::Orbit(window_kind_id) => {
                                assert!(window_kind_ids.contains(window_kind_id), "app {} introduction step {} interaction references undeclared window kind {}", self.id, step.id, window_kind_id)
                            }
                        }
                    }
                }
            }
            let mut tutorial_ids = HashSet::new();
            for tutorial in &self.tutorials {
                assert!(!tutorial.id.trim().is_empty(), "app {} tutorial id must be non-empty", self.id);
                assert!(tutorial_ids.insert(tutorial.id.clone()), "app {} duplicate tutorial id {}", self.id, tutorial.id);
                if let Err(reason) = semio_framework::validate_tutorial(tutorial) {
                    panic!("app {} tutorial {} failed validation: {}", self.id, tutorial.id, reason);
                }
                let owner = format!("tutorial {}", tutorial.id);
                let mut ui_changes: Vec<&semio_framework::TutorialUiChange> = Vec::new();
                for keyframe in &tutorial.tracks.ui {
                    if let semio_framework::TutorialUiSample::Delta { changes } = &keyframe.sample {
                        ui_changes.extend(changes.iter());
                    }
                }
                for change in ui_changes {
                    match change {
                        semio_framework::TutorialUiChange::ActiveUtility { utility_id: Some(utility_id), .. } => assert!(declared_utility_ids.contains(utility_id), "app {} {} references undeclared utility {}", self.id, owner, utility_id),
                        semio_framework::TutorialUiChange::ActiveTool { id: Some(tool_id) } => assert!(declared_tool_ids.contains(tool_id), "app {} {} references undeclared tool {}", self.id, owner, tool_id),
                        _ => {}
                    }
                }
                for utility_id in tutorial.base.ui.active_utility_by_window_id.values() {
                    assert!(declared_utility_ids.contains(utility_id), "app {} {} base.ui references undeclared utility {}", self.id, owner, utility_id);
                }
                if let Some(tool_id) = &tutorial.base.ui.active_tool_id {
                    assert!(declared_tool_ids.contains(tool_id), "app {} {} base.ui references undeclared tool {}", self.id, owner, tool_id);
                }
                for event in &tutorial.tracks.events {
                    match &event.kind {
                        semio_framework::TutorialEventKind::Action { action, .. } => assert!(declared_action_ids.contains(action), "app {} {} event references undeclared action {}", self.id, owner, action),
                        semio_framework::TutorialEventKind::Command { command, .. } => assert!(declared_command_scopes.contains_key(command), "app {} {} event references undeclared command {}", self.id, owner, command),
                        semio_framework::TutorialEventKind::Key { .. } => {}
                    }
                }
                for gesture_cue in &tutorial.tracks.gestures {
                    for point in introduction_gesture_points(&gesture_cue.gesture) {
                        if let semio_framework::IntroductionPoint::Element { id, .. } = point {
                            validate_referenced_element_id(&self.id, &owner, "gesture", id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                        }
                    }
                }
            }
            let mut dialog_ids = HashSet::new();
            for dialog in &self.dialogs {
                assert!(!dialog.id.trim().is_empty(), "app {} dialog id must be non-empty", self.id);
                assert!(dialog_ids.insert(dialog.id.clone()), "app {} duplicate dialog id {}", self.id, dialog.id);
                assert!(declared_action_ids.contains(dialog.submit_action.as_str()), "app {} dialog {} submit_action references undeclared action {}", self.id, dialog.id, dialog.submit_action.as_str());
                if let Some(cancel_action) = &dialog.cancel_action {
                    assert!(declared_action_ids.contains(cancel_action.as_str()), "app {} dialog {} cancel_action references undeclared action {}", self.id, dialog.id, cancel_action.as_str());
                }
                validate_arg_defs(&self.id, &format!("dialog {}", dialog.id), &dialog.args);
            }
            let mut media_port_ids = HashSet::new();
            for port in self.media_inputs.iter().chain(self.media_outputs.iter()) {
                assert!(!port.id.trim().is_empty(), "app {} media port id must be non-empty", self.id);
                assert!(media_port_ids.insert(port.id.clone()), "app {} duplicate media port id {}", self.id, port.id);
            }
            for port in &self.media_inputs {
                assert!(port.direction == MediaPortDirection::In, "app {} media input {} must declare direction In", self.id, port.id);
            }
            for port in &self.media_outputs {
                assert!(port.direction == MediaPortDirection::Out, "app {} media output {} must declare direction Out", self.id, port.id);
                assert!(!matches!(port.media_type.form, MediaForm::Any), "app {} media output {} must not declare MediaForm::Any (Any is only legal on inputs, see media_types_compatible)", self.id, port.id);
            }
            AppDefinition {
                id: self.id,
                label: self.label,
                breadcrumb: self.document,
                icon_id: self.icon_id,
                controller_id: self.controller_id,
                modes: Modes::try_from(self.modes.into_iter().map(|mode| ModeDefinition { id: mode.id, label: mode.label, icon_id: mode.icon_id, tools: mode.tools, layout_id: mode.layout_id, commands: mode.commands }).collect::<Vec<_>>())
                    .expect("app must declare at least one mode (checked above)"),
                default_mode_id,
                window_kinds: WindowKinds::try_from(
                    self.window_kinds
                        .into_iter()
                        .map(|window| WindowKindDefinition {
                            id: window.id,
                            label: window.label,
                            body_key: window.body_key,
                            surface_kind: window.surface_kind,
                            icon_id: window.icon_id,
                            options: WindowOptions { measures: window.measures, engagement: window.engagement.map_or(WindowEngagementSlot::None, WindowEngagementSlot::Some) },
                            actions: window.actions,
                            utilities: window.utilities,
                            params_schema: window.params_schema,
                            artifact_snapshot_schema: window.artifact_snapshot_schema,
                            input_event_schema: window.input_event_schema,
                            output_schema: window.output_schema,
                            capabilities: window.capabilities,
                        })
                        .collect::<Vec<_>>(),
                )
                .expect("app must declare at least one window kind (checked above)"),
                panel_tabs: self.panel_tabs.into_iter().map(panel_tab_spec_to_definition).collect(),
                keybindings,
                actions,
                utilities: self.utilities,
                tools: self.tools,
                commands: self.commands,
                named_layouts: self.named_layouts,
                default_layout: self.default_layout,
                terminologies: self.terminologies,
                terminology_breadcrumbs: self.terminology_breadcrumbs,
                introduction: self.introduction,
                tutorials: self.tutorials,
                dialogs: self.dialogs,
                media_inputs: self.media_inputs,
                media_outputs: self.media_outputs,
                artifact_kinds: self.artifact_kinds,
                config: self.config,
                command_grammar: self.command_grammar,
                io: self.io,
            }
        }
    }

    //#region 🔖️PanelKit
    // 🌳️ Shared panel-tree builders — lifts the verbatim-duplicated `tree_item*`/`selection_ids` helpers
    // and the `build_document_tree`/`build_inspector_tree`/`build_catalogue_tree` skeleton found across
    // ~15 plugin crates (flow, procedural, layout, gis, puzzle, sequence, trinity, dag, …) into the SDK.

    /// 🌳️ A bare tree item — thin wrapper over `UiTreeItemNode::base`.
    pub fn tree_item(id: impl Into<String>, label: impl Into<Label>) -> UiTreeItemNode {
        UiTreeItemNode::base(id, label)
    }

    /// 🌳️ A tree item with a description line.
    pub fn tree_item_desc(id: impl Into<String>, label: impl Into<Label>, description: Option<String>) -> UiTreeItemNode {
        UiTreeItemNode { description, menu: None, ..UiTreeItemNode::base(id, label) }
    }

    /// 🌳️ A tree item that dispatches `action` on click.
    pub fn tree_item_with_action(id: impl Into<String>, label: impl Into<Label>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
        UiTreeItemNode { description, action: Some(action), menu: None, ..UiTreeItemNode::base(id, label) }
    }

    /// 🌳️ A draggable tree item: `drag_data` is a JSON object whose entries become the item's
    /// MIME-type -> payload drag-data map (string values are used verbatim; non-string values are
    /// serialized), e.g. `json!({ "application/x-my-widget": descriptor.to_string() })`. Generalizes the
    /// single-hardcoded-MIME-key pattern duplicated per app (each app previously baked its own MIME
    /// constant into this function) — the caller now supplies the key(s) explicitly.
    pub fn tree_item_with_action_draggable(id: impl Into<String>, label: impl Into<Label>, description: Option<String>, action: ActionDescriptor, drag_data: &Value) -> UiTreeItemNode {
        let mut item = tree_item_with_action(id, label, description, action);
        item.draggable = Some(true);
        item.drag_data = drag_data.as_object().map(|entries| entries.iter().map(|(key, value)| (key.clone(), value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string()))).collect());
        item
    }

    /// 🎯️ Parses a selection-action's `ids` array arg into a plain `Vec<String>` — the shape used by the
    /// majority of duplicate copies (`layout`, `gis`, `presentation`, …). A handful of apps additionally
    /// fall back to a singular `id`/`nodeId`/`nodeIds` key (`puzzle`, `sequence`, `trinity`, `procedural`,
    /// `mindmap`); those apps keep their own fallback wrapper around this shared core for now.
    pub fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
    }

    /// 🌳️ Fluent builder for the `build_document_tree`/`build_inspector_tree`/`build_catalogue_tree`
    /// skeleton duplicated across plugin crates: namespaced item ids, sections (optionally substituting a
    /// single "(none)" placeholder item for the empty state), a selected/highlighted id set, a
    /// selection-change action, and a drop action — ending in `.build()` -> a `UiNode::Tree`.
    pub struct PanelTreeBuilder {
        namespace: String,
        sections: Vec<UiTreeSectionNode>,
        selected_ids: Option<Vec<String>>,
        highlighted_ids: Option<Vec<String>>,
        selection_change: Option<ActionDescriptor>,
        drop_action: Option<ActionDescriptor>,
    }

    impl PanelTreeBuilder {
        /// 🌳️ `namespace` prefixes every id built via `.item_id()`, e.g. `"flow-play-document"`.
        pub fn new(namespace: impl Into<String>) -> Self {
            Self { namespace: namespace.into(), sections: Vec::new(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None }
        }

        /// 🌳️ Builds a namespaced item id: `"{namespace}.{kind}.{id}"`.
        pub fn item_id(&self, kind: &str, id: &str) -> String {
            format!("{}.{kind}.{id}", self.namespace)
        }

        /// 🌳️ Adds a section verbatim.
        pub fn section(mut self, id: impl Into<String>, label: Option<Label>, default_open: bool, items: Vec<UiTreeItemNode>) -> Self {
            self.sections.push(UiTreeSectionNode { id: id.into(), label, default_open: Some(default_open), presence: UiPresence::default(), items });
            self
        }

        /// 🌳️ Adds a section, substituting a single "(none)"-style placeholder item when `items` is empty —
        /// the empty-state pattern duplicated in `build_document_tree`/`build_catalogue_tree` across apps.
        pub fn section_or_placeholder(mut self, id: impl Into<String>, label: Option<Label>, default_open: bool, items: Vec<UiTreeItemNode>, placeholder_label: impl Into<Label>) -> Self {
            let id = id.into();
            let items = if items.is_empty() { vec![tree_item(format!("{id}.empty"), placeholder_label)] } else { items };
            self.sections.push(UiTreeSectionNode { id, label, default_open: Some(default_open), presence: UiPresence::default(), items });
            self
        }

        pub fn selected(mut self, ids: Vec<String>) -> Self {
            self.selected_ids = Some(ids);
            self
        }

        pub fn highlighted(mut self, ids: Vec<String>) -> Self {
            self.highlighted_ids = Some(ids);
            self
        }

        pub fn selection_change(mut self, action: ActionDescriptor) -> Self {
            self.selection_change = Some(action);
            self
        }

        pub fn drop_action(mut self, action: ActionDescriptor) -> Self {
            self.drop_action = Some(action);
            self
        }

        pub fn build(mut self) -> UiNode {
            let selected = self.selected_ids.iter().flatten().cloned().collect::<HashSet<_>>();
            let highlighted = self.highlighted_ids.iter().flatten().cloned().collect::<HashSet<_>>();
            ui_tree_stamp_presence(&mut self.sections, &selected, &highlighted);
            UiNode::Tree(UiTreeNode {
                sections: self.sections,
                presence: UiPresence::default(),
                selected_ids: self.selected_ids.clone(),
                highlighted_ids: self.highlighted_ids.clone(),
                selection_change: self.selection_change,
                drop_action: self.drop_action,
                menu: None,
            })
        }
    }

    #[cfg(test)]
    mod panel_kit_tests {
            use ui_wgpu::wgpu::Label;
        use super::*;

        #[test]
        fn tree_item_builds_a_bare_item() {
            let item = tree_item("ns.kind.a", Label::data("A"));
            assert_eq!(item.id, "ns.kind.a");
            assert_eq!(item.label.as_str(), "A");
            assert!(item.description.is_none());
            assert!(item.action.is_none());
        }

        #[test]
        fn tree_item_with_action_draggable_maps_json_object_to_string_drag_data() {
            let action = ActionDescriptor { controller_id: "app".into(), action: "addWidget".into(), args: None };
            let item = tree_item_with_action_draggable("ns.kind.a", Label::data("A"), None, action, &serde_json::json!({ "application/x-widget": "{\"kind\":\"a\"}" }));
            assert_eq!(item.draggable, Some(true));
            assert_eq!(item.drag_data.unwrap().get("application/x-widget").map(String::as_str), Some("{\"kind\":\"a\"}"));
        }

        #[test]
        fn selection_ids_reads_the_ids_array_arg() {
            let args = serde_json::json!({ "ids": ["a", "b"] });
            assert_eq!(selection_ids(Some(&args)), vec!["a".to_string(), "b".to_string()]);
            assert!(selection_ids(None).is_empty());
        }

        #[test]
        fn panel_tree_builder_produces_a_namespaced_tree_with_placeholder() {
            let builder = PanelTreeBuilder::new("ns-play-document");
            let item_id = builder.item_id("widget", "w1");
            assert_eq!(item_id, "ns-play-document.widget.w1");
            let node = builder
                .section("ns-play-document.widgets", Some(Label::data("Widgets")), true, vec![tree_item(item_id, Label::data("W1"))])
                .section_or_placeholder("ns-play-document.synapses", Some(Label::data("Synapses")), false, vec![], Label::data("(none)"))
                .selected(vec!["ns-play-document.widget.w1".into()])
                .build();
            let UiNode::Tree(tree) = node else { panic!("expected a Tree node") };
            assert_eq!(tree.sections.len(), 2);
            assert_eq!(tree.sections[0].items.len(), 1);
            assert_eq!(tree.sections[1].items[0].label.as_str(), "(none)");
            assert!(tree.sections[0].items[0].presence.selected, "the .selected(...) id must be stamped as selected presence on its matching item");
        }
    }
    //#endregion 🔖️PanelKit

    //#region 🔖️FormKit
    // 📋️ Shared form-panel builder — lifts the `Section > labeled Field rows > submit Button` skeleton
    // (and the sibling `entity_detail` read-only `KeyValue` summary block) duplicated across plugin crates
    // that render declarative forms/detail panels, mirroring `PanelTreeBuilder`'s namespaced builder-pattern
    // shape above (`namespace` prefixes every id, method chaining ends in `.build() -> UiNode`).

    /// 📋️ Fluent builder for a `Section` of labeled `Field` rows ending in an optional submit `Button` —
    /// same namespaced-id / method-chaining shape as `PanelTreeBuilder`.
    pub struct FormPanelBuilder {
        namespace: String,
        fields: Vec<UiNode>,
        submit: Option<UiButtonNode>,
    }

    impl FormPanelBuilder {
        /// 📋️ `namespace` prefixes every field id built via `.field_id()`/`.field()`/`.from_dictionary()`.
        pub fn new(namespace: impl Into<String>) -> Self {
            Self { namespace: namespace.into(), fields: Vec::new(), submit: None }
        }

        /// 📋️ Builds a namespaced field id: `"{namespace}.field.{id}"` — mirrors `PanelTreeBuilder::item_id`.
        pub fn field_id(&self, id: &str) -> String {
            format!("{}.field.{id}", self.namespace)
        }

        /// 📋️ Adds one labeled field row: `control` wraps into a `UiFieldNode` via `ui_control_to_node`.
        pub fn field(mut self, id: &str, label: impl Into<Label>, description: Option<String>, control: UiControlNode) -> Self {
            let field_id = self.field_id(id);
            self.fields.push(UiNode::Field(UiFieldNode { id: field_id, label: label.into(), description, required: None, error: None, child: Box::new(ui_control_to_node(control)), presence: UiPresence::default(), menu: None }));
            self
        }

        /// 📋️ Routes the OS `form.dictionary` resource shape (see the `ArtifactKindSpec { id:
        /// "form.dictionary", source_format: "forms.dictionary", .. }` registered by `forms/plugin`'s
        /// `create_forms_app`) into a sequence of text-input field rows: each top-level entry in the
        /// `dictionary_json` array — `{ "id", "label"?, "description"?, "value"? }` — becomes one field
        /// dispatching the shared `on_change` action (its `args` are left to the caller; the emitted input's
        /// own id already carries which field changed).
        pub fn from_dictionary(mut self, dictionary_json: &Value, on_change: ActionDescriptor) -> Self {
            let Some(entries) = dictionary_json.as_array() else { return self };
            for entry in entries {
                let Some(id) = entry.get("id").and_then(Value::as_str) else { continue };
                // 📊️ Field labels here come from a runtime `dictionary_json` resource, not a static bundle.
                let label = Label::data(entry.get("label").and_then(Value::as_str).unwrap_or(id).to_string());
                let description = entry.get("description").and_then(Value::as_str).map(str::to_string);
                let value = entry.get("value").and_then(Value::as_str).unwrap_or_default().to_string();
                let field_id = self.field_id(id);
                let control = UiControlNode::Input(UiInputNode {
                    id: field_id,
                    input_kind: "text".into(),
                    value,
                    placeholder: None,
                    commit: None,
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    on_change: on_change.clone(),
                    presence: UiPresence::default(),
                    menu: None,
                });
                self = self.field(id, label, description, control);
            }
            self
        }

        /// 📋️ Sets the trailing submit `Button` row.
        pub fn submit(mut self, label: impl Into<Label>, action: ActionDescriptor) -> Self {
            self.submit = Some(UiButtonNode { id: Some(self.field_id("submit")), icon_id: IconName::CircleDot, label: label.into(), action, style: None, presence: UiPresence::default(), menu: None });
            self
        }

        /// 📋️ Builds `Section > Fields > Button` — the section id is the builder's namespace.
        pub fn build(self) -> UiNode {
            let mut children = self.fields;
            if let Some(submit) = self.submit {
                children.push(UiNode::Button(submit));
            }
            UiNode::Section(UiSectionNode { id: self.namespace, label: None, default_open: Some(true), presence: UiPresence::default(), children, menu: None })
        }
    }

    /// 📋️ A read-only entity-detail panel: `title`/`subtitle` header text, a `KeyValue` summary block built
    /// from `entries` (reusing `ui_wgpu`'s existing `UiKeyValueEntry` rather than a duplicate local type),
    /// and trailing action buttons.
    pub fn entity_detail(title: impl Into<Label>, subtitle: Option<Label>, entries: Vec<UiKeyValueEntry>, actions: Vec<UiButtonNode>) -> UiNode {
        let mut children = vec![ui_text(title)];
        if let Some(subtitle) = subtitle {
            children.push(ui_text(subtitle));
        }
        children.push(UiNode::KeyValue(UiKeyValueNode { entries, presence: UiPresence::default(), menu: None }));
        children.extend(actions.into_iter().map(UiNode::Button));
        ui_stack_vertical(children)
    }

    #[cfg(test)]
    mod form_kit_tests {
            use ui_wgpu::wgpu::Label;
        use super::*;

        #[test]
        fn form_panel_builder_wraps_a_field_control_and_submit_button() {
            let on_change = ActionDescriptor { controller_id: "app".into(), action: "setValue".into(), args: None };
            let submit_action = ActionDescriptor { controller_id: "app".into(), action: "submit".into(), args: None };
            let node = FormPanelBuilder::new("ns-play-form")
                .field(
                    "name",
                    Label::data("Name"),
                    Some("Full name".into()),
                    UiControlNode::Input(UiInputNode {
                        id: "ns-play-form.field.name".into(),
                        input_kind: "text".into(),
                        value: String::new(),
                        placeholder: None,
                        commit: None,
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        on_change,
                        presence: UiPresence::default(),
                        menu: None,
                    }),
                )
                .submit(Label::data("Submit"), submit_action)
                .build();
            let UiNode::Section(section) = node else { panic!("expected a Section node") };
            assert_eq!(section.id, "ns-play-form");
            assert_eq!(section.children.len(), 2);
            let UiNode::Field(field) = &section.children[0] else { panic!("expected a Field node") };
            assert_eq!(field.id, "ns-play-form.field.name");
            assert_eq!(field.description.as_deref(), Some("Full name"));
            let UiNode::Button(button) = &section.children[1] else { panic!("expected a Button node") };
            assert_eq!(button.label.as_str(), "Submit");
        }

        #[test]
        fn form_panel_builder_from_dictionary_routes_entries_into_field_rows() {
            let on_change = ActionDescriptor { controller_id: "app".into(), action: "setValue".into(), args: None };
            let dictionary = serde_json::json!([
                { "id": "email", "label": "Email", "description": "Contact email", "value": "a@b.com" },
                { "id": "phone" },
            ]);
            let node = FormPanelBuilder::new("ns-play-form").from_dictionary(&dictionary, on_change).build();
            let UiNode::Section(section) = node else { panic!("expected a Section node") };
            assert_eq!(section.children.len(), 2);
            let UiNode::Field(email_field) = &section.children[0] else { panic!("expected a Field node") };
            assert_eq!(email_field.id, "ns-play-form.field.email");
            assert_eq!(email_field.label.as_str(), "Email");
            let UiNode::Field(phone_field) = &section.children[1] else { panic!("expected a Field node") };
            assert_eq!(phone_field.label.as_str(), "phone");
        }

        #[test]
        fn entity_detail_builds_a_stack_with_header_key_value_and_actions() {
            let action = ActionDescriptor { controller_id: "app".into(), action: "edit".into(), args: None };
            let node = entity_detail(
                Label::data("Widget"),
                Some(Label::data("A widget")),
                vec![UiKeyValueEntry { label: Label::data("Kind"), value: "gizmo".into() }],
                vec![UiButtonNode { id: None, icon_id: "edit".into(), label: Label::data("Edit"), action, style: None, presence: UiPresence::default(), menu: None }],
            );
            let UiNode::Stack(stack) = node else { panic!("expected a Stack node") };
            assert_eq!(stack.children.len(), 4);
            let UiNode::KeyValue(key_value) = &stack.children[2] else { panic!("expected a KeyValue node") };
            assert_eq!(key_value.entries[0].value, "gizmo");
        }
    }
    //#endregion 🔖️FormKit

    //#region 🔖️Terminology
    // 🗣️ Shared two-axis (locale × terminology) label resolution — replaces the ~25x hand-rolled
    // `struct XLabels { .. }` + `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels`
    // pattern duplicated per app, AND the 4-crate hand-rolled `NATIVE_EN/NATIVE_DE/REUSE_EN/REUSE_DE` +
    // non-exhaustive `match (terminology, is_de) { ..., (_, true) => ... }` terminology resolvers this
    // SDK never covered. `app_labels!` now declares all four cells per field and resolves them via an
    // exhaustive match on the generated `Locale`/`Terminology` enums — no catch-all arm is possible, so
    // adding a locale or terminology to `🔣️ui-axes.json` breaks every invocation until it supplies that
    // cell. See ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.

    pub use ui_wgpu::wgpu::AppLabels;

    /// 🗣️ Anything `resolve_labels` can resolve a label set from — `ViewModel` (locale+terminology
    /// from the shell) and, since the B1 config-driven apps stopped threading `ViewModel` through
    /// render, any per-app `Config` exposing just a raw `cfg.locale` string (region-tolerant: `"de"`
    /// and `"de-DE"` both resolve to `Locale::De`, matching every hand-rolled `is_de_locale` this
    /// replaces) — `terminology()` defaults to `Native`, matching every one of those apps' behavior
    /// (none of them threaded a terminology axis themselves).
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

    /// 🗣️ Region-tolerant `"de"`/`"de-DE"` → `Locale::De` parse, `_` → `Locale::En` — the shared body
    /// of every hand-rolled per-app `is_de_locale`/`fn locale(cfg) -> Locale` this replaces.
    pub fn locale_from_str(locale: &str) -> Locale {
        if locale.starts_with("de") {
            Locale::De
        } else {
            Locale::En
        }
    }

    /// 🗣️ Resolves the active label set for the shell-provided locale/terminology axes.
    pub fn resolve_labels<L: AppLabels>(axes: &impl LabelAxes) -> &'static L {
        L::labels(axes.locale(), axes.terminology())
    }

    /// 🗣️ Config-driven counterpart of `resolve_labels` for the B1 apps whose `Config` type lives
    /// outside their `_ui` crate (the orphan rule blocks `impl LabelAxes for` a foreign `Config` from
    /// `_ui`) — call as `resolve_labels_for_locale::<XLabels>(&cfg.locale)`. Always resolves
    /// `Terminology::Native`, matching every one of those apps' pre-existing behavior.
    pub fn resolve_labels_for_locale<L: AppLabels>(locale: &str) -> &'static L {
        L::labels(locale_from_str(locale), Terminology::Native)
    }

    /// 🗣️ Declares a two-axis label struct plus its four `NATIVE_EN`/`NATIVE_DE`/`REUSE_EN`/`REUSE_DE`
    /// consts and `AppLabels` impl in one compact block — resolve the active set with
    /// `resolve_labels::<XLabels>(view_state)`. Every field requires all four cells explicitly (no
    /// implicit "reuse falls back to native") so a plugin's terminology coverage is always visible at
    /// the declaration site, not inferred.
    ///
    /// ```ignore
    /// semio_framework_plugin::app_labels! {
    ///     struct Puzzle3dLabels {
    ///         object: native_en "Object", native_de "Objekt", reuse_en "Component", reuse_de "Bestandskomponente";
    ///         lod: native_en "LOD", native_de "LOD", reuse_en "LOD", reuse_de "LOD";
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules! app_labels {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident {
            $( $field:ident: native_en $nen:expr, native_de $nde:expr, reuse_en $ren:expr, reuse_de $rde:expr );+ $(;)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $Name {
            $( $vis $field: $crate::LabelText ),+
        }

        impl $Name {
            // 🌐️ PROCESS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION: these carry the
            // struct's own `$vis` (was hardcoded private) — once a plugin's taxonomy splits its
            // `app_labels!` struct into its own `🦀️terminology.rs`, sibling `🎮️commands/*`/
            // `🪟️windows/*` test modules need `Labels::NATIVE_EN` etc. directly (mirrors how flow's
            // pilot never hit this because it never referenced the const cross-module). Additive:
            // only widens visibility, never narrows it, so every existing `app_labels!` caller is
            // unaffected.
            $vis const NATIVE_EN: Self = Self { $( $field: $crate::LabelText::__from_app_labels($nen) ),+ };
            $vis const NATIVE_DE: Self = Self { $( $field: $crate::LabelText::__from_app_labels($nde) ),+ };
            $vis const REUSE_EN: Self = Self { $( $field: $crate::LabelText::__from_app_labels($ren) ),+ };
            $vis const REUSE_DE: Self = Self { $( $field: $crate::LabelText::__from_app_labels($rde) ),+ };
        }

        impl $crate::AppLabels for $Name {
            fn labels(locale: $crate::Locale, terminology: $crate::Terminology) -> &'static Self {
                match (terminology, locale) {
                    ($crate::Terminology::Native, $crate::Locale::En) => &Self::NATIVE_EN,
                    ($crate::Terminology::Native, $crate::Locale::De) => &Self::NATIVE_DE,
                    ($crate::Terminology::Reuse, $crate::Locale::En) => &Self::REUSE_EN,
                    ($crate::Terminology::Reuse, $crate::Locale::De) => &Self::REUSE_DE,
                }
            }
        }
    };
}

    #[cfg(test)]
    mod terminology_tests {
        use super::*;

        app_labels! {
            struct SampleLabels {
                greeting: native_en "Hello", native_de "Hallo", reuse_en "Hi", reuse_de "Servus";
            }
        }

        #[test]
        fn resolve_labels_is_exhaustive_over_all_four_cells() {
            let native_en = ViewModel { locale: Locale::En, terminology: Terminology::Native, ..ViewModel::default() };
            let native_de = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..ViewModel::default() };
            let reuse_en = ViewModel { locale: Locale::En, terminology: Terminology::Reuse, ..ViewModel::default() };
            let reuse_de = ViewModel { locale: Locale::De, terminology: Terminology::Reuse, ..ViewModel::default() };
            assert_eq!(resolve_labels::<SampleLabels>(&native_en).greeting.as_str(), "Hello");
            assert_eq!(resolve_labels::<SampleLabels>(&native_de).greeting.as_str(), "Hallo");
            assert_eq!(resolve_labels::<SampleLabels>(&reuse_en).greeting.as_str(), "Hi");
            assert_eq!(resolve_labels::<SampleLabels>(&reuse_de).greeting.as_str(), "Servus");
        }
    }
    //#endregion 🔖️Terminology

    //#region 🔖️ActionFactory
    // 🎯️ Shared ~30x hand-rolled `fn x_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    // ActionDescriptor { controller_id: X_CONTROLLER_ID.into(), action: action.into(), args:
    // optional_json_to_dsl(args) } }` body — every app keeps its own locally-named wrapper (so call
    // sites never change), delegating to `ActionFactory::new(X_CONTROLLER_ID).action(action, args)`.

    /// 🎯️ Constructs `ActionDescriptor`s bound to one controller id.
    pub struct ActionFactory {
        controller_id: &'static str,
    }

    impl ActionFactory {
        pub const fn new(controller_id: &'static str) -> Self {
            Self { controller_id }
        }

        pub fn action(&self, action: &str, args: Option<Value>) -> ActionDescriptor {
            ActionDescriptor { controller_id: self.controller_id.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
        }
    }
    //#endregion 🔖️ActionFactory

    //#region 🔖️Testkit
    pub mod testkit {
        //! 🧪️ Generic test-harness helpers for `ArtifactApp` implementors. Factors out the ~24x duplicated
        //! `meta()`/`new_app()`/`new_app_with_registry()`/`paired_apps()` boilerplate plus the repeated
        //! undo-redo / two-instance-convergence / ingest-idempotency test *bodies* (parameterized by closures
        //! for the app-specific action names/snapshot shape, so only the control flow is shared). Not
        //! `#[cfg(test)]` — apps' own `#[cfg(test)]` modules call these as a regular dependency; see
        //! `terminology_tests`/`panel_kit_tests` above for the sibling pattern of testing SDK primitives
        //! themselves inline.

        use super::{ActionMeta, App, AppActionRegistry, ArtifactApp, PluginApp, VcsArtifactApp};
        use store::{Backbone, BackboneMessage, MemoryBackbone, SpaceConflict};

        /// 🪪️ A local-actor `ActionMeta` for test dispatch (`instance_id: 1`).
        pub fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        /// 🏛️ Gate 2 (test-time): asserts `manifest_dir` (pass `env!("CARGO_MANIFEST_DIR")` from the plugin
        /// bundle crate) sits next to an `app/` directory whose apps (either a single flattened
        /// `app/{slot}` when the plugin has exactly one app, or `app/<name>/{slot}` per app for multi-app
        /// plugins) each carry all seven constitutional-crate slots (`rs`, `engine`, `dsl`, `op`, `pack`,
        /// `protocol`, `ui`). Invoked automatically by `Plugin::builder` plugin-root sanity checks — see
        /// `.🦑️repo/🎫️tickets/26/07/29/MOVE-APPS-INTO-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES/w31-constitutional-split-recipe.md`.
        /// The bundle crate itself lives at `s/plugin/<p>/rs` — both `../app` and `../../app` are tried
        /// since a manual `#[path]`-included bundle could shift the depth by one. A no-op outside
        /// `s/plugin/` (e.g. the plugin SDK's own in-crate builder tests, which have no real app tree) — the
        /// gate only applies to real migrated plugins, not synthetic test fixtures exercising the macro.
        pub fn assert_constitutional_crates(manifest_dir: &str) {
            const SLOTS: [(&str, &str); 6] = [("engine", "⚙️engine"), ("dsl", "🗣️dsl"), ("op", "🔧️op"), ("pack", "🎒️pack"), ("protocol", "📡️protocol"), ("ui", "🖱️ui")];
            let normalized = manifest_dir.replace('\\', "/");
            if !normalized.contains("/✏️s/🔌️plugins/") {
                return;
            }
            let base = std::path::Path::new(manifest_dir);
            // 🏛️ `../../../../🎛️apps` covers the constitutional-split bundle layout
            // `✏️s/🔌️plugins/<p>/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust`, whose app tree lives four
            // levels up at `✏️s/🔌️plugins/<p>/🎛️apps`.
            let app_root = [base.join("../🎛️apps"), base.join("../../🎛️apps"), base.join("../../../🎛️apps"), base.join("../../../../🎛️apps")].into_iter().find(|candidate| candidate.is_dir());
            let Some(app_root) = app_root else {
                panic!("constitutional-crate gate: no `🎛️apps/` directory found next to {manifest_dir} (tried ../🎛️apps, ../../🎛️apps, ../../../🎛️apps, ../../../../🎛️apps)");
            };
            // 🗿️ A plugin already migrated to the one-crate-per-plugin taxonomy (master ticket
            // `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`) has no per-slot crates left
            // to find — its slots are `🦀️component.rs` files under `🗿️artifacts/<a>/`. Detected by the same
            // marker the registry's discovery contract uses and checked against the taxonomy shape instead.
            // Mirrors the registry's `LEGACY_LAYOUT_TOLERANT` flag: both shapes pass while the migration is
            // in flight. Two entry-file locations are both accepted here (ticket
            // `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`): the pre-V2 owner-root `📦️glue.rs` beside
            // `🗿️artifacts/`, and the V2 shape's `📦️packages/🦀️rust/📦️glue.rs` (entry file relocated inside
            // packages; owner root no longer carries it). A plugin can be in either shape depending on
            // whether its retrofit pass has landed yet.
            if let Some(plugin_root) = app_root.parent() {
                let has_entry_file = plugin_root.join("📦️glue.rs").is_file() || plugin_root.join("📦️packages").join("🦀️rust").join("📦️glue.rs").is_file();
                if has_entry_file && plugin_root.join("🗿️artifacts").is_dir() {
                    assert_taxonomy_components(plugin_root, &app_root);
                    return;
                }
            }
            let is_flat = app_root.join("⚡️implementations").join("🦀️rust").join("Cargo.toml").is_file();
            let app_dirs: Vec<std::path::PathBuf> = if is_flat {
                vec![app_root.clone()]
            } else {
                std::fs::read_dir(&app_root)
                    .unwrap_or_else(|error| panic!("constitutional-crate gate: cannot read {}: {error}", app_root.display()))
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .filter(|path| path.is_dir() && path.file_name().and_then(|name| name.to_str()) != Some("🤝️shared"))
                    .collect()
            };
            if app_dirs.is_empty() {
                panic!("constitutional-crate gate: {} declares no apps", app_root.display());
            }
            for app_dir in &app_dirs {
                let mut missing = Vec::new();
                if !app_dir.join("⚡️implementations").join("🦀️rust").join("Cargo.toml").is_file() {
                    missing.push("rs".to_string());
                }
                for (slot, slot_dir) in SLOTS {
                    if !app_dir.join("🔨️modules").join(slot_dir).join("⚡️implementations").join("🦀️rust").join("Cargo.toml").is_file() {
                        missing.push(slot.to_string());
                    }
                }
                assert!(missing.is_empty(), "constitutional-crate gate: {} is missing slot(s): {}", app_dir.display(), missing.join(", "));
            }
        }

        /// 🗿️ The taxonomy-shape half of [`assert_constitutional_crates`]: every `🗿️artifacts/<artifact>/`
        /// carries the completeness component slots (incl. `🧬️mutations` + `⚙️engine`) as `🦀️component.rs` leaves,
        /// and every `🎛️apps/<app>/` has its own `🦀️component.rs`. The Rust-side twin of the registry script's
        /// `validateTaxonomyTree`, kept here so a plugin's own `cargo test` catches a half-finished migration
        /// without waiting for the TS gate.
        ///
        /// Full `🧬️mutations/<mutation>/{🦠️mutation,🔺️diff,↩️inverse}` triad walking is policy/registry-side
        /// for now (`validateTaxonomyTree`); this twin only hard-requires the facet leaves themselves.
        fn assert_taxonomy_components(plugin_root: &std::path::Path, app_root: &std::path::Path) {
            let taxonomy = load_taxonomy_json();
            let artifact_components = string_array(&taxonomy, "artifactComponentDirs");
            let schema_child_dirs = string_array(&taxonomy, "schemaChildDirs");
            let representation_dirs = string_array(&taxonomy, "representationDirs");
            let _representation_dirs = representation_dirs;
            let _schema_child_dirs = schema_child_dirs;
            let config_child_dirs = string_array(&taxonomy, "configChildDirs");
            let presence_child_dirs = string_array(&taxonomy, "presenceChildDirs");
            let schema_leaf_filenames = schema_format_leaf_filenames(&taxonomy);
            let forbidden_example_plurals = string_array(&taxonomy, "forbiddenExamplePluralDirs");
            let example_asset_kind_prefixes = object_string_values(&taxonomy, "exampleAssetKindPrefixes");
            let examples = taxonomy
                .get("artifactChildDirs")
                .and_then(|v| v.as_array())
                .and_then(|dirs| dirs.iter().find_map(|d| d.as_str().filter(|s| s.contains("examples"))))
                .unwrap_or("📚️examples");
            let example_assets = taxonomy.get("exampleAssetsDirName").and_then(|v| v.as_str()).unwrap_or("🖼️assets");
            let example_tests = taxonomy.get("exampleTestsDirName").and_then(|v| v.as_str()).unwrap_or("🧪️tests");
            let leaf = taxonomy
                .get("taxonomyLeafFilenames")
                .and_then(|v| v.as_object())
                .and_then(|m| m.values().find_map(|v| v.as_str().filter(|s| s.ends_with("component.rs"))))
                .unwrap_or("🦀️component.rs");
            let schema_dir = "🧬️schema";
            let config_dir = "🎚️config";
            let presence_dir = "👥️presence";
            let io_dir = "🚪️io";
            let legacy_config_dir = "🧮️config";
            assert!(
                !example_asset_kind_prefixes.is_empty(),
                "taxonomy gate: exampleAssetKindPrefixes must be non-empty in 🔣️taxonomy.json"
            );
            let subdirectories = |dir: &std::path::Path| -> Vec<std::path::PathBuf> {
                std::fs::read_dir(dir)
                    .unwrap_or_else(|error| panic!("taxonomy gate: cannot read {}: {error}", dir.display()))
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .filter(|path| path.is_dir())
                    .collect()
            };

            let artifacts_dir_name = taxonomy.get("artifactsDirName").and_then(|v| v.as_str()).unwrap_or("🗿️artifacts");
            let artifacts_root = plugin_root.join(artifacts_dir_name);
            let artifacts = subdirectories(&artifacts_root);
            assert!(!artifacts.is_empty(), "taxonomy gate: {} declares no artifacts", artifacts_root.display());
            for artifact in &artifacts {
                //#region StdioCompleteness
                // Soft-require builder/decomposer; require schema dir, engine leaf, io dir+leaf (nested schema/io matrix lands in W5/W6).
                //#endregion StdioCompleteness
                let builder_dir = "🏗️builder";
                let decomposer_dir = "🪓️decomposer";
                let missing: Vec<&str> = artifact_components
                    .iter()
                    .map(String::as_str)
                    .filter(|component| {
                        if *component == builder_dir || *component == decomposer_dir {
                            false
                        } else if *component == schema_dir {
                            !artifact.join(component).is_dir()
                        } else if *component == io_dir {
                            let root = artifact.join(component);
                            !root.is_dir() || !root.join(leaf).is_file()
                        } else {
                            !artifact.join(component).join(leaf).is_file()
                        }
                    })
                    .collect();
                assert!(missing.is_empty(), "taxonomy gate: artifact {} is missing component(s): {}", artifact.display(), missing.join(", "));

                let schema_root = artifact.join(schema_dir);
                if schema_root.is_dir() {
                    let missing_leaves: Vec<&str> = schema_leaf_filenames
                        .iter()
                        .map(String::as_str)
                        .filter(|name| !schema_root.join(name).is_file())
                        .collect();
                    assert!(
                        missing_leaves.is_empty(),
                        "taxonomy gate: artifact {} is missing {schema_dir} leaf(ves): {}",
                        artifact.display(),
                        missing_leaves.join(", ")
                    );
                    let _ = &_schema_child_dirs;
                    let _ = &_representation_dirs;
                }

                let examples_root = artifact.join(examples);
                assert!(examples_root.is_dir(), "taxonomy gate: artifact {} is missing {examples}", artifact.display());
                let example_sets = subdirectories(&examples_root);
                assert!(!example_sets.is_empty(), "taxonomy gate: artifact {} {examples} has no example set", artifact.display());
                for example_set in &example_sets {
                    for plural in &forbidden_example_plurals {
                        assert!(
                            !example_set.join(plural).is_dir(),
                            "taxonomy gate: artifact {} example {} still has plural {plural}",
                            artifact.display(),
                            example_set.file_name().unwrap_or_default().to_string_lossy()
                        );
                    }
                    assert!(
                        example_set.join(example_assets).is_dir(),
                        "taxonomy gate: artifact {} example {} missing {example_assets}",
                        artifact.display(),
                        example_set.file_name().unwrap_or_default().to_string_lossy()
                    );
                    assert!(
                        example_set.join(example_tests).is_dir(),
                        "taxonomy gate: artifact {} example {} missing {example_tests}",
                        artifact.display(),
                        example_set.file_name().unwrap_or_default().to_string_lossy()
                    );
                }
            }

            assert!(!plugin_root.join(examples).is_dir(), "taxonomy gate: plugin-root {examples} is forbidden at {}", plugin_root.display());

            let plugin_child_dirs = string_array(&taxonomy, "pluginChildDirs");
            assert!(!plugin_root.join("🔌️plugin").is_dir(), "taxonomy gate: redundant 🔌️plugin directory at {}; move its contract and facets directly into the plugin root", plugin_root.display());
            assert!(plugin_root.join(leaf).is_file(), "taxonomy gate: plugin root missing {leaf} at {}", plugin_root.display());
            for child in &plugin_child_dirs {
                assert!(
                    plugin_root.join(child).join(leaf).is_file(),
                    "taxonomy gate: plugin root missing {child}/{leaf} at {}",
                    plugin_root.display()
                );
            }

            let apps = subdirectories(app_root);
            assert!(!apps.is_empty(), "taxonomy gate: {} declares no apps", app_root.display());
            let assert_app_schema_owner = |owner_label: &str, parent: &std::path::Path| {
                let config_root = parent.join(config_dir);
                if !config_root.is_dir() {
                    return;
                }
                for child in &config_child_dirs {
                    let child_dir = config_root.join(child);
                    if child == schema_dir {
                        assert!(child_dir.is_dir(), "taxonomy gate: {owner_label} is missing {config_dir}/{child}");
                        let missing_leaves: Vec<&str> = schema_leaf_filenames
                            .iter()
                            .map(String::as_str)
                            .filter(|name| !child_dir.join(name).is_file())
                            .collect();
                        assert!(
                            missing_leaves.is_empty(),
                            "taxonomy gate: {owner_label} is missing {config_dir}/{child} leaf(ves): {}",
                            missing_leaves.join(", ")
                        );
                    }
                }
                let presence_root = parent.join(presence_dir);
                assert!(presence_root.is_dir(), "taxonomy gate: {owner_label} is missing {presence_dir}");
                for child in &presence_child_dirs {
                    let child_dir = presence_root.join(child);
                    if child == schema_dir {
                        assert!(child_dir.is_dir(), "taxonomy gate: {owner_label} is missing {presence_dir}/{child}");
                        let missing_leaves: Vec<&str> = schema_leaf_filenames
                            .iter()
                            .map(String::as_str)
                            .filter(|name| !child_dir.join(name).is_file())
                            .collect();
                        assert!(
                            missing_leaves.is_empty(),
                            "taxonomy gate: {owner_label} is missing {presence_dir}/{child} leaf(ves): {}",
                            missing_leaves.join(", ")
                        );
                    }
                }
            };
            for app in &apps {
                assert!(app.join(leaf).is_file(), "taxonomy gate: app {} is missing its {leaf}", app.display());
                // ⚙️ `appComponentDirs` puts the headless engine directly under the app; app examples
                // live at the APP root (`🎛️apps/<app>/📚️examples`), never inside the engine — the
                // registry's `validateTaxonomyTree` flags `⚙️engine/📚️examples` for exactly that reason,
                // so this twin requires the engine dir itself instead of an engine-owned examples dir.
                assert!(
                    app.join("⚙️engine").is_dir(),
                    "taxonomy gate: app {} is missing ⚙️engine/",
                    app.display()
                );
                assert!(
                    !app.join(legacy_config_dir).is_dir(),
                    "taxonomy gate: app {} still has {legacy_config_dir} — rename to {config_dir}",
                    app.display()
                );
                assert_app_schema_owner(&format!("app {}", app.display()), app);
            }
            assert!(
                !plugin_root.join(legacy_config_dir).is_dir(),
                "taxonomy gate: plugin-root still has {legacy_config_dir} — rename to {config_dir}"
            );
            assert_app_schema_owner(&format!("plugin-root {}", plugin_root.display()), plugin_root);
        }

        //#region TaxonomyJson
        /// 🔣️ Walks from `CARGO_MANIFEST_DIR` up to the repo root (marker: `nx.json`) and parses `🔣️taxonomy.json`.
        fn load_taxonomy_json() -> serde_json::Value {
            let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let mut dir = manifest_dir.as_path();
            let repo_root = loop {
                if dir.join("nx.json").is_file() {
                    break dir.to_path_buf();
                }
                dir = dir.parent().unwrap_or_else(|| panic!("taxonomy gate: walked above filesystem without finding nx.json from {}", manifest_dir.display()));
            };
            let taxonomy_path = repo_root
                .join("🧰️framework")
                .join("🛍️products")
                .join("🦑️repo")
                .join("🔨️modules")
                .join("📚️library")
                .join("🔣️taxonomy.json");
            let raw = std::fs::read_to_string(&taxonomy_path).unwrap_or_else(|error| panic!("taxonomy gate: cannot read {}: {error}", taxonomy_path.display()));
            serde_json::from_str(&raw).unwrap_or_else(|error| panic!("taxonomy gate: cannot parse {}: {error}", taxonomy_path.display()))
        }

        fn string_array(taxonomy: &serde_json::Value, key: &str) -> Vec<String> {
            taxonomy
                .get(key)
                .and_then(|v| v.as_array())
                .unwrap_or_else(|| panic!("taxonomy gate: 🔣️taxonomy.json missing non-empty array `{key}`"))
                .iter()
                .map(|v| v.as_str().unwrap_or_else(|| panic!("taxonomy gate: `{key}` entry is not a string")).to_string())
                .collect()
        }

        fn object_string_values(taxonomy: &serde_json::Value, key: &str) -> Vec<String> {
            taxonomy
                .get(key)
                .and_then(|v| v.as_object())
                .unwrap_or_else(|| panic!("taxonomy gate: 🔣️taxonomy.json missing object `{key}`"))
                .values()
                .map(|v| v.as_str().unwrap_or_else(|| panic!("taxonomy gate: `{key}` value is not a string")).to_string())
                .collect()
        }

        fn schema_format_leaf_filenames(taxonomy: &serde_json::Value) -> Vec<String> {
            taxonomy
                .get("schemaFormats")
                .and_then(|v| v.as_object())
                .unwrap_or_else(|| panic!("taxonomy gate: 🔣️taxonomy.json missing object `schemaFormats`"))
                .values()
                .map(|v| {
                    v.get("leafFilename")
                        .and_then(|n| n.as_str())
                        .unwrap_or_else(|| panic!("taxonomy gate: schemaFormats entry missing leafFilename"))
                        .to_string()
                })
                .collect()
        }
        //#endregion TaxonomyJson

        /// 🧬️ A registry-less wrapper (`VcsArtifactApp::new`) — contract enforcement (required args, kind
        /// discipline) is skipped, matching most apps' plain unit tests.
        pub fn new_app<A: ArtifactApp + Default>() -> VcsArtifactApp<A> {
            VcsArtifactApp::new(A::default())
        }

        /// 🧬️ A registry-backed wrapper carrying `manifest`'s real `AppActionRegistry` — needed whenever a
        /// test must exercise declared-arg defaults/required-arg enforcement or View/Shell kind discipline.
        pub fn new_app_with_registry<A: ArtifactApp + Default>(manifest: fn() -> App) -> VcsArtifactApp<A> {
            let definition = manifest().definition;
            VcsArtifactApp::with_registry(A::default(), AppActionRegistry::from_definition(&definition))
        }

        /// 🔗️ Two registry-less instances joined by an in-memory backbone on `channel` — the standard fixture
        /// for convergence tests.
        pub fn paired_apps<A: ArtifactApp + Default>(channel: &str) -> (VcsArtifactApp<A>, VcsArtifactApp<A>) {
            let mut a = new_app::<A>();
            let mut b = new_app::<A>();
            let (backbone_a, backbone_b) = MemoryBackbone::pair(channel, channel);
            a.attach_backbone(Box::new(backbone_a)).expect("attach a");
            b.attach_backbone(Box::new(backbone_b)).expect("attach b");
            (a, b)
        }

        /// 🧪️ Runs `command` once via the typed channel, asserts `probe(app)` matches `after`, undoes (still
        /// the framework-reserved `"undo"` action) and asserts `before`, redoes and asserts `after` again — the
        /// repeated undo/redo round-trip test body. B1: takes a typed `A::Command` value (`dispatch_typed`) —
        /// `ArtifactApp::handle_action`'s stringly-typed dispatch no longer exists.
        pub fn assert_undo_redo_round_trip<A, P>(app: &mut VcsArtifactApp<A>, command: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, before: P, after: P)
        where
            A: ArtifactApp,
            P: PartialEq + std::fmt::Debug,
        {
            app.dispatch_typed(command, &meta("local")).expect("apply command");
            assert_eq!(probe(app), after, "command did not produce the expected snapshot");
            app.handle_action("undo", None, &meta("local")).expect("undo");
            assert_eq!(probe(app), before, "undo did not revert to the expected snapshot");
            app.handle_action("redo", None, &meta("local")).expect("redo");
            assert_eq!(probe(app), after, "redo did not reapply the expected snapshot");
        }

        /// 🧪️ Every declared app action must bridge through `command_from_action` and round-trip `command_id`.
        pub fn assert_declared_actions_bridge_to_commands<A: ArtifactApp + Default>(manifest: fn() -> App) {
            use semio_framework::{effective_action_args, DslValue};
            use store::pack_rt::dsl_value_to_json;
            let definition = manifest().definition;
            let app = A::default();
            let skip = [
                "undo",
                "redo",
                "commitCheckpoint",
                "createAlternative",
                "switchAlternative",
                "checkoutCheckpoint",
                "copy",
                "cut",
                "paste",
                "revertToCommand",
                "setHistoryCommandFilter",
                "noteShellCommand",
                "recordTutorial",
                "startIntroduction",
                "startTutorial",
                "setActiveUtility",
                "setActiveTool",
            ];
            for action in &definition.actions {
                if skip.contains(&action.id.as_str()) {
                    continue;
                }
                let staged = effective_action_args(&action.args, &DslValue::Object(Vec::new()));
                let args_json = dsl_value_to_json(staged);
                let command = A::command_from_action(&action.id, Some(&args_json)).unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
                assert_eq!(A::command_id(&command), action.id.as_str(), "command_id mismatch for action {}", action.id);
            }
        }

        /// 🧪️ `command_a`/`command_b` are applied to two `paired_apps` instances, a neutral history action
        /// (`commitCheckpoint`) pumps each side's inbound operations, then `probe` must agree on both — the repeated
        /// two-instance-convergence test body (see `playbook-plugin`'s
        /// `two_instances_converge_disjoint_edits_via_backbone` for the original, app-specific version).
        pub fn assert_two_instances_converge<A, P>(channel: &str, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P)
        where
            A: ArtifactApp + Default,
            P: PartialEq + std::fmt::Debug,
        {
            let (mut instance_a, mut instance_b) = paired_apps::<A>(channel);
            instance_a.dispatch_typed(command_a, &meta("actor-a")).expect("a applies its edit");
            instance_b.dispatch_typed(command_b, &meta("actor-b")).expect("b applies its edit");
            instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
            instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");
            assert_eq!(probe(&instance_a), probe(&instance_b), "both instances must converge on the same snapshot");
        }

        /// 🧪️ The `Mutation::reconcile` counterpart to `assert_two_instances_converge`: `command_delete`/
        /// `command_wire` race on two `paired_apps` instances (typically one deletes a graph node, the other
        /// concurrently wires an edge to it), a `commitCheckpoint` pumps each side's inbound operations, then both
        /// sides' post-reconcile `probe` results (`(snapshot, conflicts)`) must agree, `has_dangling_ref`
        /// must be false for the converged snapshot, and at least one `SpaceConflict` must have been
        /// reported (dropping a dangling reference silently, with no conflict, would hide real data loss).
        pub fn assert_graph_merge_preserves_referential_integrity<A, P>(channel: &str, command_delete: A::Command, command_wire: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> (P, Vec<SpaceConflict>), has_dangling_ref: impl Fn(&P) -> bool)
        where
            A: ArtifactApp + Default,
            P: PartialEq + std::fmt::Debug,
        {
            let (mut instance_a, mut instance_b) = paired_apps::<A>(channel);
            instance_a.dispatch_typed(command_delete, &meta("actor-a")).expect("a deletes the node");
            instance_b.dispatch_typed(command_wire, &meta("actor-b")).expect("b wires the edge");
            instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
            instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");
            let (snapshot_a, conflicts_a) = probe(&instance_a);
            let (snapshot_b, conflicts_b) = probe(&instance_b);
            assert_eq!(snapshot_a, snapshot_b, "both instances must converge on the same reconciled snapshot");
            assert!(!has_dangling_ref(&snapshot_a), "converged snapshot must not retain a dangling reference");
            assert!(!conflicts_a.is_empty(), "dropping the dangling reference must surface a SpaceConflict");
            assert_eq!(conflicts_a, conflicts_b, "both instances must report the same reconciliation conflicts");
        }

        /// 🧪️ Applies `command` on a sender attached to a backbone, replays the resulting envelopes onto a
        /// fresh receiver twice, and asserts `probe` sees the same result both times — feeding the same operation
        /// twice must not double-apply.
        pub fn assert_ingest_idempotent<A, P>(command: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P)
        where
            A: ArtifactApp + Default,
            P: PartialEq + std::fmt::Debug,
        {
            let mut sender = new_app::<A>();
            let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent");
            sender.attach_backbone(Box::new(near)).expect("attach sender");
            sender.dispatch_typed(command, &meta("local")).expect("apply command");

            let mut envelopes = Vec::new();
            for message in far.receive().expect("receive") {
                if let BackboneMessage::Mutations { envelopes: operations } = message {
                    envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
                }
            }
            let operations = protocol::encode_envelopes(&envelopes);

            let mut receiver = new_app::<A>();
            receiver.ingest_operations(&operations).expect("ingest once");
            let once = probe(&receiver);
            receiver.ingest_operations(&operations).expect("ingest twice");
            assert_eq!(probe(&receiver), once, "feeding the same operation twice must not double-apply");
        }

        #[cfg(test)]
        mod testkit_tests {
            //! 🧪️ Proves each `testkit` primitive against a minimal dummy `ArtifactApp` before any real app
            //! adopts them.
            use super::super::{ConfigView, DraftView, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation};
            use super::*;
            use crate::app::{ArtifactView, Emit};
            use store::EngineHandles;
            use crate::{ui_text, UiNode};
            use protocol::{Mutation, MutationDiff};
            use serde::{Deserialize, Serialize};
            use semio_framework::Fault;
            use ui_wgpu::wgpu::Label;

            #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
            #[dsl(extension = "testkit-dummy")]
            struct DummySnapshot {
                count: i32,
            }

            /// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack for SDK test double (artifact coincides with snapshot only in tests).
            impl store::ArtifactDsl for DummySnapshot {
                const EXTENSION: &'static str = "testkit-dummy";
                fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
                    if text.trim().is_empty() {
                        return Ok(Self::default());
                    }
                    serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
                }
                fn print_dsl(&self) -> String {
                    serde_json::to_string(self).unwrap_or_default()
                }
            }

            impl store::ArtifactPack for DummySnapshot {
                fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                    serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
                }
                fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                    if bytes.is_empty() {
                        return Ok(Self::default());
                    }
                    serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
                }
            }

            #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
            struct DummyDiff {
                count: Option<i32>,
            }

            impl MutationDiff<DummySnapshot> for DummyDiff {
                fn apply(&self, snapshot: &DummySnapshot) -> DummySnapshot {
                    DummySnapshot { count: self.count.unwrap_or(snapshot.count) }
                }

                fn absorb(&mut self, other: Self) {
                    if other.count.is_some() {
                        self.count = other.count;
                    }
                }
            }

            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
            #[serde(tag = "operation", rename_all = "camelCase")]
            enum DummyMutation {
                #[dsl(key = "set-count")]
                SetCount { value: i32 },
            }

            impl Mutation<DummySnapshot> for DummyMutation {
                type Diff = DummyDiff;

                fn diff(&self, _snapshot: &DummySnapshot) -> DummyDiff {
                    match self {
                        DummyMutation::SetCount { value } => DummyDiff { count: Some(*value) },
                    }
                }

                fn inverse(&self, snapshot: &DummySnapshot) -> Vec<Self> {
                    vec![DummyMutation::SetCount { value: snapshot.count }]
                }
            }

            impl ::protocol::OpText for DummyMutation {
                fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                    let variants = <Self as ::dsl::DslVariants>::variants();
                    for (keyword, spec_fn) in &variants {
                        let probe = format!("{keyword} ");
                        if line == keyword.as_str() || line.starts_with(&probe) {
                            let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                            let record = ::dsl::parse(
                                body,
                                &spec_fn(),
                                &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                            )?;
                            return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                        }
                    }
                    Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
                }
                fn print_op(&self) -> String {
                    let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                    let variants = <Self as ::dsl::DslVariants>::variants();
                    let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                    let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                    if body.is_empty() {
                        keyword
                    } else {
                        format!("{keyword} {body}")
                    }
                }
            }

            impl ::protocol::OpBinary for DummyMutation {
                fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                    ::dsl::variants_binary::encode_op(self)
                }
                fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                    ::dsl::variants_binary::decode_op(bytes)
                }
            }

            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
            enum DummyCommand {
                #[dsl(key = "increment")]
                Increment,
            }

            impl ::protocol::OpText for DummyCommand {
                fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                    let variants = <Self as ::dsl::DslVariants>::variants();
                    for (keyword, spec_fn) in &variants {
                        let probe = format!("{keyword} ");
                        if line == keyword.as_str() || line.starts_with(&probe) {
                            let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                            let record = ::dsl::parse(
                                body,
                                &spec_fn(),
                                &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                            )?;
                            return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                        }
                    }
                    Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
                }
                fn print_op(&self) -> String {
                    let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                    let variants = <Self as ::dsl::DslVariants>::variants();
                    let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                    let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                    if body.is_empty() {
                        keyword
                    } else {
                        format!("{keyword} {body}")
                    }
                }
            }

            impl ::protocol::OpBinary for DummyCommand {
                fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                    ::dsl::variants_binary::encode_op(self)
                }
                fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                    ::dsl::variants_binary::decode_op(bytes)
                }
            }

            #[derive(Default)]
            struct DummyApp;

            impl ArtifactApp for DummyApp {
                const APP_ID: &'static str = "testkit-dummy";
                const DOCUMENT_SCHEMA: &'static str = "semio.testkit/v1";
                type Snapshot = DummySnapshot;
                type Mutation = DummyMutation;
                type Config = NoConfig;
                type ConfigMutation = NoConfigMutation;
                type Draft = NoDraft;
                type DraftMutation = NoDraftMutation;
                type Presence = NoPresence;
                type PresenceMutation = NoPresenceMutation;
                type Transient = crate::app::NoTransient;
                type TransientMutation = crate::app::NoTransientMutation;
                type Command = DummyCommand;

                fn initial_snapshot() -> DummySnapshot {
                    DummySnapshot::default()
                }

                fn handle(command: &DummyCommand, doc: &ArtifactView<'_, DummySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> Result<Emit<DummyMutation>, Fault> {
                    match command {
                        DummyCommand::Increment => Ok(Emit { artifact_mutations: vec![DummyMutation::SetCount { value: doc.snapshot.count + 1 }], description: Some("increment".into()), ..Default::default() }),
                    }
                }

                fn render(_body_key: &str, doc: &ArtifactView<'_, DummySnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> UiNode {
                    ui_text(Label::data(format!("count={}", doc.snapshot.count)))
                }
            }

            #[test]
            fn meta_carries_actor_and_local_instance_id() {
                let m = meta("actor-x");
                assert_eq!(m.actor, "actor-x");
                assert_eq!(m.instance_id, 1);
            }

            #[test]
            fn new_app_constructs_a_registry_less_wrapper() {
                let mut app = new_app::<DummyApp>();
                app.dispatch_typed(DummyCommand::Increment, &meta("local")).expect("increment");
                assert_eq!(app.snapshot().unwrap().count, 1);
            }

            #[test]
            fn assert_undo_redo_round_trip_passes_for_a_real_operation() {
                let mut app = new_app::<DummyApp>();
                assert_undo_redo_round_trip(&mut app, DummyCommand::Increment, |app| app.snapshot().unwrap().count, 0, 1);
            }

            #[test]
            fn assert_two_instances_converge_on_disjoint_edits() {
                assert_two_instances_converge::<DummyApp, i32>("mem://testkit-converge", DummyCommand::Increment, DummyCommand::Increment, |app| app.snapshot().unwrap().count);
            }

            #[test]
            fn assert_ingest_idempotent_does_not_double_apply() {
                assert_ingest_idempotent::<DummyApp, i32>(DummyCommand::Increment, |app| app.snapshot().unwrap().count);
            }
        }
    }
    //#endregion 🔖️Testkit

    #[cfg(test)]
    mod app_builder_tests {
            use ui_wgpu::wgpu::LocalizedLabel;
        use super::*;
        use ui_wgpu::wgpu::create_default_layout;

        #[test]
        fn build_definition_rejects_layout_with_unknown_window_kind() {
            let result = std::panic::catch_unwind(|| {
                App::builder("bad-app", LocalizedLabel::data("Bad"))
                    .document(["semio", "bad"])
                    .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                    .mode_tools("edit", vec![])
                    .window_kind("main", LocalizedLabel::data("Main"), "bad.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                    .default_layout(create_default_layout(&["missing".into()], "row", None, None))
                    .build_definition();
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_accepts_valid_manifest() {
            let definition = App::builder("good-app", LocalizedLabel::data("Good"))
                .document(["semio", "good"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .mode_tools("edit", vec![])
                .window_kind("main", LocalizedLabel::data("Main"), "good.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .panel_tab("framework.panel.artifact", LocalizedLabel::data("Document"), PanelGroup::Workbench, "good.document")
                .default_layout(create_default_layout(&["main".into()], "row", None, None))
                .build_definition();
            assert_eq!(definition.window_kinds.len(), 1);
            assert_eq!(definition.window_kinds.iter().next().map(|kind| kind.icon_id.as_str()), Some("app-window"));
            assert_eq!(definition.modes.first().icon_id.as_str(), "pencil");
            // 🕰️ 1 declared + the auto-injected framework History tab.
            assert_eq!(definition.panel_tabs.len(), 2);
        }

        #[test]
        fn catalog_chrome_icons_resolve_to_vendored_icon_names() {
            for mode in ["edit", "paint", "generate", "explore", "builder", "review", "report"] {
                let icon = semio_framework::catalog_mode_icon_id(mode);
                assert_eq!(IconName::from_str(icon.as_str()), Some(icon), "mode {mode} -> {}", icon.as_str());
            }
            assert_eq!(IconName::from("menu").as_str(), "list");
            assert_eq!(IconName::from("square-pen").as_str(), "pencil");
            assert_eq!(IconName::from("trees").as_str(), "list-tree");
            let definition = App::builder("icon-app", LocalizedLabel::data("Icon"))
                .document(["semio", "icon"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .mode_tools("edit", vec![])
                .window_kind("main", LocalizedLabel::data("Main"), "icon.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .default_layout(create_default_layout(&["main".into()], "row", None, None))
                .build_definition();
            assert_eq!(definition.modes.first().icon_id.as_str(), "pencil");
        }

        #[test]
        fn build_definition_rejects_terminology_document_for_undeclared_terminology() {
            let result = std::panic::catch_unwind(|| {
                App::builder("bad-terminology-app", LocalizedLabel::data("Bad"))
                    .document(["semio", "bad"])
                    .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                    .mode_tools("edit", vec![])
                    .window_kind("main", LocalizedLabel::data("Main"), "bad.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                    .default_layout(create_default_layout(&["main".into()], "row", None, None))
                    .terminology_document("reuse", ["Entwerfen mit Bestand", "Bad"])
                    .build_definition();
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_accepts_declared_terminology_document() {
            let definition = App::builder("good-terminology-app", LocalizedLabel::data("Good"))
                .document(["semio", "good"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .mode_tools("edit", vec![])
                .window_kind("main", LocalizedLabel::data("Main"), "good.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .default_layout(create_default_layout(&["main".into()], "row", None, None))
                .terminology("reuse")
                .terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])
                .build_definition();
            assert_eq!(definition.terminology_breadcrumbs.get("reuse").map(Vec::as_slice), Some(["Entwerfen mit Bestand".to_string(), "Aggregator".to_string()].as_slice()));
        }

        fn minimal_app(id: &str) -> AppBuilder {
            App::builder(id, LocalizedLabel::data("App")).document(["semio", id]).mode("edit", LocalizedLabel::data("Edit"), "pencil").window_kind("main", LocalizedLabel::data("Main"), format!("{id}.main"), SurfaceKind::Canvas2d, IconName::AppWindow)
        }

        #[test]
        fn build_definition_auto_injects_history_actions_and_keybindings() {
            let definition = minimal_app("history-app").build_definition();
            let history_ids: HashSet<&str> = definition.actions.iter().map(|c| c.id.as_str()).collect();
            assert!(history_ids.contains("undo"));
            assert!(history_ids.contains("redo"));
            assert!(history_ids.contains("commitCheckpoint"));
            assert!(history_ids.contains("createAlternative"));
            assert!(history_ids.contains("switchAlternative"));
            assert!(history_ids.contains("checkoutCheckpoint"));
            let undo_binding = definition.keybindings.iter().find(|binding| binding.keys == "mod+z").expect("undo keybinding auto-injected");
            assert_eq!(undo_binding.action.action, "undo");
            assert_eq!(undo_binding.action.controller_id, "history-app");
        }

        #[test]
        fn build_definition_does_not_duplicate_manually_declared_history_keybinding() {
            let definition = minimal_app("manual-undo-app").keybinding("mod+z", "undo").build_definition();
            assert_eq!(definition.keybindings.iter().filter(|b| b.keys == "mod+z").count(), 1);
        }

        #[test]
        fn build_definition_auto_injects_clipboard_actions_and_keybindings() {
            let definition = minimal_app("clipboard-app").build_definition();
            let clipboard_ids: HashSet<&str> = definition.actions.iter().map(|c| c.id.as_str()).collect();
            assert!(clipboard_ids.contains("copy"));
            assert!(clipboard_ids.contains("cut"));
            assert!(clipboard_ids.contains("paste"));
            let copy_action = definition.actions.iter().find(|a| a.id == "copy").expect("copy declared");
            assert_eq!(copy_action.kind, ActionKind::Clipboard);
            let paste_action = definition.actions.iter().find(|a| a.id == "paste").expect("paste declared");
            assert!(paste_action.args.iter().any(|arg| arg.id == "anchor"));
            let copy_binding = definition.keybindings.iter().find(|binding| binding.keys == "mod+c").expect("copy keybinding auto-injected");
            assert_eq!(copy_binding.action.action, "copy");
            assert_eq!(copy_binding.action.controller_id, "clipboard-app");
            let paste_binding = definition.keybindings.iter().find(|binding| binding.keys == "mod+v").expect("paste keybinding auto-injected");
            assert_eq!(paste_binding.action.action, "paste");
        }

        #[test]
        fn build_definition_auto_injects_the_history_panel_tab_and_filter_action() {
            let definition = minimal_app("history-panel-app").build_definition();
            assert!(definition.panel_tabs.iter().any(|tab| tab.id() == ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID));
            let action_ids: HashSet<&str> = definition.actions.iter().map(|a| a.id.as_str()).collect();
            assert!(action_ids.contains(REVERT_TO_COMMAND_ACTION_ID));
            assert!(action_ids.contains(SET_HISTORY_COMMAND_FILTER_ACTION_ID));
            let revert = definition.actions.iter().find(|a| a.id == REVERT_TO_COMMAND_ACTION_ID).expect("revertToCommand declared");
            assert_eq!(revert.kind, ActionKind::History);
            assert!(!revert.in_palette);
            let filter = definition.actions.iter().find(|a| a.id == SET_HISTORY_COMMAND_FILTER_ACTION_ID).expect("setHistoryCommandFilter declared");
            assert_eq!(filter.kind, ActionKind::View);
            assert!(!filter.in_palette);
        }

        #[test]
        fn build_definition_does_not_duplicate_a_manually_declared_history_panel_tab() {
            let definition = minimal_app("manual-history-app").panel_tab(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID, LocalizedLabel::data("Custom History"), PanelGroup::Settings, "custom.history").build_definition();
            assert_eq!(definition.panel_tabs.iter().filter(|tab| tab.id() == ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID).count(), 1);
            let tab = definition.panel_tabs.iter().find(|tab| tab.id() == ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID).expect("history tab present");
            assert_eq!(tab.body_key.as_deref(), Some("custom.history"));
        }

        #[test]
        fn operation_view_and_shell_actions_are_declared_with_their_kind() {
            let definition =
                minimal_app("typed-actions-app").mutation("addLayer", LocalizedLabel::data("Add Layer")).view_action("setCamera", LocalizedLabel::data("Set Camera")).shell_action("exportPng", LocalizedLabel::data("Export PNG")).build_definition();
            let by_id = |id: &str| definition.actions.iter().find(|c| c.id == id).expect("declared");
            assert_eq!(by_id("addLayer").kind, ActionKind::Mutation);
            assert_eq!(by_id("setCamera").kind, ActionKind::View);
            assert_eq!(by_id("exportPng").kind, ActionKind::Shell);
        }

        #[test]
        fn build_definition_rejects_duplicate_action_ids() {
            let result = std::panic::catch_unwind(|| minimal_app("dupe-action-app").mutation("addLayer", LocalizedLabel::data("Add Layer")).mutation("addLayer", LocalizedLabel::data("Add Layer Again")).build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_keybinding_for_undeclared_action_once_opted_in() {
            let result = std::panic::catch_unwind(|| minimal_app("undeclared-keybinding-app").mutation("addLayer", LocalizedLabel::data("Add Layer")).keybinding("mod+l", "removeLayer").build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn declaring_utilities_injects_set_active_utility_action_and_keybinding() {
            use semio_framework::{ActionKind, UtilityDefinition, SET_ACTIVE_UTILITY_ACTION_ID};
            let definition = minimal_app("utility-app")
                .utility(UtilityDefinition { keys: Some("b".into()), ..UtilityDefinition::new("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush) })
                .utility_simple("eraser", LocalizedLabel::data("Eraser"), IconName::Eraser)
                .build_definition();
            let set_active_utility = definition.actions.iter().find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility injected");
            assert_eq!(set_active_utility.kind, ActionKind::View);
            assert!(!set_active_utility.in_palette);
            let binding = definition.keybindings.iter().find(|binding| binding.keys == "b").expect("utility keybinding auto-injected");
            assert_eq!(binding.action.action, SET_ACTIVE_UTILITY_ACTION_ID);
            assert_eq!(binding.action.args, Some(DslValue::Object(vec![("utilityId".into(), DslValue::String("brush".into()))])));
        }

        #[test]
        fn no_utilities_means_no_set_active_utility_action() {
            use semio_framework::SET_ACTIVE_UTILITY_ACTION_ID;
            let definition = minimal_app("no-utility-app").build_definition();
            assert!(!definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID));
        }

        #[test]
        fn build_definition_accepts_and_resolves_mode_tools() {
            use semio_framework::ToolRef;
            let definition = minimal_app("tool-app").tool_simple("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket).mode_tools("edit", vec![ToolRef::new("fill")]).build_definition();
            assert_eq!(definition.tools.len(), 1);
            assert_eq!(definition.modes[0].tools, vec![ToolRef::new("fill")]);
        }

        #[test]
        fn build_definition_rejects_mode_tool_ref_to_undeclared_tool() {
            use semio_framework::ToolRef;
            let result = std::panic::catch_unwind(|| minimal_app("undeclared-mode-tool-app").mode_tools("edit", vec![ToolRef::new("missing")]).build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_tool_referenced_by_no_mode() {
            let result = std::panic::catch_unwind(|| minimal_app("orphan-tool-app").tool_simple("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket).build_definition());
            assert!(result.is_err(), "a declared tool must be referenced by mode_tools on at least one mode");
        }

        #[test]
        fn declaring_tools_injects_set_active_tool_action_and_keybinding() {
            use semio_framework::{ActionKind, ToolDefinition, ToolRef, SET_ACTIVE_TOOL_ACTION_ID};
            let definition =
                minimal_app("tool-keybinding-app").tool(ToolDefinition { keys: Some("f".into()), ..ToolDefinition::new("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket) }).mode_tools("edit", vec![ToolRef::new("fill")]).build_definition();
            let set_active_tool = definition.actions.iter().find(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID).expect("setActiveTool injected");
            assert_eq!(set_active_tool.kind, ActionKind::View);
            assert!(!set_active_tool.in_palette);
            let binding = definition.keybindings.iter().find(|binding| binding.keys == "f").expect("tool keybinding auto-injected");
            assert_eq!(binding.action.action, SET_ACTIVE_TOOL_ACTION_ID);
            assert_eq!(binding.action.args, Some(DslValue::Object(vec![("toolId".into(), DslValue::String("fill".into()))])));
        }

        #[test]
        fn no_tools_means_no_set_active_tool_action() {
            use semio_framework::SET_ACTIVE_TOOL_ACTION_ID;
            let definition = minimal_app("no-tool-app").build_definition();
            assert!(!definition.actions.iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID));
        }

        #[test]
        fn action_args_attaches_declared_arguments() {
            let definition = minimal_app("args-app").mutation("resize", LocalizedLabel::data("Resize")).action_args("resize", vec![ActionArgDef::slider("scale", LocalizedLabel::data("Scale"), 0.0, 4.0).required()]).build_definition();
            let resize = definition.actions.iter().find(|action| action.id == "resize").expect("declared");
            assert_eq!(resize.args.len(), 1);
            assert_eq!(resize.args[0].id, "scale");
            assert!(resize.args[0].required);
        }

        #[test]
        fn build_definition_rejects_window_kind_utility_referencing_undeclared_utility() {
            let result = std::panic::catch_unwind(|| minimal_app("bad-utility-ref-app").utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush).window_kind_utilities("main", vec!["missing".into()]).build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_window_kind_action_referencing_undeclared_action() {
            let result = std::panic::catch_unwind(|| minimal_app("bad-action-ref-app").mutation("addLayer", LocalizedLabel::data("Add Layer")).window_kind_actions("main", vec!["removeLayer".into()]).build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_select_arg_with_no_options() {
            use semio_framework::{ActionArgControl, ActionArgDef};
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-select-app")
                    .mutation("pick", LocalizedLabel::data("Pick"))
                    .action_args("pick", vec![ActionArgDef { control: ActionArgControl::Select { options: vec![] }, ..ActionArgDef::text("choice", LocalizedLabel::data("Choice")) }])
                    .build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn declaring_introduction_injects_start_introduction_action() {
            use semio_framework::{ActionKind, IntroductionDefinition, IntroductionStepDefinition, START_INTRODUCTION_ACTION_ID};
            use ui_wgpu::wgpu::LocalizedLabel;
            let definition = minimal_app("intro-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("welcome", LocalizedLabel::data("Welcome"), LocalizedLabel::data("Hi there"))] }).build_definition();
            let start_introduction = definition.actions.iter().find(|action| action.id == START_INTRODUCTION_ACTION_ID).expect("startIntroduction injected");
            assert_eq!(start_introduction.kind, ActionKind::View);
            assert!(!start_introduction.in_palette, "the shell-owned Introduce App command owns palette discovery");
        }

        #[test]
        fn no_introduction_means_no_start_introduction_action() {
            use semio_framework::START_INTRODUCTION_ACTION_ID;
            let definition = minimal_app("no-intro-app").build_definition();
            assert!(!definition.actions.iter().any(|action| action.id == START_INTRODUCTION_ACTION_ID));
        }

        #[test]
        fn build_definition_rejects_introduction_with_no_steps() {
            use semio_framework::IntroductionDefinition;
            let result = std::panic::catch_unwind(|| minimal_app("empty-intro-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![] }).build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_duplicate_introduction_step_ids() {
            use semio_framework::{IntroductionDefinition, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let result = std::panic::catch_unwind(|| {
                minimal_app("dupe-step-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")), IntroductionStepDefinition::new("step", LocalizedLabel::data("B"), LocalizedLabel::data("b"))] }).build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_introduction_step_introducing_undeclared_window_kind() {
            use semio_framework::{window_element_id, IntroductionDefinition, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-window-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(window_element_id("missing"))] }).build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_introduction_step_introducing_undeclared_panel_tab() {
            use semio_framework::{panel_tab_element_id, panel_tab_first_draggable_element_id, IntroductionDefinition, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-panel-tab-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(panel_tab_element_id("missing"))] }).build_definition()
            });
            assert!(result.is_err());
            let result_first_draggable = std::panic::catch_unwind(|| {
                minimal_app("bad-panel-tab-first-draggable-app")
                    .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(panel_tab_first_draggable_element_id("missing"))] })
                    .build_definition()
            });
            assert!(result_first_draggable.is_err());
        }

        #[test]
        fn build_definition_rejects_introduction_step_targeting_malformed_element_id() {
            use semio_framework::{IntroductionDefinition, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-element-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce("not-camel-case")] }).build_definition()
            });
            assert!(result.is_err());
            let result_show = std::panic::catch_unwind(|| {
                minimal_app("bad-element-show-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).show(vec!["not-camel-case".into()])] }).build_definition()
            });
            assert!(result_show.is_err());
        }

        #[test]
        fn build_definition_accepts_introduction_step_introducing_escape_hatch_element_id() {
            use semio_framework::{IntroductionDefinition, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let definition = minimal_app("good-escape-hatch-app").introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce("ui.custom.thing")] }).build_definition();
            let introduction = definition.introduction.expect("introduction present");
            assert_eq!(introduction.steps.len(), 1);
        }

        #[test]
        fn build_definition_rejects_introduction_step_interacting_on_undeclared_utility() {
            use semio_framework::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-interaction-utility-app")
                    .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).interact(vec![IntroductionInteraction::utility("missing", "Activate")])] })
                    .build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_introduction_step_interacting_on_undeclared_window_kind() {
            use semio_framework::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-interaction-window-app")
                    .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).interact(vec![IntroductionInteraction::orbit("missing", "Orbit")])] })
                    .build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_accepts_introduction_with_declared_window_utility_and_action_targets() {
            use semio_framework::{window_element_id, IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
            use ui_wgpu::wgpu::LocalizedLabel;
            let definition = minimal_app("good-intro-app")
                .mutation("addLayer", LocalizedLabel::data("Add Layer"))
                .utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush)
                .window_kind_utilities("main", vec!["brush".into()])
                .window_kind_actions("main", vec!["addLayer".into()])
                .introduction(IntroductionDefinition {
                    title: LocalizedLabel::data("Welcome"),
                    steps: vec![
                        IntroductionStepDefinition::new("welcome", LocalizedLabel::data("Welcome"), LocalizedLabel::data("Hi")),
                        IntroductionStepDefinition::new("main-window", LocalizedLabel::data("Main Window"), LocalizedLabel::data("…")).introduce(window_element_id("main")),
                        IntroductionStepDefinition::new("brush-utility", LocalizedLabel::data("Brush"), LocalizedLabel::data("…")).introduce("brush").interact(vec![IntroductionInteraction::utility("brush", "Activate Brush")]),
                        IntroductionStepDefinition::new("add-layer", LocalizedLabel::data("Add Layer"), LocalizedLabel::data("…")).interact(vec![IntroductionInteraction::action("addLayer", "Add a Layer")]),
                        IntroductionStepDefinition::new("navigate-main", LocalizedLabel::data("Navigate"), LocalizedLabel::data("…")).interact(vec![IntroductionInteraction::pan("main", "Pan"), IntroductionInteraction::zoom("main", "Zoom")]),
                    ],
                })
                .build_definition();
            let introduction = definition.introduction.expect("introduction present");
            assert_eq!(introduction.steps.len(), 5);
        }

        fn minimal_tutorial(id: &str) -> semio_framework::TutorialDefinition {
            use semio_framework::{TutorialBase, TutorialDefinition, TutorialTracks, TutorialUiSnapshot};
            TutorialDefinition {
                id: id.into(),
                title: LocalizedLabel::data("Tutorial"),
                description: None,
                duration_ms: 10_000,
                chapters: vec![],
                base: TutorialBase { artifact_dsl: None, example_id: None, ui: TutorialUiSnapshot::default(), cameras: vec![] },
                tracks: TutorialTracks::default(),
                recorded_at: None,
            }
        }

        #[test]
        fn declaring_tutorial_injects_start_tutorial_action() {
            use semio_framework::{ActionKind, START_TUTORIAL_ACTION_ID};
            let definition = minimal_app("tutorial-app").tutorial(minimal_tutorial("welcome-tour")).build_definition();
            let start_tutorial = definition.actions.iter().find(|action| action.id == START_TUTORIAL_ACTION_ID).expect("startTutorial injected");
            assert_eq!(start_tutorial.kind, ActionKind::View);
            assert!(!start_tutorial.in_palette, "the shell-owned Play Tutorial command owns palette discovery");
        }

        #[test]
        fn no_tutorial_means_no_start_tutorial_action_but_record_is_always_injected() {
            use semio_framework::{RECORD_TUTORIAL_ACTION_ID, START_TUTORIAL_ACTION_ID};
            let definition = minimal_app("no-tutorial-app").build_definition();
            assert!(!definition.actions.iter().any(|action| action.id == START_TUTORIAL_ACTION_ID));
            assert!(definition.actions.iter().any(|action| action.id == RECORD_TUTORIAL_ACTION_ID), "recordTutorial is injected unconditionally — recording needs no app declaration");
        }

        #[test]
        fn build_definition_rejects_tutorial_failing_structural_validation() {
            let result = std::panic::catch_unwind(|| {
                let mut tutorial = minimal_tutorial("out-of-range-tour");
                tutorial.duration_ms = 100;
                tutorial.chapters.push(semio_framework::TutorialChapter { id: "late".into(), at: 999_999, title: LocalizedLabel::data("Late"), body: None });
                minimal_app("bad-structural-tutorial-app").tutorial(tutorial).build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_duplicate_tutorial_ids() {
            let result = std::panic::catch_unwind(|| minimal_app("dupe-tutorial-app").tutorial(minimal_tutorial("tour")).tutorial(minimal_tutorial("tour")).build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_tutorial_event_referencing_undeclared_action() {
            use semio_framework::{TutorialEvent, TutorialEventKind};
            let result = std::panic::catch_unwind(|| {
                let mut tutorial = minimal_tutorial("bad-event-tour");
                tutorial.tracks.events = vec![TutorialEvent { at: 10, kind: TutorialEventKind::Action { action: "missingAction".into(), args: None } }];
                minimal_app("bad-tutorial-event-app").tutorial(tutorial).build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_tutorial_ui_change_referencing_undeclared_utility() {
            use semio_framework::{TutorialUiChange, TutorialUiKeyframe, TutorialUiSample};
            let result = std::panic::catch_unwind(|| {
                let mut tutorial = minimal_tutorial("bad-ui-change-tour");
                tutorial.tracks.ui = vec![TutorialUiKeyframe { at: 10, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveUtility { window_id: "main".into(), utility_id: Some("missing".into()) }] } }];
                minimal_app("bad-tutorial-ui-app").tutorial(tutorial).build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_tutorial_gesture_targeting_malformed_element_id() {
            use semio_framework::{IntroductionGesture, IntroductionPoint, TutorialGestureCue};
            let result = std::panic::catch_unwind(|| {
                let mut tutorial = minimal_tutorial("bad-gesture-tour");
                tutorial.tracks.gestures = vec![TutorialGestureCue { at: 10, duration_ms: 200, gesture: IntroductionGesture::LeftClick { at: IntroductionPoint::Element { id: "not-camel-case".into(), offset: None } }, cursor: None }];
                minimal_app("bad-tutorial-gesture-app").tutorial(tutorial).build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_accepts_tutorial_with_declared_action_utility_and_gesture_targets() {
            use semio_framework::{window_element_id, IntroductionGesture, IntroductionPoint, TutorialEvent, TutorialEventKind, TutorialGestureCue, TutorialUiChange, TutorialUiKeyframe, TutorialUiSample};
            let mut tutorial = minimal_tutorial("good-tour");
            tutorial.tracks.events = vec![TutorialEvent { at: 10, kind: TutorialEventKind::Action { action: "addLayer".into(), args: None } }];
            tutorial.tracks.ui = vec![TutorialUiKeyframe { at: 20, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveUtility { window_id: "main".into(), utility_id: Some("brush".into()) }] } }];
            tutorial.tracks.gestures = vec![TutorialGestureCue { at: 30, duration_ms: 200, gesture: IntroductionGesture::LeftClick { at: IntroductionPoint::Element { id: window_element_id("main"), offset: None } }, cursor: None }];
            let definition = minimal_app("good-tutorial-app").mutation("addLayer", LocalizedLabel::data("Add Layer")).utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush).tutorial(tutorial).build_definition();
            assert_eq!(definition.tutorials.len(), 1);
            assert_eq!(definition.tutorials[0].id, "good-tour");
        }

        #[test]
        fn declaring_dialog_appends_to_definition() {
            use semio_framework::{ActionRef, DialogDefinition};
            let definition = minimal_app("dialog-app").mutation("addLayer", LocalizedLabel::data("Add Layer")).dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer"))).build_definition();
            assert_eq!(definition.dialogs.len(), 1);
            assert_eq!(definition.dialogs[0].id, "addLayer");
            assert_eq!(definition.dialogs[0].submit_label, LocalizedLabel::data("OK"));
        }

        #[test]
        fn build_definition_rejects_duplicate_dialog_ids() {
            use semio_framework::{ActionRef, DialogDefinition};
            let result = std::panic::catch_unwind(|| {
                minimal_app("dupe-dialog-app")
                    .mutation("addLayer", LocalizedLabel::data("Add Layer"))
                    .dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer")))
                    .dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer Again"), ActionRef::new("addLayer")))
                    .build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_dialog_submit_action_referencing_undeclared_action() {
            use semio_framework::{ActionRef, DialogDefinition};
            let result = std::panic::catch_unwind(|| minimal_app("bad-dialog-submit-app").dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("missing"))).build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_dialog_cancel_action_referencing_undeclared_action() {
            use semio_framework::{ActionRef, DialogDefinition};
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-dialog-cancel-app").mutation("addLayer", LocalizedLabel::data("Add Layer")).dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer")).on_cancel(ActionRef::new("missing"))).build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn dialog_submit_action_may_reference_an_injected_history_action() {
            use semio_framework::{ActionRef, DialogDefinition};
            let definition = minimal_app("dialog-injected-action-app").dialog(DialogDefinition::new("confirmUndo", LocalizedLabel::data("Undo?"), ActionRef::new("undo"))).build_definition();
            assert_eq!(definition.dialogs[0].submit_action, ActionRef::new("undo"));
        }

        #[test]
        fn build_definition_rejects_dialog_duplicate_arg_ids() {
            use semio_framework::{ActionArgDef, ActionRef, DialogDefinition};
            let result = std::panic::catch_unwind(|| {
                minimal_app("dupe-dialog-arg-app")
                    .mutation("addLayer", LocalizedLabel::data("Add Layer"))
                    .dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer")).args(vec![ActionArgDef::text("name", LocalizedLabel::data("Name")), ActionArgDef::text("name", LocalizedLabel::data("Name Again"))]))
                    .build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_dialog_select_arg_with_no_options() {
            use semio_framework::{ActionArgControl, ActionArgDef, ActionRef, DialogDefinition};
            let result = std::panic::catch_unwind(|| {
                minimal_app("bad-dialog-select-app")
                    .mutation("addLayer", LocalizedLabel::data("Add Layer"))
                    .dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer")).args(vec![ActionArgDef { control: ActionArgControl::Select { options: vec![] }, ..ActionArgDef::text("kind", LocalizedLabel::data("Kind")) }]))
                    .build_definition()
            });
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_accepts_app_and_mode_scope_commands() {
            use semio_framework::{CommandDefinition, CommandRef, CommandScope};
            let definition = minimal_app("command-app")
                .app_command("app.export", LocalizedLabel::data("Export"), "document")
                .command(CommandDefinition::new_catalog("mode.focus", LocalizedLabel::data("Focus"), CommandScope::Mode, "view"))
                .mode_commands("edit", vec![CommandRef::new("mode.focus")])
                .build_definition();
            assert_eq!(definition.commands.len(), 2);
            assert_eq!(definition.modes[0].commands, vec![CommandRef::new("mode.focus")]);
        }

        #[test]
        fn build_definition_rejects_duplicate_command_ids() {
            let result = std::panic::catch_unwind(|| minimal_app("dupe-command-app").app_command("app.export", LocalizedLabel::data("Export"), "document").app_command("app.export", LocalizedLabel::data("Export Again"), "document").build_definition());
            assert!(result.is_err());
        }

        #[test]
        fn build_definition_rejects_os_or_plugin_scope_command() {
            use semio_framework::{CommandDefinition, CommandScope};
            let result = std::panic::catch_unwind(|| minimal_app("os-scope-command-app").command(CommandDefinition::new_catalog("os.theme", LocalizedLabel::data("Theme"), CommandScope::Os, "appearance")).build_definition());
            assert!(result.is_err(), "AppBuilder must reject Os/Plugin-scope commands — those are declared by the shell/Plugin, not an app");
        }

        #[test]
        fn build_definition_rejects_mode_command_ref_to_undeclared_or_wrong_scope_command() {
            use semio_framework::CommandRef;
            let undeclared = std::panic::catch_unwind(|| minimal_app("undeclared-mode-command-app").mode_commands("edit", vec![CommandRef::new("nope")]).build_definition());
            assert!(undeclared.is_err());

            let wrong_scope = std::panic::catch_unwind(|| minimal_app("wrong-scope-mode-command-app").app_command("app.export", LocalizedLabel::data("Export"), "document").mode_commands("edit", vec![CommandRef::new("app.export")]).build_definition());
            assert!(wrong_scope.is_err(), "an App-scope command must not be referenceable from a mode's commands list");
        }

        #[test]
        fn build_definition_rejects_mode_scope_command_referenced_by_no_mode() {
            use semio_framework::{CommandDefinition, CommandScope};
            let result = std::panic::catch_unwind(|| minimal_app("orphan-mode-command-app").command(CommandDefinition::new_catalog("mode.focus", LocalizedLabel::data("Focus"), CommandScope::Mode, "view")).build_definition());
            assert!(result.is_err());
        }
    }

    //#region 📚️ExampleSource
    /// 📚️ Value type exported by an example definition leaf (`📚️examples/<slug>/🦀️component.rs`):
    /// stable id, localized label, icon, and document/payload accessors — converts into
    /// [`ExampleDefinition`] for [`PluginManifest::examples`].
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExampleSource {
        id: String,
        label: LocalizedLabel,
        icon_id: IconName,
        document_json: String,
    }

    impl ExampleSource {
        /// 🧱 Builds an example source from id, label, document payload, and icon.
        pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, document_json: impl Into<String>, icon_id: impl Into<IconName>) -> Self {
            Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), document_json: document_json.into() }
        }

        /// 🏷️ Stable example id (navbar picker / `setActiveExample`).
        pub fn id(&self) -> &str {
            &self.id
        }

        /// 🗣️ Localized label sourced from the definition leaf.
        pub fn label(&self) -> &LocalizedLabel {
            &self.label
        }

        /// 🖼️ Icon id sourced from the definition leaf.
        pub fn icon_id(&self) -> IconName {
            self.icon_id
        }

        /// 📄️ Document JSON payload registered on the manifest.
        pub fn document_json(&self) -> &str {
            &self.document_json
        }

        /// 📄️ Alias of [`Self::document_json`] for leaf call sites that speak in document terms.
        pub fn document(&self) -> &str {
            &self.document_json
        }

        /// 📒️ Alias of [`Self::document_json`] for leaf call sites that speak in payload terms.
        pub fn payload(&self) -> &str {
            &self.document_json
        }

        /// 🧬️ Converts into a manifest [`ExampleDefinition`] (`app_id` filled at plugin registration).
        pub fn into_example_definition(self) -> ExampleDefinition {
            ExampleDefinition { id: self.id, label: self.label, icon_id: self.icon_id, artifact_json: self.document_json, app_id: String::new() }
        }
    }

    impl From<ExampleSource> for ExampleDefinition {
        fn from(source: ExampleSource) -> Self {
            source.into_example_definition()
        }
    }

    impl From<&ExampleSource> for ExampleDefinition {
        fn from(source: &ExampleSource) -> Self {
            ExampleDefinition {
                id: source.id.clone(),
                label: source.label.clone(),
                icon_id: source.icon_id,
                artifact_json: source.document_json.clone(),
                app_id: String::new(),
            }
        }
    }

    impl From<&ExampleSource> for ExampleSource {
        fn from(source: &ExampleSource) -> Self {
            source.clone()
        }
    }

    #[cfg(test)]
    mod example_source_tests {
        use super::*;

        #[test]
        fn example_source_converts_into_example_definition_and_registers_on_app() {
            let source = ExampleSource::new("nakagin", LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm"), "{\"kind\":\"demo\"}", "building");
            assert_eq!(source.id(), "nakagin");
            assert_eq!(source.document(), "{\"kind\":\"demo\"}");
            assert_eq!(source.payload(), source.document_json());
            let definition = ExampleDefinition::from(&source);
            assert_eq!(definition.id, "nakagin");
            assert_eq!(definition.artifact_json, "{\"kind\":\"demo\"}");
            assert!(definition.app_id.is_empty());
            let app = App::from_builder(App::builder("puzzle2d-play", LocalizedLabel::data("Puzzle")).document(["semio", "puzzle"]).mode("edit", LocalizedLabel::data("Edit"), "pencil").window_kind("main", LocalizedLabel::data("Main"), "puzzle.main", SurfaceKind::Canvas2d, IconName::AppWindow)).example_source(&source);
            assert_eq!(app.examples.len(), 1);
            assert_eq!(app.examples[0].id, "nakagin");
            assert_eq!(app.examples[0].icon_id, IconName::from("building"));
        }

        #[test]
        fn example_delegates_to_example_source() {
            let app = App::from_builder(App::builder("demo-play", LocalizedLabel::data("Demo")).document(["semio", "demo"]).mode("edit", LocalizedLabel::data("Edit"), "pencil").window_kind("main", LocalizedLabel::data("Main"), "demo.main", SurfaceKind::Canvas2d, IconName::AppWindow)).example("default", LocalizedLabel::native("Default", "Standard"), "{}", "file");
            assert_eq!(app.examples.len(), 1);
            assert_eq!(app.examples[0].id, "default");
        }
    }
    //#endregion 📚️ExampleSource

    pub struct App {
        pub definition: AppDefinition,
        pub examples: Vec<ExampleDefinition>,
    }

    impl App {
        pub fn builder(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> AppBuilder {
            AppBuilder::new(id, label)
        }

        /// 🗂️ Resource kinds declared via `.artifact_kind(...)` — see `AppDefinition.artifact_kinds`
        /// (round-trips through the plugin manifest; `semio_framework_os`'s artifact catalog registry
        /// consumes it from there at plugin registration time).
        pub fn from_builder(builder: AppBuilder) -> Self {
            Self { definition: builder.build_definition(), examples: Vec::new() }
        }

        /// 📚️ Registers an example from a definition-leaf [`ExampleSource`] (canonical path).
        pub fn example_source(mut self, source: impl Into<ExampleSource>) -> Self {
            self.examples.push(ExampleDefinition::from(source.into()));
            self
        }

        /// 📚️ Registers an example by restating id/label/payload/icon.
        /// Prefer [`Self::example_source`] once the definition leaf owns those fields; kept so
        /// existing `create_*_app()` call sites keep compiling during the example-shape migration.
        pub fn example(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, document_json: impl Into<String>, icon_id: impl Into<IconName>) -> Self {
            self.example_source(ExampleSource::new(id, label, document_json, icon_id))
        }

        /// 🚧️ B1 stub: `WorkflowDefinition` + `PluginManifest.workflows` were deleted from framework-core
        /// (WP-0.1, concurrent) — real workflow-palette derivation moves to `register_app_io`/
        /// `workflow_palette()` in Wave 1 (`AppIo`-driven, not this free-text `yields`). Kept as a no-op
        /// so the ~50 existing `.workflow(...)` call sites across every app manifest keep compiling
        /// unchanged until their own wave touches them.
        pub fn workflow(self, _workflow_step_id: impl Into<String>, _label: impl Into<String>, _yields: impl Into<String>) -> Self {
            self
        }
    }

    //#region 🔖️DocumentContract
    /// @emoji 🧾️ Read-only view of an app's document handed to `ArtifactApp::handle_action`/`render`:
    /// the materialized snapshot plus the history metadata (checkpoints/alternatives/undo state)
    /// derived from the owning {@link VcsArtifactApp}'s persistent {@link ArtifactStore}.
    pub struct ArtifactView<'a, P> {
        pub snapshot: &'a P,
        pub history: &'a HistoryView,
        /// 🧸️ Read access to this document's OWNED CHILDREN, each of which is its own envelope with
        /// its own `ArtifactVcs` history (never an inline subtree). See {@link ChildContentView}.
        pub children: ChildContentView<'a>,
    }

    impl<'a, P> ArtifactView<'a, P> {
        /// 🏗️ A view over a document with no composed children — the overwhelming majority, and the
        /// shape every leaf app and test uses. Prefer this over a struct literal: lane views grow
        /// over time, and a constructor absorbs that growth without touching every call site.
        pub fn new(snapshot: &'a P, history: &'a HistoryView) -> Self {
            Self { snapshot, history, children: ChildContentView::EMPTY }
        }

        /// 🏗️ A view over a composing document, wired to its live child stores.
        pub fn with_children(snapshot: &'a P, history: &'a HistoryView, children: ChildContentView<'a>) -> Self {
            Self { snapshot, history, children }
        }
    }

    /// @emoji 🧸️ Read-only access to a composing document's live child stores, keyed `(slot,
    /// child_id)` exactly as the parent's `ArtifactChild` handles name them.
    ///
    /// This is the seam that replaces the `thread_local!`/session `HashMap<child_id, content>`
    /// caches every composed plugin used to carry. Those caches went STALE the moment anything
    /// moved a child's history without going through `ArtifactApp::handle` — store-level undo/redo
    /// and checkout do exactly that. Reading straight through the live `SpaceMember` cannot go
    /// stale by construction: there is no second copy to fall out of date. That is a stronger
    /// guarantee than the fail-closed staleness checks it supersedes, which could only DETECT the
    /// divergence after it happened.
    ///
    /// `Copy` + `Default` (`EMPTY`) so a leaf app's view costs nothing and needs no ceremony.
    #[derive(Clone, Copy, Default)]
    pub struct ChildContentView<'a> {
        children: Option<&'a HashMap<(String, String), (store::os_io::ArtifactDialect, Box<dyn SpaceMember>)>>,
    }

    impl<'a> ChildContentView<'a> {
        /// 🈳️ The view a document with no children gets.
        pub const EMPTY: Self = Self { children: None };

        /// 🏗️ Wraps a live child-store map (built by {@link VcsArtifactApp} before each dispatch).
        pub fn new(children: &'a HashMap<(String, String), (store::os_io::ArtifactDialect, Box<dyn SpaceMember>)>) -> Self {
            Self { children: Some(children) }
        }

        /// 📦️ The child's CURRENT content, pack-encoded — reads through the live store, so it always
        /// reflects the child's present state including any undo/redo/checkout it has undergone.
        pub fn pack(&self, slot: &str, child_id: &str) -> Result<Vec<u8>, Fault> {
            let member = self.children.and_then(|children| children.get(&(slot.to_string(), child_id.to_string()))).ok_or_else(|| plugin_sdk_fault(format!("no live child store for slot {slot} child {child_id}")))?;
            member.1.document_pack_bytes().map_err(|error| plugin_sdk_fault(error.to_string()))
        }

        /// 🧩️ {@link Self::pack} decoded as the child's snapshot type — what a composing app's
        /// `handle`/`render` actually calls.
        pub fn typed<S: ArtifactPack>(&self, slot: &str, child_id: &str) -> Result<S, Fault> {
            S::decode_pack(&self.pack(slot, child_id)?).map_err(|error| plugin_sdk_fault(error.to_string()))
        }

        /// 🎯️ The dialect a child materializes as, for a caller that must route by kind.
        pub fn dialect(&self, slot: &str, child_id: &str) -> Option<store::os_io::ArtifactDialect> {
            self.children.and_then(|children| children.get(&(slot.to_string(), child_id.to_string()))).map(|(dialect, _)| dialect.clone())
        }

        /// 📋️ Every `(slot, child_id)` currently live under this document.
        pub fn slots(&self) -> Vec<(String, String)> {
            self.children.map(|children| children.keys().cloned().collect()).unwrap_or_default()
        }

        /// 🈳️ Whether this document has any live children at all.
        pub fn is_empty(&self) -> bool {
            self.children.map(HashMap::is_empty).unwrap_or(true)
        }
    }

    /// @emoji 🧮️ Read-only view of an app's config snapshot — same role as {@link ArtifactView} for the
    /// config {@link ConfigStore} owned by {@link VcsArtifactApp}.
    pub struct ConfigView<'a, C> {
        pub snapshot: &'a C,
    }

    /// @emoji 📝️ Read-only view of an app's volatile draft snapshot — same role as {@link ConfigView}
    /// for the draft {@link store::DraftStore} (ephemeral; never checkpoints).
    pub struct DraftView<'a, D> {
        pub snapshot: &'a D,
    }

    /// @emoji 👥️ Read-only view of the PRESENCE lane: this actor's own live shared state plus every
    /// peer's, as last broadcast. Ephemeral and shared — never persisted, never undoable.
    pub struct PresenceView<'a, P> {
        /// 👤️ This actor's own presence — the value that gets broadcast.
        pub local: &'a P,
        /// 👥️ Every other peer's presence, sorted by actor id for a stable render order.
        pub peers: Vec<(&'a str, &'a P)>,
    }

    /// @emoji 🫧️ Read-only view of the TRANSIENT lane: ephemeral state local to this client that is
    /// never document content — the typed replacement for plugin `thread_local!` scratch state.
    pub struct TransientView<'a, T> {
        pub snapshot: &'a T,
    }

    //#region 🔖️NoConfig
    /// @emoji 🧮️ Default `ArtifactApp::Config` for apps with no config artifact yet.
    #[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NoConfig {}

    impl store::ArtifactDsl for NoConfig {
        const EXTENSION: &'static str = "nocfg";
        fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
            if text.trim().is_empty() {
                return Ok(Self::default());
            }
            Err(store::TextError::new("no config", store::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            String::new()
        }
    }

    impl ArtifactPack for NoConfig {
        fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
            Ok(Vec::new())
        }
        fn decode_pack_with(_bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
            Ok(Self::default())
        }
    }

    impl store::ConfigRecord for NoConfig {}

    impl ::protocol::MutationDiff<NoConfig> for NoConfig {
        fn apply(&self, base: &NoConfig) -> NoConfig {
            base.clone()
        }
        fn absorb(&mut self, _other: Self) {}
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
    #[serde(rename_all = "camelCase")]
    pub enum NoConfigMutation {
        Noop,
    }

    impl ::protocol::Mutation<NoConfig> for NoConfigMutation {
        type Diff = NoConfig;

        fn diff(&self, _base: &NoConfig) -> NoConfig {
            NoConfig::default()
        }

        fn inverse(&self, _base: &NoConfig) -> Vec<Self> {
            vec![NoConfigMutation::Noop]
        }
    }

    impl ::protocol::OpText for NoConfigMutation {
        fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
            let variants = <Self as ::dsl::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = ::dsl::parse(
                        body,
                        &spec_fn(),
                        &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                    )?;
                    return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
            let variants = <Self as ::dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
            if body.is_empty() {
                keyword
            } else {
                format!("{keyword} {body}")
            }
        }
    }

    impl ::protocol::OpBinary for NoConfigMutation {
        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }
    //#endregion 🔖️NoConfig

    //#region 🔖️NoDraft
    /// @emoji 📝️ Default `ArtifactApp::Draft` for apps with no draft lane yet.
    pub type NoDraft = NoConfig;
    /// @emoji 📝️ Default `ArtifactApp::DraftMutation` twin of {@link NoDraft}.
    pub type NoDraftMutation = NoConfigMutation;
    //#endregion 🔖️NoDraft

    //#region 🔖️NoPresence
    /// @emoji 👥️ Default `ArtifactApp::Presence` for apps with no shareable live state yet.
    #[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NoPresence {}

    impl store::ArtifactDsl for NoPresence {
        const EXTENSION: &'static str = "nopres";
        fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
            if text.trim().is_empty() {
                return Ok(Self::default());
            }
            Err(store::TextError::new("no presence", store::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            String::new()
        }
    }

    impl ArtifactPack for NoPresence {
        fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
            Ok(Vec::new())
        }
        fn decode_pack_with(_bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
            Ok(Self::default())
        }
    }

    impl ::protocol::MutationDiff<NoPresence> for NoPresence {
        fn apply(&self, base: &NoPresence) -> NoPresence {
            base.clone()
        }
        fn absorb(&mut self, _other: Self) {}
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
    #[serde(rename_all = "camelCase")]
    pub enum NoPresenceMutation {
        Noop,
    }

    impl ::protocol::Mutation<NoPresence> for NoPresenceMutation {
        type Diff = NoPresence;

        fn diff(&self, _base: &NoPresence) -> NoPresence {
            NoPresence::default()
        }

        fn inverse(&self, _base: &NoPresence) -> Vec<Self> {
            vec![NoPresenceMutation::Noop]
        }
    }

    impl ::protocol::OpText for NoPresenceMutation {
        fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
            let variants = <Self as ::dsl::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = ::dsl::parse(
                        body,
                        &spec_fn(),
                        &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                    )?;
                    return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
            let variants = <Self as ::dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
            if body.is_empty() {
                keyword
            } else {
                format!("{keyword} {body}")
            }
        }
    }

    impl ::protocol::OpBinary for NoPresenceMutation {
        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }
    //#endregion 🔖️NoPresence

    //#region 🔖️NoTransient
    /// @emoji 🫧️ Default `ArtifactApp::Transient` for apps with no ephemeral local UI state yet.
    ///
    /// 🎯️ The FOURTH and last state mechanism. The four are exhaustive and mutually exclusive:
    /// **artifact** = persisted + shared, **config** = persisted + local-only, **presence** =
    /// ephemeral + shared, **transient** = ephemeral + local-only. Anything that used to live in a
    /// plugin `thread_local!`, a process-local ephemeral box, or an untyped shell field belongs
    /// here — typed, dispatched through `Emit`, and readable through `Lanes`, exactly like the other
    /// three, rather than reachable from anywhere at any time.
    ///
    /// ⚠️ Transient is NOT the draft lane. A DRAFT is an ephemeral *artifact* — real document
    /// content that simply has not been committed. TRANSIENT is UI state that is never document
    /// content at all (which pane is focused, what is hovered, an in-flight gesture). They differ in
    /// what the state IS, not in how long it lives.
    #[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NoTransient {}

    impl store::ArtifactDsl for NoTransient {
        const EXTENSION: &'static str = "notrans";
        fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
            if text.trim().is_empty() {
                return Ok(Self::default());
            }
            Err(store::TextError::new("no transient", store::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            String::new()
        }
    }

    impl ArtifactPack for NoTransient {
        fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
            Ok(Vec::new())
        }
        fn decode_pack_with(_bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
            Ok(Self::default())
        }
    }

    impl ::protocol::MutationDiff<NoTransient> for NoTransient {
        fn apply(&self, base: &NoTransient) -> NoTransient {
            base.clone()
        }
        fn absorb(&mut self, _other: Self) {}
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
    #[serde(rename_all = "camelCase")]
    pub enum NoTransientMutation {
        Noop,
    }

    impl ::protocol::Mutation<NoTransient> for NoTransientMutation {
        type Diff = NoTransient;

        fn diff(&self, _base: &NoTransient) -> NoTransient {
            NoTransient::default()
        }

        fn inverse(&self, _base: &NoTransient) -> Vec<Self> {
            vec![NoTransientMutation::Noop]
        }
    }

    impl ::protocol::OpText for NoTransientMutation {
        fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
            let variants = <Self as ::dsl::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;
                    return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
            let variants = <Self as ::dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
            if body.is_empty() {
                keyword
            } else {
                format!("{keyword} {body}")
            }
        }
    }

    impl ::protocol::OpBinary for NoTransientMutation {
        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }
    //#endregion 🔖️NoTransient

    //#region 🔖️CommandLog
    /// @emoji 🎚️ Tri-state operations filter of the framework history panel — `All` is the default.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum HistoryCommandFilter {
        #[default]
        All,
        WithoutMutations,
        OnlyMutations,
    }

    /// @emoji ⏪️ A stored, replayable inverse for a `View`/`Shell`-kind command — the memory-only
    /// counterpart to a VCS edit's `Mutation::inverse`. `action_id` is a plugin action id (`View` rows —
    /// replayed locally via `dispatch_action`) or a shell command id (`Shell` rows — bubbled out as
    /// `HostEffect::ReplayShellCommand` since the plugin has no access to shell-owned state). Never
    /// persisted: it lives only on the in-memory `CommandLogEntry`/`CommandView`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct InverseAction {
        pub action_id: String,
        pub args: Option<Value>,
    }

    /// @emoji 🧾️ One appended session-command record — runtime-only, never persisted (the VCS envelope
    /// is the persisted half; see `CommandView::op_lines`, derived live from `envelope.vcs.edits`).
    /// Append-only: `VcsArtifactApp` only ever pushes to `command_log`, including for undo/redo.
    #[derive(Clone, Debug, PartialEq)]
    pub struct CommandLogEntry {
        pub seq: u64,
        pub action_id: String,
        pub label: String,
        pub kind: ActionKind,
        pub timestamp: String,
        /// @emoji 🔗️ Set iff this command created/amended a DOCUMENT VCS edit — `None` for pure cursor
        /// motion (undo/redo/revert) and config-only dispatches that never touch the document store.
        pub edit_id: Option<String>,
        /// @emoji 🧮️ B1: the CONFIG-store twin of `edit_id` — every CONFIG edit this command created
        /// (the former "View"-kind self-computed `InverseAction` path: a config op carries a real
        /// `inverse`, so reverting it is a real config-store undo-to-position, not a memory replay). A
        /// `Vec` (not a single id) because a folded row may accumulate several distinct config edits — one
        /// per fold tick, since a config-only "View" dispatch is a plain `Apply`, not an `AmendLast` (unlike
        /// the document side, which reuses one edit id for a whole coalesced gesture) — `backfill_command_log`
        /// needs every one of them to correctly recognize "already logged". `CommandView::config_edit_id`
        /// exposes just the LATEST for display/revert purposes. A single dispatch may also set `edit_id`
        /// (touching both stores at once); `revertToCommand` prefers `edit_id` when both are present.
        pub config_edit_ids: Vec<String>,
        /// @emoji 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): every CHILD document edit id this
        /// command's composite `dispatch_group` call produced — the `config_edit_ids` precedent
        /// applied to the composition seam. Unlike `config_edit_ids` (one store, possibly several
        /// fold-ticks), this is one entry per touched CHILD member of a single group dispatch (never
        /// folded — a group dispatch always issues a plain `Apply`, see `Emit::child_emits`'s own doc
        /// comment), so it needs no "latest" reduction on the `CommandView` side.
        pub child_edit_ids: Vec<String>,
        /// @emoji 🔢️ How many consecutive identical `View`/`Shell` dispatches folded into this one row —
        /// see `VcsArtifactApp::record_command`. Always `1` for `Mutation`/`History`/`Clipboard` entries
        /// and for anything carrying an `edit_id`, which never fold.
        pub count: u32,
        /// @emoji ⏪️ A real inverse for a `Shell`-kind row with neither `edit_id` nor `config_edit_ids`
        /// (`noteShellCommand`-authored — shell-owned state this plugin cannot touch itself) — see
        /// `InverseAction`. `None` means this row has no working inverse (not just unauthored/foreign).
        pub inverse: Option<InverseAction>,
    }

    /// @emoji 🧾️ One row of the merged command+operation timeline handed to renderers — `CommandLogEntry`
    /// plus everything derived live from the current `envelope.vcs.edits` state.
    #[derive(Clone, Debug, PartialEq)]
    pub struct CommandView {
        pub seq: u64,
        pub action_id: String,
        pub label: String,
        pub kind: ActionKind,
        pub timestamp: String,
        pub edit_id: Option<String>,
        /// @emoji 🧮️ The LATEST of `CommandLogEntry::config_edit_ids` — the id `revertToCommand` targets.
        pub config_edit_id: Option<String>,
        /// @emoji 🧩️ Verbatim `CommandLogEntry::child_edit_ids` — see that field's own doc comment.
        pub child_edit_ids: Vec<String>,
        /// @emoji 📜️ This entry's edit's forward operations, printed via `OpText::print_op` — empty for
        /// cursor-motion entries and for a dangling `edit_id` (document replaced mid-session).
        pub op_lines: Vec<String>,
        /// @emoji ✅️ The linked DOCUMENT edit is currently on the applied stack (`false` once undone) —
        /// `false` (not meaningful) for a row with no `edit_id`.
        pub applied: bool,
        /// @emoji ⏪️ Either (document edit-linked) `applied` AND authored by the local actor, (config
        /// edit-linked) the config edit is similarly applied+local, or (memory-only) this row carries a
        /// stored `inverse` — only these entries offer "inverse".
        pub revertible: bool,
        /// @emoji 🔢️ See `CommandLogEntry::count`.
        pub count: u32,
        /// @emoji ⏪️ See `CommandLogEntry::inverse`.
        pub inverse: Option<InverseAction>,
    }

    /// @emoji 📜️ Checkpoint/alternative history summary exposed to apps — the swimlane columns, the
    /// undo/redo availability, the current checkout position, and the merged command+operation timeline.
    /// Built once per store generation.
    #[derive(Clone, Debug, PartialEq)]
    pub struct HistoryView {
        pub columns: Vec<HistoryColumn>,
        pub can_undo: bool,
        pub can_redo: bool,
        pub active_alternative_id: Option<String>,
        pub current_checkpoint_id: Option<String>,
        /// @emoji 📜️ The session command log merged with live VCS op-text, newest first. Every edit in
        /// `envelope.vcs.edits` is referenced by exactly one entry (see `VcsArtifactApp::backfill_command_log`).
        pub commands: Vec<CommandView>,
        pub command_filter: HistoryCommandFilter,
    }

    impl HistoryView {
        /// @emoji 🕳️ An empty view for hand-built test/fixture `ArtifactView`s that don't exercise history.
        pub fn empty() -> Self {
            Self { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None, commands: Vec::new(), command_filter: HistoryCommandFilter::default() }
        }
    }
    //#endregion 🔖️CommandLog

    //#region 🔖️HistoryPanel
    /// @emoji 🔢️ Newest-rendered rows in `ui_history_panel` — the log itself stays complete, only display
    /// is capped, matching the "no silent caps" convention (never truncates data, only the view).
    const HISTORY_PANEL_ROW_LIMIT: usize = 300;

    fn history_panel_icon_id(kind: ActionKind) -> IconName {
        match kind {
            ActionKind::Mutation => IconName::Pencil,
            ActionKind::History => IconName::Undo,
            ActionKind::Clipboard => IconName::Clipboard,
            // 🕶️ View is ephemeral cursor/selection/camera state; Shell is an outside-the-document
            // host effect — distinct icons so a folded ×count row reads at a glance.
            ActionKind::View => IconName::Eye,
            ActionKind::Shell => IconName::Monitor,
        }
    }

    /// @emoji 🕰️ Builds the framework's history panel body from a `HistoryView` as a pure side-panel
    /// `Tree` (same shape as Document/Catalogue): an Actions section (undo/redo/checkpoint/alternative +
    /// filter control) and a Commands section of newest-first rows with optional "inverse" revert.
    /// Shared by both renderers — `VcsArtifactApp::render` returns this verbatim for
    /// `FRAMEWORK_HISTORY_BODY_KEY`.
    pub fn ui_history_panel(history: &HistoryView, controller_id: &str, is_de: bool) -> UiNode {
        let act = |action: &str, args: Option<DslValue>| ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args };
        let action_item = |id: &str, icon_id: IconName, label_en: &str, label_de: &str, action: &str, enabled: bool| {
            let label = Label::data(if is_de { label_de } else { label_en });
            let mut item = UiTreeItemNode::base(id, label.clone());
            item.icon_id = Some(icon_id);
            item.control = Some(UiControlNode::Button(UiButtonNode {
                id: Some(format!("{id}.run")),
                icon_id,
                label,
                action: act(action, None),
                style: None,
                presence: if enabled { UiPresence::default() } else { UiPresence::state(UiState::Disabled) },
                menu: None,
            }));
            if !enabled {
                item.presence = UiPresence::state(UiState::Disabled);
            }
            item
        };

        let filter_value = match history.command_filter {
            HistoryCommandFilter::All => "all",
            HistoryCommandFilter::WithoutMutations => "withoutMutations",
            HistoryCommandFilter::OnlyMutations => "onlyMutations",
        };
        let mut filter_item = UiTreeItemNode::base("framework.history.filter", Label::data("Filter"));
        filter_item.icon_id = Some(IconName::Filter);
        filter_item.control = Some(UiControlNode::Select(UiSelectNode {
            id: "framework.history.filter.control".into(),
            value: filter_value.into(),
            items: vec![
                UiSelectItem { value: "all".into(), label: Label::data(if is_de { "Alle" } else { "All" }) },
                UiSelectItem { value: "withoutMutations".into(), label: Label::data(if is_de { "Ohne Operationen" } else { "Without Operations" }) },
                UiSelectItem { value: "onlyMutations".into(), label: Label::data(if is_de { "Nur Operationen" } else { "Only Operations" }) },
            ],
            placeholder: None,
            on_change: act(SET_HISTORY_COMMAND_FILTER_ACTION_ID, None),
            presence: UiPresence::default(),
            menu: None,
        }));

        let command_items: Vec<UiTreeItemNode> = history
            .commands
            .iter()
            .filter(|entry| match history.command_filter {
                HistoryCommandFilter::All => true,
                HistoryCommandFilter::WithoutMutations => entry.edit_id.is_none(),
                HistoryCommandFilter::OnlyMutations => entry.edit_id.is_some(),
            })
            .take(HISTORY_PANEL_ROW_LIMIT)
            .map(|entry| {
                // 🔢️ A folded row (`count > 1`) shows "Label xN" instead of the bare label.
                let label = if entry.count > 1 { format!("{} x{}", entry.label, entry.count) } else { entry.label.clone() };
                let mut item = UiTreeItemNode::base(format!("framework.history.entry.{}", entry.seq), Label::data(label));
                item.description = if entry.op_lines.is_empty() { None } else { Some(entry.op_lines.join(" · ")) };
                item.icon_id = Some(history_panel_icon_id(entry.kind));
                item.dimmed = (entry.edit_id.is_some() && !entry.applied).then_some(true);
                if entry.revertible {
                    item.actions = Some(vec![UiTreeItemAction {
                        icon_id: IconName::RotateCcw,
                        label: Some(Label::data(if is_de { "Zurück bis hier" } else { "Backwards" })),
                        action: act(REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::Object(vec![("entrySeq".into(), DslValue::Number(entry.seq as f64))]))),
                        placement: Some(UiTreeActionPlacement::Menu),
                    }]);
                }
                item
            })
            .collect();

        UiNode::Tree(UiTreeNode {
            sections: vec![
                UiTreeSectionNode {
                    id: "framework.history.actions".into(),
                    label: Some(Label::data(if is_de { "Aktionen" } else { "Actions" })),
                    default_open: Some(true),
                    presence: UiPresence::default(),
                    items: vec![
                        action_item("framework.history.undo", IconName::Undo, "Undo", "Rückgängig", "undo", history.can_undo),
                        action_item("framework.history.redo", IconName::Redo, "Redo", "Wiederholen", "redo", history.can_redo),
                        action_item("framework.history.commitCheckpoint", IconName::GitCommit, "Commit Checkpoint", "Checkpoint", "commitCheckpoint", true),
                        action_item("framework.history.createAlternative", IconName::GitBranch, "Create Alternative", "Alternative erstellen", "createAlternative", true),
                        filter_item,
                    ],
                },
                UiTreeSectionNode { id: "framework.history.commands".into(), label: Some(Label::data(if is_de { "Befehle" } else { "Commands" })), default_open: Some(true), presence: UiPresence::default(), items: command_items },
            ],
            presence: UiPresence::default(),
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
            menu: None,
        })
    }
    //#endregion 🔖️HistoryPanel

    /// @emoji 📤️ What a pure `ArtifactApp::handle` emits: zero-or-more typed document operations (applied
    /// through the document store with a true inverse) and zero-or-more typed config operations (applied
    /// through the config store, also with a true inverse via `ConfigMutation::inverse` — the config-op
    /// twin of a document op, replacing the old `ActionEmit::inverse`/`InverseAction` ad hoc self-computed
    /// inverse: a former "View"-kind action now just emits a `ConfigMutation` and gets a real inverse
    /// for free), plus an optional description/coalesce key for the resulting edit(s), host effects
    /// (navigate/export/spawn…), and app events. `B1` rename: was `ActionEmit`, `operations` renamed
    /// `artifact_mutations` to sit next to `config_mutations` unambiguously.
    pub struct Emit<Mutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation> {
        pub artifact_mutations: Vec<Mutation>,
        pub config_mutations: Vec<ConfigMutation>,
        pub draft_mutations: Vec<DraftMutation>,
        pub description: Option<String>,
        pub coalesce_key: Option<String>,
        pub effects: Vec<HostEffect>,
        pub events: Vec<AppEvent>,
        /// 🐢️ Which rendered UI sections this action actually invalidates — `Full` (the default) preserves
        /// today's whole-shell-refresh behavior for every app that doesn't opt in to narrower scopes.
        pub ui_scope: semio_framework::kernel::UiDirtyScope,
        /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): zero-or-more owned-child edits riding
        /// alongside this SAME gesture — the seam that lets one user action touch a parent AND N of
        /// its composed children as ONE undo step. Never hand-built; always via `ChildEmit::of`, which
        /// captures each op's `SemanticMutation` label/semantics BEFORE `OpBinary`-encoding it, so the
        /// child artifact's own semantic vocabulary survives end-to-end into history instead of being
        /// re-invented as a parent-side "child routing" mutation. `VcsArtifactApp::dispatch_emit`
        /// routes `artifact_mutations` (the parent's own ops) plus every entry here through
        /// `store::CompositionCoordinator::dispatch_group` as one atomic multi-document gesture — see
        /// that method's own doc comment for the two-phase validate/apply protocol.
        pub child_emits: Vec<ChildEmit>,
    }

    impl<Mutation, ConfigMutation, DraftMutation> Default for Emit<Mutation, ConfigMutation, DraftMutation> {
        fn default() -> Self {
            Self { artifact_mutations: Vec::new(), config_mutations: Vec::new(), draft_mutations: Vec::new(), description: None, coalesce_key: None, effects: Vec::new(), events: Vec::new(), ui_scope: semio_framework::kernel::UiDirtyScope::default(), child_emits: Vec::new() }
        }
    }

    //#region 🔖️EphemeralEmit
    /// @emoji 👥️🫧️ The two EPHEMERAL lanes' emission, deliberately separate from {@link Emit}.
    ///
    /// 🎯️ Why not more fields on `Emit`: the document lanes (artifact/config/draft) all have an op
    /// log, an edit id, an undo group and a failure mode; presence and transient have NONE of those.
    /// They cannot fail, cannot be undone, never enter a checkpoint and never appear in the command
    /// log. Folding them into `Emit` would put five type parameters on ~1000 signatures to express a
    /// thing that shares none of `Emit`'s machinery — and would force every app that emits no
    /// presence to name its presence type anyway.
    ///
    /// Apps opt in by overriding {@link ArtifactApp::ephemeral}; the default emits nothing, so an
    /// app with no shareable or UI-local state writes no code at all.
    #[derive(Debug)]
    pub struct EphemeralEmit<A: ArtifactApp + ?Sized> {
        /// 👥️ Ephemeral SHARED — broadcast to peers, never persisted.
        pub presence: Vec<A::PresenceMutation>,
        /// 🫧️ Ephemeral LOCAL-ONLY — never leaves this client.
        pub transient: Vec<A::TransientMutation>,
    }

    impl<A: ArtifactApp + ?Sized> Default for EphemeralEmit<A> {
        fn default() -> Self {
            Self { presence: Vec::new(), transient: Vec::new() }
        }
    }

    impl<A: ArtifactApp + ?Sized> EphemeralEmit<A> {
        /// 👥️ Presence-only emission — the common case (a moved cursor, a changed selection).
        pub fn presence(presence: Vec<A::PresenceMutation>) -> Self {
            Self { presence, transient: Vec::new() }
        }

        /// 🫧️ Transient-only emission — the common case (a hover, an in-flight gesture).
        pub fn transient(transient: Vec<A::TransientMutation>) -> Self {
            Self { presence: Vec::new(), transient }
        }

        /// 🈳️ Whether this emission touches neither ephemeral lane.
        pub fn is_empty(&self) -> bool {
            self.presence.is_empty() && self.transient.is_empty()
        }
    }
    //#endregion 🔖️EphemeralEmit

    /// 🧩️ One composed child's share of an `Emit` — the plugin-layer twin of `store::ChildDispatch`,
    /// minted exclusively by `ChildEmit::of` so a plugin author never hand-encodes an op or
    /// hand-writes a `SchemaId`. `ops` are already `protocol::OpBinary`-encoded (the same per-op wire
    /// shape `store::ChildDispatch::ops` bundles); `labels` are each op's `SemanticMutation::label()`,
    /// captured BEFORE encoding — the vocabulary a history UI shows for this child's edit without
    /// ever decoding the raw bytes back into a concrete `Mutation` type it has no way to name
    /// generically.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ChildEmit {
        pub slot: String,
        pub child_id: String,
        pub ops: Vec<Vec<u8>>,
        pub op_schema: SchemaId,
        pub labels: Vec<String>,
    }

    impl ChildEmit {
        /// 🏭️ The one sanctioned constructor: `S` is the child artifact's OWN snapshot type (turbofish
        /// it explicitly, e.g. `ChildEmit::of::<ChildSnapshot, _>("mesh", &child_id, ops)` — `S`
        /// appears only in the `SemanticMutation<S>` bound below, so Rust cannot infer it from `ops`
        /// alone). Captures `op.label()`/`op.semantics()` per op (the child's real semantic
        /// vocabulary) BEFORE `OpBinary::encode_op`, so nothing about the child's own history
        /// authoring is lost crossing into the parent's `Emit`. `op_schema` is a best-effort
        /// diagnostic tag derived from the first op's `SemanticDescriptor` (`"<entity>.<kind>"`,
        /// falling back to `"child.empty"` for a no-op `ChildEmit`) — `store::dispatch_group` carries
        /// `ChildDispatch.op_schema` through as forward-compat metadata without interpreting it (see
        /// `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b2-store-composition-report.md`), so
        /// there is no schema REGISTRY this must resolve against yet.
        pub fn of<S, M>(slot: impl Into<String>, child_id: impl Into<String>, ops: Vec<M>) -> ChildEmit
        where
            M: protocol::SemanticMutation<S> + ::protocol::OpBinary,
        {
            let labels: Vec<String> = ops.iter().map(|op| protocol::SemanticMutation::label(op)).collect();
            let op_schema = ops
                .first()
                .map(|op| {
                    let semantics = protocol::SemanticMutation::semantics(op);
                    SchemaId(format!("{}.{}", semantics.entity, semantics.kind))
                })
                .unwrap_or_else(|| SchemaId("child.empty".to_string()));
            let encoded: Vec<Vec<u8>> = ops.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect();
            ChildEmit { slot: slot.into(), child_id: child_id.into(), ops: encoded, op_schema, labels }
        }
    }

    impl<Mutation, ConfigMutation, DraftMutation> Emit<Mutation, ConfigMutation, DraftMutation> {
        /// @emoji ✏️ A document-operation emission carrying `artifact_mutations` and nothing else.
        pub fn mutations(artifact_mutations: Vec<Mutation>) -> Self {
            Self { artifact_mutations, ..Default::default() }
        }

        /// @emoji 🔁️ Preview pattern (a): a per-tick coalesced DOCUMENT emission. The `coalesce_key` folds
        /// every tick of one live gesture (drag/scrub) into a single amendable edit, so the whole gesture is
        /// one undo. Use for cheap per-tick document operations. See `🔖️UtilityPreviewContract`.
        pub fn amend(artifact_mutations: Vec<Mutation>, coalesce_key: impl Into<String>) -> Self {
            Self { artifact_mutations, coalesce_key: Some(coalesce_key.into()), ..Default::default() }
        }

        /// @emoji 📌️ Preview pattern (b): the gesture-end commit of an app-runtime scratch draft as one
        /// described DOCUMENT edit (`coalesce_key: None`). Use for megabyte-scale content where per-tick
        /// amending would be O(N²) (draw drafts, lowpoly strokes). See `🔖️UtilityPreviewContract`.
        pub fn commit(artifact_mutations: Vec<Mutation>, description: impl Into<String>) -> Self {
            Self { artifact_mutations, description: Some(description.into()), ..Default::default() }
        }

        /// @emoji 🧮️ A config-operation emission carrying `config_mutations` and nothing else — the
        /// replacement for a former "View"-kind `ActionEmit::view_with_inverse`: selection/camera/hover/…
        /// changes now flow through the config store, which computes their real `inverse` itself.
        pub fn config(config_mutations: Vec<ConfigMutation>) -> Self {
            Self { config_mutations, ..Default::default() }
        }

        /// @emoji 📝️ A draft-operation emission carrying `draft_mutations` and nothing else.
        pub fn draft(draft_mutations: Vec<DraftMutation>) -> Self {
            Self { draft_mutations, ..Default::default() }
        }

        /// @emoji 🔁️ `amend`'s CONFIG-targeted twin — coalesces one live gesture's ticks into a single
        /// amendable config edit (e.g. a live camera drag).
        pub fn amend_config(config_mutations: Vec<ConfigMutation>, coalesce_key: impl Into<String>) -> Self {
            Self { config_mutations, coalesce_key: Some(coalesce_key.into()), ..Default::default() }
        }

        /// @emoji 📌️ `commit`'s CONFIG-targeted twin — a described, non-coalesced config edit.
        pub fn commit_config(config_mutations: Vec<ConfigMutation>, description: impl Into<String>) -> Self {
            Self { config_mutations, description: Some(description.into()), ..Default::default() }
        }

        /// @emoji 🐚️ A single host effect and no operations (a shell action).
        pub fn effect(effect: HostEffect) -> Self {
            Self { effects: vec![effect], ..Default::default() }
        }

        /// @emoji 📣️ A single app event and no operations.
        pub fn event(event: AppEvent) -> Self {
            Self { events: vec![event], ..Default::default() }
        }
    }

    /// @emoji 🪪️ Per-invocation runtime metadata handed to the object-safe {@link PluginApp} — the local
    /// actor id (author of resulting operations, drives `UndoPolicy` foreign-edit classification) and
    /// the instance id used to stamp operation/document handles.
    #[derive(Clone, Debug, Default)]
    pub struct ActionMeta {
        pub actor: String,
        pub instance_id: u32,
    }

    /// @emoji 🔤️ Parses the raw action id crossing the WASM ABI (`ArtifactApp::handle_action`'s `action: &str`)
    /// into a closed, per-app enum — the seam where "stringly-typed at the edge" becomes exhaustively
    /// matched one line in. Not yet wired into `ArtifactApp` itself (that would break every existing
    /// implementer at once); adopt it per app by matching on the parsed variant instead of the raw string
    /// inside `handle_action`, e.g. `let action = MyAppAction::from_action_id(action)?; match action { ... }`.
    pub trait AppAction: Sized {
        fn from_action_id(id: &str) -> Result<Self, String>;
    }

    /// @emoji 🏭️ Generates a closed per-app action enum plus its `AppAction` impl from a list of
    /// `Variant = "actionId"` pairs — the ids should match what's passed to `.mutation()/.view_action()/
    /// .shell_action()` on the app's `AppBuilder` so the declared action registry and the dispatch match
    /// can't drift apart silently.
    #[macro_export]
    macro_rules! app_action_enum {
    ($vis:vis enum $Name:ident { $($Variant:ident = $id:literal),* $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis enum $Name {
            $($Variant),*
        }

        impl $crate::app::AppAction for $Name {
            fn from_action_id(id: &str) -> Result<Self, String> {
                match id {
                    $($id => Ok(Self::$Variant),)*
                    other => Err(Fault::from(format!("unknown action id {other}"))),
                }
            }
        }
    };
}

    //#region 🔖️AppCommands
    /// @emoji 🎮️ Generates a closed per-app `ArtifactApp::Command` enum from `"id" => module::Payload`
    /// rows — the taxonomy-decomposed replacement for a hand-written
    /// `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)] pub enum XCommand { #[dsl(key = "...")] Variant { .. }, .. }`
    /// (see e.g. `flow_protocol::FlowCommand`, `dag_protocol::DagCommand`, `shooting_protocol::ShootingCommand`
    /// — every hand-written Command enum in the repo already follows this exact derive/attribute shape, none
    /// use `#[serde(tag = ...)]`; the real wire codec is `dsl::DslOps`'s generated `OpText`/`OpBinary`, keyed
    /// by each variant's `#[dsl(key = ..)]` string).
    ///
    /// Each row becomes a single-field tuple variant `$Payload($module::$Payload)`. This is deliberate, not
    /// incidental: `dsl::DslOps`'s codegen special-cases single-field tuple variants (see
    /// `dsl_variants_codegen` in the `dsl-derive` crate) to delegate ENTIRELY to the inner type's own
    /// `DslField` impl — its `RecordSpec` IS the payload type's, not a wrapper with one positional field — so
    /// a payload struct declared with `#[derive(dsl::DslRecord)]` (its own fields, no extra nesting) prints/
    /// parses byte-identically whether reached through this enum or used standalone. Concretely: migrating a
    /// hand-written `AddWidget { kind: String, x: Option<f64> }` struct-variant into a
    /// `🎮️commands/add_widget/🦀️component.rs` payload `struct AddWidget { kind: String, x: Option<f64> }`
    /// (deriving `DslRecord`) plus `AddWidget(add_widget::AddWidget)` here reproduces the exact same
    /// `OpText`/`OpBinary` wire bytes for the same `#[dsl(key = "...")]` string — the migration is pure code
    /// motion, not a wire-format change.
    ///
    /// Expands to:
    /// - the enum itself (`$vis enum $Name`), deriving `Clone, Debug, PartialEq, ::serde::Serialize,
    ///   ::serde::Deserialize, dsl::DslOps` — satisfies `ArtifactApp::Command: protocol::OpBinary + Send`
    ///   directly. `dsl`/`serde` are referenced unqualified/fully-qualified (not via `$crate`, since they are
    ///   NOT the defining crate) — every crate that hosts `🎮️commands/*` payload modules already depends on
    ///   `dsl` directly (payloads themselves derive `dsl::DslRecord`), matching the convention every existing
    ///   hand-written `*_protocol` crate already follows.
    /// - `command_id(&self) -> &'static str`, returning each row's `$id` — mirrors the per-command labeling
    ///   `ArtifactApp::command_id` needs (today hand-rolled as a parallel `match` per app, e.g.
    ///   `TestApp::command_id` above).
    /// - `dispatch(&self, doc: &ArtifactView<'_, $Snapshot>, cfg: &ConfigView<'_, $Config>) -> Result<Emit<$Mutation, $ConfigMutation>, Fault>`,
    ///   matching each variant to `$module::handle(payload, doc, cfg)` — `$Snapshot`/`$Mutation`/`$Config`/
    ///   `$ConfigMutation` are the four types declared after `for` in the invocation (see below).
    ///
    /// # 🔖️DispatchDesignChoice
    /// The macro invocation names the app's four `ArtifactApp` associated types up front —
    /// `enum $Name for $Snapshot, $Mutation, $Config, $ConfigMutation { .. }` — so `dispatch` can be a
    /// perfectly ordinary, CONCRETE (non-generic) inherent method. This was chosen over the alternative
    /// tried first: a `dispatch<P, O, C, CO>` generic over all four, inferring them from the call site. That
    /// alternative does not type-check, and the `app_commands_tests` module below hit exactly this in
    /// practice — every real `🎮️commands/<command>/🦀️component.rs::handle` fn will be written against ONE
    /// app's concrete `Snapshot`/`Mutation`/`Config`/`ConfigMutation` (the same way
    /// `flow_protocol`/`dag_protocol`/... payload handlers are today), never generically; a generic
    /// `dispatch<P,O,C,CO>` body calling a concrete-typed `$module::handle` fails to unify (`P` is not
    /// literally `FlowFixture`, even though there is only ever one real instantiation). Naming the four
    /// types once, at the `app_commands!` invocation, costs one extra clause per app but produces a
    /// `dispatch` whose signature matches `ArtifactApp::handle` exactly — so an app's whole `handle` impl
    /// collapses to one line, `command.dispatch(doc, cfg)`. A per-command closure/trait-object API (the
    /// plan's other suggested option) would force hand-rolling one closure per row at adoption time instead
    /// — strictly more ceremony than stating the four types once.
    ///
    /// # 🔖️KeyedAndContextualForms
    /// Two further arms exist, both discovered as hard requirements by the W1 🌊️flow pilot (the first real
    /// consumer) and additive to the plain arm above, which is unchanged:
    ///
    /// 1. **Keyed rows** — `"commandId" as "wire-key" => module::Payload`. Every hand-written Command enum in
    ///    the repo keys its wire form in kebab-case (`#[dsl(key = "add-widget")]`) while `command_id()` must
    ///    return the camelCase manifest ACTION id (`"addWidget"`) the app declared via `.mutation()`/
    ///    `.view_action()`. The plain arm conflates the two into one literal, which silently rewrites the wire
    ///    format of any existing app it is applied to. Stating both keeps a migration pure code motion.
    /// 2. **A dispatch context** — `…, $ConfigMutation:ty, ctx = $Ctx:ty { … }` adds a fourth
    ///    `ctx: &mut $Ctx` parameter to `dispatch` and to every row's `$module::handle`. Apps whose handlers
    ///    need app-struct state that is deliberately NOT in the document or the config (flow's
    ///    `Mutex<FlowEvalSession>` off-main-thread eval driver, reached once per dispatch and threaded through
    ///    every handler) have nowhere else to put it — `handle(&self, …)` on `ArtifactApp` has `&self`, but the
    ///    macro-generated `dispatch` does not.
    ///
    /// The two are independent; combine them or use `ctx = ()` when only the keys differ.
    #[macro_export]
    macro_rules! app_commands {
    ($(#[$meta:meta])* $vis:vis enum $Name:ident for $Snapshot:ty, $Mutation:ty, $Config:ty, $ConfigMutation:ty, ctx = $Ctx:ty { $($id:literal as $key:literal => $module:ident :: $Payload:ident),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, dsl::DslOps)]
        $vis enum $Name {
            $(
                #[dsl(key = $key)]
                $Payload($module::$Payload),
            )*
        }

        impl $Name {
            /// 🏷️ Each row's `$id` — the manifest action id, distinct from the `$key` wire keyword.
            pub fn command_id(&self) -> &'static str {
                match self {
                    $(Self::$Payload(_) => $id,)*
                }
            }

            /// 🎯️ Matches each variant to `$module::handle(payload, doc, cfg, ctx)` — see
            /// `🔖️KeyedAndContextualForms` for why `ctx` exists.
            pub fn dispatch(&self, doc: &$crate::ArtifactView<'_, $Snapshot>, cfg: &$crate::ConfigView<'_, $Config>, ctx: &mut $Ctx) -> Result<$crate::Emit<$Mutation, $ConfigMutation>, $crate::Fault> {
                match self {
                    $(Self::$Payload(payload) => $module::handle(payload, doc, cfg, ctx),)*
                }
            }
        }

        impl ::protocol::OpText for $Name {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{keyword} ");
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                        let record = ::dsl::parse(
                            body,
                            &spec_fn(),
                            &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                        )?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                if body.is_empty() {
                    keyword
                } else {
                    format!("{keyword} {body}")
                }
            }
        }

        impl ::protocol::OpBinary for $Name {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::variants_binary::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::variants_binary::decode_op(bytes)
            }
        }
    };

    ($(#[$meta:meta])* $vis:vis enum $Name:ident for $Snapshot:ty, $Mutation:ty, $Config:ty, $ConfigMutation:ty { $($id:literal as $key:literal => $module:ident :: $Payload:ident),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, dsl::DslOps)]
        $vis enum $Name {
            $(
                #[dsl(key = $key)]
                $Payload($module::$Payload),
            )*
        }

        impl $Name {
            /// 🏷️ Each row's `$id` — the manifest action id, distinct from the `$key` wire keyword.
            pub fn command_id(&self) -> &'static str {
                match self {
                    $(Self::$Payload(_) => $id,)*
                }
            }

            /// 🎯️ Matches each variant to `$module::handle(payload, doc, cfg)`.
            pub fn dispatch(&self, doc: &$crate::ArtifactView<'_, $Snapshot>, cfg: &$crate::ConfigView<'_, $Config>) -> Result<$crate::Emit<$Mutation, $ConfigMutation>, $crate::Fault> {
                match self {
                    $(Self::$Payload(payload) => $module::handle(payload, doc, cfg),)*
                }
            }
        }

        impl ::protocol::OpText for $Name {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{keyword} ");
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                        let record = ::dsl::parse(
                            body,
                            &spec_fn(),
                            &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                        )?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                if body.is_empty() {
                    keyword
                } else {
                    format!("{keyword} {body}")
                }
            }
        }

        impl ::protocol::OpBinary for $Name {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::variants_binary::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::variants_binary::decode_op(bytes)
            }
        }
    };

    ($(#[$meta:meta])* $vis:vis enum $Name:ident for $Snapshot:ty, $Mutation:ty, $Config:ty, $ConfigMutation:ty { $($id:literal => $module:ident :: $Payload:ident),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, dsl::DslOps)]
        $vis enum $Name {
            $(
                #[dsl(key = $id)]
                $Payload($module::$Payload),
            )*
        }

        impl $Name {
            /// 🏷️ Each row's `$id` — mirrors `ArtifactApp::command_id`'s per-command labeling need.
            pub fn command_id(&self) -> &'static str {
                match self {
                    $(Self::$Payload(_) => $id,)*
                }
            }

            /// 🎯️ Matches each variant to `$module::handle(payload, doc, cfg)` — see `🔖️DispatchDesignChoice`
            /// above for why this is concrete (not generic) in the app's own associated types.
            pub fn dispatch(&self, doc: &$crate::ArtifactView<'_, $Snapshot>, cfg: &$crate::ConfigView<'_, $Config>) -> Result<$crate::Emit<$Mutation, $ConfigMutation>, $crate::Fault> {
                match self {
                    $(Self::$Payload(payload) => $module::handle(payload, doc, cfg),)*
                }
            }
        }

        impl ::protocol::OpText for $Name {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{keyword} ");
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                        let record = ::dsl::parse(
                            body,
                            &spec_fn(),
                            &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                        )?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                if body.is_empty() {
                    keyword
                } else {
                    format!("{keyword} {body}")
                }
            }
        }

        impl ::protocol::OpBinary for $Name {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::variants_binary::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::variants_binary::decode_op(bytes)
            }
        }
    };
}

    /// 🧪️ Minimal end-to-end proof for `app_commands!` — no real plugin adopts it yet (payload modules
    /// land per-command in W1/W2), so this defines 2 trivial fake payload modules inline and exercises the
    /// macro's full generated surface: enum construction, `command_id()`, `dispatch()`, and the
    /// `dsl::DslOps` wire round trip (`OpText`/`OpBinary`) via the same `store::test_support` helper every
    /// real `*_protocol` crate's own Command enum tests use (see e.g. `flow_protocol`'s
    /// `flow_command_text_binary_round_trips_document_mutating_variants`).
    #[cfg(test)]
    mod app_commands_tests {
        use crate::{ConfigView, ArtifactView, Emit, HistoryView, NoConfigMutation};

        mod add_widget {
            #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, dsl::DslRecord)]
            pub struct AddWidget {
                pub kind: String,
                pub x: f64,
            }

            /// 🎯️ Mirrors the shape a real `🎮️commands/add_widget/🦀️component.rs::handle` will have —
            /// `(payload, doc, cfg) -> Result<Emit<Mutation, ConfigMutation>, Fault>`.
            pub fn handle(payload: &AddWidget, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
                Ok(crate::Emit::mutations(vec![format!("add:{}:{}", payload.kind, payload.x)]))
            }
        }

        mod delete_selection {
            #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, dsl::DslRecord)]
            pub struct DeleteSelection {
                pub id: String,
            }

            pub fn handle(payload: &DeleteSelection, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
                Ok(crate::Emit::mutations(vec![format!("delete:{}", payload.id)]))
            }
        }

        app_commands! {
            pub enum TestFakeCommand for u32, String, (), NoConfigMutation {
                "addWidget" => add_widget::AddWidget,
                "deleteSelection" => delete_selection::DeleteSelection,
            }
        }

        #[test]
        fn command_id_matches_declared_row() {
            assert_eq!(TestFakeCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: 1.5 }).command_id(), "addWidget");
            assert_eq!(TestFakeCommand::DeleteSelection(delete_selection::DeleteSelection { id: "n1".into() }).command_id(), "deleteSelection");
        }

        #[test]
        fn dispatch_forwards_to_the_payload_modules_own_handle() {
            let snapshot = 0u32;
            let config = ();
            let history = HistoryView::empty();
            let doc = ArtifactView::new(&snapshot, &history);
            let cfg = ConfigView { snapshot: &config };

            let emit: Emit<String, NoConfigMutation> = TestFakeCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: 1.5 }).dispatch(&doc, &cfg).expect("dispatch add-widget");
            assert_eq!(emit.artifact_mutations, vec!["add:inputSlider:1.5".to_string()]);

            let emit: Emit<String, NoConfigMutation> = TestFakeCommand::DeleteSelection(delete_selection::DeleteSelection { id: "n1".into() }).dispatch(&doc, &cfg).expect("dispatch delete-selection");
            assert_eq!(emit.artifact_mutations, vec!["delete:n1".to_string()]);
        }

        #[test]
        fn wire_round_trips_through_dsl_ops_op_text_and_op_binary() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&TestFakeCommand::AddWidget(add_widget::AddWidget { kind: "neuron".into(), x: 2.0 }));
            store::os_store::test_support::assert_op_text_binary_equivalence(&TestFakeCommand::DeleteSelection(delete_selection::DeleteSelection { id: "n1".into() }));
        }

        mod keyed {
            #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, dsl::DslRecord)]
            pub struct AddWidget {
                pub kind: String,
            }

            pub fn handle(payload: &AddWidget, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>, ctx: &mut u32) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
                *ctx += 1;
                Ok(crate::Emit::mutations(vec![format!("add:{}:{ctx}", payload.kind)]))
            }
        }

        mod keyed_unit {
            #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, dsl::DslRecord)]
            pub struct DeleteSelection {}

            pub fn handle(_payload: &DeleteSelection, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>, _ctx: &mut u32) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
                Ok(crate::Emit::mutations(vec!["delete".to_string()]))
            }
        }

        app_commands! {
            pub enum TestKeyedCommand for u32, String, (), NoConfigMutation, ctx = u32 {
                "addWidget" as "add-widget" => keyed::AddWidget,
                "deleteSelection" as "delete-selection" => keyed_unit::DeleteSelection,
            }
        }

        /// 🧪️ The keyed arm must keep `command_id()` (manifest action id) and the `dsl` wire keyword
        /// independent — the exact split every hand-written `*_protocol` Command enum already has.
        #[test]
        fn keyed_rows_separate_the_command_id_from_the_wire_keyword() {
            let command = TestKeyedCommand::AddWidget(keyed::AddWidget { kind: "inputSlider".into() });
            assert_eq!(command.command_id(), "addWidget");
            assert!(protocol::OpText::print_op(&command).starts_with("add-widget "), "wire keyword must be the kebab `as` literal, got {:?}", protocol::OpText::print_op(&command));
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }

        /// 🧪️ A fieldless payload struct must print/encode exactly like the unit variant it replaces —
        /// the migration-safety property for every `DeleteSelection`-style bare variant.
        #[test]
        fn fieldless_payload_matches_a_unit_variants_wire_form() {
            let command = TestKeyedCommand::DeleteSelection(keyed_unit::DeleteSelection {});
            assert_eq!(protocol::OpText::print_op(&command), "delete-selection");
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode"), vec![1u8, 1, 0, 0]);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }

        #[test]
        fn ctx_is_threaded_through_dispatch_into_every_handler() {
            let snapshot = 0u32;
            let config = ();
            let history = HistoryView::empty();
            let doc = ArtifactView::new(&snapshot, &history);
            let cfg = ConfigView { snapshot: &config };
            let mut ctx = 41u32;
            let emit: Emit<String, NoConfigMutation> = TestKeyedCommand::AddWidget(keyed::AddWidget { kind: "neuron".into() }).dispatch(&doc, &cfg, &mut ctx).expect("dispatch");
            assert_eq!(emit.artifact_mutations, vec!["add:neuron:42".to_string()]);
            assert_eq!(ctx, 42);
        }
    }
    //#endregion 🔖️AppCommands

    /// @emoji 🧩️ Typed, per-app author surface. An app declares its `Snapshot` and `Mutation` (a
    /// `store::Mutation<Snapshot>`), mutates nothing directly, and returns an {@link ActionEmit} whose
    /// operations flow through a persistent `ArtifactStore` owned by {@link VcsArtifactApp}. Ephemeral
    /// view state (selection/camera/active utility) lives in the app struct itself, not in the document.
    ///
    /// # 🔖️UtilityPreviewContract
    /// The formalized actions-vs-utilities contract:
    /// - **Actions** are non-interactive: they carry optional declared `ActionArgDef`s, stage in the
    ///   renderer, and execute once. `Mutation`-kind actions emit operations; `View`/`Shell`-kind actions must
    ///   emit **zero** operations ({@link VcsArtifactApp} enforces this — a View/Shell action returning operations is a
    ///   hard error).
    /// - **Utilities** are interactive live-preview pointer modes. Exactly one utility is active per window kind;
    ///   the active utility arrives via `view_state.active_utility_id` and is **never** stored in the document
    ///   nor emitted as an operation. Switching utilities dispatches the framework `setActiveUtility` View action; on a
    ///   switch the app must clear any in-progress preview scratch.
    /// - **Two blessed preview patterns** (both funnel through {@link ActionEmit}):
    ///   1. per-tick coalesced — {@link ActionEmit::amend} folds each tick of a gesture into one amendable
    ///      edit (one undo per gesture); use for cheap operations (camera/opacity drags).
    ///   2. scratch + commit — hold a draft in app-runtime state, render it as an overlay, and on gesture
    ///      end emit {@link ActionEmit::commit} once; use for megabyte-scale content where per-tick
    ///      amending is O(N²) (draw drafts, lowpoly strokes).
    /// - The pointer vocabulary (`canvasPointerDown/Move/Up`, `worldPointerDown/Move/Up`,
    ///   `paintStrokeBegin/End`) are `View`-kind internal action ids driving the above.
    pub trait ArtifactApp: Default + Send + 'static {
        /// @emoji 🪪 Stable app id — prefer this over `app_id(&self)` on the path to receiverless ZSTs.
        const APP_ID: &'static str;
        /// @emoji 📜️ Stable document schema id — prefer this over `document_schema(&self)`.
        const DOCUMENT_SCHEMA: &'static str;
        type Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack;
        type Mutation: ::protocol::Mutation<Self::Snapshot> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        type Config: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::ConfigRecord + ArtifactPack;
        type ConfigMutation: ::protocol::Mutation<Self::Config> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        /// @emoji 📝️ Volatile draft snapshot — use {@link NoDraft} when the app has no draft lane.
        type Draft: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack;
        /// @emoji 📝️ Draft-lane operations applied to {@link store::DraftStore}.
        type DraftMutation: ::protocol::Mutation<Self::Draft> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        /// @emoji 👥️ Shared live presence — use {@link NoPresence} when the app has no shareable live state.
        type Presence: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack;
        /// @emoji 👥️ Presence-lane operations applied to the app's typed presence snapshot.
        type PresenceMutation: ::protocol::Mutation<Self::Presence> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;
        /// @emoji 🫧️ Ephemeral LOCAL-ONLY UI state — use {@link NoTransient} when the app has none.
        /// The fourth and last state mechanism; see `NoTransient`'s doc for how it differs from a
        /// draft (which is ephemeral *artifact* content, not UI state).
        type Transient: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack;
        /// @emoji 🫧️ Transient-lane operations applied to {@link store::TransientStore}.
        type TransientMutation: ::protocol::Mutation<Self::Transient> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary;

        /// @emoji 👥️🫧️ The two EPHEMERAL lanes this command touches — presence (shared) and
        /// transient (local-only UI). Separate from `handle` because neither lane has an op log, an
        /// undo group, or a failure mode: they are applied unconditionally and cannot fail, so
        /// folding them into `handle`'s `Result<Emit, Fault>` would misrepresent them.
        ///
        /// Defaults to emitting nothing, so an app with no shareable or UI-local state writes no
        /// code. Called on every dispatched command, right before `handle`.
        fn ephemeral(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _presence: &PresenceView<'_, Self::Presence>, _transient: &TransientView<'_, Self::Transient>) -> EphemeralEmit<Self> {
            EphemeralEmit::default()
        }
        /// @emoji 🎯️ B1: this app's closed, typed command enum — the SOLE dispatch surface for
        /// `handle` below, replacing the deleted stringly-typed `handle_action`/`handle_command`/
        /// `handle_typed_command` trio. Decoded off the wire once, by `VcsArtifactApp::dispatch_typed_command`,
        /// via `OpBinary::decode_op`; framework-reserved verbs (undo/redo/checkpoint/alternative/clipboard/
        /// revert/history-filter/noteShellCommand) never reach here — the wrapper intercepts those itself
        /// (see `VcsArtifactApp::dispatch_framework_action`) since they are host mechanics, not app behavior.
        type Command: ::protocol::OpBinary + Send;

        
        
        fn config_schema() -> &'static str {
            "config.empty"
        }
        fn initial_snapshot() -> Self::Snapshot;
        fn initial_config() -> Self::Config {
            Self::Config::default()
        }
        fn initial_draft() -> Self::Draft {
            Self::Draft::default()
        }
        /// @emoji 🧩️ B1: the pure heart of the app — a total, side-effect-free function from
        /// `(command, document, config, draft, engines)` to an {@link Emit}. No `&mut self`.
        /// `engines` is the host-owned {@link EngineHandles} bag (empty until WIT engine-derive/read
        /// is threaded through exchange).
        fn handle(
            command: &Self::Command,
            doc: &ArtifactView<'_, Self::Snapshot>,
            cfg: &ConfigView<'_, Self::Config>,
            draft: &DraftView<'_, Self::Draft>,
            engines: &EngineHandles,
        ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault>;
        /// @emoji 🏷️ `command`'s action/command id string — used for command-log labeling and
        /// `AppActionRegistry` kind-discipline lookup (`View`/`Shell`-kind must not emit
        /// `artifact_mutations`; `VcsArtifactApp::dispatch_typed_command_inner` enforces this when the
        /// registry has a matching declaration). Default: `"typed-command"` for every command (correct but
        /// generic — an app that wants per-command labels/kind-discipline overrides this to match the id
        /// the command was declared under in its `AppDefinition`, e.g. via `.operation`/`.view_action`).
        fn command_id(_command: &Self::Command) -> &'static str {
            "typed-command"
        }
        /// @emoji 🎯️ Builds this app's typed `Command` from a host action id + JSON args — the bridge
        /// the React/wgpu shells still speak (`{action,args}`) until every call site sends `OpBinary`
        /// bytes directly. Default rejects (same error as the pre-bridge `dispatch_action` arm); apps
        /// that the shells drive must override this so chrome actions reach `handle`.
        fn command_from_action(action: &str, _args: Option<&serde_json::Value>) -> Result<Self::Command, Fault> {
            Err(Fault::new(
                FaultOrigin::App,
                FaultCode::new("app.command.unsupported"),
                format!(
                    "action '{action}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
                ),
            ))
        }
        /// 🧮️ This app's typed configuration spec — empty (the default) means "no configuration options."
        fn config_spec() -> ConfigSpec {
            ConfigSpec::empty()
        }
        /// 📋️ The `MediaType` this app copies fragments as and accepts pastes of by default — `None` (the
        /// default) means this app doesn't participate in the clipboard mechanism, and `VcsArtifactApp`'s
        /// injected `copy`/`cut`/`paste` actions silently no-op. Override alongside `copy_fragment`/
        /// `cut_operations`/`paste_operations`.
        fn clipboard_media_type() -> Option<MediaType> {
            None
        }
        /// 📋️ Every `MediaType` this app accepts a paste of — defaults to just `clipboard_media_type()`.
        /// Override to additionally accept fragments copied from a compatible sibling app (e.g. puzzle
        /// accepting a block kind-definition fragment as a catalog merge).
        fn clipboard_accepts() -> Vec<MediaType> {
            Self::clipboard_media_type().into_iter().collect()
        }
        /// 📋️ Builds a `ClipboardFragment` from the current selection. Called by `VcsArtifactApp`'s
        /// injected `copy` and `cut` actions; `cut` additionally calls `cut_operations`. Default: always
        /// empty (apps that don't override `clipboard_media_type` never reach here in practice, since the
        /// interception only calls this when a media type is declared).
        fn copy_fragment( _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<ClipboardFragment, ClipboardError> {
            Err(ClipboardError::EmptySelection)
        }
        fn cut_operations( _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Vec<Self::Mutation> {
            Vec::new()
        }
        fn paste_operations( _doc: &ArtifactView<'_, Self::Snapshot>, _fragment: &ClipboardFragment, _placement: &PastePlacement) -> Result<Vec<Self::Mutation>, ClipboardError> {
            Ok(Vec::new())
        }
        fn pending_effects( _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Vec<HostEffect> {
            Vec::new()
        }
        fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> UiNode;
        /// 🪟️ Keyed by window INSTANCE id — an app with two open instances of the same kind (e.g. split
        /// panes) returns one entry per instance so their chrome/options never collapse together. Apps with
        /// a single window kind and no splitting return `vec![kind_id]`-worth of entries either way.
        fn window_engagements( _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> HashMap<String, WindowEngagement> {
            HashMap::new()
        }
        /// 🪟️ See `window_engagements` — same per-window-instance keying.
        fn window_measures( _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> HashMap<String, Vec<WindowMeasure>> {
            HashMap::new()
        }
        /// 🛠️ Keyed by TOOL id (`AppDefinition.tools[].id`), not window instance — a tool's live options
        /// (e.g. puzzle3d fill's count slider) rendered in the mode-level tool panel rather than a
        /// window's utility-options rail. Reuses `WindowMeasure` as the shared control vocabulary; the
        /// `Group.active_utility_id` tag is simply unused for tool measures.
        fn tool_measures( _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> HashMap<String, Vec<WindowMeasure>> {
            HashMap::new()
        }
        /// 🖱️ Answers an on-demand right-click menu request — the WIT `context-menu` export's SDK
        /// counterpart. Called fresh at right-click time (never cached, never part of `refreshUi`);
        /// `request.menu.id` names the target the host resolved (a `UiMenuRef` a plugin attached to a
        /// `UiNode`, or a scene-surface convention id like `"world3d"`/`"nodeGraph"`/`"window"`).
        /// Default: empty — the host falls through to the next outer menu layer (window/OS chrome),
        /// so apps that never attach a `menu` never need to override this. `registry` is this app's
        /// `AppActionRegistry` (the same one `VcsArtifactApp` enforces the actions contract with) —
        /// pass it to `Menu::of(registry)` to resolve labels/icons from declared `ActionDefinition`s
        /// instead of hand-building rows.
        fn context_menu( _request: &ContextMenuRequest, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
            Vec::new()
        }
        /// 🌱️ Initial mutations applied through normal dispatch right after the store is constructed —
        /// replaces the old `seed(&mut ArtifactStore)` direct-store-touch hook (ticket
        /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4: this was the only place an app touched a
        /// store directly; routing genesis mutations through the same `ArtifactCommand::Apply` path
        /// every user edit takes removes that exception). Default: no mutations — only apps whose
        /// fixture is itself a rich history (e.g. a history-UI demo/exerciser) need this — every
        /// program driven purely by user actions leaves it untouched.
        fn genesis() -> Vec<Self::Mutation> {
            Vec::new()
        }

        /// 🪪️ This app's own config+presence schema descriptor, auto-registered by
        /// `register_document_app`/`document_app` the moment this type is bound to a plugin (ticket
        /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c: closes the last legitimate `.setup()`
        /// reason — app-scope schema is app-scope precisely because it is keyed off `Self`, so the
        /// builder call that already names `Self` is the correct place to register it, not a plugin-root
        /// side callback). `None` (the default) means this app has no schema of its own — a plugin
        /// library with zero document apps, or a document app whose config truly has no dedicated facet.
        fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            None
        }

        /// 🔌️ This app's typed media I/O surface — `None` (the default) means "declares no ports beyond
        /// the implicit document ports" (`media_ports` below still returns those two). Override to return
        /// a real `AppIo` (e.g. `shooting_engine::shooting_io()`) to declare extra workflow ports.
        fn io() -> Option<AppIo> {
            None
        }
        /// 🎞️ Declares this app's workflow ports — delegates to `Self::io().all_ports()` (implicit
        /// `document:in`/`document:out` plus any app-specific ports) when `io()` is overridden; an app that
        /// never overrides `io()` still gets no ports (matches the old `Vec::new()` default) since there is
        /// no document media type to synthesize the implicit pair from.
        fn media_ports() -> Vec<MediaPortSpec> {
            Self::io().map(|io| io.all_ports()).unwrap_or_default()
        }
        /// 🎞️ Pure export of the current document onto one declared output port — must not mutate
        /// anything. Called by both the UI (preview/export) and a headless runner (moving media along a
        /// workflow edge). Default: the whole document pack, base64-wrapped, for `"document:out"`;
        /// `MediaError::NotImplemented` for any other port (apps declaring extra output ports override
        /// this to handle them, falling through to `ArtifactApp::export_media`'s default via `_ =>` for
        /// `"document:out"` if desired).
        fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
            let bytes = doc.snapshot.encode_pack();
            Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
        }
        /// 🎞️ Builds the operation that replaces the whole document with `snapshot` — the seam the
        /// default `import_media("document:in")` below needs to turn a decoded document pack into a real,
        /// undoable operation. `None` (the default) means "not implemented" (there is no generic "replace
        /// whole snapshot" operation); an app whose `Mutation` enum has such a variant (e.g.
        /// `SetFixture`/`SetArtifact`) overrides this one-liner to unlock the default `import_media`.
        fn whole_document_operation( _snapshot: Self::Snapshot) -> Option<Self::Mutation> {
            None
        }
        /// 🎞️ Translates an incoming media value on one declared input port into operations — never mutates
        /// state directly, so a headless import is exactly as undoable/syncable as a UI edit. Default:
        /// decodes a `"document:in"` structured (base64 pack) payload via `whole_document_operation`;
        /// `MediaError::NotImplemented` for any other port or when `whole_document_operation` is `None`.
        fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, MediaError> {
            if port != "document:in" {
                return Err(MediaError::NotImplemented);
            }
            let MediaPayload::Structured { json, .. } = &media.payload else {
                return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
            };
            let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            let snapshot = <Self::Snapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            match Self::whole_document_operation(snapshot) {
                Some(operation) => Ok(Emit::mutations(vec![operation])),
                None => Err(MediaError::NotImplemented),
            }
        }
        /// 🎞️ Cheap identity for one output port's current value, without serializing the payload.
        /// Default re-derives it from `export_media`; override when a fingerprint is derivable without
        /// materializing the full export (e.g. from a cached head edit id).
        fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
            Self::export_media(port, doc).map(|media| MediaFingerprint::of(&media))
        }
    }

    /// 🎞️ Rust mirror of WIT's `media-artifact` record (`framework/wit/📜️world.wit`): `descriptor` is the
    /// parsed `descriptor-json`, `data` the sibling raw-bytes field. Deliberately separate from
    /// `mesh::Media` (which pairs a `MediaType` with a `MediaPayload` for the declared-port workflow
    /// via `export_media`/`import_media`) — `consume-media`/`produce-media` operate at the whole-document
    /// level, not a specific `MediaPortSpec`, so `PluginApp::{consume_media, produce_media}` below default
    /// to a document passthrough rather than requiring any `media_ports()` declaration at all.
    #[derive(Clone, Debug)]
    pub struct MediaArtifact {
        pub descriptor: MediaArtifactDescriptor,
        pub data: Vec<u8>,
    }

    /// 🎞️ Rust mirror of WIT's `media-artifact.descriptor-json` JSON shape. `edge_id`/`port_id`/`kind_id`
    /// are dataflow-driver bookkeeping the SDK default below leaves untouched (opaque pass-through for the
    /// caller); `media_type` is the declared `MediaType` the wire claims to satisfy; `wire` is the actual
    /// encoding; `blob_hash` is set instead of inline `data` when the payload already lives in the host's
    /// blob store.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MediaArtifactDescriptor {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub edge_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kind_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub media_type: Option<MediaType>,
        pub wire: MediaWireFormat,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub blob_hash: Option<String>,
    }

    /// 🚧️ Failure producing/consuming a `MediaArtifact` through `PluginApp::{produce_media, consume_media}`.
    #[derive(Debug, thiserror::Error)]
    pub enum MediaArtifactError {
        #[error("{0}")]
        Payload(String),
        #[error("document schema mismatch: expected {expected}, found {found}")]
        SchemaMismatch { expected: String, found: String },
        #[error("no binary importer registered for format {0:?}")]
        NoImporter(String),
    }

    /// @emoji 🗄️ Object-safe runtime contract every hosted app satisfies. Owns persistent document state
    /// (via {@link VcsArtifactApp}'s store) across calls — no per-call document JSON is threaded in.
    /// History actions (undo/redo/checkpoint/alternative) are intercepted by the wrapper; typed
    /// operations are dispatched with real inverses; operations flow to/from the backbone as the wire format.
    pub trait PluginApp: Send {
        fn app_id(&self) -> &str;
        fn document_schema(&self) -> &str;
        /// @emoji 🕰️ FRAMEWORK-reserved action dispatch only (undo/redo/checkpoint/alternative/clipboard/
        /// revert-to-command/history-filter/noteShellCommand) — B1 deleted the generic app-declared-action
        /// fallback this used to carry (`ArtifactApp::handle_action` no longer exists; an app's own
        /// behavior is reached exclusively through `handle_command_frame`'s typed `Self::Command` decode).
        /// An unrecognized `action` id is a hard error pointing at the typed channel.
        fn handle_action(&mut self, action: &str, args: Option<&Value>, meta: &ActionMeta) -> Result<InvocationResult, Fault>;
        /// @emoji 🕰️ Same FRAMEWORK-reserved scope as `handle_action` above — kept as a distinct entry
        /// point for `CommandDefinition`-shaped host calls (a command has no `ActionKind` of its own).
        fn handle_command(&mut self, command: &str, args: Option<&Value>, meta: &ActionMeta) -> Result<InvocationResult, Fault>;
        /// 🎯️ The single dispatch point `plugin_runtime::plugin_exchange` calls for every
        /// `AppCommand::Command` frame: recognizes a framework-reserved `{kind,name,args}` wire-value
        /// envelope (routed through `handle_action`/`handle_command` above) and otherwise decodes
        /// `command_bytes` directly as the app's typed `ArtifactApp::Command` via `OpBinary::decode_op`
        /// and calls the pure `ArtifactApp::handle`.
        fn handle_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta) -> Result<InvocationResult, Fault>;
        /// 🧾 Drains the last Emit op packs captured during `handle_command_frame` (PureCommand path).
        fn take_last_emit_wire(&mut self) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)>;
        /// 📥 Hydrate document lane from host pack bytes (PureCommand / host authority).
        fn hydrate_document_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault>;
        /// 📥 Hydrate config lane from host pack bytes.
        fn hydrate_config_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault>;
        /// 📥 Hydrate draft lane from host pack bytes.
        fn hydrate_draft_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault>;
        /// 🧮️ Object-safe counterpart to `ArtifactApp::Config`'s binary-pack encoding — the config-store
        /// twin of `document_pack` below.
        fn config_pack(&self) -> Result<store::ArtifactPackFiles, Fault>;
        /// 🧮️ Object-safe counterpart to `load_document_pack`, targeting the config store.
        fn load_config_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault>;
        /// 🧸️ Adopts one owned child's persisted envelope into a live child store — the
        /// `AppCommand::LoadChildren` handler. A composing document restores its children through
        /// this, one call per child, after its own `load_document_pack`.
        fn load_child_pack(&mut self, slot: &str, child_id: &str, dialect: store::os_io::ArtifactDialect, envelope_pack: &[u8]) -> Result<(), Fault>;
        /// 🧸️ Every live owned child's current envelope, for persistence — the `ReadChildren`
        /// handler and the child-side twin of `document_pack`.
        fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, Fault>;
        /// 🧮️ Dispatches one binary-encoded `store::ArtifactCommand<Self::ConfigMutation>` against the
        /// config store — the `AppCommand::ConfigCommand` wire frame's real handler (replaces the deleted
        /// `apply_config_bytes` whole-record-replace legacy path).
        fn dispatch_config_command(&mut self, command_bytes: &[u8], meta: &ActionMeta) -> Result<InvocationResult, Fault>;
        /// @emoji 📥️ Ingests binary-encoded remote `MutationEnvelope`s (`protocol::decode_envelopes`)
        /// into the causal DAG (idempotent — duplicate mutation ids are dropped).
        fn ingest_operations(&mut self, mutations: &[u8]) -> Result<(), Fault>;
        /// @emoji 📜️ Text-DSL counterpart to {@link Self::ingest_operations}: applies one already-authored
        /// `Self::Mutation` per non-blank line (via `store::OpText::parse_op`) as a fresh local edit — unlike
        /// the JSON path (which ingests already-caused remote `MutationEnvelope`s into the causal DAG
        /// via `store.ingest_remote`, preserving their original ids/deps), each parsed line here goes
        /// through the normal `ArtifactCommand::Apply` path (a fresh id/timestamp, a real computed
        /// inverse) — the natural mapping for hand-authored or externally-generated op-text, which carries
        /// no envelope metadata of its own.
        fn ingest_operations_text(&mut self, operations_text: &str) -> Result<(), Fault>;
        /// @emoji 📜️ Text-DSL counterpart to {@link Self::document_pack}: the whole document as
        /// {@link store::ArtifactTextFiles} (the `dsl` initial-snapshot text plus the full `ops` op-log
        /// text) via `store::print_document_text` — returned as the established two-file struct rather than
        /// a single concatenated string, since that struct (not an ad hoc delimiter format) is already the
        /// canonical text representation everywhere else in this codebase (`FolderTextStorage`,
        /// `parse_document_text`).
        fn document_text(&self) -> Result<store::ArtifactTextFiles, Fault>;
        /// @emoji 📜️ Text-DSL counterpart to {@link Self::load_document_pack}.
        fn load_document_text(&mut self, files: &store::ArtifactTextFiles) -> Result<(), Fault>;
        /// @emoji 📦️ Binary-pack counterpart to {@link Self::document_text}: the whole document as
        /// {@link store::ArtifactPackFiles} (pack-encoded initial snapshot plus the same `ops` op-log
        /// text — the op grammar is format-invariant) via `store::print_document_pack`.
        fn document_pack(&self) -> Result<store::ArtifactPackFiles, Fault>;
        /// @emoji 📦️ Binary-pack counterpart to {@link Self::load_document_text}.
        fn load_document_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault>;
        fn attach_backbone(&mut self, backbone: Box<dyn store::Backbone>) -> Result<(), Fault>;
        fn detach_backbone(&mut self);
        /// @emoji 🕰️ `view_state` is kept here ONLY for wrapper-owned framework chrome (the injected
        /// history panel body's locale — see `VcsArtifactApp::render`); it is never forwarded into
        /// `ArtifactApp::render`, which dropped `ViewModel` entirely in B1.
        fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel) -> Result<UiNode, Fault>;
        fn window_engagements(&mut self) -> HashMap<String, WindowEngagement> {
            HashMap::new()
        }
        fn window_measures(&mut self) -> HashMap<String, Vec<WindowMeasure>> {
            HashMap::new()
        }
        /// 🛠️ Object-safe counterpart to `ArtifactApp::tool_measures` — keyed by tool id.
        fn tool_measures(&mut self) -> HashMap<String, Vec<WindowMeasure>> {
            HashMap::new()
        }
        /// ⏱️ Object-safe counterpart to `ArtifactApp::pending_effects` — called once per `refreshUi` pass.
        fn pending_effects(&mut self) -> Vec<HostEffect> {
            Vec::new()
        }
        /// 🖱️ Object-safe counterpart to `ArtifactApp::context_menu` — the WIT `context-menu` export's
        /// dispatch target.
        fn context_menu(&mut self, _request: &ContextMenuRequest) -> Vec<ContextMenuItemSpec> {
            Vec::new()
        }
        /// 🎞️ Object-safe counterpart to `ArtifactApp::export_media` — the seam a headless workflow
        /// runner calls without knowing the app's concrete `Snapshot`/`Mutation` types.
        fn export_media(&mut self, _port: &str) -> Result<Media, MediaError> {
            Err(MediaError::NotImplemented)
        }
        /// 🎞️ Object-safe counterpart to `ArtifactApp::import_media` — dispatches through the same
        /// `ArtifactStore` as `handle_action`, so a headless import is an ordinary, undoable edit.
        fn import_media(&mut self, _port: &str, _media: &Media, _meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            Err(plugin_sdk_fault(MediaError::NotImplemented.to_string()))
        }
        fn media_fingerprint(&mut self, _port: &str) -> Result<MediaFingerprint, MediaError> {
            Err(MediaError::NotImplemented)
        }
        /// 🎞️ ABI-level media artifact request for one port (`framework/wit/📜️world.wit`'s `produce-media`).
        /// Default: a whole-document passthrough (`wire: Document{schema: document_schema()}` wrapping
        /// `document_pack()`'s pack+spr bytes via `store::encode_document_pack_bytes`) — the fallback every
        /// `PluginApp` gets for free without declaring any `media_ports()`. Apps whose media output isn't
        /// simply their raw document (computed/derived outputs) override this directly; `port` is accepted
        /// for parity with `export_media` and ignored by the default (there is exactly one document to hand
        /// back).
        fn produce_media(&mut self, port: &str) -> Result<MediaArtifact, MediaArtifactError> {
            let files = self.document_pack().map_err(|fault| MediaArtifactError::Payload(fault.message))?;
            Ok(MediaArtifact {
                descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port.to_string()), kind_id: None, media_type: None, wire: MediaWireFormat::Document { schema: self.document_schema().to_string() }, blob_hash: None },
                data: store::encode_document_pack_bytes(&files.pack, &files.spr),
            })
        }
        /// 🎞️ ABI-level media artifact delivery for one port (`framework/wit/📜️world.wit`'s `consume-media`).
        /// Default: a `Document{schema}` wire matching this app's own `document_schema()` loads straight
        /// through `load_document_pack` — the same pack+spr bytes `read-app-document-pack`/
        /// `load-app-document-pack` already round-trip. Anything else (a foreign document schema, or a
        /// `Binary{format}` wire) has no SDK-level importer registry yet, so the default rejects it; apps
        /// that need one override this method directly.
        fn consume_media(&mut self, _port: &str, artifact: MediaArtifact) -> Result<(), MediaArtifactError> {
            match artifact.descriptor.wire {
                MediaWireFormat::Document { schema } if schema == self.document_schema() => {
                    let (pack, spr) = store::decode_document_pack_bytes(&artifact.data).map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
                    self.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).map_err(|fault| MediaArtifactError::Payload(fault.message))
                }
                MediaWireFormat::Document { schema } => Err(MediaArtifactError::SchemaMismatch { expected: self.document_schema().to_string(), found: schema }),
                MediaWireFormat::Binary { format_kind } => Err(MediaArtifactError::NoImporter(format_kind)),
            }
        }
    }

    /// @emoji 📇️ An app's action declarations indexed by id, built from its {@link AppDefinition}. Threaded
    /// into {@link VcsArtifactApp} at registration time so the wrapper can enforce the actions contract
    /// (default materialization, required-arg validation, kind discipline) without the plugin re-checking.
    /// An empty registry (the test/registry-less construction path) skips all enforcement.
    #[derive(Clone, Default)]
    pub struct AppActionRegistry {
        actions: HashMap<String, ActionDefinition>,
        commands: HashMap<String, CommandDefinition>,
        /// @emoji 📇️ The app's `controller_id` — addresses `ActionDescriptor.controller_id` for the
        /// framework-built history panel. Empty for the registry-less test path.
        controller_id: String,
    }

    impl AppActionRegistry {
        /// @emoji 📇️ Indexes an app definition's declared actions and commands (including
        /// framework-injected ones) by id.
        pub fn from_definition(definition: &AppDefinition) -> Self {
            Self {
                actions: definition.actions.iter().map(|action| (action.id.clone(), action.clone())).collect(),
                commands: definition.commands.iter().map(|command| (command.id.clone(), command.clone())).collect(),
                controller_id: definition.controller_id.clone(),
            }
        }

        fn get(&self, id: &str) -> Option<&ActionDefinition> {
            self.actions.get(id)
        }

        fn get_command(&self, id: &str) -> Option<&CommandDefinition> {
            self.commands.get(id)
        }

        /// 🗂️ Ribbon-parent-taxonomy category (a `ui_wgpu::wgpu::RIBBON_PARENT_CATEGORIES` id) for a declared
        /// action id — the `organize_context_menu` `category_of` lookup at the `VcsArtifactApp::context_menu`
        /// funnel. `None` for a command id (`CommandDefinition.category` is an unrelated footer-tab
        /// grouping, not this taxonomy) and for any action that never called `.with_category(...)`.
        pub fn category_of(&self, id: &str) -> Option<String> {
            self.actions.get(id).and_then(|action| action.category.clone())
        }
    }

    //#region 🖱️MenuBuilder
    /// 🖱️ Ergonomic `ContextMenuItemSpec` builder for `ArtifactApp::context_menu` — resolves
    /// label/icon from this app's declared `ActionDefinition`/`CommandDefinition`s (via the same
    /// `AppActionRegistry` `VcsArtifactApp` already enforces the actions contract with) so plugins stop
    /// restating them in hand-rolled `serde_json::json!` blobs. Shortcuts are deliberately left unset —
    /// the host enriches them from the keybinding registry at menu-open time (`mapContextMenuSpecs`),
    /// exactly as it already does for every existing emitter.
    ///
    /// ```ignore
    /// Menu::of(&registry)
    ///     .action("deleteSelection")
    ///     .separator()
    ///     .checked("toggleGrid", grid_on)
    ///     .when(has_selection, |m| m.destructive("clearSelection"))
    ///     .submenu("align", "Align", |m| m.action("alignLeft").action("alignRight"))
    ///     .build()
    /// ```
    pub struct Menu<'a> {
        registry: &'a AppActionRegistry,
        items: Vec<ContextMenuItemSpec>,
    }

    impl<'a> Menu<'a> {
        pub fn of(registry: &'a AppActionRegistry) -> Self {
            Self { registry, items: Vec::new() }
        }

        /// 🎯️ Appends a row for a declared action id, resolving `label`/`icon` from the app's
        /// `ActionDefinition`. An unresolvable id is dropped with a debug-mode panic (a construction-time
        /// typo, the same enforcement style as `AppBuilder::build_definition`'s ref validation) — in
        /// release builds it is silently skipped so a stale reference never hard-crashes production.
        pub fn action(self, action_id: impl Into<String>) -> Self {
            self.action_with_args(action_id, None)
        }

        pub fn action_args(self, action_id: impl Into<String>, args: Value) -> Self {
            self.action_with_args(action_id, Some(to_dsl_value(&args).expect("menu action args must convert to DslValue")))
        }

        fn action_with_args(mut self, action_id: impl Into<String>, args: Option<DslValue>) -> Self {
            let action_id = action_id.into();
            match self.registry.get(&action_id) {
                Some(definition) => {
                    self.items.push(ContextMenuItemSpec {
                        id: action_id.clone(),
                        // 🚧️ `ArtifactApp::context_menu` carries no `ViewModel` (dropped entirely in B1), so
                        // there is no locale/terminology to resolve against here — hardcoded to
                        // native/English pending a protocol change to thread the active axes through
                        // context-menu construction. Flagged as a follow-up, not fixed in this pass.
                        label: Some(definition.label.resolve(Terminology::Native, Locale::En).to_string()),
                        icon: Some(definition.icon_id.as_str().to_string()),
                        action: Some(action_id),
                        args,
                        ..Default::default()
                    });
                }
                None => debug_assert!(false, "Menu::action: unknown action id {action_id:?}"),
            }
            self
        }

        /// 🎛️ Appends a row for a declared command id (os/plugin/app/mode-scoped) — same resolution
        /// discipline as `action`, against `AppActionRegistry::get_command`.
        pub fn command(mut self, command_id: impl Into<String>) -> Self {
            let command_id = command_id.into();
            match self.registry.get_command(&command_id) {
                Some(definition) => {
                    self.items.push(ContextMenuItemSpec {
                        id: command_id.clone(),
                        // 🚧️ See the identical note in `action_with_args` above — no locale context reaches
                        // context-menu construction yet.
                        label: Some(definition.label.resolve(Terminology::Native, Locale::En).to_string()),
                        icon: Some(definition.icon_id.as_str().to_string()),
                        action: Some(command_id),
                        ..Default::default()
                    });
                }
                None => debug_assert!(false, "Menu::command: unknown command id {command_id:?}"),
            }
            self
        }

        /// 🎯️ Same as `action`, with `checked` set — for a toggleable verb (e.g. "Show grid").
        pub fn checked(mut self, action_id: impl Into<String>, checked: bool) -> Self {
            self = self.action(action_id);
            if let Some(last) = self.items.last_mut() {
                last.checked = Some(checked);
            }
            self
        }

        /// 🎯️ Same as `action`, with `destructive` set — sorts visually distinct and last in
        /// `ContextMenuController`'s rendering.
        pub fn destructive(mut self, action_id: impl Into<String>) -> Self {
            self = self.action(action_id);
            if let Some(last) = self.items.last_mut() {
                last.destructive = Some(true);
            }
            self
        }

        /// 🎯️ Same as `action`, with `disabled` set — keeps a verb visible-but-inert (discoverability)
        /// rather than omitting it outright.
        pub fn disabled(mut self, action_id: impl Into<String>, disabled: bool) -> Self {
            self = self.action(action_id);
            if let Some(last) = self.items.last_mut() {
                last.disabled = Some(disabled);
            }
            self
        }

        /// 🧩️ Appends a fully custom row (dynamic label/hover actions/data-driven lists — the escape
        /// hatch for rows that don't map onto one declared action, e.g. a per-candidate suggestion row).
        pub fn item(mut self, item: ContextMenuItemSpec) -> Self {
            self.items.push(item);
            self
        }

        pub fn separator(mut self) -> Self {
            self.items.push(ContextMenuItemSpec { id: format!("separator-{}", self.items.len()), separator: Some(true), ..Default::default() });
            self
        }

        /// 🌿️ Appends a nested submenu, its rows built by a fresh `Menu` sharing this app's registry.
        pub fn submenu(mut self, id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<IconName>, build: impl FnOnce(Menu<'a>) -> Menu<'a>) -> Self {
            let children = build(Menu::of(self.registry)).build();
            self.items.push(ContextMenuItemSpec { id: id.into(), label: Some(label.into()), icon: Some(icon_id.into().as_str().to_string()), children: (!children.is_empty()).then_some(children), ..Default::default() });
            self
        }

        /// 🗂️ Appends a taxonomy group row (`menu.group.<category>`, `label: None` — the host resolves the
        /// localized label via `ui_wgpu::wgpu::ribbon_parent_label`) built by a fresh `Menu` sharing this app's
        /// registry. Unlike `submenu`, a group's id/label are not bespoke: `organize_context_menu` (run at
        /// the `VcsArtifactApp::context_menu` funnel) merges every row sharing the same category across the
        /// whole level, dedupes their children by id, and orders groups by the canonical
        /// `RIBBON_PARENT_CATEGORIES` taxonomy — see D3/the canonical migration pattern in the
        /// grouped-context-menu mechanism design.
        pub fn group(mut self, category: impl Into<String>, build: impl FnOnce(Menu<'a>) -> Menu<'a>) -> Self {
            let children = build(Menu::of(self.registry)).build();
            self.items.push(ContextMenuItemSpec { id: format!("menu.group.{}", category.into()), label: None, children: (!children.is_empty()).then_some(children), ..Default::default() });
            self
        }

        /// 🔀️ Conditionally applies `build` to the menu so far — the idiomatic way to gate a section on
        /// a guard (selection kind, hover target, ...) without breaking the fluent chain.
        pub fn when(self, condition: bool, build: impl FnOnce(Self) -> Self) -> Self {
            if condition {
                build(self)
            } else {
                self
            }
        }

        pub fn build(self) -> Vec<ContextMenuItemSpec> {
            self.items
        }
    }

    /// 🗣️ Localized conjunction phrase for selection-scoped menu labels — `(count, singular, plural)` per domain.
    pub fn selection_count_phrase(is_de: bool, counts: &[(usize, &str, &str)]) -> String {
        let parts: Vec<String> = counts.iter().filter(|(count, _, _)| *count > 0).map(|(count, singular, plural)| if *count == 1 { format!("1 {singular}") } else { format!("{count} {plural}") }).collect();
        match parts.len() {
            0 => String::new(),
            1 => parts[0].clone(),
            2 => {
                if is_de {
                    format!("{} und {}", parts[0], parts[1])
                } else {
                    format!("{} and {}", parts[0], parts[1])
                }
            }
            _ => {
                let joiner = if is_de { " und " } else { ", and " };
                let last = parts.last().cloned().unwrap_or_default();
                let head = parts[..parts.len() - 1].join(", ");
                if is_de {
                    format!("{head} und {last}")
                } else {
                    format!("{head}{joiner}{last}")
                }
            }
        }
    }
    //#endregion 🖱️MenuBuilder

    /// 🎯️ Node and edge ids from a context-menu surface snapshot, falling back to runtime selection.
    pub fn selection_domains_from_surface(surface: Option<&ContextMenuSurfaceTarget>, fallback_nodes: &[String], fallback_edges: &[String]) -> (Vec<String>, Vec<String>) {
        let groups = surface.map(|target| target.selection.as_slice()).unwrap_or(&[]);
        let mut nodes: Vec<String> = groups.iter().filter(|group| group.domain == "node").flat_map(|group| group.ids.iter().cloned()).collect();
        let mut edges: Vec<String> = groups.iter().filter(|group| group.domain == "edge").flat_map(|group| group.ids.iter().cloned()).collect();
        if nodes.is_empty() && edges.is_empty() {
            nodes = fallback_nodes.to_vec();
            edges = fallback_edges.to_vec();
        }
        (nodes, edges)
    }

    /// 🗑️ How delete-selection is dispatched from a node-graph context menu row.
    pub enum NodeGraphDeleteDispatch {
        /// `deleteSelection` view action (flow and similar).
        Direct,
        /// `nodeGraphEdit` with a `deleteSelection` operation (dag, sequence, procedural).
        ViaNodeGraphEdit,
    }

    /// 🗑️ Delete-selection row with a localized count phrase — omitted when the selection is empty.
    pub fn node_graph_delete_selection_spec(delete_label: &str, is_de: bool, node_count: usize, edge_count: usize, dispatch: NodeGraphDeleteDispatch) -> Option<ContextMenuItemSpec> {
        if node_count == 0 && edge_count == 0 {
            return None;
        }
        let phrase = selection_count_phrase(is_de, &[(node_count, if is_de { "Knoten" } else { "node" }, if is_de { "Knoten" } else { "nodes" }), (edge_count, if is_de { "Kante" } else { "edge" }, if is_de { "Kanten" } else { "edges" })]);
        if phrase.is_empty() {
            return None;
        }
        let (action, args) = match dispatch {
            NodeGraphDeleteDispatch::Direct => ("deleteSelection".into(), None),
            NodeGraphDeleteDispatch::ViaNodeGraphEdit => ("nodeGraphEdit".into(), Some(to_dsl_value(&serde_json::json!({ "operations": [{ "operation": "deleteSelection" }] })).expect("delete menu args"))),
        };
        Some(ContextMenuItemSpec { id: "delete-selection".into(), label: Some(format!("{delete_label} ({phrase})")), icon: Some("trash".into()), destructive: Some(true), action: Some(action), args, ..Default::default() })
    }

    /// @emoji 🧬️ Generic wrapper turning any typed {@link ArtifactApp} into the object-safe runtime
    /// {@link PluginApp}. Owns a persistent `ArtifactStore<Snapshot, Mutation>` — the single source of
    /// truth for the app's document across every call — intercepts the six injected history actions into
    /// `ArtifactCommand`s, dispatches `Apply`/`AmendLast` for typed operations, and builds an
    /// `InvocationResult` whose inverses come from the just-recorded `Edit.inverse`. A snapshot+history
    /// cache keyed on `(store generation, log generation, history filter)` keeps renders O(1). Holds an
    /// {@link AppActionRegistry} to enforce the actions contract before/after delegating to the app.
    /// Host `ArtifactSession` already mirrors opaque document/config/draft packs; typed stores here
    /// remain guest-owned until `AppCommand::PureCommand` returns `AppFrame::Emit` for host apply.
    pub struct VcsArtifactApp<A: ArtifactApp> {
        app: A,
        pub(crate) store: ArtifactStore<A::Snapshot, A::Mutation>,
        config_store: ConfigStore<A::Config, A::ConfigMutation>,
        /// @emoji 📝️ Volatile draft lane — never checkpoints; prune via `ArtifactCommand::PruneDrafts`.
        /// Moves to host `ArtifactSession` when CHANNEL_VERSION 5 exchange lands.
        draft_store: store::DraftStore<A::Draft, A::DraftMutation>,
        /// @emoji 👥️ Ephemeral SHARED lane — a last-writer-wins peer roster, not an event log. Holds
        /// this actor's own presence plus whatever peers last broadcast; never persisted, never
        /// checkpointed, never undoable.
        pub(crate) presence_store: store::PresenceStore<A::Presence, A::PresenceMutation>,
        /// @emoji 🫧️ Ephemeral LOCAL-ONLY lane — typed UI state that never leaves this client and
        /// never becomes document content. The typed home for what used to live in plugin
        /// `thread_local!`s.
        pub(crate) transient_store: store::TransientStore<A::Transient, A::TransientMutation>,
        /// 🧾 Last Emit op packs produced by `dispatch_emit` — consumed by `AppCommand::PureCommand`.
        last_emit_wire: Option<(Vec<u8>, Vec<u8>, Vec<u8>)>,
        /// @emoji 🗂️ Keyed on `(store.generation(), log_generation, history_filter)` — any of the three
        /// changing invalidates the cached snapshot/`HistoryView` pair.
        cache: Option<((u64, u64, u64, HistoryCommandFilter), A::Snapshot, A::Config, HistoryView)>,
        registry: AppActionRegistry,
        /// @emoji 🧾️ Append-only session command log — see `🔖️CommandLog`. Never persisted, never
        /// truncated: undo/redo/revert push entries, they never remove any.
        command_log: Vec<CommandLogEntry>,
        next_command_seq: u64,
        /// @emoji 🗂️ Bumped by `push_log_entry`/`record_command` on every log mutation (a push OR a fold)
        /// — part of the cache key so a folded ×count bump alone (no store-generation change) still
        /// invalidates a stale render.
        log_generation: u64,
        history_filter: HistoryCommandFilter,
        /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): every owned child's LIVE store, keyed by
        /// `(slot, child_id)` — mirrors how `store`/`config_store`/`draft_store` above each hold one
        /// lane's live `ArtifactStore`, generalized to N children instead of one fixed lane. Each
        /// entry also carries the dialect captured at open/genesis time (`SpaceMember` itself has no
        /// object-safe dialect getter — see `dispatch_emit_group`'s own doc comment), so a
        /// `store::ChildDispatch`/`store::ArtifactRef` can be rebuilt for it without re-deriving one.
        pub(crate) children: HashMap<(String, String), (store::os_io::ArtifactDialect, Box<dyn SpaceMember>)>,
        /// 📌️ Checkout pins for children that were NOT open when a checkpoint cascade ran. Draining
        /// this on `open_child` is what keeps a lazily-adopted child from silently sitting at head
        /// while the rest of the composition sits at a pinned checkpoint — the alternative (dropping
        /// the pin) would make a checked-out composition quietly inconsistent, which is exactly the
        /// class of bug the cascade exists to prevent.
        pub(crate) pending_child_pins: Vec<vcs::CompositionPin>,
        /// 🧩️ STATEFUL by design (owns the incrementally-maintained `store::CompositionGraph`) — see
        /// `store::CompositionCoordinator`'s own doc comment. One coordinator per `VcsArtifactApp`
        /// instance (i.e. per document), exactly like `store`/`config_store`/`draft_store` are
        /// per-instance rather than global.
        pub(crate) composition: CompositionCoordinator,
    }

    /// 🆔️ Deterministic session-local `ArtifactHandle` for a CHILD's real (string) artifact id.
    /// `result_from_last_edit`'s own `ArtifactHandle(meta.instance_id as u128)` only identifies the
    /// ONE wasm-hosted document this plugin instance runs (the parent) — a child pulled in through
    /// `dispatch_emit_group` is a different document with its own string `artifact_id` and no
    /// instance id of its own, so `KernelMutation.document` needs a handle derived from THAT id
    /// instead, so history correctly attributes a child's edit to the child, not the parent. FNV-1a
    /// (128-bit, two interleaved 64-bit lanes) rather than `std::collections::hash_map::DefaultHasher`
    /// — the latter's hash is only guaranteed stable for one process/compiler invocation, not the
    /// byte-identical value the SAME child id must always map to.
    pub(crate) fn artifact_handle_of(artifact_id: &str) -> ArtifactHandle {
        const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
        const PRIME: u64 = 0x0000_0100_0000_01b3;
        let mut lane0 = OFFSET;
        let mut lane1 = OFFSET ^ 0x9e37_79b9_7f4a_7c15;
        for byte in artifact_id.as_bytes() {
            lane0 = (lane0 ^ *byte as u64).wrapping_mul(PRIME);
            lane1 = (lane1 ^ (*byte as u64).rotate_left(7)).wrapping_mul(PRIME);
        }
        ArtifactHandle(((lane0 as u128) << 64) | lane1 as u128)
    }

    const HISTORY_ACTION_IDS: [&str; 6] = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"];

    const CLIPBOARD_ACTION_IDS: [&str; 3] = ["copy", "cut", "paste"];

    impl<A: ArtifactApp> VcsArtifactApp<A> {
        /// @emoji 🧬️ Constructs a wrapper with an empty registry — contract enforcement is skipped. Used by
        /// tests and any registry-less construction path.
        pub fn new(app: A) -> Self {
            Self::with_registry(app, AppActionRegistry::default())
        }

        /// @emoji 🧬️ Constructs a wrapper carrying the app's {@link AppActionRegistry} so `handle_action`
        /// enforces default materialization, required-arg validation, and kind discipline.
        pub fn with_registry(app: A, registry: AppActionRegistry) -> Self {
            let envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, A::APP_ID, A::initial_snapshot(), None);
            let config_id = format!("{}-config", A::APP_ID);
            let config_envelope = create_config_envelope::<A::Config, A::ConfigMutation>(A::config_schema(), &config_id, A::initial_config(), None);
            let draft_id = format!("{}-draft", A::APP_ID);
            let draft_envelope = create_document_envelope::<A::Draft, A::DraftMutation>("draft.empty", &draft_id, A::initial_draft(), None);
            let mut store = ArtifactStore::new(envelope);
            let config_store = ConfigStore::new(config_envelope);
            let draft_store = store::DraftStore::new(draft_envelope);
            let genesis_mutations = A::genesis();
            if !genesis_mutations.is_empty() {
                store
                    .dispatch(ArtifactCommand::Apply { mutations: genesis_mutations, description: Some("genesis".to_string()) })
                    .expect("ArtifactApp::genesis mutations must apply cleanly onto a freshly constructed store");
            }
            Self {
                app,
                store,
                config_store,
                draft_store,
                presence_store: store::PresenceStore::new(A::Presence::default()),
                transient_store: store::TransientStore::new(A::Transient::default()),
                last_emit_wire: None,
                cache: None,
                registry,
                command_log: Vec::new(),
                next_command_seq: 0,
                log_generation: 0,
                history_filter: HistoryCommandFilter::default(),
                children: HashMap::new(),
                pending_child_pins: Vec::new(),
                composition: CompositionCoordinator::new(),
            }
        }

        /// 🌱️ Opens an already-persisted child store for `slot`/`child_id` via the registered
        /// `store::ChildStoreFactory` for `dialect.artifact_kind` — the mechanism Task 2 names:
        /// "wiring to the `ChildStoreFactory` registry so children can be created/opened by artifact
        /// kind". `envelope_pack` is the child's own full envelope pack (`ChildStoreFactory::open`'s
        /// own contract — it self-describes id/history, unlike `create`'s bare `initial_pack`); the
        /// caller supplies `dialect` regardless, since no object-safe `SpaceMember` getter can read it
        /// back out (see the `children` field's own doc comment). Also seeds the ownership edge into
        /// `self.composition`'s graph immediately, since `dispatch_group`'s phase 1 ownership check
        /// (`CompositionGraph::owner_of`) is fail-closed — a child dispatched against before being
        /// registered here (or absorbed from a genesis `GroupReceipt`, see `dispatch_emit_group`) is
        /// rejected as an `OwnershipViolation`, not silently accepted.
        pub fn open_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: store::os_io::ArtifactDialect, envelope_pack: &[u8]) -> Result<(), Fault> {
            let slot = slot.into();
            let child_id = child_id.into();
            let kind = ArtifactKindId::parse(&dialect.artifact_kind).map_err(plugin_sdk_fault)?;
            let factory = child_store_factory(&kind).ok_or_else(|| plugin_sdk_fault(format!("open_child: no ChildStoreFactory registered for kind {}", dialect.artifact_kind)))?;
            let mut member = factory.open(envelope_pack).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            let parent_id = self.store.envelope().id.clone();
            self.composition.graph_mut().insert_owns(&parent_id, &slot, &child_id).map_err(plugin_sdk_fault)?;
            // 📌️ Drain any checkout pin this child missed by not being open at cascade time.
            if let Some(index) = self.pending_child_pins.iter().position(|pin| pin.child_ref.artifact_id == child_id) {
                let pin = self.pending_child_pins.remove(index);
                let alternative_id = member.current_alternative_id().unwrap_or_default();
                let _ = member.checkout(&pin.checkpoint_id, &alternative_id);
            }
            self.children.insert((slot, child_id), (dialect, member));
            Ok(())
        }

        /// 🔎️ The live child store for `(slot, child_id)`, if adopted/created — the read half of
        /// `open_child`/`register_child`.
        pub fn child_store(&self, slot: &str, child_id: &str) -> Option<&dyn SpaceMember> {
            self.children.get(&(slot.to_string(), child_id.to_string())).map(|(_, member)| member.as_ref())
        }

        /// 🌱️ Adopts an already-live `Box<dyn SpaceMember>` into the child-store map directly — the
        /// counterpart `dispatch_emit_group` uses to absorb a `store::GroupReceipt::created_children`
        /// entry (a `ChildGenesis`-minted member has no prior caller-held reference an `open_child`
        /// call could have produced it from) and the general escape hatch for a caller that already
        /// holds a constructed member from elsewhere (e.g. a host-level `SpaceHost`).
        /// 🌱️ Adopts an already-built child store under `(slot, child_id)`. Like [`Self::open_child`]
        /// it MUST seed the ownership edge into `self.composition`'s graph: `dispatch_group`'s phase 1
        /// ownership check (`CompositionGraph::owner_of`) is fail-closed, so a child present in
        /// `self.children` but absent from the graph is rejected as an `OwnershipViolation` the first
        /// time anything dispatches against it. Registering in one place and validating from the other
        /// is precisely the inconsistency this seeds away.
        pub fn register_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: store::os_io::ArtifactDialect, member: Box<dyn SpaceMember>) -> Result<(), Fault> {
            let slot = slot.into();
            let child_id = child_id.into();
            let parent_id = self.store.envelope().id.clone();
            self.composition.graph_mut().insert_owns(&parent_id, &slot, &child_id).map_err(plugin_sdk_fault)?;
            self.children.insert((slot, child_id), (dialect, member));
            Ok(())
        }

        //#region 🔖️CheckpointCascade
        /// @emoji 📌️ Leaves-first half of a composing document's checkpoint: commits every DIRTY
        /// child (a clean child is already pinned at its current checkpoint, and committing it again
        /// would mint an empty checkpoint per parent commit) and returns the `CompositionPin`s the
        /// parent's own about-to-be-created checkpoint must carry.
        ///
        /// A child with no checkpoint at all after committing contributes no pin rather than
        /// aborting the parent's checkpoint: a pin that named nothing would be worse than an absent
        /// one, and the parent's history is still perfectly valid without it.
        fn commit_children_for_checkpoint(&mut self, message: Option<String>, authors: Vec<vcs::Author>) -> Result<Vec<vcs::CompositionPin>, Fault> {
            let message = message.unwrap_or_else(|| "checkpoint".to_string());
            let mut pins = Vec::new();
            for ((_, child_id), (dialect, member)) in self.children.iter_mut() {
                if member.is_dirty() {
                    member.commit_checkpoint(message.clone(), authors.clone()).map_err(|error| error.into_fault())?;
                }
                if let Some(checkpoint_id) = member.current_checkpoint_id() {
                    pins.push(vcs::CompositionPin { child_ref: store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect: dialect.clone() }, checkpoint_id });
                }
            }
            pins.sort_by(|left, right| left.child_ref.artifact_id.cmp(&right.child_ref.artifact_id));
            Ok(pins)
        }

        /// @emoji 📌️ Records `pins` on the checkpoint the parent's dispatch just created. Runs AFTER
        /// that dispatch because the checkpoint does not exist until then, and `composition_pins`
        /// participates in `content_addressed_checkpoint_id`, so the pins must be present on the
        /// envelope that gets persisted.
        fn stamp_checkpoint_composition_pins(&mut self, pins: Vec<vcs::CompositionPin>) {
            if pins.is_empty() {
                return;
            }
            if let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) {
                self.store.set_checkpoint_composition_pins(&checkpoint_id, pins);
            }
        }

        /// @emoji ⏮️ Checkout half of the cascade: restores every live child to the checkpoint the
        /// parent's now-current checkpoint pinned it at. A pin naming a child that is not currently
        /// open is QUEUED (`pending_child_pins`) rather than dropped, so a child adopted later still
        /// lands on its pinned state instead of silently staying at head — see `open_child`.
        fn cascade_checkout_to_children(&mut self) {
            let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) else { return };
            let Some(pins) = self.store.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).map(|checkpoint| checkpoint.composition_pins.clone()) else { return };
            self.pending_child_pins.clear();
            for pin in pins {
                match self.children.iter_mut().find(|((_, child_id), _)| *child_id == pin.child_ref.artifact_id) {
                    Some((_, (_, member))) => {
                        let alternative_id = member.current_alternative_id().unwrap_or_default();
                        let _ = member.checkout(&pin.checkpoint_id, &alternative_id);
                    }
                    None => self.pending_child_pins.push(pin),
                }
            }
        }
        //#endregion 🔖️CheckpointCascade

        #[cfg(test)]
        pub(crate) fn test_snapshot(&self) -> A::Snapshot {
            self.store.snapshot().expect("materialize snapshot")
        }

        /// @emoji 🧪️ The document store itself — needed to assert on checkpoint metadata
        /// (`composition_pins`), which no snapshot-level accessor exposes.
        #[cfg(test)]
        pub(crate) fn test_store(&self) -> &store::ArtifactStore<A::Snapshot, A::Mutation> {
            &self.store
        }

        /// @emoji 🧪️ The config-store twin of `test_snapshot`.
        #[cfg(test)]
        pub(crate) fn test_config(&self) -> A::Config {
            self.config_store.snapshot().expect("materialize config snapshot")
        }

        /// @emoji 🧪️ Direct access to the wrapped app — used to assert on app-private test fixtures (e.g.
        /// `TestApp::received_actions`) that a framework-owned interception must never populate.
        #[cfg(test)]
        pub(crate) fn test_app(&self) -> &A {
            &self.app
        }

        /// @emoji 🧪️ Refreshes (backfilling the command log) and returns the current `HistoryView` —
        /// the merged command+operation timeline test harness accessor.
        #[cfg(test)]
        pub(crate) fn test_history(&mut self) -> HistoryView {
            self.refresh_cache().expect("refresh cache");
            self.build_history_view()
        }

        /// @emoji 📸️ Materializes and returns the current snapshot — the typed counterpart to
        /// `render`'s `UiNode` output, for callers (host code, downstream plugin crates' own tests) that
        /// need direct structural access to document state instead of a rendered node.
        pub fn snapshot(&self) -> Result<A::Snapshot, Fault> {
            self.store.snapshot().map_err(|error| error.into_fault())
        }

        /// @emoji 🤝️ Fresh replay plus whatever `Mutation::reconcile` reports for the result — the typed
        /// counterpart to `store::ArtifactStore::snapshot_with_conflicts`.
        pub fn snapshot_with_conflicts(&self) -> Result<(A::Snapshot, Vec<SpaceConflict>), Fault> {
            self.store.snapshot_with_conflicts().map_err(|error| error.into_fault())
        }

        /// @emoji 🔗️ The store's current backbone descriptor, `None` when unattached (the default).
        pub fn backbone_ref(&self) -> Option<&store::ArtifactBackboneRef> {
            self.store.backbone_ref()
        }

        /// @emoji 🧾️ Appends one entry to the session command log. `timestamp: None` stamps "now"
        /// (live dispatch); `Some(..)` preserves an edit's original `started_at` (backfill). Always a
        /// fresh row (`count: 1`) — folding consecutive `View`/`Shell` dispatches is `record_command`'s job.
        fn push_log_entry(&mut self, action_id: &str, label: String, kind: ActionKind, edit_id: Option<String>, config_edit_id: Option<String>, timestamp: Option<String>, inverse: Option<InverseAction>) {
            self.next_command_seq += 1;
            self.command_log.push(CommandLogEntry {
                seq: self.next_command_seq,
                action_id: action_id.to_string(),
                label,
                kind,
                timestamp: timestamp.unwrap_or_else(store::now_iso),
                edit_id,
                config_edit_ids: config_edit_id.into_iter().collect(),
                child_edit_ids: Vec::new(),
                count: 1,
                inverse,
            });
            self.log_generation += 1;
        }

        /// @emoji 🧾️ The single entry point every live dispatch logs through (`push_log_entry` remains for
        /// backfill, which never folds). Consecutive `View`/`Shell` dispatches of the SAME `(action_id,
        /// kind)` with no `edit_id` fold into one row — its `count` increments and its `label`/`timestamp`
        /// refresh, but its ORIGINAL `seq` is kept so the panel's tree-item id stays stable across
        /// re-renders. `Mutation`/`History`/`Clipboard` entries and anything with an `edit_id` never fold.
        /// `inverse` (computed from state BEFORE this dispatch) is only stored on a FRESH row — a folded
        /// row keeps its original inverse, since inverse on a folded "×N" row must undo the whole run,
        /// not just the last dispatch that folded into it.
        fn record_command(&mut self, action_id: &str, kind: ActionKind, label: Option<String>, edit_id: Option<String>, config_edit_id: Option<String>, inverse: Option<InverseAction>) {
            // 🚧️ Same locale-context gap as `Menu::action_with_args` — history log entries are recorded
            // without a `ViewModel`, so a fallback label resolves native/English pending a protocol change.
            let label = label
                .or_else(|| self.registry.get(action_id).map(|def| def.label.resolve(Terminology::Native, Locale::En).to_string()))
                .or_else(|| self.registry.get_command(action_id).map(|def| def.label.resolve(Terminology::Native, Locale::En).to_string()))
                .unwrap_or_else(|| action_id.to_string());
            let folds = edit_id.is_none() && matches!(kind, ActionKind::View | ActionKind::Shell) && self.command_log.last().is_some_and(|last| last.action_id == action_id && last.kind == kind && last.edit_id.is_none());
            if folds {
                let last = self.command_log.last_mut().expect("checked above");
                last.count += 1;
                last.label = label;
                last.timestamp = store::now_iso();
                // 🧮️ APPEND (never overwrite) — a folded row accumulates one distinct config edit per
                // tick (each tick's config dispatch is a plain `Apply`, not an `AmendLast`); overwriting
                // would drop earlier ticks' edit ids from `config_edit_ids`, making `backfill_command_log`
                // wrongly think they were never logged and re-append them as phantom rows.
                if let Some(id) = config_edit_id {
                    last.config_edit_ids.push(id);
                }
                self.log_generation += 1;
                return;
            }
            self.push_log_entry(action_id, label, kind, edit_id, config_edit_id, None, inverse);
        }

        /// @emoji 🕰️ Appends a command-log entry for every VCS edit not yet referenced by the log —
        /// covers seeded (`app.seed`), ingested (`ingest_operations*`), and loaded
        /// (`load_document_text`/`load_document_pack`) edits that never passed through `dispatch_emit`.
        /// Invariant: after this runs, every `envelope.vcs.edits` entry is referenced by exactly one
        /// `CommandLogEntry`. Idempotent — re-running finds nothing missing. Always `push_log_entry`
        /// (never `record_command`) — a backfilled edit is always its own distinct row, never folded.
        fn backfill_command_log(&mut self) {
            let logged: HashSet<&str> = self.command_log.iter().filter_map(|entry| entry.edit_id.as_deref()).collect();
            let missing: Vec<(String, String, String)> = self
                .store
                .envelope()
                .vcs
                .edits
                .iter()
                .filter(|edit| !logged.contains(edit.id.as_str()))
                .map(|edit| {
                    let label = edit.description.clone().unwrap_or_else(|| edit.forwards.first().map(OpText::print_op).unwrap_or_else(|| edit.id.clone()));
                    (edit.id.clone(), label, edit.started_at.clone())
                })
                .collect();
            for (edit_id, label, timestamp) in missing {
                self.push_log_entry("apply", label, ActionKind::Mutation, Some(edit_id), None, Some(timestamp), None);
            }
            // 🧮️ Same backfill, for the CONFIG store's own edits — a config edit reached via `seed`/
            // `load_config_pack`/ingest never passes through `dispatch_emit` either.
            let logged_config: HashSet<&str> = self.command_log.iter().flat_map(|entry| entry.config_edit_ids.iter().map(String::as_str)).collect();
            let missing_config: Vec<(String, String, String)> = self
                .config_store
                .envelope()
                .vcs
                .edits
                .iter()
                .filter(|edit| !logged_config.contains(edit.id.as_str()))
                .map(|edit| {
                    let label = edit.description.clone().unwrap_or_else(|| edit.forwards.first().map(OpText::print_op).unwrap_or_else(|| edit.id.clone()));
                    (edit.id.clone(), label, edit.started_at.clone())
                })
                .collect();
            for (config_edit_id, label, timestamp) in missing_config {
                self.push_log_entry("configApply", label, ActionKind::Mutation, None, Some(config_edit_id), Some(timestamp), None);
            }
        }

        fn build_history_view(&self) -> HistoryView {
            let applied_ids: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();
            let local_actor = self.store.local_actor_id();
            let config_applied_ids: HashSet<&str> = self.config_store.applied_edit_ids().iter().map(String::as_str).collect();
            let config_local_actor = self.config_store.local_actor_id();
            let mut commands: Vec<CommandView> = self
                .command_log
                .iter()
                .map(|entry| {
                    let edit = entry.edit_id.as_deref().and_then(|edit_id| self.store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id));
                    let op_lines = edit.map(|edit| edit.forwards.iter().map(OpText::print_op).collect()).unwrap_or_default();
                    // 🪞️ Mirrors `store::ArtifactStore::edit_is_local` (private to that crate): an edit
                    // with no recorded actor is treated as local, same as a real undo would.
                    let applied = entry.edit_id.as_deref().is_some_and(|edit_id| applied_ids.contains(edit_id));
                    let latest_config_edit_id = entry.config_edit_ids.last().map(String::as_str);
                    let config_edit = latest_config_edit_id.and_then(|edit_id| self.config_store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id));
                    let config_applied = latest_config_edit_id.is_some_and(|edit_id| config_applied_ids.contains(edit_id));
                    // ⏪️ Three disjoint ways a row earns "inverse": document edit-linked (applied +
                    // locally authored), config edit-linked (same, on the config store — B1's replacement
                    // for the old memory-only "View"-kind inverse), or memory-only (a stored
                    // `InverseAction`, the remaining `Shell`-kind `noteShellCommand` path).
                    let revertible = (applied && edit.is_some_and(|edit| edit.actor.is_none() || edit.actor.as_deref() == local_actor))
                        || (config_applied && config_edit.is_some_and(|edit| edit.actor.is_none() || edit.actor.as_deref() == config_local_actor))
                        || entry.inverse.is_some();
                    CommandView {
                        seq: entry.seq,
                        action_id: entry.action_id.clone(),
                        label: entry.label.clone(),
                        kind: entry.kind,
                        timestamp: entry.timestamp.clone(),
                        edit_id: entry.edit_id.clone(),
                        config_edit_id: latest_config_edit_id.map(str::to_string),
                        child_edit_ids: entry.child_edit_ids.clone(),
                        op_lines,
                        applied,
                        revertible,
                        count: entry.count,
                        inverse: entry.inverse.clone(),
                    }
                })
                .collect();
            commands.reverse();
            HistoryView {
                columns: build_history_columns(self.store.envelope()),
                can_undo: !self.store.applied_edit_ids().is_empty(),
                can_redo: !self.store.redo_edit_ids().is_empty(),
                active_alternative_id: self.store.envelope().active_alternative_id.clone(),
                current_checkpoint_id: self.store.current_checkpoint_id().map(str::to_string),
                commands,
                command_filter: self.history_filter,
            }
        }

        /// @emoji 🗂️ Refreshes the snapshot cache if the store advanced, the command log grew/folded, or
        /// the history filter changed since the last materialization. The key is recomputed a SECOND time
        /// after `backfill_command_log` — backfill itself may `push_log_entry` (bumping `log_generation`),
        /// so keying only on the pre-backfill snapshot would store a stale key and thrash on every call.
        fn refresh_cache(&mut self) -> Result<(), Fault> {
            let key = (self.store.generation(), self.config_store.generation(), self.log_generation, self.history_filter);
            if self.cache.as_ref().map(|(cached_key, _, _, _)| *cached_key) == Some(key) {
                return Ok(());
            }
            self.backfill_command_log();
            let key = (self.store.generation(), self.config_store.generation(), self.log_generation, self.history_filter);
            let (snapshot, config) = match self.cache.take() {
                Some((cached_key, snapshot, config, _)) if cached_key.0 == key.0 && cached_key.1 == key.1 => (snapshot, config),
                Some((cached_key, snapshot, _, _)) if cached_key.0 == key.0 => {
                    let config = self.config_store.snapshot().map_err(|error| error.into_fault())?;
                    (snapshot, config)
                }
                _ => {
                    let snapshot = self.store.snapshot().map_err(|error| error.into_fault())?;
                    let config = self.config_store.snapshot().map_err(|error| error.into_fault())?;
                    (snapshot, config)
                }
            };
            let history = self.build_history_view();
            self.cache = Some((key, snapshot, config, history));
            Ok(())
        }

        /// @emoji 🕰️ Maps one of the six injected history action ids to its `ArtifactCommand`.
        fn history_command(action: &str, args: Option<&Value>) -> Option<ArtifactCommand<A::Mutation>> {
            let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
            match action {
                "undo" => Some(ArtifactCommand::Undo),
                "redo" => Some(ArtifactCommand::Redo),
                "commitCheckpoint" => Some(ArtifactCommand::CommitCheckpoint { message: arg_str("message"), authors: Vec::new() }),
                "createAlternative" => Some(ArtifactCommand::CreateAlternative { name: arg_str("name").unwrap_or_else(|| "Alternative".into()) }),
                "switchAlternative" => arg_str("alternativeId").map(|alternative_id| ArtifactCommand::SwitchAlternative { alternative_id }),
                "checkoutCheckpoint" => arg_str("checkpointId").map(|checkpoint_id| ArtifactCommand::CheckoutCheckpoint { checkpoint_id }),
                _ => None,
            }
        }

        /// @emoji 📇️ An empty `InvocationResult` carrying only host effects/events (view/shell actions,
        /// no-operation commands, and history notifications produce no `KernelMutation`s).
        fn empty_result(verb: &str, meta: &ActionMeta, effects: Vec<HostEffect>, events: Vec<AppEvent>, ui_scope: semio_framework::kernel::UiDirtyScope) -> InvocationResult {
            let invocation_id = InvocationId(format!("{verb}:{}", meta.instance_id));
            InvocationResult { output: DslValue::Null, mutations: Vec::new(), inverse_group: UndoGroup { invocation_id, mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() }, diagnostics: Vec::new(), requested_effects: effects, events, ui_scope }
        }

        /// @emoji 🧱️ Builds the `InvocationResult` for a just-dispatched edit: one `KernelMutation` per
        /// forward operation NEW in this dispatch (`tail_offset`), each carrying just this dispatch's
        /// `inverse` as its inverse diff. For a coalesced (`AmendLast`) edit, `edit_mutations()` returns
        /// the WHOLE accumulated edit — without slicing to `tail_offset`, every dispatch would rebuild and
        /// serialize every `KernelMutation` since the gesture started (O(edit-size) per dispatch, O(edit-
        /// size²) over the whole gesture) purely to report operations the caller already knows about.
        fn result_from_last_edit(&self, verb: &str, meta: &ActionMeta, effects: Vec<HostEffect>, events: Vec<AppEvent>, ui_scope: semio_framework::kernel::UiDirtyScope, tail_offset: (usize, usize)) -> InvocationResult {
            let schema = A::DOCUMENT_SCHEMA.to_string();
            let invocation_id = InvocationId(format!("{verb}:{}:{}", meta.instance_id, self.store.generation()));
            let document = ArtifactHandle(meta.instance_id as u128);
            let mut mutations: Vec<KernelMutation> = Vec::new();
            if let Some((forwards, inverse, mutation_meta)) = self.store.edit_mutations() {
                let (forwards_offset, backwards_offset) = tail_offset;
                let forwards = &forwards[forwards_offset.min(forwards.len())..];
                let inverse = &inverse[backwards_offset.min(inverse.len())..];
                let mutation_meta = &mutation_meta[forwards_offset.min(mutation_meta.len())..];
                // 🎯️ B5: real binary — each backward op's own `OpBinary::encode_op()`, framed as a
                // binary ops-vec (`protocol::encode_ops_vec`, replacing the old `json!({"inverse":
                // [...]})` convention). Every op in `inverse` shares the same encoding; a decode
                // failure here would mean a real corrupt/foreign operation, not a schema choice, so
                // `unwrap_or_default` on an individual encode failure degrades to an empty inverse
                // (same fallback behavior `payload: Vec::new()` already has elsewhere) rather than
                // panicking mid-invocation.
                let inverse_payload = protocol::encode_ops_vec(&inverse.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect::<Vec<_>>());
                for (index, forward) in forwards.iter().enumerate() {
                    let entry = mutation_meta.get(index);
                    // 🎯️ W6 kernel unification: `mutation_meta` entries are `protocol::MutationMeta`,
                    // and this kernel's own `ArtifactDiff`/`UndoPolicy`/`HybridLogicalTimestamp` are now
                    // `pub use` re-exports of the SAME `protocol`/`protocol_core` types (see
                    // `framework/core`'s kernel cut-over note) — no bridging left to do, just direct
                    // field moves. `ArtifactDiff.schema`/`.payload` are `SchemaId`/`Vec<u8>` now (was
                    // `schema_id`/`Value`), so the payload is JSON-encoded to bytes at construction.
                    let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                    let base_version = ArtifactVersion(entry.map(|entry_meta| entry_meta.base_version).unwrap_or(0));
                    let undo_policy = entry.map(|entry_meta| entry_meta.undo_policy).unwrap_or(UndoPolicy::ExactBaseOnly);
                    let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                    let timestamp = entry.map(|entry_meta| entry_meta.timestamp).unwrap_or_else(|| HybridLogicalTimestamp::new(0, 0));
                    mutations.push(KernelMutation {
                        id: mutation_id.clone(),
                        document,
                        base_version,
                        invocation_id: invocation_id.clone(),
                        diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: ::protocol::OpBinary::encode_op(forward).unwrap_or_default() },
                        inverse: InverseMutation {
                            target_mutation: mutation_id,
                            inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: inverse_payload.clone() },
                            base_version,
                            dependencies: Vec::new(),
                            undo_policy,
                        },
                        dependencies: Vec::new(),
                        author,
                        timestamp,
                    });
                }
            }
            let mutation_ids: Vec<MutationId> = mutations.iter().map(|operation| operation.id.clone()).collect();
            let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|operation| operation.inverse.clone()).collect();
            InvocationResult { output: DslValue::Null, mutations, inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits: Vec::new() }, diagnostics: Vec::new(), requested_effects: effects, events, ui_scope }
        }

        // 🧮️ B1: `materialize_args` (JSON-args default-fill + required-arg enforcement for app-declared
        // actions/commands, plus its now-unused `effective_action_args`/`missing_required_args` imports)
        // was deleted — dead code once `dispatch_action`'s generic app-action fallback and
        // `dispatch_command`'s registry-backed dispatch were removed (an app's own behavior now dispatches
        // exclusively through the typed `Self::Command` channel, where a Rust caller supplies a complete
        // value — there is no "missing arg" to materialize).

        /// @emoji 🧬️ Shared dispatch tail for `handle_action`/`handle_command`/`import_media`: given the
        /// app's `ActionEmit`, either records the op-less dispatch and returns an empty result, or commits
        /// `Apply`/`AmendLast`, records the resulting edit, and builds the `InvocationResult` from it.
        /// `verb` is the action/command id, used to resolve the registry kind/label and to synthesize the
        /// `InvocationId`.
        /// @emoji 🧬️ B1: the single dispatch tail for BOTH `dispatch_typed_command` (the app's own
        /// `Emit`) and the framework-reserved clipboard actions below — commits `artifact_mutations` to
        /// the document store and `config_mutations` to the config store (independently, each its own
        /// undo stack), records exactly one command-log row carrying whichever edit id(s) were produced,
        /// and builds the resulting `InvocationResult` from the document side (config-only dispatches
        /// never touch `KernelMutation`/space-sync — see `🔖️CommandLog`'s doc region for why).
        fn dispatch_emit(&mut self, verb: &str, emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let Emit { artifact_mutations, config_mutations, draft_mutations, description, coalesce_key, effects, events, ui_scope, child_emits } = emit;
            self.last_emit_wire = Some((
                protocol::encode_ops_vec(&artifact_mutations.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect::<Vec<_>>()),
                protocol::encode_ops_vec(&config_mutations.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect::<Vec<_>>()),
                protocol::encode_ops_vec(&draft_mutations.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect::<Vec<_>>()),
            ));

            // 📝️ Draft lane — ephemeral; applied without command-log rows (never checkpoints).
            if !draft_mutations.is_empty() {
                self.draft_store.set_local_actor_id(Some(meta.actor.clone()));
                self.draft_store
                    .dispatch(ArtifactCommand::Apply { mutations: draft_mutations, description: None })
                    .map_err(|error| error.into_fault())?;
            }

            // 🧮️ Config side dispatches first, independent of whether this verb ALSO touches the document
            // — captures the resulting (possibly amended) config edit id for the command-log row below.
            let mut config_edit_id: Option<String> = None;
            if !config_mutations.is_empty() {
                self.config_store.set_local_actor_id(Some(meta.actor.clone()));
                let before_config_edit_id = self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
                let config_command = match &coalesce_key {
                    Some(key) => ArtifactCommand::AmendLast { mutations: config_mutations, coalesce_key: Some(format!("config:{key}")) },
                    None => ArtifactCommand::Apply { mutations: config_mutations, description: description.clone() },
                };
                self.config_store.dispatch(config_command).map_err(|error| error.into_fault())?;
                self.cache = None;
                let amended_same_config_edit = before_config_edit_id.is_some() && self.config_store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_config_edit_id.as_ref();
                config_edit_id = if amended_same_config_edit { before_config_edit_id } else { self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) };
            }

            // 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): a non-empty `child_emits` means this
            // gesture must land as ONE atomic multi-document group — routed entirely through
            // `dispatch_emit_group` (parent ops included), never through the solitary single-store
            // path below. See that method's own doc comment for why the two paths stay genuinely
            // separate rather than being unified into one (the group protocol has no `AmendLast`).
            if !child_emits.is_empty() {
                return self.dispatch_emit_group(verb, artifact_mutations, child_emits, description, effects, events, ui_scope, config_edit_id, meta);
            }

            if artifact_mutations.is_empty() {
                // 🧾️ An app-declared action logs under its declared kind (so an `Mutation`-kind action
                // that happened to produce zero document operations — e.g. paste with no clipboard
                // fragment, or a pure config-op dispatch — still gets a real row, `edit_id: None`,
                // correctly filed under "Without Operations"); a declared command (no `ActionKind` of its
                // own) logs as `Shell`; anything unresolved (registry-less test construction, an ad-hoc
                // verb like an import-media port id) logs as `View`.
                let kind = match self.registry.get(verb) {
                    Some(def) => def.kind,
                    None if self.registry.get_command(verb).is_some() => ActionKind::Shell,
                    None => ActionKind::View,
                };
                self.record_command(verb, kind, description.clone(), None, config_edit_id, None);
                return Ok(Self::empty_result(verb, meta, effects, events, ui_scope));
            }
            self.store.set_local_actor_id(Some(meta.actor.clone()));
            // 🪢️ Captured before dispatch so `result_from_last_edit` can report only the operations THIS dispatch
            // added — if `AmendLast` amends the same edit (`before_edit_id` unchanged after dispatch), these
            // are the tail offsets into that edit's now-longer forwards/inverse; if a new edit was created
            // instead, the offsets are moot (checked via edit identity, not reused blindly).
            let before_edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
            let (before_forwards_len, before_backwards_len) = self.store.edit_mutations().map(|(f, b, _)| (f.len(), b.len())).unwrap_or((0, 0));
            let log_label = description.clone();
            let vcs_command = match coalesce_key {
                Some(key) => ArtifactCommand::AmendLast { mutations: artifact_mutations, coalesce_key: Some(key) },
                None => ArtifactCommand::Apply { mutations: artifact_mutations, description },
            };
            self.store.dispatch(vcs_command).map_err(|error| error.into_fault())?;
            self.cache = None;
            let amended_same_edit = before_edit_id.is_some() && self.store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_edit_id.as_ref();
            // 🧾️ One command-log entry per VCS edit — a coalesced gesture (`amended_same_edit`) grows the
            // existing entry's `op_lines` live (see `build_history_view`), it never appends a new entry.
            if !amended_same_edit {
                if let Some(edit_id) = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) {
                    // 🧾️ `verb` is the app's typed-command tag or a framework command id — `CommandDefinition`
                    // has no `ActionKind` (commands aren't View/Shell/History), and reaching here means
                    // operations were dispatched, so `Mutation` is always correct.
                    let kind = self.registry.get(verb).map(|def| def.kind).unwrap_or(ActionKind::Mutation);
                    // ⏪️ No memory `inverse` for an edit-linked row — the VCS edit's own `Mutation::inverse`
                    // is already the real inverse, and `revertToCommand`'s edit_id branch replays that.
                    self.record_command(verb, kind, log_label, Some(edit_id), config_edit_id, None);
                }
            }
            let tail_offset = if amended_same_edit { (before_forwards_len, before_backwards_len) } else { (0, 0) };
            Ok(self.result_from_last_edit(verb, meta, effects, events, ui_scope, tail_offset))
        }

        /// @emoji 🌱️ Absorbs every `store::GroupReceipt::created_children` entry into the live
        /// `self.children` map — B2 flagged that a `ChildGenesis`-created member has no pre-existing
        /// caller-held reference the way an already-registered child does, so without this it would
        /// be silently dropped the moment `dispatch_group` returns, making `ChildGenesis` pointless
        /// (see `GroupReceipt::created_children`'s own doc comment). The slot name is recovered from
        /// `self.composition`'s own graph (`dispatch_group`'s phase 2 already called `insert_owns`
        /// for every genesis child before returning the receipt, so `slot_of` is always populated
        /// here). A free-standing method (not inlined into `dispatch_emit_group`) so it is directly
        /// unit-testable against a synthetic `created_children` list, independent of the `ChildEmit`
        /// wire path (which never emits a `ChildGenesis` itself today).
        pub(crate) fn absorb_created_children(&mut self, created_children: Vec<(ArtifactRef, Box<dyn SpaceMember>)>) {
            for (target, member) in created_children {
                let slot = self.composition.graph().slot_of(&target.artifact_id).unwrap_or_default().to_string();
                self.children.insert((slot, target.artifact_id.clone()), (target.dialect.clone(), member));
            }
        }

        /// @emoji 🧩️ `dispatch_emit`'s composite-gesture branch, taken whenever `emit.child_emits` is
        /// non-empty. Routes `artifact_mutations` (the parent's own ops) plus every `ChildEmit`
        /// through `store::CompositionCoordinator::dispatch_group` as ONE atomic multi-document
        /// gesture, so a single user action spanning a parent and N owned children still produces ONE
        /// `UndoGroup`: `inverse_group.member_edits` names every touched document (parent included),
        /// and `dispatch_action`'s history-action arm routes an undo/redo of a group-tailed edit
        /// through `CompositionCoordinator::undo_group`/`redo_group` — see that arm's own doc comment.
        /// Deliberately narrower than the solitary (zero-children) path above: `dispatch_group`'s
        /// phase 2 always issues a plain `Apply` on every touched member (never `AmendLast` — there is
        /// no group-aware per-tick coalescing protocol), so a composite gesture with children is
        /// always ONE described edit per member, never folded across ticks — the solitary path keeps
        /// `AmendLast` coalescing exactly as before. `inverse_group.mutations`/`.inverse_mutations`
        /// are best-effort for CHILD members specifically: real per-op inverse bytes are retrievable
        /// only through `ArtifactStore<P, Mutation>`'s own inherent `edit_mutations()`, and each
        /// child's concrete `P`/`Mutation` is erased behind `Box<dyn SpaceMember>` — the REAL
        /// reversal mechanism for a child member is `CompositionCoordinator::undo_group` calling
        /// `undo()` directly on the live child store (driven by `inverse_group.member_edits`, not by
        /// replaying these bytes).
        #[allow(clippy::too_many_arguments)]
        fn dispatch_emit_group(
            &mut self,
            verb: &str,
            artifact_mutations: Vec<A::Mutation>,
            child_emits: Vec<ChildEmit>,
            description: Option<String>,
            effects: Vec<HostEffect>,
            events: Vec<AppEvent>,
            ui_scope: semio_framework::kernel::UiDirtyScope,
            config_edit_id: Option<String>,
            meta: &ActionMeta,
        ) -> Result<InvocationResult, Fault> {
            let parent_id = self.store.envelope().id.clone();
            // 🚧️ `ArtifactEnvelope.dialect` stays `Option<ArtifactDialect>` per B2's own DEFERRED
            // scope decision (`📓️wave1-reports/b2-store-composition-report.md`) — this fallback (a
            // synthetic "native" dialect from the app's own `DOCUMENT_SCHEMA`) is only ever consulted
            // when no real dialect was ever threaded through `create_document_envelope`, and only
            // matters for the OWNERSHIP-GRAPH bookkeeping `dispatch_group` needs a real `ArtifactRef`
            // for, not for any wire/codec decision.
            let parent_dialect = self.store.envelope().dialect.clone().unwrap_or_else(|| store::os_io::ArtifactDialect { artifact_kind: A::DOCUMENT_SCHEMA.to_string(), standard: "native".to_string(), subset: "*".to_string() });
            let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
            let parent_ops: Vec<Vec<u8>> = artifact_mutations.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect();

            self.store.set_local_actor_id(Some(meta.actor.clone()));
            let mut seen_keys = ::std::collections::HashSet::with_capacity(child_emits.len());
            let mut child_ptrs: Vec<(*mut dyn SpaceMember, ChildDispatch)> = Vec::with_capacity(child_emits.len());
            for child_emit in &child_emits {
                let key = (child_emit.slot.clone(), child_emit.child_id.clone());
                if !seen_keys.insert(key.clone()) {
                    return Err(plugin_sdk_fault(format!(
                        "dispatch_emit: verb {verb:?} emitted duplicate ChildEmit for slot {:?} child {:?}",
                        child_emit.slot, child_emit.child_id
                    )));
                }
                let (dialect, member) = self.children.get_mut(&key).ok_or_else(|| {
                    plugin_sdk_fault(format!(
                        "dispatch_emit: verb {verb:?} emitted a ChildEmit for slot {:?} child {:?} with no live child store — call VcsArtifactApp::open_child/register_child first",
                        child_emit.slot, child_emit.child_id
                    ))
                })?;
                let target = ArtifactRef { artifact_id: child_emit.child_id.clone(), dialect: dialect.clone() };
                let member_ptr: *mut dyn SpaceMember = member.as_mut();
                child_ptrs.push((member_ptr, ChildDispatch { child: target, ops: child_emit.ops.clone(), op_schema: child_emit.op_schema.clone(), labels: child_emit.labels.clone() }));
            }
            // 🛡️ Keys are unique and `self.children` is not resized until after `dispatch_group`, so
            // these entry pointers stay valid and non-aliasing for the simultaneous child borrows
            // `dispatch_group` requires.
            let mut dispatches: Vec<(&mut dyn SpaceMember, ChildDispatch)> = child_ptrs
                .into_iter()
                .map(|(ptr, dispatch)| (unsafe { &mut *ptr }, dispatch))
                .collect();

            // 🎛️ `meta.actor`/`description` are honored; `coalesce_key` is intentionally dropped here
            // — `GroupMeta.coalesce_key` is accepted-but-not-wired by `dispatch_group` itself today
            // (per B2's own scoping note: `SpaceMember` has no object-safe `AmendLast` seam yet), and
            // this whole branch is already documented as never coalescing.
            let group_meta = GroupMeta { actor: Some(meta.actor.clone()), description: description.clone(), coalesce_key: None };
            let receipt = self
                .composition
                .dispatch_group(&parent_ref, &mut self.store as &mut dyn SpaceMember, &mut dispatches, parent_ops, Vec::new(), group_meta)
                .map_err(|error| plugin_sdk_fault(error.to_string()))?;
            drop(dispatches);
            self.cache = None;

            // 🌱️ Absorb any freshly-created children into the live map — extracted into its own
            // method (below) both so it is directly unit-testable and so it needs no revisiting once
            // a genesis-emitting `Emit` constructor lands (`genesis` is always `Vec::new()` on THIS
            // call today — a `ChildEmit` only ever targets an ALREADY-live child).
            self.absorb_created_children(receipt.created_children);

            let invocation_id = InvocationId(receipt.invocation_id.clone());
            // 🪪️ The PARENT's handle must be the same value its own `KernelMutation.document` carries
            // (`ArtifactHandle(meta.instance_id)`), not `artifact_handle_of(parent_id)` — otherwise one
            // `InvocationResult` identifies the same document two different ways and any consumer
            // correlating `mutations` with `member_edits` by handle silently fails to match the parent.
            // Children keep the content-addressed `artifact_handle_of`, which is what their own
            // `KernelMutation.document` uses.
            let member_edits: Vec<EditRef> = receipt
                .member_edits
                .iter()
                .map(|(reference, edit_id)| {
                    let document = if reference.artifact_id == parent_id { ArtifactHandle(meta.instance_id as u128) } else { artifact_handle_of(&reference.artifact_id) };
                    EditRef { document, edit_id: edit_id.clone() }
                })
                .collect();

            let mut mutations: Vec<KernelMutation> = Vec::new();

            // 🧱️ Parent side: `dispatch_wire` landed a REAL edit on `self.store` above (whenever
            // `parent_ops` was non-empty), so its tail edit's real forward/inverse mutations are
            // available exactly like the solitary path's `result_from_last_edit`.
            let parent_touched = receipt.member_edits.iter().any(|(reference, _)| reference.artifact_id == parent_id);
            if parent_touched {
                if let Some((forwards, inverse, mutation_meta)) = self.store.edit_mutations() {
                    let inverse_payload = protocol::encode_ops_vec(&inverse.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect::<Vec<_>>());
                    let document = ArtifactHandle(meta.instance_id as u128);
                    let schema = A::DOCUMENT_SCHEMA.to_string();
                    for (index, forward) in forwards.iter().enumerate() {
                        let entry = mutation_meta.get(index);
                        let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                        let base_version = ArtifactVersion(entry.map(|entry_meta| entry_meta.base_version).unwrap_or(0));
                        let undo_policy = entry.map(|entry_meta| entry_meta.undo_policy).unwrap_or(UndoPolicy::ExactBaseOnly);
                        let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                        let timestamp = entry.map(|entry_meta| entry_meta.timestamp).unwrap_or_else(|| HybridLogicalTimestamp::new(0, 0));
                        mutations.push(KernelMutation {
                            id: mutation_id.clone(),
                            document,
                            base_version,
                            invocation_id: invocation_id.clone(),
                            diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: ::protocol::OpBinary::encode_op(forward).unwrap_or_default() },
                            inverse: InverseMutation {
                                target_mutation: mutation_id,
                                inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: inverse_payload.clone() },
                                base_version,
                                dependencies: Vec::new(),
                                undo_policy,
                            },
                            dependencies: Vec::new(),
                            author,
                            timestamp,
                        });
                    }
                }
            }

            // 🧩️ Child side: real per-op inverse bytes are not retrievable through the object-safe
            // `SpaceMember` seam (see this fn's own doc comment) — each touched child's edit surfaces
            // as ONE best-effort `KernelMutation` bundling all of that child's ops as its diff
            // payload, with an empty (but validly-encoded, decodes to zero ops) inverse payload.
            // `dispatch_group`'s phase 2 walks `children` (built here in the SAME order as
            // `child_emits`) by index, skipping empty-ops entries — so the non-empty-ops subsequence
            // of `child_emits`, in order, lines up 1:1 with the non-parent tail of `member_edits`.
            let child_member_edits: Vec<&(ArtifactRef, String)> = receipt.member_edits.iter().filter(|(reference, _)| reference.artifact_id != parent_id).collect();
            let touched_child_emits: Vec<&ChildEmit> = child_emits.iter().filter(|child_emit| !child_emit.ops.is_empty()).collect();
            debug_assert_eq!(touched_child_emits.len(), child_member_edits.len(), "dispatch_group's per-child edit order must match the non-empty-ops subsequence of child_emits");
            let mut child_edit_ids: Vec<String> = Vec::with_capacity(child_member_edits.len());
            for (child_emit, (reference, edit_id)) in touched_child_emits.into_iter().zip(child_member_edits.into_iter()) {
                child_edit_ids.push(edit_id.clone());
                let document = artifact_handle_of(&reference.artifact_id);
                let mutation_id = MutationId(edit_id.clone());
                mutations.push(KernelMutation {
                    id: mutation_id.clone(),
                    document,
                    base_version: ArtifactVersion(0),
                    invocation_id: invocation_id.clone(),
                    diff: ArtifactDiff { schema: child_emit.op_schema.clone(), payload: protocol::encode_ops_vec(&child_emit.ops) },
                    inverse: InverseMutation {
                        target_mutation: mutation_id,
                        inverse_diff: ArtifactDiff { schema: SchemaId(format!("{}.inverse", child_emit.op_schema.0)), payload: protocol::encode_ops_vec(&[]) },
                        base_version: ArtifactVersion(0),
                        dependencies: Vec::new(),
                        undo_policy: UndoPolicy::ExactBaseOnly,
                    },
                    dependencies: Vec::new(),
                    author: ActorId(meta.actor.clone()),
                    timestamp: HybridLogicalTimestamp::new(0, 0),
                });
            }

            let mutation_ids: Vec<MutationId> = mutations.iter().map(|mutation| mutation.id.clone()).collect();
            let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|mutation| mutation.inverse.clone()).collect();

            // 🧾️ One command-log row for the whole group — `child_edit_ids` follows the existing
            // `config_edit_ids` precedent (see `CommandLogEntry::child_edit_ids`'s own doc comment).
            let parent_edit_id = receipt.member_edits.iter().find(|(reference, _)| reference.artifact_id == parent_id).map(|(_, edit_id)| edit_id.clone());
            let kind = self.registry.get(verb).map(|def| def.kind).unwrap_or(ActionKind::Mutation);
            self.record_command(verb, kind, description, parent_edit_id, config_edit_id, None);
            if let Some(last) = self.command_log.last_mut() {
                last.child_edit_ids = child_edit_ids;
            }

            Ok(InvocationResult {
                output: DslValue::Null,
                mutations,
                inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits },
                diagnostics: Vec::new(),
                requested_effects: effects,
                events,
                ui_scope,
            })
        }

        /// @emoji 🕸️ Re-syncs `self.composition`'s ownership/link graph from the parent's own live
        /// `ArtifactRefs` projection (`child_refs()`/`links()`) — the mechanism
        /// `store::CompositionGraph::sync_member`'s own doc comment names as the required follow-up
        /// "after any dispatch that might have changed an artifact's own `ArtifactRefs`". Deliberately
        /// NOT called automatically by `dispatch_emit`/`dispatch_emit_group`: doing so would force
        /// `A::Snapshot: store::ArtifactRefs` onto EVERY `ArtifactApp` impl in the workspace (today
        /// zero of them implement it — `ArtifactRefs`-bearing snapshots are new, later-wave-scoped
        /// schema work), breaking every existing plugin's compile for a graph no leaf artifact needs
        /// synced. A composed artifact's own app calls this explicitly wherever its `handle` might
        /// have changed a `#[child(...)]`/`#[link_slot(...)]` field — see B2's own
        /// `📓️wave1-reports/b2-store-composition-report.md` sharedFileRequest #6.
        pub fn sync_composition_graph(&mut self) -> Result<(), Fault>
        where
            A::Snapshot: store::ArtifactRefs,
        {
            let snapshot = self.store.snapshot().map_err(|error| error.into_fault())?;
            let parent_id = self.store.envelope().id.clone();
            self.composition.graph_mut().sync_member(&parent_id, &snapshot).map_err(plugin_sdk_fault)
        }

        /// @emoji ↩️ Task 4: group-aware undo/redo. Routes through
        /// `store::CompositionCoordinator::undo_group`/`redo_group` across `self.store` (the parent)
        /// plus every LIVE child in `self.children` — the coordinator itself filters to members whose
        /// tail actually carries `group_id` (a child never touched by this particular gesture, or one
        /// whose own `undo()`/`redo()` call fails, is SKIPPED, not aborted — see `GroupUndoReport`'s
        /// own doc comment for why abort-all would be actively harmful: one collaborator's foreign or
        /// failed child edit must never permanently freeze this parent's undo stack). This generalizes
        /// the plain single-store path's existing benign `NothingToUndo`/`NothingToRedo`/`ForeignEdit`
        /// collapse to the multi-member case. Member order matters only for readability/parity with
        /// `dispatch_group`'s own fixed apply order (children → parent): undo reverses it
        /// (parent → children), redo re-establishes it (children → parent) — `undo_group`/`redo_group`
        /// themselves treat every member independently regardless of list order.
        fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let parent_id = self.store.envelope().id.clone();
            let parent_dialect = self.store.envelope().dialect.clone().unwrap_or_else(|| store::os_io::ArtifactDialect { artifact_kind: A::DOCUMENT_SCHEMA.to_string(), standard: "native".to_string(), subset: "*".to_string() });
            let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
            // 🪞️ Two SEPARATE passes over `self.children` (an immutable key/dialect snapshot, then
            // `values_mut()`) rather than repeated `get_mut` calls in a loop — `HashMap` iteration
            // order is unspecified but DETERMINISTIC for a given map instance as long as it is not
            // mutated in between two iterations, which holds here (nothing inserts/removes between
            // the two calls below), so the positional zip lines each dialect/id up with its own member.
            let child_refs: Vec<ArtifactRef> = self.children.iter().map(|(key, (dialect, _))| ArtifactRef { artifact_id: key.1.clone(), dialect: dialect.clone() }).collect();

            let mut members: Vec<(&ArtifactRef, &mut dyn SpaceMember)> = Vec::with_capacity(1 + child_refs.len());
            if action == "undo" {
                members.push((&parent_ref, &mut self.store as &mut dyn SpaceMember));
                for (reference, (_, member)) in child_refs.iter().zip(self.children.values_mut()) {
                    members.push((reference, member.as_mut()));
                }
            } else {
                for (reference, (_, member)) in child_refs.iter().zip(self.children.values_mut()) {
                    members.push((reference, member.as_mut()));
                }
                members.push((&parent_ref, &mut self.store as &mut dyn SpaceMember));
            }

            let report = if action == "undo" { CompositionCoordinator::undo_group(&mut members, group_id) } else { CompositionCoordinator::redo_group(&mut members, group_id) };
            drop(members);
            self.cache = None;

            let diagnostics: Vec<dsl::Diagnostic> = report
                .skipped
                .iter()
                .map(|(reference, error)| dsl::Diagnostic::error("composition.group-history.skipped-member", dsl::TextSpan::default(), format!("{action} skipped member {} ({error})", reference.artifact_id)))
                .collect();

            if report.undone.is_empty() {
                // 🧾️ Nothing actually moved (every member was foreign/failed) — benign collapse,
                // mirroring `NothingToUndo`/`NothingToRedo`/`ForeignEdit` just above: NOT logged
                // (never touched any store), but the skip diagnostics still ride along so the caller
                // can see WHY nothing happened instead of silently no-op'ing.
                let mut result = Self::empty_result(action, meta, Vec::new(), Vec::new(), semio_framework::kernel::UiDirtyScope::None);
                result.diagnostics = diagnostics;
                return Ok(result);
            }

            // 🧾️ Append-only, same as the plain path: undo/redo are pure cursor motion, never
            // logged with an `edit_id` — `child_edit_ids` records every CHILD member this group call
            // actually touched, following the `config_edit_ids`/`child_edit_ids` precedent.
            self.record_command(action, ActionKind::History, None, None, None, None);
            let child_edit_ids: Vec<String> = report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id).map(|(_, edit_id)| edit_id.clone()).collect();
            if let Some(last) = self.command_log.last_mut() {
                last.child_edit_ids = child_edit_ids;
            }

            let mut result = Self::empty_result(action, meta, Vec::new(), vec![history_changed_event()], semio_framework::kernel::UiDirtyScope::Full);
            result.diagnostics = diagnostics;
            Ok(result)
        }

        /// @emoji 🕰️ The actual body of `PluginApp::handle_action` — renamed to an inherent method so
        /// `handle_action` itself can stay a thin `finish_recorded` wrapper (see `🔖️CommandLog`). B1:
        /// FRAMEWORK-reserved verbs only (history/revert/filter/noteShellCommand/clipboard) — an app's own
        /// behavior is dispatched exclusively through `dispatch_typed_command` now.
        pub(crate) fn dispatch_action(&mut self, action: &str, args: Option<&Value>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            if HISTORY_ACTION_IDS.contains(&action) {
                // 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): when the tail edit (undo) / redo-tail
                // edit (redo) carries a `MutationMeta.group_id`, this history action must reverse the
                // WHOLE composite gesture — every member the group's `dispatch_group` call touched,
                // not just `self.store` — so it is routed through `dispatch_group_history_action`
                // instead of the plain single-store path below. A solitary (non-grouped) edit has no
                // `group_id` and falls through unchanged.
                if action == "undo" || action == "redo" {
                    let group_id = if action == "undo" { self.store.tail_group_id() } else { self.store.redo_tail().and_then(|(_, group_id)| group_id) };
                    if let Some(group_id) = group_id {
                        return self.dispatch_group_history_action(action, &group_id, meta);
                    }
                }
                let command = Self::history_command(action, args).ok_or_else(|| format!("history action {action} missing required argument"))?;
                // 📌️ Composition cascade, BEFORE the parent's own dispatch: a checkpoint on a
                // composing document must pin what its children looked like at that moment, and a
                // checkout must restore them to their pinned state. Both run leaves-first — commit
                // the children, THEN record their resulting checkpoint ids on the parent's
                // checkpoint — because the pin can only name a child checkpoint that already exists.
                let pending_pins = match &command {
                    ArtifactCommand::CommitCheckpoint { message, authors } => self.commit_children_for_checkpoint(message.clone(), authors.clone())?,
                    _ => Vec::new(),
                };
                match self.store.dispatch(command) {
                    Ok(_) => {
                        self.stamp_checkpoint_composition_pins(pending_pins);
                        if action == "checkoutCheckpoint" {
                            self.cascade_checkout_to_children();
                        }
                        self.cache = None;
                        // 🧾️ Undo/redo/checkpoint/alternative are pure cursor motion (`edit_id: None`) —
                        // append-only: this NEVER removes a prior entry, including undo's.
                        self.record_command(action, ActionKind::History, None, None, None, None);
                        // 🐢️ History actions (undo/redo/checkpoint/alternative) can touch any part of the
                        // document — always Full, never opt into a narrower scope.
                        Ok(Self::empty_result(action, meta, Vec::new(), vec![history_changed_event()], semio_framework::kernel::UiDirtyScope::Full))
                    }
                    // Benign no-operations (nothing to undo/redo, foreign tail) collapse to an empty result
                    // and are NOT logged — they never touched the store.
                    Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::NothingToRedo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), semio_framework::kernel::UiDirtyScope::None)),
                    Err(error) => Err(error.into_fault()),
                }
            } else if action == REVERT_TO_COMMAND_ACTION_ID {
                self.refresh_cache()?;
                let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(Value::as_u64);
                // ⏪️ Only a `revertible` entry has a real anchor to revert to — see `CommandView::revertible`
                // in `build_history_view`. Four disjoint shapes, branched below: (1) document edit-linked
                // (VCS undo to position), (2) config edit-linked (config-store undo to position — B1's
                // replacement for the old memory-only "View"-kind inverse), (3) `Shell`-kind memory-only
                // (bubble the inverse out — the plugin can't touch shell-owned state itself).
                let target = entry_seq.and_then(|seq| {
                    self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.config_edit_id.clone(), entry.kind, entry.inverse.clone()))
                });
                match target {
                    Some((Some(edit_id), _, _, _)) => {
                        let target_position = self.store.applied_edit_ids().iter().position(|id| *id == edit_id);
                        match target_position {
                            Some(position) => {
                                let undo_count = self.store.applied_edit_ids().len() - (position + 1);
                                for _ in 0..undo_count {
                                    match self.store.dispatch(ArtifactCommand::Undo) {
                    Ok(_) => {}
                                        // 🛑️ Stop early rather than error — a foreign edit further up the stack
                                        // still leaves the revert partially applied, which is the best this can do.
                                        Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::ForeignEdit(_)) => break,
                                        Err(error) => return Err(error.into_fault()),
                                    }
                                }
                                self.cache = None;
                                // 🧾️ One entry for the revert itself — the internal undos it performs are an
                                // implementation detail, not separately logged commands.
                                self.record_command(action, ActionKind::History, Some("Revert to Command".to_string()), None, None, None);
                                Ok(Self::empty_result(action, meta, Vec::new(), vec![history_changed_event()], semio_framework::kernel::UiDirtyScope::Full))
                            }
                            None => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), semio_framework::kernel::UiDirtyScope::None)),
                        }
                    }
                    // 🧮️ B1: config edit-linked — the config-store twin of the document branch above.
                    Some((None, Some(config_edit_id), _, _)) => {
                        let target_position = self.config_store.applied_edit_ids().iter().position(|id| *id == config_edit_id);
                        match target_position {
                            Some(position) => {
                                let undo_count = self.config_store.applied_edit_ids().len() - (position + 1);
                                for _ in 0..undo_count {
                                    match self.config_store.dispatch(ArtifactCommand::Undo) {
                        Ok(_) => {}
                                        Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::ForeignEdit(_)) => break,
                                        Err(error) => return Err(error.into_fault()),
                                    }
                                }
                                self.cache = None;
                                self.record_command(action, ActionKind::History, Some("Revert to Command".to_string()), None, None, None);
                                Ok(Self::empty_result(action, meta, Vec::new(), vec![history_changed_event()], semio_framework::kernel::UiDirtyScope::Full))
                            }
                            None => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), semio_framework::kernel::UiDirtyScope::None)),
                        }
                    }
                    // ⏪️ `Shell`-kind memory-only: the plugin has no access to shell-owned state (theme/dock/
                    // panel layout live client-side), so it can't replay this itself — bubble the inverse out
                    // as a `HostEffect` for the shell to redispatch through its own command funnel.
                    Some((None, None, ActionKind::Shell, Some(inverse))) => Ok(Self::empty_result(
                        action,
                        meta,
                        vec![HostEffect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args.as_ref().map(|value| to_dsl_value(value).unwrap_or(DslValue::Null)) }],
                        Vec::new(),
                        semio_framework::kernel::UiDirtyScope::None,
                    )),
                    _ => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), semio_framework::kernel::UiDirtyScope::None)),
                }
            } else if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID {
                // 🎚️ Arg key is `"value"`, not `"filter"` — see `set_history_command_filter_action_definition`'s doc.
                let filter = args.and_then(|value| value.get("value")).and_then(Value::as_str).unwrap_or("all");
                self.history_filter = match filter {
                    "withoutMutations" => HistoryCommandFilter::WithoutMutations,
                    "onlyMutations" => HistoryCommandFilter::OnlyMutations,
                    _ => HistoryCommandFilter::All,
                };
                // 🗂️ Deliberately UNLOGGED — the panel operating its own filter chrome shouldn't fill the
                // very list it's filtering. No explicit cache invalidation either: `history_filter` is part
                // of `refresh_cache`'s key tuple now, so the next refresh naturally rebuilds.
                Ok(Self::empty_result(
                    action,
                    meta,
                    Vec::new(),
                    Vec::new(),
                    semio_framework::kernel::UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false },
                ))
            } else if action == NOTE_SHELL_COMMAND_ACTION_ID {
                // 🗒️ Interception happens BEFORE the app ever sees this action — records a shell effect
                // that already happened (dock drag, window resize/close, theme/locale change, …) into the
                // session command log for effects dispatched outside the normal `Emit`/`dispatch_emit` path.
                let command_id = args.and_then(|value| value.get("commandId")).and_then(Value::as_str).ok_or_else(|| "noteShellCommand missing required commandId".to_string())?;
                let label = args.and_then(|value| value.get("label")).and_then(Value::as_str).unwrap_or(command_id);
                let detail = args.and_then(|value| value.get("detail")).and_then(Value::as_str);
                let label = match detail {
                    Some(detail) => format!("{label} - {detail}"),
                    None => label.to_string(),
                };
                // ⏪️ Optional real inverse: the shell already knows the pre-change value at its call site
                // (e.g. the previous theme id) and can supply the command that restores it, giving this row
                // a working Backwards button too — see `HostEffect::ReplayShellCommand`.
                let inverse =
                    args.and_then(|value| value.get("inverseCommandId")).and_then(Value::as_str).map(|inverse_command_id| InverseAction { action_id: inverse_command_id.to_string(), args: args.and_then(|value| value.get("inverseArgs")).cloned() });
                self.record_command(command_id, ActionKind::Shell, Some(label), None, None, inverse);
                Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), semio_framework::kernel::UiDirtyScope::None))
            } else if CLIPBOARD_ACTION_IDS.contains(&action) {
                self.refresh_cache()?;
                let emit = {
                    let VcsArtifactApp { app, cache, children, .. } = self;
                    let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
                    let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
                    let cfg = ConfigView { snapshot: config };
                    match action {
                        "copy" => match A::copy_fragment(&doc, &cfg) {
                            Ok(fragment) => Emit { effects: vec![HostEffect::ClipboardWrite { fragment }], ..Default::default() },
                            Err(_) => Emit::default(),
                        },
                        "cut" => {
                            let fragment = A::copy_fragment(&doc, &cfg).ok();
                            let operations = A::cut_operations(&doc, &cfg);
                            let mut emit = Emit::mutations(operations);
                            emit.description = Some("Cut".into());
                            if let Some(fragment) = fragment {
                                emit.effects.push(HostEffect::ClipboardWrite { fragment });
                            }
                            emit
                        }
                        "paste" => {
                            let fragment = args.and_then(|value| value.get("fragment")).and_then(|value| serde_json::from_value::<ClipboardFragment>(value.clone()).ok());
                            let placement = args.and_then(|value| serde_json::from_value::<PastePlacement>(value.clone()).ok()).unwrap_or_default();
                            match fragment {
                                Some(fragment) => match A::paste_operations(&doc, &fragment, &placement) {
                                    Ok(operations) => {
                                        let mut emit = Emit::mutations(operations);
                                        emit.description = Some("Paste".into());
                                        emit
                                    }
                                    Err(_) => Emit::default(),
                                },
                                None => Emit::default(),
                            }
                        }
                        _ => unreachable!("CLIPBOARD_ACTION_IDS exhaustively matched above"),
                    }
                };
                self.dispatch_emit(action, emit, meta)
            } else {
                let command = A::command_from_action(action, args)?;
                self.dispatch_typed_command_inner(command, meta)
            }
        }

        /// @emoji 🕰️ FRAMEWORK-reserved command dispatch — see `dispatch_action`'s doc. There are currently
        /// no framework-reserved COMMANDS (only actions), so this always errors pointing at the typed
        /// channel; kept as a distinct entry point so `PluginApp::handle_command`'s object-safe shape (used
        /// by `CommandDefinition`-driven host call sites) stays stable.
        fn dispatch_command(&mut self, command: &str, _args: Option<&Value>, _meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            Err(Fault::from(format!("command '{command}' — app commands are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)")))
        }

        /// 🎯️ The actual body of `PluginApp::handle_command_frame`. `command_bytes.first() == Some(1)`
        /// (the `OpBinary` format tag) means "typed `A::Command`" — decoded and dispatched via
        /// `dispatch_typed_command`. Anything else is the legacy `{kind,name,args}` wire-value envelope,
        /// routed through `dispatch_action`/`dispatch_command` — now FRAMEWORK-reserved verbs only.
        fn dispatch_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            if command_bytes.first().copied() == Some(1) {
                return self.dispatch_typed_command(command_bytes, meta);
            }
            let envelope = store::pack_rt::decode_wire_value(command_bytes).map_err(|error| error.into_fault())?;
            let kind = envelope.get("kind").and_then(DslValue::as_str).unwrap_or("action").to_string();
            let name = envelope.get("name").and_then(DslValue::as_str).unwrap_or("").to_string();
            let args = envelope.get("args").cloned().map(store::pack_rt::dsl_value_to_json);
            if kind == "command" {
                self.dispatch_command(&name, args.as_ref(), meta)
            } else {
                self.dispatch_action(&name, args.as_ref(), meta)
            }
        }

        /// @emoji 🎯️ B1: decodes `command_bytes` as the app's typed `A::Command` (`OpBinary::decode_op`)
        /// and calls the pure `ArtifactApp::handle` — the sole surface for an app's own behavior now.
        fn dispatch_typed_command(&mut self, command_bytes: &[u8], meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let command = <A::Command as ::protocol::OpBinary>::decode_op(command_bytes).map_err(|error| error.into_fault())?;
            self.dispatch_typed_command_inner(command, meta)
        }

        /// @emoji 🎯️ B1: the shared body behind both `dispatch_typed_command` (wire bytes, decoded above)
        /// and `dispatch_typed` (an already-typed value, for direct Rust callers — see `testkit`).
        fn dispatch_typed_command_inner(&mut self, command: A::Command, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            self.refresh_cache()?;
            let draft_snapshot = self.draft_store.snapshot().map_err(|error| error.into_fault())?;
                let (verb, emit, ephemeral) = {
                let VcsArtifactApp { app, cache, children, presence_store, transient_store, .. } = self;
                let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
                let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
                let cfg = ConfigView { snapshot: config };
                let draft = DraftView { snapshot: &draft_snapshot };
                let presence = PresenceView { local: presence_store.local(), peers: presence_store.peers() };
                let transient = TransientView { snapshot: transient_store.current() };
                let engines = EngineHandles::empty();
                let verb = A::command_id(&command).to_string();
                // 👥️🫧️ The ephemeral lanes are computed BEFORE `handle` so they still see the
                // pre-command state, and they are computed unconditionally: a command that fails
                // still moved the cursor that provoked it.
                let ephemeral = A::ephemeral(&command, &doc, &cfg, &presence, &transient);
                let emit = A::handle(&command, &doc, &cfg, &draft, &engines)?;
                (verb, emit, ephemeral)
            };
            // 👥️🫧️ Applied outside the borrow above, and never through `dispatch_emit`: neither lane
            // has an op log, an edit id, an undo group or a command-log row.
            self.presence_store.apply(&ephemeral.presence);
            self.transient_store.apply(&ephemeral.transient);
            // 🛂️ Kind discipline: a `View`/`Shell`-kind command must not emit document operations — mirrors
            // the pre-B1 `dispatch_action`'s enforcement, now keyed off `command_id()` since dispatch is
            // typed rather than stringly. Only enforced when the registry actually declares `verb` (a
            // registry-less construction, or a command whose id isn't declared, skips this check).
            if let Some(def) = self.registry.get(&verb) {
                if matches!(def.kind, ActionKind::View | ActionKind::Shell) && !emit.artifact_mutations.is_empty() {
                    return Err(Fault::from(format!("{:?}-kind command '{verb}' must not emit operations", def.kind)));
                }
            }
            self.dispatch_emit(&verb, emit, meta)
        }

        /// @emoji 🎯️ B1: public typed-value dispatch entry point — the direct-Rust-caller counterpart to
        /// the wire-level `PluginApp::handle_command_frame`, self-contained (applies `finish_recorded`
        /// itself). Used by `testkit`'s generic app-agnostic helpers and any other in-process caller that
        /// already holds a concrete `A::Command` value.
        pub fn dispatch_typed(&mut self, command: A::Command, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let log_generation_before = self.log_generation;
            let result = self.dispatch_typed_command_inner(command, meta)?;
            Ok(self.finish_recorded(log_generation_before, "typed-command", result))
        }

        /// @emoji 🕰️ The actual body of `PluginApp::import_media` — see `dispatch_action`'s doc.
        fn dispatch_import_media(&mut self, port: &str, media: &Media, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            self.refresh_cache()?;
            let emit = {
                let VcsArtifactApp { app, cache, children, .. } = self;
                let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
                let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
                let _cfg = ConfigView { snapshot: config };
                A::import_media(port, media, &doc).map_err(|error| plugin_sdk_fault(error.to_string()))?
            };
            self.dispatch_emit(&format!("import-media:{port}"), emit, meta)
        }

        /// @emoji 🧮️ B1: dispatches a binary-encoded `store::ArtifactCommand<A::ConfigMutation>` against
        /// the config store — real work for `AppCommand::ConfigCommand` (replaces the deleted
        /// `apply_config_bytes` whole-record-replace legacy path).
        fn dispatch_config_command_inner(&mut self, command_bytes: &[u8], meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            self.config_store.set_local_actor_id(Some(meta.actor.clone()));
            self.config_store.dispatch_binary(command_bytes).map_err(|error| error.into_fault())?;
            self.cache = None;
            let config_edit_id = self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
            self.record_command("configCommand", ActionKind::Shell, None, None, config_edit_id, None);
            Ok(Self::empty_result("configCommand", meta, Vec::new(), Vec::new(), semio_framework::kernel::UiDirtyScope::Full))
        }

        /// @emoji 🕰️ Upgrades `result.ui_scope` to also refresh the framework history panel body whenever
        /// `dispatch_action`/`dispatch_command`/`dispatch_import_media` actually logged something
        /// (`log_generation` advanced) — the seam that makes the panel live without every action opting in.
        fn finish_recorded(&self, log_generation_before: u64, verb: &str, mut result: InvocationResult) -> InvocationResult {
            if self.log_generation != log_generation_before {
                let skip_history_panel = self.registry.get(verb).is_some_and(|def| matches!(def.kind, ActionKind::View));
                if !skip_history_panel {
                    result.ui_scope = with_history_panel_scope(result.ui_scope);
                }
            }
            result
        }
    }

    /// @emoji 📣️ Signals the shell that the document's checkpoint/alternative history changed (after an
    /// undo/redo/checkpoint/alternative command) so it can re-render history-dependent surfaces.
    fn history_changed_event() -> AppEvent {
        AppEvent { kind: "history-changed".into(), payload: DslValue::Null }
    }

    /// @emoji 🕰️ Upgrades a returned `UiDirtyScope` to also cover the framework history panel body
    /// (`FRAMEWORK_HISTORY_BODY_KEY`) — `Full` is already maximal and passes through unchanged, `None`
    /// becomes a `Partial` naming just the history body, and an existing `Partial` gains it alongside
    /// whatever the app already asked to refresh (idempotent — checks before pushing).
    fn with_history_panel_scope(scope: semio_framework::kernel::UiDirtyScope) -> semio_framework::kernel::UiDirtyScope {
        use semio_framework::kernel::UiDirtyScope;
        match scope {
            UiDirtyScope::Full => UiDirtyScope::Full,
            UiDirtyScope::None => UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false },
            UiDirtyScope::Partial { window_bodies, mut panel_bodies, utilities, tools, engagements, measures, labels } => {
                if !panel_bodies.iter().any(|key| key == FRAMEWORK_HISTORY_BODY_KEY) {
                    panel_bodies.push(FRAMEWORK_HISTORY_BODY_KEY.to_string());
                }
                UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels }
            }
        }
    }

    impl<A: ArtifactApp> PluginApp for VcsArtifactApp<A> {
        fn app_id(&self) -> &str {
            A::APP_ID
        }

        fn document_schema(&self) -> &str {
            A::DOCUMENT_SCHEMA
        }

        fn handle_action(&mut self, action: &str, args: Option<&Value>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let log_generation_before = self.log_generation;
            let result = self.dispatch_action(action, args, meta)?;
            Ok(self.finish_recorded(log_generation_before, action, result))
        }

        fn handle_command(&mut self, command: &str, args: Option<&Value>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let log_generation_before = self.log_generation;
            let result = self.dispatch_command(command, args, meta)?;
            Ok(self.finish_recorded(log_generation_before, command, result))
        }

        fn handle_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let log_generation_before = self.log_generation;
            let result = self.dispatch_command_frame(command_bytes, meta)?;
            Ok(self.finish_recorded(log_generation_before, "typed-command", result))
        }

        fn take_last_emit_wire(&mut self) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
            self.last_emit_wire.take()
        }

        fn hydrate_document_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {
            if pack.is_empty() && spr.is_empty() {
                return Ok(());
            }
            self.load_document_pack(&store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() })
        }

        fn hydrate_config_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {
            if pack.is_empty() && spr.is_empty() {
                return Ok(());
            }
            self.load_config_pack(&store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() })
        }

        fn hydrate_draft_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {
            if pack.is_empty() && spr.is_empty() {
                return Ok(());
            }
            let parsed: store::ParsedDocumentText<A::Draft, A::DraftMutation> = store::parse_document_pack(pack, spr).map_err(|error| error.into_fault())?;
            let (applied, redo) = match &parsed.envelope.cursor {
                Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                None => (parsed.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new()),
            };
            self.draft_store.reset(parsed.envelope, applied, redo).map_err(|error| error.into_fault())?;
            Ok(())
        }

        fn config_pack(&self) -> Result<store::ArtifactPackFiles, Fault> {
            store::print_document_pack(self.config_store.envelope()).map_err(|error| error.into_fault())
        }

        fn load_child_pack(&mut self, slot: &str, child_id: &str, dialect: store::os_io::ArtifactDialect, envelope_pack: &[u8]) -> Result<(), Fault> {
            self.open_child(slot, child_id, dialect, envelope_pack)?;
            self.cache = None;
            Ok(())
        }

        fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, Fault> {
            // 🔢️ Sorted by `(slot, child_id)`: the map's iteration order is not stable, and a
            // persisted child list that reshuffles between reads would make every save look like a
            // change to anything diffing it.
            let mut entries: Vec<protocol::ChildPackEntry> = self
                .children
                .iter()
                .map(|((slot, child_id), (dialect, member))| {
                    member.envelope_pack_bytes().map(|envelope_pack| protocol::ChildPackEntry { slot: slot.clone(), child_id: child_id.clone(), dialect: dialect.to_coordinate(), envelope_pack }).map_err(|error| error.into_fault())
                })
                .collect::<Result<_, Fault>>()?;
            entries.sort_by(|left, right| (&left.slot, &left.child_id).cmp(&(&right.slot, &right.child_id)));
            Ok(entries)
        }

        fn load_config_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault> {
            let parsed: store::ParsedDocumentText<A::Config, A::ConfigMutation> = store::parse_document_pack(&files.pack, &files.spr).map_err(|error| error.into_fault())?;
            let (applied, redo) = match &parsed.envelope.cursor {
                Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                None => (parsed.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new()),
            };
            self.config_store.reset(parsed.envelope, applied, redo).map_err(|error| error.into_fault())?;
            self.cache = None;
            Ok(())
        }

        fn dispatch_config_command(&mut self, command_bytes: &[u8], meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let log_generation_before = self.log_generation;
            let result = self.dispatch_config_command_inner(command_bytes, meta)?;
            Ok(self.finish_recorded(log_generation_before, "configCommand", result))
        }

        fn ingest_operations(&mut self, mutations: &[u8]) -> Result<(), Fault> {
            let envelopes = protocol::decode_envelopes(mutations).map_err(|error| error.into_fault())?;
            for envelope in envelopes {
                self.store.dispatch(ArtifactCommand::IngestRemote { envelope }).map_err(|error| error.into_fault())?;
            }
            self.cache = None;
            Ok(())
        }

        fn ingest_operations_text(&mut self, operations_text: &str) -> Result<(), Fault> {
            let mutations: Vec<A::Mutation> = operations_text.lines().map(str::trim).filter(|line| !line.is_empty()).map(|line| A::Mutation::parse_op(line).map_err(|error| error.into_fault())).collect::<Result<Vec<_>, _>>()?;
            if mutations.is_empty() {
                return Ok(());
            }
            self.store.dispatch(ArtifactCommand::Apply { mutations, description: None }).map_err(|error| error.into_fault())?;
            self.cache = None;
            Ok(())
        }

        fn document_text(&self) -> Result<store::ArtifactTextFiles, Fault> {
            store::print_document_text(self.store.envelope()).map_err(|error| error.into_fault())
        }

        fn load_document_text(&mut self, files: &store::ArtifactTextFiles) -> Result<(), Fault> {
            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).map_err(|error| error.into_fault())?;
            let applied: Vec<String> = parsed.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
            self.store.reset(parsed.envelope, applied, Vec::new()).map_err(|error| error.into_fault())?;
            self.cache = None;
            Ok(())
        }

        fn document_pack(&self) -> Result<store::ArtifactPackFiles, Fault> {
            store::print_document_pack(self.store.envelope()).map_err(|error| error.into_fault())
        }

        fn load_document_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault> {
            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(&files.pack, &files.spr).map_err(|error| error.into_fault())?;
            // 🎯️ W4: honor a persisted cursor (undo/redo position) when present — falling back to
            // "every edit applied" for a pack predating this field, matching `ArtifactStore::new`'s
            // own cursor-aware seeding.
            let (applied, redo) = match &parsed.envelope.cursor {
                Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                None => (parsed.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new()),
            };
            self.store.reset(parsed.envelope, applied, redo).map_err(|error| error.into_fault())?;
            self.cache = None;
            Ok(())
        }

        fn attach_backbone(&mut self, backbone: Box<dyn store::Backbone>) -> Result<(), Fault> {
            self.store.attach_backbone(backbone).map_err(|error| error.into_fault())?;
            self.cache = None;
            Ok(())
        }

        fn detach_backbone(&mut self) {
            self.store.detach_backbone();
            self.cache = None;
        }

        fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel) -> Result<UiNode, Fault> {
            self.refresh_cache()?;
            if body_key == FRAMEWORK_HISTORY_BODY_KEY {
                // 🕰️ Framework-owned, snapshot-independent — served before any app body-key match.
                let (_, _, _, history) = self.cache.as_ref().expect("cache refreshed above");
                return Ok(ui_history_panel(history, &self.registry.controller_id, view_state.locale == Locale::De));
            }
            let effective_body_key = if let Some(ref wid) = view_state.window_id {
                if !body_key.contains(':') {
                    format!("{body_key}:{wid}")
                } else {
                    body_key.to_string()
                }
            } else {
                body_key.to_string()
            };
            if let Some(json) = snapshot_override_json {
                let snapshot: A::Snapshot = serde_json::from_str(json).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                let history = self.build_history_view();
                let doc = ArtifactView::new(&snapshot, &history);
                let config = self.config_store.snapshot().unwrap_or_else(|_| A::Config::default());
                let cfg = ConfigView { snapshot: &config };
                return Ok(A::render(&effective_body_key, &doc, &cfg));
            }
            let VcsArtifactApp { app, cache, children, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
            let cfg = ConfigView { snapshot: config };
            Ok(A::render(&effective_body_key, &doc, &cfg))
        }

        fn window_engagements(&mut self) -> HashMap<String, WindowEngagement> {
            if self.refresh_cache().is_err() {
                return HashMap::new();
            }
            let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(&self.children));
            let cfg = ConfigView { snapshot: config };
            A::window_engagements(&doc, &cfg)
        }

        fn window_measures(&mut self) -> HashMap<String, Vec<WindowMeasure>> {
            if self.refresh_cache().is_err() {
                return HashMap::new();
            }
            let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(&self.children));
            let cfg = ConfigView { snapshot: config };
            A::window_measures(&doc, &cfg)
        }

        fn tool_measures(&mut self) -> HashMap<String, Vec<WindowMeasure>> {
            if self.refresh_cache().is_err() {
                return HashMap::new();
            }
            let VcsArtifactApp { app, cache, children, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
            let cfg = ConfigView { snapshot: config };
            A::tool_measures(&doc, &cfg)
        }

        fn pending_effects(&mut self) -> Vec<HostEffect> {
            if self.refresh_cache().is_err() {
                return Vec::new();
            }
            let VcsArtifactApp { app, cache, children, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
            let cfg = ConfigView { snapshot: config };
            A::pending_effects(&doc, &cfg)
        }

        /// 🗂️ Every context menu is organized (D2 of the grouped-context-menu mechanism design) at this
        /// single funnel — a raw-vec emitter is grouped for free, and an emitter that already built its own
        /// `Menu::group(...)` rows is never re-flattened (`organize_context_menu` is idempotent on already-organized input).
        fn context_menu(&mut self, request: &ContextMenuRequest) -> Vec<ContextMenuItemSpec> {
            if self.refresh_cache().is_err() {
                return Vec::new();
            }
            let VcsArtifactApp { app, cache, registry, children, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
            let cfg = ConfigView { snapshot: config };
            let items = A::context_menu(request, &doc, &cfg, registry);
            ui_wgpu::wgpu::organize_context_menu(items, &|id| registry.category_of(id))
        }

        fn export_media(&mut self, port: &str) -> Result<Media, MediaError> {
            self.refresh_cache().map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
            let VcsArtifactApp { app, cache, children, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
            let _cfg = ConfigView { snapshot: config };
            A::export_media(port, &doc)
        }

        fn import_media(&mut self, port: &str, media: &Media, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
            let log_generation_before = self.log_generation;
            let result = self.dispatch_import_media(port, media, meta)?;
            Ok(self.finish_recorded(log_generation_before, &format!("import-media:{port}"), result))
        }

        fn media_fingerprint(&mut self, port: &str) -> Result<MediaFingerprint, MediaError> {
            self.refresh_cache().map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
            let VcsArtifactApp { app, cache, children, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_children(snapshot, history, ChildContentView::new(children));
            let _cfg = ConfigView { snapshot: config };
            A::media_fingerprint(port, &doc)
        }
    }

    pub struct AppInstance {
        pub id: u32,
        pub app: Box<dyn PluginApp>,
    }
    //#endregion 🔖️DocumentContract

    pub trait PluginProgram: Send {
        fn manifest(&self) -> PluginManifest;
        fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>>;
    }

    pub struct Plugin {
        pub manifest: PluginManifest,
        apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
    }

    impl Plugin {
        pub fn new(plugin_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
            Self {
                manifest: PluginManifest { plugin_id: plugin_id.into(), label: label.into(), version: version.into(), apps: Vec::new(), examples: Vec::new(), capabilities: Vec::new(), topic_contributions: Vec::new(), commands: Vec::new(), artifact_kinds: Vec::new() },
                apps: HashMap::new(),
            }
        }

        /// @emoji 🎛️ Declares a plugin-scope command (applies whenever any of this plugin's apps is
        /// focused). Panics if `command.scope != CommandScope::Plugin`.
        pub fn plugin_command(mut self, command: CommandDefinition) -> Self {
            assert!(command.scope == CommandScope::Plugin, "plugin {} command {} must be declared CommandScope::Plugin", self.manifest.plugin_id, command.id);
            self.manifest.commands.push(command);
            self
        }

        pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
            if !self.manifest.capabilities.contains(&capability) {
                self.manifest.capabilities.push(capability);
            }
            self
        }

        /// 🗂️ Declares one plugin-level artifact kind (library plugins use this; repeatable).
        pub fn artifact_kind(mut self, spec: ArtifactKindSpec) -> Self {
            self.manifest.artifact_kinds.push(spec);
            self
        }

        pub fn local_backbone_storage(self) -> Self {
            self.capability(CapabilityRequirement { artifact: ArtifactKind::Backbone, rights: Rights::Read, scope: Scope::Plugin }).capability(CapabilityRequirement { artifact: ArtifactKind::Backbone, rights: Rights::Write, scope: Scope::Plugin })
        }

        /// 🧬️ Registers an already-wrapped app factory (used by `PluginBuilder` and `register_document_app`).
        pub fn register_app_factory(mut self, app: App, factory: impl Fn() -> Box<dyn PluginApp> + Send + 'static) -> Self {
            let app_id = app.definition.id.clone();
            self.manifest.apps.push(app.definition);
            for mut example in app.examples {
                example.app_id = app_id.clone();
                self.manifest.examples.push(example);
            }
            self.apps.insert(self.manifest.apps.last().unwrap().id.clone(), Box::new(factory));
            self
        }

        /// @emoji 🧬️ Registers a typed {@link ArtifactApp} as a ZST — turbofish-only, no factory closure.
        /// Wraps each instance in {@link VcsArtifactApp}. Stateful app structs are unrepresentable.
        /// Also registers `A::app_schema()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) —
        /// see {@link ArtifactApp::app_schema}'s own doc.
        pub fn register_document_app<A: ArtifactApp>(self, app: App) -> Self {
            if let Some(descriptor) = A::app_schema() {
                ::semio_framework_schema::register_app_schema_descriptor(descriptor);
            }
            let registry = AppActionRegistry::from_definition(&app.definition);
            self.register_app_factory(app, move || Box::new(VcsArtifactApp::with_registry(A::default(), registry.clone())))
        }

        pub fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
            self.apps.get(app_id).map(|factory| factory())
        }
    }

    impl PluginProgram for Plugin {
        fn manifest(&self) -> PluginManifest {
            self.manifest.clone()
        }

        fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
            Plugin::create_app(self, app_id)
        }
    }

    pub use super::builder::{NeedsLabel, NeedsVersion, PluginBuilder, Ready};
    // #endregion app
}

pub mod plugin_runtime {
    // #region plugin_runtime
    //! 📤️ WASM component export glue for plugin bundles.

    use crate::app::{ActionMeta, AppInstance, MediaArtifact, MediaArtifactDescriptor, Plugin, PluginProgram};
    use crate::ArtifactApp;
    use dsl::{from_dsl_value, to_dsl_value};
    use semio_framework::{
        kernel::{CapabilityRequirement, HostEffect, InvocationResult},
        TopicContribution, Fault, FaultCode, FaultFrom, FaultOrigin, PluginManifest, ViewModel,
    };
    use std::collections::HashMap;
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::cell::{Cell, RefCell};
    use std::sync::atomic::{AtomicU32, Ordering};
    use ui_wgpu::wgpu::{ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSurfaceTarget, UiMenuRef, UiNode};

    thread_local! {
        static PLUGIN: RefCell<Option<Plugin>> = const { RefCell::new(None) };
        // 🔓️ `UnsafeCell` (not `RefCell`): a wasm trap skips `RefMut::drop` and permanently poisons
        // `RefCell`'s borrow flag. Exclusive access is enforced by `InstanceGuard` + the host's
        // serialized plugin bridge instead.
        static INSTANCES: std::cell::UnsafeCell<Vec<AppInstance>> = const { std::cell::UnsafeCell::new(Vec::new()) };
        static INSTANCE_GUARD: Cell<u32> = const { Cell::new(0) };
        /// 🪪️ Per-instance local actor id, set by `AppCommand::Hello` and read back by every `Command`
        /// frame's `ActionMeta` — see `plugin_exchange`. Never cleared on `Bye`/instance destruction (Wave
        /// 1 scope: a destroyed instance id is never reused within one plugin's lifetime today).
        static INSTANCE_ACTORS: RefCell<std::collections::HashMap<u32, String>> = RefCell::new(std::collections::HashMap::new());
    }

    fn encode_wire_serialized<T: Serialize>(value: &T) -> Vec<u8> {
        store::pack_rt::encode_wire_value(&to_dsl_value(value).expect("wire payload must serialize to DslValue"))
    }

    fn push_app_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, fault: Fault) {
        frames.push(protocol::AppFrame::Error { in_reply_to, fault: encode_wire_serialized(&fault) });
    }

    fn push_os_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, code: &str, message: String) {
        push_app_fault(frames, in_reply_to, Fault::new(FaultOrigin::Os, FaultCode::new(code), message));
    }

    fn plugin_internal_fault(message: impl Into<String>) -> Fault {
        Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), message)
    }

    fn decode_wire_serialized<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Fault> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| plugin_internal_fault(error.to_string()))?;
        from_dsl_value(value).map_err(plugin_internal_fault)
    }

    fn decode_wire_serialized_or<T: DeserializeOwned>(bytes: &[u8], default: T) -> T {
        decode_wire_serialized(bytes).unwrap_or(default)
    }

    /// 🪪️ Records `actor` as the local actor id for `instance_id` (from `AppCommand::Hello`).
    fn set_instance_actor(instance_id: u32, actor: String) {
        INSTANCE_ACTORS.with(|slot| {
            slot.borrow_mut().insert(instance_id, actor);
        });
    }

    /// 🪪️ The actor id last recorded for `instance_id` via `set_instance_actor`, or `"local"` when no
    /// `Hello` has been processed yet (mirrors `plugin_handle_action`'s own `"local"` fallback).
    fn instance_actor(instance_id: u32) -> String {
        INSTANCE_ACTORS.with(|slot| slot.borrow().get(&instance_id).cloned()).unwrap_or_else(|| "local".to_string())
    }

    /// 🗣️ Decodes a packed `ViewModel` payload (empty → default). No process-global    /// 🗣️ Decodes a packed `ViewModel` payload (empty → default). No process-global cache —
    /// host-authoritative chrome/draft owns locale; every command/refresh carries view_state on the wire.
    fn decode_view_state(view_state_bytes: &[u8]) -> ViewModel {
        if view_state_bytes.is_empty() {
            ViewModel::default()
        } else {
            store::pack_rt::decode_wire_value(view_state_bytes).ok().and_then(|value| from_dsl_value::<ViewModel>(value).ok()).unwrap_or_default()
        }
    }

    struct InstanceGuard;

    impl InstanceGuard {
        fn enter() -> Result<Self, Fault> {
            if INSTANCE_GUARD.get() > 0 {
                return Err(plugin_internal_fault("plugin instance busy"));
            }
            INSTANCE_GUARD.set(1);
            Ok(Self)
        }

        /// 🩹️ Clears a guard left set when a prior call trapped without running `Drop` — safe between
        /// host-serialized top-level calls; must not be invoked mid-call.
        fn clear_poison() {
            INSTANCE_GUARD.set(0);
        }
    }

    impl Drop for InstanceGuard {
        fn drop(&mut self) {
            INSTANCE_GUARD.set(0);
        }
    }

    fn with_instances_mut<R, F: FnOnce(&mut Vec<AppInstance>) -> Result<R, Fault>>(f: F) -> Result<R, Fault> {
        let _guard = InstanceGuard::enter()?;
        // SAFETY: `InstanceGuard` + the JS/host serialized plugin bridge ensure exclusive access.
        INSTANCES.with(|instances| f(unsafe { &mut *instances.get() }))
    }

    /// 🩹️ Heals `InstanceGuard` after a wasm trap so the next host-serialized call is not stuck on
    /// `plugin instance busy`. No-operation when the guard is already clear.
    pub fn plugin_clear_instance_guard() {
        InstanceGuard::clear_poison();
    }

    static NEXT_INSTANCE_ID: AtomicU32 = AtomicU32::new(1);

    pub fn install_plugin_bundle(bundle: Plugin) {
        PLUGIN.with(|slot| {
            *slot.borrow_mut() = Some(bundle);
        });
    }

    static PLUGIN_INIT_ONCE: std::sync::Once = std::sync::Once::new();

    static PLUGIN_BUNDLE_INSTALLER: std::sync::OnceLock<fn()> = std::sync::OnceLock::new();

    /// @emoji 🧩️ Registers the embedding plugin crate's bundle installer (expanded from `plugin_exports!`).
    pub fn register_plugin_bundle_installer(install: fn()) {
        let _ = PLUGIN_BUNDLE_INSTALLER.set(install);
    }

    /// 🔗️ Weak default so intermediate `cdylib` links (e.g. `semio-framework-os` pulled into a
    /// wasip2 plugin build via feature unification of `component-guest`) succeed; the embedding
    /// plugin's `plugin_exports!` provides the strong installer override.
    #[cfg(feature = "component-guest")]
    #[unsafe(no_mangle)]
    #[linkage = "weak"]
    pub extern "C" fn semio_plugin_bundle_installer_link_shim() {}

    /// Ensures the embedding plugin crate's bundle installer ran before any WIT export is served.
    pub fn ensure_plugin_initialized() {
        PLUGIN_INIT_ONCE.call_once(|| {
            #[cfg(target_arch = "wasm32")]
            console_error_panic_hook::set_once();
            #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
            crate::host_port::register_host_backbone_channel();
            #[cfg(feature = "component-guest")]
            {
                unsafe {
                    semio_plugin_bundle_installer_link_shim();
                }
                if let Some(install) = PLUGIN_BUNDLE_INSTALLER.get() {
                    install();
                }
            }
        });
    }

    pub fn plugin_manifest() -> PluginManifest {
        ensure_plugin_initialized();
        PLUGIN.with(|slot| {
            slot.borrow().as_ref().map(|bundle| Plugin::manifest(bundle)).unwrap_or_else(|| PluginManifest {
                plugin_id: "empty".into(),
                label: "Empty".into(),
                version: "0.0.0".into(),
                apps: vec![],
                examples: vec![],
                capabilities: vec![],
                topic_contributions: vec![],
                commands: vec![],
             artifact_kinds: vec![] })
        })
    }

    pub fn plugin_create_app(app_id: &str) -> Result<u32, Fault> {
        PLUGIN.with(|slot| {
            let program = slot.borrow();
            let program = program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?;
            let app = program.create_app(app_id).ok_or_else(|| plugin_internal_fault(format!("unknown app: {app_id}")))?;
            let id = NEXT_INSTANCE_ID.fetch_add(1, Ordering::SeqCst);
            with_instances_mut(|list| {
                list.push(AppInstance { id, app });
                Ok(())
            })?;
            Ok(id)
        })
    }

    pub fn plugin_destroy_app(instance_id: u32) -> Result<(), Fault> {
        with_instances_mut(|list| {
            let index = list.iter().position(|instance| instance.id == instance_id).ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))?;
            list.remove(index);
            Ok(())
        })
    }

    fn find_instance(list: &mut [AppInstance], instance_id: u32) -> Result<&mut AppInstance, Fault> {
        list.iter_mut().find(|instance| instance.id == instance_id).ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))
    }

    pub fn plugin_handle_action(instance_id: u32, action_json: &str, context_json: &str) -> Result<InvocationResult, Fault> {
        let action: Value = serde_json::from_str(action_json).map_err(|error| plugin_internal_fault(error.to_string()))?;
        let context: Value = serde_json::from_str(context_json).map_err(|error| plugin_internal_fault(error.to_string()))?;
        let action_name = action.get("action").and_then(|value| value.as_str()).unwrap_or("");
        let args = action.get("args").cloned();
        let actor = context.get("actor").and_then(|value| value.as_str()).unwrap_or("local").to_string();
        let meta = ActionMeta { actor, instance_id };
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.handle_action(action_name, args.as_ref(), &meta)
        })
    }

    /// @emoji 🎛️ Dispatches a scoped command (os/plugin/app/mode) through the same instance/context
    /// parsing as `plugin_handle_action` — mirrors its shape exactly.
    pub fn plugin_handle_command(instance_id: u32, command_json: &str, context_json: &str) -> Result<InvocationResult, Fault> {
        let command: Value = serde_json::from_str(command_json).map_err(|error| plugin_internal_fault(error.to_string()))?;
        let context: Value = serde_json::from_str(context_json).map_err(|error| plugin_internal_fault(error.to_string()))?;
        let command_name = command.get("command").and_then(|value| value.as_str()).unwrap_or("");
        let args = command.get("args").cloned();
        let actor = context.get("actor").and_then(|value| value.as_str()).unwrap_or("local").to_string();
        let meta = ActionMeta { actor, instance_id };
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.handle_command(command_name, args.as_ref(), &meta)
        })
    }

    /// @emoji 📥️ Ingests binary-encoded remote `MutationEnvelope`s (`protocol::decode_envelopes`) into
    /// the instance's document store (idempotent — duplicate mutation ids are dropped by the causal
    /// DAG / edit-id dedupe).
    pub fn plugin_ingest_operations(instance_id: u32, mutations: &[u8]) -> Result<(), Fault> {
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.ingest_operations(mutations)
        })
    }

    /// @emoji 📜️ Text-DSL counterpart of {@link plugin_ingest_operations}: one already-authored operation
    /// per non-blank op-text line instead of a binary `MutationEnvelope` array.
    pub fn plugin_ingest_operations_text(instance_id: u32, operations_text: &str) -> Result<(), Fault> {
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.ingest_operations_text(operations_text)
        })
    }

    /// @emoji 📜️ Text-DSL counterpart of {@link plugin_document_pack}.
    pub fn plugin_document_text(instance_id: u32) -> Result<store::ArtifactTextFiles, Fault> {
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.document_text()
        })
    }

    /// @emoji 📜️ Text-DSL counterpart of {@link plugin_load_document_pack}.
    pub fn plugin_load_document_text(instance_id: u32, files: &store::ArtifactTextFiles) -> Result<(), Fault> {
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.load_document_text(files)
        })
    }

    /// @emoji 📦️ Serializes the instance's full persistent document as pack+spr bytes
    /// ({@link store::ArtifactPackFiles}) via `store::print_document_pack`.
    pub fn plugin_document_pack(instance_id: u32) -> Result<store::ArtifactPackFiles, Fault> {
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.document_pack()
        })
    }

    /// @emoji 📦️ Replaces the instance's document from pack+spr bytes ({@link store::ArtifactPackFiles})
    /// via `store::parse_document_pack`.
    pub fn plugin_load_document_pack(instance_id: u32, files: &store::ArtifactPackFiles) -> Result<(), Fault> {
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.load_document_pack(files)
        })
    }

    /// @emoji 🗂️ Registers `A::Snapshot`'s pack↔dsl codec under `schema` in the process-wide
    /// `store::ArtifactCodec` registry — the one-liner every app's own native registration fn
    /// (`register_<app>_exports()`-style) calls once per document kind so `framework/sync`'s
    /// `FolderEndpoint` (and any other schema-string-keyed caller) can print/parse that kind without
    /// depending on its concrete `Snapshot`/`Mutation` types.
    pub fn register_document_codec_for_app<A: ArtifactApp>(schema: impl Into<String>) {
        store::register_document_codec(store::ArtifactCodec::of::<A::Snapshot, A::Mutation>(schema));
    }

    /// @emoji 🔗️ Attaches a backbone channel by URI. The URI is resolved to a `store::PortBackbone`
    /// (a pure queue relayed across the wasm sandbox to the host); the host owns the real IO endpoint.
    pub fn plugin_attach_backbone(instance_id: u32, uri: &str) -> Result<(), Fault> {
        let backbone: Box<dyn store::Backbone> = Box::new(store::PortBackbone::new(uri));
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.attach_backbone(backbone)
        })
    }

    /// @emoji ✂️ Detaches the instance's backbone channel; the document graph stays in memory.
    pub fn plugin_detach_backbone(instance_id: u32) -> Result<(), Fault> {
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.detach_backbone();
            Ok(())
        })
    }

    /// 🎞️ WIT `consume-media` glue — decodes the incoming `media-artifact` (`descriptor-json` + `data`)
    /// and dispatches to `PluginApp::consume_media`.
    pub fn plugin_consume_media(instance_id: u32, port_id: &str, descriptor_json: &str, data: Vec<u8>) -> Result<(), Fault> {
        let descriptor: MediaArtifactDescriptor = serde_json::from_str(descriptor_json).map_err(|error| plugin_internal_fault(error.to_string()))?;
        let artifact = MediaArtifact { descriptor, data };
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.consume_media(port_id, artifact).map_err(|error| plugin_internal_fault(error.to_string()))
        })
    }

    /// 🎞️ WIT `produce-media` glue — dispatches to `PluginApp::produce_media` and encodes the result back
    /// into `(descriptor-json, data)` for the `media-artifact` WIT record. `_request_json` is reserved for
    /// future parameterized requests; unused by the SDK default (see `PluginApp::produce_media`).
    pub fn plugin_produce_media(instance_id: u32, port_id: &str, _request_json: &str) -> Result<(String, Vec<u8>), Fault> {
        let artifact = with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.produce_media(port_id).map_err(|error| plugin_internal_fault(error.to_string()))
        })?;
        let descriptor_json = serde_json::to_string(&artifact.descriptor).map_err(|error| plugin_internal_fault(error.to_string()))?;
        Ok((descriptor_json, artifact.data))
    }

    pub fn plugin_render(instance_id: u32, body_key: &str, view_state_json: &str) -> Result<UiNode, Fault> {
        plugin_render_with_document(instance_id, body_key, None, view_state_json)
    }

    pub fn plugin_render_with_document(instance_id: u32, body_key: &str, snapshot_override_json: Option<&str>, view_state_json: &str) -> Result<UiNode, Fault> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct WindowRenderInput {
            #[serde(default)]
            body_key: String,
            view_state: ViewModel,
            #[serde(default)]
            document_json: Option<String>,
        }
        let (resolved_body_key, view_state, override_snapshot) = if body_key.is_empty() {
            let input: WindowRenderInput = serde_json::from_str(view_state_json).map_err(|error| plugin_internal_fault(error.to_string()))?;
            (input.body_key, input.view_state, input.document_json)
        } else if let Ok(input) = serde_json::from_str::<WindowRenderInput>(view_state_json) {
            let key = if input.body_key.is_empty() { body_key.to_string() } else { input.body_key };
            (key, input.view_state, input.document_json.or_else(|| snapshot_override_json.map(str::to_string)))
        } else {
            let view_state: ViewModel = serde_json::from_str(view_state_json).map_err(|error| plugin_internal_fault(error.to_string()))?;
            (body_key.to_string(), view_state, snapshot_override_json.map(str::to_string))
        };
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            instance.app.render(&resolved_body_key, override_snapshot.as_deref(), &view_state)
        })
    }

    //#region 🔖️RefreshUi
    /// 🐢️ The raw fnv1a-64 core — a tiny non-cryptographic hash for cheap "did this section's content
    /// change" checks, not a security boundary, just change detection, so speed over collision-resistance
    /// is the right tradeoff (mirrors the identical pattern already used for `cached_fixture_json` in
    /// puzzle's plugin). Extracted from `ui_refresh_fnv1a_hash` (which formats it as hex for the legacy
    /// JSON `refreshUi` wire) so `plugin_exchange`'s `AppFrame::UiSection.hash: u64` can reuse the same
    /// numeric core instead of hex-formatting and reparsing.
    fn fnv1a64(bytes: &[u8]) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    /// 🐢️ A tiny non-cryptographic hash (fnv1a-64) for cheap "did this section's content change" checks —
    /// not a security boundary, just change detection, so speed over collision-resistance is the right
    /// tradeoff (mirrors the identical pattern already used for `cached_fixture_json` in puzzle's plugin).
    fn ui_refresh_fnv1a_hash(bytes: &[u8]) -> String {
        format!("{:016x}", fnv1a64(bytes))
    }

    /// 🐢️ Hashes `value`'s canonical JSON serialization and returns `(hash, Some(value))` when it differs
    /// from `known_hash`, or `(hash, None)` when unchanged — the response omits the payload either way the
    /// caller doesn't need it, keeping the wire payload proportional to what actually changed.
    fn ui_refresh_section<T: Serialize>(value: &T, known_hash: Option<&str>) -> (String, Option<Value>) {
        let wire = serde_json::to_string(value).unwrap_or_default();
        let hash = ui_refresh_fnv1a_hash(wire.as_bytes());
        if known_hash == Some(hash.as_str()) {
            (hash, None)
        } else {
            let payload = serde_json::from_str(&wire).unwrap_or(Value::Null);
            (hash, Some(payload))
        }
    }

    /// 🐢️ `ui_refresh_section`'s `u64`-hash twin for `protocol::SectionProbe`/`AppFrame::UiSection` on the
    /// new binary channel (which carries `hash: Option<u64>`/`u64` directly, no hex string) — same
    /// hash-conditional payload-omission behavior, reusing `fnv1a64`.
    fn channel_refresh_section<T: Serialize>(value: &T, known_hash: Option<u64>) -> (u64, Option<Value>) {
        let wire = serde_json::to_string(value).unwrap_or_default();
        let hash = fnv1a64(wire.as_bytes());
        if known_hash == Some(hash) {
            (hash, None)
        } else {
            let payload = serde_json::from_str(&wire).unwrap_or(Value::Null);
            (hash, Some(payload))
        }
    }

    /// 🐢️ Batched, hash-conditional UI refresh: replaces the individual
    /// `render`/`windowEngagements`/`windowMeasures`/`appLabels` WASM round trips a full `refreshUi` used
    /// to make with **one** call. `request_json` lists every section the host wants (windows/panels by
    /// `{key, bodyKey, hash}`, engagements/measures/labels each `{hash}`); the response includes a payload
    /// only for sections whose hash differs from what the host already holds. Utility bars are no longer a
    /// plugin section — the renderer derives them from the utility registry via `derive_utility_nodes`.
    pub fn plugin_refresh_ui(instance_id: u32, request_json: &str) -> Result<String, String> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct SectionRequest {
            key: String,
            #[serde(default)]
            body_key: String,
            #[serde(default)]
            hash: Option<String>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct SingleRequest {
            #[serde(default)]
            hash: Option<String>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RefreshRequest {
            view_state: ViewModel,
            #[serde(default)]
            windows: Vec<SectionRequest>,
            #[serde(default)]
            panels: Vec<SectionRequest>,
            #[serde(default)]
            utilities: Vec<SectionRequest>,
            #[serde(default)]
            engagements: Option<SingleRequest>,
            #[serde(default)]
            measures: Option<SingleRequest>,
            #[serde(default)]
            tools: Option<SingleRequest>,
            #[serde(default)]
            labels: Option<SingleRequest>,
        }
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SectionResponse {
            key: String,
            hash: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<Value>,
        }
        #[derive(Serialize, Default)]
        #[serde(rename_all = "camelCase")]
        struct RefreshResponse {
            #[serde(skip_serializing_if = "Vec::is_empty")]
            windows: Vec<SectionResponse>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            panels: Vec<SectionResponse>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            utilities: Vec<SectionResponse>,
            #[serde(skip_serializing_if = "Option::is_none")]
            engagements: Option<SectionResponse>,
            #[serde(skip_serializing_if = "Option::is_none")]
            measures: Option<SectionResponse>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tools: Option<SectionResponse>,
            #[serde(skip_serializing_if = "Option::is_none")]
            labels: Option<SectionResponse>,
            /// ⏱️ See `ArtifactApp::pending_effects` — e.g. a `flowEvalTick` chain resuming after this refresh.
            #[serde(skip_serializing_if = "Vec::is_empty")]
            requested_effects: Vec<HostEffect>,
        }

        let request: RefreshRequest = serde_json::from_str(request_json).map_err(|error| error.to_string())?;

        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;

            let mut response = RefreshResponse::default();
            // ⏱️ Arm/advance background work BEFORE rendering below, not after — e.g. a `flowEvalTick`
            // chain's `computing_json` must be fresh by the time this same pass renders the graph, or a
            // cold-start load would render one full refresh cycle behind (nothing flagged as computing
            // until the *next* refresh).
            response.requested_effects = instance.app.pending_effects();

            for entry in &request.windows {
                // 🪟️ Stamp this window's instance id and its own active utility into the view state before
                // rendering, so a `ArtifactApp` can key per-window options and utility-driven scene state off
                // `view_state.window_id` / `view_state.active_utility_id` and never off the focused window alone.
                let active_utility_id = request.view_state.active_utility_by_window_id.get(&entry.key).cloned().or_else(|| request.view_state.active_utility_id.clone());
                let window_view_state = ViewModel { window_id: Some(entry.key.clone()), active_utility_id, ..request.view_state.clone() };
                let node = instance.app.render(&entry.body_key, None, &window_view_state)?;
                let (hash, value) = ui_refresh_section(&node, entry.hash.as_deref());
                response.windows.push(SectionResponse { key: entry.key.clone(), hash, value });
            }
            for entry in &request.panels {
                let node = instance.app.render(&entry.body_key, None, &request.view_state)?;
                let (hash, value) = ui_refresh_section(&node, entry.hash.as_deref());
                response.panels.push(SectionResponse { key: entry.key.clone(), hash, value });
            }
            // 🚧️ `utilities` intentionally unhandled here: `PluginApp`/`ArtifactApp` currently expose no
            // object-safe `utilities()` accessor (mid-refactor elsewhere toward a declarative window-kind
            // utility builder — unrelated to this ticket; see `tools`/`tool_measures` above for the analogous
            // mode-level mechanism that IS wired up). No puzzle2d scope ever requests `utilities: true` (it
            // uses static window-kind-scoped utilities only), so `request.utilities` is always empty in
            // practice; wire this up once the utilities API refactor lands.
            let _ = &request.utilities;
            if let Some(requested) = &request.engagements {
                let engagements = instance.app.window_engagements();
                let (hash, value) = ui_refresh_section(&engagements, requested.hash.as_deref());
                response.engagements = Some(SectionResponse { key: "engagements".into(), hash, value });
            }
            if let Some(requested) = &request.measures {
                let measures = instance.app.window_measures();
                let (hash, value) = ui_refresh_section(&measures, requested.hash.as_deref());
                response.measures = Some(SectionResponse { key: "measures".into(), hash, value });
            }
            if let Some(requested) = &request.tools {
                let tool_measures = instance.app.tool_measures();
                let (hash, value) = ui_refresh_section(&tool_measures, requested.hash.as_deref());
                response.tools = Some(SectionResponse { key: "tools".into(), hash, value });
            }
            // 🗣️ Labels no longer need a runtime overlay round-trip: the manifest itself now carries a full
            // `LocalizedLabel` matrix per field, resolved shell-side from the active locale/terminology —
            // see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND. A
            // `labels` refresh request now always comes back empty; kept accepting it (rather than erroring)
            // so an unupdated shell doesn't break.
            let _ = &request.labels;

            Ok(serde_json::to_string(&response).unwrap_or_else(|_| "{}".into()))
        })
        .map_err(|fault| fault.message)
    }
    //#endregion 🔖️RefreshUi

    //#region 🔖️ContextMenu
    /// 🖱️ Wire shape for an on-demand context-menu request — mirrors TS `PluginContextMenuRequest` minus
    /// `viewState` (B1 dropped `ViewModel` from `ArtifactApp::context_menu` entirely, so this struct no
    /// longer parses-and-discards a field it never forwards). Module-scoped (not nested in
    /// `plugin_context_menu`) so `plugin_exchange`'s `AppCommand::ContextMenu` arm below can decode the
    /// same typed shape directly off the binary wire instead of round-tripping through JSON strings.
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ContextMenuWireRequest {
        menu: UiMenuRef,
        #[serde(default)]
        surface: Option<ContextMenuSurfaceTarget>,
        #[serde(default)]
        window_instance_id: Option<String>,
        #[serde(default)]
        point: Option<ContextMenuPoint>,
    }

    impl From<ContextMenuWireRequest> for ContextMenuRequest {
        fn from(wire: ContextMenuWireRequest) -> Self {
            ContextMenuRequest { menu: wire.menu, surface: wire.surface, window_instance_id: wire.window_instance_id, point: wire.point }
        }
    }

    /// 🖱️ On-demand context-menu computation (WIT `context-menu` export's SDK entry point) — never
    /// cached, never part of `refresh_ui`. String-in/string-out JSON entry point for the WIT boundary.
    pub fn plugin_context_menu(instance_id: u32, request_json: &str) -> Result<String, String> {
        let wire: ContextMenuWireRequest = serde_json::from_str(request_json).map_err(|error| error.to_string())?;
        let request: ContextMenuRequest = wire.into();
        with_instances_mut(|list| {
            let instance = find_instance(list, instance_id)?;
            let items = instance.app.context_menu(&request);
            Ok(serde_json::to_string(&ContextMenuResponse { items }).unwrap_or_else(|_| r#"{"items":[]}"#.into()))
        })
        .map_err(|fault| fault.message)
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️Exchange
    /// 🔍️ `SectionProbe.kind` byte convention for `plugin_exchange`'s `AppCommand::RefreshUi` handling —
    /// no shared WIT/protocol_channel enum exists for this yet (Wave 1 scope), so the mapping lives here,
    /// the single producer+consumer of it until a TS-side client needs the same constants.
    const SECTION_KIND_WINDOW: u8 = 0;
    const SECTION_KIND_PANEL: u8 = 1;
    const SECTION_KIND_ENGAGEMENTS: u8 = 2;
    const SECTION_KIND_MEASURES: u8 = 3;
    const SECTION_KIND_TOOLS: u8 = 4;
    const SECTION_KIND_LABELS: u8 = 5;

    /// 🎯️ `AppCommand::ArtifactCommand`'s Wave 1 mapping — the same magic action-name strings
    /// `VcsArtifactApp::dispatch_action` already intercepts for `store::ArtifactCommand` verbs (duplicated
    /// here rather than imported since `app`'s `HISTORY_ACTION_IDS` const is private — see
    /// `plugin_exchange`'s doc for why this frame is scoped to history verbs only in Wave 1).
    const DOCUMENT_COMMAND_ACTION_IDS: [&str; 6] = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"];

    /// 📤️ Appends `AppFrame::Effects`/`AppFrame::Events` for `result`'s side channels, if any — shared by
    /// `AppCommand::Command`'s and `AppCommand::ArtifactCommand`'s handling below.
    fn push_invocation_side_frames(frames: &mut Vec<protocol::AppFrame>, seq: u64, result: &InvocationResult) {
        if !result.requested_effects.is_empty() {
            let effects = result.requested_effects.iter().map(|effect| encode_wire_serialized(effect)).collect();
            frames.push(protocol::AppFrame::Effects { in_reply_to: Some(seq), effects });
        }
        if !result.events.is_empty() {
            let events = result.events.iter().map(|event| encode_wire_serialized(event)).collect();
            frames.push(protocol::AppFrame::Events { in_reply_to: Some(seq), events });
        }
    }

    /// 🔀️ The single bidirectional entry point behind WIT `exchange` (see `📜️wit/📜️world.wit`'s
    /// `interface plugin` doc) — decodes each `protocol::AppCommand` in `commands`, dispatches it against
    /// `instance_id`, and returns every `protocol::AppFrame` produced, encoded back to bytes.
    ///
    /// 🚧️ Wave 1 scope — documented here rather than silently: `AppCommand::CommandText` is stubbed
    /// (`Error{code:"unsupported"}`, a later wave wires real headless op-text scripts);
    /// `AppCommand::ArtifactCommand` only accepts the six history verbs (`DOCUMENT_COMMAND_ACTION_IDS`),
    /// mapped onto the existing magic action-name interception rather than a real typed
    /// `store::ArtifactCommand` wire codec; `AppCommand::RefreshUi` carries a packed `view_state` so first-paint
    /// labels/locale resolve correctly before any `AppCommand::Command` has been seen (via
    /// `instance_view_state`), defaulting to `ViewModel::default()` before any `Command` has been
    /// processed; the unsolicited outbox (backbone-driven `AppFrame::DocumentChanged`, a persistent
    /// per-instance frame queue surviving across calls) is NOT implemented — only `pending_effects` is
    /// drained, once, at the end of the batch, whenever a dispatched command mutated the document,
    /// mirroring exactly where `plugin_refresh_ui` already calls it.
    pub fn plugin_exchange(instance_id: u32, commands: &[Vec<u8>]) -> Result<Vec<Vec<u8>>, Fault> {
        let mut frames: Vec<protocol::AppFrame> = Vec::new();
        let mut mutated = false;

        for bytes in commands {
            let command = protocol::decode_app_command(bytes).map_err(|error| error.into_fault())?;
            match command {
                protocol::AppCommand::Hello { channel_version, app_id: _, actor, config } => {
                    if channel_version != protocol::CHANNEL_VERSION {
                        push_os_fault(&mut frames, None, "channel-version", format!("expected channel version {}, got {channel_version}", protocol::CHANNEL_VERSION));
                        continue;
                    }
                    set_instance_actor(instance_id, actor);
                    if !config.is_empty() {
                        // 🧮️ B1: `Hello.config` carries the SAME `store::encode_document_pack_bytes(pack,
                        // spr)` wire shape `AppCommand::LoadConfig` does — a whole config-artifact snapshot,
                        // loaded through the real config `ArtifactStore` rather than the deleted
                        // `apply_config_bytes` whole-record-replace legacy path.
                        let loaded = store::decode_document_pack_bytes(&config).map_err(|error| error.into_fault()).and_then(|(pack, spr)| {
                            with_instances_mut(|list| {
                                let instance = find_instance(list, instance_id)?;
                                instance.app.load_config_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() })
                            })
                        });
                        if let Err(fault) = loaded {
                            push_app_fault(&mut frames, None, fault);
                            continue;
                        }
                    }
                    let manifest_bytes = encode_wire_serialized(&plugin_manifest());
                    frames.push(protocol::AppFrame::Welcome { channel_version: protocol::CHANNEL_VERSION, instance: instance_id, manifest: manifest_bytes });
                }
                protocol::AppCommand::ConfigCommand { seq, command } => {
                    // 🧮️ B1: `command` is a binary-encoded `store::ArtifactCommand<A::ConfigMutation>` —
                    // real dispatch against the config store (replaces the deleted `apply_config_bytes`
                    // whole-record-replace legacy path); undo/redo/checkpoint all work on config now.
                    let meta = ActionMeta { actor: instance_actor(instance_id), instance_id };
                    let dispatched = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.dispatch_config_command(&command, &meta)
                    });
                    match dispatched {
                        Ok(result) => {
                            mutated = true;
                            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                            push_invocation_side_frames(&mut frames, seq, &result);
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::Command { seq, command, view_state: _view_state } => {
                    let meta = ActionMeta { actor: instance_actor(instance_id), instance_id };
                    let dispatched = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.handle_command_frame(&command, &meta)
                    });
                    match dispatched {
                        Ok(result) => {
                            mutated = true;
                            let output = encode_wire_serialized(&result.output);
                            let diagnostics = encode_wire_serialized(&result.diagnostics);
                            frames.push(protocol::AppFrame::Invocation { in_reply_to: seq, output, diagnostics });
                            push_invocation_side_frames(&mut frames, seq, &result);
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::CommandText { seq, line: _ } => {
                    // 🚧️ Wave 1 stub — see this function's doc comment.
                    push_os_fault(&mut frames, Some(seq), "unsupported", "CommandText not yet wired".into());
                }
                protocol::AppCommand::RefreshUi { seq, sections, view_state } => {
                    let view_state = decode_view_state(&view_state);
                    let outcome = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        let mut section_frames = Vec::new();
                        for probe in &sections {
                            let (hash, body) = match probe.kind {
                                SECTION_KIND_WINDOW | SECTION_KIND_PANEL => {
                                    let node = instance.app.render(&probe.key, None, &view_state)?;
                                    channel_refresh_section(&node, probe.hash)
                                }
                                SECTION_KIND_ENGAGEMENTS => channel_refresh_section(&instance.app.window_engagements(), probe.hash),
                                SECTION_KIND_MEASURES => channel_refresh_section(&instance.app.window_measures(), probe.hash),
                                SECTION_KIND_TOOLS => channel_refresh_section(&instance.app.tool_measures(), probe.hash),
                                SECTION_KIND_LABELS => (0u64, None),
                                _ => (0u64, None),
                            };
                            section_frames.push(protocol::AppFrame::UiSection { in_reply_to: Some(seq), kind: probe.kind, key: probe.key.clone(), hash, body: body.map(|value| encode_wire_serialized(&value)) });
                        }
                        let pending = instance.app.pending_effects();
                        Ok((section_frames, pending))
                    });
                    match outcome {
                        Ok((section_frames, pending_effects)) => {
                            frames.extend(section_frames);
                            if !pending_effects.is_empty() {
                                let encoded = pending_effects.iter().map(|effect| encode_wire_serialized(effect)).collect();
                                frames.push(protocol::AppFrame::Effects { in_reply_to: Some(seq), effects: encoded });
                            }
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::ContextMenu { seq, request } => {
                    // 🗂️ Decodes straight into the typed wire shape and encodes the typed response straight
                    // back out — no intermediate `Value`/JSON-string hop through `plugin_context_menu`
                    // (which stays as the separate string-in/string-out entry point the WIT boundary needs).
                    match decode_wire_serialized::<ContextMenuWireRequest>(&request) {
                        Ok(wire) => {
                            let request: ContextMenuRequest = wire.into();
                            let outcome = with_instances_mut(|list| {
                                let instance = find_instance(list, instance_id)?;
                                Ok(instance.app.context_menu(&request))
                            });
                            match outcome {
                                Ok(items) => frames.push(protocol::AppFrame::ContextMenu { in_reply_to: seq, items: encode_wire_serialized(&items) }),
                                Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                            }
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::ArtifactCommand { seq, command } => {
                    let envelope: Value = decode_wire_serialized_or(&command, Value::Null);
                    let action = envelope.get("action").and_then(Value::as_str).unwrap_or("").to_string();
                    let args = envelope.get("args").cloned();
                    if !DOCUMENT_COMMAND_ACTION_IDS.contains(&action.as_str()) {
                        push_os_fault(&mut frames, Some(seq), "unsupported", format!("ArtifactCommand action {action:?} not supported (Wave 1: history verbs only)"));
                        continue;
                    }
                    let meta = ActionMeta { actor: instance_actor(instance_id), instance_id };
                    let dispatched = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.handle_action(&action, args.as_ref(), &meta)
                    });
                    match dispatched {
                        Ok(result) => {
                            mutated = true;
                            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                            push_invocation_side_frames(&mut frames, seq, &result);
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::ApplyEnvelopes { seq, envelopes } => {
                    let encoded = protocol::encode_envelopes(&envelopes);
                    match plugin_ingest_operations(instance_id, &encoded) {
                        Ok(()) => {
                            mutated = true;
                            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::LoadDocument { seq, pack, spr } => {
                    let files = store::ArtifactPackFiles { pack, spr, ops: String::new() };
                    match plugin_load_document_pack(instance_id, &files) {
                        Ok(()) => {
                            mutated = true;
                            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::ReadDocument { seq } => match plugin_document_pack(instance_id) {
                    Ok(files) => frames.push(protocol::AppFrame::Document { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                },
                // 🧸️ Composed children are their own envelopes with their own histories, so they
                // need their own load/read pair — a parent's `LoadDocument`/`Document` carries none
                // of them, and before this existed a genesis child lived only until the process
                // ended.
                protocol::AppCommand::LoadChildren { seq, entries } => {
                    let loaded = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        for entry in &entries {
                            let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), error))?;
                            instance.app.load_child_pack(&entry.slot, &entry.child_id, dialect, &entry.envelope_pack)?;
                        }
                        Ok(())
                    });
                    match loaded {
                        Ok(()) => {
                            mutated = true;
                            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::ReadChildren { seq } => {
                    let read = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.child_packs()
                    });
                    match read {
                        Ok(entries) => frames.push(protocol::AppFrame::Children { in_reply_to: seq, entries }),
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::LoadConfig { seq, pack, spr } => {
                    // 🧮️ B1: real work — loads straight into the config `ArtifactStore` (was routed
                    // through the deleted `apply_config_bytes` whole-record-replace legacy path).
                    let applied = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.load_config_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() })
                    });
                    match applied {
                        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::ReadConfig { seq } => {
                    // 🧮️ B1: real work — the config store's current pack+spr+ops (was a stub always
                    // returning empty bytes).
                    let read = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.config_pack()
                    });
                    match read {
                        Ok(files) => frames.push(protocol::AppFrame::Config { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::AttachBackbone { seq, uri } => match plugin_attach_backbone(instance_id, &uri) {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                },
                protocol::AppCommand::DetachBackbone { seq } => match plugin_detach_backbone(instance_id) {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                },
                protocol::AppCommand::MediaIn { seq, port, descriptor, data } => {
                    let descriptor_value: Value = decode_wire_serialized_or(&descriptor, Value::Null);
                    let descriptor_json = serde_json::to_string(&descriptor_value).unwrap_or_else(|_| "{}".into());
                    match plugin_consume_media(instance_id, &port, &descriptor_json, data) {
                        Ok(()) => {
                            mutated = true;
                            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::MediaOut { seq, port, request: _ } => match plugin_produce_media(instance_id, &port, "") {
                    Ok((descriptor_json, data)) => {
                        let descriptor_value: Value = serde_json::from_str(&descriptor_json).unwrap_or(Value::Null);
                        frames.push(protocol::AppFrame::Media { in_reply_to: seq, port, descriptor: encode_wire_serialized(&descriptor_value), data });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                },
                protocol::AppCommand::MediaFingerprint { seq, port } => {
                    let fingerprint = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.media_fingerprint(&port).map_err(|error| plugin_internal_fault(error.to_string()))
                    });
                    match fingerprint {
                        Ok(fingerprint) => {
                            let value = serde_json::to_value(&fingerprint).unwrap_or_default();
                            frames.push(protocol::AppFrame::MediaFingerprint { in_reply_to: seq, port, fingerprint: encode_wire_serialized(&value) });
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::PureCommand {
                    seq,
                    command,
                    document,
                    document_spr,
                    config,
                    config_spr,
                    draft,
                    draft_spr,
                } => {
                    let meta = ActionMeta { actor: instance_actor(instance_id), instance_id };
                    let dispatched = with_instances_mut(|list| {
                        let instance = find_instance(list, instance_id)?;
                        instance.app.hydrate_document_lane(&document, &document_spr)?;
                        instance.app.hydrate_config_lane(&config, &config_spr)?;
                        instance.app.hydrate_draft_lane(&draft, &draft_spr)?;
                        let result = instance.app.handle_command_frame(&command, &meta)?;
                        let emit_wire = instance.app.take_last_emit_wire().unwrap_or_default();
                        Ok((result, emit_wire))
                    });
                    match dispatched {
                        Ok((result, (document_ops, config_ops, draft_ops))) => {
                            mutated = true;
                            let output = encode_wire_serialized(&result.output);
                            let diagnostics = encode_wire_serialized(&result.diagnostics);
                            frames.push(protocol::AppFrame::Emit {
                                in_reply_to: seq,
                                document_ops,
                                config_ops,
                                draft_ops,
                                output,
                                diagnostics,
                            });
                            push_invocation_side_frames(&mut frames, seq, &result);
                        }
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault),
                    }
                }
                protocol::AppCommand::Bye => {}
            }
        }

        if mutated {
            let effects = with_instances_mut(|list| {
                let instance = find_instance(list, instance_id)?;
                Ok(instance.app.pending_effects())
            })
            .unwrap_or_default();
            if !effects.is_empty() {
                let encoded = effects.iter().map(|effect| encode_wire_serialized(effect)).collect();
                frames.push(protocol::AppFrame::Effects { in_reply_to: None, effects: encoded });
            }
        }

        Ok(frames.iter().map(protocol::encode_app_frame).collect())
    }
    //#endregion 🔖️Exchange

    #[macro_export]
    macro_rules! plugin_exports {
        ($bundle_fn:expr) => {
            fn __semio_install_plugin_bundle() {
                $crate::plugin_runtime::install_plugin_bundle(($bundle_fn)());
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_plugin_bundle_installer_link_shim() {
                $crate::plugin_runtime::register_plugin_bundle_installer(__semio_install_plugin_bundle);
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_plugin_install_bundle() {
                __semio_install_plugin_bundle();
            }

            #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
            #[used]
            static _SEMIO_PLUGIN_COMPONENT_LINK: fn() = $crate::component_export_anchor;
        };
    }

    //#region 🧩️Extension
    /// 🧩️ Extension guest bundle — no apps; contributes + invoke handlers.
    pub struct ExtensionBundle {
        pub manifest: ExtensionManifest,
        handlers: HashMap<String, Box<dyn Fn(&[u8]) -> Result<Vec<u8>, Fault> + Send + 'static>>,
    }

    /// 📦️ Manifest for a runtime-installable extension (WIT `extension::manifest` payload).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ExtensionManifest {
        pub extension_id: String,
        pub label: String,
        pub version: String,
        pub extends: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub capabilities: Vec<CapabilityRequirement>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub topic_contributions: Vec<TopicContribution>,
    }

    impl ExtensionBundle {
        /// 🧩️ Starts an extension bundle with identity + version.
        pub fn new(extension_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
            Self {
                manifest: ExtensionManifest {
                    extension_id: extension_id.into(),
                    label: label.into(),
                    version: version.into(),
                    extends: String::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                },
                handlers: HashMap::new(),
            }
        }

        /// 🔗 Declares the host app/plugin this extension extends.
        pub fn extends(mut self, extends: impl Into<String>) -> Self {
            self.manifest.extends = extends.into();
            self
        }

        /// 🔒️ Declares a capability requirement for the extension.
        pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
            if !self.manifest.capabilities.contains(&capability) {
                self.manifest.capabilities.push(capability);
            }
            self
        }

        /// 🗂️ Adds an open topic contribution to the extension manifest; `topic` reuses this crate's
        /// own `contributes`/`consumes` metadata vocabulary (e.g. `"cad.computer"`).
        pub fn contributes_topic(mut self, topic: impl Into<String>, payload: Value) -> Self {
            self.manifest.topic_contributions.push(TopicContribution::new(topic, payload));
            self
        }

        /// 🔀️ Registers a capability handler invoked via WIT `extension::invoke`.
        pub fn handler(mut self, capability: impl Into<String>, handler: impl Fn(&[u8]) -> Result<Vec<u8>, Fault> + Send + 'static) -> Self {
            self.handlers.insert(capability.into(), Box::new(handler));
            self
        }
    }

    thread_local! {
        static EXTENSION_BUNDLE: RefCell<Option<ExtensionBundle>> = const { RefCell::new(None) };
        static EXTENSION_ACTIVE: Cell<bool> = const { Cell::new(false) };
    }

    /// 📤️ Installs the process-local extension bundle (from `extension_exports!`).
    pub fn install_extension_bundle(bundle: ExtensionBundle) {
        EXTENSION_BUNDLE.with(|slot| {
            *slot.borrow_mut() = Some(bundle);
        });
        EXTENSION_ACTIVE.with(|slot| slot.set(false));
    }

    static EXTENSION_BUNDLE_INSTALLER: std::sync::OnceLock<fn()> = std::sync::OnceLock::new();

    /// 🧩️ Registers the embedding extension crate's bundle installer (expanded from `extension_exports!`).
    pub fn register_extension_bundle_installer(install: fn()) {
        let _ = EXTENSION_BUNDLE_INSTALLER.set(install);
    }

    /// 🔗️ Weak default mirroring `semio_plugin_bundle_installer_link_shim` (see that symbol's own
    /// doc comment): lets an intermediate link succeed before the embedding extension crate's
    /// `extension_exports!` provides the strong override. Without this default AND the explicit call
    /// below, `EXTENSION_BUNDLE_INSTALLER` is never populated — no code path ever invoked this
    /// symbol, so `register_extension_bundle_installer` was silently never called and every real
    /// extension's `manifest()`/`activate()` observed only the empty-default `ExtensionBundle`.
    #[cfg(feature = "component-extension-guest")]
    #[unsafe(no_mangle)]
    #[linkage = "weak"]
    pub extern "C" fn semio_extension_bundle_installer_link_shim() {}

    /// Ensures the embedding extension crate's bundle installer ran before any WIT export is served
    /// — mirrors `ensure_plugin_initialized`'s explicit weak/strong-linkage shim call.
    fn ensure_extension_initialized() {
        EXTENSION_BUNDLE.with(|slot| {
            if slot.borrow().is_none() {
                #[cfg(feature = "component-extension-guest")]
                unsafe {
                    semio_extension_bundle_installer_link_shim();
                }
                if let Some(install) = EXTENSION_BUNDLE_INSTALLER.get() {
                    install();
                }
            }
        });
    }

    /// 📦️ Returns the installed extension manifest (empty defaults when unset).
    pub fn extension_manifest() -> ExtensionManifest {
        ensure_extension_initialized();
        EXTENSION_BUNDLE.with(|slot| {
            slot.borrow()
                .as_ref()
                .map(|bundle| bundle.manifest.clone())
                .unwrap_or_else(|| ExtensionManifest {
                    extension_id: String::new(),
                    label: String::new(),
                    version: String::new(),
                    extends: String::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                })
        })
    }

    /// 🚨️ Marks the extension active for subsequent `extension_invoke` calls.
    pub fn extension_activate() -> Result<(), Fault> {
        ensure_extension_initialized();
        let ready = EXTENSION_BUNDLE.with(|slot| slot.borrow().is_some());
        if !ready {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.missing"), "extension bundle not installed"));
        }
        EXTENSION_ACTIVE.with(|slot| slot.set(true));
        Ok(())
    }

    /// 🛑 Clears the active flag without dropping handlers.
    pub fn extension_deactivate() {
        EXTENSION_ACTIVE.with(|slot| slot.set(false));
    }

    /// 🔀️ Dispatches `capability` to the registered handler with wire-encoded `request` bytes.
    pub fn extension_invoke(capability: &str, request: &[u8]) -> Result<Vec<u8>, Fault> {
        ensure_extension_initialized();
        if !EXTENSION_ACTIVE.with(|slot| slot.get()) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.inactive"), "extension not activated"));
        }
        EXTENSION_BUNDLE.with(|slot| {
            let bundle = slot.borrow();
            let Some(bundle) = bundle.as_ref() else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.missing"), "extension bundle not installed"));
            };
            let Some(handler) = bundle.handlers.get(capability) else {
                return Err(Fault::new(
                    FaultOrigin::Plugin,
                    FaultCode::new("extension.unknown-capability"),
                    format!("unknown extension capability '{capability}'"),
                ));
            };
            handler(request)
        })
    }

    /// 🧩️ Installs an extension crate's bundle builder into TLS for WIT guest exports.
    #[macro_export]
    macro_rules! extension_exports {
        ($bundle_fn:expr) => {
            fn __semio_install_extension_bundle() {
                $crate::plugin_runtime::install_extension_bundle(($bundle_fn)());
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_bundle_installer_link_shim() {
                $crate::plugin_runtime::register_extension_bundle_installer(__semio_install_extension_bundle);
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_install_bundle() {
                __semio_install_extension_bundle();
            }

            #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
            #[used]
            static _SEMIO_EXTENSION_COMPONENT_LINK: fn() = $crate::extension_component_export_anchor;
        };
    }
    //#endregion 🧩️Extension


    /// 🏗️ Plugin registration lives in each owner's root `🦀️component.rs` via
    /// [`Plugin::builder`](crate::Plugin::builder) + [`plugin_exports!`](crate::plugin_exports).
    /// The retired `semio_plugin!` macro is gone — typestate on the builder makes missing identity fields a compile error.

    #[cfg(test)]
    mod plugin_builder_contract_tests {
        //! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
        //! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
        //! operations that emit no document operations, history interception, and remote-operation ingest
        //! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
        //! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
        //! `handle_command` — everything app-specific dispatches via `dispatch_typed`.
        use ui_wgpu::wgpu::{Label, LocalizedLabel};

        use super::ContextMenuWireRequest;
        use crate::app::{ui_history_panel, ActionMeta, App, AppActionRegistry, ChildEmit, CommandView, ConfigView, ArtifactApp, ArtifactView, DraftView, Emit, HistoryCommandFilter, HistoryView, Menu, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, PluginApp, VcsArtifactApp};
        use semio_framework::kernel::ArtifactHandle;
        use crate::app::{ArtifactSerializer, ArtifactDeserializer, serializer_entry_of, deserializer_entry_of, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload};
        use store::EngineHandles;
        use semio_framework::Fault;
        use crate::{selection_count_phrase, ui_text, IconName, MediaClass, MediaType, SurfaceKind, UiNode, ViewModel};
        use protocol::{Mutation, MutationDiff};
        use semio_framework::kernel::{AppEvent, ClipboardError, ClipboardFragment, HostEffect, PasteAnchor, PastePlacement, UiDirtyScope};
        use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, MediaForm, NOTE_SHELL_COMMAND_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
        use serde::{Deserialize, Serialize};
        use serde_json::json;
        use store::{Backbone, BackboneMessage, MemoryBackbone};
        use ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY;
        use ui_wgpu::wgpu::{ContextMenuItemSpec, ContextMenuRequest, UiMenuRef};

        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
        #[dsl(extension = "testkit-macro")]
        struct TestSnapshot {
            count: i32,
            label: String,
        }

        /// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack for SDK test double (artifact coincides with snapshot only in tests).
        impl store::ArtifactDsl for TestSnapshot {
            const EXTENSION: &'static str = "testkit-macro";
            fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
                if text.trim().is_empty() {
                    return Ok(Self::default());
                }
                serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
            }
            fn print_dsl(&self) -> String {
                serde_json::to_string(self).unwrap_or_default()
            }
        }

        impl store::ArtifactPack for TestSnapshot {
            fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
            }
            fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                if bytes.is_empty() {
                    return Ok(Self::default());
                }
                serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
            }
        }

        /// 🧪️ Trivial dummy `ArtifactSerializer`/`ArtifactDeserializer` pair, round-tripping
        /// `TestSnapshot` to itself, for `serializer_entry_of`/`deserializer_entry_of` smoke tests.
        struct DummySerializer;
        impl ArtifactSerializer for DummySerializer {
            type From = TestSnapshot;
            type Into = TestSnapshot;
            const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
            const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
            fn serialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
                Ok(from.clone())
            }
        }

        struct DummyDeserializer;
        impl ArtifactDeserializer for DummyDeserializer {
            type From = TestSnapshot;
            type Into = TestSnapshot;
            const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
            const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
            fn deserialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
                Ok(from.clone())
            }
        }

        #[test]
        fn serializer_entry_of_and_deserializer_entry_of_erase_correctly() {
            let ser = serializer_entry_of::<DummySerializer>();
            assert_eq!(ser.writes, DummySerializer::INTO);
            assert_eq!(ser.reads.to_vec(), vec![DummySerializer::FROM]);
            let de = deserializer_entry_of::<DummyDeserializer>();
            assert_eq!(de.writes, DummyDeserializer::INTO);
            assert_eq!(de.reads.to_vec(), vec![DummyDeserializer::FROM]);

            let seed = TestSnapshot { count: 7, label: "x".into() };
            let bytes = store::ArtifactPack::encode_pack(&seed);
            let composed = (ser.compose)(&[ErasedComposeSource { dialect: DummySerializer::FROM, payload: IoPayload::Binary(bytes) }])
                .expect("serializer_entry_of erased compose should succeed with exactly 1 source");
            assert_eq!(composed.dialect, DummySerializer::INTO);
            match composed.payload {
                IoPayload::Binary(out) => assert_eq!(<TestSnapshot as store::ArtifactPack>::decode_pack(&out).unwrap(), seed),
                IoPayload::Text(_) => panic!("expected Binary payload"),
            }

            let zero_sources_err = match (de.compose)(&[]) {
                Err(err) => err,
                Ok(_) => panic!("deserializer_entry_of erased compose should reject 0 sources"),
            };
            assert!(zero_sources_err.message.contains("needs exactly 1 source"), "{}", zero_sources_err.message);
            let two_sources = [
                ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) },
                ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) },
            ];
            let two_sources_err = match (de.compose)(&two_sources) {
                Err(err) => err,
                Ok(_) => panic!("deserializer_entry_of erased compose should reject 2 sources"),
            };
            assert!(two_sources_err.message.contains("needs exactly 1 source"), "{}", two_sources_err.message);
        }

        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        struct TestDiff {
            count: Option<i32>,
            label: Option<String>,
        }

        impl MutationDiff<TestSnapshot> for TestDiff {
            fn apply(&self, snapshot: &TestSnapshot) -> TestSnapshot {
                TestSnapshot { count: self.count.unwrap_or(snapshot.count), label: self.label.clone().unwrap_or_else(|| snapshot.label.clone()) }
            }

            fn absorb(&mut self, other: Self) {
                if other.count.is_some() {
                    self.count = other.count;
                }
                if other.label.is_some() {
                    self.label = other.label;
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
        #[serde(tag = "operation", rename_all = "camelCase")]
        enum TestMutation {
            #[dsl(key = "set-count")]
            SetCount { value: i32 },
            #[dsl(key = "set-label")]
            SetLabel { value: String },
        }

        impl Mutation<TestSnapshot> for TestMutation {
            type Diff = TestDiff;

            fn diff(&self, _snapshot: &TestSnapshot) -> TestDiff {
                match self {
                    TestMutation::SetCount { value } => TestDiff { count: Some(*value), label: None },
                    TestMutation::SetLabel { value } => TestDiff { count: None, label: Some(value.clone()) },
                }
            }

            fn inverse(&self, snapshot: &TestSnapshot) -> Vec<Self> {
                match self {
                    TestMutation::SetCount { .. } => vec![TestMutation::SetCount { value: snapshot.count }],
                    TestMutation::SetLabel { .. } => vec![TestMutation::SetLabel { value: snapshot.label.clone() }],
                }
            }
        }


        impl ::protocol::OpText for TestMutation {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{keyword} ");
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                        let record = ::dsl::parse(
                            body,
                            &spec_fn(),
                            &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                        )?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                if body.is_empty() {
                    keyword
                } else {
                    format!("{keyword} {body}")
                }
            }
        }

        impl ::protocol::OpBinary for TestMutation {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::variants_binary::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::variants_binary::decode_op(bytes)
            }
        }

        /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): `ChildEmit::of` requires `SemanticMutation<S>`
        /// — this fixture lets both the PARENT (`TestApp`) and a CHILD (a bare `ArtifactStore<TestSnapshot,
        /// TestMutation>` registered via `VcsArtifactApp::register_child`) share the identical
        /// `TestSnapshot`/`TestMutation` pair, so the composition tests below need no second mutation enum.
        impl protocol::SemanticMutation<TestSnapshot> for TestMutation {
            fn kinds() -> &'static [protocol::SemanticDescriptor] {
                const KINDS: &[protocol::SemanticDescriptor] = &[
                    protocol::SemanticDescriptor { verb: "set", entity: "count", kind: "set-count", record: "SetCount" },
                    protocol::SemanticDescriptor { verb: "set", entity: "label", kind: "set-label", record: "SetLabel" },
                ];
                KINDS
            }
            fn semantics(&self) -> &'static protocol::SemanticDescriptor {
                match self {
                    TestMutation::SetCount { .. } => &Self::kinds()[0],
                    TestMutation::SetLabel { .. } => &Self::kinds()[1],
                }
            }
            fn label(&self) -> String {
                match self {
                    TestMutation::SetCount { value } => format!("Set count to {value}"),
                    TestMutation::SetLabel { value } => format!("Set label to {value}"),
                }
            }
            fn target(&self) -> Vec<String> {
                Vec::new()
            }
        }

        /// 🧪️ B1: `TestApp`'s config — `selected` moved out of an app-struct `RefCell` into a real config
        /// artifact (was ephemeral view state demonstrated via `ActionEmit::view_with_inverse`; now a
        /// config operation with a real `inverse`, proving the B1 replacement end to end).
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
        #[dsl(extension = "testkit-macro-cfg")]
        struct TestConfig {
            selected: Option<String>,
        }

        /// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack for SDK test double (artifact coincides with snapshot only in tests).
        impl store::ArtifactDsl for TestConfig {
            const EXTENSION: &'static str = "testkit-macro-cfg";
            fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
                if text.trim().is_empty() {
                    return Ok(Self::default());
                }
                serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
            }
            fn print_dsl(&self) -> String {
                serde_json::to_string(self).unwrap_or_default()
            }
        }

        impl store::ArtifactPack for TestConfig {
            fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
            }
            fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                if bytes.is_empty() {
                    return Ok(Self::default());
                }
                serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
            }
        }

        impl store::ConfigRecord for TestConfig {}

        impl MutationDiff<TestConfig> for TestConfig {
            fn apply(&self, _base: &TestConfig) -> TestConfig {
                self.clone()
            }
            fn absorb(&mut self, other: Self) {
                *self = other;
            }
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
        enum TestConfigMutation {
            #[dsl(key = "set-selected")]
            SetSelected { value: Option<String> },
            /// 🧮️ Full-snapshot restore — every `inverse()` below returns this, mirroring the
            /// `ShootingConfigMutation` pilot pattern (see `shooting_op`).
            #[dsl(key = "snapshot")]
            Snapshot { selected: Option<String> },
        }

        impl Mutation<TestConfig> for TestConfigMutation {
            type Diff = TestConfig;

            fn diff(&self, _base: &TestConfig) -> TestConfig {
                match self {
                    TestConfigMutation::SetSelected { value } => TestConfig { selected: value.clone() },
                    TestConfigMutation::Snapshot { selected } => TestConfig { selected: selected.clone() },
                }
            }

            fn inverse(&self, base: &TestConfig) -> Vec<Self> {
                vec![TestConfigMutation::Snapshot { selected: base.selected.clone() }]
            }
        }

        impl ::protocol::OpText for TestConfigMutation {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{keyword} ");
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                        let record = ::dsl::parse(
                            body,
                            &spec_fn(),
                            &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                        )?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                if body.is_empty() {
                    keyword
                } else {
                    format!("{keyword} {body}")
                }
            }
        }

        impl ::protocol::OpBinary for TestConfigMutation {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::variants_binary::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::variants_binary::decode_op(bytes)
            }
        }

        /// 🧪️ B1: `TestApp`'s typed command enum — the sole dispatch surface for its own behavior.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
        enum TestCommand {
            #[dsl(key = "increment")]
            Increment,
            #[dsl(key = "set-label")]
            SetLabel { value: String },
            #[dsl(key = "amend-label")]
            AmendLabel { value: String },
            #[dsl(key = "commit-label")]
            CommitLabel { value: String },
            #[dsl(key = "bad-view")]
            BadView,
            #[dsl(key = "select")]
            Select { id: Option<String> },
            #[dsl(key = "navigate")]
            Navigate,
            #[dsl(key = "noop-operation")]
            NoopMutation,
            #[dsl(key = "view-no-scope")]
            ViewNoScope,
            #[dsl(key = "view-partial-scope")]
            ViewPartialScope,
            #[dsl(key = "increment-via-command")]
            IncrementViaCommand,
            #[dsl(key = "set-label-via-command")]
            SetLabelViaCommand { value: String },
            #[dsl(key = "set-active-utility")]
            SetActiveUtility { utility_id: String },
            /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): exercises `Emit.child_emits` — emits a
            /// parent-document op alongside a `ChildEmit` targeting whichever `(slot, child_id)` the
            /// test registered via `VcsArtifactApp::register_child` beforehand.
            #[dsl(key = "composite-edit")]
            CompositeEdit { slot: String, child_id: String, child_value: i32 },
        }

        impl ::protocol::OpText for TestCommand {
            fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
                let variants = <Self as ::dsl::DslVariants>::variants();
                for (keyword, spec_fn) in &variants {
                    let probe = format!("{keyword} ");
                    if line == keyword.as_str() || line.starts_with(&probe) {
                        let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                        let record = ::dsl::parse(
                            body,
                            &spec_fn(),
                            &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline },
                        )?;
                        return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                    }
                }
                Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
            }
            fn print_op(&self) -> String {
                let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
                let variants = <Self as ::dsl::DslVariants>::variants();
                let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
                let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
                if body.is_empty() {
                    keyword
                } else {
                    format!("{keyword} {body}")
                }
            }
        }

        impl ::protocol::OpBinary for TestCommand {
            fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::dsl::variants_binary::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                ::dsl::variants_binary::decode_op(bytes)
            }
        }

        /// 🧪️ App under test. `received_actions` records every command id THIS app's own `handle` was
        /// actually called with — used to prove framework-owned interceptions (e.g. `noteShellCommand`)
        /// never reach it.
        #[derive(Default)]
        struct TestApp {
            received_actions: std::cell::RefCell<Vec<String>>,
        }

        impl ArtifactApp for TestApp {
            const APP_ID: &'static str = "synthetic-play";
            const DOCUMENT_SCHEMA: &'static str = "semio.test/v1";
            type Snapshot = TestSnapshot;
            type Mutation = TestMutation;
            type Config = TestConfig;
            type ConfigMutation = TestConfigMutation;
            type Draft = NoDraft;
            type DraftMutation = NoDraftMutation;
            type Presence = NoPresence;
            type PresenceMutation = NoPresenceMutation;
            type Transient = crate::app::NoTransient;
            type TransientMutation = crate::app::NoTransientMutation;
            type Command = TestCommand;

            fn initial_snapshot() -> TestSnapshot {
                TestSnapshot::default()
            }

            /// 👥️🫧️ Emits into BOTH ephemeral lanes on `increment`, so the dispatch path that
            /// reaches `presence_store`/`transient_store` is actually exercised rather than merely
            /// compiled. `Noop` is a real mutation of the `No*` lane types — it changes no content
            /// but does bump each store's generation, which is exactly the signal the host
            /// broadcasts (presence) and re-renders (transient) on.
            fn ephemeral(command: &TestCommand, _doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _presence: &crate::app::PresenceView<'_, NoPresence>, _transient: &crate::app::TransientView<'_, crate::app::NoTransient>) -> crate::app::EphemeralEmit<Self> {
                match command {
                    TestCommand::Increment => crate::app::EphemeralEmit { presence: vec![NoPresenceMutation::Noop], transient: vec![crate::app::NoTransientMutation::Noop] },
                    _ => crate::app::EphemeralEmit::default(),
                }
            }

            fn command_id(command: &TestCommand) -> &'static str {
                match command {
                    TestCommand::Increment => "increment",
                    TestCommand::SetLabel { .. } => "setLabel",
                    TestCommand::AmendLabel { .. } => "amendLabel",
                    TestCommand::CommitLabel { .. } => "commitLabel",
                    TestCommand::BadView => "badView",
                    TestCommand::Select { .. } => "select",
                    TestCommand::Navigate => "navigate",
                    TestCommand::NoopMutation => "noopMutation",
                    TestCommand::ViewNoScope => "viewNoScope",
                    TestCommand::ViewPartialScope => "viewPartialScope",
                    TestCommand::IncrementViaCommand => "incrementViaCommand",
                    TestCommand::SetLabelViaCommand { .. } => "setLabelViaCommand",
                    TestCommand::SetActiveUtility { .. } => "setActiveUtility",
                    TestCommand::CompositeEdit { .. } => "compositeEdit",
                }
            }

            fn handle(command: &TestCommand, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
                let _ = Self::command_id(command);
                match command {
                    TestCommand::Increment | TestCommand::IncrementViaCommand => Ok(Emit { artifact_mutations: vec![TestMutation::SetCount { value: doc.snapshot.count + 1 }], description: Some("increment".into()), ..Default::default() }),
                    TestCommand::SetLabel { value } => Ok(Emit { artifact_mutations: vec![TestMutation::SetLabel { value: value.clone() }], coalesce_key: Some("label".into()), ..Default::default() }),
                    TestCommand::SetLabelViaCommand { value } => Ok(Emit::mutations(vec![TestMutation::SetLabel { value: value.clone() }])),
                    TestCommand::AmendLabel { value } => Ok(Emit::amend(vec![TestMutation::SetLabel { value: value.clone() }], "label")),
                    TestCommand::CommitLabel { value } => Ok(Emit::commit(vec![TestMutation::SetLabel { value: value.clone() }], "commit label")),
                    TestCommand::BadView => Ok(Emit::mutations(vec![TestMutation::SetCount { value: 99 }])),
                    TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
                    TestCommand::Select { id } => Ok(Emit::config(vec![TestConfigMutation::SetSelected { value: id.clone() }])),
                    TestCommand::Navigate => Ok(Emit::effect(HostEffect::Navigate { uri: "semio://home".into() })),
                    TestCommand::NoopMutation => Ok(Emit::default()),
                    TestCommand::ViewNoScope => Ok(Emit { ui_scope: UiDirtyScope::None, ..Default::default() }),
                    TestCommand::ViewPartialScope => Ok(Emit {
                        ui_scope: UiDirtyScope::Partial { window_bodies: vec!["some.window".into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
                        ..Default::default()
                    }),
                    TestCommand::CompositeEdit { slot, child_id, child_value } => Ok(Emit {
                        artifact_mutations: vec![TestMutation::SetLabel { value: "composite".into() }],
                        child_emits: vec![ChildEmit::of::<TestSnapshot, _>(slot.clone(), child_id.clone(), vec![TestMutation::SetCount { value: *child_value }])],
                        ..Default::default()
                    }),
                }
            }

            fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> UiNode {
                ui_text(Label::data(format!("count={}", doc.snapshot.count)))
            }

            fn clipboard_media_type() -> Option<MediaType> {
                Some(MediaType { class: MediaClass::Data, form: MediaForm::Value })
            }

            fn copy_fragment(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> Result<ClipboardFragment, ClipboardError> {
                if doc.snapshot.label.is_empty() {
                    return Err(ClipboardError::EmptySelection);
                }
                Ok(ClipboardFragment {
                    schema: Self::DOCUMENT_SCHEMA.to_string(),
                    media_type: Self::clipboard_media_type().expect("declared above"),
                    dsl_text: doc.snapshot.label.clone(),
                    pack_bytes: None,
                    source_app: Self::APP_ID.to_string(),
                    label: doc.snapshot.label.clone(),
                })
            }

            fn cut_operations(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> Vec<TestMutation> {
                if doc.snapshot.label.is_empty() {
                    Vec::new()
                } else {
                    vec![TestMutation::SetLabel { value: String::new() }]
                }
            }

            fn paste_operations(_doc: &ArtifactView<'_, TestSnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<TestMutation>, ClipboardError> {
                if !Self::clipboard_accepts().contains(&fragment.media_type) {
                    return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
                }
                let value = match placement.anchor {
                    PasteAnchor::Original => fragment.dsl_text.clone(),
                    _ => format!("{}-{:?}", fragment.dsl_text, placement.anchor),
                };
                Ok(vec![TestMutation::SetLabel { value }])
            }

            /// 🧪️ Menu = always "setLabelRequired"; "incrementViaCommand" gated on a non-empty label
            /// (a selection-guard stand-in) — exercises `Menu::action`/`Menu::command`/`Menu::when`. The
            /// `flatLeaf1..10` branch only fires for the magic `"flat-menu-test"` label (so
            /// `contract_registry`-backed tests, which never set that label, are untouched) — a flat >9-row
            /// menu fixture for `context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu` below,
            /// proving `VcsArtifactApp::context_menu` runs every emitter through `organize_context_menu`.
            fn context_menu(_request: &ContextMenuRequest, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
                Menu::of(registry)
                    .action("setLabelRequired")
                    .when(!doc.snapshot.label.is_empty() && doc.snapshot.label != "flat-menu-test", |m| m.command("incrementViaCommand"))
                    .when(doc.snapshot.label == "flat-menu-test", |m| (1..=10).fold(m, |m, index| m.action(format!("flatLeaf{index}"))))
                    .build()
            }
        }

        fn meta() -> ActionMeta {
            ActionMeta { actor: "local".into(), instance_id: 1 }
        }

        fn synthetic_play_app() -> App {
            App::from_builder(App::builder("synthetic-play", LocalizedLabel::data("Synthetic")).document(["state"]).mode("edit", LocalizedLabel::data("Edit"), "pencil").window_kind(
                "main",
                LocalizedLabel::data("Main"),
                "synthetic.main",
                SurfaceKind::Canvas2d,
                IconName::AppWindow,
            ))
        }

        /// 🧪️ A registry-backed app declaring the contract-enforcement fixtures: an operation resolved by
        /// the context-menu label lookup, a declared-but-empty operation, a mis-behaving View action, and a
        /// utility (which auto-injects the `setActiveUtility` View action). B1: `setLabelRequired`'s
        /// required/default-arg materialization tests were deleted — that mechanism was JSON-args-specific
        /// (`AppActionRegistry`/`materialize_args`) and has no meaning for a typed `Self::Command` value a
        /// Rust caller constructs directly (a "missing required field" is a compile error, not a runtime
        /// one). `setLabelRequired` stays declared here purely as a registry fixture for the context-menu
        /// label-resolution test below.
        fn contract_registry() -> AppActionRegistry {
            let app = App::from_builder(
                App::builder("synthetic-play", LocalizedLabel::data("Synthetic"))
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .mutation("setLabelRequired", LocalizedLabel::data("Set Label"))
                .action_args("setLabelRequired", vec![ActionArgDef::text("value", LocalizedLabel::data("Value")).required()])
                // 🧪️ `Mutation`-kind by declaration, but `TestApp` emits zero operations for it — the
                // "declared Mutation action that happened to produce nothing" fixture.
                .mutation("noopMutation", LocalizedLabel::data("Noop Mutation"))
                .view_action("badView", LocalizedLabel::data("Bad View"))
                .utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush)
                .app_command("incrementViaCommand", LocalizedLabel::data("Increment"), "counter")
                .app_command("setLabelViaCommand", LocalizedLabel::data("Set Label"), "counter"),
            );
            AppActionRegistry::from_definition(&app.definition)
        }

        fn contract_app_under_test() -> VcsArtifactApp<TestApp> {
            VcsArtifactApp::with_registry(TestApp::default(), contract_registry())
        }

        fn synthetic_setup() {}

        fn __semio_plugin_bundle() -> crate::Plugin {
            synthetic_setup();
            crate::Plugin::builder("synthetic")
                .label("Synthetic")
                .version("0.0.1")
                .register_document_app::<TestApp>(synthetic_play_app())
                .build()
        }

        #[test]
        fn plugin_builder_builds_bundle_from_fluent_spec() {
            let bundle = __semio_plugin_bundle();
            assert_eq!(bundle.manifest.plugin_id, "synthetic");
            assert_eq!(bundle.manifest.label.as_str(), "Synthetic");
            assert_eq!(bundle.manifest.version, "0.0.1");
            assert!(bundle.manifest.apps.iter().any(|app| app.id == "synthetic-play"));
        }

        #[test]
        fn plugin_builder_wires_app_factory_for_create_app() {
            let bundle = __semio_plugin_bundle();
            let app = bundle.create_app("synthetic-play").expect("registered app");
            assert_eq!(app.app_id(), "synthetic-play");
            assert!(bundle.create_app("unknown-app").is_none());
        }

        #[test]
        fn operation_action_emits_kernel_op_with_true_inverse() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");
            assert_eq!(result.mutations.len(), 1);
            assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetCount { value: 1 }).unwrap());
            assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount { value: 0 }).unwrap()]));
            assert_eq!(result.inverse_group.mutations.len(), 1);
            assert_eq!(app.test_snapshot().count, 1);
        }

        //#region 🔖️EphemeralLaneTests
        #[test]
        fn a_command_reaches_both_ephemeral_lanes_without_touching_history() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            assert_eq!(app.presence_store.generation(), 0);
            assert_eq!(app.transient_store.generation(), 0);

            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");

            assert_eq!(app.presence_store.generation(), 1, "presence lane never received the command's emission");
            assert_eq!(app.transient_store.generation(), 1, "transient lane never received the command's emission");

            // 🧾️ Neither ephemeral lane may appear in history: they have no edits, no undo, and no
            // command-log rows of their own — the document's single edit is the only thing recorded.
            assert_eq!(app.test_store().envelope().vcs.edits.len(), 1, "an ephemeral lane leaked into the document's edit log");

            // ↩️ Undo rolls back the DOCUMENT; the ephemeral lanes are not restored, because they
            // were never part of the undoable gesture in the first place.
            app.dispatch_action("undo", None, &meta()).expect("undo");
            assert_eq!(app.test_snapshot().count, 0);
            assert_eq!(app.presence_store.generation(), 1, "undo must not rewind presence");
            assert_eq!(app.transient_store.generation(), 1, "undo must not rewind transient");
        }

        #[test]
        fn a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::SetLabel { value: "x".into() }, &meta()).expect("set label");
            assert_eq!(app.presence_store.generation(), 0);
            assert_eq!(app.transient_store.generation(), 0);
        }
        //#endregion 🔖️EphemeralLaneTests

        //#region 🔖️CompositionTests
        /// 🧪️ A live child `ArtifactStore<TestSnapshot, TestMutation>`, boxed as `Box<dyn
        /// SpaceMember>` — the shape `VcsArtifactApp::register_child`/`open_child` expect. Built
        /// directly (no real `ChildStoreFactory` registration needed for these in-process tests) —
        /// the SAME `TestSnapshot`/`TestMutation` pair `TestApp` itself uses, so a "child" here is
        /// just a second, independently-owned instance of the identical document shape.
        fn new_test_child(id: &str) -> Box<dyn store::SpaceMember> {
            let envelope = store::create_document_envelope::<TestSnapshot, TestMutation>("semio.test/v1", id, TestSnapshot::default(), None);
            Box::new(store::ArtifactStore::new(envelope))
        }

        fn test_child_dialect() -> store::os_io::ArtifactDialect {
            store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
        }

        #[test]
        fn composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.register_child("slot", "child-1", test_child_dialect(), new_test_child("child-1")).expect("register child seeds ownership");

            let result = app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).expect("composite edit");

            // 🧾️ One `KernelMutation` for the parent's own op, one for the child's — each carrying
            // its OWN document handle, never the parent's, for the child entry (Task 3's "REAL
            // target" requirement).
            assert_eq!(result.mutations.len(), 2);
            let parent_handle = ArtifactHandle(meta().instance_id as u128);
            let child_handle = crate::app::artifact_handle_of("child-1");
            assert_ne!(parent_handle, child_handle);
            let mutation_documents: std::collections::HashSet<ArtifactHandle> = result.mutations.iter().map(|mutation| mutation.document).collect();
            assert!(mutation_documents.contains(&parent_handle), "the parent's own edit must carry the parent's handle");
            assert!(mutation_documents.contains(&child_handle), "the child's edit must carry the CHILD's handle, not the parent's");

            // 🧾️ ONE `UndoGroup` names BOTH documents via `member_edits`.
            assert_eq!(result.inverse_group.member_edits.len(), 2);
            let member_documents: std::collections::HashSet<ArtifactHandle> = result.inverse_group.member_edits.iter().map(|edit_ref| edit_ref.document).collect();
            assert!(member_documents.contains(&parent_handle));
            assert!(member_documents.contains(&child_handle));

            // The child store actually applied its own op.
            let (dialect, child_member) = app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("child stays registered after dispatch");
            assert_eq!(dialect.artifact_kind, "s.test.child");
            let child_store = child_member.as_any_mut().downcast_mut::<store::ArtifactStore<TestSnapshot, TestMutation>>().expect("concrete child store type");
            assert_eq!(child_store.snapshot().expect("child snapshot").count, 7);

            // And the command log recorded the child's edit id under the `config_edit_ids` precedent.
            let history = app.test_history();
            let row = history.commands.iter().find(|entry| entry.action_id == "compositeEdit").expect("composite edit logged");
            assert_eq!(row.child_edit_ids.len(), 1);
        }

        /// 🧪️ Registers the production `TypedChildStoreFactory` for the test child kind, so the
        /// paths below go through the SAME factory a real plugin uses rather than a test-only stub.
        fn register_test_child_factory() {
            store::register_typed_child_store_factory::<TestSnapshot, TestMutation>(store::os_io::ArtifactKindId::parse("s.test.child").expect("canonical kind"), "semio.test/v1");
        }

        #[test]
        fn a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames() {
            register_test_child_factory();
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.register_child("slot", "child-1", test_child_dialect(), new_test_child("child-1")).expect("register child");
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).expect("composite edit");

            // 📤️ Persist exactly what the host would: the parent's document pack plus one
            // `ChildPackEntry` per live child.
            let entries = PluginApp::child_packs(&app).expect("child packs");
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].slot, "slot");
            assert_eq!(entries[0].child_id, "child-1");
            assert_eq!(entries[0].dialect, test_child_dialect().to_coordinate());

            // 📥️ Reload into a FRESH app, the way `LoadDocument` + `LoadChildren` would.
            let mut reloaded = VcsArtifactApp::new(TestApp::default());
            for entry in &entries {
                let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
                PluginApp::load_child_pack(&mut reloaded, &entry.slot, &entry.child_id, dialect, &entry.envelope_pack).expect("load child pack");
            }

            // The child came back as its OWN live store, at the value its own history ended on —
            // and reload went through the real factory, not a cache.
            let child = reloaded.child_store("slot", "child-1").expect("child restored");
            let restored: TestSnapshot = <TestSnapshot as store::ArtifactPack>::decode_pack(&child.document_pack_bytes().expect("child pack")).expect("decode child");
            assert_eq!(restored.count, 7, "the reloaded child lost its own edit history");
        }

        #[test]
        fn a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them() {
            register_test_child_factory();
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.register_child("slot", "child-1", test_child_dialect(), new_test_child("child-1")).expect("register child");
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).expect("first composite edit");

            // 📌️ Checkpoint the parent: the cascade must commit the dirty child first, then pin the
            // child checkpoint that commit produced.
            app.dispatch_action("commitCheckpoint", Some(&serde_json::json!({ "message": "v1" })), &meta()).expect("checkpoint");
            let pinned_checkpoint = app.test_store().current_checkpoint_id().map(str::to_string).expect("parent checkpoint exists");
            let pins = app.test_store().envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == pinned_checkpoint).map(|checkpoint| checkpoint.composition_pins.clone()).expect("checkpoint found");
            assert_eq!(pins.len(), 1, "a composing document's checkpoint must pin its children");
            assert_eq!(pins[0].child_ref.artifact_id, "child-1");

            // ⏭️ Move both forward, past the pin.
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 42 }, &meta()).expect("second composite edit");
            let live = reads_child_count(&app);
            assert_eq!(live, 42);

            // ⏮️ Checking the parent out to the pinned checkpoint must drag the child back with it —
            // otherwise a restored composition silently mixes an old parent with a new child.
            app.dispatch_action("checkoutCheckpoint", Some(&serde_json::json!({ "checkpointId": pinned_checkpoint })), &meta()).expect("checkout");
            assert_eq!(reads_child_count(&app), 7, "checkout did not cascade to the pinned child");
        }

        /// 🧪️ The child's current `count`, read through the same `ChildContentView` seam an app uses.
        fn reads_child_count(app: &VcsArtifactApp<TestApp>) -> i32 {
            let view = crate::app::ChildContentView::new(&app.children);
            view.typed::<TestSnapshot>("slot", "child-1").expect("child readable through the view").count
        }

        #[test]
        fn the_child_content_view_never_goes_stale_across_undo_and_redo() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.register_child("slot", "child-1", test_child_dialect(), new_test_child("child-1")).expect("register child");
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).expect("composite edit");
            assert_eq!(reads_child_count(&app), 7);

            // ↩️ Store-level undo bypasses `ArtifactApp::handle` entirely — this is exactly where the
            // `thread_local!` child caches this view replaces used to go stale.
            app.dispatch_action("undo", None, &meta()).expect("undo");
            assert_eq!(reads_child_count(&app), 0, "the view must reflect the child's undone state");
            app.dispatch_action("redo", None, &meta()).expect("redo");
            assert_eq!(reads_child_count(&app), 7, "the view must reflect the child's redone state");
        }

        #[test]
        fn group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.register_child("slot", "child-a", test_child_dialect(), new_test_child("child-a")).expect("register child seeds ownership");
            // `child-b` is registered but NEVER targeted by the composite gesture below — its
            // `tail_group_id()` stays `None`, the textbook "foreign tail" `GroupUndoReport` must
            // skip rather than abort the whole group over.
            app.register_child("slot", "child-b", test_child_dialect(), new_test_child("child-b")).expect("register child seeds ownership");

            let before = app.test_snapshot();
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-a".into(), child_value: 5 }, &meta()).expect("composite edit");
            assert_eq!(app.test_snapshot().label, "composite");

            let result = app.dispatch_action("undo", None, &meta()).expect("group undo");

            // The parent reverted...
            assert_eq!(app.test_snapshot(), before);
            // ...child-a (the real group member) reverted too...
            let (_, child_a) = app.children.get_mut(&("slot".to_string(), "child-a".to_string())).expect("child-a");
            let child_a_store = child_a.as_any_mut().downcast_mut::<store::ArtifactStore<TestSnapshot, TestMutation>>().expect("concrete child store type");
            assert_eq!(child_a_store.snapshot().expect("child-a snapshot").count, 0);
            // ...and child-b — a genuine foreign tail, never touched by this group — is reported as
            // SKIPPED, not silently dropped nor allowed to abort the rest of the group.
            assert!(!result.diagnostics.is_empty(), "child-b's foreign tail must surface a diagnostic, not vanish silently");
            assert!(result.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("child-b")), "the skip diagnostic must name the actual skipped member");
        }

        #[test]
        fn created_children_survive_absorb_into_the_child_store_map() {
            // 🌱️ Proves `VcsArtifactApp::absorb_created_children` — the mechanism a
            // `ChildGenesis`-authoring `Emit` constructor (a later wave) will rely on to make a
            // freshly-minted child reachable at all; per B2's own `GroupReceipt::created_children`
            // doc comment, skipping this step would make `ChildGenesis` pointless.
            let mut app = VcsArtifactApp::new(TestApp::default());
            let parent_id = app.store.envelope().id.clone();
            app.composition.graph_mut().insert_owns(&parent_id, "genesisSlot", "genesis-child").expect("seed ownership so absorb's slot_of lookup resolves");
            let target = store::os_io::ArtifactRef { artifact_id: "genesis-child".into(), dialect: test_child_dialect() };
            let created: Vec<(store::os_io::ArtifactRef, Box<dyn store::SpaceMember>)> = vec![(target, new_test_child("genesis-child"))];

            app.absorb_created_children(created);

            let (dialect, member) = app.children.get_mut(&("genesisSlot".to_string(), "genesis-child".to_string())).expect("genesis child absorbed into the live map under its real slot");
            assert_eq!(dialect.artifact_kind, "s.test.child");
            assert_eq!(member.document_id(), "genesis-child");
        }
        //#endregion 🔖️CompositionTests

        #[test]
        fn view_action_emits_no_operations() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta()).expect("select");
            assert!(result.mutations.is_empty());
            assert!(result.requested_effects.is_empty());
            // A view command never advances the document.
            assert_eq!(app.test_snapshot(), TestSnapshot::default());
        }

        #[test]
        fn view_action_with_inverse_is_revertible_and_backwards_restores_app_runtime_state() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            // Keep the two selects from folding into one row by dispatching an unrelated Mutation between them.
            app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta()).expect("select a");
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");
            app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta()).expect("select b");
            assert_eq!(app.test_config().selected, Some("b".to_string()));

            let history = app.test_history();
            // 🧾️ Revert-to-command semantics are VCS-consistent (same as the document side): "leave the
            // TARGET row applied, undo everything after it" — so to land back on `selected == "a"`, target
            // the "select a" row itself (the one with the SMALLEST seq — `history.commands` is newest-first).
            let select_a = history.commands.iter().filter(|entry| entry.action_id == "select").min_by_key(|entry| entry.seq).expect("select-a row carrying a config edit id");
            assert!(select_a.revertible, "a config edit-linked row must be revertible");
            let seq = select_a.seq;
            let log_len_before = history.commands.len();

            app.handle_action(REVERT_TO_COMMAND_ACTION_ID, Some(&json!({ "entrySeq": seq })), &meta()).expect("revertToCommand on a config-edit row");

            assert_eq!(app.test_config().selected, Some("a".to_string()), "reverting to the select-a row must leave it applied and undo select-b");
            let after = app.test_history();
            // 🧾️ Unlike the pre-B1 memory-replay (which redispatched "select" and folded a new row), a
            // config-store undo-to-position is pure cursor motion on the config store — it appends its own
            // "revertToCommand" row, exactly like the document-edit branch above it.
            assert_eq!(after.commands.len(), log_len_before + 1, "the revert appends one History-kind row");
            assert_eq!(after.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
        }

        #[test]
        fn shell_action_with_inverse_bubbles_a_replay_effect_instead_of_replaying_locally() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.handle_action(NOTE_SHELL_COMMAND_ACTION_ID, Some(&json!({ "commandId": "os.setThemeId", "label": "Set Theme", "inverseCommandId": "os.setThemeId", "inverseArgs": { "themeId": "light" } })), &meta())
                .expect("noteShellCommand with inverse");

            let history = app.test_history();
            let entry = history.commands.first().expect("one logged shell row");
            assert_eq!(entry.kind, ActionKind::Shell);
            assert!(entry.revertible, "a Shell row with a stored inverse must be revertible");
            let seq = entry.seq;

            let result = app.handle_action(REVERT_TO_COMMAND_ACTION_ID, Some(&json!({ "entrySeq": seq })), &meta()).expect("revertToCommand on a Shell row");

            // The plugin cannot touch shell-owned state itself — it bubbles the inverse out as an effect
            // instead of replaying anything locally, and does NOT append a new log entry on its own.
            assert_eq!(result.requested_effects, vec![HostEffect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
            assert_eq!(app.test_history().commands.len(), history.commands.len(), "bubbling the effect logs nothing new by itself");
        }

        #[test]
        fn shell_action_emits_host_effect_without_operations() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.dispatch_typed(TestCommand::Navigate, &meta()).expect("navigate");
            assert!(result.mutations.is_empty());
            assert_eq!(result.requested_effects, vec![HostEffect::Navigate { uri: "semio://home".into() }]);
        }

        #[test]
        fn copy_emits_clipboard_write_effect_with_no_operations() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta()).expect("setLabel");
            let result = app.handle_action("copy", None, &meta()).expect("copy");
            assert!(result.mutations.is_empty(), "copy must not record an undo entry");
            assert_eq!(result.requested_effects.len(), 1);
            let HostEffect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
            assert_eq!(fragment.dsl_text, "hello");
            assert_eq!(fragment.source_app, "synthetic-play");
        }

        #[test]
        fn copy_on_empty_selection_is_a_benign_no_operation() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.handle_action("copy", None, &meta()).expect("copy");
            assert!(result.mutations.is_empty());
            assert!(result.requested_effects.is_empty());
        }

        #[test]
        fn cut_removes_label_and_emits_clipboard_write_as_one_undo_unit() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta()).expect("setLabel");
            let result = app.handle_action("cut", None, &meta()).expect("cut");
            assert_eq!(app.test_snapshot().label, "");
            assert_eq!(result.requested_effects.len(), 1);
            assert!(matches!(&result.requested_effects[0], HostEffect::ClipboardWrite { fragment } if fragment.dsl_text == "hello"));
            // One undo restores the cut label — cut is a single coalesced edit, not two.
            app.handle_action("undo", None, &meta()).expect("undo");
            assert_eq!(app.test_snapshot().label, "hello");
        }

        #[test]
        fn paste_materializes_fragment_at_original_anchor() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let fragment =
                ClipboardFragment { schema: "semio.test/v1".into(), media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, dsl_text: "pasted".into(), pack_bytes: None, source_app: "synthetic-play".into(), label: "pasted".into() };
            let args = json!({ "fragment": fragment, "anchor": "original" });
            app.handle_action("paste", Some(&args), &meta()).expect("paste");
            assert_eq!(app.test_snapshot().label, "pasted");
        }

        #[test]
        fn paste_with_non_original_anchor_reaches_the_app_placement() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let fragment =
                ClipboardFragment { schema: "semio.test/v1".into(), media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, dsl_text: "pasted".into(), pack_bytes: None, source_app: "synthetic-play".into(), label: "pasted".into() };
            let args = json!({ "fragment": fragment, "anchor": "centroid" });
            app.handle_action("paste", Some(&args), &meta()).expect("paste");
            assert_eq!(app.test_snapshot().label, format!("pasted-{:?}", PasteAnchor::Centroid));
        }

        #[test]
        fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.handle_action("paste", None, &meta()).expect("paste");
            assert!(result.mutations.is_empty());
            assert_eq!(app.test_snapshot().label, "");
        }

        #[test]
        fn copy_cut_paste_are_registered_as_clipboard_kind_actions() {
            let definition = synthetic_play_app().definition;
            for id in ["copy", "cut", "paste"] {
                let action = definition.actions.iter().find(|a| a.id == id).unwrap_or_else(|| panic!("{id} must be auto-injected into every app's manifest"));
                assert_eq!(action.kind, semio_framework::ActionKind::Clipboard);
            }
        }

        #[test]
        fn coalesced_operations_amend_a_single_edit() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            for value in ["a", "ab", "abc"] {
                app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta()).expect("setLabel");
            }
            assert_eq!(app.test_snapshot().label, "abc");
            // One undo reverts the whole coalesced gesture back to the empty label.
            app.handle_action("undo", None, &meta()).expect("undo");
            assert_eq!(app.test_snapshot().label, "");
        }

        #[test]
        fn history_actions_round_trip_through_the_store() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("inc1");
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("inc2");
            assert_eq!(app.test_snapshot().count, 2);

            let undo = app.handle_action("undo", None, &meta()).expect("undo");
            assert!(undo.mutations.is_empty());
            assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
            assert_eq!(app.test_snapshot().count, 1);

            app.handle_action("redo", None, &meta()).expect("redo");
            assert_eq!(app.test_snapshot().count, 2);

            let checkpoint = app.handle_action("commitCheckpoint", None, &meta()).expect("checkpoint");
            assert!(checkpoint.mutations.is_empty());
            assert!(checkpoint.events.iter().any(|event| event.kind == "history-changed"));
        }

        //#region 🔖️CommandLogTests
        #[test]
        fn an_operation_action_appends_one_command_log_entry_linked_to_its_edit() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");
            let history = app.test_history();
            assert_eq!(history.commands.len(), 1);
            let entry = &history.commands[0];
            assert_eq!(entry.action_id, "increment");
            assert_eq!(entry.label.as_str(), "increment");
            assert_eq!(entry.kind, ActionKind::Mutation);
            assert!(entry.edit_id.is_some());
            assert!(!entry.op_lines.is_empty(), "operation entry must carry printed op-text");
            assert!(entry.applied && entry.revertible);
        }

        #[test]
        fn a_coalesced_gesture_appends_exactly_one_command_log_entry() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            for value in ["a", "ab", "abc"] {
                app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta()).expect("setLabel");
            }
            let history = app.test_history();
            let set_label_entries: Vec<&CommandView> = history.commands.iter().filter(|entry| entry.action_id == "setLabel").collect();
            assert_eq!(set_label_entries.len(), 1, "a coalesced gesture must grow one entry's op_lines, not append new entries");
        }

        #[test]
        fn undo_and_redo_append_entries_and_never_shrink_the_log() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");
            assert_eq!(app.test_history().commands.len(), 1);
            app.handle_action("undo", None, &meta()).expect("undo");
            let after_undo = app.test_history();
            assert_eq!(after_undo.commands.len(), 2, "undo appends, it does not remove the increment entry");
            assert!(after_undo.commands.iter().any(|entry| entry.action_id == "increment"));
            app.handle_action("redo", None, &meta()).expect("redo");
            let after_redo = app.test_history();
            assert_eq!(after_redo.commands.len(), 3, "redo appends a third entry");
            assert!(after_redo.commands.iter().any(|entry| entry.action_id == "undo"));
        }

        #[test]
        fn revert_to_command_restores_the_snapshot_and_appends_one_entry() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("inc1");
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("inc2");
            assert_eq!(app.test_snapshot().count, 2);
            // 🧾️ `commands` is newest-first — take the MINIMUM seq among "increment" entries to target inc1, not inc2.
            let first_increment_seq = app.test_history().commands.iter().filter(|entry| entry.action_id == "increment").map(|entry| entry.seq).min().expect("first increment entry");
            let before_len = app.test_history().commands.len();

            let result = app.handle_action(REVERT_TO_COMMAND_ACTION_ID, Some(&json!({ "entrySeq": first_increment_seq })), &meta()).expect("revertToCommand");
            assert!(result.events.iter().any(|event| event.kind == "history-changed"));
            assert_eq!(app.test_snapshot().count, 1, "revert leaves the target edit applied, undoing only what came after it");
            let history = app.test_history();
            assert_eq!(history.commands.len(), before_len + 1, "exactly one entry appended for the revert itself");
            // 🧾️ `commands` is newest-first — the just-appended revert entry is the FIRST element, not the last.
            assert_eq!(history.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
        }

        #[test]
        fn ingested_remote_edits_are_backfilled_into_the_command_log() {
            let mut sender = VcsArtifactApp::new(TestApp::default());
            let (near, mut far) = MemoryBackbone::pair("mem://doc-history-backfill", "mem://doc-history-backfill");
            sender.attach_backbone(Box::new(near)).expect("attach");
            sender.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");

            let mut envelopes = Vec::new();
            for message in far.receive().expect("receive") {
                if let BackboneMessage::Mutations { envelopes: operations } = message {
                    envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
                }
            }
            let operations = protocol::encode_envelopes(&envelopes);

            // 🧾️ The receiver never dispatched anything itself — any log entry it has must come from backfill.
            let mut receiver = VcsArtifactApp::new(TestApp::default());
            receiver.ingest_operations(&operations).expect("ingest");
            let history = receiver.test_history();
            assert_eq!(history.commands.len(), 1);
            assert!(history.commands[0].edit_id.is_some());
            assert!(!history.commands[0].op_lines.is_empty());
        }

        #[test]
        fn set_history_command_filter_emits_no_operations_and_updates_the_view() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&json!({ "value": "onlyMutations" })), &meta()).expect("setHistoryCommandFilter");
            assert!(result.mutations.is_empty());
            assert_eq!(app.test_history().command_filter, HistoryCommandFilter::OnlyMutations);
        }

        #[test]
        fn ui_history_panel_filters_rows_and_gates_the_backwards_action() {
            let history = HistoryView {
                columns: Vec::new(),
                can_undo: true,
                can_redo: false,
                active_alternative_id: None,
                current_checkpoint_id: None,
                commands: vec![
                    CommandView {
                        seq: 1,
                        action_id: "increment".into(),
                        label: "Increment".into(),
                        kind: ActionKind::Mutation,
                        timestamp: "0".into(),
                        edit_id: Some("e1".into()),
                        config_edit_id: None,
                        child_edit_ids: Vec::new(),
                        op_lines: vec!["set-count value=1".into()],
                        applied: true,
                        revertible: true,
                        count: 1,
                        inverse: None,
                    },
                    CommandView {
                        seq: 2,
                        action_id: "undo".into(),
                        label: "Undo".into(),
                        kind: ActionKind::History,
                        timestamp: "1".into(),
                        edit_id: None,
                        config_edit_id: None,
                        child_edit_ids: Vec::new(),
                        op_lines: Vec::new(),
                        applied: false,
                        revertible: false,
                        count: 1,
                        inverse: None,
                    },
                ],
                command_filter: HistoryCommandFilter::All,
            };
            let UiNode::Tree(all_tree) = ui_history_panel(&history, "ctrl", false) else { panic!("expected a Tree root like Document/Catalogue") };
            assert_eq!(all_tree.sections.len(), 2, "Actions + Commands sections");
            assert_eq!(all_tree.sections[0].label.as_ref().map(|l| l.as_str()), Some("Actions"));
            assert_eq!(all_tree.sections[0].items.len(), 5, "undo/redo/commit/alternative/filter");
            assert!(all_tree.sections[0].items.iter().all(|item| item.control.is_some()), "Actions rows use label+control like Settings/Theme");
            assert_eq!(all_tree.sections[1].label.as_ref().map(|l| l.as_str()), Some("Commands"));
            assert_eq!(all_tree.sections[1].items.len(), 2);
            assert!(all_tree.sections[1].items[0].actions.is_some(), "the revertible entry must offer inverse");
            assert!(all_tree.sections[1].items[1].actions.is_none(), "the non-revertible entry must not offer inverse");

            let only_ops = HistoryView { command_filter: HistoryCommandFilter::OnlyMutations, ..history.clone() };
            let UiNode::Tree(ops_tree) = ui_history_panel(&only_ops, "ctrl", false) else { panic!("expected a Tree root") };
            assert_eq!(ops_tree.sections[1].items.len(), 1);
            assert_eq!(ops_tree.sections[1].items[0].id, "framework.history.entry.1");

            let without_ops = HistoryView { command_filter: HistoryCommandFilter::WithoutMutations, ..history };
            let UiNode::Tree(no_ops_tree) = ui_history_panel(&without_ops, "ctrl", false) else { panic!("expected a Tree root") };
            assert_eq!(no_ops_tree.sections[1].items.len(), 1);
            assert_eq!(no_ops_tree.sections[1].items[0].id, "framework.history.entry.2");
        }

        #[test]
        fn an_op_less_view_action_is_logged_with_edit_id_none_and_count_one() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta()).expect("select");
            let history = app.test_history();
            assert_eq!(history.commands.len(), 1);
            let entry = &history.commands[0];
            assert_eq!(entry.action_id, "select");
            assert_eq!(entry.kind, ActionKind::View);
            assert!(entry.edit_id.is_none());
            assert!(entry.config_edit_id.is_some(), "select is a config-op emission");
            assert_eq!(entry.count, 1);
        }

        #[test]
        fn consecutive_identical_view_dispatches_fold_into_one_entry_with_a_growing_count() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            for id in ["node-1", "node-2", "node-3"] {
                app.dispatch_typed(TestCommand::Select { id: Some(id.into()) }, &meta()).expect("select");
            }
            let history = app.test_history();
            assert_eq!(history.commands.len(), 1, "folding must not grow the log");
            assert_eq!(history.commands[0].count, 3);
        }

        #[test]
        fn folding_breaks_across_an_interleaved_different_entry() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta()).expect("select a");
            app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta()).expect("select b");
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");
            app.dispatch_typed(TestCommand::Select { id: Some("c".into()) }, &meta()).expect("select c");
            let history = app.test_history();
            assert_eq!(history.commands.len(), 3, "folded select x2, one increment, one fresh select — the interleaved operation breaks the fold");
            let select_counts: Vec<u32> = history.commands.iter().filter(|entry| entry.action_id == "select").map(|entry| entry.count).collect();
            assert_eq!(select_counts.len(), 2, "two distinct select entries, not one");
            assert!(select_counts.contains(&2), "the first two selects folded together");
            assert!(select_counts.contains(&1), "the select after the interleaved increment starts a fresh entry");
        }

        #[test]
        fn note_shell_command_is_intercepted_before_the_app_and_folds_on_repeat() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let args = json!({ "commandId": "os.setThemeId", "label": "Set Theme", "detail": "dark" });
            app.handle_action(NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &meta()).expect("noteShellCommand");
            assert!(app.test_app().received_actions.borrow().is_empty(), "interception must happen before the app ever sees noteShellCommand");
            let history = app.test_history();
            assert_eq!(history.commands.len(), 1);
            let entry = &history.commands[0];
            assert_eq!(entry.action_id, "os.setThemeId");
            assert_eq!(entry.kind, ActionKind::Shell);
            assert!(entry.label.contains("dark"));

            app.handle_action(NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &meta()).expect("noteShellCommand again");
            assert!(app.test_app().received_actions.borrow().is_empty());
            let history = app.test_history();
            assert_eq!(history.commands.len(), 1, "the same commandId folds instead of appending");
            assert_eq!(history.commands[0].count, 2);
        }

        #[test]
        fn scope_upgrade_none_becomes_partial_naming_the_history_body() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.dispatch_typed(TestCommand::ViewNoScope, &meta()).expect("viewNoScope");
            let UiDirtyScope::Partial { panel_bodies, .. } = result.ui_scope else { panic!("expected an upgraded Partial scope") };
            assert!(panel_bodies.iter().any(|key| key == FRAMEWORK_HISTORY_BODY_KEY));
        }

        #[test]
        fn scope_upgrade_partial_keeps_window_bodies_and_gains_the_history_body() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.dispatch_typed(TestCommand::ViewPartialScope, &meta()).expect("viewPartialScope");
            let UiDirtyScope::Partial { window_bodies, panel_bodies, .. } = result.ui_scope else { panic!("expected a Partial scope") };
            assert_eq!(window_bodies, vec!["some.window".to_string()], "the app's own window_bodies must survive the upgrade");
            assert!(panel_bodies.iter().any(|key| key == FRAMEWORK_HISTORY_BODY_KEY));
        }

        #[test]
        fn scope_upgrade_full_stays_full() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.dispatch_typed(TestCommand::Select { id: Some("x".into()) }, &meta()).expect("select");
            assert_eq!(result.ui_scope, UiDirtyScope::Full);
        }

        #[test]
        fn benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let before_len = app.test_history().commands.len();
            let result = app.handle_action("undo", None, &meta()).expect("undo");
            assert_eq!(result.ui_scope, UiDirtyScope::None, "nothing was logged, so the scope must not be upgraded either");
            assert_eq!(app.test_history().commands.len(), before_len);
        }

        #[test]
        fn set_history_command_filter_is_never_logged() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let before_len = app.test_history().commands.len();
            app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&json!({ "value": "onlyMutations" })), &meta()).expect("setHistoryCommandFilter");
            assert_eq!(app.test_history().commands.len(), before_len, "the filter's own chrome must not fill the list it filters");
        }

        #[test]
        fn rendering_the_history_body_reflects_a_log_only_change_with_no_store_generation_bump() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::default()).expect("render before");
            app.dispatch_typed(TestCommand::Select { id: Some("x".into()) }, &meta()).expect("select");
            let rendered = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::default()).expect("render after");
            let UiNode::Tree(tree) = rendered else { panic!("expected a Tree root like Document/Catalogue") };
            assert_eq!(tree.sections.len(), 2, "Actions + Commands");
            assert_eq!(tree.sections[1].items.len(), 1, "a log-only cache key change (no store generation bump) must still refresh the rendered panel");
        }

        #[test]
        fn an_operation_kind_action_with_zero_operations_still_logs_one_entry() {
            let mut app = contract_app_under_test();
            app.dispatch_typed(TestCommand::NoopMutation, &meta()).expect("noopMutation");
            let history = app.test_history();
            assert_eq!(history.commands.len(), 1);
            let entry = &history.commands[0];
            assert_eq!(entry.action_id, "noopMutation");
            assert_eq!(entry.kind, ActionKind::Mutation);
            assert!(entry.edit_id.is_none(), "no operations means no VCS edit, even though the action is Mutation-kind");
            assert_eq!(entry.count, 1);
        }
        //#endregion 🔖️CommandLogTests

        #[test]
        fn undo_on_empty_history_is_a_benign_no_operation() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            let result = app.handle_action("undo", None, &meta()).expect("undo");
            assert!(result.mutations.is_empty());
            assert!(result.events.is_empty());
        }

        #[test]
        fn document_round_trips_through_serialization() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("inc");
            app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta()).expect("label");
            let files = app.document_pack().expect("document pack");

            let mut restored = VcsArtifactApp::new(TestApp::default());
            restored.load_document_pack(&files).expect("load document pack");
            assert_eq!(restored.test_snapshot(), TestSnapshot { count: 1, label: "hi".into() });
        }

        #[test]
        fn ingest_operations_is_idempotent() {
            let mut sender = VcsArtifactApp::new(TestApp::default());
            let (near, mut far) = MemoryBackbone::pair("mem://doc", "mem://doc");
            sender.attach_backbone(Box::new(near)).expect("attach");
            sender.dispatch_typed(TestCommand::Increment, &meta()).expect("increment");

            let mut envelopes = Vec::new();
            for message in far.receive().expect("receive") {
                if let BackboneMessage::Mutations { envelopes: operations } = message {
                    envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
                }
            }
            assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
            let operations = protocol::encode_envelopes(&envelopes);

            let mut receiver = VcsArtifactApp::new(TestApp::default());
            receiver.ingest_operations(&operations).expect("ingest once");
            receiver.ingest_operations(&operations).expect("ingest twice");
            assert_eq!(receiver.test_snapshot().count, 1, "feeding the same operation twice must not double-apply");
        }

        #[test]
        fn attach_detach_reattach_resumes_backbone_convergence() {
            let mut app = VcsArtifactApp::new(TestApp::default());
            assert!(app.backbone_ref().is_none(), "default is unattached");

            let (near, mut far) = MemoryBackbone::pair("mem://reattach", "mem://reattach");
            app.attach_backbone(Box::new(near)).expect("attach");
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment while attached");
            assert!(!far.receive().expect("receive after attach").is_empty(), "attached edits reach the peer");

            app.detach_backbone();
            assert!(app.backbone_ref().is_none());
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment while detached");
            assert_eq!(app.test_snapshot().count, 2, "detached edits still land on the in-memory graph");
            assert!(far.receive().expect("receive while detached").is_empty(), "detached edits never reach the peer");

            let (near_again, mut far_again) = MemoryBackbone::pair("mem://reattach-2", "mem://reattach-2");
            app.attach_backbone(Box::new(near_again)).expect("re-attach");
            assert!(app.backbone_ref().is_some());
            app.dispatch_typed(TestCommand::Increment, &meta()).expect("increment after re-attach");
            assert_eq!(app.test_snapshot().count, 3);
            assert!(!far_again.receive().expect("receive after re-attach").is_empty(), "re-attaching resumes outbound convergence on the new backbone");
        }

        #[test]
        fn selection_count_phrase_formats_mixed_selection() {
            assert_eq!(selection_count_phrase(false, &[(8, "node", "nodes"), (13, "edge", "edges")]), "8 nodes and 13 edges");
            assert_eq!(selection_count_phrase(false, &[(1, "node", "nodes")]), "1 node");
            assert_eq!(selection_count_phrase(true, &[(8, "Knoten", "Knoten"), (13, "Kante", "Kanten")]), "8 Knoten und 13 Kanten");
        }

        /// 🖱️ `PluginApp::context_menu` end-to-end through `VcsArtifactApp`: with an empty label the
        /// "selection guard" (`Menu::when`) drops the gated command; once a label is set (a stand-in for
        /// "something is selected"), both rows resolve label/icon from the declared registry entries.
        #[test]
        fn context_menu_resolves_labels_from_the_registry_and_respects_guards() {
            let mut app = contract_app_under_test();
            let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

            use semio_framework::{catalog_action_icon_id, catalog_command_icon_id};

            let set_label_icon = catalog_action_icon_id("setLabelRequired", ActionKind::Mutation).as_str().to_string();
            let increment_icon = catalog_command_icon_id("incrementViaCommand").as_str().to_string();

            let empty_label = app.context_menu(&request);
            assert_eq!(empty_label.len(), 1, "the gated command must be absent with no label set: {empty_label:?}");
            assert_eq!(empty_label[0], ContextMenuItemSpec { id: "setLabelRequired".into(), label: Some("Set Label".into()), icon: Some(set_label_icon), action: Some("setLabelRequired".into()), ..Default::default() });

            app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta()).expect("set label");
            let with_label = app.context_menu(&request);
            assert_eq!(with_label.len(), 2, "the guard must open once a label is set: {with_label:?}");
            assert_eq!(with_label[1], ContextMenuItemSpec { id: "incrementViaCommand".into(), label: Some("Increment".into()), icon: Some(increment_icon), action: Some("incrementViaCommand".into()), ..Default::default() });
        }

        //#region 🗂️GroupedContextMenu
        #[test]
        fn action_definition_with_category_sets_the_ribbon_taxonomy_field() {
            let action = ActionDefinition::new_catalog("x", LocalizedLabel::data("X"), ActionKind::Mutation).with_category("view");
            assert_eq!(action.category.as_deref(), Some("view"));
        }

        #[test]
        fn menu_group_produces_a_group_row_keyed_by_category() {
            let registry = contract_registry();
            let items = Menu::of(&registry).action("setLabelRequired").group("export", |m| m.command("incrementViaCommand")).build();
            assert_eq!(items.len(), 2);
            assert_eq!(items[1].id, "menu.group.export");
            assert_eq!(items[1].label, None, "group rows travel with no label — the host resolves it via `ribbon_parent_label`");
            let children: Vec<&str> = items[1].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
            assert_eq!(children, vec!["incrementViaCommand"]);
        }

        /// 🧪️ A registry declaring `setLabelRequired` plus ten `flatLeaf1..10` actions (four carrying a
        /// `RIBBON_PARENT_CATEGORIES` category via `with_category`) — feeds `TestApp::context_menu`'s
        /// `"flat-menu-test"` branch below.
        fn flat_menu_registry() -> AppActionRegistry {
            let app = App::from_builder(
                App::builder("flat-menu-test", LocalizedLabel::data("FlatMenuTest"))
                    .document(["state"])
                    .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                    .window_kind("main", LocalizedLabel::data("Main"), "flat-menu-test.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                    .mutation("setLabelRequired", LocalizedLabel::data("Set Label"))
                    .action_args("setLabelRequired", vec![ActionArgDef::text("value", LocalizedLabel::data("Value")).required()])
                    .mutation("flatLeaf1", LocalizedLabel::data("Flat Leaf 1"))
                    .mutation("flatLeaf2", LocalizedLabel::data("Flat Leaf 2"))
                    .mutation("flatLeaf3", LocalizedLabel::data("Flat Leaf 3"))
                    .mutation("flatLeaf4", LocalizedLabel::data("Flat Leaf 4"))
                    .action_with(ActionDefinition::new_catalog("flatLeaf5", LocalizedLabel::data("Flat Leaf 5"), ActionKind::Mutation).with_category("view"))
                    .action_with(ActionDefinition::new_catalog("flatLeaf6", LocalizedLabel::data("Flat Leaf 6"), ActionKind::Mutation).with_category("view"))
                    .action_with(ActionDefinition::new_catalog("flatLeaf7", LocalizedLabel::data("Flat Leaf 7"), ActionKind::Mutation).with_category("export"))
                    .action_with(ActionDefinition::new_catalog("flatLeaf8", LocalizedLabel::data("Flat Leaf 8"), ActionKind::Mutation).with_category("export"))
                    .mutation("flatLeaf9", LocalizedLabel::data("Flat Leaf 9"))
                    .mutation("flatLeaf10", LocalizedLabel::data("Flat Leaf 10")),
            );
            AppActionRegistry::from_definition(&app.definition)
        }

        /// 🖱️ End to end through `VcsArtifactApp::context_menu`: a synthetic emitter's 11 flat leaves come
        /// back as 5 primaries + 3 taxonomy-sorted `menu.group.<category>` rows — proving the funnel applies
        /// `organize_context_menu` to every emitter, not just ones that call `Menu::group` themselves.
        #[test]
        fn context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu() {
            let mut app = VcsArtifactApp::with_registry(TestApp::default(), flat_menu_registry());
            app.dispatch_typed(TestCommand::SetLabel { value: "flat-menu-test".into() }, &meta()).expect("set label");
            let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

            let organized = app.context_menu(&request);
            let ids: Vec<&str> = organized.iter().map(|item| item.id.as_str()).collect();
            assert_eq!(
                ids,
                vec!["setLabelRequired", "flatLeaf1", "flatLeaf2", "flatLeaf3", "flatLeaf4", "menu.group.view", "menu.group.actions", "menu.group.export"],
                "5 primaries, then groups in RIBBON_PARENT_CATEGORIES taxonomy order (view < actions < export): {ids:?}"
            );
            let view_children: Vec<&str> = organized[5].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
            assert_eq!(view_children, vec!["flatLeaf5", "flatLeaf6"]);
            let actions_children: Vec<&str> = organized[6].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
            assert_eq!(actions_children, vec!["flatLeaf9", "flatLeaf10"], "uncategorized overflow leaves default to menu.group.actions");
        }

        #[test]
        fn context_menu_wire_request_without_view_state_still_parses() {
            let wire: ContextMenuWireRequest = serde_json::from_str(r#"{"menu":{"id":"window"}}"#).expect("viewState is no longer a required field");
            assert_eq!(wire.menu.id, "window");
            assert!(wire.surface.is_none());
            assert!(wire.window_instance_id.is_none());
            assert!(wire.point.is_none());
        }
        //#endregion 🗂️GroupedContextMenu

        #[test]
        fn view_action_emitting_ops_is_rejected() {
            let mut app = contract_app_under_test();
            let error = app.dispatch_typed(TestCommand::BadView, &meta()).expect_err("a View command emitting operations must be rejected");
            assert!(error.message.contains("must not emit operations"), "unexpected error: {}", error.message);
            assert_eq!(app.test_snapshot(), TestSnapshot::default());
        }

        #[test]
        fn set_active_utility_carries_its_value_directly_and_emits_no_operations() {
            let mut app = contract_app_under_test();
            let result = app.dispatch_typed(TestCommand::SetActiveUtility { utility_id: "brush".into() }, &meta()).expect("setActiveUtility is a valid View command");
            assert!(result.mutations.is_empty(), "utility switching must not create history");
            let event = result.events.iter().find(|event| event.kind == "active-utility").expect("echoed active utility");
            assert_eq!(event.payload, dsl::to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
        }

        #[test]
        fn action_emit_amend_coalesces_while_commit_does_not() {
            let mut app = contract_app_under_test();
            for value in ["a", "ab", "abc"] {
                app.dispatch_typed(TestCommand::AmendLabel { value: value.into() }, &meta()).expect("amendLabel");
            }
            assert_eq!(app.test_snapshot().label, "abc");
            // One undo reverts the whole coalesced amend gesture.
            app.handle_action("undo", None, &meta()).expect("undo amend");
            assert_eq!(app.test_snapshot().label, "");

            for value in ["x", "xy"] {
                app.dispatch_typed(TestCommand::CommitLabel { value: value.into() }, &meta()).expect("commitLabel");
            }
            assert_eq!(app.test_snapshot().label, "xy");
            // Each commit is its own edit: one undo only reverts the last commit.
            app.handle_action("undo", None, &meta()).expect("undo commit");
            assert_eq!(app.test_snapshot().label, "x");
        }

        #[test]
        fn amend_dispatch_reports_only_this_dispatch_new_operations() {
            // 🪢️ Regression guard for `result_from_last_edit`'s `tail_offset` slicing: even though the
            // coalesced edit accumulates every amend's operations (3 after this loop), each dispatch's
            // `InvocationResult` must report only the operation IT just added — never re-serializing the whole
            // growing edit into every `KernelMutation`/`UndoGroup` on every single dispatch.
            let mut app = contract_app_under_test();
            app.dispatch_typed(TestCommand::AmendLabel { value: "a".into() }, &meta()).expect("amendLabel a");
            app.dispatch_typed(TestCommand::AmendLabel { value: "ab".into() }, &meta()).expect("amendLabel ab");
            let result = app.dispatch_typed(TestCommand::AmendLabel { value: "abc".into() }, &meta()).expect("amendLabel abc");
            assert_eq!(result.mutations.len(), 1, "must report only this dispatch's new operation, not the whole coalesced edit");
            assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetLabel { value: "abc".into() }).unwrap());
            assert_eq!(
                result.mutations[0].inverse.inverse_diff.payload,
                protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetLabel { value: "ab".into() }).unwrap()]),
                "the new operation's own inverse undoes back to the pre-dispatch label, not the whole gesture"
            );
            assert_eq!(result.inverse_group.mutations.len(), 1);
            assert_eq!(result.inverse_group.inverse_mutations.len(), 1);
            assert_eq!(app.test_snapshot().label, "abc");
            // The narrowed per-dispatch reporting must not affect coalescing/undo semantics.
            app.handle_action("undo", None, &meta()).expect("undo amend");
            assert_eq!(app.test_snapshot().label, "");
        }

        #[test]
        fn operation_command_emits_kernel_op_with_true_inverse() {
            let mut app = contract_app_under_test();
            let result = app.dispatch_typed(TestCommand::IncrementViaCommand, &meta()).expect("incrementViaCommand");
            assert_eq!(result.mutations.len(), 1);
            assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetCount { value: 1 }).unwrap());
            assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount { value: 0 }).unwrap()]));
            assert_eq!(app.test_snapshot().count, 1);
        }

        #[test]
        fn unknown_string_command_is_a_hard_error() {
            // B1: `PluginApp::handle_command` (string-keyed) is FRAMEWORK-reserved only now — there are no
            // framework-reserved commands, so it always errors, pointing callers at the typed channel.
            let mut app = contract_app_under_test();
            let error = app.handle_command("nope", None, &meta()).expect_err("the string command channel always rejects now");
            assert!(error.message.contains("typed command channel"), "unexpected error: {}", error.message);
        }

        #[test]
        fn command_op_records_history_exactly_like_an_operation_action() {
            let mut app = contract_app_under_test();
            app.dispatch_typed(TestCommand::IncrementViaCommand, &meta()).expect("inc");
            app.dispatch_typed(TestCommand::IncrementViaCommand, &meta()).expect("inc");
            assert_eq!(app.test_snapshot().count, 2);
            app.handle_action("undo", None, &meta()).expect("undo");
            assert_eq!(app.test_snapshot().count, 1);
        }

        #[test]
        fn registry_less_construction_skips_enforcement() {
            // The empty-registry path (VcsArtifactApp::new) passes commands through unchecked.
            let mut app = VcsArtifactApp::new(TestApp::default());
            app.dispatch_typed(TestCommand::BadView, &meta()).expect("no registry ⇒ kind discipline skipped");
        }
    }
    // #endregion plugin_runtime
}

pub mod world3d_host {
    // #region world3d_host
    //! 🌐️ Shared world-3d scene payload builders for plugin apps.

    use semio_framework::mesh_from_kind;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::f64::consts::PI;
    use ui_wgpu::wgpu::{world3d_camera_json, world3d_default_selection_json, ActionDescriptor, MeasureSelectItem, WindowMeasure, World3dScene};

    //#region 🌞️ WorldSunConfig
    /** 🌞️ Plugin-owned directional-light state for a `world-3d` scene; off by default so meshes render flat until a dev opts in via the window-options Sun toggle. */
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", default)]
    pub struct WorldSunConfig {
        pub enabled: bool,
        pub azimuth: f64,
        pub elevation: f64,
        pub intensity: f64,
        pub color: String,
    }

    impl Default for WorldSunConfig {
        fn default() -> Self {
            Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
        }
    }

    /** 🌞️ Builds the `environment_json` payload consumed by `world-3d-host.tsx`'s `WorldEnvironmentRecord.sun`. */
    pub fn world3d_environment_json(sun: &WorldSunConfig) -> String {
        json!({ "sun": sun }).to_string()
    }

    /** 🌳️ Measure group with optional open state and no header slider fields. */
    fn measure_group_with_open(id: String, label: impl Into<String>, default_open: Option<bool>, children: Vec<WindowMeasure>) -> WindowMeasure {
        WindowMeasure::Group { id, label: label.into(), default_open, active_utility_id: None, value: None, min: None, max: None, step: None, ready: None, loading: None, waiting: None, on_change: None, children }
    }

    /** 🌞️ Shared "Sun" window-options group (enable toggle + azimuth/elevation/intensity sliders), see `lowpoly_window_measures`'s "Show Edges" toggle for the sibling pattern. */
    pub fn world3d_sun_measures(id_prefix: &str, sun: &WorldSunConfig, action: impl Fn(&str, Option<Value>) -> ActionDescriptor) -> WindowMeasure {
        measure_group_with_open(
            format!("{id_prefix}-measure-sun"),
            "Sun",
            Some(false),
            vec![
                WindowMeasure::Toggle { id: format!("{id_prefix}-measure-sun-enabled"), icon_id: "sun".into(), label: Some("Enabled".into()), pressed: sun.enabled, text: None, on_change: action("toggleSun", None) },
                WindowMeasure::Slider {
                    id: format!("{id_prefix}-measure-sun-azimuth"),
                    label: Some("Azimuth".into()),
                    value: sun.azimuth,
                    min: 0.0,
                    max: 360.0,
                    step: Some(1.0),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: action("setSunAzimuth", None),
                },
                WindowMeasure::Slider {
                    id: format!("{id_prefix}-measure-sun-elevation"),
                    label: Some("Elevation".into()),
                    value: sun.elevation,
                    min: 0.0,
                    max: 90.0,
                    step: Some(1.0),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: action("setSunElevation", None),
                },
                WindowMeasure::Slider {
                    id: format!("{id_prefix}-measure-sun-intensity"),
                    label: Some("Intensity".into()),
                    value: sun.intensity,
                    min: 0.0,
                    max: 4.0,
                    step: Some(0.05),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: action("setSunIntensity", None),
                },
            ],
        )
    }

    /** 🌞️ Applies a sun-related action id to `sun`, returning whether it was handled — mirrors `lowpoly`'s `"toggleShowEdges"` action-handler shape. */
    pub fn apply_world3d_sun_action(sun: &mut WorldSunConfig, action_id: &str, args: Option<&Value>) -> bool {
        match action_id {
            "toggleSun" => {
                sun.enabled = !sun.enabled;
                true
            }
            "setSunAzimuth" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
                    sun.azimuth = value;
                }
                true
            }
            "setSunElevation" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
                    sun.elevation = value;
                }
                true
            }
            "setSunIntensity" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
                    sun.intensity = value;
                }
                true
            }
            _ => false,
        }
    }
    //#endregion 🌞️ WorldSunConfig

    //#region 📐️ WorldProjection
    /** 📐️ Plugin-owned projection state for a `world-3d` scene camera — the full classical taxonomy
     * (Parallel: Orthographic/Axonometric/Oblique, Perspective: 1/2/3-Point/Curvilinear). Flat so
     * switching `kind` and back restores whatever a dev last dialed in on the other kinds. See
     * https://en.wikipedia.org/wiki/Axonometric_projection and https://en.wikipedia.org/wiki/Oblique_projection. */
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", default)]
    pub struct WorldProjectionConfig {
        pub kind: String,
        pub orthographic_view: String,
        pub axonometric_variant: String,
        pub axonometric_angle_a: f64,
        pub axonometric_angle_b: f64,
        pub axonometric_quadrant: String,
        pub oblique_variant: String,
        pub oblique_angle: f64,
        pub oblique_depth: f64,
        pub one_point_axis: String,
        pub fov: f64,
        pub two_point_shift: f64,
        pub curvilinear_fov: f64,
        pub curvilinear_strength: f64,
        pub curvilinear_mapping: String,
    }

    impl Default for WorldProjectionConfig {
        fn default() -> Self {
            Self {
                kind: "threePoint".into(),
                orthographic_view: "top".into(),
                axonometric_variant: "isometric".into(),
                axonometric_angle_a: 15.0,
                axonometric_angle_b: 12.0,
                axonometric_quadrant: "ne".into(),
                oblique_variant: "cavalier".into(),
                oblique_angle: 45.0,
                oblique_depth: 1.0,
                one_point_axis: "y".into(),
                fov: 50.0,
                two_point_shift: 0.0,
                curvilinear_fov: 120.0,
                curvilinear_strength: 1.0,
                curvilinear_mapping: "fisheye".into(),
            }
        }
    }

    /** 📐️ Serializes mode ⊗ orientation — the `WorldProjectionSpec` shape the JS world layer parses. */
    pub fn world3d_projection_spec_json(p: &WorldProjectionConfig) -> Value {
        let mode = match p.kind.as_str() {
            "orthographic" => json!({ "kind": "orthographic" }),
            "axonometric" => {
                let (angle_a, angle_b) = match p.axonometric_variant.as_str() {
                    "isometric" => (30.0, 30.0),
                    "dimetric" => (p.axonometric_angle_a, p.axonometric_angle_a),
                    _ => (p.axonometric_angle_a, p.axonometric_angle_b),
                };
                json!({ "kind": "axonometric", "variant": p.axonometric_variant, "angleA": angle_a, "angleB": angle_b })
            }
            "oblique" => json!({ "kind": "oblique", "variant": p.oblique_variant, "angle": p.oblique_angle, "depthScale": p.oblique_depth }),
            "onePoint" => json!({ "kind": "onePoint", "fov": p.fov }),
            "twoPoint" => json!({ "kind": "twoPoint", "fov": p.fov, "verticalShift": p.two_point_shift }),
            "curvilinear" => json!({ "kind": "curvilinear", "fov": p.curvilinear_fov, "strength": p.curvilinear_strength, "mapping": p.curvilinear_mapping }),
            _ => json!({ "kind": "threePoint", "fov": p.fov }),
        };
        let orientation = match p.kind.as_str() {
            "axonometric" => json!({ "type": "corner", "quadrant": p.axonometric_quadrant, "hemisphere": "upper" }),
            "twoPoint" | "threePoint" | "curvilinear" => json!({ "type": "free" }),
            "onePoint" => {
                let view = match p.one_point_axis.as_str() {
                    "x" => "left",
                    "z" => "top",
                    _ => "front",
                };
                json!({ "type": "cardinal", "view": view })
            }
            "oblique" if p.oblique_variant == "military" => json!({ "type": "cardinal", "view": "plan" }),
            "oblique" => json!({ "type": "cardinal", "view": "front" }),
            _ => json!({ "type": "cardinal", "view": p.orthographic_view }),
        };
        json!({ "mode": mode, "orientation": orientation })
    }

    /** 📐️ `camera_json` with `position`/`target`/`up`/`zoom` plus the active-kind `projection` spec object — replaces the plain `world3d_camera_json` for worlds that carry the full taxonomy. */
    pub fn world3d_camera_projection_json(position: [f64; 3], target: [f64; 3], up: Option<[f64; 3]>, zoom: f64, p: &WorldProjectionConfig) -> String {
        let mut value = json!({
            "position": position,
            "target": target,
            "zoom": zoom,
            "projection": world3d_projection_spec_json(p),
        });
        if let Some(object) = value.as_object_mut() {
            if let Some(up) = up {
                object.insert("up".into(), json!(up));
            }
        }
        value.to_string()
    }

    /** 📐️ Canonical camera pose (`position`, `up`) for a projection config, orbiting `target` at `distance` — mirrors
     * `computeWorldProjectionPose` in `infinite/world/r3f/index.tsx`; used to snap the viewport on kind/view changes. */
    pub fn world3d_projection_pose(p: &WorldProjectionConfig, target: [f64; 3], distance: f64) -> ([f64; 3], [f64; 3]) {
        let [tx, ty, tz] = target;
        match p.kind.as_str() {
            "orthographic" => match p.orthographic_view.as_str() {
                "bottom" => ([tx, ty, tz - distance], [0.0, 1.0, 0.0]),
                "front" => ([tx, ty - distance, tz], [0.0, 0.0, 1.0]),
                "back" => ([tx, ty + distance, tz], [0.0, 0.0, 1.0]),
                "left" => ([tx - distance, ty, tz], [0.0, 0.0, 1.0]),
                "right" => ([tx + distance, ty, tz], [0.0, 0.0, 1.0]),
                _ => ([tx, ty, tz + distance], [0.0, 1.0, 0.0]), // "plan" | "top"
            },
            "axonometric" => {
                let spec = world3d_projection_spec_json(p);
                let angle_a = spec.get("angleA").and_then(Value::as_f64).unwrap_or(30.0).to_radians();
                let angle_b = spec.get("angleB").and_then(Value::as_f64).unwrap_or(30.0).to_radians();
                let elevation = (angle_a.tan() * angle_b.tan()).sqrt().asin();
                let azimuth = (angle_a.tan() / angle_b.tan()).sqrt().atan();
                let (sx, sy) = match p.axonometric_quadrant.as_str() {
                    "nw" => (-1.0, 1.0),
                    "se" => (1.0, -1.0),
                    "sw" => (-1.0, -1.0),
                    _ => (1.0, 1.0),
                };
                let dir = [sx * elevation.cos() * azimuth.sin(), sy * elevation.cos() * azimuth.cos(), elevation.sin()];
                ([tx + dir[0] * distance, ty + dir[1] * distance, tz + dir[2] * distance], [0.0, 0.0, 1.0])
            }
            "oblique" => {
                if p.oblique_variant == "military" {
                    let angle = p.oblique_angle.to_radians();
                    ([tx, ty, tz + distance], [angle.sin(), angle.cos(), 0.0])
                } else {
                    ([tx, ty - distance, tz], [0.0, 0.0, 1.0])
                }
            }
            "onePoint" => match p.one_point_axis.as_str() {
                "x" => ([tx - distance, ty, tz], [0.0, 0.0, 1.0]),
                "z" => ([tx, ty, tz + distance], [0.0, 1.0, 0.0]),
                _ => ([tx, ty - distance, tz], [0.0, 0.0, 1.0]),
            },
            "twoPoint" => {
                let azimuth = PI / 4.0;
                ([tx + azimuth.sin() * distance, ty - azimuth.cos() * distance, tz], [0.0, 0.0, 1.0])
            }
            _ => ([tx + distance * 0.6, ty - distance * 0.6, tz + distance * 0.45], [0.0, 0.0, 1.0]),
        }
    }

    /** 📐️ Shared "Projection" window-measures tree: Parallel > Orthographic/Axonometric/Oblique, Perspective > 1/2/3-Point/Curvilinear —
     * every leaf targets `action`, parameter sliders gated to the kind/variant they apply to (see `puzzle3d_fill_utility_options` for the sibling gating pattern). */
    pub fn world3d_projection_measures(id_prefix: &str, p: &WorldProjectionConfig, action: impl Fn(&str, Option<Value>) -> ActionDescriptor) -> WindowMeasure {
        let select = |id: String, value: String, items: Vec<(&str, &str)>, field: &str| WindowMeasure::Select {
            id,
            label: None,
            value,
            items: items.into_iter().map(|(v, label)| MeasureSelectItem { id: v.into(), value: v.into(), label: label.into() }).collect(),
            on_change: action("setProjection", Some(json!({ "field": field }))),
        };
        let slider = |id: String, label: &str, value: f64, min: f64, max: f64, step: f64, param: &str| WindowMeasure::Slider {
            id,
            label: Some(label.into()),
            value,
            min,
            max,
            step: Some(step),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: action("setProjectionParam", Some(json!({ "param": param }))),
        };

        let orthographic_view = if p.kind == "orthographic" { p.orthographic_view.clone() } else { String::new() };
        let orthographic = measure_group_with_open(
            format!("{id_prefix}-projection-orthographic"),
            "Orthographic",
            Some(true),
            vec![select(
                format!("{id_prefix}-projection-orthographic-view"),
                orthographic_view,
                vec![("plan", "Plan"), ("top", "Top"), ("bottom", "Bottom"), ("front", "Front"), ("back", "Back"), ("left", "Left"), ("right", "Right")],
                "orthographicView",
            )],
        );

        let mut axo_children = vec![
            select(
                format!("{id_prefix}-projection-axonometric-variant"),
                if p.kind == "axonometric" { p.axonometric_variant.clone() } else { String::new() },
                vec![("isometric", "Isometric"), ("dimetric", "Dimetric"), ("trimetric", "Trimetric")],
                "axonometricVariant",
            ),
            select(format!("{id_prefix}-projection-axonometric-quadrant"), if p.kind == "axonometric" { p.axonometric_quadrant.clone() } else { String::new() }, vec![("ne", "NE"), ("nw", "NW"), ("se", "SE"), ("sw", "SW")], "axonometricQuadrant"),
        ];
        if p.kind == "axonometric" && p.axonometric_variant != "isometric" {
            axo_children.push(slider(format!("{id_prefix}-projection-axonometric-angle-a"), "Angle", p.axonometric_angle_a, 5.0, if p.axonometric_variant == "dimetric" { 60.0 } else { 75.0 }, 0.5, "axonometricAngleA"));
        }
        if p.kind == "axonometric" && p.axonometric_variant == "trimetric" {
            axo_children.push(slider(format!("{id_prefix}-projection-axonometric-angle-b"), "Angle B", p.axonometric_angle_b, 5.0, 75.0, 0.5, "axonometricAngleB"));
        }
        let axonometric = measure_group_with_open(format!("{id_prefix}-projection-axonometric"), "Axonometric", Some(false), axo_children);

        let mut oblique_children = vec![select(
            format!("{id_prefix}-projection-oblique-variant"),
            if p.kind == "oblique" { p.oblique_variant.clone() } else { String::new() },
            vec![("cabinet", "Cabinet"), ("cavalier", "Cavalier"), ("military", "Military")],
            "obliqueVariant",
        )];
        if p.kind == "oblique" {
            oblique_children.push(slider(format!("{id_prefix}-projection-oblique-angle"), "Angle", p.oblique_angle, 5.0, 90.0, 1.0, "obliqueAngle"));
            if p.oblique_variant != "military" {
                oblique_children.push(slider(format!("{id_prefix}-projection-oblique-depth"), "Depth Scale", p.oblique_depth, 0.05, 1.0, 0.05, "obliqueDepth"));
            }
        }
        let oblique = measure_group_with_open(format!("{id_prefix}-projection-oblique"), "Oblique", Some(false), oblique_children);

        let parallel = measure_group_with_open(format!("{id_prefix}-projection-parallel"), "Parallel", Some(true), vec![orthographic, axonometric, oblique]);

        let perspective_kind_value = match p.kind.as_str() {
            "onePoint" => "onePoint",
            "twoPoint" => "twoPoint",
            "curvilinear" => "curvilinear",
            "threePoint" => "threePoint",
            _ => "",
        };
        let mut perspective_children =
            vec![select(format!("{id_prefix}-projection-perspective-kind"), perspective_kind_value.into(), vec![("onePoint", "1-Point"), ("twoPoint", "2-Point"), ("threePoint", "3-Point"), ("curvilinear", "Curvilinear")], "perspectiveKind")];
        match p.kind.as_str() {
            "onePoint" => {
                perspective_children.push(select(format!("{id_prefix}-projection-one-point-axis"), p.one_point_axis.clone(), vec![("y", "Front (Y)"), ("x", "Side (X)"), ("z", "Down (Z)")], "onePointAxis"));
                perspective_children.push(slider(format!("{id_prefix}-projection-fov"), "Field of View", p.fov, 15.0, 120.0, 1.0, "fov"));
            }
            "twoPoint" => {
                perspective_children.push(slider(format!("{id_prefix}-projection-fov"), "Field of View", p.fov, 15.0, 120.0, 1.0, "fov"));
                perspective_children.push(slider(format!("{id_prefix}-projection-two-point-shift"), "Vertical Shift", p.two_point_shift, -1.0, 1.0, 0.01, "twoPointShift"));
            }
            "threePoint" => {
                perspective_children.push(slider(format!("{id_prefix}-projection-fov"), "Field of View", p.fov, 15.0, 120.0, 1.0, "fov"));
            }
            "curvilinear" => {
                perspective_children.push(slider(format!("{id_prefix}-projection-curvilinear-fov"), "Field of View", p.curvilinear_fov, 60.0, 160.0, 1.0, "curvilinearFov"));
                perspective_children.push(slider(format!("{id_prefix}-projection-curvilinear-strength"), "Strength", p.curvilinear_strength, 0.0, 1.0, 0.01, "curvilinearStrength"));
                perspective_children.push(select(format!("{id_prefix}-projection-curvilinear-mapping"), p.curvilinear_mapping.clone(), vec![("fisheye", "Fisheye"), ("panini", "Panini")], "curvilinearMapping"));
            }
            _ => {}
        }
        let perspective = measure_group_with_open(format!("{id_prefix}-projection-perspective"), "Perspective", Some(true), perspective_children);

        measure_group_with_open(format!("{id_prefix}-projection"), "Projection", Some(false), vec![parallel, perspective])
    }

    /** 📐️ Applies `setProjection`/`setProjectionParam` to `p`, returning whether the action was handled. */
    pub fn apply_world3d_projection_action(p: &mut WorldProjectionConfig, action_id: &str, args: Option<&Value>) -> bool {
        let field = args.and_then(|v| v.get("field")).and_then(Value::as_str);
        let value_str = args.and_then(|v| v.get("value")).and_then(Value::as_str);
        let value_f64 = args.and_then(|v| v.get("value")).and_then(Value::as_f64);
        let param = args.and_then(|v| v.get("param")).and_then(Value::as_str);
        match action_id {
            "setProjection" => {
                match (field, value_str) {
                    (Some("orthographicView"), Some(value)) => {
                        p.kind = "orthographic".into();
                        p.orthographic_view = value.into();
                    }
                    (Some("axonometricVariant"), Some(value)) => {
                        p.kind = "axonometric".into();
                        p.axonometric_variant = value.into();
                    }
                    (Some("axonometricQuadrant"), Some(value)) => {
                        p.kind = "axonometric".into();
                        p.axonometric_quadrant = value.into();
                    }
                    (Some("obliqueVariant"), Some(value)) => {
                        p.kind = "oblique".into();
                        p.oblique_variant = value.into();
                    }
                    (Some("perspectiveKind"), Some(value)) => {
                        p.kind = value.into();
                    }
                    (Some("onePointAxis"), Some(value)) => {
                        p.one_point_axis = value.into();
                    }
                    (Some("curvilinearMapping"), Some(value)) => {
                        p.curvilinear_mapping = value.into();
                    }
                    _ => return false,
                }
                true
            }
            "setProjectionParam" => {
                let (Some(param), Some(value)) = (param, value_f64) else { return false };
                match param {
                    "axonometricAngleA" => p.axonometric_angle_a = value,
                    "axonometricAngleB" => p.axonometric_angle_b = value,
                    "obliqueAngle" => p.oblique_angle = value,
                    "obliqueDepth" => p.oblique_depth = value,
                    "fov" => p.fov = value,
                    "twoPointShift" => p.two_point_shift = value,
                    "curvilinearFov" => p.curvilinear_fov = value,
                    "curvilinearStrength" => p.curvilinear_strength = value,
                    _ => return false,
                }
                true
            }
            _ => false,
        }
    }

    /** 📐️ Whether a `setProjection`/`setProjectionParam` action requires a pose recompute (kind/view/variant changes) vs. a pure in-place parameter tweak that keeps the current pose (oblique shear / two-point shift / fov / curvilinear re-shade live). */
    pub fn world3d_projection_action_moves_pose(action_id: &str, args: Option<&Value>) -> bool {
        if action_id != "setProjection" {
            return false;
        }
        matches!(args.and_then(|v| v.get("field")).and_then(Value::as_str), Some("orthographicView") | Some("axonometricVariant") | Some("axonometricQuadrant") | Some("obliqueVariant") | Some("perspectiveKind"))
    }
    //#endregion 📐️ WorldProjection

    pub fn mesh_kind_from_json(mesh_json: &str) -> String {
        serde_json::from_str::<Value>(mesh_json).ok().and_then(|value| value.get("kind").and_then(|v| v.as_str()).map(str::to_string)).unwrap_or_else(|| "box".into())
    }

    pub fn world3d_meshes_json_from_kinds(kinds: &[String]) -> String {
        let meshes: Vec<Value> = kinds
            .iter()
            .map(|kind| {
                let data = mesh_from_kind(kind);
                json!({ "id": kind, "data": data })
            })
            .collect();
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
    }

    pub fn world3d_mesh_id_from_url(url: &str) -> String {
        let slug = url.trim_start_matches('/').rsplit('/').next().unwrap_or(url).trim_end_matches(".glb").trim_end_matches(".gltf");
        format!("mesh:{slug}")
    }

    pub fn world3d_meshes_json_from_urls(urls: &[String]) -> String {
        let meshes: Vec<Value> = urls
            .iter()
            .map(|url| {
                json!({
                    "id": world3d_mesh_id_from_url(url),
                    "url": url,
                })
            })
            .collect();
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
    }

    pub fn world3d_meshes_json_from_kinds_and_urls(kinds: &[String], urls: &[String]) -> String {
        let mut meshes: Vec<Value> = kinds
            .iter()
            .map(|kind| {
                let data = mesh_from_kind(kind);
                json!({ "id": kind, "data": data })
            })
            .collect();
        for url in urls {
            let id = world3d_mesh_id_from_url(url);
            if meshes.iter().any(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(id.as_str())) {
                continue;
            }
            meshes.push(json!({ "id": id, "url": url }));
        }
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
    }

    pub fn world3d_selection_json(method: &str, ids: &[String], hovered_id: Option<&str>) -> String {
        world3d_selection_json_with_granularity(method, ids, hovered_id, None)
    }

    pub fn world3d_selection_json_with_granularity(method: &str, ids: &[String], hovered_id: Option<&str>, granularity: Option<&str>) -> String {
        let mut value = json!({
            "method": method,
            "mode": "replace",
            "ids": ids,
            "hoveredId": hovered_id,
        });
        if let Some(entry) = granularity {
            if let Some(object) = value.as_object_mut() {
                object.insert("granularity".into(), json!(entry));
            }
        }
        value.to_string()
    }

    pub fn world3d_scene(camera_json: String, meshes_json: String, instances_json: String, selection_json: String, sun: &WorldSunConfig) -> World3dScene {
        world3d_scene_extended(camera_json, meshes_json, instances_json, selection_json, None, None, None, None, None, None, None, None, None, Some(world3d_environment_json(sun)), None, None, None, None, None)
    }

    pub fn world3d_scene_extended(
        camera_json: String,
        meshes_json: String,
        instances_json: String,
        selection_json: String,
        vortices_json: Option<String>,
        attractions_json: Option<String>,
        target_volumes_json: Option<String>,
        references_json: Option<String>,
        brush_preview_json: Option<String>,
        interaction_json: Option<String>,
        engagement_preview_json: Option<String>,
        lod_json: Option<String>,
        chunking_json: Option<String>,
        environment_json: Option<String>,
        frame_json: Option<String>,
        fit_json: Option<String>,
        terrain_json: Option<String>,
        points_json: Option<String>,
        status_json: Option<String>,
    ) -> World3dScene {
        World3dScene {
            camera_json,
            meshes_json,
            instances_json,
            selection_json,
            vortices_json,
            attractions_json,
            target_volumes_json,
            references_json,
            brush_preview_json,
            interaction_json,
            engagement_preview_json,
            lod_json,
            chunking_json,
            environment_json,
            frame_json,
            fit_json,
            terrain_json,
            points_json,
            status_json,
        }
    }

    pub fn world3d_default_camera() -> String {
        world3d_camera_json([4.0, -4.0, 3.0], [0.0, 0.0, 0.0], 45.0)
    }

    /** @emoji ✅️ Ordered selection ids with O(1) membership — serializes as a plain JSON string array. */
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct SelectionSet {
        ids: Vec<String>,
        index: std::collections::HashSet<String>,
    }

    impl SelectionSet {
        pub fn from_ids(ids: Vec<String>) -> Self {
            let index: std::collections::HashSet<String> = ids.iter().cloned().collect();
            Self { ids, index }
        }

        pub fn contains(&self, id: &str) -> bool {
            self.index.contains(id)
        }

        pub fn is_empty(&self) -> bool {
            self.ids.is_empty()
        }

        pub fn len(&self) -> usize {
            self.ids.len()
        }

        pub fn clear(&mut self) {
            self.ids.clear();
            self.index.clear();
        }

        pub fn first(&self) -> Option<&str> {
            self.ids.first().map(String::as_str)
        }

        pub fn as_slice(&self) -> &[String] {
            &self.ids
        }

        pub fn iter(&self) -> impl Iterator<Item = &String> {
            self.ids.iter()
        }

        pub fn push_unique(&mut self, id: String) {
            if self.index.insert(id.clone()) {
                self.ids.push(id);
            }
        }

        pub fn remove_id(&mut self, id: &str) {
            if self.index.remove(id) {
                self.ids.retain(|entry| entry != id);
            }
        }

        pub fn to_vec(&self) -> Vec<String> {
            self.ids.clone()
        }
    }

    impl IntoIterator for SelectionSet {
        type Item = String;
        type IntoIter = std::vec::IntoIter<String>;

        fn into_iter(self) -> Self::IntoIter {
            self.ids.into_iter()
        }
    }

    impl<'a> IntoIterator for &'a SelectionSet {
        type Item = &'a String;
        type IntoIter = std::slice::Iter<'a, String>;

        fn into_iter(self) -> Self::IntoIter {
            self.ids.iter()
        }
    }

    impl Serialize for SelectionSet {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.ids.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for SelectionSet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let ids = Vec::<String>::deserialize(deserializer)?;
            Ok(Self::from_ids(ids))
        }
    }

    impl From<Vec<String>> for SelectionSet {
        fn from(ids: Vec<String>) -> Self {
            Self::from_ids(ids)
        }
    }

    impl FromIterator<String> for SelectionSet {
        fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
            Self::from_ids(iter.into_iter().collect())
        }
    }

    pub fn merge_world_selection_ids(existing: &SelectionSet, incoming: &[String], merge: &str) -> SelectionSet {
        match merge {
            "add" => {
                let mut merged = existing.clone();
                for id in incoming {
                    merged.push_unique(id.clone());
                }
                merged
            }
            "toggle" | "invertive" => {
                let mut merged = existing.clone();
                for id in incoming {
                    if merged.index.remove(id) {
                        merged.ids.retain(|entry| entry != id);
                    } else {
                        merged.push_unique(id.clone());
                    }
                }
                merged
            }
            "remove" | "subtractive" => {
                let mut merged = existing.clone();
                for id in incoming {
                    if merged.index.remove(id) {
                        merged.ids.retain(|entry| entry != id);
                    }
                }
                merged
            }
            _ => SelectionSet::from_ids(incoming.to_vec()),
        }
    }

    pub fn default_world3d_selection() -> String {
        world3d_default_selection_json()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn merge_world_selection_ids_supports_add_toggle_invertive_and_remove() {
            let a = || SelectionSet::from_ids(vec!["a".into()]);
            let ab = || SelectionSet::from_ids(vec!["a".into(), "b".into()]);
            let abc = || SelectionSet::from_ids(vec!["a".into(), "b".into(), "c".into()]);
            assert_eq!(merge_world_selection_ids(&a(), &["b".into()], "add").as_slice(), &["a".to_string(), "b".to_string()]);
            assert_eq!(merge_world_selection_ids(&ab(), &["b".into(), "c".into()], "toggle").as_slice(), &["a".to_string(), "c".to_string()]);
            assert_eq!(merge_world_selection_ids(&ab(), &["b".into()], "invertive").as_slice(), &["a".to_string()]);
            assert_eq!(merge_world_selection_ids(&a(), &["b".into()], "replace").as_slice(), &["b".to_string()]);
            assert_eq!(merge_world_selection_ids(&abc(), &["b".into()], "remove").as_slice(), &["a".to_string(), "c".to_string()]);
            assert_eq!(merge_world_selection_ids(&abc(), &["b".into()], "subtractive").as_slice(), &["a".to_string(), "c".to_string()]);
        }

        #[test]
        fn selection_set_membership_is_constant_time() {
            let set = SelectionSet::from_ids((0..100).map(|index| format!("id-{index}")).collect());
            assert!(set.contains("id-50"));
            assert!(!set.contains("missing"));
        }

        #[test]
        fn isometric_pose_matches_the_classic_35_264_45_direction() {
            let mut p = WorldProjectionConfig { kind: "axonometric".into(), axonometric_variant: "isometric".into(), ..WorldProjectionConfig::default() };
            p.axonometric_quadrant = "ne".into();
            let (position, up) = world3d_projection_pose(&p, [0.0, 0.0, 0.0], 10.0);
            assert!((position[2] / 10.0 - 35.264_f64.to_radians().sin()).abs() < 1e-3);
            assert_eq!(up, [0.0, 0.0, 1.0]);
            let azimuth = (position[0] / position[1]).atan();
            assert!((azimuth.to_degrees() - 45.0).abs() < 1e-3);
        }

        #[test]
        fn projection_spec_json_projects_only_active_kind_fields() {
            let p = WorldProjectionConfig { kind: "oblique".into(), oblique_variant: "cabinet".into(), oblique_angle: 45.0, oblique_depth: 0.5, ..WorldProjectionConfig::default() };
            let spec = world3d_projection_spec_json(&p);
            let mode = spec.get("mode").expect("mode object");
            assert_eq!(mode.get("kind").and_then(Value::as_str), Some("oblique"));
            assert_eq!(mode.get("depthScale").and_then(Value::as_f64), Some(0.5));
            assert!(mode.get("axonometricVariant").is_none());
        }

        #[test]
        fn apply_action_switches_kind_and_leaves_other_kinds_untouched_for_later_recall() {
            let mut p = WorldProjectionConfig::default();
            p.axonometric_angle_a = 22.0;
            assert!(apply_world3d_projection_action(&mut p, "setProjection", Some(&json!({ "field": "obliqueVariant", "value": "military" }))));
            assert_eq!(p.kind, "oblique");
            assert_eq!(p.oblique_variant, "military");
            assert_eq!(p.axonometric_angle_a, 22.0);
            assert!(apply_world3d_projection_action(&mut p, "setProjectionParam", Some(&json!({ "param": "obliqueAngle", "value": 30.0 }))));
            assert_eq!(p.oblique_angle, 30.0);
            assert!(!world3d_projection_action_moves_pose("setProjectionParam", Some(&json!({ "param": "obliqueAngle" }))));
            assert!(world3d_projection_action_moves_pose("setProjection", Some(&json!({ "field": "obliqueVariant" }))));
        }

        #[test]
        fn projection_measures_tree_matches_the_requested_taxonomy() {
            let p = WorldProjectionConfig::default();
            let tree = world3d_projection_measures("t", &p, |action, args| ActionDescriptor { controller_id: "t".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) });
            let WindowMeasure::Group { children: families, .. } = &tree else { panic!("expected root group") };
            assert_eq!(families.len(), 2);
            let WindowMeasure::Group { label: parallel_label, children: parallel_children, .. } = &families[0] else { panic!("expected parallel group") };
            assert_eq!(parallel_label, "Parallel");
            assert_eq!(parallel_children.len(), 3);
            let WindowMeasure::Group { label: perspective_label, .. } = &families[1] else { panic!("expected perspective group") };
            assert_eq!(perspective_label, "Perspective");
        }
    }
    // #endregion world3d_host
}

pub mod host_port {
    // #region host_port
    //! 🗄️ Host-capability access for WASI component builds — the backbone duplex channel and wall-clock time.

    /** @emoji 📤️ Sends a backbone message through the component host; errs when no host is linked. */
    pub fn host_backbone_send(uri: &str, message: &[u8]) -> Result<(), String> {
        #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
        {
            return crate::component::host_backbone_send(uri, message);
        }
        #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
        {
            let _ = message;
            Err(format!("host backbone unavailable: {uri}"))
        }
    }

    /** @emoji 📥️ Polls queued backbone messages through the component host; errs when no host is linked. */
    pub fn host_backbone_poll(uri: &str) -> Result<Vec<Vec<u8>>, String> {
        #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
        {
            return crate::component::host_backbone_poll(uri);
        }
        #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
        Err(format!("host backbone unavailable: {uri}"))
    }

    /** @emoji 🩺️ Queries the host for the sync status of a backbone uri; errs when no host is linked. */
    pub fn host_backbone_status(uri: &str) -> Result<String, String> {
        #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
        {
            return crate::component::host_backbone_status(uri);
        }
        #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
        Err(format!("host backbone unavailable: {uri}"))
    }

    /** @emoji ⏱️ Wall-clock milliseconds from the component host, falling back to system time. */
    pub fn host_now_ms() -> f64 {
        #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
        {
            return crate::component::host_now_ms() as f64;
        }
        #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as f64).unwrap_or(0.0)
    }

    /** @emoji 📦️ Fetches a host-registered static asset by handle (e.g. `infinite_canvas`'s GuestSlim
    typst font blob); errs when no host is linked or the handle is unknown. */
    pub fn host_read_asset(handle: u64) -> Result<Vec<u8>, String> {
        #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
        {
            return crate::component::host_read_asset(handle);
        }
        #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
        Err(format!("host asset unavailable: {handle}"))
    }

    /** @emoji 🔌️ vcs backbone channel backed by the component host's duplex capability. */
    pub struct HostBackboneChannel;

    impl store::BackboneChannelPort for HostBackboneChannel {
        fn send(&self, uri: &str, message: &[u8]) -> Result<(), vcs::VcsError> {
            host_backbone_send(uri, message).map_err(vcs::VcsError::Backbone)
        }

        fn poll(&self, uri: &str) -> Result<Vec<Vec<u8>>, vcs::VcsError> {
            host_backbone_poll(uri).map_err(vcs::VcsError::Backbone)
        }
    }

    /** @emoji 🧷️ Installs the component host as the vcs backbone channel so the plugin's document store
    can synchronize across the wasm sandbox boundary. */
    pub fn register_host_backbone_channel() {
        store::set_host_backbone_channel(std::sync::Arc::new(HostBackboneChannel));
    }
    // #endregion host_port
}

pub mod engagement {
    // #region engagement
    //! 🎛️ Parses engagement command-line drafts submitted by the React shell, which PascalCases every
    //! draft and strips separators (`ui/js/react/index.tsx` `normalizeEngagementActionText`) before
    //! dispatching — so `"fill 20"` arrives as `"Fill20"`, not `"fill 20"`.

    /** @emoji ✂️ Strips a leading `command` token from `raw`, ignoring case and separators on both
    sides, and returns the trimmed remainder (e.g. `strip_engagement_prefix("Fill20", "fill")`
    and `strip_engagement_prefix("fill 20", "fill")` both yield `Some("20")`). Decimal points
    inside numeric remainders are preserved. Returns `None` when `raw` doesn't start with `command`. */
    pub fn strip_engagement_prefix<'a>(raw: &'a str, command: &str) -> Option<&'a str> {
        let raw_bytes = raw.as_bytes();
        let mut raw_index = 0usize;
        let mut command_chars = command.chars().filter(|ch| ch.is_alphanumeric());
        while let Some(expected) = command_chars.next() {
            while raw_index < raw_bytes.len() {
                let ch = raw[raw_index..].chars().next().unwrap();
                if ch.is_alphanumeric() {
                    break;
                }
                raw_index += ch.len_utf8();
            }
            let Some(actual) = raw[raw_index..].chars().next() else {
                return None;
            };
            if !actual.eq_ignore_ascii_case(&expected) {
                return None;
            }
            raw_index += actual.len_utf8();
        }
        let mut remainder_start = raw_index;
        while remainder_start < raw_bytes.len() {
            let ch = raw[remainder_start..].chars().next().unwrap();
            if ch.is_alphanumeric() || ch == '.' {
                break;
            }
            remainder_start += ch.len_utf8();
        }
        Some(raw[remainder_start..].trim())
    }

    /** @emoji 🔤️ True when `raw` matches `command` in full, ignoring case and separators (e.g.
    `engagement_token_matches("LineNumbers", "line numbers")` is `true`). */
    pub fn engagement_token_matches(raw: &str, command: &str) -> bool {
        strip_engagement_prefix(raw, command).is_some_and(str::is_empty)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn strip_engagement_prefix_accepts_normalized_and_raw_forms() {
            assert_eq!(strip_engagement_prefix("Fill20", "fill"), Some("20"));
            assert_eq!(strip_engagement_prefix("fill 20", "fill"), Some("20"));
            assert_eq!(strip_engagement_prefix("fill  20", "fill"), Some("20"));
            assert_eq!(strip_engagement_prefix("Fill", "fill"), Some(""));
            assert_eq!(strip_engagement_prefix("FILL20", "fill"), Some("20"));
        }

        #[test]
        fn strip_engagement_prefix_preserves_decimal_points() {
            assert_eq!(strip_engagement_prefix("SetHeight3.5", "set height"), Some("3.5"));
            assert_eq!(strip_engagement_prefix("set height 3.5", "set height"), Some("3.5"));
        }

        #[test]
        fn strip_engagement_prefix_rejects_non_matching_commands() {
            assert_eq!(strip_engagement_prefix("Brush", "fill"), None);
            assert_eq!(strip_engagement_prefix("Filled", "fill"), Some("ed"));
        }

        #[test]
        fn engagement_token_matches_full_token_only() {
            assert!(engagement_token_matches("LineNumbers", "line numbers"));
            assert!(engagement_token_matches("linenumbers", "line numbers"));
            assert!(!engagement_token_matches("LineNumbers2", "line numbers"));
            assert!(!engagement_token_matches("Line", "line numbers"));
        }
    }
    // #endregion engagement
}

pub use app::testkit;
pub use app::ActionFactory;
pub use app::{
    node_graph_delete_selection_spec, selection_count_phrase, selection_domains_from_surface, ActionMeta, App, AppActionRegistry, AppBuilder, AppInstance, ArtifactBuilder, ArtifactDecomposer, ArtifactAnalyzer, ArtifactComposer, ArtifactAnalysis, ArtifactChildren, ArtifactComposition, ArtifactInferrer, ArtifactSerializer, ArtifactDeserializer, DerivedArtifactSpec, DerivedArtifactParts, DerivedArtifactBuilder, DerivedArtifactAnalyzer, DerivedArtifactComposer, composer_entry_of, deserializer_entry_of, serializer_entry_of, ArtifactKindSpec, Confidence, Decomposition, DecomposeSource, ConfigView, ArtifactApp, ArtifactView, DraftView, Emit, ExampleSource, HistoryView,
    KeybindingSpec, MediaClass, MediaType, Menu, ModeSpec, NoChildren, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NodeGraphDeleteDispatch, OsMediaCapability, PanelTabSpec, PanelTreeBuilder, Plugin, PluginApp, PluginBuilder, PluginProgram, VcsArtifactApp,
    WindowKindSpec, ArtifactDeclaration,
    // 🧸️👥️🫧️ Composition child-read seam plus the two ephemeral state lanes.
    ChildContentView, EphemeralEmit, NoTransient, NoTransientMutation, PresenceView, TransientView,
};
pub use app::{locale_from_str, resolve_labels, resolve_labels_for_locale, selection_ids, tree_item, tree_item_desc, tree_item_with_action, tree_item_with_action_draggable, LabelAxes};
pub use engagement::{engagement_token_matches, strip_engagement_prefix};
pub use host_port::{host_backbone_poll, host_backbone_send, host_backbone_status, host_now_ms, host_read_asset, register_host_backbone_channel, HostBackboneChannel};
pub use plugin_runtime::{
    extension_activate, extension_deactivate, extension_invoke, extension_manifest, install_extension_bundle, install_plugin_bundle,
    plugin_attach_backbone, plugin_detach_backbone, plugin_document_pack, plugin_ingest_operations, plugin_load_document_pack, ExtensionBundle,
    ExtensionManifest,
};
pub use semio_framework::*;
pub use semio_framework::{MediaForm, MediaPortDirection, MediaPortSpec};
pub use world3d_host::{
    apply_world3d_projection_action, apply_world3d_sun_action, default_world3d_selection, merge_world_selection_ids, mesh_kind_from_json, world3d_camera_projection_json, world3d_default_camera,
    world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds, world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_projection_action_moves_pose, world3d_projection_measures,
    world3d_projection_pose, world3d_projection_spec_json, world3d_scene, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, SelectionSet, WorldProjectionConfig, WorldSunConfig,
};
// 🧩️ Declarative component model (UiNode, layouts, utilities) — moved into ui_wgpu; re-exported here so
// apps keep the flat `semio_framework_plugin::*` import surface with zero Cargo.toml churn.
pub use ui_wgpu::wgpu::*;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle($bundle);
    };
}

/// 🧬️ Declares uniform artifact lifecycle types from schema and IO hooks. UNIFIED-COMPOSABLE-
/// ARTIFACT-SYSTEM (C1, Task 5): accepts an optional trailing `children: $ty` field naming a real
/// `ArtifactChildren` impl (`$ty::Snapshot` must equal `<$construction as ArtifactBuilder>::Snapshot`,
/// enforced by `DerivedArtifactSpec::Children`'s own bound) — when present, `$composer`'s `reads()`
/// gains each declared slot's dialect and `compose()` routes matching sources through
/// `$ty::compose_from_children` (see `DerivedArtifactComposer`'s own doc comment for the mechanism;
/// this macro only has to name the type, the behavior lives centrally there). Omitting `children`
/// (every pre-C1 invocation) defaults `type Children` to `NoChildren<Snapshot>` — a leaf artifact
/// needs zero changes to keep compiling.
#[macro_export]
macro_rules! derive_artifact_facets {
    (
        $visibility:vis spec $spec:ident {
            construction: $construction:ty,
            analysis: $analysis:ty,
            composition: $composition:ty
            $(, children: $children:ty)? $(,)?
        }
        builder: $builder:ident,
        analyzer: $analyzer:ident,
        composer: $composer:ident $(,)?
    ) => {
        $visibility struct $spec;

        impl $crate::DerivedArtifactSpec for $spec {
            type Snapshot = <$construction as $crate::ArtifactBuilder>::Snapshot;
            type Mutation = <$construction as $crate::ArtifactBuilder>::Mutation;
            type Diff = <$construction as $crate::ArtifactBuilder>::Diff;
            type Construction = $construction;
            type Analysis = $analysis;
            type Composition = $composition;
            type Children = $crate::derive_artifact_facets!(@children_ty <$construction as $crate::ArtifactBuilder>::Snapshot $(, $children)?);
        }

        #[derive(Clone, Debug, Default)]
        $visibility struct $builder($crate::DerivedArtifactBuilder<$spec>);

        impl $crate::ArtifactBuilder for $builder {
            type Snapshot = <$spec as $crate::DerivedArtifactSpec>::Snapshot;
            type Mutation = <$spec as $crate::DerivedArtifactSpec>::Mutation;
            type Diff = <$spec as $crate::DerivedArtifactSpec>::Diff;

            fn empty() -> Self { Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::empty()) }
            fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_snapshot(snapshot)) }
            fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_text(text)?)) }
            fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::from_binary(bytes)?)) }
            fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (builder, diff) = <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::mutate(self.0, mutation); (Self(builder), diff) }
            fn absorb(self, diff: Self::Diff) -> Self { Self(<$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::absorb(self.0, diff)) }
            fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { <$crate::DerivedArtifactBuilder<$spec> as $crate::ArtifactBuilder>::build(self.0) }
        }

        $visibility struct $analyzer;

        impl $crate::ArtifactAnalyzer for $analyzer {
            type Parts = <$analysis as $crate::ArtifactAnalysis>::Parts;
            const DIALECT: $crate::Dialect = <$analysis as $crate::ArtifactAnalysis>::DIALECT;
            fn sniff(source: &$crate::AnalyzeSource<'_>) -> $crate::IoConfidence { <$analysis as $crate::ArtifactAnalysis>::sniff(source) }
            fn analyze(sources: &[$crate::AnalyzeSource<'_>]) -> $crate::Analysis<Self::Parts> { <$analysis as $crate::ArtifactAnalysis>::analyze(sources) }
        }

        impl $analyzer {
            pub fn sniff(source: &$crate::AnalyzeSource<'_>) -> $crate::IoConfidence { <Self as $crate::ArtifactAnalyzer>::sniff(source) }
            pub fn analyze(sources: &[$crate::AnalyzeSource<'_>]) -> $crate::Analysis<<Self as $crate::ArtifactAnalyzer>::Parts> { <Self as $crate::ArtifactAnalyzer>::analyze(sources) }
        }

        $visibility struct $composer;

        // 🧩️ `reads()`/`compose()` delegate to `DerivedArtifactComposer<$spec>` (NOT `$composition`
        // directly, as before C1) so the child-slot union/routing logic lives in exactly ONE place
        // (`DerivedArtifactComposer`'s own `impl ArtifactComposer` — see its doc comment) rather than
        // being re-derived per macro expansion; `WRITES` is unaffected by children and stays a
        // direct read of `$composition::WRITES`.
        impl $crate::ArtifactComposer for $composer {
            type Snapshot = <$spec as $crate::DerivedArtifactSpec>::Snapshot;
            const WRITES: $crate::Dialect = <$composition as $crate::ArtifactComposition>::WRITES;
            fn reads() -> &'static [$crate::Dialect] { <$crate::DerivedArtifactComposer<$spec> as $crate::ArtifactComposer>::reads() }
            fn compose(sources: &[$crate::ComposeSource<'_>]) -> Result<$crate::Composition<Self::Snapshot>, $crate::ComposeError> { <$crate::DerivedArtifactComposer<$spec> as $crate::ArtifactComposer>::compose(sources) }
        }

        impl $composer {
            pub fn compose(sources: &[$crate::ComposeSource<'_>]) -> Result<$crate::Composition<<Self as $crate::ArtifactComposer>::Snapshot>, $crate::ComposeError> { <Self as $crate::ArtifactComposer>::compose(sources) }
        }
    };
    // 🧩️ Internal dispatch arm (Task 5): resolves `DerivedArtifactSpec::Children` — `$children` if
    // the caller supplied one, else `NoChildren<$snapshot>`. Never invoked directly by a caller (the
    // leading `@children_ty` token is not part of the public macro grammar above).
    (@children_ty $snapshot:ty) => { $crate::app::NoChildren<$snapshot> };
    (@children_ty $snapshot:ty, $children:ty) => { $children };
}

/// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5) — `ArtifactChildren`/`DerivedArtifactSpec::Children`
/// tests. Placed here (rather than beside `🧬️DerivedArtifactFacets` near the top of the file) so
/// `derive_artifact_facets!` is already in scope textually — `macro_rules!` visibility is
/// definition-order-sensitive within a crate for a bare (non-`$crate`-qualified) invocation.
#[cfg(test)]
mod derived_artifact_children_tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, PartialEq)]
    struct ChildrenTestSnapshot;

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct ChildrenTestDiff;

    impl protocol::MutationDiff<ChildrenTestSnapshot> for ChildrenTestDiff {
        fn apply(&self, snapshot: &ChildrenTestSnapshot) -> ChildrenTestSnapshot {
            snapshot.clone()
        }
        fn absorb(&mut self, _other: Self) {}
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct ChildrenTestMutation;

    impl protocol::Mutation<ChildrenTestSnapshot> for ChildrenTestMutation {
        type Diff = ChildrenTestDiff;
        fn diff(&self, _snapshot: &ChildrenTestSnapshot) -> ChildrenTestDiff {
            ChildrenTestDiff
        }
        fn inverse(&self, _snapshot: &ChildrenTestSnapshot) -> Vec<Self> {
            Vec::new()
        }
    }

    #[derive(Clone, Debug)]
    struct ChildrenTestConstruction(ChildrenTestSnapshot);

    impl ArtifactBuilder for ChildrenTestConstruction {
        type Snapshot = ChildrenTestSnapshot;
        type Mutation = ChildrenTestMutation;
        type Diff = ChildrenTestDiff;
        fn empty() -> Self {
            Self(ChildrenTestSnapshot)
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self(snapshot)
        }
        fn from_text(_text: &str) -> Result<Self, store::TextError> {
            Ok(Self::empty())
        }
        fn from_binary(_bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::empty())
        }
        fn mutate(self, _mutation: Self::Mutation) -> (Self, Self::Diff) {
            (self, ChildrenTestDiff)
        }
        fn absorb(self, _diff: Self::Diff) -> Self {
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.0)
        }
    }

    struct ChildrenTestAnalysis;

    impl ArtifactAnalysis for ChildrenTestAnalysis {
        type Parts = ();
        const DIALECT: Dialect = Dialect { artifact_kind: "s.test.children-parent", standard: StandardId("1"), subset: SubsetId::ANY };
        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Low
        }
        fn analyze(_sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            Analysis { parts: (), dialect: Self::DIALECT, confidence: IoConfidence::Low, diagnostics: Vec::new() }
        }
    }

    struct ChildrenTestComposition;

    impl ArtifactComposition for ChildrenTestComposition {
        type Snapshot = ChildrenTestSnapshot;
        const WRITES: Dialect = ChildrenTestAnalysis::DIALECT;
        fn reads() -> &'static [Dialect] {
            &[]
        }
        fn compose(_sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            Err(ComposeError { message: "ChildrenTestComposition::compose is unreachable in this test — only reads()/child routing is exercised".into(), diagnostics: Vec::new() })
        }
    }

    const CHILD_SLOTS: &[::semio_framework_schema::ChildSlotSpec] = &[::semio_framework_schema::ChildSlotSpec { name: "primaryMesh", kind: "s.stdio.mesh", many: false }];

    struct ChildrenTestChildren;

    impl ArtifactChildren for ChildrenTestChildren {
        type Snapshot = ChildrenTestSnapshot;
        fn slots() -> &'static [::semio_framework_schema::ChildSlotSpec] {
            CHILD_SLOTS
        }
        fn compose_from_children(parts: &[(ArtifactDialect, Vec<u8>)]) -> Result<Self::Snapshot, ComposeError> {
            if parts.iter().all(|(dialect, _)| dialect.artifact_kind == "s.stdio.mesh") {
                Ok(ChildrenTestSnapshot)
            } else {
                Err(ComposeError { message: "unexpected child dialect".into(), diagnostics: Vec::new() })
            }
        }
        fn decompose_to_children(_snapshot: &Self::Snapshot) -> Vec<(ArtifactDialect, Vec<u8>)> {
            Vec::new()
        }
    }

    struct ChildrenTestSpec;

    impl DerivedArtifactSpec for ChildrenTestSpec {
        type Snapshot = ChildrenTestSnapshot;
        type Mutation = ChildrenTestMutation;
        type Diff = ChildrenTestDiff;
        type Construction = ChildrenTestConstruction;
        type Analysis = ChildrenTestAnalysis;
        type Composition = ChildrenTestComposition;
        type Children = ChildrenTestChildren;
    }

    #[test]
    fn derived_composer_reads_includes_child_slot_dialects() {
        let reads = <DerivedArtifactComposer<ChildrenTestSpec> as ArtifactComposer>::reads();
        assert_eq!(reads.len(), 1, "Composition::reads() is empty here — the ONE entry must be the child slot's synthesized dialect");
        assert_eq!(reads[0].artifact_kind, "s.stdio.mesh");
        assert_eq!(reads[0].standard, StandardId("*"));
        assert_eq!(reads[0].subset, SubsetId::ANY);
    }

    #[test]
    fn derived_composer_reads_defaults_to_composition_reads_for_a_leaf_with_no_children() {
        // 🍃️ `NoChildren<S>::slots()` is `&[]` — proves the pre-C1 behavior is byte-identical for
        // any spec that never names a `children: $ty` (every macro invocation before this wave).
        struct LeafSpec;
        impl DerivedArtifactSpec for LeafSpec {
            type Snapshot = ChildrenTestSnapshot;
            type Mutation = ChildrenTestMutation;
            type Diff = ChildrenTestDiff;
            type Construction = ChildrenTestConstruction;
            type Analysis = ChildrenTestAnalysis;
            type Composition = ChildrenTestComposition;
            type Children = NoChildren<ChildrenTestSnapshot>;
        }
        let reads = <DerivedArtifactComposer<LeafSpec> as ArtifactComposer>::reads();
        assert!(reads.is_empty(), "a leaf spec's reads() must equal Composition::reads() (empty here) exactly");
    }

    #[test]
    fn derived_composer_compose_routes_matching_sources_through_compose_from_children() {
        let sources = vec![ComposeSource { dialect: Dialect { artifact_kind: "s.stdio.mesh", standard: StandardId("1"), subset: SubsetId::ANY }, payload: AnalyzeSource::Binary(&[1, 2, 3]) }];
        let composed = <DerivedArtifactComposer<ChildrenTestSpec> as ArtifactComposer>::compose(&sources).expect("child-slot-matching sources route through compose_from_children");
        assert_eq!(composed.snapshot, ChildrenTestSnapshot);
    }

    /// 🧬️ Smoke-tests the macro's own `children: $ty` grammar (not just the underlying
    /// `DerivedArtifactComposer` mechanism above) — proves `derive_artifact_facets!` actually parses
    /// and wires the optional field end to end.
    derive_artifact_facets! {
        spec ChildrenMacroSpec {
            construction: ChildrenTestConstruction,
            analysis: ChildrenTestAnalysis,
            composition: ChildrenTestComposition,
            children: ChildrenTestChildren,
        }
        builder: ChildrenMacroBuilder,
        analyzer: ChildrenMacroAnalyzer,
        composer: ChildrenMacroComposer,
    }

    #[test]
    fn derive_artifact_facets_children_arm_wires_the_macro_generated_composer() {
        let reads = <ChildrenMacroComposer as ArtifactComposer>::reads();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].artifact_kind, "s.stdio.mesh");
    }
}

/// 🪆️ Whether a subset owns snapshot/diff/mutation types or derives behavior from a base dialect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubsetKind {
    Owning,
    Derived,
}

/// 🪆️ Declares one subset's lifecycle facets, runtime registration, and inline conformance tests.
#[macro_export]
macro_rules! subset {
    (
        $vis:vis owning dialect $artifact:literal / $standard:literal / $subset:literal {
            spec $spec:ident {
                construction: $construction:ty,
                analysis: $analysis:ty,
                composition: $composition:ty,
            }
            builder: $builder:ident,
            analyzer: $analyzer:ident,
            composer: $composer:ident,
            $(io: [$($io_entry:expr),+ $(,)?],)?
            $(validator: $validator:ty,)?
            $(examples: [$($example:expr),+ $(,)?],)?
        }
    ) => {
        $crate::derive_artifact_facets! {
            $vis spec $spec {
                construction: $construction,
                analysis: $analysis,
                composition: $composition,
            }
            builder: $builder,
            analyzer: $analyzer,
            composer: $composer,
        }

        #[doc(hidden)]
        pub mod __subset_registration {
            use super::*;
            use std::sync::{Once, OnceLock};

            pub const SUBSET_DIALECT: $crate::Dialect = $crate::Dialect {
                artifact_kind: $artifact,
                standard: $crate::StandardId($standard),
                subset: $crate::SubsetId($subset),
            };
            pub const KIND: $crate::SubsetKind = $crate::SubsetKind::Owning;
            static REGISTERED: Once = Once::new();
            $(static VALIDATOR_ENTRY: OnceLock<$crate::SubsetValidatorEntry> = OnceLock::new();)?

            $(fn validator_entry() -> &'static $crate::SubsetValidatorEntry {
                VALIDATOR_ENTRY.get_or_init(|| $crate::subset_validator_entry_of::<$validator>())
            })?

            pub fn register() {
                REGISTERED.call_once(|| {
                    let mut entries = vec![$crate::composer_entry_of::<$composer>()];
                    $(entries.extend([$($io_entry),+]);)?
                    $crate::register_composer_entries(&entries);
                    $($crate::register_subset_validator(validator_entry());)?
                });
            }

            $(pub const EXAMPLES: &'static [$crate::ExampleSource] = &[$($example),+];)?

            #[cfg(test)]
            mod conformance {
                use super::*;

                #[test]
                fn subset_macro_owning_dialect_matches_spec() {
                    assert_eq!(SUBSET_DIALECT, <$composition as $crate::ArtifactComposition>::WRITES);
                }
            }
        }

        $vis use __subset_registration::{register as register_subset, KIND, SUBSET_DIALECT};
    };

    (
        $vis:vis derived dialect $artifact:literal / $standard:literal / $subset:literal {
            validator: $validator:ty,
            $(io: [$($io_entry:expr),+ $(,)?],)?
            $(positive: [$($pos_example:expr),+ $(,)?],)?
            $(negative: [$($neg_example:expr),+ $(,)?],)?
        }
    ) => {
        #[doc(hidden)]
        pub mod __subset_registration {
            use super::*;
            use std::sync::{Once, OnceLock};

            pub const SUBSET_DIALECT: $crate::Dialect = $crate::Dialect {
                artifact_kind: $artifact,
                standard: $crate::StandardId($standard),
                subset: $crate::SubsetId($subset),
            };
            pub const KIND: $crate::SubsetKind = $crate::SubsetKind::Derived;
            static REGISTERED: Once = Once::new();
            static VALIDATOR_ENTRY: OnceLock<$crate::SubsetValidatorEntry> = OnceLock::new();

            fn validator_entry() -> &'static $crate::SubsetValidatorEntry {
                VALIDATOR_ENTRY.get_or_init(|| $crate::subset_validator_entry_of::<$validator>())
            }

            pub fn register() {
                REGISTERED.call_once(|| {
                    $crate::register_subset_validator(validator_entry());
                    $( $crate::register_composer_entries(&[$($io_entry),+]); )?
                });
            }

            $(pub const POSITIVE_EXAMPLES: &'static [$crate::ExampleSource] = &[$($pos_example),+];)?
            $(pub const NEGATIVE_EXAMPLES: &'static [$crate::ExampleSource] = &[$($neg_example),+];)?

            #[cfg(test)]
            mod conformance {
                use super::*;

                #[test]
                fn subset_macro_derived_dialect_is_non_any() {
                    assert_ne!(SUBSET_DIALECT.subset, $crate::SubsetId::ANY);
                }

                #[test]
                fn subset_macro_derived_validator_registers() {
                    register();
                    let payload = $crate::IoPayload::Text(String::new());
                    let _ = <$validator as $crate::SubsetValidator>::validate(&payload);
                }
            }
        }

        $vis use __subset_registration::{register as register_subset, KIND, SUBSET_DIALECT};
    };
}

#[cfg(test)]
mod subset_macro_tests {
    use super::*;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};

    struct MacroDerivedValidator;

    impl SubsetValidator for MacroDerivedValidator {
        const DIALECT: Dialect = Dialect { artifact_kind: "s.test.subset-macro", standard: StandardId("1"), subset: SubsetId("derived") };

        fn validate(_payload: &IoPayload) -> Vec<Diagnostic> {
            vec![Diagnostic {
                code: FaultCode::new("test.subset-macro.ok"),
                severity: Severity::Hint,
                span: TextSpan::at(1, 1),
                message: "subset! macro derived validator smoke".into(),
                expected: None,
                scope: dsl::FaultScope::default(),
            }]
        }
    }

    subset! {
        pub derived dialect "s.test.subset-macro" / "1" / "derived" {
            validator: MacroDerivedValidator,
        }
    }

    #[test]
    fn subset_macro_derived_register_is_idempotent() {
        register_subset();
        register_subset();
        let diagnostics = MacroDerivedValidator::validate(&IoPayload::Text(String::new()));
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.0, "test.subset-macro.ok");
    }

    #[test]
    fn subset_macro_derived_kind_and_dialect() {
        assert_eq!(KIND, SubsetKind::Derived);
        assert_eq!(SUBSET_DIALECT.artifact_kind, "s.test.subset-macro");
        assert_eq!(SUBSET_DIALECT.standard.0, "1");
        assert_eq!(SUBSET_DIALECT.subset.0, "derived");
    }
}
