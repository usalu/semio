//! 🔐️ Fixed document-writer capabilities and backend-owned exclusion guards.
use super::{DbIoBackendControl, DbIoText};
use crate::DbError;

pub(crate) const WAL_WRITER_CAPACITY: usize = 32;
const _: () = assert!(WAL_WRITER_CAPACITY <= u8::MAX as usize);

/// 🎟️ Non-cloneable writer ownership; only its issuing backend may validate or retire it.
pub struct WalWriterPermit {
    key: WalWriterKey,
    document: DbIoText,
}

/// 🧷 Internal task stamp; an old stamp never authorizes a recycled slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WalWriterKey {
    backend: DbIoBackendControl,
    slot: u8,
    generation: u64,
}

impl WalWriterPermit {
    pub(crate) fn key(&self) -> WalWriterKey { self.key }
    pub(crate) fn document(&self) -> &DbIoText { &self.document }
}

struct WalWriterEntry<G> {
    document: DbIoText,
    generation: u64,
    active_operation: Option<u64>,
    releasing: bool,
    guard: G,
}

/// 🧹 A backend guard keeps failed or partial release ownership until its terminal witness.
pub(crate) trait WalWriterGuard {
    fn close_step(&mut self) -> Result<bool, DbError>;
    fn terminal_is_empty(&self) -> bool;
}

impl WalWriterGuard for () {
    fn close_step(&mut self) -> Result<bool, DbError> { Ok(false) }
    fn terminal_is_empty(&self) -> bool { true }
}

/// 🗃️ Executor-owned bounded slots; a guard remains retained until explicit release.
pub(crate) struct WalWriterTable<G> {
    backend: DbIoBackendControl,
    next_generation: u64,
    close_cursor: usize,
    entries: [Option<WalWriterEntry<G>>; WAL_WRITER_CAPACITY],
}

impl<G: WalWriterGuard> WalWriterTable<G> {
    pub(crate) fn new(backend: DbIoBackendControl) -> Self {
        Self { backend, next_generation: 1, close_cursor: 0, entries: std::array::from_fn(|_| None) }
    }

    pub(crate) fn acquire(&mut self, document: &DbIoText, guard: G) -> Result<WalWriterPermit, (DbError, G)> {
        if document.as_str().is_empty() { return Err((DbError::InvalidArgument("empty WAL writer document".to_string()), guard)); }
        if self.entries.iter().flatten().any(|entry| entry.document == *document) { return Err((DbError::Conflict("WAL document already has a writer".to_string()), guard)); }
        let Some(slot) = self.entries.iter().position(Option::is_none) else { return Err((DbError::LimitExceeded("WAL writer capacity"), guard)); };
        let Some(next) = self.next_generation.checked_add(1) else { return Err((DbError::LimitExceeded("WAL writer generation"), guard)); };
        let generation = self.next_generation;
        self.entries[slot] = Some(WalWriterEntry { document: document.clone(), generation, active_operation: None, releasing: false, guard });
        self.next_generation = next;
        Ok(WalWriterPermit { key: WalWriterKey { backend: self.backend, slot: slot as u8, generation }, document: document.clone() })
    }

    fn matching_entry(&self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<&WalWriterEntry<G>, DbError> {
        let entry = self.entries.get(usize::from(key.slot)).and_then(Option::as_ref);
        if backend != self.backend || key.backend != backend || entry.is_none_or(|entry| entry.generation != key.generation || entry.document != *document) {
            return Err(DbError::Fenced { expected: entry.map_or(0, |entry| entry.generation), actual: key.generation });
        }
        Ok(entry.expect("validated writer slot"))
    }

    pub(crate) fn validate(&self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<&G, DbError> {
        let entry = self.matching_entry(key, backend, document)?;
        if entry.releasing { return Err(DbError::Closed); }
        Ok(&entry.guard)
    }

    pub(crate) fn pin_operation(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<&G, DbError> {
        self.matching_entry(key, backend, document)?;
        if operation == 0 { return Err(DbError::InvalidArgument("zero WAL writer operation".to_string())); }
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if entry.releasing && entry.active_operation != Some(operation) { return Err(DbError::Closed); }
        if entry.active_operation.is_some_and(|active| active != operation) { return Err(DbError::Conflict("WAL writer operation already admitted".to_string())); }
        entry.active_operation = Some(operation);
        Ok(&entry.guard)
    }

    pub(crate) fn finish_operation(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText, operation: u64) -> Result<(), DbError> {
        self.matching_entry(key, backend, document)?;
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        if entry.active_operation != Some(operation) { return Err(DbError::Fenced { expected: entry.active_operation.unwrap_or(0), actual: operation }); }
        entry.active_operation = None;
        Ok(())
    }

    pub(crate) fn release_step(&mut self, key: WalWriterKey, backend: DbIoBackendControl, document: &DbIoText) -> Result<bool, DbError> {
        self.matching_entry(key, backend, document)?;
        let entry = self.entries[usize::from(key.slot)].as_mut().expect("validated writer slot");
        entry.releasing = true;
        if entry.active_operation.is_some() || entry.guard.close_step()? { return Ok(true); }
        if !entry.guard.terminal_is_empty() { return Err(DbError::Internal("WAL writer guard returned a false terminal witness".to_string())); }
        self.entries[usize::from(key.slot)] = None;
        Ok(false)
    }

    pub(crate) fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(slot) = (0..WAL_WRITER_CAPACITY).map(|offset| (self.close_cursor + offset) % WAL_WRITER_CAPACITY).find(|slot| self.entries[*slot].is_some()) else { return Ok(false) };
        self.close_cursor = (slot + 1) % WAL_WRITER_CAPACITY;
        let entry = self.entries[slot].as_ref().expect("selected retained writer slot");
        let key = WalWriterKey { backend: self.backend, slot: slot as u8, generation: entry.generation };
        let document = entry.document.clone();
        self.release_step(key, self.backend, &document)?;
        Ok(true)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool { self.entries.iter().all(Option::is_none) }
}

/// 📁 Cross-process sidecar exclusion; called only from the backend's admitted I/O lane.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct WalFileWriterGuard {
    file: Option<std::fs::File>,
}

#[cfg(not(target_arch = "wasm32"))]
impl WalFileWriterGuard {
    pub(crate) fn try_acquire(path: &std::path::Path) -> Result<Self, DbError> {
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path).map_err(|error| DbError::Io(error.to_string()))?;
        file.try_lock().map_err(|error| match error {
            std::fs::TryLockError::WouldBlock => DbError::Conflict("WAL document already has a filesystem writer".to_string()),
            std::fs::TryLockError::Error(error) => DbError::Io(error.to_string()),
        })?;
        Ok(Self { file: Some(file) })
    }

    pub(crate) fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(file) = self.file.as_ref() else { return Ok(false) };
        file.unlock().map_err(|error| DbError::Io(error.to_string()))?;
        self.file = None;
        Ok(true)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool { self.file.is_none() }
}

#[cfg(not(target_arch = "wasm32"))]
impl WalWriterGuard for WalFileWriterGuard {
    fn close_step(&mut self) -> Result<bool, DbError> { WalFileWriterGuard::close_step(self) }
    fn terminal_is_empty(&self) -> bool { WalFileWriterGuard::terminal_is_empty(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl WalWriterGuard for usize {
        fn close_step(&mut self) -> Result<bool, DbError> {
            if *self == usize::MAX { return Ok(false); }
            *self = usize::MAX;
            Ok(true)
        }
        fn terminal_is_empty(&self) -> bool { *self == usize::MAX }
    }

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap() }
    fn backend(slot: u16) -> DbIoBackendControl { DbIoBackendControl::Memory { slot, generation: 1 } }
    fn disposition(result: Result<(), DbError>) -> &'static str {
        match result {
            Ok(()) => "ok",
            Err(DbError::Conflict(_)) => "conflict",
            Err(DbError::Fenced { .. }) => "fenced",
            Err(DbError::LimitExceeded("WAL writer generation")) => "exhausted",
            Err(error) => panic!("unexpected writer result: {error}"),
        }
    }

    #[test]
    fn wal_writer_table_matches_neutral_exact_scope_and_aba_rejection() {
        let fixture = fixture();
        for row in fixture["cases"].as_array().unwrap() {
            let mut table = WalWriterTable::new(backend(0));
            table.next_generation = row["firstGeneration"].as_str().unwrap().parse().unwrap();
            let mut owners = std::collections::BTreeMap::<String, WalWriterPermit>::new();
            for step in row["steps"].as_array().unwrap() {
                let document = DbIoText::try_from_str(step["document"].as_str().unwrap()).unwrap();
                let owner = step["owner"].as_str().unwrap();
                let selected = backend(step["backend"].as_u64().unwrap() as u16);
                let result = match step["action"].as_str().unwrap() {
                    "acquire" => match table.acquire(&document, ()) {
                        Ok(permit) => { assert!(permit.document() == &document); owners.insert(owner.into(), permit); Ok(()) }
                        Err((error, ())) => Err(error),
                    },
                    "validate" => {
                        let key = owners[owner].key();
                        for _ in fixture["mutations"].as_array().unwrap() {
                            assert_eq!(disposition(table.validate(key, selected, &document).map(|_| ())), step["expected"].as_str().unwrap());
                        }
                        table.validate(key, selected, &document).map(|_| ())
                    }
                    "release" => table.release_step(owners[owner].key(), selected, &document).map(|pending| assert!(!pending)),
                    _ => unreachable!(),
                };
                assert_eq!(disposition(result), step["expected"].as_str().unwrap(), "{step}");
            }
            assert!(table.terminal_is_empty());
            eprintln!("[DEBUG] fixed WAL writer table matched exact neutral ownership and stale-generation decisions: {}", row["name"]);
        }
    }

    #[test]
    fn wal_writer_table_capacity_recycles_slots_without_reusing_generations() {
        let mut table = WalWriterTable::new(backend(0));
        assert_eq!(fixture()["capacity"].as_u64().unwrap() as usize, WAL_WRITER_CAPACITY);
        let mut owners = Vec::new();
        for ordinal in 0..WAL_WRITER_CAPACITY {
            let document = DbIoText::try_from_str(&format!("document-{ordinal}")).unwrap();
            owners.push(table.acquire(&document, ordinal).unwrap());
        }
        let spare = DbIoText::try_from_str("spare").unwrap();
        assert!(matches!(table.acquire(&spare, 99), Err((DbError::LimitExceeded("WAL writer capacity"), 99))));
        let old = owners.remove(0);
        assert!(table.release_step(old.key(), backend(0), old.document()).unwrap());
        assert!(!table.release_step(old.key(), backend(0), old.document()).unwrap());
        let fresh = table.acquire(old.document(), 100).unwrap();
        assert_eq!(old.key().slot, fresh.key().slot);
        assert_ne!(old.key().generation, fresh.key().generation);
        assert!(matches!(table.validate(old.key(), backend(0), old.document()), Err(DbError::Fenced { .. })));
        assert_eq!(*table.validate(fresh.key(), backend(0), fresh.document()).unwrap(), 100);
        let mut retired = 0;
        while table.close_step().unwrap() { retired += 1; }
        assert_eq!(retired, WAL_WRITER_CAPACITY * 2);
        assert!(table.terminal_is_empty());
        assert!(matches!(table.validate(fresh.key(), backend(0), fresh.document()), Err(DbError::Fenced { .. })));
        eprintln!("[DEBUG] WAL writer slots rejected capacity+1, preserved guards, recycled without ABA, and retired {WAL_WRITER_CAPACITY} guards in {retired} opportunities");
    }

    #[derive(Debug)]
    struct FaultGuard { fail: bool, closed: bool }

    impl WalWriterGuard for FaultGuard {
        fn close_step(&mut self) -> Result<bool, DbError> {
            if self.fail { self.fail = false; return Err(DbError::Io("injected writer unlock failure".to_string())); }
            if self.closed { return Ok(false); }
            self.closed = true;
            Ok(true)
        }
        fn terminal_is_empty(&self) -> bool { self.closed }
    }

    #[test]
    fn wal_writer_release_retains_pinned_operation_and_faulted_guard() {
        let fixture = fixture();
        let operation = fixture["guardRetirement"]["operation"].as_u64().unwrap();
        let contender = fixture["guardRetirement"]["contender"].as_u64().unwrap();
        let document = DbIoText::try_from_str("pinned-writer").unwrap();
        let mut table = WalWriterTable::new(backend(0));
        let permit = table.acquire(&document, FaultGuard { fail: true, closed: false }).unwrap();
        let key = permit.key();
        table.pin_operation(key, backend(0), &document, operation).unwrap();
        assert!(matches!(table.pin_operation(key, backend(0), &document, contender), Err(DbError::Conflict(_))));
        let mut trace = vec![!table.release_step(key, backend(0), &document).unwrap()];
        assert!(!table.terminal_is_empty());
        assert!(matches!(table.validate(key, backend(0), &document), Err(DbError::Closed)));
        assert!(matches!(table.pin_operation(key, backend(0), &document, contender), Err(DbError::Closed)));
        assert!(!table.pin_operation(key, backend(0), &document, operation).unwrap().closed);
        assert!(matches!(table.finish_operation(key, backend(0), &document, contender), Err(DbError::Fenced { .. })));
        table.finish_operation(key, backend(0), &document, operation).unwrap();
        assert!(matches!(table.release_step(key, backend(0), &document), Err(DbError::Io(_))));
        trace.push(table.terminal_is_empty());
        trace.push(!table.release_step(key, backend(0), &document).unwrap());
        assert!(!table.terminal_is_empty());
        trace.push(!table.release_step(key, backend(0), &document).unwrap());
        assert!(table.terminal_is_empty());
        assert_eq!(serde_json::to_value(trace).unwrap(), fixture["guardRetirement"]["terminal"]);
        assert!(matches!(table.pin_operation(key, backend(0), &document, operation), Err(DbError::Fenced { .. })));
        eprintln!("[DEBUG] WAL writer release retained the in-flight operation, fenced new work, preserved a faulted guard, and waited for its terminal close witness");
    }

    #[test]
    fn wal_writer_table_close_advances_other_guards_while_first_operation_is_pinned() {
        let fixture = fixture();
        let row = &fixture["fairRetirement"];
        let mut table = WalWriterTable::new(backend(0));
        let pinned = table.acquire(&DbIoText::try_from_str("pinned").unwrap(), FaultGuard { fail: false, closed: false }).unwrap();
        let releasing = table.acquire(&DbIoText::try_from_str("releasing").unwrap(), FaultGuard { fail: false, closed: false }).unwrap();
        assert_eq!(usize::from(pinned.key().slot), row["pinnedSlot"].as_u64().unwrap() as usize);
        assert_eq!(usize::from(releasing.key().slot), row["releasingSlot"].as_u64().unwrap() as usize);
        table.pin_operation(pinned.key(), backend(0), pinned.document(), 7).unwrap();
        assert!(table.release_step(releasing.key(), backend(0), releasing.document()).unwrap());
        for _ in 0..row["maximumOpportunities"].as_u64().unwrap() { assert!(table.close_step().unwrap()); }
        assert_eq!(table.entries[usize::from(pinned.key().slot)].is_some(), row["pinnedRetained"].as_bool().unwrap());
        assert_eq!(table.entries[usize::from(releasing.key().slot)].is_none(), row["releasingRetired"].as_bool().unwrap());
        assert!(matches!(table.pin_operation(pinned.key(), backend(0), pinned.document(), 8), Err(DbError::Closed)));
        assert!(matches!(table.pin_operation(releasing.key(), backend(0), releasing.document(), 8), Err(DbError::Fenced { .. })));
        table.finish_operation(pinned.key(), backend(0), pinned.document(), 7).unwrap();
        for _ in 0..WAL_WRITER_CAPACITY * 2 { let _ = table.close_step().unwrap(); }
        assert!(table.terminal_is_empty());
        eprintln!("[DEBUG] bounded WAL writer close retired the later guard while the first operation stayed pinned, then retired the final owner");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn wal_writer_file_lock_excludes_independent_instances_and_processes() {
        if let Some(path) = std::env::var_os("SEMIO_WAL_WRITER_CHILD_PATH") {
            assert!(matches!(WalFileWriterGuard::try_acquire(std::path::Path::new(&path)), Err(DbError::Conflict(_))));
            let sentinel = std::env::var_os("SEMIO_WAL_WRITER_CHILD_SENTINEL").expect("child writer proof path");
            std::fs::write(sentinel, format!("{}:conflict", std::process::id())).unwrap();
            return;
        }
        let fixture = fixture();
        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = base.join(format!("wal-writer-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join(fixture["filesystem"]["sidecar"].as_str().unwrap());
        let sentinel = root.join("child-conflict-proof.txt");
        let mut first = WalFileWriterGuard::try_acquire(&path).unwrap();
        assert!(matches!(WalFileWriterGuard::try_acquire(&path), Err(DbError::Conflict(_))));
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["db_storage::writer::tests::wal_writer_file_lock_excludes_independent_instances_and_processes", "--exact", "--test-threads=1"])
            .env("SEMIO_WAL_WRITER_CHILD_PATH", &path).env("SEMIO_WAL_WRITER_CHILD_SENTINEL", &sentinel).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            if let Some(status) = child.try_wait().unwrap() { assert!(status.success(), "independent writer process did not reject the held lock"); break; }
            if std::time::Instant::now() >= deadline { child.kill().unwrap(); child.wait().unwrap(); panic!("independent writer process deadline"); }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(std::fs::read_to_string(&sentinel).unwrap(), format!("{}:conflict", child.id()));
        for flag in ["independentInstancesConflict", "independentProcessesConflict", "reacquireAfterClose"] { assert_eq!(fixture["filesystem"][flag], true); }
        assert_eq!(fixture["filesystem"]["unlinkOnClose"], false);
        while first.close_step().unwrap() {}
        assert!(first.terminal_is_empty());
        assert!(path.exists(), "closing must not unlink the lock inode");
        let mut second = WalFileWriterGuard::try_acquire(&path).unwrap();
        assert!(matches!(WalFileWriterGuard::try_acquire(&path), Err(DbError::Conflict(_))));
        while second.close_step().unwrap() {}
        assert!(second.terminal_is_empty());
        eprintln!("[DEBUG] native WAL sidecar lock excluded independent handles and a separate process, then reacquired after terminal close without unlinking");
    }
}
