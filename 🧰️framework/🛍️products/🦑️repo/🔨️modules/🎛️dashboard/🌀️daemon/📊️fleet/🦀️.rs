//! 📊️ The fleet feed of the daemon: a view that asked to watch the fleet gets compact snapshots of the shared store, read off
//! the supervisor thread, never more often than the range allows (at least one second apart), only when something changed
//! (or as a heartbeat so the ages of origins advance) and only for the range, the column budget and the origin it asked for.
//! A store that cannot be read is told as an unavailable snapshot; a read never blocks a turn.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/📊️fleet/🔣️.json

use super::clients::Client;
use super::ipc::{self, ServerMsg};
use super::transport::Waker;
use dashboard_fleet::{origins_directory, read_snapshot_with, Policy, Reader, Request, Snapshot};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

/// 🔭️ The environment names that point at the store: an explicit store root and the declared machine state directory.
pub const STORE_ROOT_ENVIRONMENT: &str = "SEMIO_FLEET_STORE_ROOT";
pub const MACHINE_STATE_ENVIRONMENT: &str = "SEMIO_MACHINE_STATE_DIRECTORY";

/// 👁️ What one view watches and what it was told last.
pub struct Watch {
    pub request: Request,
    next: Instant,
    sent: Option<(Instant, Box<Snapshot>)>,
}

impl Watch {
    /// 🆕 A watch that is due at once.
    pub fn new(request: Request) -> Self { Self { request, next: Instant::now(), sent: None } }
}

type Done = (u64, Request, Snapshot);

/// 🔄️ The reader thread of the daemon, one read at a time.
pub struct Feed {
    sender: mpsc::Sender<Done>,
    results: mpsc::Receiver<Done>,
    busy: bool,
    reader: Arc<Mutex<Reader>>,
}

impl Default for Feed {
    fn default() -> Self {
        let (sender, results) = mpsc::channel();
        Self { sender, results, busy: false, reader: Arc::default() }
    }
}

fn pick(environment: &[(String, String)], name: &str) -> Option<String> {
    let found = environment.iter().rev().find(|(key, _)| if cfg!(windows) { key.eq_ignore_ascii_case(name) } else { key == name }).map(|(_, value)| value.clone());
    match found { Some(value) => Some(value), None => std::env::var(name).ok() }.filter(|value| !value.is_empty())
}

/// 🗄️ The origins folder of the store for a view's environment, or `None` when no store is configured.
pub fn store_of(workspace: &Path, environment: &[(String, String)]) -> Option<PathBuf> {
    let explicit = pick(environment, STORE_ROOT_ENVIRONMENT);
    let state = pick(environment, MACHINE_STATE_ENVIRONMENT).map(PathBuf::from).filter(|path| path.is_absolute());
    origins_directory(workspace, explicit.as_deref(), state.as_deref())
}

impl Feed {
    /// 🔁️ Takes finished reads to the views that asked for them and starts the next due read.
    pub fn pump(&mut self, clients: &mut BTreeMap<u64, Client>, workspace: &Path, waker: &Waker) {
        let heartbeat = Duration::from_millis(Policy::get().heartbeat_ms);
        while let Ok((id, request, snapshot)) = self.results.try_recv() {
            self.busy = false;
            let Some(client) = clients.get_mut(&id) else { continue };
            let Some(watch) = client.fleet.as_mut().filter(|watch| watch.request == request) else { continue };
            let now = Instant::now();
            watch.next = now + Duration::from_millis(request.range.refresh_ms());
            let changed = watch.sent.as_ref().is_none_or(|(at, last)| !last.same_content(&snapshot) || now.duration_since(*at) >= heartbeat);
            if changed {
                client.tell(&ServerMsg::Fleet { snapshot: Box::new(snapshot.clone()) });
                if let Some(watch) = client.fleet.as_mut() { watch.sent = Some((now, Box::new(snapshot))); }
            }
        }
        if self.busy { return; }
        let now = Instant::now();
        let due = clients.iter().filter_map(|(id, client)| client.fleet.as_ref().filter(|watch| watch.next <= now).map(|watch| (watch.next, *id))).min();
        let Some((_, id)) = due else { return };
        let client = &clients[&id];
        let request = client.fleet.as_ref().map(|watch| watch.request.clone()).expect("a due watch");
        let store = store_of(workspace, client.environment.as_deref().unwrap_or(&[]));
        let (sender, wake, reader) = (self.sender.clone(), waker.clone(), Arc::clone(&self.reader));
        let spawned = std::thread::Builder::new().name("dashboard-fleet-read".into()).spawn(move || {
            let snapshot = match reader.lock() { Ok(mut reader) => read_snapshot_with(&mut reader, store.as_deref(), &request, ipc::now_ms()), Err(poisoned) => read_snapshot_with(&mut poisoned.into_inner(), store.as_deref(), &request, ipc::now_ms()) };
            let _ = sender.send((id, request, snapshot));
            wake.wake();
        });
        match spawned {
            Ok(_) => self.busy = true,
            Err(_) => if let Some(watch) = clients.get_mut(&id).and_then(|client| client.fleet.as_mut()) { watch.next = now + Duration::from_secs(1); },
        }
    }
}
