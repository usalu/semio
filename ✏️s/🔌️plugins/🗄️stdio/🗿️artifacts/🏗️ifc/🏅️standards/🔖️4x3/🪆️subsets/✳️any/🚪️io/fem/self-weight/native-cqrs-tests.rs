//! 🏋️ Shared authored case flags traverse the actual canonical FEM mutation and inverse.
use super::{cantilever_fixture, round_trip, Fem3dMutation};

#[semio_framework_async_macros::async_test]
async fn self_weight_shared_flags_keep_case_scope_and_exact_inverse() {
    let flags: Vec<bool> = semio_framework_pack_json::from_json_str(include_str!("commands.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed boolean transitions");
    let (mut source, ..) = cantilever_fixture();
    let mut other = source.load_cases[0].clone();
    other.id = "other".into();
    source.load_cases.push(other);
    let untouched = source.load_cases[1].clone();
    for flag in flags {
        source = round_trip(
            &source,
            &Fem3dMutation::ChangeLoadCaseSelfWeight(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "point".into(), new_self_weight: flag }),
        );
        assert_eq!(source.load_cases[0].self_weight, flag);
        assert_eq!(source.load_cases[1], untouched);
    }
}
