use super::*;
use std::collections::HashMap;

#[test]
fn unknown_subcommand_returns_usage_without_side_effects() {
    let parsed = ParsedArgs { verb: "daemon".into(), segments: vec!["unknown".into()], flags: HashMap::new() };
    assert_eq!(run(Path::new("."), &parsed), 1);
}

/// 🪟 A control frame written by a client reaches the server half of a real named pipe unchanged,
/// and the answer written back reaches the client — the transport `serve` is built on.
#[cfg(windows)]
#[test]
fn a_named_pipe_round_trips_one_frame_each_way() {
    use ipc::{ClientMsg, ServerMsg};
    let name = format!(r"\\.\pipe\semio-dashboard-test-{}", std::process::id());
    let listening = name.clone();
    let server = std::thread::spawn(move || {
        let mut stream = ipc::accept(&listening).expect("accept");
        let (kind, payload) = ipc::read_frame(&mut stream).expect("server frame");
        assert_eq!(kind, ipc::KIND_CONTROL);
        let message: ClientMsg = ipc::decode_control(&payload).expect("client message");
        assert_eq!(message, ClientMsg::Attach { client_id: "frame".into() });
        ipc::write_control(&mut stream, &ServerMsg::Attached { daemon_pid: 7 }).expect("server answer");
    });
    let mut client = None;
    for _ in 0..100 {
        match std::fs::OpenOptions::new().read(true).write(true).open(&name) {
            Ok(handle) => {
                client = Some(handle);
                break;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(20)),
        }
    }
    let mut client = client.expect("client open");
    ipc::write_control(&mut client, &ClientMsg::Attach { client_id: "frame".into() }).expect("client frame");
    let (kind, payload) = ipc::read_frame(&mut client).expect("client answer");
    assert_eq!(kind, ipc::KIND_CONTROL);
    assert_eq!(ipc::decode_control::<ServerMsg>(&payload).expect("server message"), ServerMsg::Attached { daemon_pid: 7 });
    server.join().expect("server thread");
}

/// 🪟 The whole windows serve loop: a client that opens the daemon's named pipe is greeted, and a
/// ping it sends over that pipe is answered — the transport `semio daemon attach` rides on.
#[cfg(windows)]
#[test]
fn the_windows_serve_loop_greets_and_answers_a_client() {
    use ipc::{ClientMsg, ServerMsg};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    let root = std::env::temp_dir().join(format!("semio-daemon-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("root");
    let running = Arc::new(AtomicBool::new(true));
    let serving = Arc::clone(&running);
    let served_root = root.clone();
    let daemon = std::thread::spawn(move || supervisor::serve(&served_root, serving));
    let mut client = ipc::connect(&root).expect("connect");
    let (kind, payload) = ipc::read_frame(&mut client).expect("greeting");
    assert_eq!(kind, ipc::KIND_CONTROL);
    assert!(matches!(ipc::decode_control::<ServerMsg>(&payload).expect("greeting message"), ServerMsg::Attached { .. }));
    ipc::write_control(&mut client, &ClientMsg::Ping).expect("ping");
    let (_, payload) = ipc::read_frame(&mut client).expect("pong");
    assert_eq!(ipc::decode_control::<ServerMsg>(&payload).expect("pong message"), ServerMsg::Pong);
    running.store(false, Ordering::SeqCst);
    daemon.join().expect("daemon thread").expect("serve");
    let _ = std::fs::remove_dir_all(&root);
}
