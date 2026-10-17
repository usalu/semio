//! 🚚️ Independent laws of paged section text and typed scene lane carriers.
extern crate semio_framework_plugin_window_transient as window_surface;
use semio_framework_plugin_window_transient::section_component_tree;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework::UiRefreshSection;
use semio_framework_ui_contract::{UI_TEXT_MAX_BYTES,UI_BUILT_CHILDREN_MAX,UI_FIXED_LIST_ITEMS};
use serde_json::{Value,json};
use std::collections::BTreeMap;
#[global_allocator]
static SURFACE_CARRIER_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

    /// 🧮️ Sized FROM the two ceilings rather than against a frozen literal: the law's whole subject is
    /// a payload one whole node of children cannot hold, so its precondition has to follow whatever
    /// `UI_TEXT_MAX_BYTES`/`UI_BUILT_CHILDREN_MAX` currently are. Each map row is at least 24 bytes of
    /// JSON (`"window-0000":"measure-0000",`), so this many rows always overflows the product.
    #[semio_framework_async_macros::async_test]
    async fn reserved_section_carrier_pages_a_payload_past_one_node_of_children() {
        let rows = UI_TEXT_MAX_BYTES * UI_BUILT_CHILDREN_MAX / 24;
        let payload = serde_json::to_string(&(0..rows).map(|index| (format!("window-{index:04}"), format!("measure-{index:04}"))).collect::<BTreeMap<_, _>>()).unwrap();
        assert!(payload.len() > UI_TEXT_MAX_BYTES * UI_BUILT_CHILDREN_MAX);
        let tree = section_component_tree(UiRefreshSection::Measures, &payload).unwrap();
        let projected: Value = serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap()).unwrap();
        let mut chunks = Vec::new();
        let mut depth = 0usize;
        let mut frontier = vec![(projected.clone(), 0usize)];
        while let Some((node, level)) = frontier.pop() {
            depth = depth.max(level);
            if node["component"]["type"] == "text" {
                let value = node["component"]["value"].as_str().unwrap();
                assert!(value.len() <= UI_TEXT_MAX_BYTES);
                chunks.push(value.to_string());
                if let Some(attributes) = node["component"]["dataAttributes"].as_object() {
                    let mut keys: Vec<&String> = attributes.keys().collect();
                    keys.sort();
                    for key in keys {
                        let attr = attributes[key].as_str().unwrap();
                        assert!(attr.len() <= UI_TEXT_MAX_BYTES);
                        chunks.push(attr.to_string());
                    }
                }
            }
            assert!(node["children"].as_array().unwrap().len() <= UI_BUILT_CHILDREN_MAX);
            for child in node["children"].as_array().unwrap().iter().rev() {
                frontier.push((child.clone(), level + 1));
            }
        }
        let pack = UI_TEXT_MAX_BYTES * (1 + UI_FIXED_LIST_ITEMS);
        assert_eq!(artifact_app_laws::fixture_carrier_text(&projected), payload);
        assert_eq!(chunks.concat(), payload);
        let leaves = count_projected_text_leaves(&projected);
        assert!(leaves <= payload.len().div_ceil(pack).max(1) + 2, "packed leaves must stay near ceil(bytes/pack), got {leaves} for {} bytes", payload.len());
        assert!(count_projected_nodes(&projected) <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "a reserved measures carrier must fit the document node table");
        eprintln!("section carrier packed {} bytes into {} text leaves at depth {depth} ({} nodes)", payload.len(), chunks.len(), count_projected_nodes(&projected));
    }

    /// 🧩️ Pins the pack this producer writes against the language-neutral fixture the TypeScript
    /// reader (`sectionValueFromBuiltNode` in `📺️renderer/…/🔌️PluginRuntime/🟦️.tsx`) reassembles from —
    /// a reader that read only `value` handed `JSON.parse` an exact 512-byte prefix, see ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END `📓️json-512-truncation-2026-09-10.md`.
    #[semio_framework_async_macros::async_test]
    async fn packed_section_carrier_matches_the_neutral_fixture_slices() {
        let fixture: Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🔬️paged-text-carrier/🔣️.json")).unwrap();
        assert_eq!(fixture["textMaxBytes"].as_u64().unwrap() as usize, UI_TEXT_MAX_BYTES);
        assert_eq!(fixture["packSlices"].as_u64().unwrap() as usize, 1 + UI_FIXED_LIST_ITEMS);
        let payload = fixture["payload"].as_str().unwrap();
        assert_eq!(payload.len(), fixture["payloadBytes"].as_u64().unwrap() as usize);
        let slices: Vec<&str> = fixture["slices"].as_array().unwrap().iter().map(|slice| slice.as_str().unwrap()).collect();
        assert_eq!(slices.concat(), payload);
        assert!(slices.len() > 1 && slices.len() <= 1 + UI_FIXED_LIST_ITEMS, "the fixture must be one packed leaf past a single slice");
        let section = UiRefreshSection::from_body_key(fixture["rootKey"].as_str().unwrap()).unwrap();
        let projected: Value = serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(section_component_tree(section, payload).unwrap()).unwrap()).unwrap();
        assert_eq!(projected["key"].as_str().unwrap(), fixture["rootKey"].as_str().unwrap());
        assert_eq!(count_projected_text_leaves(&projected), 1, "a payload inside one pack must ride on exactly one leaf");
        let leaf = &projected["children"][0];
        assert_eq!(leaf["component"]["value"].as_str().unwrap(), slices[0]);
        let attributes = leaf["component"]["dataAttributes"].as_object().unwrap();
        assert_eq!(attributes.len(), slices.len() - 1);
        for (offset, slice) in slices.iter().enumerate().skip(1) {
            assert_eq!(attributes[&format!("{offset:02}")].as_str().unwrap(), *slice);
        }
        assert_eq!(artifact_app_laws::fixture_carrier_text(&projected), payload);
    }

    /// 🧱 Presented-node census of a projected builder tree (surface + every descendant).
    fn count_projected_nodes(node: &Value) -> usize {
        1 + node["children"].as_array().map(|children| children.iter().map(count_projected_nodes).sum()).unwrap_or(0)
    }

    fn count_projected_text_leaves(node: &Value) -> usize {
        let here = usize::from(node["component"]["type"] == "text");
        here + node["children"].as_array().map(|children| children.iter().map(count_projected_text_leaves).sum()).unwrap_or(0)
    }

    /// 🚚️ Projects one built surface node and returns `(projection, lane subtree by carrier key)`.
    fn project_scene_surface(node: semio_framework_ui_contract::BuiltNode) -> (Value, BTreeMap<String, Value>) {
        let projection: Value = serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(crate::window_surface::built_to_component_tree(node)).unwrap()).unwrap();
        let lanes = projection["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|child| (child["key"].as_str().unwrap().to_string(), child.clone()))
            .collect();
        (projection, lanes)
    }

    /// 🚚️ Asserts one lane carrier obeys the contract's own bounds and reproduces `payload` exactly.
    fn assert_lane_carrier(carrier: &Value, payload: &str) -> (usize, usize) {
        let mut leaves = 0usize;
        let mut depth = 0usize;
        let mut frontier = vec![(carrier, 0usize)];
        while let Some((node, level)) = frontier.pop() {
            assert!(node["children"].as_array().unwrap().len() <= UI_BUILT_CHILDREN_MAX);
            if node["component"]["type"] == "text" {
                leaves += 1;
                depth = depth.max(level);
                assert!(node["component"]["value"].as_str().unwrap().len() <= UI_TEXT_MAX_BYTES);
            }
            for child in node["children"].as_array().unwrap().iter().rev() {
                frontier.push((child, level + 1));
            }
        }
        assert_eq!(artifact_app_laws::fixture_carrier_text(carrier), payload);
        (leaves, depth)
    }

    /// 🌍️ A world-3d scene whose payload is far past the 32 KiB `UiFixedBytes` doc ceiling.
    fn oversized_world_scene(camera: &str, selection: &str) -> semio_framework_ui_scene::World3dScene {
        let instances = serde_json::to_string(
            &(0..600)
                .map(|index| json!({"id": format!("capsule-{index:04}"), "meshId": "box", "position": [index as f64, 0.0, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0]}))
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let mut scene = semio_framework_ui_scene::World3dScene::base(camera.into(), r#"[{"id":"box","kind":"box"}]"#.into(), instances, selection.into());
        scene.vortices_json = Some(serde_json::to_string(&(0..40).map(|index| json!({"id": format!("vortex-{index}"), "position": [0.0, index as f64, 0.0]})).collect::<Vec<_>>()).unwrap());
        scene.lod_json = Some(r#"{"maxInstances":8000}"#.into());
        scene.chunking_json = Some(r#"{"chunkSize":64,"radius":8000}"#.into());
        scene.domain_id = Some("puzzle3d".into());
        scene
    }

    #[semio_framework_async_macros::async_test]
    async fn world3d_scene_surface_pages_every_lane_beside_a_spine_that_fits_the_fixed_doc() {
        let scene = oversized_world_scene("{}", r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#);
        let assembled_bytes = <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::encode_pack(&scene).unwrap().len();
        assert!(assembled_bytes > semio_framework_ui_contract::UI_FIXED_BYTES, "the unsplit scene must be past the fixed doc ceiling to prove anything");

        let node = crate::window_surface::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &scene).unwrap();
        let (projection, lanes) = project_scene_surface(node);

        let doc_bytes = projection["component"]["doc"]["bytes"].as_array().unwrap().len();
        assert!(doc_bytes <= semio_framework_ui_contract::UI_FIXED_BYTES);

        let (spine, expected) = semio_framework_ui_scene::SceneDoc::split_lanes(&scene);
        assert_eq!(lanes.len(), expected.len());
        let mut total = 0usize;
        for lane in &expected {
            let carrier = lanes.get(lane.key).unwrap_or_else(|| panic!("lane {} publishes a carrier", lane.key));
            assert!(semio_framework_ui_scene::World3dSceneLane::from_body_key(lane.key).is_some());
            assert_lane_carrier(carrier, &lane.payload);
            total += lane.payload.len();
        }
        for reference in &spine.lanes {
            let lane = semio_framework_ui_scene::World3dSceneLane::from_name(&reference.lane).unwrap();
            assert_eq!(reference.bytes as usize, expected.iter().find(|entry| entry.key == lane.body_key()).unwrap().payload.len());
            assert_eq!(reference.hash, semio_framework_ui_scene::scene_lane_hash(&expected.iter().find(|entry| entry.key == lane.body_key()).unwrap().payload));
        }

        let mut reassembled: semio_framework_ui_scene::World3dScene = artifact_app_laws::decode_fixture_scene_with_lanes(&serde_json::to_string(&projection).unwrap()).unwrap();
        reassembled.lanes = Vec::new();
        assert_eq!(reassembled, scene);
    }

    #[semio_framework_async_macros::async_test]
    async fn world3d_scene_surface_republishes_only_the_lanes_that_changed() {
        let selection = r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#;
        let (_, first) = project_scene_surface(crate::window_surface::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &oversized_world_scene("{}", selection)).unwrap());

        let (moved_projection, moved) = project_scene_surface(crate::window_surface::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &oversized_world_scene(r#"{"position":[9,9,9]}"#, selection)).unwrap());
        assert_eq!(moved, first, "a camera move must leave every lane carrier byte-identical");

        let picked = r#"{"method":"rectangle","mode":"replace","ids":["capsule-0007"],"hoveredId":null}"#;
        let (picked_projection, picked_lanes) = project_scene_surface(crate::window_surface::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &oversized_world_scene("{}", picked)).unwrap());
        let changed: Vec<&String> = picked_lanes.keys().filter(|key| picked_lanes.get(*key) != first.get(*key)).collect();
        assert_eq!(changed, vec![semio_framework_ui_scene::World3dSceneLane::Selection.body_key()]);
        assert_ne!(picked_projection["component"], moved_projection["component"], "a changed lane must still move the spine so its consumer re-reads");
    }

    #[semio_framework_async_macros::async_test]
    async fn world3d_scene_surface_pages_one_oversized_lane_instead_of_faulting() {
        let instances = serde_json::to_string(&(0..3_000).map(|index| json!({"id": format!("capsule-{index:05}"), "meshId": "box"})).collect::<Vec<_>>()).unwrap();
        assert!(instances.len() > UI_TEXT_MAX_BYTES * UI_BUILT_CHILDREN_MAX, "one lane must be past a single carrier level to prove paging");
        let scene = semio_framework_ui_scene::World3dScene::base("{}".into(), "[]".into(), instances.clone(), "{}".into());
        let (_, lanes) = project_scene_surface(crate::window_surface::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &scene).unwrap());
        let (leaves, depth) = assert_lane_carrier(lanes.get(semio_framework_ui_scene::World3dSceneLane::Instances.body_key()).unwrap(), &instances);
        let pack = UI_TEXT_MAX_BYTES * (1 + UI_FIXED_LIST_ITEMS);
        assert!(leaves <= instances.len().div_ceil(pack).max(1) + 2, "packed instances leaves must stay near ceil(bytes/pack), got {leaves} for {} bytes", instances.len());
        eprintln!("one {}-byte instances lane packed into {leaves} text leaves at depth {depth}", instances.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn a_scene_that_declares_no_lanes_still_publishes_one_childless_surface() {
        let scene = semio_framework_ui_scene::IconRenderScene { request_json: r#"{"meshId":"box"}"#.into(), footer: None, frame_json: None };
        let (projection, lanes) = project_scene_surface(crate::window_surface::scene_surface("preview", semio_framework_ui_contract::SurfaceKind::IconRender, &scene).unwrap());
        assert!(lanes.is_empty());
        assert_eq!(projection["component"]["type"], "surface");
        assert_eq!(artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_ui_scene::IconRenderScene>(&serde_json::to_string(&projection).unwrap()).unwrap(), scene);
    }

    #[semio_framework_async_macros::async_test]
    async fn nakagin_scale_world3d_surface_fits_document_node_cap() {
        let scene = oversized_world_scene("{}", r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#);
        let node = crate::window_surface::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &scene).unwrap();
        let (projection, _) = project_scene_surface(node);
        let nodes = count_projected_nodes(&projection);
        assert!(nodes <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "Nakagin-scale world-3d surface presented {nodes} nodes over UI_DOCUMENT_NODES");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_100kib_measures_section_fits_document_node_cap() {
        let payload = serde_json::to_string(&(0..2_400).map(|index| (format!("window-{index:04}"), format!("measure-{index:04}"))).collect::<BTreeMap<_, _>>()).unwrap();
        assert!(payload.len() > UI_TEXT_MAX_BYTES * 128, "unpacked 512-byte leaves of this payload would overflow UI_DOCUMENT_NODES");
        let tree = section_component_tree(UiRefreshSection::Measures, &payload).unwrap();
        let projected: Value = serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap()).unwrap();
        let nodes = count_projected_nodes(&projected);
        assert!(nodes <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "a 100 KiB measures section presented {nodes} nodes over UI_DOCUMENT_NODES");
    }
