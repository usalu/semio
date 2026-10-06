use super::*;

#[test]
fn job_completion_ownership_survives_newer_render_generations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🏁️job-completion-ownership/🔣️.json")).expect("neutral job ownership fixture");
    for case in fixture["cases"].as_array().unwrap() {
        let mut registry = JobRenderBindingRegistry::new();
        for step in case["steps"].as_array().unwrap() {
            let instance = step["instance"].as_u64().unwrap() as u32;
            let job = step["job"].as_str().unwrap().parse::<u64>().unwrap();
            match step["action"].as_str().unwrap() {
                "bind" => { registry.bind(instance, job).expect("admit independent job"); }
                "complete" => { registry.complete(job); }
                "close" => registry.close_instance(instance),
                _ => unreachable!(),
            }
            let mut owned: Vec<String> = registry.by_job.iter().flatten().map(|binding| binding.job.to_string()).collect();
            let mut accepted: Vec<String> = registry.by_job.iter().flatten().filter(|binding| registry.accepted(binding.job).is_some()).map(|binding| binding.job.to_string()).collect();
            owned.sort();
            accepted.sort();
            assert_eq!(serde_json::to_value(owned).unwrap(), step["owned"], "{}: ownership after {}", case["id"], step["action"]);
            assert_eq!(serde_json::to_value(accepted).unwrap(), step["accepted"], "{}: render freshness after {}", case["id"], step["action"]);
            for job in step["owned"].as_array().unwrap() {
                let job = job.as_str().unwrap().parse::<u64>().unwrap();
                assert_eq!(registry.owned(job).map(|binding| binding.job), Some(job));
            }
        }
    }
    println!("[DEBUG] Job completion ownership: five neutral multi-job histories preserve terminals and reject stale repaint");
}

#[test]
fn progress_accepts_only_the_current_instance_generation() {
    let mut registry = JobRenderBindingRegistry::new();
    let first = registry.bind(7, 41).expect("first binding");
    assert_eq!(registry.accepted(41), Some(first));
    let second = registry.bind(7, 42).expect("superseding binding");
    assert_ne!(first.generation, second.generation);
    assert_eq!(registry.owned(41), Some(first));
    assert_eq!(registry.accepted(41), None);
    assert_eq!(registry.accepted(42), Some(second));
    assert_eq!(registry.complete(41), None);
    assert_eq!(registry.owned(41), None);
    assert_eq!(registry.complete(42), Some(second));
}

#[test]
fn direct_slots_reject_collisions_and_close_exactly_one_instance() {
    let mut registry = JobRenderBindingRegistry::new();
    let binding = registry.bind(3, 9).expect("binding");
    assert!(registry.bind(3 + REACTOR_TASK_SLOTS as u32, 10).is_err());
    assert!(registry.bind(4, 9 + REACTOR_TASK_SLOTS as u64).is_err());
    let second = registry.bind(3, 10).expect("second independent job");
    assert!(registry.bind(4, 9 + REACTOR_TASK_SLOTS as u64).is_err());
    registry.close_instance(3);
    assert_eq!(registry.accepted(binding.job), None);
    assert_eq!(registry.owned(binding.job), None);
    assert_eq!(registry.owned(second.job), None);
}
