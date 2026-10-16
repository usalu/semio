use super::*;
use crate::geometry::Rect;
use crate::kernel_3d_scene::ScenePass3d;
use crate::theme::Rgba;

#[test]
fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner() {
    let mut registry = FixedMeshGpuRegistry::default();
    for index in 0..MESH_GPU_TABLE_CAPACITY {
        let key = MeshGpuKey::new(&format!("mesh-{index}")).expect("bounded key");
        registry.insert(MeshGpuEntry { key, version: index as u64, value: Box::new(index) }).ok().expect("fixed mesh slot");
    }
    let rejected = Box::new(MESH_GPU_TABLE_CAPACITY);
    let rejected_pointer = (&*rejected) as *const usize;
    let rejected = registry.insert(MeshGpuEntry { key: MeshGpuKey::new("overflow").expect("bounded key"), version: 1, value: rejected }).expect_err("capacity plus one");
    assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
    assert_eq!(*rejected.value, MESH_GPU_TABLE_CAPACITY);
    let first = registry.take(0).expect("first exact owner");
    assert_eq!(*first.value, 0);
    registry.insert(rejected).ok().expect("returned owner retries after one slot retires");
    assert_eq!(registry.len, MESH_GPU_TABLE_CAPACITY);
}

#[test]
fn fixed_mesh_gpu_registry_version_identity_rejects_stale_aba_lookup() {
    let mut registry = FixedMeshGpuRegistry::default();
    registry.insert(MeshGpuEntry { key: MeshGpuKey::new("terrain").expect("bounded key"), version: 7, value: 70u64 }).ok().expect("first generation");
    assert_eq!(registry.get("terrain", 7), Some(&70));
    let old = registry.take(0).expect("old generation owner");
    assert_eq!(old.version, 7);
    registry.insert(MeshGpuEntry { key: MeshGpuKey::new("terrain").expect("bounded key"), version: 8, value: 80u64 }).ok().expect("replacement generation");
    assert!(registry.get("terrain", 7).is_none());
    assert_eq!(registry.get("terrain", 8), Some(&80));
}

#[test]
fn mesh_gpu_retirement_preserves_acknowledged_versions() {
    let key = MeshGpuKey::new("terrain").expect("bounded key");
    let other = MeshGpuKey::new("other").expect("bounded key");
    let mut versions = [0; MESH_GPU_KEEP_VERSION_CAPACITY];
    versions[..2].copy_from_slice(&[8, 9]);
    let selector = super::MeshGpuRetirementSelector::KeyExcept { key, versions, len: 2 };
    assert!(selector.selects(&MeshGpuEntry { key, version: 7, value: () }));
    assert!(!selector.selects(&MeshGpuEntry { key, version: 8, value: () }));
    assert!(!selector.selects(&MeshGpuEntry { key, version: 9, value: () }));
    assert!(!selector.selects(&MeshGpuEntry { key: other, version: 7, value: () }));
}


#[test]
fn world_mesh_instance_packs_policy_and_standard_material_into_one_fixed_stride() {
    let gpu = super::World3dGpuInstance::from_instance([0.0; 16], [1.0; 4], true, 0.2, 0.63, 0.27, true);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/🧱️world-instance-layout/🔣️.json")).unwrap();
    assert_eq!(std::mem::size_of::<super::World3dGpuInstance>(), fixture["stride"].as_u64().unwrap() as usize);
    assert_eq!(std::mem::align_of::<super::World3dGpuInstance>(), fixture["alignment"].as_u64().unwrap() as usize);
    let offsets = [std::mem::offset_of!(super::World3dGpuInstance, model0), std::mem::offset_of!(super::World3dGpuInstance, model1), std::mem::offset_of!(super::World3dGpuInstance, model2), std::mem::offset_of!(super::World3dGpuInstance, model3), std::mem::offset_of!(super::World3dGpuInstance, color), std::mem::offset_of!(super::World3dGpuInstance, flags), std::mem::offset_of!(super::World3dGpuInstance, emissive_cutoff), std::mem::offset_of!(super::World3dGpuInstance, surface_parameters)];
    for (index, offset) in offsets.into_iter().enumerate() {
        assert_eq!(offset, fixture["lanes"][index]["offset"].as_u64().unwrap() as usize);
    }
    for case in fixture["cases"].as_array().unwrap() {
        let model: [f32; 16] = serde_json::from_value(case["model"].clone()).unwrap();
        let color: [f32; 4] = serde_json::from_value(case["color"].clone()).unwrap();
        let expected: Vec<f32> = serde_json::from_value(case["expected"].clone()).unwrap();
        let packed = super::World3dGpuInstance::from_instance(model, color, case["preserveVertexColor"].as_bool().unwrap(), case["emissiveIntensity"].as_f64().unwrap() as f32, case["metalness"].as_f64().unwrap() as f32, case["roughness"].as_f64().unwrap() as f32, case["receivesShadow"].as_bool().unwrap());
        assert_eq!(bytemuck::cast_slice::<super::World3dGpuInstance, f32>(std::slice::from_ref(&packed)), expected.as_slice());
    }
    assert_eq!(gpu.surface_parameters, [1.0, 1.0, 1.0, 0.0]);
    assert_eq!(gpu.flags, [3.0, 0.2, 0.63, 0.27]);
    assert_eq!(gpu.emissive_cutoff, [0.0, 0.0, 0.0, -1.0]);
}
