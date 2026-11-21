//! 🗄️ Tool run trace: subjects, ops, pages, deltas, cursors and the resident trace store.

use super::*;
use semio_framework_value::numeric_scratch::NumericIndex;
use semio_framework_value_persistent::list::PagedList;

//#region 🔖️Trace
/// 👻️ What one trace record shows; meshes and shapes index the plugin's existing lanes.
#[derive(semio_framework_value::RetireOwned)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolRunTraceSubject {
    Instance3d { mesh: u32, position: [f32; 3], rotation: [f32; 4], scale: f32 },
    Placement2d { shape: u32, position: [f32; 2], rotation: f32 },
    Entity { entity: u64 },
}

impl ToolRunTraceSubject {
    pub fn ordinal(self) -> u8 {
        match self {
            Self::Instance3d { .. } => 0,
            Self::Placement2d { .. } => 1,
            Self::Entity { .. } => 2,
        }
    }

    pub(crate) fn wire_bytes(self) -> usize {
        match self {
            Self::Instance3d { .. } => 36,
            Self::Placement2d { .. } => 16,
            Self::Entity { .. } => 8,
        }
    }
}

/// ✏️ One trace delta operation.
#[derive(semio_framework_value::RetireOwned)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolRunTraceOp {
    Upsert { key: u64, verdict: ToolRunVerdict, reason: u16, subject: ToolRunTraceSubject },
    Retire { key: u64 },
    Clear,
}

impl ToolRunTraceOp {
    /// 📏️ Deterministic column-byte estimate used for page and delivery budgets.
    pub fn wire_bytes(self) -> usize {
        match self {
            Self::Upsert { subject, .. } => 13 + subject.wire_bytes(),
            Self::Retire { .. } => 9,
            Self::Clear => 1,
        }
    }
}

/// 📎️ Fixed per-page overhead of the delivery budget estimate.
pub const TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES: usize = 64;

/// 📃️ One columnar delta page.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct ToolRunTracePage {
    pub identity: ToolRunIdentity,
    pub page: u32,
    pub ops: Vec<ToolRunTraceOp>,
}

impl ToolRunTracePage {
    /// 📐️ Delivery budget estimate: overhead plus every op's column bytes.
    pub fn wire_bytes_estimate(ops: &[ToolRunTraceOp]) -> usize {
        TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES + ops.iter().map(|op| op.wire_bytes()).sum::<usize>()
    }

    /// 🎒️ Pack record body; rejects pages over the op or byte cap.
    pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError> {
        let bytes = encode_body(page_spec(), &page_record(self)?)?;
        if bytes.len() > TOOL_RUN_TRACE_PAGE_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("trace page bytes"));
        }
        Ok(bytes)
    }

    pub fn decode<S:semio_framework_value::ByteReadSource+?Sized>(bytes:&S) -> Result<Self, ToolRunCodecError> {
        if bytes.byte_len() > TOOL_RUN_TRACE_PAGE_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("trace page bytes"));
        }
        page_from_record(&decode_body(page_spec(), bytes)?)
    }
}

/// 📬️ One refresh of trace pages for a window; `clear` drops every record before `pages`.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunTraceDelta {
    pub identity: ToolRunIdentity,
    pub clear: bool,
    pub next: u32,
    pub pages: Vec<ToolRunTracePage>,
}

impl ToolRunTraceDelta {
    pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError> {
        let mut record = RecordValue::default();
        record.fields.insert(1, FieldValue::Record(identity_record(&self.identity)?));
        record.fields.insert(2, FieldValue::Bool(self.clear));
        record.fields.insert(3, FieldValue::UInt(u64::from(self.next)));
        if !self.pages.is_empty() {
            record.fields.insert(4, FieldValue::List(self.pages.iter().map(|page| page.encode().and_then(|bytes|record_octets(&bytes))).collect::<Result<_, _>>()?));
        }
        encode_body(delta_spec(), &record)
    }

    pub fn decode<S:semio_framework_value::ByteReadSource+?Sized>(bytes:&S) -> Result<Self, ToolRunCodecError> {
        let record = decode_body(delta_spec(), bytes)?;
        let pages = list(&record, 4)?.iter().map(|item| ToolRunTracePage::decode(&item_bytes(item)?)).collect::<Result<_, _>>()?;
        Ok(Self { identity: identity_from_field(&record, 1)?, clear: boolean(&record, 2)?, next: narrow(uint(&record, 3)?, "next")?, pages })
    }
}

/// 🧭️ Echoed by a renderer in its window instance view state; `page` is the next page it expects.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::NativeReceive)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunTraceCursor {
    pub generation: u32,
    pub page: u32,    pub run: u64,
}

/// 🗂️ One resident trace record.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct ToolRunTraceRecord {
    pub verdict: ToolRunVerdict,
    pub reason: u16,
    pub subject: ToolRunTraceSubject,
    stamp: u64,
}

/// 📋️ Outcome of applying ops to a `ToolRunTraceStore`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ToolRunTraceApply {
    pub logged: u32,
    pub evicted: u32,
    pub overflowed: u32,
    pub compacted: bool,
}

#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
enum TraceLogOps {
    Inline([ToolRunTraceOp; 2], usize),
    Group(Vec<ToolRunTraceOp>),
}

impl TraceLogOps {
    fn ops(&self) -> &[ToolRunTraceOp] {
        match self {
            Self::Inline(ops, n) => &ops[..*n],
            Self::Group(ops) => ops,
        }
    }
}

#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
struct ToolRunTraceLogPage {
    ops: TraceLogOps,
    bytes: usize,
}

impl ToolRunTraceLogPage {
    fn new(ops: TraceLogOps) -> Self {
        Self { bytes: ToolRunTracePage::wire_bytes_estimate(ops.ops()), ops }
    }
}

#[derive(Default, semio_framework_value::RetireOwned)]
struct TraceBacking {
    records: NumericIndex<u64, ToolRunTraceRecord>,
    stamps: NumericIndex<u64, u64>,
    rejected: NumericIndex<u64, u64>,
    log: PagedList<ToolRunTraceLogPage, {usize::MAX}>,
}

impl TraceBacking {
    fn records(&self) -> impl Iterator<Item = (u64, &ToolRunTraceRecord)> {
        (0..self.records.len()).map(|rank| {
            let (key, record) = self.records.entry_at_rank(rank).expect("resident trace rank");
            (*key, record)
        })
    }

    fn replace(&mut self, key: u64, record: ToolRunTraceRecord, reserved: bool) {
        self.remove(key);
        if reserved {
            assert!(self.records.insert_reserved(key, record).is_ok());
            assert!(self.stamps.insert_reserved(record.stamp, key).is_ok());
            if record.verdict.is_rejected() {
                assert!(self.rejected.insert_reserved(record.stamp, key).is_ok());
            }
        } else {
            self.records.insert(key, record);
            self.stamps.insert(record.stamp, key);
            if record.verdict.is_rejected() {
                self.rejected.insert(record.stamp, key);
            }
        }
    }

    fn remove(&mut self, key: u64) -> bool {
        let Some(prior) = self.records.remove(&key) else { return false };
        self.stamps.remove(&prior.stamp);
        if prior.verdict.is_rejected() {
            self.rejected.remove(&prior.stamp);
        }
        true
    }

    fn clear(&mut self) {
        self.records.reset();
        self.stamps.reset();
        self.rejected.reset();
    }

    fn oldest_rejected(&self) -> Option<u64> {
        self.rejected.entry_at_rank(0).map(|(_, key)| *key)
    }
}

impl Clone for TraceBacking {
    fn clone(&self) -> Self {
        let mut copy = Self { log: self.log.clone(), ..Self::default() };
        for (key, record) in self.records() {
            copy.replace(key, *record, false);
        }
        copy
    }
}

/// 🗃️ Resident trace records and the immutable publication order of window deltas.
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunTraceStore {
    pub(crate) identity: ToolRunIdentity,
    capacity: usize,
    compact_floor: usize,
    data: TraceBacking,
    next_stamp: u64,
    pub(crate) log_base: u32,
    pub(crate) next_page: u32,
    logged_ops: usize,
    compaction: Vec<funded::Compaction>,
    retiring: bool,
}

impl std::fmt::Debug for ToolRunTraceStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolRunTraceStore")
            .field("identity", &self.identity)
            .field("records", &self.len())
            .field("log_base", &self.log_base)
            .field("next_page", &self.next_page)
            .finish()
    }
}

impl Clone for ToolRunTraceStore {
    fn clone(&self) -> Self {
        Self {
            identity: self.identity, capacity: self.capacity, compact_floor: self.compact_floor,
            data: self.data.clone(), next_stamp: self.next_stamp, log_base: self.log_base,
            next_page: self.next_page, logged_ops: self.logged_ops, compaction: Vec::new(), retiring: false,
        }
    }
}

impl ToolRunTraceStore {
    pub fn new(identity: ToolRunIdentity) -> Self {
        Self::with_limits(identity, TOOL_RUN_TRACE_RESIDENT_RECORDS, TOOL_RUN_TRACE_LOG_COMPACT_FLOOR)
    }

    /// 🎛️ Store with an explicit residency capacity and log compaction floor.
    pub fn with_limits(identity: ToolRunIdentity, capacity: usize, compact_floor: usize) -> Self {
        Self {
            identity, capacity: capacity.max(1), compact_floor: compact_floor.max(1),
            data: Default::default(), next_stamp: 0, log_base: 0, next_page: 0, logged_ops: 0,
            compaction: Vec::new(), retiring: false,
        }
    }

    pub fn identity(&self) -> ToolRunIdentity { self.identity }

    /// 🔗️ Adopts a new generation (records kept) or a new run (everything dropped).
    pub fn rebind(&mut self, identity: ToolRunIdentity) {
        if identity.id != self.identity.id {
            *self = Self::with_limits(identity, self.capacity, self.compact_floor);
        }
        self.identity = identity;
    }

    pub fn len(&self) -> usize { self.data.records.len() }

    pub fn is_empty(&self) -> bool { self.data.records.is_empty() }

    pub fn record(&self, key: u64) -> Option<&ToolRunTraceRecord> { self.data.records.get(&key) }

    pub fn records(&self) -> impl Iterator<Item = (u64, &ToolRunTraceRecord)> { self.data.records() }

    pub fn log_base(&self) -> u32 { self.log_base }

    pub fn next_page(&self) -> u32 { self.next_page }

    /// 📖️ Ops of logged page, if still retained.
    pub fn log_page(&self, page: u32) -> Option<&[ToolRunTraceOp]> {
        let index = page.checked_sub(self.log_base)?;
        self.data.log.get(usize::try_from(index).ok()?).map(|logged| logged.ops.ops())
    }

    /// 🛂️ Applies a job page in a cold call. Pages of another run or generation are stale.
    pub fn apply_page(&mut self, page: &ToolRunTracePage) -> Result<ToolRunTraceApply, ToolRunRejection> {
        if page.identity.id != self.identity.id || page.identity.generation != self.identity.generation {
            return Err(ToolRunRejection::Stale);
        }
        Ok(self.apply_ops(&page.ops))
    }

    /// 🧺️ Applies ops with residency eviction and logs what renderers must replay.
    pub fn apply_ops(&mut self, ops: &[ToolRunTraceOp]) -> ToolRunTraceApply {
        assert!(self.compaction.is_empty(), "cold trace input during original apply");
        let mut outcome = ToolRunTraceApply::default();
        let mut logged = Vec::with_capacity(ops.len());
        for &op in ops {
            let result = self.apply_one(op, false);
            outcome.evicted += result.evicted;
            outcome.overflowed += result.overflowed;
            logged.extend_from_slice(&result.ops[..result.count]);
        }
        outcome.logged = self.log_ops(&logged);
        if self.logged_ops > 2 * self.len().max(self.compact_floor) {
            self.compact();
            outcome.compacted = true;
        }
        outcome
    }

    /// 📮️ Pages after the window cursor within the byte budget.
    pub fn delta_after(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> ToolRunTraceDelta {
        let resend = cursor.is_none_or(|cursor| cursor.run != self.identity.id.run || cursor.generation != self.identity.generation || cursor.page < self.log_base || cursor.page > self.next_page);
        let start = if resend { self.log_base } else { cursor.map_or(self.log_base, |cursor| cursor.page) };
        let mut pages = Vec::new();
        let mut bytes = 0usize;
        for offset in (start - self.log_base) as usize..self.data.log.len() {
            let logged = self.data.log.get(offset).expect("retained trace page");
            if !pages.is_empty() && bytes + logged.bytes > byte_budget {
                break;
            }
            bytes += logged.bytes;
            pages.push(ToolRunTracePage { identity: self.identity, page: self.log_base + offset as u32, ops: logged.ops.ops().to_vec() });
        }
        ToolRunTraceDelta { identity: self.identity, clear: resend, next: start + pages.len() as u32, pages }
    }

    fn log_ops(&mut self, ops: &[ToolRunTraceOp]) -> u32 {
        let mut pages = 0;
        for chunk in ops.chunks(TOOL_RUN_TRACE_PAGE_OPS_MAX) {
            self.logged_ops += chunk.len();
            self.data.log.push(ToolRunTraceLogPage::new(TraceLogOps::Group(chunk.to_vec())));
            self.next_page += 1;
            pages += 1;
        }
        pages
    }

    fn compact(&mut self) {
        let snapshot: Vec<_> = std::iter::once(ToolRunTraceOp::Clear).chain(
            (0..self.len()).map(|rank| {
                let (_, key) = self.data.stamps.entry_at_rank(rank).expect("original trace order");
                self.snapshot_op(*key)
            }),
        ).collect();
        self.data.log = Default::default();
        self.logged_ops = 0;
        self.log_base = self.next_page;
        self.log_ops(&snapshot);
    }

    fn snapshot_op(&self, key: u64) -> ToolRunTraceOp {
        let record = self.record(key).expect("stamped trace record");
        ToolRunTraceOp::Upsert { key, verdict: record.verdict, reason: record.reason, subject: record.subject }
    }

    fn apply_one(&mut self, op: ToolRunTraceOp, reserved: bool) -> TraceChange {
        let mut result = TraceChange { ops: [op, op], count: 1, evicted: 0, overflowed: 0 };
        match op {
            ToolRunTraceOp::Clear => self.data.clear(),
            ToolRunTraceOp::Retire { key } => result.count = usize::from(self.data.remove(key)),
            ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                if self.len() >= self.capacity && self.record(key).is_none() {
                    if let Some(victim) = self.data.oldest_rejected() {
                        self.data.remove(victim);
                        result.ops[0] = ToolRunTraceOp::Retire { key: victim };
                        result.count = 2;
                        result.evicted = 1;
                    } else {
                        result.count = 0;
                        result.evicted = u32::from(verdict.is_rejected());
                        result.overflowed = u32::from(!verdict.is_rejected());
                        return result;
                    }
                }
                let stamp = self.next_stamp;
                self.next_stamp += 1;
                self.data.replace(key, ToolRunTraceRecord { verdict, reason, subject, stamp }, reserved);
            }
        }
        result
    }
}

struct TraceChange {
    ops: [ToolRunTraceOp; 2],
    count: usize,
    evicted: u32,
    overflowed: u32,
}

#[path = "🎟️funded/🦀️.rs"]
mod funded;
pub use funded::{ToolRunTraceApplyCursor, ToolRunTraceFundedStep};
//#endregion 🔖️Trace
