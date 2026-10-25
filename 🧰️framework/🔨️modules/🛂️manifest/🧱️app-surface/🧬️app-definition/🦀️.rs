//! 🧬️ App definition of the app surface: the app reference, the surface app id and the declared app with its builders. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Surface
/// 🎯️ A surface addressed across plugin boundaries.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AppRef {
    pub plugin_id: String,
    pub app_id: String,
}

/// 🪪️ The one canonical spelling of a surface id: `<artifact_kind>@<standard>/<subset>#<role>`.
pub fn surface_app_id(dialect: &ArtifactDialect, role: AppRole) -> String {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    format!("{}#{}", dialect.to_coordinate(), role.as_str())
}

/// 📖️ Inverse of `surface_app_id`; rejects anything not matching the grammar.
pub fn parse_surface_app_id(id: &str) -> Result<(ArtifactDialect, AppRole), String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let (coordinate, role_str) = id.rsplit_once('#').ok_or_else(|| format!("surface id {id:?} missing '#'"))?;
    let dialect = ArtifactDialect::parse_coordinate(coordinate)?;
    let role: AppRole = role_str.parse().map_err(|err| format!("surface id {id:?}: {err}"))?;
    Ok((dialect, role))
}
//#endregion 🔖️Surface

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppDefinition {
    pub id: String,
    /// 👁️✏️ Whether this surface may mutate the artifact it is bound to — see `AppRole`.
    pub role: AppRole,
    /// 📕️ The dialect coordinate (artifact kind, standard, subset) this surface is bound to — see
    /// `ArtifactDialect`. Together with `role` this derives the canonical `id` via `surface_app_id`.
    pub dialect: ArtifactDialect,
    /// 🗣️ The app's own display name (e.g. "Puzzle 3D") — manifest-level, locale×terminology-checked,
    /// see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub breadcrumb: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<IconName>,
    pub controller_id: String,
    /// 🚧️ `Modes` is `NonEmptyVec<ModeDefinition>`, whose `serde(try_from/into = "Vec<T>")` wire
    /// format is a flat array — not the `{ first, rest }` shape owned schema exporter would infer from the struct
    /// fields, so the wire-accurate array shape is supplied directly instead of deriving `TS` on
    /// `NonEmptyVec` itself.
    pub modes: Modes,
    pub default_mode_id: String,
    /// 🪟️ See `modes` above — `WindowKinds` is `NonEmptyVec<WindowKindDefinition>`.
    pub window_kinds: WindowKinds,
    /// 🕹️ The app-wide action roster: every action dispatchable in ANY window of this app that no
    /// window kind claims as its own, including the framework-injected History/Clipboard verbs and
    /// every reserved contribution. A window kind's dispatchable set is `WindowKindDefinition.actions` followed by
    /// this roster minus the ids some window claims — [`resolve_window_actions`], whose TypeScript
    /// twin is `resolveWindowActions` (`🎯️action-bus/🟦️.ts`). Kept here rather than copied into
    /// every window kind because a copy made the package descriptor grow as
    /// `apps × window kinds × actions`: 21 distinct rows were stored 675 times in one plugin, and
    /// the duplicates were 31.9 % of every shipped descriptor's bytes (ticket
    /// 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, slice DS1).
    #[serde(default)]
    #[value(default)]
    pub actions: Vec<ActionDefinition>,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
    /// 🧰️ The interactive utilities this app exposes (referenced by `WindowKindDefinition.utilities`).
    #[serde(default)]
    #[value(default)]
    pub utilities: Vec<UtilityDefinition>,
    /// 🛠️ The mode-level tools this app exposes (referenced by `ModeDefinition.tools`).
    #[serde(default)]
    #[value(default)]
    pub tools: Vec<ToolDefinition>,
    /// 🎛️ Commands owned by this app and active whenever it is focused.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
    /// 🖱️ The interaction domains (hover + selection) this app exposes (referenced by
    /// `WindowKindDefinition.interactions`) — see `crate::InteractionDefinition`.
    #[serde(default)]
    #[value(default)]
    pub interactions: Vec<InteractionDefinition>,
    #[serde(default)]
    #[value(default)]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_layout: Option<WindowLayout>,
    /// 🔑️ Terminology ids this app declares beyond the implicit "native" default.
    #[serde(default)]
    #[value(default)]
    pub terminologies: Vec<String>,
    /// 🗺️ Terminology id -> full replacement breadcrumb (product + app segments), e.g. "reuse" ->
    /// ["Entwerfen mit Bestand", "Aggregator"]; ids absent here keep the canonical breadcrumb under that terminology.
    #[serde(default)]
    #[value(default)]
    pub terminology_breadcrumbs: std::collections::HashMap<String, Vec<String>>,
    /// 🎓️ This app's first-run walkthrough, if it declares one — see `IntroductionDefinition`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<IntroductionDefinition>,
    /// 🎬️ Recorded, timed walkthroughs this app declares — see `TutorialDefinition`. A brand's own
    /// `tutorials` (if any) are shown alongside these, never replacing them (unlike `introduction`).
    #[serde(default)]
    #[value(default)]
    pub tutorials: Vec<TutorialDefinition>,
    /// 🗨️ The modal form dialogs this app can open via `Effect::OpenDialog`.
    #[serde(default)]
    #[value(default)]
    pub dialogs: Vec<DialogDefinition>,
    /// 🔌️ This app's workflow input ports — see `crate::MediaPortSpec`.
    #[serde(default)]
    #[value(default)]
    pub media_inputs: Vec<MediaPortSpec>,
    /// 🖨️ This app's workflow output ports — see `crate::MediaPortSpec`.
    #[serde(default)]
    #[value(default)]
    pub media_outputs: Vec<MediaPortSpec>,
    /// 🗂️ OS resource kinds this app produces/consumes — see `crate::ArtifactKindSpec`. Drives
    /// `framework/product/os/core`'s artifact catalog registry instead of a hardcoded per-app match.
    #[serde(default)]
    #[value(default)]
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    /// 🧮️ This app's typed configuration record — see `crate::ConfigSpec`. Empty until per-app waves
    /// populate it.
    #[serde(default)]
    #[value(default)]
    pub config: ConfigSpec,
    /// ⌨️ This app's typed binary command grammar — see `crate::CommandGrammar`. Empty until per-app
    /// waves populate it.
    #[serde(default)]
    #[value(default)]
    pub command_grammar: CommandGrammar,
    /// 📺️ This app's typed media I/O surface — see `crate::AppIo`. Not yet populated; `media_inputs`/
    /// `media_outputs`/`artifact_kinds` above remain the live source of truth until later waves migrate
    /// onto this.
    #[serde(default)]
    #[value(default)]
    pub io: AppIo,
    /// 📢️ The localized notices of the fault codes this app's guest refuses with (`ArtifactApp::fault_notices`, design
    /// §20.12), stamped at registration — what both shells show for a refused dispatch after the framework's own table.
    #[serde(default)]
    #[value(default)]
    pub fault_notices: Vec<FaultNoticeDefinition>,
}
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
