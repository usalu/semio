//! 🧪️ components tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{world_mesh_ownership_grant, world_scene_raster_pool};
use semio_framework_os_infinite_world_components::{WorldMeshBuffers, WorldMeshInput, WorldPlaceholderMeshCursor, WorldPlaceholderMeshStep};
use semio_framework_test_fixtures::fixture_text;
use ui_scene::math::{mesh3d_begin_close, mesh3d_close_step, Mesh3dField, Mesh3dLease};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::{SceneMaterialAlpha3d, SceneMaterialKind3d};

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

fn close_oracle_mesh(lease:Mesh3dLease) {
    loop {
        let demand=ui_scene::math::mesh3d_close_demands(lease,4096).unwrap();
        let grant=world_mesh_ownership_grant(4096,demand.capacity_bytes,demand.release_bytes,demand.depth);
        let step=mesh3d_close_step(lease,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

/// 🎨️ Native publication consumes the same indexed surface fixture as installed Three.js.
#[test]
fn authored_inline_surface_preserves_corner_face_channels_five_maps_and_cancellation() {
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../../🔨️modules/🖱️ui/🖌️render/🧪️tests/🧫️fixtures/🎨️world3d-inline-surface/🔣️.json")).unwrap();
    let text=law["mesh"].to_string();
    let mesh=semio_framework_pack_json::from_json_str::<WorldMeshBuffers>(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut cursor=WorldPlaceholderMeshCursor::inline("authored",mesh.clone(),817,1).unwrap();
    let mut phases=std::collections::BTreeSet::new();
    let texture_phase=|cursor:&WorldPlaceholderMeshCursor|cursor.texture_job.as_ref().map(|job|job.progress().phase).or_else(||cursor.texture_retirement.as_ref().map(|_|"retiring"));
    let mut texture_phases=std::collections::BTreeSet::new();
    let mut turns=0;
    let (lease,mut appearance)=loop {
        turns+=1;assert!(turns<32768);
        phases.insert(format!("{:?}",cursor.phase));
        if let Some(phase)=texture_phase(&cursor){texture_phases.insert(phase);}
        match cursor.step(){WorldPlaceholderMeshStep::Pending=>{},WorldPlaceholderMeshStep::Fault=>panic!("inline surface refused"),WorldPlaceholderMeshStep::Ready(_,lease,appearance)=>break(lease,appearance.unwrap())}
    };
    assert_eq!(lease.schema().unwrap().vertices,6);
    for item in 0..6u32 {
        for (field,key,width) in [(Mesh3dField::Normals,"normal",3usize),(Mesh3dField::Uvs,"uv",2),(Mesh3dField::Colors,"color",4)] {
            let actual=match width {2=>lease.vec2(field,item).unwrap().to_vec(),3=>lease.vec3(field,item).unwrap().to_vec(),_=>lease.vec4(field,item).unwrap().to_vec()};
            for axis in 0..width {assert!((actual[axis] as f64-law["expected"][key][item as usize*width+axis].as_f64().unwrap()).abs()<1e-6);}
        }
    }
    for item in 0..6 {
        assert_eq!(lease.vec4(Mesh3dField::Tangents,item).unwrap(),[0.0,1.0,0.0,-1.0]);
        for (field, selected) in [(Mesh3dField::UvsNormal,"uv63"),(Mesh3dField::UvsOcclusion,"uv1")] {
            let attribute=&mesh.attributes[if item<3 {selected}else{"uv0"}];
            let domain=if attribute.domain==semio_framework::MeshAttributeDomain::Vertex {mesh.indices[item as usize] as usize}else{item as usize};
            let expected=attribute.value_at(domain).unwrap().as_array().unwrap();let actual=lease.vec2(field,item).unwrap();
            for axis in 0..2 {assert!((actual[axis] as f64-expected[axis].as_f64().unwrap()).abs()<1e-6,"role {field:?} item {item} axis {axis}");}
        }
    }
    assert_eq!(appearance.primitives().len(),2);assert_eq!(appearance.texture_count(),2);
    let reference=image::load_from_memory(&mesh.textures["pixel"].bytes).unwrap().to_rgba8();
    let pool=world_scene_raster_pool();
    for(key,raster)in &appearance.textures{assert_eq!((raster.identity.descriptor().width,raster.identity.descriptor().height),(reference.width(),reference.height()));appearance.texture_upload(key,&pool).unwrap().with_rows(0,8,|pixels,rows|{assert_eq!(rows,reference.height());assert_eq!(pixels,reference.as_raw());}).unwrap();}
    assert!(cursor.texture_job.is_none()&&cursor.texture_retirement.is_none());
    let material=&appearance.primitives()[0].material;
    assert_eq!(SceneMaterialKind3d::Authored(material.clone()).texture_keys().count(),5);
    assert_eq!(material.texture_sampler.wrap_u,ui_scene::math::SceneTextureWrap3d::ClampToEdge);assert_eq!(material.texture_sampler.wrap_v,ui_scene::math::SceneTextureWrap3d::MirrorRepeat);assert_eq!(material.texture_sampler.mag_filter,ui_scene::math::SceneTextureFilter3d::Nearest);assert_eq!(material.texture_sampler.min_filter,ui_scene::math::SceneTextureFilter3d::Linear);assert_eq!(material.additional_texture_samplers[0],ui_scene::math::SceneTextureSampler3d{min_filter:ui_scene::math::SceneTextureFilter3d::LinearMipmapLinear,..material.texture_sampler});assert_eq!(material.additional_texture_samplers[1..],[material.texture_sampler;3]);
    assert_eq!(material.normal_scale,[0.5;2]);assert_eq!(material.occlusion_strength,0.25);
    assert_eq!(material.base_color[3],1.0);assert_eq!(material.alpha,SceneMaterialAlpha3d::Opaque);assert!(!material.double_sided);
    assert_eq!(appearance.primitives()[1].material.alpha,SceneMaterialAlpha3d::Blend);
    for phase in phases {
        let mut canceled=WorldPlaceholderMeshCursor::inline("canceled",mesh.clone(),818,1).unwrap();
        for _ in 0..32768 {if format!("{:?}",canceled.phase)==phase {break}assert!(matches!(canceled.step(),WorldPlaceholderMeshStep::Pending));}
        let mut steps=0;while !canceled.close_step(){steps+=1;assert!(steps<32768)}assert!(canceled.terminal_is_empty());
    }
    assert!(texture_phases.contains("retiring"));
    for phase in texture_phases{
        let mut canceled=WorldPlaceholderMeshCursor::inline("canceled-decoder",mesh.clone(),820,1).unwrap();
        for turn in 0..32768{if texture_phase(&canceled)==Some(phase){break}assert!(turn<32767);assert!(matches!(canceled.step(),WorldPlaceholderMeshStep::Pending));}
        let mut turns=0;while !canceled.close_step(){turns+=1;assert!(turns<32768)}assert!(canceled.terminal_is_empty());assert!(canceled.texture_job.is_none()&&canceled.texture_retirement.is_none());
    }
    while !appearance.close_step() {}mesh3d_begin_close(lease).unwrap();close_oracle_mesh(lease);
    assert!(cursor.terminal_is_empty());
}

/// ♻️ Retains the exact authored source through independent copy, capacity, release and depth refusals.
#[test]
fn world_inline_source_retirement_preserves_original_owner_and_full_receipts() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/♻️inline-source-ownership/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let mut mesh=WorldMeshBuffers::default();
        mesh.positions=row["positions"].as_array().unwrap().iter().map(|value|value.as_f64().unwrap() as f32).collect();
        mesh.indices=row["indices"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u32).collect();
        mesh.edge_positions=row["edgePositions"].as_array().unwrap().iter().map(|value|value.as_f64().unwrap() as f32).collect();
        mesh.edge_ids=row["edgeIds"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u32).collect();
        let mut cursor=WorldPlaceholderMeshCursor::inline(row["id"].as_str().unwrap(),mesh,819,1).unwrap();
        let pointers=|mesh:&Box<WorldMeshBuffers>|(mesh.as_ref() as *const WorldMeshBuffers,mesh.positions.as_ptr(),mesh.edge_positions.as_ptr(),mesh.indices.as_ptr());
        let original=match &cursor.source{WorldMeshInput::Inline(mesh)=>pointers(mesh),_=>panic!("inline source disappeared")};
        let zero=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
        assert!(matches!(cursor.retire_source_step(zero).unwrap(),RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()));
        assert_eq!(match &cursor.source{WorldMeshInput::Inline(mesh)=>pointers(mesh),_=>panic!("denied source moved")},original);
        let shallow=RetainedCloneGrant{maximum_items:1,maximum_depth:0,..zero};
        assert!(matches!(cursor.retire_source_step(shallow).unwrap(),RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()));
        assert_eq!(match &cursor.source{WorldMeshInput::Inline(mesh)=>pointers(mesh),_=>panic!("depth-denied source moved")},original);
        let grant=cursor.source_retirement_grant(4096).unwrap();
        assert_eq!(grant.maximum_capacity_bytes,law["ownership"]["inlineMoveCapacityBytes"].as_u64().unwrap() as usize);
        let step=cursor.retire_source_step(grant).unwrap();
        assert!(matches!(step,RetainedCloneStep::Progress(progress) if progress.copied_items==1 && progress.retained_capacity_bytes==0 && progress.fits(grant)));
        assert_eq!(pointers(cursor.source_retirement.as_ref().unwrap().original().unwrap()),original);
        let next=cursor.source_retirement_grant(4096).unwrap();
        assert!(next.maximum_capacity_bytes>0);
        let short=RetainedCloneGrant{maximum_capacity_bytes:next.maximum_capacity_bytes-1,..next};
        assert!(matches!(cursor.retire_source_step(short).unwrap(),RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()));
        assert_eq!(pointers(cursor.source_retirement.as_ref().unwrap().original().unwrap()),original);
        let mut turns=0;
        let mut release_refusals=0;
        loop {
            turns+=1;assert!(turns<32768);
            let grant=cursor.source_retirement_grant(4096).unwrap();
            if grant.maximum_release_bytes>0{
                let short=RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant};
                assert!(matches!(cursor.retire_source_step(short).unwrap(),RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()));
                assert!(cursor.source_retirement.is_some());
                release_refusals+=1;
            }
            match cursor.retire_source_step(grant).unwrap(){RetainedCloneStep::Progress(progress)=>assert!(progress.fits(grant)),RetainedCloneStep::Complete(progress)=>{assert!(progress.fits(grant));break}}
        }
        assert!(cursor.source_retirement.is_none());
        assert!(!matches!(cursor.source,WorldMeshInput::Inline(_)));
        assert!(release_refusals>0);
        let mut close_turns=0;while !cursor.close_step(){close_turns+=1;assert!(close_turns<32768)}
        assert!(cursor.terminal_is_empty());
    }
}

/// 🖼️ JPEG authored textures use the same image, role raster, publication and retirement owners.
#[test]
fn authored_inline_jpeg_surface_publishes_owned_role_rasters() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../🔨️modules/🖱️ui/🖌️render/🧪️tests/🧫️fixtures/🎨️world3d-inline-surface/🔣️.json")).unwrap();
    let row = &law["jpegPublication"];
    let mut mesh = semio_framework_pack_json::from_json_str::<WorldMeshBuffers>(&law["mesh"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let bytes: Vec<u8> = row["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let reference = image::load_from_memory_with_format(&bytes, image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let independent: Vec<u8> = row["decodedRgba"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    assert_eq!(reference.as_raw(), &independent);
    assert_eq!((reference.width(), reference.height()), (row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32));
    mesh.textures.get_mut("pixel").unwrap().mime = row["mime"].as_str().unwrap().into();
    mesh.textures.get_mut("pixel").unwrap().bytes = bytes;
    let mut cursor = WorldPlaceholderMeshCursor::inline("jpeg-authored", mesh, 819, 1).unwrap();
    let mut turns = 0;
    let (lease, mut appearance) = loop {
        turns += 1;
        assert!(turns < 131072);
        match cursor.step() {
            WorldPlaceholderMeshStep::Pending => std::thread::yield_now(),
            WorldPlaceholderMeshStep::Fault => panic!("JPEG authored surface refused"),
            WorldPlaceholderMeshStep::Ready(_, lease, appearance) => break (lease, appearance.unwrap()),
        }
    };
    assert_eq!(appearance.texture_count(), 2);
    let pool = world_scene_raster_pool();
    for (key, raster) in &appearance.textures {
        assert_eq!((raster.identity.descriptor().width, raster.identity.descriptor().height), (reference.width(), reference.height()));
        let texture = appearance.texture_upload(key, &pool).unwrap();
        texture.with_rows(0, 8, |pixels, rows| { assert_eq!(rows, 1); assert_eq!(pixels, reference.as_raw()); }).unwrap();
        drop(texture);
    }
    while !appearance.close_step() {}
    mesh3d_begin_close(lease).unwrap();
    close_oracle_mesh(lease);
    assert!(cursor.terminal_is_empty());
}
