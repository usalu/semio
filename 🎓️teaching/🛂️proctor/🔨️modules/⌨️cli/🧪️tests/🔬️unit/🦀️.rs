use super::*;
use crate::actors::tests::{id, ADA, BOB, TENANT};
use crate::catalog::tests::{fixture, fixture_path};
use crate::projections::tests::{bus, play, submit};
use crate::storage::tests::{scratch, Scratch};
use crate::storage::{ErasedActor, Inspection};
use quiz::Command;
use server::contract::CommandOutcome;

fn code(exit: ExitCode) -> String {
    format!("{exit:?}")
}

fn arguments(line: &[&str]) -> Vec<String> {
    line.iter().map(|argument| (*argument).to_string()).collect()
}

/// 🌱️ The environment of a proctor over `data` and the fixture catalog.
fn environment(data: &Path) -> impl Fn(&str) -> Option<String> {
    let (data, catalog) = (data.to_string_lossy().into_owned(), fixture_path().to_string_lossy().into_owned());
    move |name: &str| match name {
        "PROCTOR_DATA" => Some(data.clone()),
        "PROCTOR_CATALOG" => Some(catalog.clone()),
        _ => None,
    }
}

fn files(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory).expect("listing").filter_map(Result::ok).map(|entry| entry.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    names
}

/// 🎒️ A stopped proctor's data directory in which Ada (a name), Bob (anonymous) and Cy (a pseudonym)
/// registered and Ada and Bob each submitted a perfect run.
async fn played(label: &str) -> Scratch {
    let directory = scratch(label);
    let catalog = Arc::new(fixture());
    let mut bus = bus(&Database::open(&directory.0).expect("open"), &catalog).await;
    for (seed, learner, identity) in [(1, ADA, Identity::Name { handle: "Ada Lovelace".into() }), (2, BOB, Identity::Anonymous), (3, &*id(0xc), Identity::Pseudonym { handle: "Cy".into() })] {
        assert!(matches!(submit(&mut bus, &Command::IdentifyLearner { id: id(seed), learner: learner.into(), identity }, 100).await, CommandOutcome::Accepted { .. }));
    }
    play(&mut bus, &catalog, ADA, &id(10), "power", 200).await;
    play(&mut bus, &catalog, BOB, &id(11), "power", 300).await;
    directory
}

#[test]
fn check_accepts_the_fixture_and_refuses_a_broken_catalog() {
    assert_eq!(code(check(&fixture_path())), code(ExitCode::SUCCESS));
    let directory = scratch("cli-check");
    std::fs::write(directory.0.join("catalog.json"), "{\"schema\":\"semio.quiz.catalog/v1\"}").unwrap();
    assert_eq!(code(check(&directory.0.join("catalog.json"))), code(ExitCode::FAILURE));
}

#[tokio::test]
async fn a_wrong_command_line_prints_usage_and_exits_two() {
    let nothing = |_: &str| None;
    for line in [&[][..], &["check"], &["serve", "now"], &["ready"], &["health", "now"], &["backup"], &["backup", "a", "b"], &["restore"], &["erase"], &["erase", "--dry-run"], &["erase", "--handle"], &["erase", "--handle", "Ada", "--tag", "0badcafe"], &["erase", "--handle", "Ada", "--dry-run", "--dry-run"], &["erase", "Ada"], &["prune"], &["prune", "--dry-run"], &["prune", "--older-than"], &["prune", "--older-than", "7"], &["prune", "7d"], &["prune", "--older-than", "7d", "--dry-run", "--dry-run"]] {
        assert_eq!(code(run(&arguments(line), nothing).await), code(ExitCode::from(2)), "{line:?}");
    }
    assert_eq!(code(run(&arguments(&["serve"]), nothing).await), code(ExitCode::FAILURE));
    for verb in ["serve", "check", "rebuild", "health", "backup", "restore", "erase", "prune"] {
        assert!(USAGE.contains(&format!("proctor {verb}")), "{verb}");
    }
}

#[tokio::test]
async fn rebuild_refolds_a_data_directory() {
    let directory = scratch("cli-rebuild");
    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    assert!(directory.0.join(DATABASE_FILE).is_file());
}

#[tokio::test]
async fn rebuild_fails_over_a_log_it_cannot_read() {
    let directory = played("cli-unreadable").await;
    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    {
        let mut log = SqliteAuthorityStore::new(Database::open(&directory.0).expect("open"));
        let stream = learner_key(TENANT, BOB);
        let seq = log.last_seq(&stream).await.unwrap() + 1;
        let recalled = server::contract::EventRecord { stream: stream.clone(), seq, hlc: server::contract::HybridLogicalClock::default(), kind: "quiz.learner-recalled".into(), payload: format!(r#"{{"type":"learner-recalled","learner":"{BOB}","at":5}}"#).into_bytes() };
        log.append_events(&stream, &[recalled], &[]).await.unwrap();
    }
    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&directory.0)).await), code(ExitCode::FAILURE), "an event of another vocabulary is no reason to skip it");
}

#[tokio::test]
async fn a_backup_restores_into_another_data_directory() {
    let (source, target) = (played("cli-backup").await, scratch("cli-restore"));
    let copy = target.0.join("copy.sqlite").to_string_lossy().into_owned();
    assert_eq!(code(run(&arguments(&["backup", &copy]), environment(&source.0)).await), code(ExitCode::SUCCESS));
    let taken = Database::inspect(Path::new(&copy)).expect("a whole copy");
    assert_eq!(taken, Database::inspect(&source.0.join(DATABASE_FILE)).expect("the source"));
    assert!(taken.events > 10 && files(&target.0) == ["copy.sqlite"], "{taken:?} {:?}", files(&target.0));
    assert_eq!(code(run(&arguments(&["backup", &copy]), environment(&source.0)).await), code(ExitCode::FAILURE), "an existing backup is never overwritten");
    assert_eq!(code(run(&arguments(&["restore", &copy]), environment(&target.0)).await), code(ExitCode::SUCCESS));
    assert_eq!(Database::inspect(&target.0.join(DATABASE_FILE)).expect("restored"), taken);
    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&target.0)).await), code(ExitCode::SUCCESS), "the restored directory serves");
    std::fs::write(&copy, b"not a database").expect("garbage");
    assert_eq!(code(run(&arguments(&["restore", &copy]), environment(&target.0)).await), code(ExitCode::FAILURE));
    assert_eq!(files(&target.0).iter().filter(|name| name.contains(".restore-")).count(), 0, "a refused restore leaves no spool file");
    assert_eq!(Database::inspect(&target.0.join(DATABASE_FILE)).expect("still restored"), taken, "a refused restore leaves the database alone");
    assert_eq!(code(run(&arguments(&["backup", "x"]), |_: &str| None).await), code(ExitCode::FAILURE), "PROCTOR_DATA is required");
}

#[tokio::test]
async fn a_backup_into_a_directory_is_named_after_its_time_and_restores_while_the_source_is_served() {
    let (source, backups) = (played("cli-backup-directory").await, scratch("cli-backups"));
    let serving = Database::open(&source.0).expect("a serving handle");
    let existing = backups.0.to_string_lossy().into_owned();
    assert_eq!(code(run(&arguments(&["backup", &existing]), environment(&source.0)).await), code(ExitCode::SUCCESS));
    let created = format!("{}/nightly/", backups.0.to_string_lossy());
    assert_eq!(code(run(&arguments(&["backup", &created]), environment(&source.0)).await), code(ExitCode::SUCCESS), "a directory named with a trailing separator is created");
    let named = |directory: &Path| {
        let listed = files(directory);
        let [name] = listed.iter().filter(|name| name.ends_with(".sqlite")).collect::<Vec<_>>()[..] else { panic!("one backup in {listed:?}") };
        assert!(name.len() == "proctor-YYYYMMDDTHHMMSSZ.sqlite".len() && name.starts_with("proctor-2") && name.ends_with("Z.sqlite") && name.as_bytes()[16] == b'T', "{name}");
        assert!(!listed.iter().any(|name| name.contains(".partial-")), "{listed:?}");
        directory.join(name)
    };
    let (first, second) = (named(&backups.0), named(&backups.0.join("nightly")));
    assert_eq!(Database::inspect(&first).expect("whole"), Database::inspect(&second).expect("whole"));
    assert_eq!(files(&source.0).iter().filter(|name| name.contains(".backup-") || name.contains(".partial-")).count(), 0, "the data directory keeps no spool");
    let restored = scratch("cli-restored");
    assert_eq!(code(run(&arguments(&["restore", &first.to_string_lossy()]), environment(&restored.0)).await), code(ExitCode::SUCCESS));
    assert_eq!(code(run(&arguments(&["restore", &first.to_string_lossy()]), environment(&source.0)).await), code(ExitCode::FAILURE), "a served database is never replaced");
    drop(serving);
}

#[test]
fn a_backup_is_named_by_the_utc_second_it_was_taken() {
    let at = |seconds: u64| timestamp(UNIX_EPOCH + Duration::from_secs(seconds));
    assert_eq!([at(0), at(951_782_400), at(951_868_799), at(1_709_251_199), at(1_790_899_200), at(4_102_444_799)], ["19700101T000000Z", "20000229T000000Z", "20000229T235959Z", "20240229T235959Z", "20261002T000000Z", "20991231T235959Z"]);
    let directory = scratch("cli-backup-name");
    let moment = UNIX_EPOCH + Duration::from_secs(1_790_899_200);
    let inside = directory.0.to_string_lossy().into_owned();
    assert_eq!(backup_file(&inside, moment).unwrap(), directory.0.join("proctor-20261002T000000Z.sqlite"));
    assert_eq!(backup_file(&format!("{inside}/new/"), moment).unwrap(), directory.0.join("new").join("proctor-20261002T000000Z.sqlite"));
    assert!(directory.0.join("new").is_dir());
    assert_eq!(backup_file(&format!("{inside}/file.sqlite"), moment).unwrap(), directory.0.join("file.sqlite"));
    assert!(!directory.0.join("file.sqlite").exists());
}

#[tokio::test]
async fn health_asks_the_listener_as_the_proxy_would() {
    let answering = |status: &'static str| {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("listener");
        let port = listener.local_addr().expect("address").port().to_string();
        let served = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accepted");
            let mut request = [0_u8; 512];
            let read = std::io::Read::read(&mut stream, &mut request).expect("request");
            stream.write_all(format!("HTTP/1.1 {status}\r\ncontent-length: 0\r\n\r\n").as_bytes()).expect("answer");
            String::from_utf8_lossy(&request[..read]).into_owned()
        });
        (port, served)
    };
    let environment = |port: String| move |name: &str| (name == "PROCTOR_PORT").then(|| port.clone());
    let (port, served) = answering("200 OK");
    assert_eq!(code(run(&arguments(&["health"]), environment(port)).await), code(ExitCode::SUCCESS));
    let request = served.join().expect("served");
    assert!(request.starts_with("GET /instance HTTP/1.1\r\n") && request.contains("\r\nX-Forwarded-Proto: https\r\n"), "{request}");
    let (port, served) = answering("403 Forbidden");
    assert_eq!(code(run(&arguments(&["health"]), environment(port.clone())).await), code(ExitCode::FAILURE));
    served.join().expect("served");
    assert_eq!(code(run(&arguments(&["health"]), environment(port)).await), code(ExitCode::FAILURE), "a closed port is not healthy");
    let every_interface = |name: &str| match name {
        "PROCTOR_BIND" => Some("0.0.0.0".to_string()),
        "PROCTOR_PORT" => Some("8791".to_string()),
        _ => None,
    };
    assert_eq!(ProctorConfig::probe_address(every_interface).expect("address").to_string(), "127.0.0.1:8791");
}

#[test]
fn progress_is_announced_in_ten_percent_steps() {
    let mut reported = 0;
    let steps: Vec<u64> = [5, 12, 19, 40, 41, 100]
        .into_iter()
        .filter_map(|position| {
            let before = reported;
            announce("test", Progress { position, head: 100, folded: position }, &mut reported);
            (reported != before).then_some(reported)
        })
        .collect();
    assert_eq!(steps, [12, 40, 100]);
    let mut copied = 0;
    let bytes: Vec<u64> = [(0, 4096), (400, 4096), (2048, 4096), (2100, 4096), (4096, 4096), (9000, 4096), (0, 0)]
        .into_iter()
        .filter_map(|(written, size)| {
            let before = copied;
            announce_bytes(written, size, &mut copied);
            (copied != before).then_some(copied)
        })
        .collect();
    assert_eq!(bytes, [50, 100]);
    let mut pruned = 0;
    let streams: Vec<usize> = [(512, 5000), (600, 5000), (1024, 5000), (4999, 5000), (5000, 5000), (0, 0)]
        .into_iter()
        .filter_map(|(done, streams)| {
            let before = pruned;
            announce_pruned(done, streams, &mut pruned);
            (pruned != before).then_some(pruned)
        })
        .collect();
    assert_eq!(streams, [10, 20, 99, 100]);
}

#[test]
fn erase_takes_exactly_one_selector_and_at_most_one_dry_run() {
    assert_eq!(Selector::parse(&["--handle", "Ada Lovelace"]), Some((Selector::Handle("Ada Lovelace".into()), false)));
    assert_eq!(Selector::parse(&["--dry-run", "--tag", "0badcafe"]), Some((Selector::Tag("0badcafe".into()), true)));
    assert_eq!(Selector::parse(&["--learner", ADA, "--dry-run"]), Some((Selector::Learner(ADA.into()), true)));
    for refused in [&[][..], &["--dry-run"], &["--handle"], &["--handle", "a", "b"], &["--handle", "a", "--learner", ADA], &["--dry-run", "--tag", "0badcafe", "--dry-run"], &["--name", "a"]] {
        assert_eq!(Selector::parse(refused), None, "{refused:?}");
    }
}

#[tokio::test]
async fn a_selector_names_the_learner_and_every_handle_it_holds() {
    let directory = played("cli-subjects").await;
    let log = SqliteAuthorityStore::new(Database::open(&directory.0).expect("open"));
    let ada = Subject { tenant: TENANT.into(), learner: ADA.into(), identity: Some(Identity::Name { handle: "Ada Lovelace".into() }), handles: vec!["ada lovelace".into()] };
    let bob = Subject { tenant: TENANT.into(), learner: BOB.into(), identity: Some(Identity::Anonymous), handles: Vec::new() };
    assert_eq!(subjects(&log, &Selector::Handle("  ADA   lovelace ".into())).await.unwrap(), vec![ada.clone()], "a handle is found however it is typed");
    assert_eq!(subjects(&log, &Selector::Tag(quiz::learner_tag(ADA))).await.unwrap(), vec![ada.clone()]);
    assert_eq!(subjects(&log, &Selector::Learner(ADA.into())).await.unwrap(), vec![ada.clone()]);
    assert_eq!(subjects(&log, &Selector::Tag(quiz::learner_tag(BOB))).await.unwrap(), vec![bob.clone()]);
    assert_eq!(subjects(&log, &Selector::Handle("Grace".into())).await.unwrap(), Vec::new());
    assert_eq!(subjects(&log, &Selector::Learner(id(0x99))).await.unwrap(), Vec::new());
    assert_eq!(ada.actors(), vec![learner_key(TENANT, ADA), handle_key(TENANT, "ada lovelace")]);
    assert_eq!(bob.actors(), vec![learner_key(TENANT, BOB)]);
    for refused in [Selector::Handle("A\u{200b}da".into()), Selector::Tag("0BADCAFE".into()), Selector::Tag("cafe".into()), Selector::Learner("ada".into())] {
        assert!(subjects(&log, &refused).await.is_err(), "{refused:?}");
    }
    let erasure = Erasure { actors: vec![ErasedActor { actor: learner_key(TENANT, ADA), events: 9, receipts: 8, outbox: 9, snapshots: 0, leases: 0 }, ErasedActor { actor: handle_key(TENANT, "ada lovelace"), events: 1, receipts: 1, outbox: 1, snapshots: 0, leases: 0 }], projections: 12 };
    let dry = erasure_report(&ada, &erasure, true);
    assert_eq!(dry, format!("would erase learner #{} name \"Ada Lovelace\" of catalog {TENANT}\n  learner stream: 9 events, 8 receipts, 9 outbox rows, 0 snapshots, 0 leases\n  handle \"ada lovelace\": 1 events, 1 receipts, 1 outbox rows, 0 snapshots, 0 leases\n  read models: 12 rows dropped; the next start rebuilds them from the events that are left\ndry run: nothing was changed\n", quiz::learner_tag(ADA)));
    assert!(!dry.contains(ADA), "the report names the public tag, not the learner id");
    assert!(erasure_report(&bob, &erasure, false).starts_with(&format!("erased learner #{} anonymous of catalog", quiz::learner_tag(BOB))));
}

#[tokio::test]
async fn erase_removes_one_learner_from_a_stopped_proctor_and_frees_its_handle() {
    const NAME: &str = "Ada Lovelace";
    let directory = played("cli-erase").await;
    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    let holds = |needle: &str| files(&directory.0).iter().any(|file| std::fs::read(directory.0.join(file)).expect("file").windows(needle.len()).any(|window| window == needle.as_bytes()));
    let before = Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole");
    assert!(holds(NAME) && holds(ADA) && holds(BOB));

    let serving = Database::open(&directory.0).expect("a serving handle");
    assert_eq!(code(run(&arguments(&["erase", "--handle", NAME]), environment(&directory.0)).await), code(ExitCode::FAILURE), "a served database is never erased from");
    drop(serving);
    assert_eq!(code(run(&arguments(&["erase", "--handle", NAME, "--dry-run"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    assert_eq!(code(run(&arguments(&["erase", "--dry-run", "--tag", &quiz::learner_tag(ADA)]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    assert_eq!((Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole"), holds(NAME)), (before, true), "a dry run changes nothing");
    assert_eq!(code(run(&arguments(&["erase", "--handle", "Grace"]), environment(&directory.0)).await), code(ExitCode::FAILURE), "nobody holds it");
    assert_eq!(code(run(&arguments(&["erase", "--handle", "A\u{200b}da"]), environment(&directory.0)).await), code(ExitCode::FAILURE));
    assert_eq!(code(run(&arguments(&["erase", "--handle", NAME]), |_: &str| None).await), code(ExitCode::FAILURE), "PROCTOR_DATA is required");
    assert_eq!(Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole"), before);

    assert_eq!(code(run(&arguments(&["erase", "--handle", "ada  LOVELACE"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    let after = Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole");
    assert!(after.events < before.events && after.head == before.head, "{before:?} {after:?}");
    assert!(!holds(NAME) && !holds(&NAME.to_lowercase()) && !holds(ADA) && !holds(&quiz::handle_actor_id("ada lovelace")) && holds(BOB), "neither the name nor the learner id is in any file");
    assert_eq!(code(run(&arguments(&["erase", "--handle", NAME]), environment(&directory.0)).await), code(ExitCode::FAILURE), "an erased learner is nobody");
    assert_eq!(code(run(&arguments(&["erase", "--learner", ADA]), environment(&directory.0)).await), code(ExitCode::FAILURE));

    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&directory.0)).await), code(ExitCode::SUCCESS), "the proctor starts over the log that is left");
    assert!(!holds(NAME) && !holds(ADA), "the rebuilt read models do not know the learner");
    let catalog = Arc::new(fixture());
    let database = Database::open(&directory.0).expect("open");
    let projections = crate::storage::SqliteProjectionStore::new(database.clone());
    {
        use server::storage::ProjectionStore;
        assert_eq!(projections.get(crate::projections::LEARNERS, ADA).await, None);
        assert_eq!(projections.get(crate::projections::HANDLES, "ada lovelace").await, None);
        assert_eq!(projections.list(crate::projections::LEADERBOARD, "").await.iter().map(|(learner, _)| learner.clone()).collect::<Vec<_>>(), [BOB], "the scores of the erased learner are gone, the others stay");
        assert_eq!(projections.get(crate::projections::META, crate::projections::LEARNER_COUNT_KEY).await, Some(b"2".to_vec()));
    }
    let mut bus = bus(&database, &catalog).await;
    let claimed = submit(&mut bus, &Command::IdentifyLearner { id: id(0x50), learner: id(0xd), identity: Identity::Pseudonym { handle: NAME.into() } }, 900).await;
    assert!(matches!(claimed, CommandOutcome::Accepted { .. }), "the handle is free again: {claimed:?}");
    let reused = submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 901).await;
    assert!(matches!(reused, CommandOutcome::Accepted { .. }), "the learner id and its command ids are unknown again: {reused:?}");
    assert_eq!(Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole"), Inspection { events: after.events + 3, head: after.head + 3 }, "a claim, its enrollment and a registration");
}

#[test]
fn prune_takes_one_age_with_a_unit_and_at_most_one_dry_run() {
    assert_eq!([age("45s"), age("90m"), age("36h"), age("7d"), age("2w"), age("0s")].map(|age| age.map(|age| age.as_secs())), [Some(45), Some(5_400), Some(129_600), Some(604_800), Some(1_209_600), Some(0)]);
    for refused in ["", "7", "d", "7 d", " 7d", "-7d", "+7d", "7.5d", "7D", "7days", "1y", "٧d", "7é", "18446744073709551615w"] {
        assert_eq!(age(refused), None, "{refused:?}");
    }
    assert_eq!(Pruning::parse(&["--older-than", "7d"]), Some(Pruning { older_than: "7d".into(), age: Duration::from_secs(604_800), dry_run: false }));
    assert_eq!(Pruning::parse(&["--dry-run", "--older-than", "36h"]), Some(Pruning { older_than: "36h".into(), age: Duration::from_secs(129_600), dry_run: true }));
    assert_eq!(Pruning::parse(&["--older-than", "2w", "--dry-run"]).map(|pruning| pruning.dry_run), Some(true));
    for refused in [&[][..], &["--dry-run"], &["--older-than"], &["--older-than", "7"], &["--older-than", "7d", "8d"], &["--older-than", "7d", "--dry-run", "--dry-run"], &["7d"], &["--before", "7d"]] {
        assert_eq!(Pruning::parse(refused), None, "{refused:?}");
    }
}

const DORA: &str = "Dormant Dora";
const GUS: &str = "Gus Halfway";

/// 🕸️ The data directory of [`played`] with what nobody played under on top: beside Cy (a pseudonym
/// that never started a run) Dora (a pseudonym), Dee (anonymous), Fay (anonymous, holding two more
/// handles) and Gus (a name, who started a run and never submitted it), all registered at 100 ms;
/// Eve (anonymous) registered at 5000 ms; and two handles Ada claimed beside her name, one at
/// 100 ms and one at 5000 ms.
async fn cluttered(label: &str) -> Scratch {
    let directory = played(label).await;
    let catalog = Arc::new(fixture());
    let mut bus = bus(&Database::open(&directory.0).expect("open"), &catalog).await;
    let registrations = [
        (0x20, id(0xd0), Identity::Pseudonym { handle: DORA.into() }, 100),
        (0x21, id(0xd1), Identity::Anonymous, 100),
        (0x22, id(0xf0), Identity::Anonymous, 100),
        (0x23, id(0xf0), Identity::Pseudonym { handle: "Hoarded One".into() }, 100),
        (0x24, id(0xf0), Identity::Name { handle: "Hoarded Two".into() }, 100),
        (0x25, id(0xe0), Identity::Name { handle: GUS.into() }, 100),
        (0x26, id(0xe1), Identity::Anonymous, 5_000),
        (0x27, ADA.to_string(), Identity::Pseudonym { handle: "Spare Early".into() }, 100),
        (0x28, ADA.to_string(), Identity::Pseudonym { handle: "Spare Late".into() }, 5_000),
    ];
    for (seed, learner, identity, millis) in registrations {
        assert!(matches!(submit(&mut bus, &Command::IdentifyLearner { id: id(seed), learner, identity }, millis).await, CommandOutcome::Accepted { .. }), "registration {seed:#x}");
    }
    crate::projections::tests::answer(&mut bus, &catalog, &id(0xe0), &id(0x12), "power", 200).await;
    directory
}

#[tokio::test]
async fn the_stale_registrations_are_those_nobody_played_under_before_the_horizon() {
    let directory = cluttered("cli-stale").await;
    let log = SqliteAuthorityStore::new(Database::open(&directory.0).expect("open"));
    let subject = |learner: String, identity: Identity, handles: &[&str]| Subject { tenant: TENANT.into(), learner, identity: Some(identity), handles: handles.iter().map(|handle| (*handle).to_string()).collect() };
    let found = stale(&log, 1_000).expect("read");
    let [stale_now] = found.as_slice() else { panic!("one catalog: {found:?}") };
    let expected = Stale {
        tenant: TENANT.into(),
        learners: vec![
            subject(id(0xc), Identity::Pseudonym { handle: "Cy".into() }, &["cy"]),
            subject(id(0xd0), Identity::Pseudonym { handle: DORA.into() }, &["dormant dora"]),
            subject(id(0xd1), Identity::Anonymous, &[]),
            subject(id(0xe0), Identity::Name { handle: GUS.into() }, &["gus halfway"]),
            subject(id(0xf0), Identity::Anonymous, &["hoarded one", "hoarded two"]),
        ],
        surplus: vec!["spare early".into()],
        kept_learners: 3,
        kept_played: 2,
        kept_handles: 2,
        registrations_left: 4,
    };
    assert_eq!(stale_now, &expected, "whoever submitted a run stays whenever it registered, and so does what was registered since the horizon");
    assert_eq!(stale_now.registrations(), 8, "two anonymous learners, the five handles of the learners and the handle beside Ada's name");
    let batches = stale_now.batches();
    let relay = enrollment_receipt(TENANT, &quiz::handle_actor_id("spare early"));
    assert_eq!(batches, vec![Batch { actors: expected.learners.iter().flat_map(Subject::actors).collect(), relays: Vec::new() }, Batch { actors: vec![handle_key(TENANT, "spare early")], relays: vec![relay.clone()] }], "a learner goes in one transaction with every handle it holds; a handle beside an identity goes with the receipt of its relay");
    assert_eq!((batches[0].actors.len(), log.receipt(&relay).await.expect("read").map(|receipt| receipt.actor)), (10, Some(learner_key(TENANT, ADA))), "the receipt of that relay is one Ada's stream holds");

    let nothing = &stale(&log, 100).expect("read")[0];
    assert_eq!((nothing.learners.len(), nothing.surplus.len(), nothing.kept_learners, nothing.kept_handles, nothing.registrations_left, nothing.registrations(), nothing.batches().len()), (0, 0, 8, 8, 12, 0, 0), "a registration at the horizon is not older than it");
    let everything = &stale(&log, 10_000).expect("read")[0];
    assert_eq!((everything.learners.len(), everything.surplus.clone(), everything.kept_learners, everything.kept_played, everything.kept_handles, everything.registrations_left), (6, vec!["spare early".to_string(), "spare late".to_string()], 2, 2, 1, 2));

    let pruning = Pruning { older_than: "7d".into(), age: Duration::from_secs(604_800), dry_run: true };
    let report = pruning_report(&found, &pruning, UNIX_EPOCH + Duration::from_secs(1_790_899_200));
    assert_eq!(
        report,
        format!("would prune 8 registrations of catalog {TENANT} older than 7d (registered before 20261002T000000Z)\n  learners that never submitted a run: 5 (2 anonymous, 2 under a pseudonym, 1 under a name), holding 5 handles\n  handles claimed beside the identity of a learner that stays: 1\n  staying: 3 learners (2 of them submitted a run), 2 handles, 4 registrations\ndry run: nothing was changed\n")
    );
    assert!(!report.contains(&id(0xd1)) && !report.contains("Dora") && !report.contains("dora"), "the report counts, it names nobody");
    let real = Pruning { dry_run: false, ..pruning };
    assert!(pruning_report(&found, &real, UNIX_EPOCH).starts_with("pruning 8 registrations of catalog") && !pruning_report(&found, &real, UNIX_EPOCH).contains("nothing"));
    assert!(pruning_report(&stale(&log, 100).expect("read"), &real, UNIX_EPOCH).ends_with("nothing to prune: nothing was changed\n"));
    assert_eq!(pruning_report(&[], &real, UNIX_EPOCH), "the database holds no registration\nnothing to prune: nothing was changed\n");
}

#[tokio::test]
async fn prune_removes_what_nobody_played_under_from_a_stopped_proctor_and_every_player_stays() {
    let directory = cluttered("cli-prune").await;
    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    let holds = |needle: &str| files(&directory.0).iter().any(|file| std::fs::read(directory.0.join(file)).expect("file").windows(needle.len()).any(|window| window == needle.as_bytes()));
    let registrations = || async {
        use server::storage::ProjectionStore;
        let projections = crate::storage::SqliteProjectionStore::new(Database::open(&directory.0).expect("open"));
        projections.get(crate::projections::META, crate::projections::LEARNER_COUNT_KEY).await.map(|count| String::from_utf8(count).expect("a number"))
    };
    let before = Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole");
    assert_eq!(registrations().await.as_deref(), Some("12"), "four anonymous learners and eight handles count as registrations");
    let junk = [DORA, "dormant dora", GUS, "Hoarded One", "hoarded two", "Spare Early", "spare late"];
    assert!(junk.iter().all(|name| holds(name)) && [id(0xd0), id(0xd1), id(0xe0), id(0xe1), id(0xf0)].iter().all(|learner| holds(learner)));

    let serving = Database::open(&directory.0).expect("a serving handle");
    assert_eq!(code(run(&arguments(&["prune", "--older-than", "1s"]), environment(&directory.0)).await), code(ExitCode::FAILURE), "a served database is never pruned");
    drop(serving);
    assert_eq!(code(run(&arguments(&["prune", "--older-than", "1s", "--dry-run"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    assert_eq!(code(run(&arguments(&["prune", "--dry-run", "--older-than", "2w"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    assert_eq!((Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole"), holds(DORA), registrations().await.as_deref()), (before, true, Some("12")), "a dry run changes nothing, the read models included");
    assert_eq!(code(run(&arguments(&["prune", "--older-than", "1s"]), |_: &str| None).await), code(ExitCode::FAILURE), "PROCTOR_DATA is required");
    assert_eq!(code(run(&arguments(&["prune", "--older-than", "20000w"]), environment(&directory.0)).await), code(ExitCode::SUCCESS), "an age nothing has reached prunes nothing");
    assert_eq!((Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole"), registrations().await.as_deref()), (before, Some("12")), "and leaves the read models where they are");

    assert_eq!(code(run(&arguments(&["prune", "--older-than", "1s"]), environment(&directory.0)).await), code(ExitCode::SUCCESS));
    let after = Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole");
    assert_eq!(after.events + 16, before.events, "the nine events of six learners, their five handles and the two handles beside Ada's name: {before:?} {after:?}");
    assert!(junk.iter().all(|name| !holds(name)), "no name nobody played under is a byte of any file: {:?}", junk.iter().filter(|name| holds(name)).collect::<Vec<_>>());
    assert!([id(0xd0), id(0xd1), id(0xe0), id(0xe1), id(0xf0), quiz::handle_actor_id("dormant dora"), quiz::handle_actor_id("spare early")].iter().all(|gone| !holds(gone)), "nor is a learner id or a handle stream of theirs");
    assert!(holds(ADA) && holds(BOB) && holds("Ada Lovelace"), "who played is still there");
    assert_eq!(registrations().await, None, "the read models are dropped");
    assert_eq!(code(run(&arguments(&["prune", "--older-than", "1s"]), environment(&directory.0)).await), code(ExitCode::SUCCESS), "a second prune finds nothing");
    assert_eq!(Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole"), after);

    assert_eq!(code(run(&arguments(&["rebuild"]), environment(&directory.0)).await), code(ExitCode::SUCCESS), "the proctor starts over the log that is left");
    assert_eq!(registrations().await.as_deref(), Some("2"), "the count of registrations went down to Bob and Ada's name");
    assert!(junk.iter().all(|name| !holds(name)), "the rebuilt read models know none of them");
    let catalog = Arc::new(fixture());
    let database = Database::open(&directory.0).expect("open");
    {
        use server::storage::ProjectionStore;
        let projections = crate::storage::SqliteProjectionStore::new(database.clone());
        assert_eq!(projections.list(crate::projections::LEADERBOARD, "").await.iter().map(|(learner, _)| learner.clone()).collect::<Vec<_>>(), [ADA, BOB], "the standings of those who played are what they were");
        assert_eq!(projections.list(crate::projections::HANDLES, "").await.iter().map(|(key, _)| key.clone()).collect::<Vec<_>>(), ["ada lovelace"]);
        assert_eq!((projections.get(crate::projections::LEARNERS, &id(0xd1)).await, projections.get(crate::projections::LEARNERS, &id(0xe0)).await), (None, None));
    }
    let mut bus = bus(&database, &catalog).await;
    let claimed = submit(&mut bus, &Command::IdentifyLearner { id: id(0x60), learner: id(0x61), identity: Identity::Name { handle: DORA.into() } }, 900).await;
    assert!(matches!(claimed, CommandOutcome::Accepted { ref events, .. } if events.len() == 1), "a pruned handle is free again: {claimed:?}");
    let reused = submit(&mut bus, &Command::IdentifyLearner { id: id(0x21), learner: id(0xd1), identity: Identity::Anonymous }, 901).await;
    assert!(matches!(reused, CommandOutcome::Accepted { ref events, .. } if events.len() == 1), "a pruned learner id and its command id are unknown again: {reused:?}");
    let stranger = submit(&mut bus, &Command::StartRun { id: id(0x62), learner: id(0xe0), run: id(0x63), quiz: "power".into() }, 902).await;
    assert!(matches!(stranger, CommandOutcome::Rejected { .. }), "a pruned learner starts no run: {stranger:?}");
    let successor = submit(&mut bus, &Command::IdentifyLearner { id: id(0x64), learner: id(0x65), identity: Identity::Pseudonym { handle: "Spare Early".into() } }, 903).await;
    assert!(matches!(successor, CommandOutcome::Accepted { ref events, .. } if events.len() == 1), "{successor:?}");
    let log = SqliteAuthorityStore::new(database.clone());
    assert_eq!(log.last_seq(&learner_key(TENANT, &id(0x65))).await.expect("read"), 1, "a handle that went from beside Ada's name is relayed to whoever claims it next: the receipt of its first relay went with it");
}
