//! 🧪️ Neutral Rewriting window configuration mutation and routing laws.

use crate::editor::rewriting::window_config::{schema};
use crate::editor::rewriting::window_config::*;
use protocol::{ColdMutation, Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_plugin::WindowConfigOwner;

/// 🧾️ The committed wire witnesses a fixture case names by its leaf folder (`"witness"`).
static WITNESSES: std::sync::LazyLock<[(&str, &str); 2]> = std::sync::LazyLock::new(|| [
    ("🎥️set-camera", runtime_fixture_text("../../🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎥️set-camera/🧾️wire-witness/🦠️mutation/🔣️.json")),
    ("🔍️set-lod-mode", runtime_fixture_text("../../🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🔍️set-lod-mode/🧾️wire-witness/🦠️mutation/🔣️.json")),
]);

/// 🧾️ The committed wire witness of one fixture case, decoded as the canonical Rust wire of its mutation.
fn witness(row: &serde_json::Value) -> (serde_json::Value, RewritingWindowConfigMutation) {
    let text = (*WITNESSES).iter().find(|(leaf, _)| row["witness"] == *leaf).map(|(_, text)| *text).expect("case names a committed wire witness");
    (serde_json::from_str(text).unwrap(), store::os_store::test_support::assert_wire_witness(text))
}

/// 🧭 Finds a named semantic scene field in the independent serialized UI projection.
fn scene_field(value: &serde_json::Value, field: &str) -> Option<serde_json::Value> {
    match value {
        serde_json::Value::Object(fields) => fields.get(field).cloned().or_else(|| fields.values().find_map(|value| scene_field(value, field))),
        serde_json::Value::Array(items) => items.iter().find_map(|value| scene_field(value, field)),
        serde_json::Value::String(text) => serde_json::from_str::<serde_json::Value>(text).ok().and_then(|value| scene_field(&value, field)),
        _ => None,
    }
}

#[semio_framework_async_macros::async_test]
async fn rewriting_window_config_retained_publication_renders_and_reloads_two_concrete_windows() {
    use crate::editor::rewriting::{create_rewriting_app, TrinityRewritingCommand, TrinityRewritingPlayApp};
use crate::editor::rewriting::ids::TRINITY_REWRITING_PLAY_BODY_BEFORE;
    use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    fn manifest() -> App {
        App { definition: create_rewriting_app(), examples: Vec::new() }
    }
    async fn render(app: &mut VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>, view: &ViewModel, cx: &mut semio_framework_plugin::StepContext<'_>) -> Result<serde_json::Value, String> {
        let text = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut *app, TRINITY_REWRITING_PLAY_BODY_BEFORE, view, cx).await.map_err(|error| format!("{error:?}"))?;
        let scene = artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&text).map_err(str::to_string)?;
        serde_json::to_value(scene).map_err(|error| error.to_string())
    }
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🧫️fixtures/🔬️window-config-ownership/🔣️.json")).unwrap();
    let view = ViewModel {
        window_instances: ["leftWindowId", "rightWindowId"].into_iter().map(|key| ViewWindowInstance { id: fixture[key].as_str().unwrap().into(), window_kind_id: BeforeWindowConfigOwner::WINDOW_KIND_ID.into() }).collect(),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let left = view.for_window_instance(fixture["leftWindowId"].as_str().unwrap()).unwrap();
    let right = view.for_window_instance(fixture["rightWindowId"].as_str().unwrap()).unwrap();
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = artifact_app_laws::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    let mut reopened = artifact_app_laws::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(1).await;
    reopened.bind_instance_id(2).await;
    let outcome: Result<(), String> = async {
        let document = app.snapshot().map_err(|error| format!("{error:?}"))?;
        let config_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        let right_before = render(&mut app, &right, scope.cx()).await?;
        for row in fixture["cases"].as_array().unwrap() {
            let context = view.for_window_instance(row["windowId"].as_str().unwrap()).unwrap();
            let (mutation, _) = witness(row);
            let command = match mutation["kind"].as_str().unwrap() {
                "set-camera" => TrinityRewritingCommand::SetViewport {
                    surface_id: None,
                    viewport: serde_json::from_value(mutation["camera"].clone()).map_err(|error| format!("independent viewport oracle rejected fixture: {error}"))?,
                },
                "set-lod-mode" => TrinityRewritingCommand::SetLodMode { value: mutation["value"].as_str().unwrap().into() },
                other => return Err(format!("unexpected neutral command {other}")),
            };
            semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &command, &ActionMeta { view_state: Some(context), ..artifact_app_laws::meta("window-config") }, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        let mut receipts = 0;
        while app.has_pending_typed_operations() {
            if std::time::Instant::now() >= deadline {
                return Err("window config operations did not finish".into());
            }
            app.maintenance_step(scope.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).map_err(|error| format!("{error:?}"))?;
            app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
            if let Some(page) = app.take_typed_operation_result_page(1) {
                let lane = page.lane;
                let bytes = page.bytes().to_vec();
                app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
                if lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                    return Err(format!("window config publication failed: {bytes:?}"));
                }
                if lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig {
                    receipts += 1;
                }
            }
            app.take_typed_operation_effect();
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            while app.take_typed_operation_completion().await.map_err(|error| format!("{error:?}"))?.is_some() {}
            std::thread::yield_now();
        }
        if receipts != fixture["cases"].as_array().unwrap().len() {
            return Err(format!("expected one receipt per window mutation, got {receipts}"));
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
            return Err("window config publication changed document content".into());
        }
        let config_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        if config_before.pack != config_after.pack || config_before.spr != config_after.spr {
            return Err("window publication changed the app configuration envelope".into());
        }
        let left_tree = render(&mut app, &left, scope.cx()).await?;
        let right_tree = render(&mut app, &right, scope.cx()).await?;
        let left_camera = scene_field(&left_tree, "viewport").ok_or_else(|| "left window is missing its rendered viewport".to_string())?;
        if ["x", "y", "zoom"].iter().any(|key| left_camera[key].as_f64() != witness(&fixture["cases"][0]).0["camera"][key].as_f64()) {
            return Err(format!("left window did not render its own camera: {left_tree}"));
        }
        if scene_field(&right_tree, "viewport") != scene_field(&right_before, "viewport") {
            return Err("left camera publication changed the right camera".into());
        }
        let measures = app.window_measures(&view).await;
        for context in [&left, &right] {
            let id = context.window_id.as_deref().unwrap();
            if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))? != Some(1) {
                return Err(format!("window {id} did not advance its own generation exactly once"));
            }
            let expected = fixture["cases"].as_array().unwrap().last().unwrap()["expected"][id]["lodMode"].as_str().unwrap();
            let Some(rows) = measures.get(id) else {
                return Err(format!("missing exact window measures for {id}"));
            };
            if !rows.iter().any(|row| matches!(row, semio_framework_plugin::WindowMeasure::Select { value, .. } if value == expected)) {
                return Err(format!("window {id} did not retain its own LOD"));
            }
        }
        let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
        if packs.len() != 2 {
            return Err(format!("expected two persisted window partitions, got {}", packs.len()));
        }
        for pack in packs {
            reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?;
        }
        for (context, before) in [(&left, &left_tree), (&right, &right_tree)] {
            let after = render(&mut reopened, context, scope.cx()).await?;
            for field in ["viewport", "lodJson"] {
                let expected = scene_field(before, field).ok_or_else(|| format!("render omitted {field} for {:?}", context.window_id))?;
                if Some(expected) != scene_field(&after, field) {
                    return Err(format!("reload changed {field} for {:?}", context.window_id));
                }
            }
        }
        Ok(())
    }
    .await;
    if let Err(error) = &outcome {
        eprintln!("Rewriting window configuration runtime failure before close: {error}");
    }
    artifact_app_laws::close_registered_fixture_app(&mut reopened, semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy(), scope.cx()).expect("original caller paid app close");
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy(), scope.cx()).expect("original caller paid app close");
    outcome.expect("retained Rewriting window configuration isolation and persistence");
}

#[test]
fn rewriting_window_config_mutations_match_the_independent_patch_trace() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🧫️fixtures/🔬️window-config-ownership/🔣️.json")).unwrap();
    let base: RewritingWindowConfig = semio_framework_pack_json::from_json_str(&fixture["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut windows = std::collections::BTreeMap::from([(fixture["leftWindowId"].as_str().unwrap().to_string(), base.clone()), (fixture["rightWindowId"].as_str().unwrap().to_string(), base)]);
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap();
        let (_, mutation) = witness(row);
        let before = windows[id].clone();
        let after = protocol::apply_diff_cold(mutation.diff(&before).diff(), &before).unwrap();
        let restored = mutation.inverse(&before).expect("valid retained mutation inverse fixture").into_iter().fold(after.clone(), |state, inverse| protocol::apply_diff_cold(inverse.diff(&state).diff(), &state).unwrap());
        assert_eq!(restored, before);
        assert_eq!(RewritingWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        let wire = mutation.encode_op().unwrap();
        let descriptor = match &mutation {
            RewritingWindowConfigMutation::SetCamera(_) => <SetCamera as protocol::MutationLeaf>::DESCRIPTOR,
            RewritingWindowConfigMutation::SetLodMode(_) => <SetLodMode as protocol::MutationLeaf>::DESCRIPTOR,
        };
        assert_eq!(Some(u32::from(wire[1])), descriptor.binary_tag);
        assert_eq!(RewritingWindowConfigMutation::decode_op(&wire).unwrap(), mutation);
        windows.insert(id.into(), after);
        for (id, state) in &windows {
            let expected: RewritingWindowConfig = semio_framework_pack_json::from_json_str(&row["expected"][id].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            assert_eq!(state, &expected);
        }
    }
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&semio_framework_plugin::NoConfig::default())).unwrap(), fixture["expectedAppConfig"]);
}

#[test]
fn rewriting_window_config_commands_use_the_trusted_concrete_window() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    use semio_framework_plugin::ViewModel;
    use semio_framework_plugin::ViewWindowInstance;
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🧫️fixtures/🔬️window-config-ownership/🔣️.json")).unwrap();
    let left = fixture["leftWindowId"].as_str().unwrap();
    let right = fixture["rightWindowId"].as_str().unwrap();
    let view = ViewModel {
        locale: Locale::En,
        terminology: Terminology::Native,
        window_id: Some(right.into()),
        window_instances: vec![ViewWindowInstance { id: left.into(), window_kind_id: BeforeWindowConfigOwner::WINDOW_KIND_ID.into() }, ViewWindowInstance { id: right.into(), window_kind_id: BeforeWindowConfigOwner::WINDOW_KIND_ID.into() }],
        ..ViewModel::new(Locale::En, Terminology::Native)
    };
    let emit = crate::editor::rewriting::commands::set_lod_mode::set_lod_mode("compact", Some(&view)).unwrap();
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), right);
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), BeforeWindowConfigOwner::WINDOW_KIND_ID);
    assert!(crate::editor::rewriting::commands::set_lod_mode::set_lod_mode("compact", None).is_err());
    assert!(crate::editor::rewriting::commands::set_lod_mode::set_lod_mode(&"x".repeat(65), Some(&view)).is_err());
}

/// ➕️ Every committed window-config mutation's concrete inverse rows sum to the negative of its diff (law L3).
#[semio_framework_async_macros::async_test]
async fn rewriting_window_config_inverses_sum_to_the_negative_diff() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🧫️fixtures/🔬️window-config-ownership/🔣️.json")).unwrap();
    let base: RewritingWindowConfig = semio_framework_pack_json::from_json_str(&fixture["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut windows = std::collections::BTreeMap::from([(fixture["leftWindowId"].as_str().unwrap().to_string(), base.clone()), (fixture["rightWindowId"].as_str().unwrap().to_string(), base)]);
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap();
        let (_, mutation) = witness(row);
        let before = windows[id].clone();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        windows.insert(id.into(), protocol::apply_diff_cold(mutation.diff(&before).diff(), &before).unwrap());
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
