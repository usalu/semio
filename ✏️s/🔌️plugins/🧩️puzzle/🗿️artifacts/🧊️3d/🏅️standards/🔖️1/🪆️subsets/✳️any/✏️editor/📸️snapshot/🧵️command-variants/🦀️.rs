//! 🧵️ Command variants: the command variant macro, the puzzle3d command enum it declares and its binary op implementation. See [`super`].

use super::*;

/// 🎯️ B1: `Puzzle3dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)` id `create_puzzle3d_app` registers
/// below). Each variant carries `window_id` (was host-pushed `view_state.window_id`) plus `args` (the
/// action's original `{...}` JSON payload, unchanged) — `handle` reconstructs the exact
/// `(action, args, window_id)` triple every `🎮️commands/*` arm expects, so each arm's internal
/// `args.get("field")` extraction stays byte-for-byte identical to the pre-B1 implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar those
/// target, so adopting them would silently rewrite this app's wire format. Keep this macro's variant
/// list, its order and its action-id literals byte-for-byte stable.
macro_rules! puzzle3d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
        pub enum Puzzle3dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle3dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle` dispatches on.
            pub fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle3dCommand::$Variant { .. } => $id),*
                }
            }

            pub fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle3dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            pub fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle3dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            /// 🎯️ Reverse of `action_id()` — builds the typed command used by both the host's
            /// transitional `{action,args}` bridge and the test context dispatch helper.
            pub fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle3dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            /// 🪶️ Hand-written JSON bridge (see this macro's own doc comment on why `OpBinary` here
            /// is a plain JSON-bytes bridge, not a derive): reproduces serde's default externally
            /// tagged struct-variant shape (`{"VariantName": {"window_id": ..., "args": ...}}`) so
            /// `encode_op`/`decode_op` stay byte-for-byte compatible with the pre-migration wire.
            fn to_json(&self) -> Value {
                match self {
                    $(Puzzle3dCommand::$Variant { window_id, args } => object([(
                        stringify!($Variant).to_string(),
                        object([("window_id".to_string(), Value::from(window_id.clone())), ("args".to_string(), args.clone().unwrap_or(Value::Null))]),
                    )])),*
                }
            }

            fn from_json(value: &Value) -> Option<Self> {
                let entries = value.as_object()?;
                if entries.len() != 1 {
                    return None;
                }
                let (tag, payload) = entries.iter().next()?;
                let window_id = payload.get("window_id").and_then(Value::as_str).map(str::to_string);
                let args = payload.get("args").cloned().filter(|value| !value.is_null());
                match tag {
                    $(stringify!($Variant) => Some(Puzzle3dCommand::$Variant { window_id, args }),)*
                    _ => None,
                }
            }
        }
    };
}

puzzle3d_command_variants! {
    OpenAddObjectDialog = "openAddObjectDialog",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    SetActiveExample = "setActiveExample",
    SetActiveTool = SET_ACTIVE_TOOL_ACTION_ID,
    AddObjectKind = "addObjectKind",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ExportSnapshot = "exportSnapshot",
    ImportSnapshot = "importSnapshot",
    OpenImportSnapshot = "openImportSnapshot",
    SelectSameKindSelection = "selectSameKindSelection",
    SetCamera = "setCamera",
    SetProjection = "setProjection",
    SetProjectionParam = "setProjectionParam",
    SetVortexShow = "setVortexShow",
    SetVortexDirection = "setVortexDirection",
    RelocateTargetVolume = "relocateTargetVolume",
    WorldRelocate = "worldRelocate",
    ToggleSun = "toggleSun",
    SetSunAzimuth = "setSunAzimuth",
    SetSunElevation = "setSunElevation",
    SetSunIntensity = "setSunIntensity",
    SetLodAutomatic = "setLodAutomatic",
    SetLodDepthVariable = "setLodDepthVariable",
    SetGridVisible = "setGridVisible",
    SetLodManual = "setLodManual",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridSpacing = "setGridSpacing",
    SetProximityRadius = "setProximityRadius",
    SetChunkSize = "setChunkSize",
    SetSelectableKind = "setSelectableKind",
    SetSelectionFlag = "setSelectionFlag",
    SetSelectionHidden = "setSelectionHidden",
    SetSelectionLocked = "setSelectionLocked",
    PatchInspector = "patchInspector",
    FocusSelection = "focusSelection",
    EngagementInput = "engagementInput",
    EngagementSubmit = "engagementSubmit",
    EngagementRepeatLast = "engagementRepeatLast",
    EngagementAbort = "engagementAbort",
    CreateAttraction = "createAttraction",
    DeleteAttraction = "deleteAttraction",
    SetTransformGumballFlag = "setTransformGumballFlag",
    SetVoxelDims = "setVoxelDims",
    AddTargetVolume = "addTargetVolume",
    DeleteTargetVolume = "deleteTargetVolume",
    SetTargetVolumeFlag = "setTargetVolumeFlag",
    EngagementControlSelect = "engagementControlSelect",
    AddBrushObject = "addBrushObject",
    SetFillCount = "setFillCount",
    SetBrushPlacementContactTolerance = "setBrushPlacementContactTolerance",
    SetObjectKindWeight = "setObjectKindWeight",
    SetVortexKindWeight = "setVortexKindWeight",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    OpenVortexSuggestions = "openVortexSuggestions",
    CloseVortexSuggestions = "closeVortexSuggestions",
    HoverSuggestion = "hoverSuggestion",
    AcceptSuggestion = "acceptSuggestion",
    TargetBrushSuggestions = "targetBrushSuggestions",
    RegisterBrushMesh = "registerBrushMesh",
    WorldPointerDown = "worldPointerDown",
    // 🗣️ B1: locale/terminology used to be host-pushed `ViewState` fields with no app-level action of
    // their own; now that `ViewState` is gone from the app-facing surface, they need a real Command.
}

impl protocol::OpBinary for Puzzle3dCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &[
        "openAddObjectDialog",
        "translateSelection",
        "rotateSelection",
        "scaleSelection",
        "setActiveExample",
        "addObjectKind",
        "deleteSelection",
        "duplicateSelection",
        "exportSnapshot",
        "importSnapshot",
        "openImportSnapshot",
        "selectSameKindSelection",
        "setCamera",
        "setProjection",
        "setProjectionParam",
        "setVortexShow",
        "setVortexDirection",
        "relocateTargetVolume",
        "worldRelocate",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setLodAutomatic",
        "setLodDepthVariable",
        "setGridVisible",
        "setLodManual",
        "setGridSnapEnabled",
        "setGridSpacing",
        "setProximityRadius",
        "setChunkSize",
        "setSelectableKind",
        "setSelectionFlag",
        "setSelectionHidden",
        "setSelectionLocked",
        "patchInspector",
        "focusSelection",
        "engagementInput",
        "engagementSubmit",
        "engagementRepeatLast",
        "engagementAbort",
        "createAttraction",
        "deleteAttraction",
        "setTransformGumballFlag",
        "setVoxelDims",
        "addTargetVolume",
        "deleteTargetVolume",
        "setTargetVolumeFlag",
        "engagementControlSelect",
        "addBrushObject",
        "setFillCount",
        "setBrushPlacementContactTolerance",
        "setObjectKindWeight",
        "setVortexKindWeight",
        "cycleBrushCandidate",
        "cycleBrushCandidateBack",
        "openVortexSuggestions",
        "closeVortexSuggestions",
        "hoverSuggestion",
        "acceptSuggestion",
        "targetBrushSuggestions",
        "registerBrushMesh",
        "worldPointerDown",
        // 🧭️ The two framework-injected host-configuration verbs. They are `interactiveJob: "migrated"`
        // in the manifest and `host_configuration_mutation` resolves each to one Config mutation, so
        // they need a generated tool id here for `validate_tool_job_rows` to admit
        // `Puzzle3dHostConfigurationProofs`' generic bounded proofs — without which `dispatch_action`
        // fails closed with `interactive-job.missing-factory` before the hook is ever consulted.
        "setActiveTool",
        "setActiveUtility",
    ];

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(to_string(&self.to_json()).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        let value = parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error.into_value_error())))?;
        Self::from_json(&value).ok_or_else(|| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unrecognized Puzzle3dCommand tag"))))
    }
}
