/// 💼️ The live-job wire a child-process shard tells its parent, and the parent ledger rows it becomes.

#[cfg(test)]
mod child_process_job_wire {
    use semio_plugin_host_shard::{decode_live_jobs, encode_live_jobs, reconcile_remote_jobs, spawned_jobs_snapshot, cancel_spawned_job, LiveJob};
    use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/💼️live-jobs/🔣️.json")).expect("live job vectors")
    }

    fn jobs(value: &serde_json::Value) -> Vec<LiveJob> {
        value.as_array().unwrap().iter().map(|job| LiveJob { actor: job["actor"].as_u64().unwrap(), job: job["job"].as_u64().unwrap(), kind: job["kind"].as_str().unwrap().into(), steps: job["steps"].as_u64().unwrap(), reports_progress: job["reportsProgress"].as_bool().unwrap() }).collect()
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn the_wire_matches_the_pinned_vectors_and_round_trips() {
        for case in fixture()["cases"].as_array().unwrap() {
            let live = jobs(&case["jobs"]);
            let id = case["id"].as_str().unwrap();
            assert_eq!(hex(&encode_live_jobs(&live)), case["hex"].as_str().unwrap(), "{id}");
            assert_eq!(decode_live_jobs(&encode_live_jobs(&live)), Some(live), "{id}");
        }
    }

    #[test]
    fn truncated_trailing_oversized_and_malformed_bytes_are_refused() {
        let one = encode_live_jobs(&jobs(&fixture()["cases"][1]["jobs"]));
        assert_eq!(decode_live_jobs(&one[..one.len() - 1]), None);
        assert_eq!(decode_live_jobs(&[one.clone(), vec![0]].concat()), None);
        assert_eq!(decode_live_jobs(&u32::MAX.to_le_bytes()), None);
        let mut flag = one.clone();
        flag[28] = 2;
        assert_eq!(decode_live_jobs(&flag), None);
        let mut utf8 = one;
        *utf8.last_mut().unwrap() = 0xFF;
        assert_eq!(decode_live_jobs(&utf8), None);
    }

    #[test]
    fn a_child_shards_jobs_list_advance_cancel_through_the_child_and_end() {
        let owner = 0x00C0_FFEE_usize;
        let prefix = format!("p{owner:x}:");
        let mine = || spawned_jobs_snapshot().into_iter().filter(|row| row.job.key.starts_with(&prefix)).collect::<Vec<_>>();
        let cancels = Arc::new(AtomicUsize::new(0));
        let sent = Arc::new(std::sync::Mutex::new(Vec::new()));
        let cancel = { let (cancels, sent) = (cancels.clone(), sent.clone()); move |actor: u64, job: u64| { cancels.fetch_add(1, Ordering::Relaxed); sent.lock().unwrap().push((actor, job)); } };
        let live = |steps, reports_progress| vec![LiveJob { actor: 5, job: 6, kind: "semio.io-run".into(), steps, reports_progress }];
        reconcile_remote_jobs(owner, &live(1, false), cancel.clone());
        let opened = mine();
        assert_eq!((opened.len(), opened[0].steps, opened[0].reports_progress, opened[0].job.kind.as_str()), (1, 1, false, "semio.io-run"));
        reconcile_remote_jobs(owner, &live(1, false), cancel.clone());
        assert_eq!(mine()[0].progress_at_ms, opened[0].progress_at_ms);
        reconcile_remote_jobs(owner, &live(4, true), cancel.clone());
        assert_eq!((mine()[0].steps, mine()[0].reports_progress), (4, true));
        assert!(cancel_spawned_job(&mine()[0].job.key));
        assert!(!cancel_spawned_job(&mine()[0].job.key));
        assert_eq!((cancels.load(Ordering::Relaxed), sent.lock().unwrap().clone()), (1, vec![(5, 6)]));
        reconcile_remote_jobs(owner, &[], cancel);
        assert!(mine().is_empty());
    }
}
