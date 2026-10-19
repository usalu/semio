//! 🛠️ The proctor end to end, the operator: the built binary probes, backs up while serving and restores elsewhere, erases one learner on request and prunes the registrations nobody played under.
//!
//! @see ../🦀️.rs — the end to end suite that mounts this module
//! @see ../../../🧫️fixtures/📚️catalog/🔣️.json — the course played

use super::*;

/// ⌨️ Run the built `proctor` binary over `data` with `stdin`, answering its exit success and stdout.
fn operator(data: &Path, port: u16, arguments: &[&str], stdin: &[u8]) -> (bool, Vec<u8>) {
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_proctor"))
        .args(arguments)
        .env("PROCTOR_DATA", data)
        .env("PROCTOR_PORT", port.to_string())
        .env_remove("PROCTOR_BIND")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("the proctor binary runs");
    child.stdin.take().expect("stdin").write_all(stdin).expect("stdin written");
    let output = child.wait_with_output().expect("the proctor binary exits");
    (output.status.success(), output.stdout)
}

/// 🗂️ The file names in `directory`, ascending.
fn files(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory).expect("listing").filter_map(Result::ok).map(|entry| entry.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    names
}

/// 🔍️ Whether any file of `directory` holds the bytes of `needle`.
fn holds(directory: &Path, needle: &str) -> bool {
    files(directory).iter().any(|file| std::fs::read(directory.join(file)).expect("file").windows(needle.len()).any(|window| window == needle.as_bytes()))
}

#[test]
fn the_operator_probes_backs_up_while_serving_and_restores_elsewhere() {
    let (data, elsewhere, backups, third) = (scratch("operator"), scratch("operator-restore"), scratch("operator-backups"), scratch("operator-restore-file"));
    let running = boot(&data.0, development());
    let (base, port) = (running.base.clone(), running.address.port());
    assert!(operator(&data.0, port, &["health"], b"").0, "a serving proctor is healthy");

    let (ada, run) = (id(0xada), id(0x100));
    accepted(&base, &json!({ "type": "identify-learner", "id": id(1), "learner": ada, "identity": { "kind": "pseudonym", "handle": "Ada" } }));
    play(&base, &ada, &run, "power", "medium", 10);
    let views = |base: &str| [json!({ "type": "learner", "learner": ada }), json!({ "type": "run", "run": run }), json!({ "type": "leaderboard", "period": "all-time", "learner": ada }), json!({ "type": "crowd", "quiz": "power" }), json!({ "type": "handle", "handle": "ada" }), json!({ "type": "course" })].map(|asked| view(base, &asked));
    let before = views(&base);

    let (backed_up, backup) = operator(&data.0, port, &["backup", "-"], b"");
    assert!(backed_up, "a backup is taken while the proctor serves");
    assert_eq!(&backup[..16], b"SQLite format 3\0");
    let directory = backups.0.to_string_lossy().into_owned();
    let (filed, printed) = operator(&data.0, port, &["backup", &directory], b"");
    let file = PathBuf::from(String::from_utf8(printed).expect("a path").trim_end());
    let name = file.file_name().expect("a file name").to_string_lossy().into_owned();
    assert!(filed && file.parent() == Some(backups.0.as_path()) && name.starts_with("proctor-2") && name.ends_with("Z.sqlite") && name.len() == 31, "a backup into a directory prints the timestamped file it wrote: {}", file.display());
    assert_eq!((files(&backups.0), std::fs::read(&file).expect("the backup").len()), (vec![name], backup.len()), "one whole file and nothing partial beside it");
    assert!(!operator(&data.0, port, &["backup", &file.to_string_lossy()], b"").0, "a backup is never overwritten");
    assert_eq!(files(&data.0).iter().filter(|name| name.contains(".backup-") || name.contains(".partial-")).count(), 0, "the spool file is gone");

    play(&base, &ada, &id(0x200), "homes", "medium", 100);
    assert_ne!(views(&base), before, "the proctor kept committing after the backup");
    assert!(!operator(&data.0, port, &["restore", "-"], &backup).0, "a serving proctor's database is never replaced");
    running.shut_down();
    assert!(!operator(&data.0, port, &["health"], b"").0, "a stopped proctor is not healthy");

    assert!(operator(&elsewhere.0, port, &["restore", "-"], &backup).0, "the backup restores into an empty directory");
    let restored = boot(&elsewhere.0, development());
    assert_eq!(views(&restored.base), before, "the proctor started on the copy answers the views of the moment the backup was taken");
    let completed = play(&restored.base, &ada, &id(0x300), "homes", "medium", 200);
    assert!(completed.iter().any(|event| event["badge"] == "complete"), "the restored authority appends after its history: {completed:?}");
    restored.shut_down();

    assert!(operator(&third.0, port, &["restore", &file.to_string_lossy()], b"").0, "the backup file restores as well");
    let restored = boot(&third.0, development());
    assert_eq!(views(&restored.base), before, "and answers the same");
    restored.shut_down();
}

#[test]
fn the_operator_erases_one_learner_on_request_and_its_handle_is_free_again() {
    const NAME: &str = "Ada Lovelace";
    let data = scratch("operator-erase");
    let running = boot(&data.0, development());
    let (base, port) = (running.base.clone(), running.address.port());
    let (ada, grace) = (id(0xada), id(0x9ace));
    accepted(&base, &json!({ "type": "identify-learner", "id": id(1), "learner": ada, "identity": { "kind": "name", "handle": NAME } }));
    accepted(&base, &json!({ "type": "identify-learner", "id": id(2), "learner": grace, "identity": { "kind": "anonymous" } }));
    play(&base, &ada, &id(0x100), "power", "medium", 10);
    play(&base, &grace, &id(0x101), "power", "medium", 100);
    let others = |base: &str| [json!({ "type": "learner", "learner": grace }), json!({ "type": "run", "run": id(0x101) })].map(|asked| view(base, &asked));
    let before = others(&base);
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": grace }));
    assert_eq!((board["learners"].as_u64(), board["own"]["rank"].as_u64(), board["rows"][0]["identity"]["handle"].as_str()), (Some(2), Some(2), Some(NAME)));
    assert_eq!(view(&base, &json!({ "type": "crowd", "quiz": "power" }))["runs"], 2);
    assert!(!operator(&data.0, port, &["erase", "--handle", NAME], b"").0, "nobody is erased from a database that is being served");
    running.shut_down();

    let tag = course::learner_tag(&ada);
    let (dry, report) = operator(&data.0, port, &["erase", "--tag", &tag, "--dry-run"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(dry && report.starts_with(&format!("would erase learner #{tag} name {NAME:?} of course {TENANT}\n")) && report.contains("  handle \"ada lovelace\": 1 events") && report.ends_with("dry run: nothing was changed\n"), "{report}");
    assert!(holds(&data.0, NAME) && holds(&data.0, &ada), "a dry run changes nothing");
    assert!(!operator(&data.0, port, &["erase", "--handle", "Grace Hopper"], b"").0, "nobody holds that handle");
    let (erased, report) = operator(&data.0, port, &["erase", "--handle", " ada  LOVELACE "], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(erased && report.starts_with(&format!("erased learner #{tag} name {NAME:?} of course {TENANT}\n")) && !report.contains(&ada), "{report}");
    assert!(!holds(&data.0, NAME) && !holds(&data.0, "ada lovelace") && !holds(&data.0, &ada) && holds(&data.0, &grace), "neither the name nor the learner id is left in any file of the data directory: {:?}", files(&data.0));
    assert!(!operator(&data.0, port, &["erase", "--learner", &ada], b"").0, "an erased learner is nobody");

    let running = boot(&data.0, development());
    let base = running.base.clone();
    assert_eq!(others(&base), before, "the other learner's views are what they were");
    assert_eq!(query(&base, &json!({ "type": "learner", "learner": ada })).0, 404);
    assert_eq!(query(&base, &json!({ "type": "run", "run": id(0x100) })).0, 404);
    assert_eq!(view(&base, &json!({ "type": "handle", "handle": NAME })), json!({ "display": NAME }), "the handle is free");
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": grace }));
    assert_eq!((board["learners"].as_u64(), board["own"]["rank"].as_u64(), board["rows"].as_array().map(Vec::len)), (Some(1), Some(1), Some(1)), "the erased scores are gone from the board: {board}");
    assert!(!board.to_string().contains(NAME) && !board.to_string().contains(&tag), "{board}");
    assert_eq!(view(&base, &json!({ "type": "crowd", "quiz": "power" }))["runs"], 1, "and from the crowd");
    assert_eq!(rejected(&base, &start(&ada, &id(0x102), "power", "medium", 20)), "unknown-learner");
    let successor = id(0xb0b);
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(21), "learner": successor, "identity": { "kind": "pseudonym", "handle": NAME } }));
    assert_eq!((events[0]["type"].as_str(), events[0]["learner"].as_str()), (Some("learner-registered"), Some(successor.as_str())), "somebody else takes the freed handle");
    assert_eq!(view(&base, &json!({ "type": "handle", "handle": NAME }))["holder"]["learner"], successor.as_str());
    running.shut_down();
}

/// 🧮️ How many registrations a proctor booted over the stopped `data` directory counts: what its
/// cap of learners is held against.
pub(super) fn registrations(data: &Path) -> u64 {
    let data = data.to_string_lossy().into_owned();
    tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().expect("runtime").block_on(async move {
        let course = Arc::new(load_course(&fixtures().join("📚️catalog/🔣️.json")).expect("fixture course"));
        let proctor = Proctor::assemble(StorageProfile::Embedded { data_dir: data }, course, development(), PresenceSettings::default()).await.expect("assembled");
        proctor.prepare().await.expect("prepared");
        proctor.reconcile().await.expect("reconciled");
        proctor.settle(&CancelToken::root_now(), |_| {}).await.expect("settled");
        proctor.admission().learners()
    })
}

#[test]
fn the_operator_prunes_the_registrations_nobody_played_under_and_every_player_stays() {
    let data = scratch("operator-prune");
    let running = boot(&data.0, development());
    let (base, port) = (running.base.clone(), running.address.port());
    let (ada, grace, hoarder, linus) = (id(0xada), id(0x9ace), id(0x5200), id(0x11a5));
    let register = |seed: u32, learner: &str, identity: Value| accepted(&base, &json!({ "type": "identify-learner", "id": id(seed), "learner": learner, "identity": identity }));
    register(1, &ada, json!({ "kind": "name", "handle": "Ada Lovelace" }));
    register(2, &grace, json!({ "kind": "anonymous" }));
    play(&base, &ada, &id(0x100), "power", "medium", 10);
    play(&base, &grace, &id(0x101), "power", "medium", 100);
    for junk in 0..40 {
        register(0x6000 + junk, &id(0x5000 + junk), json!({ "kind": "anonymous" }));
    }
    for junk in 0..20 {
        register(0x6100 + junk, &id(0x5100 + junk), json!({ "kind": "pseudonym", "handle": format!("Junk Number {junk:02}") }));
    }
    register(0x6200, &hoarder, json!({ "kind": "anonymous" }));
    for hoard in 0..5 {
        register(0x6201 + hoard, &hoarder, json!({ "kind": "pseudonym", "handle": format!("Hoard Number {hoard}") }));
    }
    for spare in 0..3 {
        register(0x6300 + spare, &ada, json!({ "kind": "pseudonym", "handle": format!("Spare Number {spare}") }));
    }
    register(3, &linus, json!({ "kind": "name", "handle": "Linus Halfway" }));
    accepted(&base, &start(&linus, &id(0x400), "power", "medium", 4));
    let players = |base: &str| [json!({ "type": "learner", "learner": ada }), json!({ "type": "run", "run": id(0x100) }), json!({ "type": "learner", "learner": grace }), json!({ "type": "leaderboard", "period": "all-time", "learner": grace }), json!({ "type": "crowd", "quiz": "power" }), json!({ "type": "handle", "handle": "ada lovelace" })].map(|asked| view(base, &asked));
    let before = players(&base);
    assert_eq!(view(&base, &json!({ "type": "handle", "handle": "junk number 07" }))["holder"]["learner"], id(0x5107).as_str());
    assert!(!operator(&data.0, port, &["prune", "--older-than", "1s"], b"").0, "nothing is pruned from a database that is being served");
    running.shut_down();
    assert_eq!(registrations(&data.0), 72, "forty-two anonymous learners and thirty handles");

    let junk = ["Junk Number 07", "junk number 07", "Hoard Number 3", "Spare Number 1", "spare number 1", "Linus Halfway"];
    let strangers = [id(0x5007), id(0x5107), id(0x6007), id(0x6107), hoarder.clone(), linus.clone(), course::handle_actor_id("junk number 07"), course::handle_actor_id("spare number 1")];
    for wrong in [&["prune"][..], &["prune", "--older-than", "7"], &["prune", "--older-than", "soon"], &["prune", "--dry-run"]] {
        assert!(!operator(&data.0, port, wrong, b"").0, "{wrong:?}");
    }
    let (recent, report) = operator(&data.0, port, &["prune", "--older-than", "1h"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(recent && report.starts_with(&format!("pruning 0 registrations of course {TENANT} older than 1h (registered before 2")) && report.ends_with("nothing to prune: nothing was changed\n"), "{report}");
    std::thread::sleep(Duration::from_millis(1500));
    let (dry, report) = operator(&data.0, port, &["prune", "--older-than", "1s", "--dry-run"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(dry && report.starts_with(&format!("would prune 70 registrations of course {TENANT} older than 1s (registered before 2")), "{report}");
    assert!(report.ends_with("Z)\n  learners that never submitted a run: 62 (41 anonymous, 20 under a pseudonym, 1 under a name), holding 26 handles\n  handles claimed beside the identity of a learner that stays: 3\n  staying: 2 learners (2 of them submitted a run), 1 handles, 2 registrations\ndry run: nothing was changed\n"), "{report}");
    assert!(junk.iter().all(|name| holds(&data.0, name)) && strangers.iter().all(|stranger| holds(&data.0, stranger)), "a dry run changes nothing");
    assert!(!junk.iter().any(|name| report.contains(name)) && !strangers.iter().any(|stranger| report.contains(stranger.as_str())), "and its report counts, it names nobody: {report}");
    assert_eq!(registrations(&data.0), 72);

    let (pruned, report) = operator(&data.0, port, &["prune", "--older-than", "1s"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(pruned && report.starts_with("pruning 70 registrations") && report.contains("\nremoved 91 of 91 streams: 92 events, ") && report.ends_with("done: the handles are free again, the learner ids are unknown to the proctor; 2 registrations are left\n"), "{report}");
    assert!(junk.iter().all(|name| !holds(&data.0, name)) && strangers.iter().all(|stranger| !holds(&data.0, stranger)), "no name, learner id, command id or handle stream of theirs is left in any file of the data directory: {:?}", files(&data.0));
    assert!(holds(&data.0, &ada) && holds(&data.0, &grace) && holds(&data.0, "Ada Lovelace"));
    let (again, report) = operator(&data.0, port, &["prune", "--older-than", "1s"], b"");
    assert!(again && String::from_utf8(report).expect("a report").ends_with("nothing to prune: nothing was changed\n"));
    assert_eq!(registrations(&data.0), 2, "the count the cap is held against went down to who played");

    let running = boot(&data.0, development());
    let base = running.base.clone();
    assert_eq!(players(&base), before, "the views of everybody who played are what they were");
    for gone in [id(0x5007), id(0x5107), hoarder, linus.clone()] {
        assert_eq!(query(&base, &json!({ "type": "learner", "learner": gone })).0, 404, "{gone}");
    }
    for free in ["Junk Number 07", "Hoard Number 3", "Spare Number 1", "Linus Halfway"] {
        assert_eq!(view(&base, &json!({ "type": "handle", "handle": free })), json!({ "display": free }), "the handle is free");
    }
    assert_eq!(rejected(&base, &start(&linus, &id(0x401), "power", "medium", 5)), "unknown-learner");
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(0x6007), "learner": id(0x5007), "identity": { "kind": "anonymous" } }));
    assert_eq!(events.len(), 1, "a pruned learner id and its command id are unknown again, not a replay");
    for (seed, successor, handle) in [(6, id(0xb0b), "junk number 07"), (7, id(0xb0c), "Spare Number 1")] {
        let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(seed), "learner": successor, "identity": { "kind": "name", "handle": handle } }));
        assert_eq!((events[0]["type"].as_str(), events[0]["identity"]["handle"].as_str()), (Some("learner-registered"), Some(handle)), "somebody takes a pruned handle");
        assert_eq!(view(&base, &json!({ "type": "learner", "learner": successor }))["identity"]["handle"], handle, "and is relayed to its learner, whoever the handle was relayed to before");
    }
    running.shut_down();
}
