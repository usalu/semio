//! 🧪️ Neutral Rewriting window configuration mutation and routing laws.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_plugin::WindowConfigOwner;

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
    use crate::editor::rewriting::{create_rewriting_app, TrinityRewritingCommand, TrinityRewritingPlayApp, TRINITY_REWRITING_PLAY_BODY_BEFORE};
    use semio_framework_plugin::{testkit, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    fn manifest() -> App {
        App { definition: create_rewriting_app(), examples: Vec::new() }
    }
    async fn render(app: &mut VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>, view: &ViewModel) -> Result<serde_json::Value, String> {
        let tree = app.render(TRINITY_REWRITING_PLAY_BODY_BEFORE, None, view).await.map_err(|error| format!("{error:?}"))?;
        let text = testkit::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
        let scene = testkit::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&text).map_err(str::to_string)?;
        serde_json::to_value(scene).map_err(|error| error.to_string())
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let view = ViewModel {
        window_instances: ["leftWindowId", "rightWindowId"].into_iter().map(|key| ViewWindowInstance { id: fixture[key].as_str().unwrap().into(), window_kind_id: BeforeWindowConfigOwner::WINDOW_KIND_ID.into() }).collect(),
        ..Default::default()
    };
    let left = view.for_window_instance(fixture["leftWindowId"].as_str().unwrap()).unwrap();
    let right = view.for_window_instance(fixture["rightWindowId"].as_str().unwrap()).unwrap();
    let mut app = testkit::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(manifest).await;
    let mut reopened = testkit::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(manifest).await;
    app.bind_instance_id(1).await;
    reopened.bind_instance_id(2).await;
    let outcome: Result<(), String> = async {
        let document = app.snapshot().map_err(|error| format!("{error:?}"))?;
        let config_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        let right_before = render(&mut app, &right).await?;
        for row in fixture["cases"].as_array().unwrap() {
            let context = view.for_window_instance(row["windowId"].as_str().unwrap()).unwrap();
            let command = match row["mutation"]["kind"].as_str().unwrap() {
                "set-camera" => TrinityRewritingCommand::SetViewport { surface_id: None, viewport_json: row["mutation"]["camera"].to_string() },
                "set-lod-mode" => TrinityRewritingCommand::SetLodMode { value: row["mutation"]["value"].as_str().unwrap().into() },
                other => return Err(format!("unexpected neutral command {other}")),
            };
            app.dispatch_typed(command, &ActionMeta { view_state: Some(context), ..testkit::meta("window-config") }).await.map_err(|error| format!("{error:?}"))?;
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        let mut receipts = 0;
        while app.has_pending_typed_operations() {
            if std::time::Instant::now() >= deadline {
                return Err("window config operations did not finish".into());
            }
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
            app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
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
        let left_tree = render(&mut app, &left).await?;
        let right_tree = render(&mut app, &right).await?;
        let left_camera = scene_field(&left_tree, "viewport").ok_or_else(|| "left window is missing its rendered viewport".to_string())?;
        if ["x", "y", "zoom"].iter().any(|key| left_camera[key].as_f64() != fixture["cases"][0]["mutation"]["camera"][key].as_f64()) {
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
            let after = render(&mut reopened, context).await?;
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
        eprintln!("[DEBUG] Rewriting window configuration runtime failure before close: {error}");
    }
    testkit::close_registered_fixture_app(&mut reopened);
    testkit::close_registered_fixture_app(&mut app);
    outcome.expect("retained Rewriting window configuration isolation and persistence");
    eprintln!("[DEBUG] two Rewriting windows published camera/LOD independently, preserved document and app envelopes, rendered separate state, and reloaded exact persisted partitions");
}

#[test]
fn rewriting_window_config_mutations_match_the_independent_patch_trace() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let base: RewritingWindowConfig = pack::from_json_str(&fixture["base"].to_string()).unwrap();
    let mut windows = std::collections::BTreeMap::from([(fixture["leftWindowId"].as_str().unwrap().to_string(), base.clone()), (fixture["rightWindowId"].as_str().unwrap().to_string(), base)]);
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap();
        let mutation: RewritingWindowConfigMutation = pack::from_json_str(&row["mutation"].to_string()).unwrap();
        let before = windows[id].clone();
        let after = mutation.diff(&before).diff().apply(&before).unwrap();
        let restored = mutation.inverse(&before).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
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
            let expected: RewritingWindowConfig = pack::from_json_str(&row["expected"][id].to_string()).unwrap();
            assert_eq!(state, &expected);
        }
    }
    assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&semio_framework_plugin::NoConfig::default())).unwrap(), fixture["expectedAppConfig"]);
    eprintln!("[DEBUG] Rewriting window config mutations matched camera/LOD patch trace and restored exact inverses; app config stayed empty");
}

#[test]
fn rewriting_window_config_commands_use_the_trusted_concrete_window() {
    use semio_framework_plugin::{Locale, Terminology, ViewModel, ViewWindowInstance};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let left = fixture["leftWindowId"].as_str().unwrap();
    let right = fixture["rightWindowId"].as_str().unwrap();
    let view = ViewModel {
        locale: Locale::En,
        terminology: Terminology::Native,
        window_id: Some(right.into()),
        window_instances: vec![ViewWindowInstance { id: left.into(), window_kind_id: BeforeWindowConfigOwner::WINDOW_KIND_ID.into() }, ViewWindowInstance { id: right.into(), window_kind_id: BeforeWindowConfigOwner::WINDOW_KIND_ID.into() }],
        ..Default::default()
    };
    let emit = crate::editor::rewriting::commands::set_lod_mode("compact", Some(&view)).unwrap();
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), right);
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), BeforeWindowConfigOwner::WINDOW_KIND_ID);
    assert!(crate::editor::rewriting::commands::set_lod_mode("compact", None).is_err());
    assert!(crate::editor::rewriting::commands::set_lod_mode(&"x".repeat(65), Some(&view)).is_err());
    eprintln!("[DEBUG] Rewriting LOD command addressed the host-selected window and emitted only a window configuration mutation");
}
