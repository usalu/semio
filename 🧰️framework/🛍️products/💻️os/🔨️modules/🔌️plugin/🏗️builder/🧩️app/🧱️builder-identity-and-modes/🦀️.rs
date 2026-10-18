//! 🧱️ Builder identity and modes: the app builder constructor and the artifact kind, config, io, command grammar, media port, terminology, onboarding, dialog, icon, document and mode setters. See [`super`].

use super::*;

impl AppBuilder {
    pub async fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
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
            interactions: Vec::new(),
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
            config: ::semio_framework_async::poll::resolve_ready(ConfigSpec::empty()),
            command_grammar: ::semio_framework_async::poll::resolve_ready(CommandGrammar::empty()),
            io: AppIo::default(),
        }
    }

    /// 🗂️ Declares one resource kind this app produces/consumes (see `ArtifactKindSpec`). Repeatable.
    pub async fn artifact_kind(mut self, spec: ArtifactKindSpec) -> Self {
        self.artifact_kinds.push(spec);
        self
    }

    /// 🧮️ Declares this app's typed configuration record (see `crate::ConfigSpec`).
    pub async fn config(mut self, spec: ConfigSpec) -> Self {
        self.config = spec;
        self
    }

    /// 🔌️ Declares this app's typed media I/O surface (see `crate::AppIo`).
    pub async fn io(mut self, io: AppIo) -> Self {
        self.io = io;
        self
    }

    /// 🎛️ Declares this app's typed binary command grammar (see `crate::CommandGrammar`).
    pub async fn command_grammar(mut self, grammar: CommandGrammar) -> Self {
        self.command_grammar = grammar;
        self
    }

    /// 🔌️ Declares one workflow input port this app accepts (see `MediaPortSpec`). Repeatable;
    /// validated in `build_definition` (non-empty/unique id, `direction` must be `In`).
    pub async fn media_input(mut self, spec: MediaPortSpec) -> Self {
        self.media_inputs.push(spec);
        self
    }

    /// 🔌️ Declares one workflow output port this app produces (see `MediaPortSpec`). Repeatable;
    /// validated in `build_definition` (non-empty/unique id, `direction` must be `Out`, `MediaForm::Any`
    /// is rejected — `Any` is only ever legal on the accepting/input side, see `media_types_compatible`).
    pub async fn media_output(mut self, spec: MediaPortSpec) -> Self {
        self.media_outputs.push(spec);
        self
    }

    /// 🗣️ Declares an alternative terminology id this app supports beyond the implicit "native" default.
    pub async fn terminology(mut self, id: impl Into<String>) -> Self {
        self.terminologies.push(id.into());
        self
    }

    /// 🗺️ Replaces the full document path (product + app segments) while terminology `id` is active;
    /// `id` must also be declared via `terminology` — validated in `build_definition`.
    pub async fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.terminology_breadcrumbs.insert(id.into(), document.into_iter().map(Into::into).collect());
        self
    }

    /// 🎓️ Declares this app's first-run introduction walkthrough. Step anchors/advance
    /// conditions are validated against declared window kinds/utilities/actions/panel tabs in
    /// `build_definition`; declaring one auto-injects the `startIntroduction` action.
    pub async fn introduction(mut self, introduction: IntroductionDefinition) -> Self {
        self.introduction = Some(introduction);
        self
    }

    /// 🎬️ Declares one recorded, timed tutorial (repeatable — an app may offer several). Every
    /// track is validated in `build_definition` (`validate_tutorial` plus referenced action/command/
    /// utility/tool/element ids); declaring at least one auto-injects the `startTutorial` action. The
    /// `recordTutorial` action is injected unconditionally (see `record_tutorial_action_definition`).
    pub async fn tutorial(mut self, tutorial: TutorialDefinition) -> Self {
        self.tutorials.push(tutorial);
        self
    }

    /// 🗨️ Declares a modal form dialog (repeatable). `submit_action`/`cancel_action` and its
    /// `args` are validated in `build_definition`; opened only via `Effect::OpenDialog`.
    pub async fn dialog(mut self, dialog: DialogDefinition) -> Self {
        self.dialogs.push(dialog);
        self
    }

    pub async fn icon_id(mut self, icon_id: impl Into<IconName>) -> Self {
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

    pub async fn mode(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        self.modes.push(ModeSpec { id: id.into(), label: label.into(), icon_id: icon_id.into(), tools: Vec::new(), layout_id: None, commands: Vec::new() });
        self
    }

    /// 🎛️ Replaces the commands structurally owned by one mode.
    pub async fn mode_commands(mut self, mode_id: impl AsRef<str>, commands: Vec<CommandDefinition>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.commands = commands;
        }
        self
    }

    /// 🎛️ Adds one command definition to the mode that owns it.
    pub async fn mode_command(mut self, mode_id: impl AsRef<str>, command: CommandDefinition) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.commands.push(command);
        }
        self
    }

    pub async fn mode_layout(mut self, mode_id: impl AsRef<str>, layout_id: impl Into<String>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.layout_id = Some(layout_id.into());
        }
        self
    }

    /// 🛠️ Scopes tools to a mode — references ids declared via `.tool()`/`.tool_simple()`. A tool is a
    /// mode-level activatable capability (e.g. puzzle3d fill); distinct from a window-scoped utility.
    pub async fn mode_tools(mut self, mode_id: impl AsRef<str>, tool_ids: Vec<ToolRef>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.tools = tool_ids;
        }
        self
    }

    pub async fn default_mode_id(mut self, id: impl Into<String>) -> Self {
        self.default_mode_id = Some(id.into());
        self
    }

}
