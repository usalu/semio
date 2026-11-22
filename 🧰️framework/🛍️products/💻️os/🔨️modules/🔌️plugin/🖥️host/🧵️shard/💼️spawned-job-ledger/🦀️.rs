//! 💼️ The ledger of LIVE spawned plugin jobs — the Rust twin of the React `🔌️PluginRuntime/💼️job-ledger`, driven by the shared `💼️job-ledger` vectors; it lives beside the shard because the shard is the one native producer. A job drive opens one entry when it admits the job, advances it per admitted step batch and ends it when the drive ends; the Task Manager lists the snapshot with a Cancel. A cancel is cooperative: it reaches the job's own owner through the entry's `cancel` and only marks the row `cancelling`, the entry leaves when the drive itself ends.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// 💼️ What a drive states when it opens its entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpawnedJob {
    pub key: String,
    pub plugin_id: String,
    pub actor_id: String,
    pub instance_id: u32,
    pub job: u64,
    pub kind: String,
    pub started_at_ms: u64,
}

/// 💼️ One live spawned job as the Task Manager shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpawnedJobRow {
    pub job: SpawnedJob,
    pub steps: u64,
    pub reports_progress: bool,
    pub progress_at_ms: u64,
    pub cancelling: bool,
}

type Cancel = Arc<dyn Fn() + Send + Sync>;

/// 📒️ The entries keyed by `${actor}#${job}`.
#[derive(Default)]
pub struct JobLedger(BTreeMap<String, (SpawnedJobRow, Cancel)>);

impl JobLedger {
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// 💼️ Opens the entry; `false` when the key is still live, so a duplicated spawn can never close the real drive's row and its drive must not call [`Self::end`].
    pub fn begin(&mut self, job: SpawnedJob, cancel: Cancel) -> bool {
        if self.0.contains_key(&job.key) {
            return false;
        }
        let progress_at_ms = job.started_at_ms;
        self.0.insert(job.key.clone(), (SpawnedJobRow { job, steps: 0, reports_progress: false, progress_at_ms, cancelling: false }, cancel));
        true
    }

    pub fn step(&mut self, key: &str, steps: u64, progressed: bool, at_ms: u64) {
        if let Some((row, _)) = self.0.get_mut(key) {
            (row.steps, row.reports_progress, row.progress_at_ms) = (steps, row.reports_progress || progressed, at_ms);
        }
    }

    /// 📈️ [`Self::step`] for an observer that polls: nothing changes (so `progress_at_ms` keeps the last real advance) until the step count moved or progress was reported for the first time.
    fn advance(&mut self, key: &str, steps: u64, progressed: bool, at_ms: u64) {
        if self.0.get(key).is_some_and(|(row, _)| row.steps != steps || (progressed && !row.reports_progress)) {
            self.step(key, steps, progressed, at_ms);
        }
    }

    pub fn end(&mut self, key: &str) {
        self.0.remove(key);
    }

    /// 🛑️ Marks the row `cancelling` and hands back the owner's cancel to call once the caller released the ledger; `None` when no such job is live or it was already asked.
    pub fn request_cancel(&mut self, key: &str) -> Option<Cancel> {
        let (row, cancel) = self.0.get_mut(key).filter(|(row, _)| !row.cancelling)?;
        row.cancelling = true;
        Some(cancel.clone())
    }

    /// 📸️ The live rows, oldest first.
    pub fn snapshot(&self) -> Vec<SpawnedJobRow> {
        let mut rows: Vec<SpawnedJobRow> = self.0.values().map(|(row, _)| row.clone()).collect();
        rows.sort_by(|left, right| (left.job.started_at_ms, &left.job.key).cmp(&(right.job.started_at_ms, &right.job.key)));
        rows
    }
}

static LEDGER: Mutex<JobLedger> = Mutex::new(JobLedger::new());

fn with_ledger<T>(use_ledger: impl FnOnce(&mut JobLedger) -> T) -> Option<T> {
    LEDGER.lock().ok().map(|mut ledger| use_ledger(&mut ledger))
}

/// 💼️ Opens the process ledger's entry of one job drive.
pub fn begin_spawned_job(job: SpawnedJob, cancel: impl Fn() + Send + Sync + 'static) -> bool {
    with_ledger(|ledger| ledger.begin(job, Arc::new(cancel))).unwrap_or(false)
}

fn advance_spawned_job(key: &str, steps: u64, progressed: bool, at_ms: u64) {
    with_ledger(|ledger| ledger.advance(key, steps, progressed, at_ms));
}

pub fn step_spawned_job(key: &str, steps: u64, progressed: bool, at_ms: u64) {
    with_ledger(|ledger| ledger.step(key, steps, progressed, at_ms));
}

pub fn end_spawned_job(key: &str) {
    with_ledger(|ledger| ledger.end(key));
}

/// 🛑️ Asks the job's own owner to stop it; `false` when no such job is live or it was already asked.
pub fn cancel_spawned_job(key: &str) -> bool {
    let Some(cancel) = with_ledger(|ledger| ledger.request_cancel(key)).flatten() else { return false };
    cancel();
    true
}

/// 📸️ The live rows of the process ledger, oldest first.
pub fn spawned_jobs_snapshot() -> Vec<SpawnedJobRow> {
    with_ledger(|ledger| ledger.snapshot()).unwrap_or_default()
}

/// 🔭️ One job a shard drives right now; `reports_progress` once a step of it handed the shell progress bytes (React's `progressed`, here a published preview).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveJob {
    pub actor: u64,
    pub job: u64,
    pub kind: String,
    pub steps: u64,
    pub reports_progress: bool,
}

static OWNERS: Mutex<BTreeMap<String, usize>> = Mutex::new(BTreeMap::new());
static CANCELS: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64)
}

fn reconcile_with(owner: usize, prefix: &str, live: &[LiveJob], cancel_of: impl Fn(u64, u64) -> Arc<dyn Fn() + Send + Sync>) {
    let keys: Vec<String> = live.iter().map(|job| format!("{prefix}{}#{}", job.actor, job.job)).collect();
    let Ok(mut owners) = OWNERS.lock() else { return };
    for (job, key) in live.iter().zip(&keys) {
        let spawned = SpawnedJob { key: key.clone(), plugin_id: String::new(), actor_id: job.actor.to_string(), instance_id: 0, job: job.job, kind: job.kind.clone(), started_at_ms: now_ms() };
        if with_ledger(|ledger| ledger.begin(spawned, cancel_of(job.actor, job.job))).unwrap_or(false) {
            owners.insert(key.clone(), owner);
        }
        advance_spawned_job(key, job.steps, job.reports_progress, now_ms());
    }
    let ended: Vec<String> = owners.iter().filter(|(key, token)| **token == owner && !keys.contains(key)).map(|(key, _)| key.clone()).collect();
    for key in ended {
        end_spawned_job(&key);
        owners.remove(&key);
    }
}

/// 🔁️ Makes the ledger rows of one shard (`owner`, any token unique to it) exactly its `live` jobs: a job seen for the first time opens its entry with a cancel that queues the cancel for the shard, a live job advances with its step count and whether it reported progress, and a job the shard no longer drives ends its entry.
pub fn reconcile_spawned_jobs(owner: usize, live: &[LiveJob]) {
    reconcile_with(owner, "", live, |actor, job| Arc::new(move || { if let Ok(mut queue) = CANCELS.lock() { queue.push((actor, job)); } }));
}

/// 🛰️ The same listing for the jobs a child-process shard drives: its rows are namespaced by the transport (`owner`) so two processes never share a key, and a Cancel press goes to the child through `cancel`, never to this process's own queue. An empty `live` ends every row of that child.
pub fn reconcile_remote_jobs(owner: usize, live: &[LiveJob], cancel: impl Fn(u64, u64) + Clone + Send + Sync + 'static) {
    reconcile_with(owner, &format!("p{owner:x}:"), live, |actor, job| { let cancel = cancel.clone(); Arc::new(move || cancel(actor, job)) });
}

/// 🛰️ Mirrors the jobs a child-process shard reported into this process's ledger (called on every receive and after a kill): the newest snapshot replaces the child's rows (owned by its pid), and a dead or killed child ends them all.
pub async fn mirror_child_jobs(transport: &semio_plugin_host_process_transport::ProcessTransport) {
    let Some(owner) = transport.child_id().await.map(|pid| pid as usize) else { return };
    if !transport.is_child_alive().await {
        return reconcile_remote_jobs(owner, &[], |_, _| {});
    }
    if let Some(live) = transport.take_jobs_snapshot().and_then(|bytes| decode_live_jobs(&bytes)) {
        let cancel = Arc::new(transport.job_cancel_sender());
        reconcile_remote_jobs(owner, &live, move |actor, job| cancel(actor, job));
    }
}

/// 📸️ The live jobs of this process's own ledger as a shard would state them, what a child process tells its parent.
pub fn local_live_jobs() -> Vec<LiveJob> {
    spawned_jobs_snapshot().into_iter().filter_map(|row| Some(LiveJob { actor: row.job.actor_id.parse().ok()?, job: row.job.job, kind: row.job.kind, steps: row.steps, reports_progress: row.reports_progress })).collect()
}

const WIRE_MAXIMUM_JOBS: usize = 4096;
const WIRE_MAXIMUM_KIND_BYTES: usize = 256;

/// 💼️ The wire form of a live-job list, `[count:u32 LE]` then per job `[actor:u64][job:u64][steps:u64][reports:u8][kind_len:u16][kind]`, all little endian; bounded, kinds cut at a character boundary.
pub fn encode_live_jobs(live: &[LiveJob]) -> Vec<u8> {
    let live = &live[..live.len().min(WIRE_MAXIMUM_JOBS)];
    let mut out = (live.len() as u32).to_le_bytes().to_vec();
    for job in live {
        let kind = job.kind.char_indices().take_while(|(index, character)| index + character.len_utf8() <= WIRE_MAXIMUM_KIND_BYTES).last().map_or("", |(index, character)| &job.kind[..index + character.len_utf8()]);
        out.extend_from_slice(&job.actor.to_le_bytes());
        out.extend_from_slice(&job.job.to_le_bytes());
        out.extend_from_slice(&job.steps.to_le_bytes());
        out.push(u8::from(job.reports_progress));
        out.extend_from_slice(&(kind.len() as u16).to_le_bytes());
        out.extend_from_slice(kind.as_bytes());
    }
    out
}

/// 💼️ Reads [`encode_live_jobs`]; `None` for anything truncated, oversized, trailing or not UTF-8.
pub fn decode_live_jobs(bytes: &[u8]) -> Option<Vec<LiveJob>> {
    fn take<'a>(bytes: &mut &'a [u8], width: usize) -> Option<&'a [u8]> {
        let (head, tail) = bytes.split_at_checked(width)?;
        *bytes = tail;
        Some(head)
    }
    let mut rest = bytes;
    let count = u32::from_le_bytes(take(&mut rest, 4)?.try_into().ok()?) as usize;
    if count > WIRE_MAXIMUM_JOBS {
        return None;
    }
    let mut live = Vec::with_capacity(count);
    for _ in 0..count {
        let actor = u64::from_le_bytes(take(&mut rest, 8)?.try_into().ok()?);
        let job = u64::from_le_bytes(take(&mut rest, 8)?.try_into().ok()?);
        let steps = u64::from_le_bytes(take(&mut rest, 8)?.try_into().ok()?);
        let reports_progress = match take(&mut rest, 1)?[0] { 0 => false, 1 => true, _ => return None };
        let kind_length = u16::from_le_bytes(take(&mut rest, 2)?.try_into().ok()?) as usize;
        if kind_length > WIRE_MAXIMUM_KIND_BYTES {
            return None;
        }
        let kind = String::from_utf8(take(&mut rest, kind_length)?.to_vec()).ok()?;
        live.push(LiveJob { actor, job, kind, steps, reports_progress });
    }
    rest.is_empty().then_some(live)
}

/// 🛑️ Takes the cancels the Task Manager queued for jobs of `owner`'s shard; the shard applies each exactly as it applies a guest's `Effect::CancelJob`.
pub fn take_job_cancels(owner: usize) -> Vec<(u64, u64)> {
    let (Ok(owners), Ok(mut queue)) = (OWNERS.lock(), CANCELS.lock()) else { return Vec::new() };
    let (mine, rest): (Vec<_>, Vec<_>) = core::mem::take(&mut *queue).into_iter().partition(|(actor, job)| owners.get(&format!("{actor}#{job}")) == Some(&owner));
    *queue = rest;
    mine
}
