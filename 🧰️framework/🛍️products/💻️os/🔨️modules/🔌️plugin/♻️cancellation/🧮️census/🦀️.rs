//! 📊️ Calling-thread typed-operation publication census.
/// 📊️ What the typed-operation publication ladder spends its units on, on the calling thread. One unit is one
/// step of the publication state machine; a HOST TURN is one or more units
/// (the host publication pump), so "how many turns does one mutation cost" is only
/// answerable by splitting the units by the ladder that ran them.
///
/// 🧾️ Before wave B54 every unit was its own host round trip, and one `deleteSelection` on a
/// one-object document spent 104–235 of them (wave B44 §2.3) with no way to say which ladder owned
/// them. This census is that instrument: `store` is the artifact/config/draft apply-batch phase
/// machine, `page` a unit that produced a result page the host must acknowledge (the only unit that
/// is INHERENTLY one round trip), `retirement` the slot-release ladder, `worker` a mounted job step,
/// `latest_wins` the coalesced-command lane, and `idle` a unit that found nothing advanceable.
/// Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TypedOperationUnitCensus {
    pub units: u64,
    pub store: u64,
    pub page: u64,
    pub retirement: u64,
    pub worker: u64,
    pub latest_wins: u64,
    pub idle: u64,
}

impl std::fmt::Display for TypedOperationUnitCensus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "units={} store={} page={} retirement={} worker={} latestWins={} idle={}", self.units, self.store, self.page, self.retirement, self.worker, self.latest_wins, self.idle)
    }
}

impl std::ops::Sub for TypedOperationUnitCensus {
    type Output = Self;

    fn sub(self, base: Self) -> Self {
        Self {
            units: self.units.saturating_sub(base.units),
            store: self.store.saturating_sub(base.store),
            page: self.page.saturating_sub(base.page),
            retirement: self.retirement.saturating_sub(base.retirement),
            worker: self.worker.saturating_sub(base.worker),
            latest_wins: self.latest_wins.saturating_sub(base.latest_wins),
            idle: self.idle.saturating_sub(base.idle),
        }
    }
}

/// 📊️ One counted ladder of the publication census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypedOperationUnitKind {
    Store,
    Page,
    Retirement,
    Worker,
    LatestWins,
    Idle,
}

thread_local! {
    /// 🧵️ The census counters of the thread that drives publication units. Per thread, because the ladder runs on
    /// the host turn's own thread and a reading must attribute exactly that host's dispatches: a process-wide
    /// counter folds every concurrently driven instance (another document, another law of the same test
    /// binary) into the reading and turns a document-independent ladder into a load-dependent number.
    static PUBLICATION_UNITS: [std::cell::Cell<u64>; 7] = const { [const { std::cell::Cell::new(0) }; 7] };
}

/// 📊️ Counts one publication unit against its ladder on the calling thread.
pub fn record_typed_operation_unit(kind: TypedOperationUnitKind) {
    let index = match kind {
        TypedOperationUnitKind::Store => 1,
        TypedOperationUnitKind::Page => 2,
        TypedOperationUnitKind::Retirement => 3,
        TypedOperationUnitKind::Worker => 4,
        TypedOperationUnitKind::LatestWins => 5,
        TypedOperationUnitKind::Idle => 6,
    };
    PUBLICATION_UNITS.with(|units| {
        units[0].set(units[0].get().saturating_add(1));
        units[index].set(units[index].get().saturating_add(1));
    });
}

/// 📊️ The calling thread's census as of now. Subtract two readings to attribute one dispatch.
pub fn typed_operation_unit_census() -> TypedOperationUnitCensus {
    PUBLICATION_UNITS.with(|units| {
        let read = |index: usize| units[index].get();
        TypedOperationUnitCensus { units: read(0), store: read(1), page: read(2), retirement: read(3), worker: read(4), latest_wins: read(5), idle: read(6) }
    })
}

