//! 🏗️ Try build definition of the app builder: the validating assembly of the app definition and the panicking build wrapper. See [`super`].

use super::*;

impl AppBuilder {
    /// 🧷️ Keybinding-vs-action-registry consistency is only enforced for apps that declare
    /// actions via `.mutation()`/`.view_action()`/`.shell_action()` — apps with an empty action
    /// registry keybind directly against controller actions instead, so there is nothing to check.
    ///
    /// 🕰️ Unlike Document/Catalogue/Inspection/Parameters (per-app content, opt-in via
    /// `.panel_tab(...)`), the history panel's content is framework-generic (`HistoryView`), so
    /// every app gets it unconditionally — unless it already declared the reserved id itself.
    ///
    /// ⏯️ The framework ToolRun panel (contract §2.6) reaches the shell only as a panel tab: every app that
    /// declares a tool run gets it, so start/pause/step/abort/finalize, the progress bar and the step log are
    /// reachable — unless the app declared the reserved id itself.
    ///
    /// 🕹️ W3b: `transitive` (hover or selection) means "expand to descendant closure" — over
    ///
    /// `HierarchyProvider::Flat` there is no descendant relation to expand along at all, so a
    /// transitive `Flat` domain could only ever silently degrade to non-transitive behavior;
    /// reject it at build time instead.
    pub fn try_build_definition(mut self) -> Result<AppDefinition, PluginAssemblyError> {
        if !(!self.document.is_empty() && self.document.iter().all(|segment| !segment.trim().is_empty())) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} document must contain non-empty segments", self.id)));
        }
        for (terminology_id, document) in &self.terminology_breadcrumbs {
            if !(self.terminologies.iter().any(|id| id == terminology_id)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} declares terminology_document for undeclared terminology {}", self.id, terminology_id)));
            }
            if !(!document.is_empty() && document.iter().all(|segment| !segment.trim().is_empty())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} terminology_document for {} must contain non-empty segments", self.id, terminology_id)));
            }
        }
        if !(!self.window_kinds.is_empty()) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} must declare at least one window kind", self.id)));
        }
        if !(!self.modes.is_empty()) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} must declare at least one mode", self.id)));
        }
        let mut window_kind_ids = HashSet::new();
        for window in &self.window_kinds {
            if !(!window.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind id must be non-empty", self.id)));
            }
            if !(!window.body_key.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} body_key must be non-empty", self.id, window.id)));
            }
            if !(window_kind_ids.insert(window.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate window kind id {}", self.id, window.id)));
            }
        }
        let mut panel_tab_ids = HashSet::new();
        for tab in &self.panel_tabs {
            validate_panel_tab_spec(&self.id, tab, &mut panel_tab_ids);
        }
        if panel_tab_ids.insert(semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()) {
            self.panel_tabs.push(PanelTabSpec::framework(
                PanelTabKind::App(semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()),
                LocalizedLabel::native(semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_LABEL, "Verlauf"),
                PanelGroup::Settings,
                Some(FRAMEWORK_HISTORY_BODY_KEY.to_string()),
                Vec::new(),
            ));
        }
        let declares_tool_run = self.tools.iter().any(|tool| tool.run.is_some()) || self.utilities.iter().any(|utility| utility.run.is_some());
        if declares_tool_run && panel_tab_ids.insert(semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_TOOL_RUN_ID.to_string()) {
            self.panel_tabs.push(PanelTabSpec::framework(
                PanelTabKind::App(semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_TOOL_RUN_ID.to_string()),
                LocalizedLabel::native(semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL, "Werkzeugläufe"),
                PanelGroup::Details,
                Some(FRAMEWORK_TOOL_RUN_BODY_KEY.to_string()),
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
            if !(window_kind_ids.contains(&window_kind_id)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} layout references undeclared window kind {}", self.id, window_kind_id)));
            }
        }
        let default_mode_id = self.default_mode_id.clone().unwrap_or_else(|| self.modes[0].id.clone());
        if !(self.modes.iter().any(|mode| mode.id == default_mode_id)) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} default_mode_id {} does not reference a declared mode", self.id, default_mode_id)));
        }
        let mut declared_action_ids = HashSet::new();
        for action in &self.actions {
            if !(declared_action_ids.insert(action.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate action id {}", self.id, action.id)));
            }
            validate_arg_defs(&self.id, &format!("action {}", action.id), &action.args);
        }
        let reserved = builder_reserved_contributions();
        for action in self.actions.iter().chain(self.window_kinds.iter().flat_map(|window| window.actions.iter())) {
            if let Some(expected) = semio_framework::reserved_audience_violation(&reserved, action) {
                return Err(PluginAssemblyError::new(
                    "app-definition.invalid",
                    format!("app {} declares the reserved verb {} with audience {:?}; its reserved contribution fixes {:?}", self.id, action.id, semio_framework::resolve_audience(action), expected),
                ));
            }
        }
        let mut declared_utility_ids = HashSet::new();
        for utility in &self.utilities {
            if !(!utility.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} utility id must be non-empty", self.id)));
            }
            if !(declared_utility_ids.insert(utility.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate utility id {}", self.id, utility.id)));
            }
        }
        let mut declared_interaction_ids = HashSet::new();
        for interaction in &self.interactions {
            if !(!interaction.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction id must be non-empty", self.id)));
            }
            if !(declared_interaction_ids.insert(interaction.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate interaction id {}", self.id, interaction.id)));
            }
            if !(!interaction.granularities.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one granularity", self.id, interaction.id)));
            }
            if !(!interaction.selection.modes.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one selection mode", self.id, interaction.id)));
            }
            if !(!interaction.selection.methods.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one selection method", self.id, interaction.id)));
            }
            if !(!interaction.selection.merges.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one merge mode", self.id, interaction.id)));
            }
            if !(!interaction.hover.transitive || !matches!(interaction.hierarchy, semio_framework::HierarchyProvider::Flat)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} declares hover.transitive with HierarchyProvider::Flat (transitive requires a real hierarchy)", self.id, interaction.id)));
            }
            if !(!interaction.selection.transitive || !matches!(interaction.hierarchy, semio_framework::HierarchyProvider::Flat)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} declares selection.transitive with HierarchyProvider::Flat (transitive requires a real hierarchy)", self.id, interaction.id)));
            }
        }
        let entity_kinds = self.interactions.iter().flat_map(|interaction| interaction.granularities.iter().map(|granularity| semio_framework::interaction_entity_kind(&interaction.id, &granularity.id))).collect::<HashSet<_>>();
        for action in self.actions.iter().chain(self.window_kinds.iter().flat_map(|window| window.actions.iter())) {
            if let Some(entity_kind) = action.args.iter().flat_map(|arg| arg.schema.entity_kinds()).find(|entity_kind| !entity_kinds.contains(*entity_kind)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} action {} names entity kind {entity_kind}, which no granularity of a declared interaction answers", self.id, action.id)));
            }
        }
        let mut declared_tool_ids = HashSet::new();
        for tool in &self.tools {
            if !(!tool.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tool id must be non-empty", self.id)));
            }
            if !(declared_tool_ids.insert(tool.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate tool id {}", self.id, tool.id)));
            }
        }
        let mut declared_command_ids = HashSet::new();
        for command in &self.commands {
            if !(!command.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} command id must be non-empty", self.id)));
            }
            if !(declared_command_ids.insert(command.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate app command id {}", self.id, command.id)));
            }
            validate_arg_defs(&self.id, &format!("command {}", command.id), &command.args);
        }
        let (dialect, role) = semio_framework::parse_surface_app_id(&self.id).map_err(|error| PluginAssemblyError::new("app-definition.invalid", format!("app id {} must be a canonical surface id: {error}", self.id)))?;
        let app_declared_actions = !self.actions.is_empty();
        let mut actions = self.actions;
        for framework_action in history_action_definitions().into_iter().chain(semio_framework::reserved_app_actions(&reserved)).chain(clipboard_action_definitions()).filter(|action| role != AppRole::Viewer || !viewer_rejects_action(&action.id)) {
            if declared_action_ids.insert(framework_action.id.clone()) {
                actions.push(framework_action);
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
        let explicitly_owned_action_ids: HashSet<String> = self.window_kinds.iter().flat_map(|window| window.actions.iter().map(|action| action.id.clone()).chain(window.action_refs.iter().map(|action| action.as_str().to_string()))).collect();
        for window in &mut self.window_kinds {
            for action_ref in &window.action_refs {
                let action = actions
                    .iter()
                    .find(|action| action.id == action_ref.as_str())
                    .ok_or_else(|| PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} references undeclared action {}", self.id, window.id, action_ref.as_str())))?;
                window.actions.push(action.clone());
            }
            let mut window_action_ids = HashSet::new();
            for action in &window.actions {
                if !(!action.id.trim().is_empty()) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} action id must be non-empty", self.id, window.id)));
                }
                if !(window_action_ids.insert(action.id.clone())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} duplicate action id {}", self.id, window.id, action.id)));
                }
                declared_action_ids.insert(action.id.clone());
                validate_arg_defs(&self.id, &format!("window kind {} action {}", window.id, action.id), &action.args);
            }
        }
        let app_roster: Vec<ActionDefinition> = actions.iter().filter(|action| !explicitly_owned_action_ids.contains(&action.id)).cloned().collect();
        let mut bound_keys: HashSet<String> = self.keybindings.iter().map(|binding| binding.keys.clone()).collect();
        let mut keybindings: Vec<Keybinding> =
            self.keybindings.into_iter().map(|binding| Keybinding { keys: binding.keys, action: KeybindingActionDescriptor { controller_id: binding.controller_id, action: binding.action, args: None } }).collect();
        for history_action in actions.iter().filter(|action| matches!(action.kind, ActionKind::History | ActionKind::Clipboard)) {
            if let Some(keys) = &history_action.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding { keys: keys.clone(), action: KeybindingActionDescriptor { controller_id: self.controller_id.clone(), action: history_action.id.clone(), args: None } });
                }
            }
        }
        for utility in &self.utilities {
            if let Some(keys) = &utility.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding {
                        keys: keys.clone(),
                        action: KeybindingActionDescriptor {
                            controller_id: self.controller_id.clone(),
                            action: SET_ACTIVE_UTILITY_ACTION_ID.to_string(),
                            args: Some(DslValue::Object(vec![("utilityId".into(), DslValue::String(utility.id.clone()))])),
                        },
                    });
                }
            }
        }
        for tool in &self.tools {
            if let Some(keys) = &tool.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding {
                        keys: keys.clone(),
                        action: KeybindingActionDescriptor { controller_id: self.controller_id.clone(), action: SET_ACTIVE_TOOL_ACTION_ID.to_string(), args: Some(DslValue::Object(vec![("toolId".into(), DslValue::String(tool.id.clone()))])) },
                    });
                }
            }
        }
        if app_declared_actions {
            for binding in &keybindings {
                if !(declared_action_ids.contains(&binding.action.action)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} keybinding {} references undeclared action {}", self.id, binding.keys, binding.action.action)));
                }
            }
        }
        for window in &self.window_kinds {
            for utility_ref in &window.utilities {
                let utility_ref_str = utility_ref.as_str();
                if !(declared_utility_ids.contains(utility_ref_str)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} references undeclared utility {}", self.id, window.id, utility_ref_str)));
                }
            }
            if let Some(initial) = &window.initial_utility_id {
                if !window.utilities.contains(initial) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} initial utility {} is outside its accepted utility roster", self.id, window.id, initial.as_str())));
                }
            }
            for interaction_ref in &window.interactions {
                let interaction_ref_str = interaction_ref.as_str();
                if !(declared_interaction_ids.contains(interaction_ref_str)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} references undeclared interaction {}", self.id, window.id, interaction_ref_str)));
                }
            }
        }
        for mode in &self.modes {
            let mut mode_command_ids = HashSet::new();
            for command in &mode.commands {
                if !(!command.id.trim().is_empty()) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} mode {} command id must be non-empty", self.id, mode.id)));
                }
                if !(mode_command_ids.insert(command.id.clone())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} mode {} duplicate command id {}", self.id, mode.id, command.id)));
                }
                declared_command_ids.insert(command.id.clone());
                validate_arg_defs(&self.id, &format!("mode {} command {}", mode.id, command.id), &command.args);
            }
            for tool_ref in &mode.tools {
                if !(declared_tool_ids.contains(tool_ref.as_str())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} mode {} references undeclared tool {}", self.id, mode.id, tool_ref.as_str())));
                }
            }
        }
        let mode_referenced_tools: HashSet<&str> = self.modes.iter().flat_map(|mode| mode.tools.iter().map(|tool_ref| tool_ref.as_str())).collect();
        for tool in &self.tools {
            if !(mode_referenced_tools.contains(tool.id.as_str())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tool {} is not referenced by any mode", self.id, tool.id)));
            }
        }
        if let Some(introduction) = &self.introduction {
            if !(!introduction.steps.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction must declare at least one step", self.id)));
            }
            let mut step_ids = HashSet::new();
            for step in &introduction.steps {
                if !(!step.id.trim().is_empty()) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step id must be non-empty", self.id)));
                }
                if !(step_ids.insert(step.id.clone())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate introduction step id {}", self.id, step.id)));
                }
                let step_context = format!("introduction step {}", step.id);
                if let Some(id) = &step.introduce {
                    validate_referenced_element_id(&self.id, &step_context, "introduce", id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                }
                for id in &step.show {
                    validate_referenced_element_id(&self.id, &step_context, "show", id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                }
                for interaction in &step.interactions {
                    if !(!interaction.label.trim().is_empty()) {
                        return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction has an empty label", self.id, step.id)));
                    }
                    match &interaction.on {
                        IntroductionInteractionKind::Action(action_ref) => {
                            let action_ref_str = action_ref.as_str();
                            if !(declared_action_ids.contains(action_ref_str)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared action {}", self.id, step.id, action_ref_str)));
                            }
                        }
                        IntroductionInteractionKind::Utility(utility_ref) => {
                            let utility_ref_str = utility_ref.as_str();
                            if !(declared_utility_ids.contains(utility_ref_str)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared utility {}", self.id, step.id, utility_ref_str)));
                            }
                        }
                        IntroductionInteractionKind::Tool(tool_ref) => {
                            let tool_ref_str = tool_ref.as_str();
                            if !(declared_tool_ids.contains(tool_ref_str)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared tool {}", self.id, step.id, tool_ref_str)));
                            }
                        }
                        IntroductionInteractionKind::Panel(panel_tab_id) => validate_referenced_element_id(&self.id, &step_context, "interaction.panel", panel_tab_id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids),
                        IntroductionInteractionKind::Expand(tree_id) => validate_referenced_element_id(&self.id, &step_context, "interaction.expand", tree_id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids),
                        IntroductionInteractionKind::Pan(window_kind_id) | IntroductionInteractionKind::Zoom(window_kind_id) | IntroductionInteractionKind::Orbit(window_kind_id) => {
                            if !(window_kind_ids.contains(window_kind_id)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared window kind {}", self.id, step.id, window_kind_id)));
                            }
                        }
                    }
                }
            }
        }
        let mut tutorial_ids = HashSet::new();
        for tutorial in &self.tutorials {
            if !(!tutorial.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tutorial id must be non-empty", self.id)));
            }
            if !(tutorial_ids.insert(tutorial.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate tutorial id {}", self.id, tutorial.id)));
            }
            if let Err(reason) = semio_framework::onboarding::validate_tutorial(tutorial) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tutorial {} failed validation: {}", self.id, tutorial.id, reason)));
            }
            let owner = format!("tutorial {}", tutorial.id);
            let mut ui_changes: Vec<&semio_framework::onboarding::TutorialUiChange> = Vec::new();
            for keyframe in &tutorial.tracks.ui {
                if let semio_framework::onboarding::TutorialUiSample::Delta { changes } = &keyframe.sample {
                    ui_changes.extend(changes.iter());
                }
            }
            for change in ui_changes {
                match change {
                    semio_framework::onboarding::TutorialUiChange::ActiveUtility { utility_id: Some(utility_id), .. } if !declared_utility_ids.contains(utility_id) => {
                        return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} references undeclared utility {}", self.id, owner, utility_id)));
                    }
                    semio_framework::onboarding::TutorialUiChange::ActiveTool { id: Some(tool_id) } if !declared_tool_ids.contains(tool_id) => {
                        return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} references undeclared tool {}", self.id, owner, tool_id)));
                    }
                    _ => {}
                }
            }
            for utility_id in tutorial.base.ui.active_utility_by_window_id.values() {
                if !(declared_utility_ids.contains(utility_id)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} base.ui references undeclared utility {}", self.id, owner, utility_id)));
                }
            }
            if let Some(tool_id) = &tutorial.base.ui.active_tool_id {
                if !(declared_tool_ids.contains(tool_id)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} base.ui references undeclared tool {}", self.id, owner, tool_id)));
                }
            }
            for event in &tutorial.tracks.events {
                match &event.kind {
                    semio_framework::onboarding::TutorialEventKind::Action { action, .. } => {
                        if !(declared_action_ids.contains(action)) {
                            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} event references undeclared action {}", self.id, owner, action)));
                        }
                    }
                    semio_framework::onboarding::TutorialEventKind::Command { command, .. } => {
                        if !(declared_command_ids.contains(command)) {
                            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} event references undeclared command {}", self.id, owner, command)));
                        }
                    }
                    semio_framework::onboarding::TutorialEventKind::Key { .. } => {}
                }
            }
            for gesture_cue in &tutorial.tracks.gestures {
                for point in introduction_gesture_points(&gesture_cue.gesture) {
                    if let semio_framework::onboarding::IntroductionPoint::Element { id, .. } = point {
                        validate_referenced_element_id(&self.id, &owner, "gesture", id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                    }
                }
            }
        }
        if role != AppRole::Viewer {
            for dialog in semio_framework::reserved_editor_dialogs(&reserved) {
                if !self.dialogs.iter().any(|declared| declared.id == dialog.id) {
                    self.dialogs.push(dialog);
                }
            }
        }
        let mut dialog_ids = HashSet::new();
        for dialog in &self.dialogs {
            if !(!dialog.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog id must be non-empty", self.id)));
            }
            if !(dialog_ids.insert(dialog.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate dialog id {}", self.id, dialog.id)));
            }
            if !(declared_action_ids.contains(dialog.submit_action.as_str())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} submit_action references undeclared action {}", self.id, dialog.id, dialog.submit_action.as_str())));
            }
            if let Some(cancel_action) = &dialog.cancel_action {
                if !(declared_action_ids.contains(cancel_action.as_str())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} cancel_action references undeclared action {}", self.id, dialog.id, cancel_action.as_str())));
                }
            }
            if let Err(reason) = dialog.validate_choices() {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} {reason}", self.id, dialog.id)));
            }
            if let Some(choice) = dialog.choices.iter().find(|choice| !declared_action_ids.contains(choice.action.as_str())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} choice {} references undeclared action {}", self.id, dialog.id, choice.id, choice.action.as_str())));
            }
            validate_arg_defs(&self.id, &format!("dialog {}", dialog.id), &dialog.args);
        }
        let mut media_port_ids = HashSet::new();
        for port in self.media_inputs.iter().chain(self.media_outputs.iter()) {
            if !(!port.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media port id must be non-empty", self.id)));
            }
            if !(media_port_ids.insert(port.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate media port id {}", self.id, port.id)));
            }
        }
        for port in &self.media_inputs {
            if !(port.direction == MediaPortDirection::In) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media input {} must declare direction In", self.id, port.id)));
            }
        }
        for port in &self.media_outputs {
            if !(port.direction == MediaPortDirection::Out) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media output {} must declare direction Out", self.id, port.id)));
            }
            if !(!matches!(port.media_type.form, MediaForm::Any)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media output {} must not declare MediaForm::Any (Any is only legal on inputs, see media_types_compatible)", self.id, port.id)));
            }
        }
        let classification_result = semio_framework::validate_interactive_job_classification(
            actions.iter().map(|action| (self.id.as_str(), action)).chain(self.window_kinds.iter().flat_map(|window| window.actions.iter().map(move |action| (window.id.as_str(), action)))),
            self.commands.iter().map(|command| (self.id.as_str(), command)).chain(self.modes.iter().flat_map(|mode| mode.commands.iter().map(move |command| (mode.id.as_str(), command)))),
        );
        if let Err(errors) = classification_result {
            let message = errors.into_iter().map(|error| error.to_string()).collect::<Vec<_>>().join("; ");
            return Err(PluginAssemblyError::new("app-definition.interactive-job-classification", message));
        }
        let mut definition = AppDefinition {
            id: self.id,
            role,
            dialect,
            label: self.label,
            breadcrumb: self.document,
            icon_id: self.icon_id,
            controller_id: self.controller_id,
            modes: Modes::try_from(self.modes.into_iter().map(|mode| ModeDefinition { id: mode.id, label: mode.label, icon_id: mode.icon_id, tools: mode.tools, layout_id: mode.layout_id, commands: mode.commands }).collect::<Vec<_>>())
                .map_err(|_| PluginAssemblyError::new("app-definition.invalid", "app definition must declare at least one mode (checked above)"))?,
            default_mode_id,
            window_kinds: WindowKinds::try_from(
                self.window_kinds
                    .into_iter()
                    .map(|window| WindowKindDefinition {
                        id: window.id,
                        label: window.label,
                        body_key: window.body_key,
                        surface_kind: legacy_surface_kind(window.surface_kind),
                        icon_id: window.icon_id,
                        options: WindowOptions { measures: window.measures, engagement: window.engagement.map_or(WindowEngagementSlot::None, WindowEngagementSlot::Some) },
                        actions: window.actions,
                        utilities: window.utilities,
                        initial_utility_id: window.initial_utility_id,
                        interactions: window.interactions,
                        params_schema: window.params_schema,
                        artifact_snapshot_schema: window.artifact_snapshot_schema,
                        input_event_schema: window.input_event_schema,
                        output_schema: window.output_schema,
                        capabilities: window.capabilities,
                    })
                    .collect::<Vec<_>>(),
            )
            .map_err(|_| PluginAssemblyError::new("app-definition.invalid", "app definition must declare at least one window kind (checked above)"))?,
            actions: app_roster,
            panel_tabs: self.panel_tabs.into_iter().map(panel_tab_spec_to_definition).collect(),
            keybindings,
            utilities: self.utilities,
            interactions: self.interactions,
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
            fault_notices: Vec::new(),
        };
        for action in semio_framework::interaction_action_definitions(&definition)
            .into_iter()
            .chain(semio_framework::reserved_window_actions(&reserved, &definition))
            .chain([cancellation_action_definition(), semio_framework::host_event_action_definition()])
            .chain(semio_framework::document_transfer_action_definitions())
        {
            if declared_action_ids.insert(action.id.clone()) {
                if let Some(keys) = &action.keys {
                    if bound_keys.insert(keys.clone()) {
                        definition.keybindings.push(Keybinding { keys: keys.clone(), action: KeybindingActionDescriptor { controller_id: definition.controller_id.clone(), action: action.id.clone(), args: None } });
                    }
                }
                for window in definition.window_kinds.iter_mut() {
                    if !window.actions.iter().any(|entry| entry.id == action.id) {
                        window.actions.push(action.clone());
                    }
                }
            }
        }
        Ok(definition)
    }

    /// 🧷️ Panicking convenience wrapper over `try_build_definition` for callers that construct
    /// `AppDefinition`s from static, author-controlled data (the overwhelming majority) and are
    /// not yet migrated onto the fallible path — a malformed definition here is a programming
    /// error, not runtime user input, and existing tests already assert on the panic message.
    /// Plugin `plugin()` entry points that assemble an `AppDefinition` from anything that could
    /// legitimately fail at runtime should call `try_build_definition` directly and propagate the
    /// `PluginAssemblyError` via `?` instead — see `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`.
    pub fn build_definition(self) -> AppDefinition {
        self.try_build_definition().unwrap_or_else(|error| panic!("{error}"))
    }
}
