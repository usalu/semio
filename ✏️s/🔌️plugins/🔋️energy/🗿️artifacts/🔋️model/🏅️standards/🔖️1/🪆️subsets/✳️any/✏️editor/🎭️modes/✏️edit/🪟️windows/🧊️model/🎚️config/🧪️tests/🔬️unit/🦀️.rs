use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn pose() -> EnergyModelCameraPose {
    EnergyModelCameraPose { position: [12.0, -9.0, 7.5], target: [4.0, 3.0, 1.35], zoom: 1.0 }
}

#[test]
fn the_default_pose_is_admissible_and_the_degenerate_ones_are_not() {
    assert!(EnergyModelCameraPose::default().is_valid());
    assert!(pose().is_valid());
    assert!(!EnergyModelCameraPose { zoom: 0.0, ..pose() }.is_valid(), "a zero zoom is not a camera");
    assert!(!EnergyModelCameraPose { zoom: -1.0, ..pose() }.is_valid());
    assert!(!EnergyModelCameraPose { position: [f64::NAN, 0.0, 0.0], ..pose() }.is_valid());
    assert!(!EnergyModelCameraPose { target: [0.0, f64::INFINITY, 0.0], ..pose() }.is_valid());
}

#[test]
fn a_pose_prints_the_scene_camera_json_the_world3d_host_reads() {
    let json = pose().scene_camera_json();
    assert!(json.contains("\"position\":[12.0,-9.0,7.5]"), "{json}");
    assert!(json.contains("\"target\":[4.0,3.0,1.35]"), "{json}");
    assert!(json.contains("\"up\":[0.0,0.0,1.0]"), "energy geometry is z-up: {json}");
}

#[test]
fn set_camera_replaces_the_pose_and_inverts_back_to_the_base() {
    let base = EnergyModelWindowConfig::default();
    let mutation = EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: pose() });
    let next = mutation.diff(&base).diff().apply(&base).expect("the camera applies");
    assert_eq!(next.camera, pose());
    let mut restored = next;
    for inverse in mutation.inverse(&base) {
        restored = inverse.diff(&restored).diff().apply(&restored).expect("the inverse applies");
    }
    assert_eq!(restored, base, "orbiting back is exactly the inverse");
}

#[test]
fn re_setting_the_same_pose_is_a_declared_no_op() {
    let base = EnergyModelWindowConfig { camera: pose() };
    let mutation = EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: pose() });
    assert!(mutation.diff(&base).warnings().iter().any(|warning| warning.code == "mutation.no-op"), "a debounced duplicate must not publish a change");
    assert!(mutation.inverse(&base).is_empty(), "a no-op has no inverse");
}

#[test]
fn the_window_config_round_trips_through_its_text_and_binary_codecs() {
    use store::{ArtifactDsl, ArtifactPack};
    let config = EnergyModelWindowConfig { camera: pose() };
    assert_eq!(EnergyModelWindowConfig::parse_dsl(&config.print_dsl()).expect("dsl round trip"), config);
    assert_eq!(EnergyModelWindowConfig::decode_pack(&config.encode_pack()).expect("pack round trip"), config);

    let mutation = EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: pose() });
    assert_eq!(EnergyModelWindowConfigMutation::parse_op(&mutation.print_op()).expect("text op"), mutation);
    assert_eq!(EnergyModelWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("binary op"), mutation);
}

#[test]
fn the_owner_binds_the_3d_window_kind() {
    use semio_framework_plugin::WindowConfigOwner;
    assert_eq!(<EnergyModelWindowConfigOwner as WindowConfigOwner>::WINDOW_KIND_ID, super::super::WINDOW_KIND_ID);
    assert_eq!(<EnergyModelWindowConfigOwner as WindowConfigOwner>::SCHEMA, "energy.model3dwindowconfig");
}
