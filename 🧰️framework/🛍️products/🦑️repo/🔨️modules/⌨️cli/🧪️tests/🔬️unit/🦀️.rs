
use crate::args::parse;
use crate::catalog::{PlaygroundEntry, Ports, playgrounds_json_text};
use crate::env_contract::{DevOptions, build_dev_env, resolve_port};
use crate::options::{Lock, parse_lock};

#[test]
fn args_split_verb_segments_and_flags() {
    let argv: Vec<String> = ["dev", "puzzle", "2d", "--renderer", "wgpu-wasm", "--skip-plugin-build"].iter().map(|s| s.to_string()).collect();
    let parsed = parse(&argv);
    assert_eq!(parsed.verb, "dev");
    assert_eq!(parsed.segments, vec!["puzzle", "2d"]);
    assert_eq!(parsed.flag("renderer"), Some("wgpu-wasm"));
    assert!(parsed.has_flag("skip-plugin-build"));
    assert_eq!(parsed.flag("skip-plugin-build"), None);
}

#[test]
fn env_contract_sets_locks_only_for_individual() {
    let row = PlaygroundEntry { variant: "puzzle2d".into(), plugin_id: "puzzle2d".into(), ports: Ports { react: 6012, wgpu: 6112 }, ..Default::default() };
    let opts = DevOptions {
        renderer: "react".into(),
        example: Lock::Individual("concrete-forest".into()),
        language: Lock::All,
        terminology: Lock::Individual("reuse".into()),
        theme: Lock::All,
        appearance: Lock::Individual("dark".into()),
        ..Default::default()
    };
    let env = build_dev_env("puzzle2d", Some(&row), &opts);
    let get = |k: &str| env.iter().find(|(key, _)| key == k).map(|(_, v)| v.clone());
    assert_eq!(get("S_OS_PORT"), Some("6012".to_string()));
    assert_eq!(get("PLAYGROUND_LOCKED_EXAMPLE_ID"), Some("concrete-forest".to_string()));
    assert_eq!(get("SEMIO_LOCKED_TERMINOLOGY"), Some("reuse".to_string()));
    assert_eq!(get("SEMIO_LOCKED_APPEARANCE"), Some("dark".to_string()));
    assert_eq!(get("SEMIO_LOCKED_LOCALE"), None);
    assert_eq!(get("SEMIO_LOCKED_THEME"), None);
}

#[test]
fn resolve_port_prefers_explicit_then_catalog_then_fallback() {
    let row = PlaygroundEntry { ports: Ports { react: 6012, wgpu: 6112 }, ..Default::default() };
    assert_eq!(resolve_port(Some(&row), "react", Some(9999)), 9999);
    assert_eq!(resolve_port(Some(&row), "wgpu", None), 6112);
    assert_eq!(resolve_port(None, "react", None), 6066);
}

#[test]
fn parse_lock_all_is_case_insensitive() {
    assert_eq!(parse_lock("All"), Lock::All);
    assert_eq!(parse_lock("dark"), Lock::Individual("dark".to_string()));
}

//#region 🔖️CatalogConsumer
/// 🧪️ Unique scratch dir under the OS temp root, cleaned up by the caller when done.
fn temp_root(name: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!("semio-repo-cli-test-{name}-{nanos}"));
    std::fs::create_dir_all(&dir).expect("create temp root");
    dir
}

fn generated_dir_under(root: &std::path::Path) -> std::path::PathBuf {
    root.join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated")
}

#[test]
fn playgrounds_json_text_falls_back_to_empty_array_when_missing() {
    let root = temp_root("json-text-missing");
    assert_eq!(playgrounds_json_text(&root), "[]\n");
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn playgrounds_json_text_passes_generated_content_through_verbatim() {
    let root = temp_root("json-text-present");
    let out = generated_dir_under(&root);
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("🔣️playgrounds.json"), "[{\"variant\":\"x\"}]\n").unwrap();
    assert_eq!(playgrounds_json_text(&root), "[{\"variant\":\"x\"}]\n");
    std::fs::remove_dir_all(&root).ok();
}

//#endregion 🔖️CatalogConsumer

//#region 🔖️Ipc
#[test]
fn ipc_frame_roundtrip_and_output_codec() {
    use crate::ipc::{self, ClientMsg, ServerMsg};
    let mut buf = Vec::new();
    let msg = ClientMsg::Ping;
    ipc::write_control(&mut buf, &msg).unwrap();
    let mut cursor = std::io::Cursor::new(buf);
    let (kind, payload) = ipc::read_frame(&mut cursor).unwrap();
    assert_eq!(kind, ipc::KIND_CONTROL);
    let decoded: ClientMsg = ipc::decode_control(&payload).unwrap();
    assert_eq!(decoded, ClientMsg::Ping);

    let out = ipc::encode_output("s1", b"hi");
    let (id, data) = ipc::decode_output(&out).unwrap();
    assert_eq!(id, "s1");
    assert_eq!(data, b"hi");

    let mut buf = Vec::new();
    ipc::write_control(&mut buf, &ServerMsg::Pong).unwrap();
    assert!(!buf.is_empty());
}

#[test]
fn ipc_nonblocking_decoder_preserves_fragmented_and_concatenated_frames() {
    use crate::ipc::{self, ClientMsg};
    let mut encoded = Vec::new();
    ipc::write_control(&mut encoded, &ClientMsg::Ping).unwrap();
    ipc::write_control(&mut encoded, &ClientMsg::Detach).unwrap();
    let split = 3usize;
    let mut buffered = encoded[..split].to_vec();
    assert_eq!(ipc::try_decode_frame(&mut buffered).unwrap(), None);
    buffered.extend_from_slice(&encoded[split..]);
    let (_, first) = ipc::try_decode_frame(&mut buffered).unwrap().expect("first frame");
    let (_, second) = ipc::try_decode_frame(&mut buffered).unwrap().expect("second frame");
    assert_eq!(ipc::decode_control::<ClientMsg>(&first).unwrap(), ClientMsg::Ping);
    assert_eq!(ipc::decode_control::<ClientMsg>(&second).unwrap(), ClientMsg::Detach);
    assert!(buffered.is_empty());
}

#[test]
fn ipc_nonblocking_decoder_rejects_oversized_prefix_without_allocating() {
    use crate::ipc;
    let mut buffered = ((ipc::MAX_FRAME_BYTES as u32) + 1).to_le_bytes().to_vec();
    let error = ipc::try_decode_frame(&mut buffered).expect_err("oversized frame must fail");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(buffered.len(), 4);
}

#[test]
fn daemon_supervisor_ping_appends_event_log() {
    use crate::daemon::{self, Supervisor};
    use crate::ipc::{self, ClientMsg, ServerMsg};
    let root = temp_root("daemon-sup");
    let mut sup = Supervisor::new(&root).unwrap();
    let (a, mut b) = duplex_pair();
    // attach uses write on the client-facing end we keep in supervisor
    sup.attach_client(a).unwrap();
    // read Attached
    let (kind, payload) = ipc::read_frame(&mut b).unwrap();
    assert_eq!(kind, ipc::KIND_CONTROL);
    let msg: ServerMsg = ipc::decode_control(&payload).unwrap();
    assert!(matches!(msg, ServerMsg::Attached { .. }));
    daemon::handle_one_for_test(&mut sup, ClientMsg::Ping).unwrap();
    let (kind, payload) = ipc::read_frame(&mut b).unwrap();
    assert_eq!(kind, ipc::KIND_CONTROL);
    let msg: ServerMsg = ipc::decode_control(&payload).unwrap();
    assert_eq!(msg, ServerMsg::Pong);
    let log = std::fs::read_to_string(ipc::event_log_path(&root)).unwrap();
    assert!(log.contains("pong") || log.contains("Pong") || log.contains("\"type\":\"pong\""));
    std::fs::remove_dir_all(&root).ok();
}

#[cfg(unix)]
#[test]
fn daemon_nonblocking_connection_cursor_serves_ping_end_to_end() {
    use crate::daemon;
    use crate::ipc::{self, ClientMsg, ServerMsg};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::path::PathBuf::from("/tmp").join(format!("sd{}{}", std::process::id(), nanos % 1_000_000));
    std::fs::create_dir_all(&root).unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let server_running = running.clone();
    let server_root = root.clone();
    let server = std::thread::spawn(move || daemon::serve(&server_root, server_running));
    let mut client = (0..100)
        .find_map(|_| match ipc::connect(&root) {
            Ok(stream) => Some(stream),
            Err(_) => {
                std::thread::sleep(Duration::from_millis(5));
                None
            }
        })
        .expect("daemon socket must become reachable");
    client.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    let (_, attached) = ipc::read_frame(&mut client).expect("attached frame");
    assert!(matches!(ipc::decode_control::<ServerMsg>(&attached).unwrap(), ServerMsg::Attached { .. }));
    ipc::write_control(&mut client, &ClientMsg::Ping).unwrap();
    let (_, pong) = ipc::read_frame(&mut client).expect("pong frame");
    assert_eq!(ipc::decode_control::<ServerMsg>(&pong).unwrap(), ServerMsg::Pong);
    running.store(false, Ordering::SeqCst);
    server.join().expect("daemon thread must not panic").expect("daemon must stop cleanly");
    std::fs::remove_dir_all(&root).ok();
}

/// 🧵 Tiny in-memory bidirectional pipe for supervisor tests.
fn duplex_pair() -> (DuplexEnd, DuplexEnd) {
    use std::sync::mpsc::channel;
    let (t1, r1) = channel::<Vec<u8>>();
    let (t2, r2) = channel::<Vec<u8>>();
    (DuplexEnd { tx: t1, rx: r2, buf: Vec::new() }, DuplexEnd { tx: t2, rx: r1, buf: Vec::new() })
}

struct DuplexEnd {
    tx: std::sync::mpsc::Sender<Vec<u8>>,
    rx: std::sync::mpsc::Receiver<Vec<u8>>,
    buf: Vec<u8>,
}

impl std::io::Read for DuplexEnd {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        while self.buf.is_empty() {
            match self.rx.recv_timeout(std::time::Duration::from_secs(2)) {
                Ok(chunk) => self.buf.extend_from_slice(&chunk),
                Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "duplex closed")),
            }
        }
        let n = out.len().min(self.buf.len());
        out[..n].copy_from_slice(&self.buf[..n]);
        self.buf.drain(..n);
        Ok(n)
    }
}

impl std::io::Write for DuplexEnd {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.tx.send(buf.to_vec()).map_err(|_| std::io::Error::new(std::io::ErrorKind::BrokenPipe, "duplex"))?;
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
//#endregion 🔖️Ipc
