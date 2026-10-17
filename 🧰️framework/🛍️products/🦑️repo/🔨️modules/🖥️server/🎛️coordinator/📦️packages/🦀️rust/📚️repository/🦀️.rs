//! 📚️ Repository of the coordinator: the event-sourced repository with its open, close, reload and command execution. See [`super`].

use super::*;

//#region 📚️Repository
/// 🗄️ Executes commands against the log and derives every query by replay.
#[derive(Debug)]
pub struct Repository {
    pub(super) store: EventStore,
    commands: Mutex<()>,
    pub(super) projection: Mutex<Projection>,
    pub(super) closed: AtomicBool,
}

impl Repository {
    /// 🆕️ Opens the owned coordinator event repository at a path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let store = EventStore::open(path, StoreLimits::default())?;
        let repository = Self { store, commands: Mutex::new(()), projection: Mutex::new(Projection::default()), closed: AtomicBool::new(false) };
        repository.reload()?;
        Ok(repository)
    }

    /// 📪️ Makes repository operations explicitly unavailable without discarding durable state.
    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
    }

    /// 🔌️ Recovers a short storage shortage and rebuilds the projection.
    pub fn reopen(&self) -> Result<(), StoreError> {
        self.reload()?;
        self.closed.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// 📊️ A snapshot of the replayed projection.
    pub fn projection(&self) -> Projection {
        self.projection.lock().expect("projection").clone()
    }

    fn reload(&self) -> Result<(), StoreError> {
        let events = self.store.replay()?;
        let mut projection = Projection::default();
        for event in &events {
            projection.apply(event)?;
        }
        *self.projection.lock().expect("projection") = projection;
        Ok(())
    }

    pub(super) fn execute(&self, kind: &str, payload: Value) -> Result<(), StoreError> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(StoreError::Unavailable("repository closed".to_string()));
        }
        let guard = self.commands.lock().expect("commands");
        let input = EventInput { stream: COORDINATOR_STREAM.to_string(), id: command_id(kind, 1, &payload), generation: 1, kind: kind.to_string(), payload };
        for _ in 0..3 {
            let expected = self.projection.lock().expect("projection").sequence;
            match self.store.append(expected, std::slice::from_ref(&input)) {
                Ok(result) => {
                    let mut projection = self.projection.lock().expect("projection");
                    for event in &result.events {
                        if event.sequence > projection.sequence {
                            projection.apply(event)?;
                        }
                    }
                    drop(guard);
                    return Ok(());
                }
                Err(StoreError::SequenceConflict(_)) => self.reload()?,
                Err(failure) => {
                    drop(guard);
                    return Err(failure);
                }
            }
        }
        drop(guard);
        Err(StoreError::SequenceConflict("retries exhausted".to_string()))
    }
}
//#endregion 📚️Repository
