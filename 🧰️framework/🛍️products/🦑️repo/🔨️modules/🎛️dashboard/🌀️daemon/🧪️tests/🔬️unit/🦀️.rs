use super::*;
use crate::registry::{Launch, LaunchProcess, Stop};
use ipc::{ClientMsg, Ready, ServerMsg, SessionInfo, SessionStatus, SpawnGroup, TaskLabel};
use serde_json::Value;
use std::collections::{BTreeSet, HashMap};

fn control_plane() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🌀️control-plane/🔣️.json")).unwrap()
}

fn schema() -> Value {
    serde_json::from_str(include_str!("../../../🧬️schema/🌀️daemon/🔣️.json")).unwrap()
}

fn reencode(value: &Value) -> Option<Value> {
    if let Ok(message) = serde_json::from_value::<ClientMsg>(value.clone()) {
        return message.validate().ok().and_then(|()| serde_json::to_value(&message).ok());
    }
    let message = serde_json::from_value::<ServerMsg>(value.clone()).ok()?;
    message.validate().ok()?;
    serde_json::to_value(&message).ok()
}

fn temporary(name: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let directory = std::env::temp_dir().join(format!("semio-daemon-{name}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

#[test]
fn unknown_subcommand_returns_usage_without_side_effects() {
    let parsed = ParsedArgs { verb: "daemon".into(), segments: vec!["unknown".into()], flags: HashMap::new() };
    assert_eq!(run(Path::new("."), &parsed), 1);
}

// #region 🔖️Protocol
#[test]
fn every_valid_protocol_vector_survives_the_rust_codec_unchanged() {
    for value in control_plane()["valid"].as_array().unwrap() {
        assert_eq!(reencode(value).as_ref(), Some(value), "valid protocol vector must round trip exactly: {value}");
    }
}

#[test]
fn every_invalid_protocol_vector_is_refused_by_the_rust_codec() {
    for value in control_plane()["invalid"].as_array().unwrap() {
        assert!(reencode(value).is_none(), "invalid protocol vector: {value}");
    }
}

#[test]
fn the_vectors_cover_every_message_status_and_error_code_of_the_schema() {
    let (schema, vectors) = (schema(), control_plane());
    let valid = vectors["valid"].as_array().unwrap();
    let types = |name: &str| -> Vec<String> { schema["definitions"][name]["oneOf"].as_array().unwrap().iter().map(|variant| variant["properties"]["type"]["const"].as_str().unwrap().to_string()).collect() };
    for kind in types("ClientMessage").into_iter().chain(types("ServerMessage")) {
        assert!(valid.iter().any(|value| value["type"] == kind), "no valid vector for message type {kind}");
    }
    let statuses = schema["definitions"]["Session"]["properties"]["status"]["enum"].as_array().unwrap();
    let in_sessions = |status: &Value| valid.iter().any(|value| value["session"]["status"] == *status || value["sessions"].as_array().is_some_and(|sessions| sessions.iter().any(|session| session["status"] == *status)));
    for status in statuses { assert!(in_sessions(status), "no valid vector for session status {status}"); }
    let errors = schema["definitions"]["ServerMessage"]["oneOf"].as_array().unwrap().iter().find(|variant| variant["properties"]["type"]["const"] == "error").unwrap();
    for code in errors["properties"]["code"]["enum"].as_array().unwrap() { assert!(valid.iter().any(|value| value["code"] == *code), "no valid vector for error code {code}"); }
}

#[test]
fn the_error_codes_of_the_rust_wire_are_the_codes_of_the_schema() {
    let schema = schema();
    let errors = schema["definitions"]["ServerMessage"]["oneOf"].as_array().unwrap().iter().find(|variant| variant["properties"]["type"]["const"] == "error").unwrap();
    let listed: BTreeSet<&str> = errors["properties"]["code"]["enum"].as_array().unwrap().iter().filter_map(Value::as_str).collect();
    let named: BTreeSet<&str> = ipc::ErrorCode::ALL.into_iter().map(ipc::ErrorCode::as_str).collect();
    assert_eq!(listed, named);
}

#[test]
fn frames_match_the_shared_bytes() {
    let hex = |text: &str| -> Vec<u8> { (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).unwrap()).collect() };
    for frame in control_plane()["frames"].as_array().unwrap() {
        let (kind, session_id, data) = (frame["kind"].as_u64().unwrap() as u8, frame["session_id"].as_str().unwrap(), hex(frame["data_hex"].as_str().unwrap()));
        let built = ipc::chunk_frame(kind, session_id, &data).unwrap();
        assert_eq!(built, hex(frame["frame_hex"].as_str().unwrap()), "{}", frame["name"]);
        let mut buffer = ipc::FrameBuffer::default();
        buffer.extend(&built);
        let (read_kind, payload) = buffer.next_frame().unwrap().unwrap();
        assert_eq!(read_kind, kind);
        assert_eq!(ipc::decode_chunk(payload).unwrap(), (session_id, &data[..]));
    }
}

#[test]
fn control_messages_travel_in_frames_that_buffer_across_arbitrary_cuts() {
    let mut encoded = Vec::new();
    ipc::write_control(&mut encoded, &ClientMsg::Ping {}).unwrap();
    ipc::write_control(&mut encoded, &ClientMsg::Detach {}).unwrap();
    let mut buffer = ipc::FrameBuffer::default();
    let mut decoded = Vec::new();
    for byte in &encoded {
        buffer.extend(&[*byte]);
        while let Some((kind, payload)) = buffer.next_frame().unwrap() {
            assert_eq!(kind, ipc::KIND_CONTROL);
            decoded.push(ipc::decode_control::<ClientMsg>(payload).unwrap());
        }
    }
    assert_eq!(decoded, vec![ClientMsg::Ping {}, ClientMsg::Detach {}]);
    let mut bounded = ((ipc::MAX_FRAME_BYTES as u32) + 1).to_le_bytes().to_vec();
    assert!(ipc::try_decode_frame(&mut bounded).is_err(), "an oversized frame must fail");
    let mut buffer = ipc::FrameBuffer::default();
    buffer.extend(&0u32.to_le_bytes());
    assert!(buffer.next_frame().is_err(), "an empty frame must fail");
}

#[test]
fn endpoint_names_are_stable_across_runs_and_toolchains() {
    let root = temporary("endpoint");
    let key = ipc::workspace_key(&root);
    assert_eq!(key.len(), 16);
    assert_eq!(key, ipc::workspace_key(&root), "the name of a workspace must not change between calls");
    assert_eq!(format!("{:016x}", ipc::stable_hash(b"")), "2d06800538d394c2", "XXH3-64 of the empty input is the published constant");
    let _ = std::fs::remove_dir_all(root);
}
// #endregion 🔖️Protocol

// #region 🔖️Ready
#[test]
fn the_ready_table_decides_every_case() {
    let table = serde_json::from_str::<Value>(include_str!("../../../🧫️fixtures/🟢️ready/🔣️.json")).unwrap();
    for case in table["cases"].as_array().unwrap() {
        let ready: Ready = serde_json::from_value(case["ready"].clone()).unwrap();
        let chunks: Vec<&str> = case["chunks"].as_array().unwrap().iter().map(|chunk| chunk.as_str().unwrap()).collect();
        let expected = case["url"].as_str().map(str::to_string);
        let whole = {
            let mut tracker = replay::Tracker::new(Some(&ready));
            let mut offset = 0u64;
            for chunk in &chunks { tracker.feed(chunk.as_bytes(), offset); offset += chunk.len() as u64; }
            tracker.settle_ready();
            tracker.take_ready()
        };
        assert_eq!(whole, expected, "{}", case["name"]);
        let bytewise = {
            let mut tracker = replay::Tracker::new(Some(&ready));
            for (offset, byte) in (0u64..).zip(chunks.concat().bytes()) { tracker.feed(&[byte], offset); }
            tracker.settle_ready();
            tracker.take_ready()
        };
        assert_eq!(bytewise, expected, "{} (one byte at a time)", case["name"]);
    }
}

#[test]
fn an_address_that_ends_the_output_waits_for_the_byte_that_decides_it() {
    let mut tracker = replay::Tracker::new(Some(&Ready { port: 6061, path: "/".into(), printed: false }));
    tracker.feed(b"Local: http://localhost:6061", 0);
    assert!(tracker.ready_undecided() && tracker.take_ready().is_none());
    tracker.feed(b"/", 28);
    assert_eq!(tracker.take_ready().as_deref(), Some("http://localhost:6061/"));
    assert!(!tracker.ready_undecided());
}

#[test]
fn a_restarted_task_announces_its_address_again() {
    let ready = Ready { port: 7000, path: String::new(), printed: false };
    let mut tracker = replay::Tracker::new(Some(&ready));
    tracker.feed(b"http://localhost:7000\r\n", 0);
    assert!(tracker.take_ready().is_some());
    tracker.feed(b"http://localhost:7000\r\n", 23);
    assert!(tracker.take_ready().is_none(), "an address is announced once per run");
    tracker.await_ready(Some(&ready));
    tracker.feed(b"http://localhost:7000\r\n", 46);
    assert_eq!(tracker.take_ready().as_deref(), Some("http://localhost:7000"));
}

#[test]
fn child_titles_are_read_from_both_title_sequences() {
    let mut tracker = replay::Tracker::new(None);
    tracker.feed(b"\x1b]0;first title\x07text\x1b]2;second\x1b\\", 0);
    assert_eq!(tracker.title(), Some("second"));
    assert!(tracker.take_title_change());
    assert!(!tracker.take_title_change());
}
// #endregion 🔖️Ready

// #region 🔖️Replay
fn lines(count: usize) -> Vec<u8> {
    (0..count).flat_map(|index| format!("line {index:07} of the output of a very talkative task\r\n").into_bytes()).collect()
}

#[test]
fn a_replay_starts_at_a_line_and_ends_at_the_present() {
    let directory = temporary("replay");
    let mut log = replay::SessionLog::create(&directory, "task", None);
    let output = lines(120_000);
    log.append(&output);
    assert_eq!(log.end(), output.len() as u64);
    let (start, preamble, truncated) = log.replay();
    assert!(truncated && start > 0 && output.len() as u64 - start <= replay::RING_BYTES as u64 + replay::MARK_SPACING * 2);
    assert!(preamble.is_empty());
    assert!(start == 0 || output[start as usize - 1] == b'\n', "a replay never starts inside a line");
    let mut replayed = Vec::new();
    let mut at = start;
    while at < log.end() {
        let before = replayed.len();
        assert_eq!(log.read(at, 32 * 1024, &mut replayed), replay::Fetch::Data);
        at += (replayed.len() - before) as u64;
    }
    assert_eq!(replayed, output[start as usize..]);
    assert_eq!(log.read(log.end(), 1024, &mut replayed), replay::Fetch::Current);
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn output_beyond_the_memory_ring_is_served_from_the_log_files() {
    let directory = temporary("segments");
    let mut log = replay::SessionLog::create(&directory, "task", None);
    let output = lines(200_000);
    log.append(&output);
    assert!(output.len() > replay::RING_BYTES * 4 && output.len() as u64 > replay::SEGMENT_BYTES);
    let mut early = Vec::new();
    assert_eq!(log.read(0, 4096, &mut early), replay::Fetch::Data, "output older than the ring is still read from the files");
    assert_eq!(early, output[..4096]);
    log.flush();
    let mut restored = replay::SessionLog::restore(&directory, "task");
    assert_eq!(restored.end(), output.len() as u64, "an ended session's log survives its daemon");
    let mut tail = Vec::new();
    assert_eq!(restored.read(output.len() as u64 - 100, 4096, &mut tail), replay::Fetch::Data);
    assert_eq!(tail, output[output.len() - 100..]);
    let (start, _, truncated) = restored.replay();
    assert!(truncated && start > 0, "a restored log replays its recent tail");
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn the_oldest_log_segments_are_retired_and_reads_of_them_are_lost() {
    let directory = temporary("retire");
    let mut log = replay::SessionLog::create(&directory, "task", None);
    let output = lines(760_000);
    log.append(&output);
    log.flush();
    assert!(log.oldest() >= replay::SEGMENT_BYTES * 2, "the log keeps the current segment and the one before it");
    let mut gone = Vec::new();
    assert_eq!(log.read(0, 16, &mut gone), replay::Fetch::Lost);
    let (start, _, truncated) = log.replay();
    assert!(truncated && start >= log.oldest());
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn a_replay_reproduces_the_terminal_modes_of_the_whole_stream() {
    let directory = temporary("modes");
    let mut stream = Vec::new();
    stream.extend_from_slice(b"\x1b]2;build output\x07\x1b[?25l\x1b[?2004h\x1b[?1003h\x1b[38;2;10;200;30;1m");
    stream.extend(lines(60_000));
    stream.extend_from_slice(b"\x1b[48;5;17m\x1b[?1049h\x1b[?1049l\x1b[3;12r\x1b[?25h");
    stream.extend(lines(10));
    let mut log = replay::SessionLog::create(&directory, "task", None);
    log.append(&stream);
    let (start, preamble, _) = log.replay();
    assert!(start > 0);
    let mut replayed = replay::Tracker::new(None);
    replayed.feed(&preamble, 0);
    replayed.feed(&stream[start as usize..], preamble.len() as u64);
    let mut whole = replay::Tracker::new(None);
    whole.feed(&stream, 0);
    assert_eq!(replayed.preamble(), whole.preamble(), "colours, modes, region and title after a replay equal those of the whole stream");
    assert_eq!(replayed.title(), Some("build output"));
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn a_replay_restores_the_input_modes_the_child_is_in() {
    let directory = temporary("input-modes");
    let mut stream = b"\x1b[?1h\x1b[?2004h\x1b[?1000h\x1b[?1002h\x1b[?1006h\x1b[?1004h\x1b=".to_vec();
    stream.extend(lines(60_000));
    let mut log = replay::SessionLog::create(&directory, "task", None);
    log.append(&stream);
    let (start, preamble, _) = log.replay();
    assert!(start > 0);
    let text = String::from_utf8_lossy(&preamble).into_owned();
    for mode in ["\x1b[?1h", "\x1b[?2004h", "\x1b[?1000h", "\x1b[?1002h", "\x1b[?1006h", "\x1b[?1004h", "\x1b="] { assert!(text.contains(mode), "{mode:?} missing from {text:?}"); }
    let off = b"\x1b[?2004l\x1b[?1000l\x1b[?1l\x1b>";
    log.append(off);
    stream.extend_from_slice(off);
    let (start, preamble, _) = log.replay();
    let mut replayed = replay::Tracker::new(None);
    replayed.feed(&preamble, 0);
    replayed.feed(&stream[start as usize..], preamble.len() as u64);
    let text = String::from_utf8_lossy(&replayed.preamble()).into_owned();
    for mode in ["\x1b[?2004h", "\x1b[?1000h", "\x1b[?1h", "\x1b="] { assert!(!text.contains(mode), "{mode:?} was switched off: {text:?}"); }
    assert!(text.contains("\x1b[?1002h") && text.contains("\x1b[?1006h") && text.contains("\x1b[?1004h"), "{text:?}");
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn a_full_screen_program_is_replayed_from_the_moment_it_took_the_screen() {
    let directory = temporary("screen");
    let mut stream = lines(40);
    let entered = stream.len() as u64;
    stream.extend_from_slice(b"\x1b[?1049h\x1b[2J\x1b[H");
    for frame in 0..120_000 { stream.extend_from_slice(format!("\x1b[{};1Hframe {frame:06}\x1b[K", frame % 20 + 1).as_bytes()); }
    let mut log = replay::SessionLog::create(&directory, "task", None);
    log.append(&stream);
    assert!(stream.len() > replay::RING_BYTES);
    let (start, preamble, _) = log.replay();
    assert_eq!(start, entered, "the screen is rebuilt from its first frame, not from the middle of a redraw");
    assert!(preamble.is_empty(), "the modes before the program took the screen are the defaults");
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn a_second_run_of_a_session_continues_its_log_from_clean_terminal_modes() {
    let directory = temporary("rerun");
    let mut log = replay::SessionLog::create(&directory, "task", None);
    log.append(b"first run\x1b[?25l\x1b[?1049h\x1b[31m");
    let separator = log.separator();
    let text = String::from_utf8_lossy(&separator).into_owned();
    assert!(text.contains("\x1b[?1049l") && text.contains("\x1b[?25h") && text.ends_with("\r\n"), "the second run starts on a clean terminal: {text:?}");
    log.append(&separator);
    log.append(b"second run\r\n");
    log.flush();
    let mut restored = replay::SessionLog::restore(&directory, "task");
    let mut bytes = Vec::new();
    assert_eq!(restored.read(0, 1 << 20, &mut bytes), replay::Fetch::Data);
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.starts_with("first run") && text.ends_with("second run\r\n"));
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn log_files_of_forgotten_sessions_are_swept() {
    let directory = temporary("sweep");
    let mut kept = replay::SessionLog::create(&directory, "kept", None);
    let mut dropped = replay::SessionLog::create(&directory, "dropped", None);
    kept.append(b"kept\r\n");
    dropped.append(b"dropped\r\n");
    kept.flush();
    dropped.flush();
    replay::sweep_logs(&directory, &BTreeSet::from([replay::log_stem("kept")]));
    assert_eq!(replay::SessionLog::restore(&directory, "kept").end(), 6);
    assert_eq!(replay::SessionLog::restore(&directory, "dropped").end(), 0);
    let _ = std::fs::remove_dir_all(directory);
}
// #endregion 🔖️Replay

// #region 🔖️Journal
fn session(id: &str, status: SessionStatus, started_ms: u64) -> SessionInfo {
    SessionInfo { session_id: id.into(), command: ipc::SessionCommand { cmd: "x".into(), cols: 80, rows: 24, ..Default::default() }, status, started_ms, ..Default::default() }
}

#[test]
fn the_journal_projects_sessions_survives_a_torn_line_and_compacts() {
    let root = temporary("journal");
    {
        let (mut journal, restored) = journal::Journal::open(&root).unwrap();
        assert!(restored.is_empty());
        for index in 0..300u64 {
            journal.append(&ServerMsg::SessionChanged { session: Box::new(session("busy", if index % 2 == 0 { SessionStatus::Running } else { SessionStatus::Stopping }, index + 1)) }).unwrap();
        }
        journal.append(&ServerMsg::SessionChanged { session: Box::new(session("gone", SessionStatus::Exited, 5)) }).unwrap();
        journal.append(&ServerMsg::SessionRemoved { session_id: "gone".into() }).unwrap();
    }
    let mut torn = std::fs::OpenOptions::new().append(true).open(ipc::event_log_path(&root)).unwrap();
    std::io::Write::write_all(&mut torn, b"{\"type\":\"session_changed\",\"sess").unwrap();
    drop(torn);
    let (_, restored) = journal::Journal::open(&root).unwrap();
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].session_id, "busy");
    assert_eq!(restored[0].started_ms, 300);
    assert_eq!(std::fs::read_to_string(ipc::event_log_path(&root)).unwrap().lines().count(), 1, "the journal is compacted to its projection");
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn the_journal_retains_a_bounded_number_of_sessions_and_drops_the_longest_ended_first() {
    let root = temporary("retention");
    {
        let (mut journal, _) = journal::Journal::open(&root).unwrap();
        for index in 0..200u64 {
            let mut ended = session(&format!("s{index:03}"), SessionStatus::Exited, 1000 + (199 - index));
            ended.ended_ms = Some(5000 + index);
            journal.append(&ServerMsg::SessionChanged { session: Box::new(ended) }).unwrap();
        }
    }
    let (_, restored) = journal::Journal::open(&root).unwrap();
    assert_eq!(restored.len(), journal::RETAINED_SESSIONS);
    assert!(restored.iter().all(|session| session.ended_ms.unwrap() >= 5000 + 72), "the sessions that ended first are the ones that left: {:?}", restored.iter().map(|session| session.ended_ms).min());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn sessions_alive_when_no_daemon_runs_were_interrupted() {
    let root = temporary("offline");
    {
        let (mut journal, _) = journal::Journal::open(&root).unwrap();
        for (id, status) in [("run", SessionStatus::Running), ("stop", SessionStatus::Stopping), ("wait", SessionStatus::Pending), ("done", SessionStatus::Exited)] {
            let mut info = session(id, status, 1);
            info.pid = (status == SessionStatus::Running).then_some(7);
            journal.append(&ServerMsg::SessionChanged { session: Box::new(info) }).unwrap();
        }
    }
    let seen: HashMap<String, SessionStatus> = journal::offline(&root).into_iter().map(|session| (session.session_id, session.status)).collect();
    assert_eq!(seen["run"], SessionStatus::Interrupted);
    assert_eq!(seen["stop"], SessionStatus::Interrupted);
    assert_eq!(seen["wait"], SessionStatus::Interrupted);
    assert_eq!(seen["done"], SessionStatus::Exited);
    let _ = std::fs::remove_dir_all(root);
}
// #endregion 🔖️Journal

// #region 🔖️Launch
fn process(command_id: &str, ready: Option<Ready>) -> LaunchProcess {
    LaunchProcess { command_id: command_id.into(), cmd: "bun".into(), args: vec!["nx".into(), "run".into(), command_id.into()], cwd: PathBuf::from("."), env: vec![("A".into(), "1".into())], label: TaskLabel { verb: "dev".into(), owner: vec!["pkg".into()], subject: command_id.into(), qualifier: String::new(), parameters: vec![], members: 0 }, ready, long_running: true }
}

fn launch(command_id: &str, processes: Vec<LaunchProcess>, requires: Vec<Launch>, group: Option<&str>, stop: Stop) -> Launch {
    Launch { command_id: command_id.into(), label: processes[0].label.clone(), processes, requires, group: group.map(str::to_string), stop }
}

#[test]
fn a_launch_becomes_one_session_per_process_with_services_first() {
    let hub = launch("hub:dev", vec![process("hub:dev", Some(Ready { port: 6070, ..Default::default() }))], vec![], None, Stop::Independent);
    let compound = launch("compound:p/both", vec![process("a:dev", Some(Ready { port: 6061, path: "/admin".into(), printed: false })), process("b:dev", None)], vec![hub], Some("compound:p/both"), Stop::Together);
    let group = SpawnGroup::from_launch(&compound, &[("A".into(), "2".into()), ("EXTRA".into(), "x".into())]);
    assert_eq!(group.stop, ipc::GroupStop::Together);
    assert_eq!(group.requires.len(), 1);
    assert_eq!(group.requires[0].command.command_id, "hub:dev");
    assert_eq!(group.requires[0].command.group, None, "a service is not a member of the group that needs it");
    assert_eq!(group.members.iter().map(|member| member.command.command_id.as_str()).collect::<Vec<_>>(), ["a:dev", "b:dev"]);
    assert!(group.members.iter().all(|member| member.command.group.as_deref() == Some(group.group_id.as_str())));
    assert_eq!(group.members[0].command.ready, Some(Ready { port: 6061, path: "/admin".into(), printed: false }));
    assert_eq!(group.members[0].command.label.subject, "a:dev");
    assert_eq!(group.members[0].command.env, [("A".to_string(), "2".to_string()), ("EXTRA".to_string(), "x".to_string())], "extra environment of the launch wins over the launch's own");
    let ids: BTreeSet<&str> = group.members.iter().chain(&group.requires).map(|member| member.session_id.as_str()).collect();
    assert_eq!(ids.len(), 3, "every session of a group has an identifier of its own");
    assert!(ClientMsg::SpawnGroup { group_id: group.group_id.clone(), stop: group.stop, members: group.members.clone(), requires: group.requires.clone() }.validate().is_ok());
    assert_ne!(SpawnGroup::from_launch(&compound, &[]).group_id, group.group_id, "two starts of the same launch are two groups");
}

#[test]
fn a_service_needed_twice_is_started_once() {
    let hub = || launch("hub:dev", vec![process("hub:dev", None)], vec![], None, Stop::Independent);
    let one = launch("one:dev", vec![process("one:dev", None)], vec![hub()], None, Stop::Independent);
    let both = launch("both", vec![process("both", None)], vec![one, hub()], None, Stop::Independent);
    let group = SpawnGroup::from_launch(&both, &[]);
    assert_eq!(group.requires.iter().map(|member| member.command.command_id.as_str()).collect::<Vec<_>>(), ["hub:dev", "one:dev"]);
}
// #endregion 🔖️Launch

// #region 🔖️Selection
#[test]
fn a_task_is_selected_by_what_a_developer_types() {
    let mut sessions = vec![session("task-1", SessionStatus::Exited, 10), session("task-2", SessionStatus::Running, 30), session("other-3", SessionStatus::Exited, 20)];
    sessions[0].command.command_id = "pkg:build".into();
    sessions[1].command.command_id = "pkg:build".into();
    sessions[2].command.command_id = "pkg:dev".into();
    let chosen = |needle: &str| control::select(&sessions, needle).map(|session| session.session_id.clone());
    assert_eq!(chosen("task-1").unwrap(), "task-1");
    assert_eq!(chosen("3").unwrap(), "other-3");
    assert_eq!(chosen("pkg:build").unwrap(), "task-2", "the live run of a command wins");
    assert_eq!(chosen("pkg:d").unwrap(), "other-3");
    assert_eq!(chosen("other").unwrap(), "other-3");
    assert!(matches!(chosen("pkg"), Err(control::SelectError::Ambiguous(_, found)) if found == ["pkg:build", "pkg:dev"]));
    assert!(matches!(chosen("nothing"), Err(control::SelectError::None(_))));
}
// #endregion 🔖️Selection

// #region 🔖️Instances
#[test]
fn a_named_instance_has_its_own_endpoint_directory_and_key() {
    let root = temporary("instances");
    assert_eq!(ipc::instance_named(None), None);
    assert_eq!(ipc::instance_named(Some("")), None);
    assert_eq!(ipc::instance_named(Some("  ")), None);
    assert_eq!(ipc::instance_named(Some("..")), None);
    assert_eq!(ipc::instance_named(Some("smoke-1.a_b")).as_deref(), Some("smoke-1.a_b"));
    assert_eq!(ipc::instance_named(Some("a/b\\c d")).as_deref(), Some("a_b_c_d"), "a name cannot leave its directory");
    assert_eq!(ipc::instance_named(Some(&"x".repeat(80))).map(|name| name.len()), Some(32));
    let default = ipc::workspace_key_of(&root, None);
    let smoke = ipc::workspace_key_of(&root, Some("smoke"));
    assert_ne!(default, smoke);
    assert_ne!(smoke, ipc::workspace_key_of(&root, Some("other")));
    assert_eq!(smoke, ipc::workspace_key_of(&root, Some("smoke")), "an instance key is stable");
    let (plain, named) = (ipc::daemon_dir_of(&root, None), ipc::daemon_dir_of(&root, Some("smoke")));
    assert_eq!(plain, ipc::dashboard_cache_dir(&root));
    assert_eq!(named, ipc::dashboard_cache_dir(&root).join("instances").join("smoke"));
    let _ = std::fs::remove_dir_all(root);
}
// #endregion 🔖️Instances

// #region 🔖️Environment
#[test]
fn the_queued_byte_count_of_a_connection_is_the_sum_of_its_frames_through_partial_writes() {
    let mut queue = client::Outbound::default();
    let sizes = [1usize, 7, 4096, 65_536, 3, 0, 31_337, 5, 262_144, 11];
    let mut expected = 0;
    for (index, size) in sizes.iter().enumerate() {
        queue.push(vec![index as u8; *size]);
        expected += size;
        assert_eq!(queue.queued_bytes(), expected);
        if index % 3 == 2 {
            let frame = queue.pop().unwrap();
            let (mut sent, mut chunk) = (0, 1);
            while sent < frame.len() { sent += chunk.min(frame.len() - sent); chunk = chunk * 3 + 1; }
            expected -= frame.len();
            assert_eq!(queue.queued_bytes(), expected, "a frame written in partial pieces leaves the count exact");
        }
    }
    while let Some(frame) = queue.pop() { expected -= frame.len(); assert_eq!(queue.queued_bytes(), expected); }
    assert_eq!((queue.queued_bytes(), expected), (0, 0));
    assert!(queue.pop().is_none());
}

#[test]
fn a_queue_refuses_frames_beyond_its_limit_without_losing_count() {
    let mut queue = client::Outbound::default();
    let big = vec![0u8; 8 * 1024 * 1024];
    assert!(queue.fits(&big));
    queue.push(big.clone());
    queue.push(big.clone());
    assert!(!queue.fits(&big) && queue.queued_bytes() == 16 * 1024 * 1024);
    queue.pop();
    assert!(queue.fits(&big));
}
#[test]
fn a_task_starts_in_the_environment_of_the_client_with_a_terminal() {
    let client = vec![("PATH".to_string(), "/client/bin".to_string()), ("TERM".to_string(), "dumb".to_string()), ("NX_INVOCATION_ROOT_PID".to_string(), "1".to_string()), ("TOKEN".to_string(), "t".to_string())];
    let env = supervisor::child_environment(&client, &[("TOKEN".into(), "declared".into())]);
    let value = |name: &str| env.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str());
    assert_eq!(value("PATH"), Some("/client/bin"));
    assert_eq!(value("TOKEN"), Some("declared"), "the command's declared environment wins over the client's");
    assert_eq!(value("TERM"), Some("xterm-256color"));
    assert_eq!(value("COLORTERM"), Some("truecolor"));
    assert_eq!(value("NX_INVOCATION_ROOT_PID"), None);
    let stated = supervisor::child_environment(&client, &[("TERM".into(), "vt100".into())]);
    assert_eq!(stated.iter().find(|(key, _)| key == "TERM").map(|(_, value)| value.as_str()), Some("vt100"));
}

#[test]
fn the_hello_of_a_connection_carries_only_variables_a_task_can_receive() {
    let environment = client::process_environment();
    assert!(!environment.is_empty());
    assert!(environment.iter().all(|(name, value)| !name.is_empty() && !name.contains(['=', '\0']) && !value.contains('\0')));
    assert!(ClientMsg::Hello { client_id: "c".into(), protocol: ipc::PROTOCOL, build_id: ipc::build_id().into(), env: environment }.validate().is_ok());
}

#[test]
fn skew_between_client_and_daemon_is_described() {
    let compatible = client::Skew { client_protocol: 2, daemon_protocol: 2, client_build: "a".into(), daemon_build: "b".into() };
    assert!(!compatible.incompatible() && compatible.describe().contains("build b") && compatible.describe().contains("build a"));
    let incompatible = client::Skew { client_protocol: 3, daemon_protocol: 2, client_build: "a".into(), daemon_build: "a".into() };
    assert!(incompatible.incompatible() && incompatible.describe().contains("protocol 2") && incompatible.describe().contains('3'));
}
// #endregion 🔖️Environment

#[test]
fn every_scenario_of_the_daemon_features_is_proved_by_a_test() {
    let sources = [include_str!("🦀️.rs"), include_str!("../🧊️integration/🦀️.rs")];
    crate::tests::assert_proved(include_str!("../../../🧪️tests/✉️ipc/🥒️.feature"), &sources, &[
        ("Every valid protocol vector survives the codec unchanged", &["every_valid_protocol_vector_survives_the_rust_codec_unchanged"]),
        ("Every invalid protocol vector is refused", &["every_invalid_protocol_vector_is_refused_by_the_rust_codec"]),
        ("The vectors cover the whole schema", &["the_vectors_cover_every_message_status_and_error_code_of_the_schema", "the_error_codes_of_the_rust_wire_are_the_codes_of_the_schema"]),
        ("Frames carry the same bytes in every implementation", &["frames_match_the_shared_bytes", "control_messages_travel_in_frames_that_buffer_across_arbitrary_cuts"]),
        ("A missing greeting, a foreign protocol and an unreadable message are refused loudly", &["a_start_without_hello_and_a_foreign_protocol_are_refused_loudly"]),
        ("A difference between client and daemon is described", &["skew_between_client_and_daemon_is_described"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/🚚️transport/🥒️.feature"), &sources, &[
        ("The endpoint name derives from the workspace path", &["endpoint_names_are_stable_across_runs_and_toolchains"]),
        ("A second daemon cannot take over a running workspace", &["a_second_daemon_cannot_take_over_a_running_workspace"]),
        ("A connection wakes its owner", &["a_connection_calls_its_notifier_when_a_message_arrives_and_when_its_sends_drain"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/📜️journal/🥒️.feature"), &sources, &[
        ("The journal projects sessions, survives a torn line and compacts", &["the_journal_projects_sessions_survives_a_torn_line_and_compacts"]),
        ("The journal retains a bounded number of sessions", &["the_journal_retains_a_bounded_number_of_sessions_and_drops_the_longest_ended_first"]),
        ("Sessions that were alive when no daemon ran come back interrupted", &["sessions_alive_when_no_daemon_runs_were_interrupted", "sessions_that_were_alive_when_the_daemon_vanished_come_back_interrupted"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/🧠️supervisor/🥒️.feature"), &sources, &[
        ("A task runs to its exit code and later views and a restarted daemon replay its output", &["a_task_runs_to_its_exit_code_and_later_views_and_a_restarted_daemon_replay_its_output"]),
        ("A restart runs the task again into the same log", &["a_restart_runs_the_task_again_into_the_same_log"]),
        ("A child title and a full-screen program reach every view", &["a_child_title_and_a_full_screen_program_reach_every_view", "child_titles_are_read_from_both_title_sequences"]),
        ("A task starts in the environment of the view that asked for it", &["a_task_starts_in_the_environment_of_the_view_that_asked_for_it", "a_task_starts_in_the_environment_of_the_client_with_a_terminal", "the_hello_of_a_connection_carries_only_variables_a_task_can_receive"]),
        ("Stopping escalates and killing reports the signal death", &["stopping_escalates_and_killing_reports_the_signal_death"]),
        ("Terminal input is queued and delivered in full", &["terminal_input_is_queued_and_delivered_in_full"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/🕹️control/🥒️.feature"), &sources, &[
        ("A launch starts services first and returns its ready address", &["a_launch_starts_services_first_members_in_order_and_stops_together"]),
        ("A member that ends before it is ready fails the run", &["a_member_that_ends_before_it_is_ready_fails_the_launch_and_the_members_after_it"]),
        ("A cancelled run starts nothing more", &["a_cancelled_launch_never_starts_the_members_still_waiting"]),
        ("The log files are read without a daemon", &["a_task_runs_to_its_exit_code_and_later_views_and_a_restarted_daemon_replay_its_output"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/📼️replay/🥒️.feature"), &sources, &[
        ("Replay starts at a line and ends at the present", &["a_replay_starts_at_a_line_and_ends_at_the_present"]),
        ("Replay restores the terminal modes", &["a_replay_reproduces_the_terminal_modes_of_the_whole_stream", "a_replay_restores_the_input_modes_the_child_is_in"]),
        ("Rebuild a full-screen program from its first frame", &["a_full_screen_program_is_replayed_from_the_moment_it_took_the_screen", "a_child_title_and_a_full_screen_program_reach_every_view"]),
        ("Keep scrollback in log files", &["output_beyond_the_memory_ring_is_served_from_the_log_files", "the_oldest_log_segments_are_retired_and_reads_of_them_are_lost", "log_files_of_forgotten_sessions_are_swept"]),
        ("Never disconnect a stalled view", &["a_view_that_does_not_read_is_neither_disconnected_nor_allowed_to_slow_the_others"]),
        ("List many sessions to a fresh view", &["a_hundred_retained_sessions_are_listed_to_a_fresh_view_beside_a_stalled_one"]),
        ("Restart appends to the log from clean terminal modes", &["a_second_run_of_a_session_continues_its_log_from_clean_terminal_modes"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/🟢️ready/🥒️.feature"), &sources, &[
        ("Decide every case of the shared table", &["the_ready_table_decides_every_case"]),
        ("Decide the same way however the output is cut", &["the_ready_table_decides_every_case"]),
        ("Wait for the byte that completes the port", &["an_address_that_ends_the_output_waits_for_the_byte_that_decides_it"]),
        ("Announce an address once per run", &["a_restarted_task_announces_its_address_again"]),
        ("Report the ready address on the session", &["a_launch_starts_services_first_members_in_order_and_stops_together"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/🧩️groups/🥒️.feature"), &sources, &[
        ("Start the services a command requires first", &["a_launch_becomes_one_session_per_process_with_services_first", "a_launch_starts_services_first_members_in_order_and_stops_together"]),
        ("Reuse a running service", &["a_service_needed_twice_is_started_once"]),
        ("Start the members of a compound in order", &["a_launch_starts_services_first_members_in_order_and_stops_together", "the_waiting_line_is_written_only_for_members_that_actually_wait"]),
        ("Stop the members of a compound together", &["a_launch_starts_services_first_members_in_order_and_stops_together"]),
        ("Fail the launch when a member cannot become ready", &["a_member_that_ends_before_it_is_ready_fails_the_launch_and_the_members_after_it"]),
        ("Cancel a launch that is still starting", &["a_cancelled_launch_never_starts_the_members_still_waiting"]),
    ]);
}
