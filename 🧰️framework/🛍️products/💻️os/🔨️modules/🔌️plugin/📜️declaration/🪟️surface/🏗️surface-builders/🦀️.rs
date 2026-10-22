//! 🏗️ Surface builders: the viewer and editor wrappers, their builders with the forwarded builder methods, the snapshot builder and the subset example tables. See [`super`].

use super::*;

/// 🎭️👁️ Marker whose `builder` seeds a `ViewerBuilder` with the surface id/role/dialect already
/// derived (contract §2.4) — mirrors `App::builder`'s `(id, label) -> AppBuilder` shape but never
/// accepts a hand-written id.
pub struct Viewer;

/// 🎭️✏️ Editor twin of `Viewer`.
pub struct Editor;

impl Viewer {
    pub fn builder(dialect: Dialect) -> ViewerBuilder {
        let dialect: ArtifactDialect = dialect.into();
        let inner = Box::new(::semio_framework_async::poll::resolve_ready(AppBuilder::new(surface_app_id(&dialect, AppRole::Viewer), LocalizedLabel::native("Viewer", "Betrachter"))));
        ViewerBuilder { inner, dialect }
    }
}

impl Editor {
    pub fn builder(dialect: Dialect) -> EditorBuilder {
        let dialect: ArtifactDialect = dialect.into();
        let inner = Box::new(::semio_framework_async::poll::resolve_ready(AppBuilder::new(surface_app_id(&dialect, AppRole::Editor), LocalizedLabel::native("Editor", "Editor"))));
        EditorBuilder { inner, dialect }
    }
}

/// 👁️ Fluent viewer surface builder (contract §2.4) — every `AppBuilder` method except
/// `.mutation(...)`, which does not exist on this type: calling it is a compile error, not a
/// runtime rejection.
pub struct ViewerBuilder {
    inner: Box<AppBuilder>,
    pub dialect: ArtifactDialect,
}

/// ✏️ Fluent editor surface builder (contract §2.4) — every `AppBuilder` method, `.mutation(...)`
/// included.
/// 📦️ `inner` is BOXED on purpose. Every fluent method takes `self` by value and returns a new
/// surface builder, so an unoptimized build gives each step of a chain its own stack slot for the
/// whole returned value: a real app manifest chains ~250 steps, and an inline `AppBuilder` made
/// that 400 KiB of one frame — 20 % of a 2 MiB test thread spent before the app even exists, which
/// is why every whole-app law in `puzzle3d` ran at 70–95 % of its stack and the heaviest one
/// overflowed (ticket 26/09/02 wave F). Behind a `Box` a chain step costs a pointer, and the
/// `AppBuilder` value itself only ever lives in the frame of the one method that moves it.
pub struct EditorBuilder {
    inner: Box<AppBuilder>,
    pub dialect: ArtifactDialect,
}

impl ViewerBuilder {
    #[inline(never)]
    pub fn document<I, S>(mut self, document: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.inner = Box::new((*self.inner).document(document));
        self
    }
    #[inline(never)]
    pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).terminology_document(id, document)));
        self
    }
    pub fn build_definition(self) -> AppDefinition {
        let mut definition = (*self.inner).build_definition();
        definition.role = AppRole::Viewer;
        definition.dialect = self.dialect;
        definition
    }
}

impl EditorBuilder {
    #[inline(never)]
    pub fn document<I, S>(mut self, document: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.inner = Box::new((*self.inner).document(document));
        self
    }
    #[inline(never)]
    pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).terminology_document(id, document)));
        self
    }
    /// ✏️ Declares a document-mutating action — the one method `ViewerBuilder` doesn't have.
    #[inline(never)]
    pub fn mutation(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).mutation(id, label)));
        self
    }
    pub fn build_definition(self) -> AppDefinition {
        let mut definition = (*self.inner).build_definition();
        definition.role = AppRole::Editor;
        definition.dialect = self.dialect;
        definition
    }
}

/// 🏭️ Forwards the `AppBuilder` methods `ViewerBuilder`/`EditorBuilder` share verbatim (every
/// `AppBuilder` builder method except `new`/`build_definition`/`document`/`terminology_document`/
/// `mutation`, each handled above) — one macro invocation is the single source of truth for the
/// shared surface instead of two ~45-method hand-copied impls drifting apart.
macro_rules! surface_builder_forward {
    ( [ $($Type:ident),+ ] ; $methods:tt ) => {
        $(
            surface_builder_forward!(@type $Type; $methods);
        )+
    };
    ( @type $Type:ident; { $( $name:ident ( $( $arg:ident : $ty:ty ),* ) ),+ $(,)? } ) => {
        impl $Type {
            $(
                #[inline(never)]
                pub fn $name(mut self, $( $arg: $ty ),* ) -> Self {
                    self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).$name( $( $arg ),* )));
                    self
                }
            )+
        }
    };
}

surface_builder_forward!(
    [ViewerBuilder, EditorBuilder];
    {
    artifact_kind(spec: ArtifactKindSpec),
    config(spec: ConfigSpec),
    io(io: AppIo),
    command_grammar(grammar: CommandGrammar),
    media_input(spec: MediaPortSpec),
    media_output(spec: MediaPortSpec),
    terminology(id: impl Into<String>),
    introduction(introduction: IntroductionDefinition),
    tutorial(tutorial: TutorialDefinition),
    dialog(dialog: DialogDefinition),
    icon_id(icon_id: impl Into<IconName>),
    mode(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>),
    mode_commands(mode_id: impl AsRef<str>, commands: Vec<CommandDefinition>),
    mode_command(mode_id: impl AsRef<str>, command: CommandDefinition),
    mode_layout(mode_id: impl AsRef<str>, layout_id: impl Into<String>),
    mode_tools(mode_id: impl AsRef<str>, tool_ids: Vec<ToolRef>),
    default_mode_id(id: impl Into<String>),
    window_kind(id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, icon_id: impl Into<IconName>),
    window_kind_with_engagement(id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, engagement: WindowEngagement, icon_id: impl Into<IconName>),
    mode_def(def: ModeDefinition),
    window_kind_def(def: WindowKindDefinition),
    panel_tab_def(def: PanelTabDefinition),
    window_kind_measures(window_kind_id: impl AsRef<str>, measures: Vec<WindowMeasure>),
    window_kind_actions(window_kind_id: impl AsRef<str>, actions: Vec<ActionDefinition>),
    window_kind_action_refs(window_kind_id: impl AsRef<str>, action_refs: Vec<ActionRef>),
    window_kind_utilities(window_kind_id: impl AsRef<str>, utility_ids: Vec<UtilityRef>),
    window_kind_initial_utility(window_kind_id: impl AsRef<str>, utility_id: impl Into<UtilityRef>),
    window_kind_interactions(window_kind_id: impl AsRef<str>, interaction_ids: Vec<InteractionRef>),
    named_layout(layout: NamedLayout),
    default_layout(layout: WindowLayout),
    panel_tab(id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: impl Into<String>),
    panel_tab_tree(tab: PanelTabSpec),
    panel_tab_framework(tab: PanelTabSpec),
    keybinding(keys: impl Into<String>, action: impl Into<String>),
    view_action(id: impl Into<String>, label: impl Into<LocalizedLabel>),
    shell_action(id: impl Into<String>, label: impl Into<LocalizedLabel>),
    action_with(action: ActionDefinition),
    action_args(action_id: impl AsRef<str>, args: Vec<ActionArgDef>),
    action_describe(action_id: impl AsRef<str>, description: impl Into<LocalizedLabel>),
    action_use_when(action_id: impl AsRef<str>, phrases: Vec<String>),
    action_audience(action_id: impl AsRef<str>, audience: CapabilityAudience),
    action_destructive(action_id: impl AsRef<str>),
    action_interactive_job(action_id: impl AsRef<str>, classification: semio_framework::InteractiveJobClassification),
    interactive_jobs(classification: semio_framework::InteractiveJobClassification),
    command(command: CommandDefinition),
    app_command(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind),
    command_args(command_id: impl AsRef<str>, args: Vec<ActionArgDef>),
    utility(utility: UtilityDefinition),
    interaction(interaction: InteractionDefinition),
    utility_simple(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>),
    tool(tool: ToolDefinition),
    tool_simple(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>),
    }
);

/// ❄️ Owns wire DTOs for explicitly selected reference decisions and synchronous diff application.
pub struct SnapshotBuilder<S, M> {
    pub(super) snapshot: S,
    _mutation: std::marker::PhantomData<fn() -> M>,
}

impl<S, M> SnapshotBuilder<S, M> {
    fn wrap(snapshot: S) -> Self {
        Self { snapshot, _mutation: std::marker::PhantomData }
    }
}

impl<S, M> ArtifactBuilder for SnapshotBuilder<S, M>
where
    S: Default + Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot,
    M: Mutation<S> + ::protocol::ColdMutation<S>,
    M::Diff: MutationDiff<S>,
{
    type Snapshot = S;
    type Mutation = M;
    type Diff = <M as ::protocol::ColdMutation<S>>::ReferenceDiff;

    fn empty() -> Self {
        Self::wrap(S::default())
    }
    fn from_snapshot(snapshot: S) -> Self {
        Self::wrap(snapshot)
    }
    fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(Self::wrap(S::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::wrap(S::decode_pack(bytes)?))
    }
    fn mutate(self, mutation: M) -> (Self, protocol::MutationOutcome<<M as ::protocol::ColdMutation<S>>::ReferenceDiff>) {
        let (snapshot, outcome) = store::os_store::replay::apply_outcome(&self.snapshot, mutation.diff(&self.snapshot));
        drop(mutation);
        drop(self.snapshot);
        (Self::wrap(snapshot), outcome)
    }
    fn absorb(self, diff: <M as ::protocol::ColdMutation<S>>::ReferenceDiff) -> protocol::MutationApplyResult<Self> {
        let applied = protocol::apply_diff_cold(&diff, &self.snapshot);
        drop(diff);
        drop(self.snapshot);
        applied.map(Self::wrap)
    }
    fn build(self) -> Result<S, Vec<semio_framework_diagnostic::Diagnostic>> {
        Ok(self.snapshot)
    }
}

#[cfg(test)]
#[path = "../🏗️snapshot-builder/🧪️tests/🦀️.rs"]
mod snapshot_builder_ownership_tests;

fn subset_example_table() -> &'static std::sync::Mutex<HashMap<Dialect, Vec<ExampleSource>>> {
    static TABLE: std::sync::OnceLock<std::sync::Mutex<HashMap<Dialect, Vec<ExampleSource>>>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| std::sync::Mutex::new(HashMap::new()))
}

pub fn remember_subset_examples(dialect: Dialect, examples: &[ExampleSource]) {
    if examples.is_empty() {
        return;
    }
    subset_example_table().lock().expect("subset example catalogue").insert(dialect, examples.to_vec());
}

/// 📚️ Examples a subset declaration registered for `dialect`.
pub(super) fn subset_examples(dialect: &Dialect) -> Vec<ExampleSource> {
    subset_example_table().lock().expect("subset example catalogue").get(dialect).cloned().unwrap_or_default()
}
