//! 🧵️ Command variants: the command variant macro, the puzzle2d command enum it declares and its binary op implementation. See [`super`].

use super::*;

/// 🎯️ B1: `Puzzle2dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)`/`.action_with(...)` id
/// `create_puzzle2d_app` registers below, plus the framework-injected `setActiveUtility` and the
/// action's original `{...}` JSON payload, unchanged) — `handle` reconstructs the exact
/// `(action, args, window_id)` triple every `🎮️commands/*` arm expects, so each arm's internal
/// `args.get("field")` extraction stays byte-for-byte identical to the pre-B1 implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar
/// those target, so adopting them would silently rewrite this app's wire format. Keep this macro's
/// variant list byte-for-byte stable.
macro_rules! puzzle2d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
        pub enum Puzzle2dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle2dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle` dispatches on.
            pub fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle2dCommand::$Variant { .. } => $id),*
                }
            }

            pub fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle2dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            pub fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle2dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            pub fn try_from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle2dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            fn to_host_value(&self)->Value {
                match self {
                    $(Self::$Variant{window_id,args}=>json!({"action":$id,"windowId":window_id,"args":args})),*
                }
            }
            fn from_host_value(value:&Value)->Option<Self>{
                let object=value.as_object()?;
                if object.iter().any(|(key,_)|!["action","windowId","args"].contains(&key)){return None;}
                let action=value.get("action")?.as_str()?;
                let window_id=match value.get("windowId"){Some(Value::String(value))=>Some(value.clone()),None|Some(Value::Null)=>None,_=>return None};
                let args=value.get("args").cloned().filter(|value|!value.is_null());
                Self::try_from_action(action,args,window_id)
            }
            #[cfg(test)]
            pub fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle2d action id in test: {action}"))
            }
        }
    };
}

puzzle2d_command_variants! {
    AddNode = "addNode",
    SetActiveExample = "setActiveExample",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ForceLayout = "forceLayout",
    FocusSelection = "focusSelection",
    SelectSameKind = "selectSameKind",
    CreateEdge = "createEdge",
    DeleteEdge = "deleteEdge",
    ProximityConnect = "proximityConnect",
    SetProximityRadius = "setProximityRadius",
    SetSelectionFlag = "setSelectionFlag",
    SetSelectionHidden = "setSelectionHidden",
    SetSelectionLocked = "setSelectionLocked",
    PatchInspectorNodes = "patchInspectorNodes",
    RedrawHandles = "redrawHandles",
    Reorganize = "reorganize",
    ApplyBoardEvents = "applyBoardEvents",
    SetFillCount = "setFillCount",
    AcceptSuggestion = "acceptSuggestion",
    SetCamera = "setCamera",
    EngagementInput = "engagementInput",
    EngagementSubmit = "engagementSubmit",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    SetLodModeForPane = "setLodModeForPane",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetGridVisible = "setGridVisible",
    SetSelectableKind = "setSelectableKind",
    SetBrushPlacementContactTolerance = "setBrushPlacementContactTolerance",
    SetBrushPlacementOverlapBudget = "setBrushPlacementOverlapBudget",
    EngagementRepeatLast = "engagementRepeatLast",
    OpenAddNodeDialog = "openAddNodeDialog",
    SetBrushKindWeights = "setBrushKindWeights",
    SetBrushNodeSize = "setBrushNodeSize",
    SetSuggestionOffset = "setSuggestionOffset",
    SetTransformGumballFlag = "setTransformGumballFlag",
    AddTargetRegion = "addTargetRegion",
    DeleteTargetRegion = "deleteTargetRegion",
    RelocateTargetRegion = "relocateTargetRegion",
    SetTargetRegionFlag = "setTargetRegionFlag",
    SetTargetRegionHidden = "setTargetRegionHidden",
    SetTargetRegionLocked = "setTargetRegionLocked",
    SetAreaBrushSize = "setAreaBrushSize",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    TargetBrushSuggestions = "targetBrushSuggestions",
    HoverSuggestion = "hoverSuggestion",
    OpenHandleSuggestions = "openHandleSuggestions",
    CloseHandleSuggestions = "closeHandleSuggestions",
    LodScaleJson = "lodScaleJson",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    ExportSnapshot = "exportSnapshot",
    ImportSnapshot = "importSnapshot",
    OpenImportSnapshot = "openImportSnapshot",
}

impl protocol::OpBinary for Puzzle2dCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &PUZZLE2D_TOOL_JOB_IDS;
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_string(&self.to_host_value()).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text=std::str::from_utf8(bytes).map_err(|error|protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))))?;
        let value=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|protocol::ProtocolError::Pack(store::PackError::from(error.into_value_error())))?;
        Self::from_host_value(&value).ok_or_else(||protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid puzzle2d command"))))
    }
}
