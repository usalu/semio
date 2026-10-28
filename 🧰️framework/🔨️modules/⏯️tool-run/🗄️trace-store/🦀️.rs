//! 🗄️ Tool run trace: subjects, ops, pages, deltas, cursors and the resident trace store.

use super::*;

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
struct ToolRunTraceLogPage {
    pub(crate) ops: Vec<ToolRunTraceOp>,
    pub(crate) bytes: usize,
}

/// 🗃️ Resident trace records plus the page log windows read through cursors (§3.2).
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct ToolRunTraceStore {
    pub(crate) identity: ToolRunIdentity,
    pub(crate) capacity: usize,
    compact_floor: usize,
    pub(crate) records: HashMap<u64, ToolRunTraceRecord>,
    pub(crate) rejected: VecDeque<(u64, u64)>,
    next_stamp: u64,
    log: VecDeque<ToolRunTraceLogPage>,
    pub(crate) log_base: u32,
    pub(crate) next_page: u32,
    logged_ops: usize,
}

impl ToolRunTraceStore {
    pub fn new(identity: ToolRunIdentity) -> Self {
        Self::with_limits(identity, TOOL_RUN_TRACE_RESIDENT_RECORDS, TOOL_RUN_TRACE_LOG_COMPACT_FLOOR)
    }

    /// 🎛️ Store with an explicit residency capacity and log compaction floor.
    pub fn with_limits(identity: ToolRunIdentity, capacity: usize, compact_floor: usize) -> Self {
        Self { identity, capacity: capacity.max(1), compact_floor: compact_floor.max(1), records: HashMap::new(), rejected: VecDeque::new(), next_stamp: 0, log: VecDeque::new(), log_base: 0, next_page: 0, logged_ops: 0 }
    }

    pub fn identity(&self) -> ToolRunIdentity {
        self.identity
    }
    /// 🔗️ Adopts a new generation (records kept) or a new run (everything dropped).
    pub fn rebind(&mut self, identity: ToolRunIdentity) {
        if identity.id != self.identity.id {
            *self = Self::with_limits(identity, self.capacity, self.compact_floor);
        }
        self.identity = identity;
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn record(&self, key: u64) -> Option<&ToolRunTraceRecord> {
        self.records.get(&key)
    }

    pub fn records(&self) -> impl Iterator<Item = (u64, &ToolRunTraceRecord)> {
        self.records.iter().map(|(key, record)| (*key, record))
    }

    pub fn log_base(&self) -> u32 {
        self.log_base
    }

    pub fn next_page(&self) -> u32 {
        self.next_page
    }

    /// 📖️ Ops of logged page `page`, if still retained.
    pub fn log_page(&self, page: u32) -> Option<&[ToolRunTraceOp]> {
        let index = page.checked_sub(self.log_base)?;
        self.log.get(usize::try_from(index).ok()?).map(|logged| logged.ops.as_slice())
    }

    /// 🛂️ Applies a job page; pages of another run or generation are stale.
    pub fn apply_page(&mut self, page: &ToolRunTracePage) -> Result<ToolRunTraceApply, ToolRunRejection> {
        if page.identity.id != self.identity.id || page.identity.generation != self.identity.generation {
            return Err(ToolRunRejection::Stale);
        }
        Ok(self.apply_ops(&page.ops))
    }

    /// 🧺️ Applies ops with residency eviction and logs what renderers must replay.
    pub fn apply_ops(&mut self, ops: &[ToolRunTraceOp]) -> ToolRunTraceApply {
        let mut outcome = ToolRunTraceApply::default();
        let mut logged = Vec::with_capacity(ops.len());
        for op in ops {
            match *op {
                ToolRunTraceOp::Clear => {
                    self.records.clear();
                    self.rejected.clear();
                    logged.push(ToolRunTraceOp::Clear);
                }
                ToolRunTraceOp::Retire { key } => {
                    if self.records.remove(&key).is_some() {
                        logged.push(*op);
                    }
                }
                ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                    if !self.records.contains_key(&key) && self.records.len() >= self.capacity {
                        if let Some(victim) = self.evict_oldest_rejected() {
                            logged.push(ToolRunTraceOp::Retire { key: victim });
                            outcome.evicted += 1;
                        } else if verdict.is_rejected() {
                            outcome.evicted += 1;
                            continue;
                        } else {
                            outcome.overflowed += 1;
                            continue;
                        }
                    }
                    let stamp = self.next_stamp;
                    self.next_stamp += 1;
                    self.records.insert(key, ToolRunTraceRecord { verdict, reason, subject, stamp });
                    if verdict.is_rejected() {
                        self.rejected.push_back((key, stamp));
                    }
                    logged.push(*op);
                }
            }
        }
        outcome.logged = self.log_ops(&logged);
        if self.rejected.len() > 2 * self.records.len() + TOOL_RUN_STEP_RING_CAPACITY {
            self.rebuild_rejected();
        }
        if self.logged_ops > 2 * self.records.len().max(self.compact_floor) {
            self.compact();
            outcome.compacted = true;
        }
        outcome
    }

    /// 📮️ Pages after `cursor` within `byte_budget` (always at least one pending page).
    pub fn delta_after(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> ToolRunTraceDelta {
        let resend = cursor.is_none_or(|cursor| cursor.run != self.identity.id.run || cursor.generation != self.identity.generation || cursor.page < self.log_base || cursor.page > self.next_page);
        let start = if resend { self.log_base } else { cursor.map_or(self.log_base, |cursor| cursor.page) };
        let mut pages = Vec::new();
        let mut bytes = 0usize;
        for (offset, logged) in self.log.iter().enumerate().skip((start - self.log_base) as usize) {
            if !pages.is_empty() && bytes + logged.bytes > byte_budget {
                break;
            }
            bytes += logged.bytes;
            pages.push(ToolRunTracePage { identity: self.identity, page: self.log_base + offset as u32, ops: logged.ops.clone() });
        }
        ToolRunTraceDelta { identity: self.identity, clear: resend, next: start + pages.len() as u32, pages }
    }

    fn log_ops(&mut self, ops: &[ToolRunTraceOp]) -> u32 {
        let mut pages = 0;
        for chunk in ops.chunks(TOOL_RUN_TRACE_PAGE_OPS_MAX) {
            self.logged_ops += chunk.len();
            self.log.push_back(ToolRunTraceLogPage { bytes: ToolRunTracePage::wire_bytes_estimate(chunk), ops: chunk.to_vec() });
            self.next_page += 1;
            pages += 1;
        }
        pages
    }

    fn evict_oldest_rejected(&mut self) -> Option<u64> {
        while let Some((key, stamp)) = self.rejected.pop_front() {
            if self.records.get(&key).is_some_and(|record| record.stamp == stamp && record.verdict.is_rejected()) {
                self.records.remove(&key);
                return Some(key);
            }
        }
        None
    }

    fn live_by_stamp(&self) -> Vec<(u64, ToolRunTraceRecord)> {
        let mut live: Vec<(u64, ToolRunTraceRecord)> = self.records.iter().map(|(key, record)| (*key, *record)).collect();
        live.sort_unstable_by_key(|(_, record)| record.stamp);
        live
    }

    fn rebuild_rejected(&mut self) {
        self.rejected = self.live_by_stamp().into_iter().filter(|(_, record)| record.verdict.is_rejected()).map(|(key, record)| (key, record.stamp)).collect();
    }

    fn compact(&mut self) {
        let live = self.live_by_stamp();
        let snapshot: Vec<ToolRunTraceOp> = std::iter::once(ToolRunTraceOp::Clear).chain(live.iter().map(|(key, record)| ToolRunTraceOp::Upsert { key: *key, verdict: record.verdict, reason: record.reason, subject: record.subject })).collect();
        self.rejected = live.iter().filter(|(_, record)| record.verdict.is_rejected()).map(|(key, record)| (*key, record.stamp)).collect();
        self.log.clear();
        self.logged_ops = 0;
        self.log_base = self.next_page;
        self.log_ops(&snapshot);
    }
}
//#endregion 🔖️Trace
