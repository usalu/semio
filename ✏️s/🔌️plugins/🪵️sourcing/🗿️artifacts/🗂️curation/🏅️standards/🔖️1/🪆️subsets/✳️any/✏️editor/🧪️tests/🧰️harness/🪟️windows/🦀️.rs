//! 🪟️ The window rendering of the sourcing editor tests: bodies, table scenes and observed trees of the mounted app, instantiated here once.
use semio_s_artifact_sourcing_curation::editor::sourcing::SourcingCurationCommand;
use semio_plugin::artifact_app_laws::{meta, OriginalFixtureCaller, settle_registered_typed_operation, dispatch_original_fixture_command, dispatch_fixture_action, admit_document_archive_load, settle_document_archive_load, observe_registered_fixture_app, render_registered_fixture_app, fixture_identity};
use semio_plugin::{ActionMeta, InvocationResult, MountedOwnerPolicyV1, ViewModel};
use semio_s_artifact_sourcing_curation_harness_mount::{SourcingTestApp, SourcingApp};
use semio_os_kernel as protocol;

pub async fn render(app: &mut SourcingTestApp, body_key: &str) -> String {
    render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), app.1.cx()).await.expect("bounded retained fixture projection")
}

/// 📊️ The table scene a rendered pool/curated body carries — found anywhere in the tree, since the
/// pool nests its surface under a filter row while the curated body is the surface itself.
pub fn table_scene_of(node: &semio_plugin::BuiltNode) -> Option<semio_plugin::TableScene> {
    if let semio_plugin::Component::Surface(props) = &node.component {
        if props.kind == semio_plugin::plugin_app_close_prelude::SurfaceKind::Table {
            let spine: semio_plugin::TableScene = semio_ui_scene::decode(props).expect("typed table spine");
            assert!(node.rejected_children.is_empty(), "exact retained table has no rejected children");
            assert_eq!(spine.lanes.len(), 2, "complete table lane declarations");
            assert_eq!(node.children.len(), spine.lanes.len(), "complete retained table carriers");
            for (name, key) in [("columns", semio_ui_scene::TABLE_COLUMNS_LANE_KEY), ("rows", semio_ui_scene::TABLE_ROWS_LANE_KEY)] {
                let declared: Vec<_> = spine.lanes.iter().filter(|lane| lane.lane == name).collect();
                assert_eq!(declared.len(), 1, "one exact declared table lane {name}");
                let carriers: Vec<_> = node.children.iter().filter(|child| child.key.as_str() == key).collect();
                assert_eq!(carriers.len(), 1, "one exact retained table carrier {key}");
                let payload = semio_plugin::artifact_app_laws::built_carrier_text(carriers[0]);
                assert_eq!(payload.len(), declared[0].bytes as usize, "complete lane UTF-8 bytes");
                assert_eq!(semio_ui_scene::scene_lane_hash(&payload), declared[0].hash, "complete lane digest");
            }
            return Some(semio_plugin::artifact_app_laws::built_surface_scene(node).expect("complete first-party retained table scene"));
        }
    }
    node.children.iter().find_map(table_scene_of)
}

/// 🖼️ Renders `body_key` through the live app and decodes its table scene's column ids and row records.
pub async fn table_of(app: &mut SourcingTestApp, body_key: &str) -> (Vec<String>, Vec<serde_json::Value>) {
    let mut out = None;
    observe(app, body_key, &mut |root| {
        let scene = table_scene_of(root).expect("a table scene in the rendered body");
        let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");
        out = Some((columns.iter().map(|column| column["id"].as_str().unwrap_or_default().to_string()).collect(), rows));
    }).await;
    out.expect("a table scene in the rendered body")
}

/// 🪪️ The `id` of every row a table body currently shows, in display order.
pub async fn row_ids(app: &mut SourcingTestApp, body_key: &str) -> Vec<String> {
    table_of(app, body_key).await.1.iter().map(|row| row["id"].as_str().unwrap_or_default().to_string()).collect()
}

/// 🔭️ Renders `body_key` through the live app and lets `observe` read the built tree.
pub async fn observe(app: &mut SourcingTestApp, body_key: &str, observe: &mut dyn FnMut(&semio_plugin::BuiltNode)) {
    observe_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), app.1.cx(), |root| observe(root)).await.expect("render")
}
