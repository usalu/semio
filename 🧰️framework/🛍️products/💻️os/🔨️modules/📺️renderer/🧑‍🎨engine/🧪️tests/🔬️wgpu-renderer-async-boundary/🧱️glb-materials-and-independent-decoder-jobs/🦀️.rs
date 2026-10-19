/// 🧱️ GLB materials and independent decoder jobs: the sub element id attributes, the owned decoder fixture, the embedded materials with their primitive ranges, the denied original invocation and the independent decoder job through cancel.

/// 🎯️ LAW: a GLB primitive that carries the `_FACE_ID` (one per triangle, read at the triangle's first corner) and `_VERTEX_ID`
/// (one per vertex) custom attributes keeps them in the resident mesh table, so face and vertex picking on the native renderer
/// resolves to the producer's own ids instead of anonymous triangle and vertex indices. A primitive without them still publishes
/// no id table, and an id accessor of the wrong shape is refused before any mesh is begun.
/// The attribute convention is `🧫️fixtures/📏️world3d-modelling/🔣️.json`'s `glbSubElementIds` row, pinned for both renderers.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn glb_sub_element_id_attributes_survive_into_the_resident_mesh_table() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/📏️world3d-modelling/🔣️.json")).expect("modelling fixture");
    let row = &fixture["glbSubElementIds"];
    let attribute_names = (row["faceAttribute"].as_str().unwrap().to_string(), row["vertexAttribute"].as_str().unwrap().to_string());
    let face_ids: Vec<u32> = row["faceIds"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32).collect();
    let vertex_ids: Vec<u32> = row["vertexIds"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32).collect();
    let indices: Vec<u16> = row["indices"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u16).collect();

    fn glb(attributes: &str, indices: &[u16], face: (&[u32], u32), vertex: (&[u32], u32)) -> Vec<u8> {
        let mut bin = Vec::new();
        for position in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]] {
            for component in position {
                bin.extend_from_slice(&component.to_le_bytes());
            }
        }
        let index_at = bin.len();
        for index in indices {
            bin.extend_from_slice(&index.to_le_bytes());
        }
        while bin.len() % 4 != 0 {
            bin.push(0);
        }
        let face_at = bin.len();
        for id in face.0 {
            bin.extend_from_slice(&id.to_le_bytes());
        }
        let vertex_at = bin.len();
        for id in vertex.0 {
            bin.extend_from_slice(&id.to_le_bytes());
        }
        let json = format!(
            r#"{{"scene":0,"scenes":[{{"nodes":[0]}}],"nodes":[{{"mesh":0}}],"buffers":[{{"byteLength":{total}}}],"accessors":[{{"bufferView":0,"componentType":5126,"count":4,"type":"VEC3"}},{{"bufferView":1,"componentType":5123,"count":{index_count},"type":"SCALAR"}},{{"bufferView":2,"componentType":{face_type},"count":{face_count},"type":"SCALAR"}},{{"bufferView":3,"componentType":{vertex_type},"count":{vertex_count},"type":"SCALAR"}}],"bufferViews":[{{"buffer":0,"byteOffset":0,"byteLength":48}},{{"buffer":0,"byteOffset":{index_at},"byteLength":{index_bytes}}},{{"buffer":0,"byteOffset":{face_at},"byteLength":{face_bytes}}},{{"buffer":0,"byteOffset":{vertex_at},"byteLength":{vertex_bytes}}}],"meshes":[{{"primitives":[{{"attributes":{{"POSITION":0{attributes}}},"indices":1,"mode":4}}]}}]}}"#,
            total = bin.len(),
            index_count = indices.len(),
            index_bytes = indices.len() * 2,
            face_type = face.1,
            face_count = face.0.len(),
            face_bytes = face.0.len() * 4,
            vertex_type = vertex.1,
            vertex_count = vertex.0.len(),
            vertex_bytes = vertex.0.len() * 4,
        );
        let mut json = json.into_bytes();
        while json.len() % 4 != 0 {
            json.push(b' ');
        }
        let total = 12 + 8 + json.len() + 8 + bin.len();
        let mut glb = Vec::with_capacity(total);
        glb.extend_from_slice(b"glTF");
        glb.extend_from_slice(&2u32.to_le_bytes());
        glb.extend_from_slice(&(total as u32).to_le_bytes());
        glb.extend_from_slice(&(json.len() as u32).to_le_bytes());
        glb.extend_from_slice(b"JSON");
        glb.extend_from_slice(&json);
        glb.extend_from_slice(&(bin.len() as u32).to_le_bytes());
        glb.extend_from_slice(b"BIN\0");
        glb.extend_from_slice(&bin);
        glb
    }

    fn run(glb: &[u8]) -> (WorldAssetIoAuthority, RendererAssetProbe, RendererAssetProbeStep) {
        let mut authority = WorldAssetIoAuthority::default();
        authority.reserve(1, 1, WorldAssetRequestKind::Glb, "ids.glb", glb.len()).unwrap();
        let mut owner = authority.take_next().unwrap();
        for page in glb.chunks(97) {
            owner.push_page(WorldAssetResponsePage::try_from_owned(page.to_vec()).unwrap()).unwrap();
        }
        owner.seal().unwrap();
        authority.return_owner(owner).unwrap();
        let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).expect("completed id probe owner");
        let mut probe = RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner));
        for _ in 0..262_144 {
            match probe.step() {
                RendererAssetProbeStep::Pending => {}
                terminal => return (authority, probe, terminal),
            }
        }
        panic!("the GLB id probe never reached a terminal step");
    }

    fn close(mut authority: WorldAssetIoAuthority, mut probe: RendererAssetProbe) {
        probe.begin_close();
        for _ in 0..262_144 {
            if probe.close_step() {
                break;
            }
        }
        let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().expect("the exact response closes") else { panic!("shared probe") };
        authority.finish(owner).unwrap();
        assert!(authority.terminal_is_empty());
    }

    let attributes = format!(r#","{}":2,"{}":3"#, attribute_names.0, attribute_names.1);
    let (authority, probe, step) = run(&glb(&attributes, &indices, (&face_ids, 5125), (&vertex_ids, 5125)));
    assert!(matches!(step, RendererAssetProbeStep::Ready), "a GLB with sub-element id attributes must materialize");
    let lease = probe.glb_materialize.as_ref().expect("materializer").lease.expect("sealed mesh");
    let schema = lease.schema().expect("the id tables seal with the mesh");
    assert_eq!((schema.vertices, schema.indices), (4, 6));
    assert_eq!((schema.face_ids, schema.vertex_ids), (2, 4), "one face id per triangle and one vertex id per vertex");
    let triangle_ids: Vec<u32> = (0..schema.face_ids).map(|item| lease.u32(Mesh3dField::FaceIds, item).unwrap()).collect();
    let point_ids: Vec<u32> = (0..schema.vertex_ids).map(|item| lease.u32(Mesh3dField::VertexIds, item).unwrap()).collect();
    assert_eq!(triangle_ids, row["expectedTriangleFaceIds"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32).collect::<Vec<_>>(), "the face id of a triangle is the id of its first corner's vertex");
    assert_eq!(point_ids, vertex_ids);
    close(authority, probe);

    let (authority, probe, step) = run(&glb("", &indices, (&face_ids, 5125), (&vertex_ids, 5125)));
    assert!(matches!(step, RendererAssetProbeStep::Ready));
    let schema = probe.glb_materialize.as_ref().unwrap().lease.unwrap().schema().unwrap();
    assert_eq!((schema.face_ids, schema.vertex_ids), (0, 0), "a primitive without the attributes publishes no id table");
    close(authority, probe);

    let (authority, probe, step) = run(&glb(&attributes, &indices, (&face_ids, 5126), (&vertex_ids, 5125)));
    assert!(matches!(step, RendererAssetProbeStep::Reject(_) | RendererAssetProbeStep::Fault(_)), "a float id accessor is refused");
    close(authority, probe);
}

#[cfg(not(target_arch = "wasm32"))]
fn owned_decoder_fixture() -> (WorldAssetIoAuthority, RendererAssetProbe) {
    let bytes = semio_framework_os_renderer_wgpu_test_support::repo_bytes("🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb");
    let mut authority = WorldAssetIoAuthority::default();
    authority.reserve(1, 1, WorldAssetRequestKind::Glb, "catalogue-decode-owner.glb", bytes.len()).unwrap();
    let mut owner = authority.take_next().unwrap();
    for page in bytes.chunks(16 * 1024) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(page.to_vec()).unwrap()).unwrap();
    }
    owner.seal().unwrap();
    authority.return_owner(owner).unwrap();
    let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).unwrap();
    (authority, RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)))
}

#[cfg(not(target_arch = "wasm32"))]
fn close_owned_decoder_fixture(mut authority: WorldAssetIoAuthority, mut probe: RendererAssetProbe) {
    probe.begin_close();
    for _ in 0..262_144 {
        if probe.close_step() {
            break;
        }
    }
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().expect("the exact decoder response closes") else {
        panic!("fixture owner changed");
    };
    authority.finish(owner).unwrap();
    authority.begin_close();
    for _ in 0..4096 {
        if authority.close_step() {
            break;
        }
    }
    assert!(authority.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn embedded_glb_materials_publish_with_their_primitive_ranges_texture_and_vertex_colors() {
    let bytes = include_bytes!("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🎨️world3d-glb-material/🧊️two-primitive.glb");
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🎨️world3d-glb-material/🔣️.json")).expect("neutral GLB material fixture");
    let mut authority = WorldAssetIoAuthority::default();
    authority.reserve(1, 1, WorldAssetRequestKind::Glb, "two-primitive.glb", bytes.len()).unwrap();
    let mut owner = authority.take_next().unwrap();
    for page in bytes.chunks(257) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(page.to_vec()).unwrap()).unwrap();
    }
    owner.seal().unwrap();
    authority.return_owner(owner).unwrap();
    let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).expect("completed material response");
    let mut probe = RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner));
    for turn in 0..262_144 {
        match probe.step() {
            RendererAssetProbeStep::Pending => {}
            RendererAssetProbeStep::Ready => break,
            RendererAssetProbeStep::Reject(detail) => panic!("neutral GLB material asset was refused at turn {turn}: {detail}"),
            RendererAssetProbeStep::Fault(detail) => panic!("neutral GLB material asset faulted at turn {turn}: {detail}"),
        }
    }
    let materialize = probe.glb_materialize.as_ref().expect("materializer owner");
    assert!(matches!(materialize.phase, GlbMaterializePhase::Ready));
    let mesh = materialize.lease.expect("sealed material mesh");
    assert_eq!(mesh.schema().unwrap().colors, 6, "a mixed primitive bank expands missing COLOR_0 values to neutral white");
    let appearance = materialize.appearance.as_ref().expect("geometry and material appearance seal together");
    assert_eq!(appearance.texture_count(), 2);
    assert_eq!(appearance.primitives().len(), law["cases"].as_array().unwrap().len());
    let blend = &appearance.primitives()[0];
    assert_eq!((blend.first_index, blend.index_count), (0, 3));
    assert_eq!(blend.material.base_color, [0.8, 0.2, 0.1, 0.75]);
    assert_eq!(blend.material.emissive, [0.1, 0.05, 0.02]);
    assert_eq!((blend.material.metalness, blend.material.roughness), (0.65, 0.2));
    assert!(blend.material.double_sided && blend.material.preserve_vertex_color);
    assert_eq!(blend.material.alpha, semio_framework_ui_scene_pass::SceneMaterialAlpha3d::Blend);
    let mask = &appearance.primitives()[1];
    assert_eq!((mask.first_index, mask.index_count), (3, 3));
    assert_eq!(mask.material.alpha, semio_framework_ui_scene_pass::SceneMaterialAlpha3d::Mask);
    assert_eq!(mask.material.alpha_cutoff, 0.37);
    assert_eq!(blend.material.normal_scale,[1.0,-1.0]);assert_eq!(mask.material.normal_scale,[0.5,0.5]);assert_eq!(mask.material.occlusion_strength,0.25);
    for item in 0..3 {assert_eq!(mesh.vec4(Mesh3dField::Tangents,item).unwrap(),[0.0;4]);assert_eq!(mesh.vec4(Mesh3dField::Tangents,item+3).unwrap(),[0.0,0.0,1.0,-1.0]);let uv1=[[0.1,0.2],[0.9,0.2],[0.5,0.8]][item as usize];let uv4=[[0.2,0.3],[0.8,0.3],[0.5,0.7]][item as usize];assert_eq!(mesh.vec2(Mesh3dField::UvsNormal,item+3).unwrap(),uv1);assert_eq!(mesh.vec2(Mesh3dField::UvsOcclusion,item+3).unwrap(),uv1);assert_eq!(mesh.vec2(Mesh3dField::UvsMetallicRoughness,item+3).unwrap(),uv4);}

    assert!(mask.material.base_color_texture.as_deref().is_some_and(|key| key.ends_with(":texture:0:srgb")));
    assert!(mask.material.metallic_roughness_texture.as_deref().unwrap().ends_with(":texture:1:linear"));
    assert_eq!(mask.material.normal_texture,mask.material.metallic_roughness_texture);
    assert_eq!(mask.material.occlusion_texture,mask.material.metallic_roughness_texture);
    assert_eq!(mask.material.emissive_texture,mask.material.base_color_texture);
    assert_eq!(mask.material.additional_texture_samplers,[mask.material.texture_sampler;4]);
    assert_eq!(
        mask.material.texture_sampler,
        semio_framework_ui_scene_pass::SceneTextureSampler3d {
            wrap_u: semio_framework_ui_scene_pass::SceneTextureWrap3d::ClampToEdge,
            wrap_v: semio_framework_ui_scene_pass::SceneTextureWrap3d::MirrorRepeat,
            mag_filter: semio_framework_ui_scene_pass::SceneTextureFilter3d::Linear,
            min_filter: semio_framework_ui_scene_pass::SceneTextureFilter3d::Nearest,
        }
    );
    let asset = probe.take_ready_mesh_asset().expect("ready material asset transfers into request-owned Icon preparation");
    probe.finish_ready_mesh();
    let request = serde_json::json!({
        "assetUrl": "two-primitive.glb",
        "format": "png",
        "camera": { "projection": "perspective", "position": [3.0, -4.0, 2.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "zoom": 1.0, "fov": 50.0 },
        "fit": { "enabled": true, "padding": 1.25 },
        "width": 64,
        "height": 32,
        "shape": "ellipse"
    })
    .to_string();
    let mut preparation = semio_framework_os_renderer_wgpu_icon_host::icon_export::IconExportScenePreparation::new(request, asset).expect("request-owned Icon scene admits the full material asset");
    assert_eq!(preparation.dimensions(), (64, 32));
    assert_eq!(preparation.format(), semio_framework_os_renderer_wgpu_icon_host::icon_export::IconExportFormat::Png);
    for turn in 0..262_144 {
        match preparation.advance() {
            Ok(true) => break,
            Ok(false) => {}
            Err(fault) => panic!("request-owned Icon scene preparation faulted at turn {turn}: {fault}"),
        }
    }
    assert!(preparation.terminal(), "request-owned Icon scene preparation reaches its accepted boundary");
    let mut prepared = preparation.take_prepared().expect("the exact prepared scene owner is published once");
    assert_eq!(prepared.dimensions(), (64, 32));
    assert_eq!(prepared.format(), semio_framework_os_renderer_wgpu_icon_host::icon_export::IconExportFormat::Png);
    let mut packet = prepared.take_packet().expect("prepared scene transfers one packet while retaining its World owner");
    let uploads = packet.uploads();
    assert!((0..uploads.len()).any(|index| matches!(uploads.get(index), Some(semio_framework_ui_prepared_staging::PreparedRenderUpload::Mesh { .. }))));
    assert!((0..uploads.len()).any(|index| matches!(uploads.get(index), Some(semio_framework_ui_prepared_staging::PreparedRenderUpload::SceneRaster { .. }))));
    while !packet.retire_step() {}
    prepared.begin_close();
    for _ in 0..262_144 {
        if prepared.close_step() {
            break;
        }
    }
    assert!(prepared.terminal_is_empty(), "packet retirement lets the retained World asset close to exact terminal empty");
    close_owned_decoder_fixture(authority, probe);
}

/// 🪪️ Denied original authority preserves the decoder's exact input and empty receipt.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn decoder_denied_original_invocation_preserves_exact_owner_and_receipt() {
    use semio_framework_job::InteractiveJob;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖌️wgpu-document-owner-move/🔣️.json")).unwrap();
    let (authority, probe) = owned_decoder_fixture();
    let token = probe.owner().owner().token();
    let mut job = RendererAssetDecodeJob::new(probe);
    let mut caller = OriginalRendererFixture::new();
    let before = caller.receipt();
    let denied = semio_framework_job::RetainedCloneGrant { maximum_items: 0, ..renderer_law_stock() };
    let (pending, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.limited(denied, |cx| Ok(job.step(cx)?.is_none())).unwrap());
    assert!(pending);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(caller.receipt(), before);
    assert_eq!(serde_json::to_value(semio_framework_job::RetainedCloneProgress::default()).unwrap(), law["workerInvocation"]["expectedDeniedReceipt"]);
    assert_eq!(job.probe.borrow().as_ref().unwrap().owner().owner().token(), token);
    assert_eq!(job.probe.borrow().as_ref().unwrap().observed_bytes, 0);
    assert_eq!(job.steps, 0);
    assert!(job.result.is_none());
    let recovered = job.take_probe().expect("denied original job returns its exact unchanged response");
    job.begin_close();
    assert!(matches!(caller.turn(|cx|job.close_step(cx)).unwrap(), semio_framework_job::InteractiveJobCloseStep::Complete { .. }));
    assert!(job.terminal_is_empty());
    close_owned_decoder_fixture(authority, recovered);
}

/// 🛑️ One worker grant decodes one page, and cancellation returns that exact owner before disposal.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_preserves_its_exact_response_through_cancellation() {
    use semio_framework_job::{InteractiveJob, JobOutcomeBorrow};
    fn assert_send<T: Send>() {}
    assert_send::<RendererAssetDecodeJob>();
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();
    let (authority, probe) = owned_decoder_fixture();
    let token = probe.owner().owner().token();
    let mut job = RendererAssetDecodeJob::new(probe);
    let mut caller = OriginalRendererFixture::new();
    caller.turn(|cx| { let deadline=cx.deadline_us();cx.with_original_work_loan(1,deadline,|same| { assert!(job.step(same)?.is_none());assert_eq!(same.fuel_remaining(),0);Ok(()) }) }).unwrap();
    let observed = job.probe.borrow().as_ref().unwrap().observed_bytes;
    assert_eq!(observed,16*1024);
    caller.cancel();
    caller.turn(|cx| { assert!(matches!(job.step(cx)?,Some(JobOutcomeBorrow::Complete { .. })));Ok(()) }).unwrap();
    assert_eq!(job.result,Some(RendererAssetDecodeResult::Cancelled));
    assert_eq!(law["cancelledDecode"]["afterClose"],"cancelled");assert_eq!(job.steps,1);
    let recovered=job.take_probe().expect("host receives the exact terminal response");assert_eq!(recovered.owner().owner().token(),token);assert_eq!(recovered.observed_bytes,observed);
    job.begin_close();assert!(matches!(caller.turn(|cx|job.close_step(cx)).unwrap(),semio_framework_job::InteractiveJobCloseStep::Complete { .. }));assert!(job.terminal_is_empty());
    close_owned_decoder_fixture(authority,recovered);
}
