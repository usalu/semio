//! 🚀️ Entrypoint: the stdio transport loop and the public run entry points.
//!
//! Holds the `🦀️Entrypoint` region of the crate root; the root re-exports its items.

use super::*;

//#region 🦀️Entrypoint

/// 🪪️ The environment variable that selects the profile.
pub const PROFILE_ENVIRONMENT: &str = "SEMIO_REPO_MCP_CLIENT";

/// 🧺️ The queue every handler thread of one served peer pulls its next payload from: the pending
/// payloads, the condition the readers wait on, and the flag that closes the queue.
type JobQueue = (Mutex<Vec<Vec<u8>>>, Condvar, AtomicBool);

/// 🚚️ Serves one peer over a line-delimited byte stream until the peer drops.
pub fn serve(server: &Arc<Server>, peer: &str, reader: impl Read + Send, writer: impl Write + Send + 'static) -> Result<(), Error> {
    let output = Arc::new(Mutex::new(writer));
    let sink_output = Arc::clone(&output);
    let sink: Sink = Arc::new(move |payload: &[u8]| {
        let mut guard = sink_output.lock().unwrap_or_else(|poison| poison.into_inner());
        guard.write_all(payload).and_then(|()| guard.write_all(b"\n")).and_then(|()| guard.flush()).map_err(|_| Error::PeerDropped)
    });
    let session = server.connect(peer, Some(Arc::clone(&sink)))?;
    let limits = server.config.limits;
    let jobs: Arc<JobQueue> = Arc::new((Mutex::new(Vec::new()), Condvar::new(), AtomicBool::new(false)));
    let mut workers = Vec::new();
    for _ in 0..limits.max_handlers {
        let jobs = Arc::clone(&jobs);
        let session = Arc::clone(&session);
        let server = Arc::clone(server);
        let sink = Arc::clone(&sink);
        workers.push(std::thread::spawn(move || loop {
            let payload = {
                let (queue, ready, closed) = &*jobs;
                let mut guard = queue.lock().unwrap_or_else(|poison| poison.into_inner());
                while guard.is_empty() {
                    if closed.load(Ordering::SeqCst) {
                        return;
                    }
                    guard = ready.wait(guard).unwrap_or_else(|poison| poison.into_inner());
                }
                guard.remove(0)
            };
            server.handler_queued.fetch_sub(1, Ordering::SeqCst);
            server.handler_active.fetch_add(1, Ordering::SeqCst);
            if let Ok(Some(response)) = session.dispatch(&payload) {
                let _ = sink(response.as_bytes());
            }
            server.handler_active.fetch_sub(1, Ordering::SeqCst);
        }));
    }
    let lines = BufReader::new(reader).split(b'\n');
    let mut outcome = Ok(());
    for line in lines {
        let Ok(mut payload) = line else {
            outcome = Err(Error::PeerDropped);
            break;
        };
        if payload.last() == Some(&b'\r') {
            payload.pop();
        }
        if payload.is_empty() {
            continue;
        }
        if payload.len() > limits.max_payload_bytes {
            outcome = Err(Error::PayloadTooLarge);
            break;
        }
        let decoded = session.decode_request(&payload);
        if matches!(&decoded, Ok(request) if request.id.is_none()) {
            let _ = session.dispatch(&payload);
            continue;
        }
        if server.handler_queued.load(Ordering::SeqCst) >= limits.max_handlers as i64 {
            let response = session.reject(&payload, RpcError::new(CODE_SERVER_BUSY, "handler queue full"))?;
            sink(response.as_bytes())?;
            continue;
        }
        if matches!(&decoded, Ok(request) if request.method == "initialize") {
            session.expect_handshake();
        }
        server.handler_queued.fetch_add(1, Ordering::SeqCst);
        let (queue, ready, _) = &*jobs;
        queue.lock().unwrap_or_else(|poison| poison.into_inner()).push(payload);
        ready.notify_one();
    }
    let (_, ready, closed) = &*jobs;
    closed.store(true, Ordering::SeqCst);
    ready.notify_all();
    for worker in workers {
        let _ = worker.join();
    }
    session.close()?;
    outcome
}

/// ▶️ Serves the repository over stdio until stdin closes.
pub fn serve_stdio(repository: impl Repository + 'static, profile: Profile) -> Result<(), Error> {
    let server = Arc::new(repository_server(repository, profile, Limits::default())?);
    match serve(&server, "stdio", std::io::stdin(), std::io::stdout()) {
        Err(Error::PeerDropped) => Ok(()),
        other => other,
    }
}

/// 🚀️ The process entry point: no arguments are accepted, the profile comes from the environment.
pub fn run_with(argv: &[String], repository: impl Repository + 'static) -> i32 {
    if !argv.is_empty() {
        eprintln!("repo MCP accepts no command arguments");
        return 1;
    }
    let profile = match Profile::parse(&std::env::var(PROFILE_ENVIRONMENT).unwrap_or_default()) {
        Ok(profile) => profile,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    match serve_stdio(repository, profile) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

//#endregion 🦀️Entrypoint
