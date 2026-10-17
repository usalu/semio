//! 🪢️ Shell command value bridge of the shell schema: the hand-written `ToValue` and `FromValue` impls of the shell command. See [`super`].

use super::*;

//#region 🌉️ Hand-written ToValue/FromValue bridge for ShellCommand
// ShellCommand cannot use #[derive(ToValue, FromValue)]: three variants (StageActionArg.value,
// StageCommandArg.value, OpenDialog.seed_args) carry a `JsonValue` (= serde_json::Value) field on an
// ENUM VARIANT, and #[value(with = "...")] is deliberately unsupported there (semio-framework-value-derive
// docs, `expand`) — this file hand-writes the identical tag="type"/rename_all="camelCase"/
// rename_all_fields="camelCase" wire shape `#[serde(...)]` produces instead, byte-for-byte. Builds
// DslValue directly (never dispatches back through ToValue/FromValue on ShellCommand itself).
impl dsl_core::ToValue for ShellCommand {
    fn to_value(&self) -> dsl_core::DslValue {
        match self {
            Self::RegisterLoadedPlugin { plugin } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("registerLoadedPlugin".to_string())),
                    ("plugin".to_string(), dsl_core::ToValue::to_value(plugin)),
                ])
            }
            Self::UnregisterLoadedPlugin { plugin_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("unregisterLoadedPlugin".to_string())),
                    ("pluginId".to_string(), dsl_core::ToValue::to_value(plugin_id)),
                ])
            }
            Self::SetPluginStatus { plugin_id, status } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setPluginStatus".to_string())),
                    ("pluginId".to_string(), dsl_core::ToValue::to_value(plugin_id)),
                    ("status".to_string(), dsl_core::ToValue::to_value(status)),
                ])
            }
            Self::SetPluginSupervisorState { plugin_id, state } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setPluginSupervisorState".to_string())),
                    ("pluginId".to_string(), dsl_core::ToValue::to_value(plugin_id)),
                    ("state".to_string(), dsl_core::ToValue::to_value(state)),
                ])
            }
            Self::SetActiveSession { session } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setActiveSession".to_string())),
                    ("session".to_string(), dsl_core::ToValue::to_value(session)),
                ])
            }
            Self::SetSessionError { error } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setSessionError".to_string())),
                    ("error".to_string(), dsl_core::ToValue::to_value(error)),
                ])
            }
            Self::SetAppLabelOverride { app_id, label_key, value } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setAppLabelOverride".to_string())),
                    ("appId".to_string(), dsl_core::ToValue::to_value(app_id)),
                    ("labelKey".to_string(), dsl_core::ToValue::to_value(label_key)),
                    ("value".to_string(), dsl_core::ToValue::to_value(value)),
                ])
            }
            Self::SetActionPaneFolded { window_id, folded } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setActionPaneFolded".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                    ("folded".to_string(), dsl_core::ToValue::to_value(folded)),
                ])
            }
            Self::SetActionPaneExpanded { window_id, action_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setActionPaneExpanded".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                    ("actionId".to_string(), dsl_core::ToValue::to_value(action_id)),
                ])
            }
            Self::StageActionArg { window_id, action_id, arg_id, value } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("stageActionArg".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                    ("actionId".to_string(), dsl_core::ToValue::to_value(action_id)),
                    ("argId".to_string(), dsl_core::ToValue::to_value(arg_id)),
                    ("value".to_string(), dsl_core::DslValue::from(value.clone())),
                ])
            }
            Self::ResetActionArgs { window_id, action_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("resetActionArgs".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                    ("actionId".to_string(), dsl_core::ToValue::to_value(action_id)),
                ])
            }
            Self::SetActiveUtility { window_id, utility_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setActiveUtility".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                    ("utilityId".to_string(), dsl_core::ToValue::to_value(utility_id)),
                ])
            }
            Self::SetActiveTool { tool_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setActiveTool".to_string())),
                    ("toolId".to_string(), dsl_core::ToValue::to_value(tool_id)),
                ])
            }
            Self::SetCommandExpanded { command_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setCommandExpanded".to_string())),
                    ("commandId".to_string(), dsl_core::ToValue::to_value(command_id)),
                ])
            }
            Self::StageCommandArg { command_id, arg_id, value } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("stageCommandArg".to_string())),
                    ("commandId".to_string(), dsl_core::ToValue::to_value(command_id)),
                    ("argId".to_string(), dsl_core::ToValue::to_value(arg_id)),
                    ("value".to_string(), dsl_core::DslValue::from(value.clone())),
                ])
            }
            Self::ResetCommandArgs { command_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("resetCommandArgs".to_string())),
                    ("commandId".to_string(), dsl_core::ToValue::to_value(command_id)),
                ])
            }
            Self::SetPanelVisible { anchor, visible } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setPanelVisible".to_string())),
                    ("anchor".to_string(), dsl_core::ToValue::to_value(anchor)),
                    ("visible".to_string(), dsl_core::ToValue::to_value(visible)),
                ])
            }
            Self::SetPanelSize { anchor, size } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setPanelSize".to_string())),
                    ("anchor".to_string(), dsl_core::ToValue::to_value(anchor)),
                    ("size".to_string(), dsl_core::ToValue::to_value(size)),
                ])
            }
            Self::SetPanelPath { anchor, path } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setPanelPath".to_string())),
                    ("anchor".to_string(), dsl_core::ToValue::to_value(anchor)),
                    ("path".to_string(), dsl_core::ToValue::to_value(path)),
                ])
            }
            Self::SetDockOverride { dock } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setDockOverride".to_string())),
                    ("dock".to_string(), dsl_core::ToValue::to_value(dock)),
                ])
            }
            Self::SetPanelPathMemory { panel_key, path } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setPanelPathMemory".to_string())),
                    ("panelKey".to_string(), dsl_core::ToValue::to_value(panel_key)),
                    ("path".to_string(), dsl_core::ToValue::to_value(path)),
                ])
            }
            Self::SetTreeOpenState { tree_id, open } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setTreeOpenState".to_string())),
                    ("treeId".to_string(), dsl_core::ToValue::to_value(tree_id)),
                    ("open".to_string(), dsl_core::ToValue::to_value(open)),
                ])
            }
            Self::HydrateDockUi { dock } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("hydrateDockUi".to_string())),
                    ("dock".to_string(), dsl_core::ToValue::to_value(dock)),
                ])
            }
            Self::ResetDock => dsl_core::DslValue::Object(vec![("type".to_string(), dsl_core::DslValue::String("resetDock".to_string()))]),
            Self::FocusWindow { window_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("focusWindow".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                ])
            }
            Self::SetShellLayout { layout } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setShellLayout".to_string())),
                    ("layout".to_string(), dsl_core::ToValue::to_value(layout)),
                ])
            }
            Self::SetActiveExample { example_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setActiveExample".to_string())),
                    ("exampleId".to_string(), dsl_core::ToValue::to_value(example_id)),
                ])
            }
            Self::SetMobilePanelPath { path } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setMobilePanelPath".to_string())),
                    ("path".to_string(), dsl_core::ToValue::to_value(path)),
                ])
            }
            Self::SetMobilePanelVisible { visible } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setMobilePanelVisible".to_string())),
                    ("visible".to_string(), dsl_core::ToValue::to_value(visible)),
                ])
            }
            Self::SetExtraWindows { windows } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setExtraWindows".to_string())),
                    ("windows".to_string(), dsl_core::ToValue::to_value(windows)),
                ])
            }
            Self::SetWindowTitle { window_id, title } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setWindowTitle".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                    ("title".to_string(), dsl_core::ToValue::to_value(title)),
                ])
            }
            Self::SetWindowIcon { window_id, icon } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setWindowIcon".to_string())),
                    ("windowId".to_string(), dsl_core::ToValue::to_value(window_id)),
                    ("icon".to_string(), dsl_core::ToValue::to_value(icon)),
                ])
            }
            Self::SetSearchOpen { open } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setSearchOpen".to_string())),
                    ("open".to_string(), dsl_core::ToValue::to_value(open)),
                ])
            }
            Self::SetFindOpen { open } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setFindOpen".to_string())),
                    ("open".to_string(), dsl_core::ToValue::to_value(open)),
                ])
            }
            Self::AutoStartIntroduction { key } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("autoStartIntroduction".to_string())),
                    ("key".to_string(), dsl_core::ToValue::to_value(key)),
                ])
            }
            Self::SetIntroductionStep { step_index } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setIntroductionStep".to_string())),
                    ("stepIndex".to_string(), dsl_core::ToValue::to_value(step_index)),
                ])
            }
            Self::CompleteIntroductionInteraction { interaction_index } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("completeIntroductionInteraction".to_string())),
                    ("interactionIndex".to_string(), dsl_core::ToValue::to_value(interaction_index)),
                ])
            }
            Self::OpenDialog { dialog_id, seed_args } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("openDialog".to_string())),
                    ("dialogId".to_string(), dsl_core::ToValue::to_value(dialog_id)),
                    ("seedArgs".to_string(), match seed_args { Some(__v) => dsl_core::DslValue::from(__v.clone()), None => dsl_core::DslValue::Null }),
                ])
            }
            Self::CloseDialog { dialog_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("closeDialog".to_string())),
                    ("dialogId".to_string(), dsl_core::ToValue::to_value(dialog_id)),
                ])
            }
            Self::ShowTransientNotice { notice } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("showTransientNotice".to_string())),
                    ("notice".to_string(), dsl_core::ToValue::to_value(notice)),
                ])
            }
            Self::DismissTransientNotice => dsl_core::DslValue::Object(vec![("type".to_string(), dsl_core::DslValue::String("dismissTransientNotice".to_string()))]),
            Self::SetOpenWithFocusRole { role } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setOpenWithFocusRole".to_string())),
                    ("role".to_string(), dsl_core::ToValue::to_value(role)),
                ])
            }
            Self::SetActiveTutorial { tutorial_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setActiveTutorial".to_string())),
                    ("tutorialId".to_string(), dsl_core::ToValue::to_value(tutorial_id)),
                ])
            }
            Self::SetUiDriverDraft { draft } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setUiDriverDraft".to_string())),
                    ("draft".to_string(), dsl_core::ToValue::to_value(draft)),
                ])
            }
            Self::SetUiThemeDraft { draft } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setUiThemeDraft".to_string())),
                    ("draft".to_string(), dsl_core::ToValue::to_value(draft)),
                ])
            }
            Self::SetSyncBackboneUri { uri } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setSyncBackboneUri".to_string())),
                    ("uri".to_string(), dsl_core::ToValue::to_value(uri)),
                ])
            }
            Self::SetSyncCardKind { kind } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setSyncCardKind".to_string())),
                    ("kind".to_string(), dsl_core::ToValue::to_value(kind)),
                ])
            }
            Self::SetSyncDraftPath { path } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setSyncDraftPath".to_string())),
                    ("path".to_string(), dsl_core::ToValue::to_value(path)),
                ])
            }
            Self::SetDocumentSyncStatus { document_id, status } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setDocumentSyncStatus".to_string())),
                    ("documentId".to_string(), dsl_core::ToValue::to_value(document_id)),
                    ("status".to_string(), dsl_core::ToValue::to_value(status)),
                ])
            }
            Self::SetDocumentInferencePort { document_id, port } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setDocumentInferencePort".to_string())),
                    ("documentId".to_string(), dsl_core::ToValue::to_value(document_id)),
                    ("port".to_string(), dsl_core::ToValue::to_value(port)),
                ])
            }
            Self::ClearDocumentInferencePort { document_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("clearDocumentInferencePort".to_string())),
                    ("documentId".to_string(), dsl_core::ToValue::to_value(document_id)),
                ])
            }
            Self::SetMergePolicy { policy } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setMergePolicy".to_string())),
                    ("policy".to_string(), dsl_core::ToValue::to_value(policy)),
                ])
            }
            Self::SetConflicts { conflicts } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setConflicts".to_string())),
                    ("conflicts".to_string(), dsl_core::ToValue::to_value(conflicts)),
                ])
            }
            Self::SelectConflict { conflict_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("selectConflict".to_string())),
                    ("conflictId".to_string(), dsl_core::ToValue::to_value(conflict_id)),
                ])
            }
            Self::SetStorageScope { scope } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setStorageScope".to_string())),
                    ("scope".to_string(), dsl_core::ToValue::to_value(scope)),
                ])
            }
            Self::SetOpeningPreference { role, dialect_id } => {
                dsl_core::DslValue::Object(vec![
                    ("type".to_string(), dsl_core::DslValue::String("setOpeningPreference".to_string())),
                    ("role".to_string(), dsl_core::ToValue::to_value(role)),
                    ("dialectId".to_string(), dsl_core::ToValue::to_value(dialect_id)),
                ])
            }
        }
    }
}

impl dsl_core::FromValue for ShellCommand {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(__entries) = value else {
            return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "expected an object for ShellCommand"));
        };
        let __tag = match __entries.iter().find(|(k, _)| k == "type") {
            Some((_, dsl_core::DslValue::String(s))) => s.clone(),
            _ => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing string field `type`")),
        };
        match __tag.as_str() {
                "registerLoadedPlugin" => {
                let plugin = match __entries.iter().find(|(k, _)| k == "plugin") { Some((_, v)) => <LoadedPlugin as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("plugin"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `plugin`")) };
                    Ok(Self::RegisterLoadedPlugin { plugin })
                }
                "unregisterLoadedPlugin" => {
                let plugin_id = match __entries.iter().find(|(k, _)| k == "pluginId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("pluginId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `pluginId`")) };
                    Ok(Self::UnregisterLoadedPlugin { plugin_id })
                }
                "setPluginStatus" => {
                let plugin_id = match __entries.iter().find(|(k, _)| k == "pluginId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("pluginId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `pluginId`")) };
                let status = match __entries.iter().find(|(k, _)| k == "status") { Some((_, v)) => <PluginPanelStatus as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("status"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `status`")) };
                    Ok(Self::SetPluginStatus { plugin_id, status })
                }
                "setPluginSupervisorState" => {
                let plugin_id = match __entries.iter().find(|(k, _)| k == "pluginId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("pluginId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `pluginId`")) };
                let state = match __entries.iter().find(|(k, _)| k == "state") { Some((_, v)) => <PluginSupervisorState as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("state"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `state`")) };
                    Ok(Self::SetPluginSupervisorState { plugin_id, state })
                }
                "setActiveSession" => {
                let session = match __entries.iter().find(|(k, _)| k == "session") { Some((_, v)) => <Option<ActiveSession> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("session"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `session`")) };
                    Ok(Self::SetActiveSession { session })
                }
                "setSessionError" => {
                let error = match __entries.iter().find(|(k, _)| k == "error") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("error"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `error`")) };
                    Ok(Self::SetSessionError { error })
                }
                "setAppLabelOverride" => {
                let app_id = match __entries.iter().find(|(k, _)| k == "appId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("appId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `appId`")) };
                let label_key = match __entries.iter().find(|(k, _)| k == "labelKey") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("labelKey"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `labelKey`")) };
                let value = match __entries.iter().find(|(k, _)| k == "value") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("value"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `value`")) };
                    Ok(Self::SetAppLabelOverride { app_id, label_key, value })
                }
                "setActionPaneFolded" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                let folded = match __entries.iter().find(|(k, _)| k == "folded") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("folded"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `folded`")) };
                    Ok(Self::SetActionPaneFolded { window_id, folded })
                }
                "setActionPaneExpanded" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                let action_id = match __entries.iter().find(|(k, _)| k == "actionId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("actionId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `actionId`")) };
                    Ok(Self::SetActionPaneExpanded { window_id, action_id })
                }
                "stageActionArg" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                let action_id = match __entries.iter().find(|(k, _)| k == "actionId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("actionId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `actionId`")) };
                let arg_id = match __entries.iter().find(|(k, _)| k == "argId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("argId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `argId`")) };
                let value = match __entries.iter().find(|(k, _)| k == "value") { Some((_, v)) => JsonValue::from(v.clone()), None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `value`")) };
                    Ok(Self::StageActionArg { window_id, action_id, arg_id, value })
                }
                "resetActionArgs" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                let action_id = match __entries.iter().find(|(k, _)| k == "actionId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("actionId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `actionId`")) };
                    Ok(Self::ResetActionArgs { window_id, action_id })
                }
                "setActiveUtility" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                let utility_id = match __entries.iter().find(|(k, _)| k == "utilityId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("utilityId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `utilityId`")) };
                    Ok(Self::SetActiveUtility { window_id, utility_id })
                }
                "setActiveTool" => {
                let tool_id = match __entries.iter().find(|(k, _)| k == "toolId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("toolId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `toolId`")) };
                    Ok(Self::SetActiveTool { tool_id })
                }
                "setCommandExpanded" => {
                let command_id = match __entries.iter().find(|(k, _)| k == "commandId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("commandId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `commandId`")) };
                    Ok(Self::SetCommandExpanded { command_id })
                }
                "stageCommandArg" => {
                let command_id = match __entries.iter().find(|(k, _)| k == "commandId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("commandId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `commandId`")) };
                let arg_id = match __entries.iter().find(|(k, _)| k == "argId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("argId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `argId`")) };
                let value = match __entries.iter().find(|(k, _)| k == "value") { Some((_, v)) => JsonValue::from(v.clone()), None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `value`")) };
                    Ok(Self::StageCommandArg { command_id, arg_id, value })
                }
                "resetCommandArgs" => {
                let command_id = match __entries.iter().find(|(k, _)| k == "commandId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("commandId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `commandId`")) };
                    Ok(Self::ResetCommandArgs { command_id })
                }
                "setPanelVisible" => {
                let anchor = match __entries.iter().find(|(k, _)| k == "anchor") { Some((_, v)) => <Anchor as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("anchor"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `anchor`")) };
                let visible = match __entries.iter().find(|(k, _)| k == "visible") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("visible"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `visible`")) };
                    Ok(Self::SetPanelVisible { anchor, visible })
                }
                "setPanelSize" => {
                let anchor = match __entries.iter().find(|(k, _)| k == "anchor") { Some((_, v)) => <Anchor as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("anchor"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `anchor`")) };
                let size = match __entries.iter().find(|(k, _)| k == "size") { Some((_, v)) => <f32 as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("size"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `size`")) };
                    Ok(Self::SetPanelSize { anchor, size })
                }
                "setPanelPath" => {
                let anchor = match __entries.iter().find(|(k, _)| k == "anchor") { Some((_, v)) => <Anchor as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("anchor"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `anchor`")) };
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <Vec<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `path`")) };
                    Ok(Self::SetPanelPath { anchor, path })
                }
                "setDockOverride" => {
                let dock = match __entries.iter().find(|(k, _)| k == "dock") { Some((_, v)) => <Option<LayoutNode> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dock"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `dock`")) };
                    Ok(Self::SetDockOverride { dock })
                }
                "setPanelPathMemory" => {
                let panel_key = match __entries.iter().find(|(k, _)| k == "panelKey") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("panelKey"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `panelKey`")) };
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `path`")) };
                    Ok(Self::SetPanelPathMemory { panel_key, path })
                }
                "setTreeOpenState" => {
                let tree_id = match __entries.iter().find(|(k, _)| k == "treeId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("treeId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `treeId`")) };
                let open = match __entries.iter().find(|(k, _)| k == "open") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("open"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `open`")) };
                    Ok(Self::SetTreeOpenState { tree_id, open })
                }
                "hydrateDockUi" => {
                let dock = match __entries.iter().find(|(k, _)| k == "dock") { Some((_, v)) => <Option<DockUiState> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dock"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `dock`")) };
                    Ok(Self::HydrateDockUi { dock })
                }
                "resetDock" => Ok(Self::ResetDock),
                "focusWindow" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                    Ok(Self::FocusWindow { window_id })
                }
                "setShellLayout" => {
                let layout = match __entries.iter().find(|(k, _)| k == "layout") { Some((_, v)) => <Option<LayoutNode> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("layout"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `layout`")) };
                    Ok(Self::SetShellLayout { layout })
                }
                "setActiveExample" => {
                let example_id = match __entries.iter().find(|(k, _)| k == "exampleId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("exampleId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `exampleId`")) };
                    Ok(Self::SetActiveExample { example_id })
                }
                "setMobilePanelPath" => {
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <Vec<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `path`")) };
                    Ok(Self::SetMobilePanelPath { path })
                }
                "setMobilePanelVisible" => {
                let visible = match __entries.iter().find(|(k, _)| k == "visible") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("visible"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `visible`")) };
                    Ok(Self::SetMobilePanelVisible { visible })
                }
                "setExtraWindows" => {
                let windows = match __entries.iter().find(|(k, _)| k == "windows") { Some((_, v)) => <Vec<ExtraWindowInstance> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windows"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windows`")) };
                    Ok(Self::SetExtraWindows { windows })
                }
                "setWindowTitle" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                let title = match __entries.iter().find(|(k, _)| k == "title") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("title"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `title`")) };
                    Ok(Self::SetWindowTitle { window_id, title })
                }
                "setWindowIcon" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `windowId`")) };
                let icon = match __entries.iter().find(|(k, _)| k == "icon") { Some((_, v)) => <IconName as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("icon"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `icon`")) };
                    Ok(Self::SetWindowIcon { window_id, icon })
                }
                "setSearchOpen" => {
                let open = match __entries.iter().find(|(k, _)| k == "open") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("open"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `open`")) };
                    Ok(Self::SetSearchOpen { open })
                }
                "setFindOpen" => {
                let open = match __entries.iter().find(|(k, _)| k == "open") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("open"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `open`")) };
                    Ok(Self::SetFindOpen { open })
                }
                "autoStartIntroduction" => {
                let key = match __entries.iter().find(|(k, _)| k == "key") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("key"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `key`")) };
                    Ok(Self::AutoStartIntroduction { key })
                }
                "setIntroductionStep" => {
                let step_index = match __entries.iter().find(|(k, _)| k == "stepIndex") { Some((_, v)) => <Option<u32> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("stepIndex"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `stepIndex`")) };
                    Ok(Self::SetIntroductionStep { step_index })
                }
                "completeIntroductionInteraction" => {
                let interaction_index = match __entries.iter().find(|(k, _)| k == "interactionIndex") { Some((_, v)) => <u32 as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("interactionIndex"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `interactionIndex`")) };
                    Ok(Self::CompleteIntroductionInteraction { interaction_index })
                }
                "openDialog" => {
                let dialog_id = match __entries.iter().find(|(k, _)| k == "dialogId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dialogId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `dialogId`")) };
                let seed_args = match __entries.iter().find(|(k, _)| k == "seedArgs") { Some((_, dsl_core::DslValue::Null)) | None => None, Some((_, v)) => Some(JsonValue::from(v.clone())) };
                    Ok(Self::OpenDialog { dialog_id, seed_args })
                }
                "closeDialog" => {
                let dialog_id = match __entries.iter().find(|(k, _)| k == "dialogId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dialogId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `dialogId`")) };
                    Ok(Self::CloseDialog { dialog_id })
                }
                "showTransientNotice" => {
                let notice = match __entries.iter().find(|(k, _)| k == "notice") { Some((_, v)) => <TransientNotice as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("notice"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `notice`")) };
                    Ok(Self::ShowTransientNotice { notice })
                }
                "dismissTransientNotice" => Ok(Self::DismissTransientNotice),
                "setOpenWithFocusRole" => {
                let role = match __entries.iter().find(|(k, _)| k == "role") { Some((_, v)) => <Option<AppRole> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("role"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `role`")) };
                    Ok(Self::SetOpenWithFocusRole { role })
                }
                "setActiveTutorial" => {
                let tutorial_id = match __entries.iter().find(|(k, _)| k == "tutorialId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("tutorialId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `tutorialId`")) };
                    Ok(Self::SetActiveTutorial { tutorial_id })
                }
                "setUiDriverDraft" => {
                let draft = match __entries.iter().find(|(k, _)| k == "draft") { Some((_, v)) => <Option<UiDriver> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("draft"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `draft`")) };
                    Ok(Self::SetUiDriverDraft { draft })
                }
                "setUiThemeDraft" => {
                let draft = match __entries.iter().find(|(k, _)| k == "draft") { Some((_, v)) => <Option<UiTheme> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("draft"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `draft`")) };
                    Ok(Self::SetUiThemeDraft { draft })
                }
                "setSyncBackboneUri" => {
                let uri = match __entries.iter().find(|(k, _)| k == "uri") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("uri"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `uri`")) };
                    Ok(Self::SetSyncBackboneUri { uri })
                }
                "setSyncCardKind" => {
                let kind = match __entries.iter().find(|(k, _)| k == "kind") { Some((_, v)) => <Option<SyncCardKind> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("kind"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `kind`")) };
                    Ok(Self::SetSyncCardKind { kind })
                }
                "setSyncDraftPath" => {
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `path`")) };
                    Ok(Self::SetSyncDraftPath { path })
                }
                "setDocumentSyncStatus" => {
                let document_id = match __entries.iter().find(|(k, _)| k == "documentId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("documentId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `documentId`")) };
                let status = match __entries.iter().find(|(k, _)| k == "status") { Some((_, v)) => <ArtifactSyncStatus as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("status"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `status`")) };
                    Ok(Self::SetDocumentSyncStatus { document_id, status })
                }
                "setDocumentInferencePort" => {
                let document_id = match __entries.iter().find(|(k, _)| k == "documentId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("documentId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `documentId`")) };
                let port = match __entries.iter().find(|(k, _)| k == "port") { Some((_, v)) => <InferencePortStatus as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("port"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `port`")) };
                    Ok(Self::SetDocumentInferencePort { document_id, port })
                }
                "clearDocumentInferencePort" => {
                let document_id = match __entries.iter().find(|(k, _)| k == "documentId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("documentId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `documentId`")) };
                    Ok(Self::ClearDocumentInferencePort { document_id })
                }
                "setMergePolicy" => {
                let policy = match __entries.iter().find(|(k, _)| k == "policy") { Some((_, v)) => <MergePolicy as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("policy"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `policy`")) };
                    Ok(Self::SetMergePolicy { policy })
                }
                "setConflicts" => {
                let conflicts = match __entries.iter().find(|(k, _)| k == "conflicts") { Some((_, v)) => <Vec<Conflict> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("conflicts"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `conflicts`")) };
                    Ok(Self::SetConflicts { conflicts })
                }
                "selectConflict" => {
                let conflict_id = match __entries.iter().find(|(k, _)| k == "conflictId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("conflictId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `conflictId`")) };
                    Ok(Self::SelectConflict { conflict_id })
                }
                "setStorageScope" => {
                let scope = match __entries.iter().find(|(k, _)| k == "scope") { Some((_, v)) => <ShellScope as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("scope"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `scope`")) };
                    Ok(Self::SetStorageScope { scope })
                }
                "setOpeningPreference" => {
                let role = match __entries.iter().find(|(k, _)| k == "role") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("role"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `role`")) };
                let dialect_id = match __entries.iter().find(|(k, _)| k == "dialectId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dialectId"))?, None => return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "missing field `dialectId`")) };
                    Ok(Self::SetOpeningPreference { role, dialect_id })
                }
            other => Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, format!("unknown ShellCommand type `{other}`"))),
        }
    }
}
//#endregion
