use super::*;

#[test]
fn poll_limits_narrow_without_replacing_authenticated_identity() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../📊️poll-limits.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let array = |key: &str| vector[key].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect::<Vec<_>>();
        let requested = array("requested");
        let authorized = array("authorized");
        let expected = array("expected");
        let grant = |v: &[u64]| semio_framework_value::RetainedCloneGrant { maximum_items:v[0] as usize, maximum_copy_bytes:v[1] as usize, maximum_capacity_bytes:v[2] as usize, maximum_release_bytes:v[3] as usize, maximum_depth:v[4] as usize };
        let authority = semio_framework::kernel::OriginalJobBudget::from_scalar_arguments([1,2,3,4,authorized[7],authorized[5],authorized[0],authorized[1],authorized[2],authorized[3],authorized[4],authorized[0],authorized[1],authorized[2],authorized[3],authorized[4],authorized[5],authorized[7],0]).unwrap();
        let limits = PollRequestLimits { retained_policy:grant(&requested), work_fuel:requested[5], fuel:requested[6], deadline_us:requested[7], max_effects:13, max_patch_bytes:14, max_frames:15 };
        let actual = limits.narrow(authority);
        let p = actual.retained_policy;
        assert_eq!(&[p.maximum_items as u64,p.maximum_copy_bytes as u64,p.maximum_capacity_bytes as u64,p.maximum_release_bytes as u64,p.maximum_depth as u64,actual.work_fuel,actual.fuel,actual.deadline_us][..], expected.as_slice());
        assert_eq!(actual.retained, authority.input);
        assert_eq!((actual.max_effects,actual.max_patch_bytes,actual.max_frames),(13,14,15));
    }
}

#[test]
fn actor_input_serde_accepts_only_the_closed_tagged_corpus() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../📊️actor-input.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let wire = serde_json::to_vec(&vector["wire"]).unwrap();
        assert_eq!(serde_json::from_slice::<ActorInput>(&wire).is_ok(), vector["valid"].as_bool().unwrap());
    }
}
