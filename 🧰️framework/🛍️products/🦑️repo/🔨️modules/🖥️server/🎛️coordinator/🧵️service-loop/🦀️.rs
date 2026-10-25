//! 🧵️ Service loop of the coordinator: the service, its start, the worker loop, the connection serving and the request reading. See [`super`].

use super::*;

//#region 🌐️Http
/// 🛎️ One running coordinator: its listener, its worker pool and the state they share.
pub struct Service {
    pub address: String,
    pub(super) server: Arc<Server>,
    running: Arc<AtomicBool>,
    workers: Vec<std::thread::JoinHandle<()>>,
    queue: Arc<(Mutex<Vec<TcpStream>>, Condvar)>,
}

impl Service {
    /// 📊️ The state the routes read and write.
    pub fn server(&self) -> &Arc<Server> {
        &self.server
    }

    /// 🛑️ Stops accepting, drains the workers and releases the store.
    pub fn stop(mut self) {
        self.running.store(false, Ordering::SeqCst);
        let _ = TcpStream::connect(&self.address);
        {
            let (queue, condition) = &*self.queue;
            let _unused = queue.lock().expect("queue");
            condition.notify_all();
        }
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
        self.server.repository.close();
    }
}

/// 🚀️ Opens the store, binds the address and serves the coordinator route table.
pub fn start(config: Config) -> Result<Service, StoreError> {
    let repository = Repository::open(&config.database_path)?;
    let listener = TcpListener::bind(&config.address).map_err(|failure| StoreError::Unavailable(format!("bind {}: {failure}", config.address)))?;
    let address = listener.local_addr().map_err(|failure| StoreError::Unavailable(failure.to_string()))?.to_string();
    let server = Arc::new(Server::new(config, repository));
    let running = Arc::new(AtomicBool::new(true));
    let queue: Arc<(Mutex<Vec<TcpStream>>, Condvar)> = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let listener = Arc::new(listener);
    let mut workers = Vec::new();
    for _ in 0..4 {
        let server = Arc::clone(&server);
        let running = Arc::clone(&running);
        let queue = Arc::clone(&queue);
        workers.push(std::thread::spawn(move || worker_loop(&server, &running, &queue)));
    }
    let acceptor = {
        let listener = Arc::clone(&listener);
        let running = Arc::clone(&running);
        let queue = Arc::clone(&queue);
        std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let Ok((stream, _)) = listener.accept() else { break };
                if !running.load(Ordering::SeqCst) {
                    let _ = stream.shutdown(Shutdown::Both);
                    break;
                }
                let (pending, condition) = &*queue;
                pending.lock().expect("queue").push(stream);
                condition.notify_one();
            }
        })
    };
    workers.push(acceptor);
    Ok(Service { address, server, running, workers, queue })
}

fn worker_loop(server: &Arc<Server>, running: &Arc<AtomicBool>, queue: &Arc<(Mutex<Vec<TcpStream>>, Condvar)>) {
    let (pending, condition) = &**queue;
    loop {
        let stream = {
            let mut guard = pending.lock().expect("queue");
            while guard.is_empty() {
                if !running.load(Ordering::SeqCst) {
                    return;
                }
                guard = condition.wait_timeout(guard, Duration::from_millis(50)).expect("queue").0;
            }
            guard.remove(0)
        };
        serve_connection(server, stream);
    }
}

fn serve_connection(server: &Arc<Server>, mut stream: TcpStream) {
    let response = match read_request(&mut stream, server.config.request_body_limit) {
        Err(message) => Response::error(400, &message),
        Ok(request) => server.handle(&request),
    };
    let mut head = format!("HTTP/1.1 {} {}\r\n", response.status, reason(response.status));
    if let Some(content_type) = &response.content_type {
        head.push_str(&format!("Content-Type: {content_type}\r\n"));
    }
    head.push_str(&format!("Content-Length: {}\r\nConnection: close\r\n\r\n", response.body.len()));
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&response.body);
    let _ = stream.flush();
    let _ = stream.shutdown(Shutdown::Both);
}

fn read_request(stream: &mut TcpStream, limit: u64) -> Result<Request, String> {
    let mut reader = BufReader::new(stream.try_clone().map_err(|failure| failure.to_string())?);
    let mut line = String::new();
    reader.read_line(&mut line).map_err(|failure| failure.to_string())?;
    let mut parts = line.trim_end().split(' ');
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or_default().to_string();
    let mut headers = HashMap::new();
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).map_err(|failure| failure.to_string())? == 0 {
            break;
        }
        let header = header.trim_end();
        if header.is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }
    let length: u64 = headers.get("content-length").and_then(|value| value.parse().ok()).unwrap_or(0);
    if length > limit {
        return Err("request body too large".to_string());
    }
    let mut body = vec![0u8; length as usize];
    if length > 0 {
        reader.read_exact(&mut body).map_err(|failure| failure.to_string())?;
    }
    let (path, raw_query) = target.split_once('?').unwrap_or((target.as_str(), ""));
    let mut query = HashMap::new();
    for pair in raw_query.split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        query.insert(percent_decode(key), percent_decode(value));
    }
    Ok(Request { method, path: percent_decode(path), query, headers, body })
}

fn percent_decode(text: &str) -> String {
    let bytes = text.replace('+', " ").into_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&String::from_utf8_lossy(&bytes[index + 1..index + 3]), 16) {
                out.push(byte);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}
//#endregion 🌐️Http
