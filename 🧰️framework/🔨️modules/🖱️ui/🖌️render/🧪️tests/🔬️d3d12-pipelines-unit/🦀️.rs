
use super::*;

#[test]
fn world3d_gpu_instance_layout_matches_the_hlsl_input_layout_strides() {
    assert_eq!(std::mem::size_of::<World3dGpuInstance>(), 96);
    assert_eq!(std::mem::size_of::<World3dGpuVertex>(), 24);
    assert_eq!(std::mem::size_of::<WorldLineGpuVertex>(), 28);
    assert_eq!(std::mem::size_of::<QuadInstance>(), 64);
    assert_eq!(std::mem::size_of::<VectorVertex>(), 24);
    assert_eq!(std::mem::size_of::<GlassInstance>(), 48);
}

#[test]
fn ui_input_layout_has_five_attributes_across_two_slots() {
    let layout = ui_layout();
    assert_eq!(layout.len(), 5);
    assert_eq!(layout[0].InputSlot, 0);
    assert!(layout[1..].iter().all(|element| element.InputSlot == 1));
}

#[test]
fn world3d_mesh_input_layout_has_eight_attributes_across_two_slots() {
    let layout = world3d_mesh_layout();
    assert_eq!(layout.len(), 8);
    assert_eq!(layout[0].InputSlot, 0);
    assert_eq!(layout[1].InputSlot, 0);
    assert!(layout[2..].iter().all(|element| element.InputSlot == 1 && element.InstanceDataStepRate == 1));
}
