//! 🧪️ The committed calibration mutation vectors `cameras`: one implementation case per fixture bundle, each checked against its vector by the shared law library.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../../../../🧪️tests/🔬️vectors/🦀️.rs"]
mod vectors;

#[path = "../../../../../🔭create-camera/🧪️tests/📷️adds/🦀️.rs"]
mod create_camera_tests_adds_a_fourth_97e912;
#[path = "../../../../../🔭create-camera/🧪️tests/📷️adds2/🦀️.rs"]
mod create_camera_tests_adds_the_cam_c_82c8fb;
#[path = "../../../../../🔭create-camera/🧪️tests/🚫️refuses/🦀️.rs"]
mod create_camera_tests_refuses_a_camera_e92a02;
#[path = "../../../../../🛠️update-camera/🧪️tests/🔁️warns/🦀️.rs"]
mod update_camera_tests_warns_that_the_697b4f;
#[path = "../../../../../🛠️update-camera/🧪️tests/🔍️refines/🦀️.rs"]
mod update_camera_tests_refines_the_9fd25a;
#[path = "../../../../../🛠️update-camera/🧪️tests/🔍️refines2/🦀️.rs"]
mod update_camera_tests_refines_the_cam_0eaef0;
#[path = "../../../../../🛠️update-camera/🧪️tests/🚫️refuses/🦀️.rs"]
mod update_camera_tests_refuses_to_b60a39;
#[path = "../../../../../🚫delete-camera/🧪️tests/⛓️refuses/🦀️.rs"]
mod delete_camera_tests_refuses_to_remove_3c8f32;
#[path = "../../../../../🚫delete-camera/🧪️tests/🚫️refuses/🦀️.rs"]
mod delete_camera_tests_refuses_to_73655a;
#[path = "../../../../../🚫delete-camera/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_camera_tests_removes_the_40cba4;
#[path = "../../../../../🚫delete-camera/🧪️tests/🚫️removes2/🦀️.rs"]
mod delete_camera_tests_removes_the_cam_f90b89;
#[path = "../../../../../⛓️create-rig/🧪️tests/🔗️adds/🦀️.rs"]
mod create_rig_tests_adds_a_rig_2df5df;
#[path = "../../../../../⛓️create-rig/🧪️tests/🔗️places/🦀️.rs"]
mod create_rig_tests_places_the_0d0b8d;
#[path = "../../../../../⛓️create-rig/🧪️tests/🚫️refuses/🦀️.rs"]
mod create_rig_tests_refuses_a_rig_cb71ba;
#[path = "../../../../../⛓️create-rig/🧪️tests/🚫️refuses2/🦀️.rs"]
mod create_rig_tests_refuses_a_second_95e04d;
#[path = "../../../../../✂️delete-rig-extrinsic/🧪️tests/⏮️unplaces/🦀️.rs"]
mod delete_rig_extrinsic_tests_unplaces_the_f5b35e;
#[path = "../../../../../✂️delete-rig-extrinsic/🧪️tests/✂️drops/🦀️.rs"]
mod delete_rig_extrinsic_tests_drops_the_cam_a_a1f8a2;
#[path = "../../../../../✂️delete-rig-extrinsic/🧪️tests/✂️unplaces/🦀️.rs"]
mod delete_rig_extrinsic_tests_unplaces_the_a39356;
#[path = "../../../../../✂️delete-rig-extrinsic/🧪️tests/🚫️refuses/🦀️.rs"]
mod delete_rig_extrinsic_tests_refuses_to_1805df;
#[path = "../../../../../🔩update-rig-extrinsic/🧪️tests/📍️retunes/🦀️.rs"]
mod update_rig_extrinsic_tests_retunes_the_675f52;
#[path = "../../../../../🔩update-rig-extrinsic/🧪️tests/📍️retunes2/🦀️.rs"]
mod update_rig_extrinsic_tests_retunes_the_cam_4ca5a2;
#[path = "../../../../../🔩update-rig-extrinsic/🧪️tests/🔁️warns/🦀️.rs"]
mod update_rig_extrinsic_tests_warns_that_the_89422a;
#[path = "../../../../../🔩update-rig-extrinsic/🧪️tests/🚫️refuses/🦀️.rs"]
mod update_rig_extrinsic_tests_refuses_to_2cfb53;
