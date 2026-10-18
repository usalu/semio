//! 📊️ Census: the executor census counters and `DbIoCensusV1`. See [`super`].

use super::*;

//#region 🔖️Limits
//#region 🔖️DbIoCensus
pub static DB_IO_CENSUS_TASKS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

pub static DB_IO_CENSUS_STEPS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

pub static DB_IO_CENSUS_TURNS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

static DB_IO_CENSUS_ADMISSION_WAITS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

static DB_IO_CENSUS_ADMISSION_WAIT_MICROS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

static DB_IO_CENSUS_ADMISSION_REFUSALS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

/// ⏳️ Counts one admission wait of a `kind` task that lasted `waited` and whether it ended in the refusal.
pub fn db_io_census_admission_wait(kind: DbIoTaskKind, waited: std::time::Duration, refused: bool) {
    DB_IO_CENSUS_ADMISSION_WAITS[kind as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    DB_IO_CENSUS_ADMISSION_WAIT_MICROS[kind as usize].fetch_add(u64::try_from(waited.as_micros()).unwrap_or(u64::MAX), std::sync::atomic::Ordering::Relaxed);
    if refused {
        DB_IO_CENSUS_ADMISSION_REFUSALS[kind as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

/// 🧮️ A reading of the process-wide DB I/O census: typed tasks submitted, executor steps
/// driven and worker-pool turns spent per [`DbIoTaskKind`] since the process started, plus the
/// admission waits of [`submit_db_io_task_admitted`] (backend opens count as `BackendOpen`): waits
/// begun, microseconds waited, and waits that ended in the refusal. Two readings subtract to the exact
/// I/O an interval cost, which is what the throughput laws bound (independent of machine load).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbIoCensusV1 {
    pub tasks: [u64; DbIoTaskKind::ALL.len()],
    pub steps: [u64; DbIoTaskKind::ALL.len()],
    pub turns: [u64; DbIoTaskKind::ALL.len()],
    admission_waits: [u64; DbIoTaskKind::ALL.len()],
    admission_wait_micros: [u64; DbIoTaskKind::ALL.len()],
    admission_refusals: [u64; DbIoTaskKind::ALL.len()],
}

impl DbIoCensusV1 {
    pub fn tasks(&self, kind: DbIoTaskKind) -> u64 {
        self.tasks[kind as usize]
    }

    pub fn steps(&self, kind: DbIoTaskKind) -> u64 {
        self.steps[kind as usize]
    }

    pub fn turns(&self, kind: DbIoTaskKind) -> u64 {
        self.turns[kind as usize]
    }

    pub fn total_tasks(&self) -> u64 {
        self.tasks.iter().sum()
    }

    pub fn total_steps(&self) -> u64 {
        self.steps.iter().sum()
    }

    pub fn total_turns(&self) -> u64 {
        self.turns.iter().sum()
    }

    pub fn admission_waits(&self, kind: DbIoTaskKind) -> u64 {
        self.admission_waits[kind as usize]
    }

    pub fn admission_wait_micros(&self, kind: DbIoTaskKind) -> u64 {
        self.admission_wait_micros[kind as usize]
    }

    pub fn admission_refusals(&self, kind: DbIoTaskKind) -> u64 {
        self.admission_refusals[kind as usize]
    }

    pub fn total_admission_waits(&self) -> u64 {
        self.admission_waits.iter().sum()
    }

    pub fn total_admission_wait_micros(&self) -> u64 {
        self.admission_wait_micros.iter().fold(0, |total, micros| total.saturating_add(*micros))
    }

    pub fn total_admission_refusals(&self) -> u64 {
        self.admission_refusals.iter().sum()
    }

    /// ➖️ What happened between `earlier` and this reading.
    pub fn since(&self, earlier: &DbIoCensusV1) -> DbIoCensusV1 {
        DbIoCensusV1 {
            tasks: std::array::from_fn(|kind| self.tasks[kind].saturating_sub(earlier.tasks[kind])),
            steps: std::array::from_fn(|kind| self.steps[kind].saturating_sub(earlier.steps[kind])),
            turns: std::array::from_fn(|kind| self.turns[kind].saturating_sub(earlier.turns[kind])),
            admission_waits: std::array::from_fn(|kind| self.admission_waits[kind].saturating_sub(earlier.admission_waits[kind])),
            admission_wait_micros: std::array::from_fn(|kind| self.admission_wait_micros[kind].saturating_sub(earlier.admission_wait_micros[kind])),
            admission_refusals: std::array::from_fn(|kind| self.admission_refusals[kind].saturating_sub(earlier.admission_refusals[kind])),
        }
    }

    /// 🧾️ `kind=tasks/steps/turns` for every kind that saw I/O, followed by
    /// `waits/µs/refusals` for every kind that waited for admission, for law output and captures.
    pub fn describe(&self) -> String {
        DbIoTaskKind::ALL
            .iter()
            .filter(|kind| self.tasks(**kind) != 0 || self.steps(**kind) != 0 || self.admission_waits(**kind) != 0)
            .map(|kind| match self.admission_waits(*kind) {
                0 => format!("{kind:?}={}/{}/{}", self.tasks(*kind), self.steps(*kind), self.turns(*kind)),
                waits => format!("{kind:?}={}/{}/{} admission={waits}/{}us/{}", self.tasks(*kind), self.steps(*kind), self.turns(*kind), self.admission_wait_micros(*kind), self.admission_refusals(*kind)),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// 🧮️ Reads the process-wide DB I/O census.
pub fn db_io_census() -> DbIoCensusV1 {
    DbIoCensusV1 {
        tasks: std::array::from_fn(|kind| DB_IO_CENSUS_TASKS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        steps: std::array::from_fn(|kind| DB_IO_CENSUS_STEPS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        turns: std::array::from_fn(|kind| DB_IO_CENSUS_TURNS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        admission_waits: std::array::from_fn(|kind| DB_IO_CENSUS_ADMISSION_WAITS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        admission_wait_micros: std::array::from_fn(|kind| DB_IO_CENSUS_ADMISSION_WAIT_MICROS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        admission_refusals: std::array::from_fn(|kind| DB_IO_CENSUS_ADMISSION_REFUSALS[kind].load(std::sync::atomic::Ordering::Relaxed)),
    }
}
//#endregion 🔖️DbIoCensus
//#endregion 🔖️Limits
