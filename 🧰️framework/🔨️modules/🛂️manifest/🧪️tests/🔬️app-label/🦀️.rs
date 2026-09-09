
use super::app_breadcrumb;

//#region 🔖️UiDirtyScopeTests
/// 🐢️ Regression: `rename_all = "camelCase"` on an enum only renames *variant* names via `tag`, not
/// the fields inside a struct variant — those need `rename_all_fields` too, or `Partial`'s fields
/// silently serialize as snake_case (`window_bodies`) while the TS `UiDirtyScope` type expects
/// camelCase (`windowBodies`), desyncing the wire contract without any compile-time signal.
#[semio_framework_async_macros::async_test]
async fn ui_dirty_scope_partial_serializes_fields_as_camel_case() {
    use crate::kernel::UiDirtyScope;
    let scope = UiDirtyScope::Partial { window_bodies: vec!["a".into()], panel_bodies: vec!["b".into()], utilities: true, tools: false, engagements: true, measures: false, labels: false };
    let json = serde_json::to_string(&scope).unwrap();
    assert!(json.contains("\"windowBodies\""), "{json}");
    assert!(json.contains("\"panelBodies\""), "{json}");
    assert!(!json.contains("window_bodies"), "{json}");
    assert!(!json.contains("panel_bodies"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn ui_dirty_scope_defaults_to_full() {
    use crate::kernel::UiDirtyScope;
    assert_eq!(UiDirtyScope::default(), UiDirtyScope::Full);
    assert_eq!(serde_json::to_string(&UiDirtyScope::Full).unwrap(), "{\"kind\":\"full\"}");
    // Absent from JSON (an older program that never sets it) must also deserialize to Full.
    #[derive(serde::Deserialize)]
    struct Wrapper {
        #[serde(default)]
        ui_scope: UiDirtyScope,
    }
    let parsed: Wrapper = serde_json::from_str("{}").unwrap();
    assert_eq!(parsed.ui_scope, UiDirtyScope::Full);
}
//#endregion UiDirtyScopeTests

#[semio_framework_async_macros::async_test]
async fn formats_app_label_for_chrome() {
    assert_eq!(app_breadcrumb(&["semio".into(), "puzzle".into(), "3d".into()]), "semio · puzzle · 3d");
}

//#region 🔖️ActionArgsAndUtilitiesTests
use crate::ui::kernel;
use crate::ui::kernel::{Effect, RequestId};
use crate::ui::{
    ActionAddress,
    ActionArgControl,
    ActionArgDef,
    ActionArgOption,
    ActionDefinition,
    ActionInvocation,
    ActionKind,
    ActionRef,
    // 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema.
    ActionSemantics,
    AppDefinition,
    AppRef,
    AppRole,
    ApprovalMode,
    CLEAR_SELECTION_ACTION_ID,
    CommandAddress,
    CommandDefinition,
    CommandInvocation,
    CommandOwnerAddress,
    DialogDefinition,
    INTERACTION_HOVER_ACTION_ID,
    INTERACTION_SELECT_ACTION_ID,
    IntroductionCursor,
    IntroductionDemonstration,
    IntroductionGesture,
    IntroductionInteraction,
    IntroductionInteractionKind,
    IntroductionKeyModifier,
    IntroductionPoint,
    IntroductionPointerButton,
    IntroductionStepDefinition,
    Locale,
    LocalizedLabel,
    Modes,
    NonEmptyVec,
    OsDefinition,
    PanelGroup,
    PanelTabDefinition,
    PanelTabKind,
    Platform,
    PlatformKeybinding,
    PreviewMode,
    RECORD_TUTORIAL_ACTION_ID,
    SELECT_ALL_ACTION_ID,
    SET_ACTIVE_UTILITY_ACTION_ID,
    SET_INTERACTION_GRANULARITY_ACTION_ID,
    SET_SELECTION_MODE_ACTION_ID,
    START_TUTORIAL_ACTION_ID,
    Terminology,
    ToolRef,
    TutorialArtifactEvent,
    TutorialArtifactEventKind,
    TutorialAssetSrc,
    TutorialBase,
    TutorialCameraKeyframe,
    TutorialCameraState,
    TutorialChapter,
    TutorialDefinition,
    TutorialEasing,
    TutorialEvent,
    TutorialEventKind,
    TutorialNarrationCue,
    TutorialTracks,
    TutorialUiChange,
    TutorialUiKeyframe,
    TutorialUiSample,
    TutorialUiSnapshot,
    UI_FOOTER_ELEMENT_ID,
    UI_NAVBAR_ELEMENT_ID,
    UndoMode,
    UtilityDefinition,
    UtilityRef,
    WindowKindDefinition,
    WindowKinds,
    app_window_label,
    child_element_id,
    compose_tutorial_ui,
    effective_action_args,
    element_id_segment,
    interaction_action_definitions,
    interpolate_tutorial_camera,
    is_element_id,
    missing_required_args,
    panel_tab_element_id,
    panel_tab_first_draggable_element_id,
    parse_surface_app_id,
    record_tutorial_action_definition,
    resolve_app_breadcrumb,
    resolve_layout_for_mode,
    resolve_mode_tools,
    resolve_window_actions,
    start_tutorial_action_definition,
    surface_app_id,
    tutorial_camera_at,
    tutorial_slice,
    validate_tutorial,
    window_element_id,
};
// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W1: the wave-0 interaction
// definition family lives at the crate root, not under `crate::ui` — see the equivalent `use`
// at this file's top.
use crate::{
    ArtifactDialect, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractiveJobClassification, InteractiveJobClassificationError, MergeMode, SelectionMethod, SelectionMode, SelectionSpec,
    validate_interactive_job_classification,
};
use dsl::DslValue;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn action_arg_def_builder_chain() {
    let arg = ActionArgDef::slider("scale", LocalizedLabel::data("Scale"), 0.0, 4.0).required().default_value(&1.0).describe("scale factor");
    assert_eq!(arg.id, "scale");
    assert!(arg.required);
    assert_eq!(arg.default, Some(dsl::to_dsl_value(&1.0f64).unwrap()));
    assert_eq!(arg.description.as_deref(), Some("scale factor"));
    assert!(matches!(arg.control(), ActionArgControl::Slider { min, max, .. } if min == 0.0 && max == 4.0));
}

/// @emoji 🧪️ D6 regression proof (ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet
/// P3-manifest-schema): each of the six `ActionArgDef` builder helpers must still derive EXACTLY
/// the `ActionArgControl` it used to construct directly, now that `control` is a stored→derived
/// field — this is the whole refactor's regression guard for the ~236 call sites across 33 plugins.
#[semio_framework_async_macros::async_test]
async fn six_arg_builder_helpers_derive_the_pre_d6_control() {
    assert_eq!(ActionArgDef::text("t", LocalizedLabel::data("T")).control(), ActionArgControl::Text { placeholder: None });
    assert_eq!(ActionArgDef::number("n", LocalizedLabel::data("N")).control(), ActionArgControl::Number { min: None, max: None, step: None });
    assert_eq!(ActionArgDef::slider("s", LocalizedLabel::data("S"), 0.0, 4.0).control(), ActionArgControl::Slider { min: 0.0, max: 4.0, step: None, unit: None });
    assert_eq!(ActionArgDef::toggle("b", LocalizedLabel::data("B")).control(), ActionArgControl::Toggle);
    let options = vec![ActionArgOption::new("x", LocalizedLabel::data("X"))];
    assert_eq!(ActionArgDef::select("o", LocalizedLabel::data("O"), options.clone()).control(), ActionArgControl::Select { options });
    assert_eq!(ActionArgDef::vec3("v", LocalizedLabel::data("V")).control(), ActionArgControl::Vec3);
}

/// @emoji 🧪️ The two host-resolved builders (unused by any current call site, per the P3 reader
/// audit) still derive their pre-D6 controls too — `ArgFormat::ArtifactKind`/`SurfaceApp` exist
/// solely so these keep working under the new stored/derived split.
#[semio_framework_async_macros::async_test]
async fn host_resolved_arg_builders_derive_their_pre_d6_controls() {
    let roles = vec![AppRole::Viewer];
    assert_eq!(ActionArgDef::artifact_kind("k", LocalizedLabel::data("K"), roles.clone()).control(), ActionArgControl::ArtifactKind { roles: roles.clone() });
    assert_eq!(ActionArgDef::surface_app("s", LocalizedLabel::data("S"), roles.clone(), "dialect").control(), ActionArgControl::SurfaceApp { roles, dialect_arg: "dialect".to_string() });
}

/// @emoji 🧪️ `ActionSemantics::for_kind` matches the `📋️master.md` §3.1 defaults table.
#[semio_framework_async_macros::async_test]
async fn action_semantics_for_kind_matches_the_defaults_table() {
    let mutation = ActionSemantics::for_kind(ActionKind::Mutation);
    assert!(mutation.effects.reversible);
    assert_eq!(mutation.execution.preview, PreviewMode::Diff);
    assert_eq!(mutation.execution.undo, UndoMode::Inverse);
    assert!(mutation.execution.expected_revision);
    assert_eq!(mutation.policy.approval, ApprovalMode::WhenDestructive);
    assert_eq!(mutation.policy.scopes, vec![kernel::CapabilityId("documents.write".into())]);

    let view = ActionSemantics::for_kind(ActionKind::View);
    let interaction = ActionSemantics::for_kind(ActionKind::Interaction);
    assert_eq!(view, interaction, "View and Interaction share the config-lane defaults");
    assert_eq!(view.policy.scopes, vec![kernel::CapabilityId("documents.read".into()), kernel::CapabilityId("shell.observe".into())]);

    assert_eq!(ActionSemantics::for_kind(ActionKind::History).policy.scopes, vec![kernel::CapabilityId("documents.write".into())]);
    assert_eq!(ActionSemantics::for_kind(ActionKind::Clipboard).policy.scopes, vec![kernel::CapabilityId("shell.clipboard".into())]);

    let shell = ActionSemantics::for_kind(ActionKind::Shell);
    assert!(!shell.effects.reversible);
    assert_eq!(shell.policy.scopes, vec![kernel::CapabilityId("shell.navigate".into())]);
}

/// @emoji 🧪️ `bounded_catalog` retains kind defaults but never grants execution authority.
#[semio_framework_async_macros::async_test]
async fn action_definition_semantics_default_from_kind_and_builders_compose() {
    let mutation = ActionDefinition::bounded_catalog("deleteThing", LocalizedLabel::data("Delete Thing"), ActionKind::Mutation);
    let expected = ActionSemantics::for_kind(ActionKind::Mutation);
    assert_eq!(mutation.semantics, expected);

    let action = ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::data("Delete"), ActionKind::Mutation)
        .destructive()
        .await
        .use_when(["delete the selected objects", "remove selection"])
        .example("deleteSelection removes every currently selected object")
        .await;
    assert!(action.semantics.effects.destructive);
    assert_eq!(action.semantics.policy.approval, ApprovalMode::WhenDestructive);
    assert_eq!(action.semantics.use_when, vec!["delete the selected objects".to_string(), "remove selection".to_string()]);
    assert_eq!(action.semantics.examples, vec!["deleteSelection removes every currently selected object".to_string()]);
}

#[test]
fn interactive_job_classification_is_explicit_and_release_validated() {
    let mut migrated_action = ActionDefinition::bounded_catalog("select", LocalizedLabel::data("Select"), ActionKind::Interaction);
    let mut migrated_command = CommandDefinition::bounded_catalog("solve", LocalizedLabel::data("Solve"), "analysis", ActionKind::Mutation);
    assert_eq!(migrated_action.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified);
    migrated_action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    migrated_command.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    assert_eq!(migrated_action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(validate_interactive_job_classification([("app", &migrated_action)], [("app", &migrated_command)]).is_ok());

    let mut unclassified = migrated_command;
    unclassified.semantics.execution.interactive_job = InteractiveJobClassification::Unclassified;
    let errors = validate_interactive_job_classification(std::iter::empty(), [("app.mode", &unclassified)]).expect_err("unclassified command must block a release catalog");
    assert_eq!(errors, vec![InteractiveJobClassificationError { owner: "app.mode".into(), id: "solve".into() }]);
}

/// @emoji 🧪️ `ActionArgDef::json_schema`/`arg_schema_json_schema` produce sane JSON Schema 2020-12
/// leaves for the shapes P3-manifest-schema actually introduces.
#[semio_framework_async_macros::async_test]
async fn action_arg_def_json_schema_covers_the_core_shapes() {
    let text = ActionArgDef::text("name", LocalizedLabel::data("Name")).describe("a name").json_schema();
    assert_eq!(text["type"], serde_json::json!("string"));
    assert_eq!(text["description"], serde_json::json!("a name"));

    let options = vec![ActionArgOption::new("obj", LocalizedLabel::data("Object")), ActionArgOption::new("stl", LocalizedLabel::data("STL"))];
    let select = ActionArgDef::select("format", LocalizedLabel::data("Format"), options).json_schema();
    assert_eq!(select["type"], serde_json::json!("string"));
    assert_eq!(select["enum"], serde_json::json!(["obj", "stl"]));

    let number = ActionArgDef::slider("scale", LocalizedLabel::data("Scale"), 0.0, 4.0).json_schema();
    assert_eq!(number["type"], serde_json::json!("number"));
    assert_eq!(number["minimum"], serde_json::json!(0.0));
    assert_eq!(number["maximum"], serde_json::json!(4.0));

    let vec3 = ActionArgDef::vec3("position", LocalizedLabel::data("Position")).json_schema();
    assert_eq!(vec3["type"], serde_json::json!("array"));
    assert_eq!(vec3["minItems"], serde_json::json!(3));
    assert_eq!(vec3["maxItems"], serde_json::json!(3));

    let toggle = ActionArgDef::toggle("flag", LocalizedLabel::data("Flag")).json_schema();
    assert_eq!(toggle["type"], serde_json::json!("boolean"));
}

#[semio_framework_async_macros::async_test]
async fn effective_args_prefer_staged_then_default() {
    let defs = vec![ActionArgDef::text("a", LocalizedLabel::data("A")).default_value(&"da"), ActionArgDef::text("b", LocalizedLabel::data("B")).default_value(&"db"), ActionArgDef::text("c", LocalizedLabel::data("C"))];
    let staged = dsl::os_pack::json::to_dsl_value(&dsl::json!({ "a": "staged-a" }));
    let effective = effective_action_args(&defs, &staged, None);
    assert_eq!(effective.get("a"), Some(&DslValue::String("staged-a".into())), "staged wins");
    assert_eq!(effective.get("b"), Some(&DslValue::String("db".into())), "default fills in");
    assert!(!effective.as_object().is_some_and(|o| o.iter().any(|(k, _)| k == "c")), "no staged, no default ⇒ omitted");
}

/// 👁️🔒 26/08/16 HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-I: the framework-shared
/// bug that dropped a dialog's seeded, non-form context arg (e.g. `shareSpace`'s `spaceId`) before
/// it ever reached the dispatched descriptor, causing the hub to authorize against an empty id.
#[semio_framework_async_macros::async_test]
async fn effective_args_preserve_a_seeded_arg_not_declared_as_a_form_field() {
    let defs = vec![ActionArgDef::text("email", LocalizedLabel::data("Email")), ActionArgDef::text("role", LocalizedLabel::data("Role")).default_value(&"author")];
    let staged = dsl::os_pack::json::to_dsl_value(&dsl::json!({ "email": "user2@semio.dev" }));
    let seed = dsl::os_pack::json::to_dsl_value(&dsl::json!({ "spaceId": "sp-1" }));
    let effective = effective_action_args(&defs, &staged, Some(&seed));
    assert_eq!(effective.get("spaceId"), Some(&DslValue::String("sp-1".into())), "the seeded, non-declared arg must reach the dispatched descriptor");
    assert_eq!(effective.get("email"), Some(&DslValue::String("user2@semio.dev".into())), "the form's own staged field still resolves");
    assert_eq!(effective.get("role"), Some(&DslValue::String("author".into())), "declared defaults still fill in alongside a seed");
}

/// 🌱️ A seed value for a DECLARED field pre-fills it (e.g. `renameSpace` seeding the current name
/// into its own editable `name` field) until the form stages its own edit, which then wins.
#[semio_framework_async_macros::async_test]
async fn effective_args_seed_prefills_a_declared_field_until_staged_overrides_it() {
    let defs = vec![ActionArgDef::text("name", LocalizedLabel::data("Name"))];
    let seed = dsl::os_pack::json::to_dsl_value(&dsl::json!({ "spaceId": "sp-1", "name": "Old Name" }));
    let untouched = effective_action_args(&defs, &DslValue::Object(Vec::new()), Some(&seed));
    assert_eq!(untouched.get("name"), Some(&DslValue::String("Old Name".into())), "seed pre-fills the declared field");
    let staged = dsl::os_pack::json::to_dsl_value(&dsl::json!({ "name": "New Name" }));
    let edited = effective_action_args(&defs, &staged, Some(&seed));
    assert_eq!(edited.get("name"), Some(&DslValue::String("New Name".into())), "staged still wins over the seed");
    assert_eq!(edited.get("spaceId"), Some(&DslValue::String("sp-1".into())), "the non-declared seed key survives regardless");
}

/// 🗑️ A zero-declared-field confirm dialog (`deleteSpace`'s confirm/cancel shape) must pass its
/// entire seeded context through wholesale — there is no form field to carry it otherwise.
#[semio_framework_async_macros::async_test]
async fn effective_args_pass_seed_through_wholesale_when_no_fields_are_declared() {
    let seed = dsl::os_pack::json::to_dsl_value(&dsl::json!({ "spaceId": "sp-1", "confirmed": true }));
    let effective = effective_action_args(&[], &DslValue::Object(Vec::new()), Some(&seed));
    assert_eq!(effective.get("spaceId"), Some(&DslValue::String("sp-1".into())));
    assert_eq!(effective.get("confirmed"), Some(&DslValue::Bool(true)));
}

/// 🔽️ Shares the schema-first choice cases with the TypeScript AJV enum oracle.
#[semio_framework_async_macros::async_test]
async fn unresolved_action_choices_follow_neutral_catalog_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️action-argument-resolution/🧫️fixtures/🔽️choices/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let options = row["options"].as_array().unwrap().iter().map(|value| ActionArgOption::new(value.as_str().unwrap(), LocalizedLabel::data(value.as_str().unwrap()))).collect();
        let mut def = ActionArgDef::select("kindChoice", LocalizedLabel::data("Kind"), options);
        def.required = row["required"].as_bool().unwrap();
        if let super::ArgSchema::String { format, .. } = &mut def.schema {
            *format = match row["format"].as_str().unwrap() {
                "artifactKind" => Some(super::ArgFormat::ArtifactKind { roles: vec![AppRole::Editor] }),
                "surfaceApp" => Some(super::ArgFormat::SurfaceApp { roles: vec![AppRole::Editor], dialect_arg: "dialect".into() }),
                _ => None,
            };
        }
        let arguments = serde_json::json!({ "kindChoice": row["value"] }).to_string();
        let effective = dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(&arguments).unwrap());
        let unresolved = super::unresolved_action_args(&[def], &effective);
        let expected: Vec<String> = if row["unresolved"].as_bool().unwrap() { vec!["kindChoice".into()] } else { Vec::new() };
        assert_eq!(unresolved, expected, "{}", row["id"]);
    }
    eprintln!("[DEBUG] Action choice validation: Rust neutral=12 shared-TypeScript-AJV-oracle=12");
}

#[semio_framework_async_macros::async_test]
async fn missing_required_args_treats_unset_select_as_missing() {
    let defs = vec![ActionArgDef::select("mode", LocalizedLabel::data("Mode"), vec![ActionArgOption::new("x", LocalizedLabel::data("X"))]).required(), ActionArgDef::toggle("flag", LocalizedLabel::data("Flag")).required()];
    // Nothing staged, no defaults: both required ids are missing.
    let empty = DslValue::Object(Vec::new());
    let effective = effective_action_args(&defs, &empty, None);
    let missing = missing_required_args(&defs, &effective);
    assert!(missing.contains(&"mode".to_string()));
    assert!(missing.contains(&"flag".to_string()));

    let effective = dsl::os_pack::json::to_dsl_value(&dsl::json!({ "mode": "", "flag": false }));
    let missing = missing_required_args(&defs, &effective);
    assert_eq!(missing, vec!["mode".to_string()], "empty-string select is unset; false toggle is set");
}

#[semio_framework_async_macros::async_test]
async fn utility_definition_and_utility_ref_construction() {
    let utility = UtilityDefinition::new("brush", LocalizedLabel::data("Brush"), "paintbrush");
    assert_eq!(utility.id, "brush");
    assert!(!utility.allows_actions_while_active, "default gates actions while active");
    assert_eq!(UtilityRef::new("brush").as_str(), "brush");
    assert_eq!(UtilityRef::from("brush").as_str(), "brush");
}

async fn app_with(actions: Vec<ActionDefinition>, window_actions: Vec<ActionRef>) -> AppDefinition {
    let owned_actions = if window_actions.is_empty() { actions } else { window_actions.iter().filter_map(|action_ref| actions.iter().find(|action| action.id == action_ref.as_str()).cloned()).collect() };
    AppDefinition {
        id: "a".into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.a".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("A"),
        breadcrumb: vec!["semio".into(), "a".into()],
        icon_id: None,
        controller_id: "a".into(),
        modes: Modes::one(crate::ui::ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
        default_mode_id: "edit".into(),
        window_kinds: WindowKinds::one(WindowKindDefinition {
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
            body_key: "a.main".into(),
            surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            icon_id: "pen-tool".into(),
            options: ui_wgpu::wgpu::WindowOptions::default(),
            actions: owned_actions,
            utilities: Vec::new(),
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        }),
        panel_tabs: vec![],
        keybindings: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        interactions: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        terminology_breadcrumbs: std::collections::HashMap::new(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: crate::ConfigSpec::empty().await,
        command_grammar: crate::CommandGrammar::empty().await,
        io: crate::AppIo::default(),
    }
}

#[semio_framework_async_macros::async_test]
async fn resolve_window_actions_explicit_scoping() {
    let app =
        app_with(vec![ActionDefinition::bounded_catalog("add", LocalizedLabel::data("Add"), ActionKind::Mutation), ActionDefinition::bounded_catalog("remove", LocalizedLabel::data("Remove"), ActionKind::Mutation)], vec![ActionRef::new("add")]).await;
    let window = app.window_kinds.first();
    let resolved: Vec<&str> = resolve_window_actions(&app, window).iter().map(|a| a.id.as_str()).collect();
    assert_eq!(resolved, vec!["add"], "window ownership replaces app-level orphan fallback");
}

#[semio_framework_async_macros::async_test]
async fn resolve_window_actions_excludes_history_and_set_active_utility_orphans() {
    let app = app_with(
        vec![
            ActionDefinition::new("undo", LocalizedLabel::data("Undo"), ActionKind::History, "undo-2"),
            crate::ui::set_active_utility_action_definition(),
            ActionDefinition::bounded_catalog("add", LocalizedLabel::data("Add"), ActionKind::Mutation),
        ],
        vec![],
    )
    .await;
    let window = app.window_kinds.first();
    let resolved: Vec<&str> = resolve_window_actions(&app, window).iter().map(|a| a.id.as_str()).collect();
    assert_eq!(resolved, vec!["add"], "history + setActiveUtility are never panel-eligible orphans");
    assert!(!resolved.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
}

//#region 🔖️InteractionTests
/// 🕹️ Minimal one-domain, one-granularity `InteractionDefinition` fixture — mirrors the wave-0
/// `sample_definition()` fixture in `🕹️interaction/🦀️.rs`'s own tests.
async fn sample_interaction_definition(id: &str) -> InteractionDefinition {
    InteractionDefinition {
        id: id.into(),
        label: LocalizedLabel::data(id),
        granularities: vec![GranularityDefinition { id: "node".into(), label: LocalizedLabel::data("Node"), icon_id: "circle".into() }],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
    }
}

#[semio_framework_async_macros::async_test]
async fn interaction_action_definitions_empty_when_app_has_no_interactions() {
    let app = app_with(vec![], vec![]).await;
    assert!(app.interactions.is_empty());
    assert!(interaction_action_definitions(&app).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn interaction_action_definitions_full_set_when_app_has_interactions() {
    let mut app = app_with(vec![], vec![]).await;
    app.interactions = vec![sample_interaction_definition("graph").await];
    let defs = interaction_action_definitions(&app);
    let ids: Vec<&str> = defs.iter().map(|action| action.id.as_str()).collect();
    assert_eq!(ids, vec![INTERACTION_SELECT_ACTION_ID, INTERACTION_HOVER_ACTION_ID, CLEAR_SELECTION_ACTION_ID, SELECT_ALL_ACTION_ID, SET_SELECTION_MODE_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID,]);
    assert!(defs.iter().all(|action| action.kind == ActionKind::Interaction));
    let by_id = |id: &str| defs.iter().find(|action| action.id == id).unwrap();
    assert!(!by_id(INTERACTION_SELECT_ACTION_ID).in_palette, "raw dispatch verb, never in the palette");
    assert!(!by_id(INTERACTION_HOVER_ACTION_ID).in_palette, "raw dispatch verb, never in the palette");
    assert!(by_id(CLEAR_SELECTION_ACTION_ID).in_palette);
    assert!(by_id(SELECT_ALL_ACTION_ID).in_palette);
    assert!(by_id(SET_SELECTION_MODE_ACTION_ID).in_palette);
    assert!(by_id(SET_INTERACTION_GRANULARITY_ACTION_ID).in_palette);
    assert_eq!(by_id(CLEAR_SELECTION_ACTION_ID).keys.as_deref(), Some("escape"));
    assert_eq!(by_id(SELECT_ALL_ACTION_ID).keys.as_deref(), Some("mod+a"));
}

#[semio_framework_async_macros::async_test]
async fn resolve_window_actions_includes_injected_interaction_actions() {
    let mut app = app_with(vec![], vec![]).await;
    app.interactions = vec![sample_interaction_definition("graph").await];
    let actions = interaction_action_definitions(&app);
    app.window_kinds.first_mut().actions = actions;
    let window = app.window_kinds.first();
    let resolved: Vec<&str> = resolve_window_actions(&app, window).iter().map(|a| a.id.as_str()).collect();
    for id in [INTERACTION_SELECT_ACTION_ID, INTERACTION_HOVER_ACTION_ID, CLEAR_SELECTION_ACTION_ID, SELECT_ALL_ACTION_ID, SET_SELECTION_MODE_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID] {
        assert!(resolved.contains(&id), "{id} injected into the owning window but not resolved");
    }
}

#[semio_framework_async_macros::async_test]
async fn action_kind_interaction_round_trips_through_json() {
    let json = serde_json::to_string(&ActionKind::Interaction).unwrap();
    assert_eq!(json, "\"interaction\"");
    assert_eq!(serde_json::from_str::<ActionKind>(&json).unwrap(), ActionKind::Interaction);
}

#[semio_framework_async_macros::async_test]
async fn app_definition_and_window_kind_definition_serde_round_trip_interactions() {
    let mut app = app_with(vec![ActionDefinition::bounded_catalog("noop", LocalizedLabel::data("No operation"), ActionKind::View)], vec![ActionRef::new("noop")]).await;
    app.interactions = vec![sample_interaction_definition("graph").await];
    app.window_kinds.first_mut().interactions = vec![InteractionRef::new("graph")];
    let json = serde_json::to_string(&app).unwrap();
    assert!(json.contains("\"interactions\":[{\"id\":\"graph\""), "{json}");
    let parsed: AppDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, app);
    assert_eq!(parsed.window_kinds.first().interactions, vec![InteractionRef::new("graph")]);
}
/// ⚖️ LAW: an EMPTY collection still reaches the wire as `[]`, never as an absent key.
///
/// The generated TypeScript (`🤖️generated/🪪️manifest.ts`) declares these fields as **required**
/// arrays — `commands: Array<CommandDefinition>`, not `commands?:` — because only
/// field carries it. A `skip_serializing_if = "Vec::is_empty"` therefore handed the host
/// `undefined` where its own types promised an array, and every unguarded `app.commands.some(…)`
/// threw. That is not hypothetical: it is what emptied the Koordinator pane in ticket
/// `26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION` — the demonstrator pushed
/// `setContributions` at `📐️cad`, an app that declares no commands.
///
/// Deserialization stays tolerant (`#[serde(default)]`), so an absent key still parses; it is only
/// the *emitted* form that is now total.
#[semio_framework_async_macros::async_test]
async fn empty_collections_serialize_as_arrays_rather_than_vanishing_from_the_manifest() {
    let app = app_with(vec![], vec![]).await;
    assert!(app.commands.is_empty(), "this law is about the EMPTY case");
    let json = serde_json::to_string(&app).unwrap();
    for key in ["commands", "utilities", "tools", "interactions", "namedLayouts", "terminologies", "tutorials", "dialogs", "mediaInputs", "mediaOutputs", "artifactKinds"] {
        assert!(json.contains(&format!("\"{key}\":[")), "`{key}` must serialize as [] so the required TS array is never undefined — missing from {json}");
    }
    let parsed: AppDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, app);
    let without_keys = serde_json::from_str::<AppDefinition>(&json.replace("\"commands\":[],", "")).expect("an absent key must still deserialize via #[serde(default)]");
    assert!(without_keys.commands.is_empty());
}
//#endregion 🔖️InteractionTests

async fn app_with_modes_and_tools(mut modes: Vec<crate::ui::ModeDefinition>, tools: Vec<crate::ui::ToolDefinition>) -> AppDefinition {
    let mut app = app_with(vec![], vec![]).await;
    let first = modes.remove(0);
    app.modes = Modes::new(first, modes);
    app.tools = tools;
    app
}

#[semio_framework_async_macros::async_test]
async fn resolve_mode_tools_declared_order() {
    let app = app_with_modes_and_tools(
        vec![crate::ui::ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: vec![ToolRef::new("fill").await, ToolRef::new("brush").await], layout_id: None, commands: Vec::new() }],
        vec![crate::ui::ToolDefinition::new("brush", LocalizedLabel::data("Brush"), "paintbrush").await, crate::ui::ToolDefinition::new("fill", LocalizedLabel::data("Fill"), "paint-bucket").await],
    )
    .await;
    let resolved: Vec<&str> = resolve_mode_tools(&app, "edit").iter().map(|t| t.id.as_str()).collect();
    assert_eq!(resolved, vec!["fill", "brush"], "resolves in the mode's declared ref order, not registry order");
}

#[semio_framework_async_macros::async_test]
async fn resolve_mode_tools_isolates_other_modes() {
    let app = app_with_modes_and_tools(
        vec![
            crate::ui::ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: vec![ToolRef::new("fill").await], layout_id: None, commands: Vec::new() },
            crate::ui::ModeDefinition { id: "view".into(), label: LocalizedLabel::data("View"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() },
        ],
        vec![crate::ui::ToolDefinition::new("fill", LocalizedLabel::data("Fill"), "paint-bucket").await],
    )
    .await;
    assert_eq!(resolve_mode_tools(&app, "edit").iter().map(|t| t.id.as_str()).collect::<Vec<_>>(), vec!["fill"]);
    assert!(resolve_mode_tools(&app, "view").is_empty(), "tools are opt-in per mode, no orphan fallback");
    assert!(resolve_mode_tools(&app, "nonexistent").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn resolve_mode_tools_skips_unresolvable_refs() {
    let app = app_with_modes_and_tools(
        vec![crate::ui::ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: vec![ToolRef::new("fill").await, ToolRef::new("ghost").await], layout_id: None, commands: Vec::new() }],
        vec![crate::ui::ToolDefinition::new("fill", LocalizedLabel::data("Fill"), "paint-bucket").await],
    )
    .await;
    let resolved: Vec<&str> = resolve_mode_tools(&app, "edit").iter().map(|t| t.id.as_str()).collect();
    assert_eq!(resolved, vec!["fill"]);
}

#[semio_framework_async_macros::async_test]
async fn resolve_layout_for_mode_prefers_named_then_default_then_none() {
    async fn stack_layout(active: &str) -> ui_wgpu::wgpu::WindowLayout {
        ui_wgpu::wgpu::WindowLayout { root: ui_wgpu::wgpu::WindowLayoutRoot::Stack(ui_wgpu::wgpu::WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: Some(active.into()), children: vec![] }) }
    }
    let mut app = app_with(vec![], vec![]).await;
    app.modes.first_mut().layout_id = Some("named".into());
    app.named_layouts.push(ui_wgpu::wgpu::NamedLayout { id: "named".into(), label: "Named".into(), icon_id: None, layout: stack_layout("main").await, origin: "app".into(), group_path: None });
    app.default_layout = Some(stack_layout("fallback").await);

    assert_eq!(resolve_layout_for_mode(&app, "edit"), Some(stack_layout("main").await), "named layout referenced by the mode wins");

    app.modes.first_mut().layout_id = Some("missing".into());
    assert_eq!(resolve_layout_for_mode(&app, "edit"), Some(stack_layout("fallback").await), "unresolved named layout id falls back to default_layout");

    app.default_layout = None;
    assert_eq!(resolve_layout_for_mode(&app, "edit"), None, "no named layout and no default_layout ⇒ none");
    assert_eq!(resolve_layout_for_mode(&app, "nonexistent"), None, "unknown mode id ⇒ none");
}

#[semio_framework_async_macros::async_test]
async fn resolve_app_label_uses_terminology_override_else_falls_back_to_native_label() {
    let mut app = app_with(vec![], vec![]).await;
    app.terminology_breadcrumbs.insert("de".into(), vec!["semio".into(), "a-de".into()]);
    assert_eq!(resolve_app_breadcrumb(&app, "de"), ["semio".to_string(), "a-de".to_string()]);
    assert_eq!(resolve_app_breadcrumb(&app, "native"), app.breadcrumb.as_slice());
    assert_eq!(resolve_app_breadcrumb(&app, "unregistered"), app.breadcrumb.as_slice());
}

#[semio_framework_async_macros::async_test]
async fn app_window_label_skips_empty_app_named_and_duplicate_trailing_window_labels() {
    let mut app = app_with(vec![], vec![]).await;
    app.label = LocalizedLabel::data("Draw"); // document (from `app_with`) already ends in "a"
    assert_eq!(app_window_label(&app, "native", Locale::En, "Layers"), "semio · a · layers");
    assert_eq!(app_window_label(&app, "native", Locale::En, ""), "semio · a", "empty window label appends nothing");
    assert_eq!(app_window_label(&app, "native", Locale::En, "Draw"), "semio · a", "window label equal to the app label appends nothing");
    assert_eq!(app_window_label(&app, "native", Locale::En, "A"), "semio · a", "window label equal to the document's trailing segment appends nothing");
}

#[semio_framework_async_macros::async_test]
async fn non_empty_vec_index_iter_first_mut_and_try_from() {
    let mut list = NonEmptyVec::new(1i32, vec![2, 3]);
    assert_eq!(list.len(), 3);
    assert_eq!(list[0], 1);
    assert_eq!(list[2], 3);
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
    *list.first_mut() = 10;
    assert_eq!(list[0], 10);

    let from_vec = NonEmptyVec::try_from(vec![9, 8]).unwrap();
    assert_eq!(*from_vec.first(), 9);
    let round_tripped: Vec<i32> = from_vec.into();
    assert_eq!(round_tripped, vec![9, 8]);

    let err = NonEmptyVec::<i32>::try_from(Vec::new()).unwrap_err();
    assert!(err.contains("non-empty"));
}

#[semio_framework_async_macros::async_test]
async fn panel_group_anchor_and_as_str_cover_all_variants() {
    assert_eq!(PanelGroup::Workbench.anchor(), "top-left");
    assert_eq!(PanelGroup::Details.anchor(), "top-right");
    assert_eq!(PanelGroup::Display.anchor(), "bottom-left");
    assert_eq!(PanelGroup::Settings.anchor(), "bottom-right");
    assert_eq!(PanelGroup::Workbench.as_str(), "workbench");
    assert_eq!(PanelGroup::Settings.as_str(), "settings");
}

#[semio_framework_async_macros::async_test]
async fn panel_tab_kind_id_str_covers_framework_and_app_variants() {
    assert_eq!(PanelTabKind::WorkbenchCategory.id_str(), "framework.category.workbench");
    assert_eq!(PanelTabKind::DisplayWindows.id_str(), "framework.display.windows");
    assert_eq!(PanelTabKind::App("puzzle.catalogue".into()).id_str(), "puzzle.catalogue");
    let tab = PanelTabDefinition { kind: PanelTabKind::App("puzzle.catalogue".into()), label: LocalizedLabel::data("Catalogue"), group: PanelGroup::Workbench, body_key: Some("puzzle.catalogue".into()), children: Vec::new() };
    assert_eq!(tab.id(), "puzzle.catalogue");
}

#[semio_framework_async_macros::async_test]
async fn action_definition_requires_and_serializes_args_field() {
    let action = ActionDefinition::bounded_catalog("x", LocalizedLabel::data("X"), ActionKind::Mutation);
    let json = serde_json::to_value(&action).unwrap();
    assert_eq!(json["args"], json!([]));
    assert!(
        serde_json::from_value::<ActionDefinition>(json!({
            "id": "x",
            "label": {"native": {"en": "X", "de": "X"}, "reuse": {"en": "X", "de": "X"}},
            "kind": "operation",
            "inPalette": true
        }))
        .is_err()
    );
}

#[semio_framework_async_macros::async_test]
async fn window_kind_deserializes_without_utilities_field() {
    let window: WindowKindDefinition = serde_json::from_str(r#"{"id":"main","label":{"native":{"en":"Main","de":"Main"},"reuse":{"en":"Main","de":"Main"}},"bodyKey":"a.main","surfaceKind":"canvas-2d","iconId":"pen-tool"}"#).unwrap();
    assert!(window.utilities.is_empty());
    assert!(window.actions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn action_arg_control_serializes_tagged() {
    let control = ActionArgControl::Select { options: vec![ActionArgOption::new("x", LocalizedLabel::data("X"))] };
    let json = serde_json::to_string(&control).unwrap();
    assert!(json.contains("\"kind\":\"select\""), "tagged with kind: {json}");
    let round: ActionArgControl = serde_json::from_str(&json).unwrap();
    assert_eq!(round, control);
}

#[semio_framework_async_macros::async_test]
async fn is_element_id_accepts_dotted_camel_case_and_rejects_the_rest() {
    assert!(is_element_id("framework.navbar"));
    assert!(is_element_id("ui.window.main.action.addLayer"));
    assert!(is_element_id("brush"));
    assert!(!is_element_id(""));
    assert!(!is_element_id("framework.display.save-label"));
    assert!(!is_element_id("Framework.navbar"));
    assert!(!is_element_id("framework..navbar"));
    assert!(!is_element_id("framework.navbar."));
}

#[semio_framework_async_macros::async_test]
async fn element_id_segment_normalizes_and_is_idempotent() {
    assert_eq!(element_id_segment("world-orbit-projection"), "worldOrbitProjection");
    assert_eq!(element_id_segment("Some Name"), "someName");
    assert_eq!(element_id_segment("myUtilityId"), "myUtilityId");
    assert_eq!(element_id_segment("addLayer"), element_id_segment(&element_id_segment("addLayer")));
}

#[semio_framework_async_macros::async_test]
async fn child_element_id_suffixes_and_normalizes_segments() {
    assert_eq!(child_element_id("ui.chat", &["send"]), "ui.chat.send");
    assert_eq!(child_element_id("ui.chat", &["message-row"]), "ui.chat.messageRow");
    assert_eq!(child_element_id("ui.tree", &["row", "3"]), "ui.tree.row.3");
}

#[semio_framework_async_macros::async_test]
async fn introduction_step_serde_defaults() {
    let step: IntroductionStepDefinition =
        serde_json::from_str(r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#).unwrap();
    assert_eq!(step.introduce, None);
    assert!(step.show.is_empty());
    let json = serde_json::to_string(&step).unwrap();
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, step);

    let with_targets = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("The Viewport"), LocalizedLabel::data("…")).introduce(window_element_id("puzzle3d-main")).show(vec![window_element_id("puzzle3d-secondary")]);
    let json = serde_json::to_string(&with_targets).unwrap();
    assert!(json.contains("\"introduce\":\"framework.window.puzzle3dMain\""), "{json}");
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_targets);
}

#[semio_framework_async_macros::async_test]
async fn element_id_authoring_helpers() {
    assert_eq!(window_element_id("puzzle3d-main"), "framework.window.puzzle3dMain");
    assert_eq!(panel_tab_element_id("framework.panel.catalogue"), "framework.panelTab.framework.panel.catalogue");
    assert_eq!(panel_tab_first_draggable_element_id("framework.panel.catalogue"), "framework.panelTab.framework.panel.catalogue.firstDraggable");
    assert!(is_element_id(UI_NAVBAR_ELEMENT_ID));
    assert!(is_element_id(UI_FOOTER_ELEMENT_ID));
    assert!(is_element_id(&window_element_id("puzzle3d-main")));
    assert!(is_element_id(&panel_tab_element_id("framework.panel.catalogue")));
    assert!(is_element_id(&panel_tab_first_draggable_element_id("framework.panel.catalogue")));
}

#[semio_framework_async_macros::async_test]
async fn introduction_interaction_kind_round_trips_tagged() {
    for (kind, tag) in [
        (IntroductionInteractionKind::Action(ActionRef::new("add")), "action"),
        (IntroductionInteractionKind::Utility(UtilityRef::new("brush")), "utility"),
        (IntroductionInteractionKind::Tool(ToolRef::new("fill").await), "tool"),
        (IntroductionInteractionKind::Panel("framework.panel.catalogue".into()), "panel"),
        (IntroductionInteractionKind::Expand("puzzle3d-play-kinds.objects".into()), "expand"),
        (IntroductionInteractionKind::Pan("puzzle3d-main".into()), "pan"),
        (IntroductionInteractionKind::Zoom("puzzle3d-main".into()), "zoom"),
        (IntroductionInteractionKind::Orbit("puzzle3d-main".into()), "orbit"),
    ] {
        let json = serde_json::to_string(&kind).unwrap();
        assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
        let round: IntroductionInteractionKind = serde_json::from_str(&json).unwrap();
        assert_eq!(round, kind);
    }
}

#[semio_framework_async_macros::async_test]
async fn introduction_interaction_round_trips_and_defaults() {
    let interaction = IntroductionInteraction::zoom("puzzle3d-main", "Zoom in").await;
    assert_eq!(interaction.celebrate, None);
    let json = serde_json::to_string(&interaction).unwrap();
    assert!(!json.contains("celebrate"), "{json}");
    let round: IntroductionInteraction = serde_json::from_str(&json).unwrap();
    assert_eq!(round, interaction);

    let with_celebrate = IntroductionInteraction::pan("puzzle3d-main", "Pan").await.celebrate(window_element_id("puzzle3d-main")).await;
    let json = serde_json::to_string(&with_celebrate).unwrap();
    assert!(json.contains("\"celebrate\":\"framework.window.puzzle3dMain\""), "{json}");
    let round: IntroductionInteraction = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_celebrate);

    let step: IntroductionStepDefinition =
        serde_json::from_str(r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#).unwrap();
    assert!(step.interactions.is_empty());
    assert!(!step.ordered);

    let with_interactions = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…")).interact_ordered(vec![
        IntroductionInteraction::zoom("puzzle3d-main", "Zoom").await,
        IntroductionInteraction::pan("puzzle3d-main", "Pan").await,
        IntroductionInteraction::orbit("puzzle3d-main", "Orbit").await,
    ]);
    assert!(with_interactions.ordered);
    assert_eq!(with_interactions.interactions.len(), 3);
    let json = serde_json::to_string(&with_interactions).unwrap();
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_interactions);
}

#[semio_framework_async_macros::async_test]
async fn introduction_point_round_trips_tagged_camel_case() {
    for (point, tag) in [
        (IntroductionPoint::Element { id: "transform".into(), offset: None }, "element"),
        (IntroductionPoint::Element { id: "transform".into(), offset: Some([0.25, 0.75]) }, "element"),
        (IntroductionPoint::Screen { x: 10.0, y: 20.0 }, "screen"),
        (IntroductionPoint::ScreenNormalized { x: 0.5, y: 0.5 }, "screenNormalized"),
        (IntroductionPoint::Window { id: window_element_id("puzzle3d-main"), x: 40.0, y: 60.0 }, "window"),
        (IntroductionPoint::WindowNormalized { id: window_element_id("puzzle3d-main"), x: 0.5, y: 0.55 }, "windowNormalized"),
        (IntroductionPoint::Scene { id: window_element_id("puzzle3d-main"), position: [1.0, 2.0, 3.0] }, "scene"),
        (IntroductionPoint::Canvas { id: window_element_id("puzzle3d-main"), x: 12.0, y: 34.0 }, "canvas"),
        (IntroductionPoint::entity(window_element_id("puzzle3d-main"), "vortex", "seed-left-001:v0").await, "entity"),
        (IntroductionPoint::any_entity(window_element_id("puzzle3d-main"), "vortex").await, "entity"),
        (IntroductionPoint::Entity { id: window_element_id("puzzle3d-main"), domain: "node".into(), entity: "add".into(), offset: Some([0.25, 0.75]) }, "entity"),
        (IntroductionPoint::curve(window_element_id("puzzle3d-main"), "attraction", "a1", 0.5).await, "curve"),
        (IntroductionPoint::domain_value(window_element_id("puzzle3d-main"), "slider", "fillCount", 3.0).await, "domain"),
    ] {
        let json = serde_json::to_string(&point).unwrap();
        assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
        let round: IntroductionPoint = serde_json::from_str(&json).unwrap();
        assert_eq!(round, point);
    }
    // 🏷️ "*" (any-entity wildcard) must round-trip byte-for-byte, not get normalized away.
    let wildcard = IntroductionPoint::any_entity(window_element_id("puzzle3d-main"), "vortex").await;
    let json = serde_json::to_string(&wildcard).unwrap();
    assert!(json.contains("\"entity\":\"*\""), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn introduction_gesture_round_trips_tagged_camel_case() {
    let at = IntroductionPoint::Element { id: "tool.fill".into(), offset: None };
    for (gesture, tag) in [
        (IntroductionGesture::LeftClick { at: at.clone() }, "leftClick"),
        (IntroductionGesture::RightClick { at: at.clone() }, "rightClick"),
        (IntroductionGesture::DoubleClick { at: at.clone() }, "doubleClick"),
        (IntroductionGesture::Drag { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Left, modifiers: vec![] }, "drag"),
        (IntroductionGesture::Scroll { at: at.clone(), delta_y: 100.0 }, "scroll"),
        (IntroductionGesture::Orbit { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Right, modifiers: vec![IntroductionKeyModifier::Alt] }, "orbit"),
    ] {
        let json = serde_json::to_string(&gesture).unwrap();
        assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
        let round: IntroductionGesture = serde_json::from_str(&json).unwrap();
        assert_eq!(round, gesture);
    }

    // 🐢️ `rename_all` on an enum renames only the variant tag, not fields *within* a struct variant —
    // `rename_all_fields` is required too, or this field would silently serialize snake_case
    // (`delta_y`) and desync from the generated TS type's camelCase `deltaY` (see `UiDirtyScope`).
    let scroll_json = serde_json::to_string(&IntroductionGesture::Scroll { at, delta_y: 100.0 }).unwrap();
    assert!(scroll_json.contains("\"deltaY\":100.0"), "{scroll_json}");
    assert!(!scroll_json.contains("delta_y"), "{scroll_json}");
}

#[semio_framework_async_macros::async_test]
async fn introduction_gesture_drag_orbit_default_button_and_modifiers() {
    let at = IntroductionPoint::Element { id: "puzzle3d-main".into(), offset: None };
    let drag: IntroductionGesture = serde_json::from_str(r#"{"kind":"drag","from":{"kind":"element","id":"puzzle3d-main"},"to":{"kind":"element","id":"puzzle3d-main"}}"#).unwrap();
    assert_eq!(drag, IntroductionGesture::Drag { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Left, modifiers: vec![] });
    // ⚖️ Defaults are still INFERRED on the way in (the input literal above names neither field),
    // but they are always WRITTEN on the way out: `🤖️generated/🪪️manifest.ts` declares both
    // `button: IntroductionPointerButton` and `modifiers: Array<IntroductionKeyModifier>` as
    // required, so omitting a defaulted value handed the host `undefined` where its own types
    // promised a value. Asserting the omission — as this test previously did — pinned the defect.
    let drag_json = serde_json::to_string(&drag).unwrap();
    assert!(drag_json.contains("\"button\":\"left\""), "{drag_json}");
    assert!(drag_json.contains("\"modifiers\":[]"), "{drag_json}");

    let orbit: IntroductionGesture = serde_json::from_str(r#"{"kind":"orbit","from":{"kind":"element","id":"puzzle3d-main"},"to":{"kind":"element","id":"puzzle3d-main"}}"#).unwrap();
    assert_eq!(orbit, IntroductionGesture::Orbit { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Right, modifiers: vec![IntroductionKeyModifier::Alt] });
    let orbit_json = serde_json::to_string(&orbit).unwrap();
    assert!(orbit_json.contains("\"button\":\"right\""), "{orbit_json}");
    assert!(orbit_json.contains("\"modifiers\":[\"alt\"]"), "{orbit_json}");

    let middle_drag = IntroductionGesture::Drag { from: at.clone(), to: at, button: IntroductionPointerButton::Middle, modifiers: vec![] };
    let middle_json = serde_json::to_string(&middle_drag).unwrap();
    assert!(middle_json.contains("\"button\":\"middle\""), "{middle_json}");
    let round: IntroductionGesture = serde_json::from_str(&middle_json).unwrap();
    assert_eq!(round, middle_drag);
}

#[semio_framework_async_macros::async_test]
async fn introduction_demonstration_round_trips_and_defaults() {
    let at = IntroductionPoint::Element { id: "transform".into(), offset: None };
    let demo = IntroductionDemonstration::left_click(at.clone()).await;
    assert_eq!(demo.cursor, None);
    let json = serde_json::to_string(&demo).unwrap();
    assert!(!json.contains("cursor"), "{json}");
    let round: IntroductionDemonstration = serde_json::from_str(&json).unwrap();
    assert_eq!(round, demo);

    let with_cursor = IntroductionDemonstration { gesture: IntroductionGesture::Drag { from: at.clone(), to: at, button: IntroductionPointerButton::Left, modifiers: vec![] }, cursor: Some(IntroductionCursor::Grabbing) };
    let json = serde_json::to_string(&with_cursor).unwrap();
    assert!(json.contains("\"cursor\":\"grabbing\""), "{json}");
    let round: IntroductionDemonstration = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_cursor);

    let step: IntroductionStepDefinition =
        serde_json::from_str(r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#).unwrap();
    assert!(step.demonstrations.is_empty());
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains("\"demonstrations\":[]"), "{json}");

    // 🎬️ A step can sequence several demonstrations (e.g. zoom, then pan, then orbit).
    let with_demos = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…")).demonstrate(vec![
        IntroductionDemonstration::scroll(IntroductionPoint::Screen { x: 400.0, y: 300.0 }, -100.0).await,
        IntroductionDemonstration::drag(IntroductionPoint::Screen { x: 300.0, y: 300.0 }, IntroductionPoint::Screen { x: 400.0, y: 320.0 }).await,
        IntroductionDemonstration::orbit(IntroductionPoint::Screen { x: 300.0, y: 300.0 }, IntroductionPoint::Screen { x: 500.0, y: 300.0 }).await,
    ]);
    assert_eq!(with_demos.demonstrations.len(), 3);
    let json = serde_json::to_string(&with_demos).unwrap();
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_demos);
}

//#region 🔖️TutorialTests
async fn minimal_tutorial() -> TutorialDefinition {
    TutorialDefinition {
        id: "welcome-tour".into(),
        title: LocalizedLabel::data("Welcome Tour"),
        description: None,
        duration_ms: 10_000,
        chapters: vec![TutorialChapter { id: "start".into(), at: 0, title: LocalizedLabel::data("Start"), body: None }],
        base: TutorialBase { document_dsl: None, example_id: Some("concrete-forest".into()), ui: TutorialUiSnapshot::default(), cameras: vec![] },
        tracks: TutorialTracks::default(),
        recorded_at: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn tutorial_definition_serde_defaults() {
    let json = r#"{"id":"t","title":{"native":{"en":"T","de":"T"},"reuse":{"en":"T","de":"T"}},"durationMs":1000,"base":{"ui":{}},"tracks":{}}"#;
    let def: TutorialDefinition = serde_json::from_str(json).unwrap();
    assert!(def.description.is_none());
    assert!(def.chapters.is_empty());
    assert!(def.tracks.narration.is_empty());
    assert!(def.tracks.document.is_empty());
    assert!(def.base.cameras.is_empty());
    let round = serde_json::to_string(&def).unwrap();
    let round: TutorialDefinition = serde_json::from_str(&round).unwrap();
    assert_eq!(round, def);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_asset_src_round_trips_tagged_camel_case() {
    for asset in
        [TutorialAssetSrc::Url { url: "https://example.test/clip.webm".into() }, TutorialAssetSrc::Blob { hash: "abc123".into(), size: 42, media_type: "video/webm".into() }, TutorialAssetSrc::DataUrl { data: "data:audio/webm;base64,AA==".into() }]
    {
        let json = serde_json::to_string(&asset).unwrap();
        assert!(json.contains("\"kind\":"), "{json}");
        let round: TutorialAssetSrc = serde_json::from_str(&json).unwrap();
        assert_eq!(round, asset);
    }
    let json = serde_json::to_string(&TutorialAssetSrc::Blob { hash: "abc".into(), size: 1, media_type: "video/webm".into() }).unwrap();
    assert!(json.contains("\"mediaType\""), "field must be camelCase: {json}");
}

#[semio_framework_async_macros::async_test]
async fn tutorial_event_kind_round_trips_tagged_camel_case() {
    let action = TutorialEventKind::Action { action: "addObjectKind".into(), args: Some(dsl::os_pack::json::to_dsl_value(&dsl::json!({"kindId": "beam"}))) };
    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"kind\":\"action\""), "{json}");
    let round: TutorialEventKind = serde_json::from_str(&json).unwrap();
    assert_eq!(round, action);

    let key = TutorialEventKind::Key { keys: "mod+z".into() };
    let json = serde_json::to_string(&key).unwrap();
    let round: TutorialEventKind = serde_json::from_str(&json).unwrap();
    assert_eq!(round, key);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_ui_change_round_trips_tagged_camel_case() {
    let change = TutorialUiChange::ActiveUtility { window_id: "puzzle3d-main".into(), utility_id: Some("transform".into()) };
    let json = serde_json::to_string(&change).unwrap();
    assert!(json.contains("\"windowId\":\"puzzle3d-main\""), "field must be camelCase: {json}");
    assert!(json.contains("\"utilityId\":\"transform\""), "field must be camelCase: {json}");
    let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
    assert_eq!(round, change);

    let tree = TutorialUiChange::TreeExpansion { id: "puzzle3d-play-kinds.objects".into(), expanded: true };
    let json = serde_json::to_string(&tree).unwrap();
    let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
    assert_eq!(round, tree);

    let selection = TutorialUiChange::Selection { domain_id: "mesh".into(), granularity: "face".into(), ids: vec!["f1".into(), "f2".into()] };
    let json = serde_json::to_string(&selection).unwrap();
    assert!(json.contains("\"domainId\":\"mesh\""), "field must be camelCase: {json}");
    assert!(json.contains("\"granularity\":\"face\""), "{json}");
    let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
    assert_eq!(round, selection);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_artifact_event_kind_round_trips_tagged_camel_case() {
    let edit = TutorialArtifactEventKind::Edit {
        forwards: vec![dsl::os_pack::json::to_dsl_value(&dsl::json!({"op": "translate"}))],
        backwards: vec![dsl::os_pack::json::to_dsl_value(&dsl::json!({"op": "translate", "inverse": true}))],
        description: Some("Move object".into()),
        coalesce_key: Some("camera".into()),
    };
    let json = serde_json::to_string(&edit).unwrap();
    assert!(json.contains("\"kind\":\"edit\""), "{json}");
    assert!(json.contains("\"coalesceKey\":\"camera\""), "field must be camelCase: {json}");
    let round: TutorialArtifactEventKind = serde_json::from_str(&json).unwrap();
    assert_eq!(round, edit);

    let undo = TutorialArtifactEventKind::Undo;
    let json = serde_json::to_string(&undo).unwrap();
    assert_eq!(json, r#"{"kind":"undo"}"#);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_state_round_trips_tagged_camel_case() {
    let orbit = TutorialCameraState::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(50.0) };
    let json = serde_json::to_string(&orbit).unwrap();
    assert!(json.contains("\"kind\":\"orbit\""), "{json}");
    let round: TutorialCameraState = serde_json::from_str(&json).unwrap();
    assert_eq!(round, orbit);

    let canvas = TutorialCameraState::Canvas { x: 1.0, y: 2.0, zoom: 3.0 };
    let json = serde_json::to_string(&canvas).unwrap();
    assert!(json.contains("\"kind\":\"canvas\""), "{json}");
    let round: TutorialCameraState = serde_json::from_str(&json).unwrap();
    assert_eq!(round, canvas);
}

#[semio_framework_async_macros::async_test]
async fn validate_tutorial_rejects_unsorted_and_out_of_range_tracks() {
    let mut def = minimal_tutorial().await;
    def.tracks.narration = vec![
        TutorialNarrationCue { id: "b".into(), at: 500, duration_ms: 100, text: LocalizedLabel::data("b"), audio: None, voice: None, rate: 1.0, captions: vec![] },
        TutorialNarrationCue { id: "a".into(), at: 100, duration_ms: 100, text: LocalizedLabel::data("a"), audio: None, voice: None, rate: 1.0, captions: vec![] },
    ];
    assert!(validate_tutorial(&def).is_err(), "unsorted narration must be rejected");

    let mut def = minimal_tutorial().await;
    def.tracks.narration = vec![TutorialNarrationCue { id: "a".into(), at: 999_999, duration_ms: 100, text: LocalizedLabel::data("a"), audio: None, voice: None, rate: 1.0, captions: vec![] }];
    assert!(validate_tutorial(&def).is_err(), "entry beyond durationMs must be rejected");

    let mut def = minimal_tutorial().await;
    def.chapters.push(TutorialChapter { id: "start".into(), at: 0, title: LocalizedLabel::data("Dup"), body: None });
    assert!(validate_tutorial(&def).is_err(), "duplicate chapter id must be rejected");

    let mut def = minimal_tutorial().await;
    def.base.cameras.push(TutorialCameraKeyframe { at: 5, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::default() });
    assert!(validate_tutorial(&def).is_err(), "base camera keyframe must be at == 0");

    assert!(validate_tutorial(&minimal_tutorial().await).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_interpolation_lerps_position_and_target() {
    let prev = TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Orbit { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(40.0) }, easing: TutorialEasing::Linear };
    let next = TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Orbit { position: [10.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(60.0) }, easing: TutorialEasing::Linear };
    let mid = interpolate_tutorial_camera(&prev, &next, 500.0);
    match mid {
        TutorialCameraState::Orbit { position, fov, .. } => {
            assert!((position[0] - 5.0).abs() < 1e-9, "expected midpoint lerp, got {position:?}");
            assert_eq!(fov, Some(50.0));
        }
        other => panic!("expected Orbit, got {other:?}"),
    }
    let start = interpolate_tutorial_camera(&prev, &next, 0.0);
    assert_eq!(start, prev.camera);
    let end = interpolate_tutorial_camera(&prev, &next, 1000.0);
    assert_eq!(end, next.camera);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_interpolation_zooms_in_log_space() {
    let prev = TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Linear };
    let next = TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 4.0 }, easing: TutorialEasing::Linear };
    let mid = interpolate_tutorial_camera(&prev, &next, 500.0);
    match mid {
        TutorialCameraState::Canvas { zoom, .. } => assert!((zoom - 2.0).abs() < 1e-9, "log-space midpoint of 1..4 is 2, got {zoom}"),
        other => panic!("expected Canvas, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_interpolation_hold_snaps_at_keyframe() {
    let prev = TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Hold };
    let next = TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 4.0 }, easing: TutorialEasing::Hold };
    assert_eq!(interpolate_tutorial_camera(&prev, &next, 999.0), prev.camera);
    assert_eq!(interpolate_tutorial_camera(&prev, &next, 1000.0), next.camera);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_at_holds_first_pose_before_first_keyframe_and_last_pose_after() {
    let mut def = minimal_tutorial().await;
    def.tracks.camera = vec![
        TutorialCameraKeyframe { at: 100, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Linear },
        TutorialCameraKeyframe { at: 900, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 9.0 }, easing: TutorialEasing::Linear },
    ];
    assert_eq!(tutorial_camera_at(&def, "w", 0.0), Some(TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }));
    assert_eq!(tutorial_camera_at(&def, "w", 10_000.0), Some(TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 9.0 }));
    assert_eq!(tutorial_camera_at(&def, "other-window", 500.0), None);
}

#[semio_framework_async_macros::async_test]
async fn compose_tutorial_ui_applies_snapshot_then_deltas() {
    let mut def = minimal_tutorial().await;
    def.base.ui.active_tool_id = Some("fill".into());
    def.tracks.ui = vec![
        TutorialUiKeyframe { at: 100, sample: TutorialUiSample::Snapshot { state: Box::new(TutorialUiSnapshot { active_mode_id: Some("edit".into()), ..Default::default() }) } },
        TutorialUiKeyframe { at: 200, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveTool { id: Some("brush".into()) }] } },
        TutorialUiKeyframe { at: 300, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::PanelTab { group: "top-left".into(), tab_id: Some("catalogue".into()) }] } },
        TutorialUiKeyframe { at: 400, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::Selection { domain_id: "mesh".into(), granularity: "face".into(), ids: vec!["f1".into()] }] } },
    ];
    // Before any sample: the base snapshot alone.
    let at_0 = compose_tutorial_ui(&def, 0.0);
    assert_eq!(at_0.active_tool_id, Some("fill".into()));
    assert_eq!(at_0.active_mode_id, None);
    // After the snapshot but before its deltas.
    let at_100 = compose_tutorial_ui(&def, 100.0);
    assert_eq!(at_100.active_mode_id, Some("edit".into()));
    assert_eq!(at_100.active_tool_id, None, "snapshot replaces the base wholesale");
    // After one delta.
    let at_200 = compose_tutorial_ui(&def, 250.0);
    assert_eq!(at_200.active_tool_id, Some("brush".into()));
    // After both deltas.
    let at_300 = compose_tutorial_ui(&def, 300.0);
    assert_eq!(at_300.active_tool_id, Some("brush".into()));
    assert_eq!(at_300.active_panel_tab_by_group.get("top-left"), Some(&"catalogue".to_string()));
    // After the selection delta: the framework-owned domain selection lands in `interaction_selection`.
    let at_400 = compose_tutorial_ui(&def, 400.0);
    let selection = at_400.interaction_selection.get("mesh").expect("mesh domain selection");
    assert_eq!(selection.granularity, "face");
    assert_eq!(selection.ids, vec!["f1".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_document_track_language_neutral_serde_parity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎞️tutorial-document-track.json")).unwrap();
    let mut def = minimal_tutorial().await;
    def.tracks.document = serde_json::from_value(fixture["document"].clone()).unwrap();
    assert_eq!(serde_json::to_value(&def.tracks.document).unwrap(), fixture["document"]);
    let tracks = serde_json::to_value(&def.tracks).unwrap();
    assert_eq!(tracks.get("document"), Some(&fixture["document"]));
    assert!(tracks.get("artifact").is_none());
    for vector in fixture["cases"].as_array().unwrap() {
        let slice = tutorial_slice(&def, vector["from"].as_f64().unwrap(), vector["to"].as_f64().unwrap());
        assert_eq!(slice.forward, vector["forward"].as_bool().unwrap());
        assert_eq!(serde_json::to_value(slice.document.iter().map(|event| event.at).collect::<Vec<_>>()).unwrap(), vector["expectedAt"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn tutorial_slice_forward_and_reverse_cross_artifact_events() {
    let mut def = minimal_tutorial().await;
    def.tracks.document = vec![
        TutorialArtifactEvent {
            at: 100,
            kind: TutorialArtifactEventKind::Edit {
                forwards: vec![dsl::os_pack::json::to_dsl_value(&dsl::json!({"op": "add", "id": "a"}))],
                backwards: vec![dsl::os_pack::json::to_dsl_value(&dsl::json!({"op": "remove", "id": "a"}))],
                description: None,
                coalesce_key: None,
            },
        },
        TutorialArtifactEvent {
            at: 200,
            kind: TutorialArtifactEventKind::Edit {
                forwards: vec![dsl::os_pack::json::to_dsl_value(&dsl::json!({"op": "add", "id": "b"}))],
                backwards: vec![dsl::os_pack::json::to_dsl_value(&dsl::json!({"op": "remove", "id": "b"}))],
                description: None,
                coalesce_key: None,
            },
        },
    ];
    let forward = tutorial_slice(&def, 0.0, 250.0);
    assert!(forward.forward);
    assert_eq!(forward.document.len(), 2);
    let TutorialArtifactEventKind::Edit { forwards, .. } = &forward.document[0].kind else { panic!("expected Edit") };
    assert_eq!(forwards[0].get("id").and_then(DslValue::as_str), Some("a"), "forward order applies oldest-first");

    let backward = tutorial_slice(&def, 250.0, 0.0);
    assert!(!backward.forward);
    assert_eq!(backward.document.len(), 2);
    let TutorialArtifactEventKind::Edit { backwards, .. } = &backward.document[0].kind else { panic!("expected Edit") };
    assert_eq!(backwards[0].get("id").and_then(DslValue::as_str), Some("b"), "backward order unwinds newest-first");

    let empty = tutorial_slice(&def, 250.0, 250.0);
    assert!(empty.document.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn tutorial_slice_partitions_events_artifact_and_ui_by_track() {
    let mut def = minimal_tutorial().await;
    def.tracks.events = vec![TutorialEvent { at: 50, kind: TutorialEventKind::Action { action: "setFillCount".into(), args: None } }];
    def.tracks.ui = vec![TutorialUiKeyframe { at: 50, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveTool { id: Some("fill".into()) }] } }];
    let slice = tutorial_slice(&def, 0.0, 100.0);
    assert_eq!(slice.events.len(), 1);
    assert_eq!(slice.ui_changes.len(), 1);
    assert!(slice.document.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn start_tutorial_action_definition_offers_declared_tutorials_as_select_options() {
    let action = start_tutorial_action_definition(std::slice::from_ref(&minimal_tutorial().await));
    assert_eq!(action.id, START_TUTORIAL_ACTION_ID);
    assert!(!action.in_palette, "shell owns palette discovery via the dedicated Play Tutorial command");
    assert_eq!(action.args.len(), 1);
    assert!(action.args[0].required);
    match action.args[0].control() {
        ActionArgControl::Select { options } => {
            assert_eq!(options.len(), 1);
            assert_eq!(options[0].value, "welcome-tour");
        }
        other => panic!("expected Select control, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn record_tutorial_action_definition_is_shell_intercepted_and_out_of_palette() {
    let action = record_tutorial_action_definition();
    assert_eq!(action.id, RECORD_TUTORIAL_ACTION_ID);
    assert!(!action.in_palette);
    assert_eq!(action.kind, ActionKind::View);
}
//#endregion 🔖️TutorialTests

#[semio_framework_async_macros::async_test]
async fn dialog_definition_round_trips_camel_case_with_defaults() {
    let dialog = DialogDefinition::new("confirm-delete", LocalizedLabel::data("Delete?"), ActionRef::new("deleteSelection"));
    let json = serde_json::to_string(&dialog).unwrap();
    assert!(json.contains("\"args\":[]"), "{json}");
    assert!(json.contains("\"submitAction\":\"deleteSelection\""), "{json}");
    assert!(json.contains("\"submitLabel\":{\"native\":{\"de\":\"OK\",\"en\":\"OK\"}"), "{json}");
    assert!(!json.contains("cancelAction"), "omitted when unset: {json}");
    let round: DialogDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, dialog);
}

#[semio_framework_async_macros::async_test]
async fn dialog_definition_builder_chain() {
    let dialog = DialogDefinition::new("addObject", LocalizedLabel::data("Add Object"), ActionRef::new("addObjectKind"))
        .body(LocalizedLabel::data("Choose a kind"))
        .args(vec![ActionArgDef::text("objectKind", LocalizedLabel::data("Kind"))])
        .submit_label(LocalizedLabel::data("Add"))
        .cancel_label(LocalizedLabel::data("Nevermind"))
        .on_cancel(ActionRef::new("closeDialog"));
    assert_eq!(dialog.body.as_ref().map(|b| b.resolve(Terminology::Native, Locale::En)), Some("Choose a kind"));
    assert_eq!(dialog.args.len(), 1);
    assert_eq!(dialog.submit_label.resolve(Terminology::Native, Locale::En), "Add");
    assert_eq!(dialog.cancel_label.as_ref().map(|c| c.resolve(Terminology::Native, Locale::En)), Some("Nevermind"));
    assert_eq!(dialog.cancel_action, Some(ActionRef::new("closeDialog")));
}

#[semio_framework_async_macros::async_test]
async fn command_definition_round_trips_camel_case_with_defaults() {
    let command = CommandDefinition::bounded_catalog("setThemeId", LocalizedLabel::data("Set Theme"), "appearance", ActionKind::Shell).with_keybinding(PlatformKeybinding::for_platform("mod+shift+t", Platform::MacOs));
    let json = serde_json::to_string(&command).unwrap();
    assert!(json.contains("\"args\":[]"), "{json}");
    assert!(json.contains("\"category\":\"appearance\""), "{json}");
    assert!(json.contains("\"kind\":\"shell\""), "{json}");
    assert!(!json.contains("\"scope\""), "{json}");
    assert!(json.contains("\"inPalette\":true"), "{json}");
    assert!(json.contains("\"keybindings\":[{\"chord\":\"mod+shift+t\",\"platform\":\"macOs\"}]"), "{json}");
    let round: CommandDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, command);
}

#[semio_framework_async_macros::async_test]
async fn command_and_action_invocations_round_trip_owner_qualified_addresses() {
    let command = CommandInvocation {
        address: CommandAddress { owner: CommandOwnerAddress::Mode { plugin_id: "flow".into(), app_id: "flow".into(), mode_id: "generate".into() }, command_id: "addGeneration".into() },
        arguments: [("name".into(), dsl::os_pack::json::to_dsl_value(&dsl::json!("A")))].into_iter().collect(),
    };
    let command_json = dsl::os_pack::json::to_json_string(&command);
    assert_eq!(command_json, r#"{"address":{"owner":{"mode":{"pluginId":"flow","appId":"flow","modeId":"generate"}},"commandId":"addGeneration"},"arguments":{"name":"A"}}"#);
    assert_eq!(dsl::os_pack::json::from_json_str::<CommandInvocation>(&command_json).unwrap(), command);

    let action = ActionInvocation {
        address: ActionAddress { plugin_id: "flow".into(), app_id: "flow".into(), mode_id: "edit".into(), window_kind_id: "main".into(), window_instance_id: "main-1".into(), action_id: "select".into() },
        arguments: [("id".into(), dsl::os_pack::json::to_dsl_value(&dsl::json!("node-1")))].into_iter().collect(),
    };
    let action_json = dsl::os_pack::json::to_json_string(&action);
    assert!(action_json.contains("\"windowInstanceId\":\"main-1\""), "{action_json}");
    assert_eq!(dsl::os_pack::json::from_json_str::<ActionInvocation>(&action_json).unwrap(), action);

    let os = OsDefinition { commands: vec![CommandDefinition::bounded_catalog("toggleFullscreen", LocalizedLabel::data("Toggle Full Screen"), "window", ActionKind::Shell)] };
    assert_eq!(serde_json::from_str::<OsDefinition>(&serde_json::to_string(&os).unwrap()).unwrap(), os);
}

#[semio_framework_async_macros::async_test]
async fn open_dialog_effect_round_trips_camel_case() {
    let effect = Effect::OpenDialog { req: RequestId(1), dialog_id: "addObject".into(), args: None };
    let json = serde_json::to_string(&effect).unwrap();
    assert_eq!(json, r#"{"openDialog":{"req":1,"dialogId":"addObject"}}"#);
    let round: Effect = serde_json::from_str(&json).unwrap();
    assert_eq!(round, effect);
}

#[semio_framework_async_macros::async_test]
async fn dispatch_action_effect_round_trips_camel_case() {
    let effect = Effect::DispatchAction { req: RequestId(2), action: "advanceReconstruction".into(), args: Some(dsl::os_pack::json::to_dsl_value(&dsl::json!({"jobId": "job-1"}))), delay_ms: 250 };
    let json = serde_json::to_string(&effect).unwrap();
    assert_eq!(json, r#"{"dispatchAction":{"req":2,"action":"advanceReconstruction","args":{"jobId":"job-1"},"delayMs":250}}"#);
    let round: Effect = serde_json::from_str(&json).unwrap();
    assert_eq!(round, effect);
    // `args` omitted entirely when unset, not serialized as `null`.
    let bare = Effect::DispatchAction { req: RequestId(3), action: "tick".into(), args: None, delay_ms: 0 };
    let bare_json = serde_json::to_string(&bare).unwrap();
    assert!(!bare_json.contains("\"args\""), "omitted when unset: {bare_json}");
    assert_eq!(serde_json::from_str::<Effect>(&bare_json).unwrap(), bare);
}

#[semio_framework_async_macros::async_test]
async fn request_file_open_effect_round_trips_multiple() {
    let effect = Effect::RequestFileOpen { req: RequestId(4), accept: ".png,.jpg".into(), read_as: Some("dataUrl".into()), import_action: "importFramePayload".into(), multiple: true };
    let json = serde_json::to_string(&effect).unwrap();
    assert!(json.contains("\"multiple\":true"), "{json}");
    let round: Effect = serde_json::from_str(&json).unwrap();
    assert_eq!(round, effect);
    // `multiple` defaults to false when absent from the wire (older callers/plugins); `req` is
    // not defaulted (mandatory on every completing effect).
    let defaulted: Effect = serde_json::from_str(r#"{"requestFileOpen":{"req":5,"accept":".png","importAction":"importFramePayload"}}"#).unwrap();
    assert_eq!(defaulted, Effect::RequestFileOpen { req: RequestId(5), accept: ".png".into(), read_as: None, import_action: "importFramePayload".into(), multiple: false });
}

#[semio_framework_async_macros::async_test]
async fn request_media_frames_effect_round_trips_camel_case() {
    let effect = Effect::RequestMediaFrames {
        req: RequestId(6),
        accept: "video/mp4,video/quicktime".into(),
        frame_action: "importVideoFramePayload".into(),
        done_action: "importVideoDone".into(),
        fallback_action: "importVideoBytesPayload".into(),
        sample_stride: 5,
        max_frames: 200,
        max_long_edge_px: 1600,
        fps_hint: 30.0,
        payload: None,
        args: Some(dsl::os_pack::json::to_dsl_value(&dsl::json!({"streamId": "s1"}))),
    };
    let json = serde_json::to_string(&effect).unwrap();
    assert!(json.contains("\"requestMediaFrames\""), "{json}");
    assert!(json.contains("\"sampleStride\":5"), "{json}");
    assert!(json.contains("\"maxLongEdgePx\":1600"), "{json}");
    assert!(!json.contains("\"payload\""), "omitted when unset: {json}");
    let round: Effect = serde_json::from_str(&json).unwrap();
    assert_eq!(round, effect);
    // Numeric hints default to 0 (host-default) and `payload`/`args` may be entirely absent.
    let defaulted: Effect = serde_json::from_str(r#"{"requestMediaFrames":{"req":7,"accept":"video/mp4","frameAction":"f","doneAction":"d","fallbackAction":"b"}}"#).unwrap();
    assert_eq!(
        defaulted,
        Effect::RequestMediaFrames {
            req: RequestId(7),
            accept: "video/mp4".into(),
            frame_action: "f".into(),
            done_action: "d".into(),
            fallback_action: "b".into(),
            sample_stride: 0,
            max_frames: 0,
            max_long_edge_px: 0,
            fps_hint: 0.0,
            payload: None,
            args: None,
        }
    );
    // `payload`-carrying variant (drop-zone bytes already in memory, no picker needed).
    let with_payload = Effect::RequestMediaFrames {
        req: RequestId(8),
        accept: "video/*".into(),
        frame_action: "f".into(),
        done_action: "d".into(),
        fallback_action: "b".into(),
        sample_stride: 1,
        max_frames: 0,
        max_long_edge_px: 0,
        fps_hint: 0.0,
        payload: Some("data:video/mp4;base64,AAAA".into()),
        args: None,
    };
    let payload_json = serde_json::to_string(&with_payload).unwrap();
    assert!(payload_json.contains("\"payload\":\"data:video/mp4;base64,AAAA\""), "{payload_json}");
    assert_eq!(serde_json::from_str::<Effect>(&payload_json).unwrap(), with_payload);
}

//#endregion 🔖️ActionArgsAndUtilitiesTests

//#region 🔖️SurfaceTests
/// ⚖️ LAW (contract freeze §1 C1): `parse_surface_app_id(surface_app_id(d, r)) == (d, r)` for
/// every dialect in a fixture set covering subset `*`, a dotted standard, and a hyphenated
/// artifact kind.
#[semio_framework_async_macros::async_test]
async fn surface_app_id_round_trips_through_parse_surface_app_id() {
    let fixtures = [
        (ArtifactDialect { artifact_kind: "s.cad.cad".into(), standard: "1".into(), subset: "*".into() }, AppRole::Editor),
        (ArtifactDialect { artifact_kind: "s.stdio.png".into(), standard: "1.7".into(), subset: "a".into() }, AppRole::Viewer),
        (ArtifactDialect { artifact_kind: "s.stdio.dwg-2d".into(), standard: "1".into(), subset: "cc6".into() }, AppRole::Editor),
    ];
    for (dialect, role) in fixtures {
        let id = surface_app_id(&dialect, role);
        let (parsed_dialect, parsed_role) = parse_surface_app_id(&id).unwrap_or_else(|err| panic!("{id}: {err}"));
        assert_eq!(parsed_dialect, dialect, "{id}");
        assert_eq!(parsed_role, role, "{id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn parse_surface_app_id_rejects_missing_hash_and_unknown_role() {
    assert!(parse_surface_app_id("s.cad.cad@1/*").is_err(), "missing '#role' suffix");
    assert!(parse_surface_app_id("s.cad.cad@1/*#owner").is_err(), "role outside viewer/editor");
}

#[semio_framework_async_macros::async_test]
async fn app_role_serde_wire_strings_are_exactly_viewer_and_editor() {
    assert_eq!(serde_json::to_string(&AppRole::Viewer).unwrap(), "\"viewer\"");
    assert_eq!(serde_json::to_string(&AppRole::Editor).unwrap(), "\"editor\"");
    assert_eq!(serde_json::from_str::<AppRole>("\"viewer\"").unwrap(), AppRole::Viewer);
    assert_eq!(serde_json::from_str::<AppRole>("\"editor\"").unwrap(), AppRole::Editor);
    assert!(serde_json::from_str::<AppRole>("\"owner\"").is_err());
}

#[semio_framework_async_macros::async_test]
async fn app_role_as_str_and_from_str_round_trip() {
    assert_eq!(AppRole::Viewer.as_str(), "viewer");
    assert_eq!(AppRole::Editor.as_str(), "editor");
    assert_eq!("viewer".parse::<AppRole>().unwrap(), AppRole::Viewer);
    assert_eq!("editor".parse::<AppRole>().unwrap(), AppRole::Editor);
    assert!("owner".parse::<AppRole>().is_err());
}

#[semio_framework_async_macros::async_test]
async fn panel_tab_kind_settings_default_apps_id_str() {
    assert_eq!(PanelTabKind::SettingsDefaultApps.id_str(), "framework.settings.default-apps");
}

#[semio_framework_async_macros::async_test]
async fn app_ref_canonical_json_round_trips_as_camel_case() {
    let app_ref = AppRef { plugin_id: "s.cad".into(), app_id: "s.cad.cad@1/*#editor".into() };
    let json = dsl::os_pack::json::to_json_string(&app_ref);
    assert_eq!(json, "{\"pluginId\":\"s.cad\",\"appId\":\"s.cad.cad@1/*#editor\"}");
    assert_eq!(dsl::os_pack::json::from_json_str::<AppRef>(&json).unwrap(), app_ref);
}
//#endregion 🔖️SurfaceTests

//#region 🎯️ActionSemanticsFixture
#[test]
fn catalog_icons_depend_only_on_action_kind_and_explicit_icons_stay_owned() {
    let first = ActionDefinition::new_catalog("domain.alpha", LocalizedLabel::data("Alpha"), ActionKind::Mutation);
    let second = ActionDefinition::new_catalog("domain.beta", LocalizedLabel::data("Beta"), ActionKind::Mutation);
    let command = CommandDefinition::new_catalog("domain.command", LocalizedLabel::data("Command"), "domain", ActionKind::Mutation);
    let owned = ActionDefinition::new("domain.owned", LocalizedLabel::data("Owned"), ActionKind::Mutation, "camera");
    assert_eq!(first.icon_id, second.icon_id);
    assert_eq!(first.icon_id, command.icon_id);
    assert_eq!(first.icon_id.as_str(), "sparkles");
    assert_eq!(owned.icon_id.as_str(), "camera");
}

#[test]
fn action_semantics_defaults_match_language_neutral_fixture() {
    #[derive(serde::Deserialize)]
    struct Case {
        kind: ActionKind,
        semantics: ActionSemantics,
    }
    let cases: Vec<Case> = serde_json::from_str(include_str!("../../🧫️fixtures/⚖️action-semantics.json")).unwrap();
    assert_eq!(cases.len(), 6);
    for case in cases {
        assert_eq!(ActionSemantics::for_kind(case.kind), case.semantics);
        assert_eq!(case.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified);
    }
}
//#endregion 🎯️ActionSemanticsFixture

#[cfg(feature = "typegen")]
#[test]
fn exports_typescript_bindings() {
    crate::schema_metadata::validate().unwrap();
    assert_eq!(crate::schema_metadata::TYPES.len(), 184);
    let rendered = crate::schema_metadata::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, include_str!("../../🤖️generated/🪪️manifest.ts"));
    }
}
