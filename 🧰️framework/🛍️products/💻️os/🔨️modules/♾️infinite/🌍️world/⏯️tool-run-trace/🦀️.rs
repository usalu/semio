//! ⏯️ The wgpu world's tool run trace layer: a keyed record store per window fed by the base64url
//! `ToolRunTraceDelta` pages of `World3dScene.tool_run_trace`, batched per `(subject, verdict)` so a
//! million tested attempts cost one instanced draw per mesh and verdict. Mirrors the React
//! `🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx` store; both are pinned by
//! `🧰️framework/🔨️modules/⏯️tool-run/🧫️fixtures/📼️trace-pages.json`.
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §3.2 and §4.

use std::collections::{BTreeMap, HashMap};

use semio_framework_tool_run::{ToolRunCodecError, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_TRACE_PAGE_BYTES_MAX};
use ui_styling::metrics::tool_run;
use ui_wgpu::wgpu::{Instance3d, Rgba, Theme};

//#region 🔖️Limits
/// 📏️ Longest `World3dScene.tool_run_trace` text the render-plan validator admits: a delta of
/// sixteen full pages, base64url-expanded.
pub const TOOL_RUN_TRACE_LANE_BYTES_MAX: usize = (TOOL_RUN_TRACE_PAGE_BYTES_MAX * 16).div_ceil(3) * 4;
//#endregion 🔖️Limits

//#region 🔖️Store
/// 🧺️ The batch one record draws in: its subject family, the mesh or shape index, and its verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToolRunTraceBatchKey {
    pub family: ToolRunTraceFamily,
    pub index: u32,
    pub verdict: u8,
}

impl ToolRunTraceBatchKey {
    /// 🔎️ The batch a subject with `verdict` belongs to.
    pub fn of(subject: ToolRunTraceSubject, verdict: ToolRunVerdict) -> Self {
        let (family, index) = match subject {
            ToolRunTraceSubject::Instance3d { mesh, .. } => (ToolRunTraceFamily::Instance3d, mesh),
            ToolRunTraceSubject::Placement2d { shape, .. } => (ToolRunTraceFamily::Placement2d, shape),
            ToolRunTraceSubject::Entity { .. } => (ToolRunTraceFamily::Entity, 0),
        };
        Self { family, index, verdict: verdict.ordinal() }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        ToolRunVerdict::from_ordinal(self.verdict).unwrap_or(ToolRunVerdict::Testing)
    }
}

/// 👪️ Subject family of a batch, in draw order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ToolRunTraceFamily {
    Instance3d,
    Placement2d,
    Entity,
}

/// 📦️ One batch: parallel columns, swap-removed so upsert and retire stay O(1).
#[derive(Clone, Debug, Default)]
pub struct ToolRunTraceBatch {
    pub keys: Vec<u64>,
    pub subjects: Vec<ToolRunTraceSubject>,
    pub stamps: Vec<u64>,
}

#[derive(Clone, Copy, Debug)]
struct ToolRunTraceSlot {
    batch: ToolRunTraceBatchKey,
    index: usize,
}

/// 🚨️ Why a lane could not be applied; the layer keeps its previous records.
#[derive(Clone, Debug, PartialEq)]
pub enum ToolRunTraceLaneFault {
    Base64,
    Codec(ToolRunCodecError),
}

/// 📋️ What one lane application changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolRunTraceLayerApply {
    pub cleared: bool,
    pub pages: u32,
    pub ops: u32,
}

/// ⏯️ The per-window resident trace — see this module's header.
#[derive(Clone, Debug, Default)]
pub struct ToolRunTraceLayer {
    cursor: Option<ToolRunTraceCursor>,
    lane_digest: Option<u64>,
    records: HashMap<u64, ToolRunTraceSlot>,
    batches: BTreeMap<ToolRunTraceBatchKey, ToolRunTraceBatch>,
    next_stamp: u64,
    newest_testing: Option<(u64, u64)>,
}

impl ToolRunTraceLayer {
    /// 📥️ Applies one `tool_run_trace` lane text. `None` (no run tracing) keeps the resident records,
    /// and an unchanged text is skipped without decoding, so republishing a scene costs nothing.
    pub fn apply_lane(&mut self, lane: Option<&str>) -> Result<ToolRunTraceLayerApply, ToolRunTraceLaneFault> {
        let Some(lane) = lane else { return Ok(ToolRunTraceLayerApply::default()) };
        let digest = lane_digest(lane);
        if self.lane_digest == Some(digest) {
            return Ok(ToolRunTraceLayerApply::default());
        }
        let bytes = base64_codec::base64_url_decode(lane).map_err(|_| ToolRunTraceLaneFault::Base64)?;
        let delta = ToolRunTraceDelta::decode(&bytes).map_err(ToolRunTraceLaneFault::Codec)?;
        self.lane_digest = Some(digest);
        Ok(self.apply_delta(&delta))
    }

    /// 📬️ Applies one decoded delta. A `clear`, or a run/generation other than the resident one,
    /// drops every record first; pages below the expected cursor page were already applied and are
    /// skipped, so a redelivered delta is idempotent.
    pub fn apply_delta(&mut self, delta: &ToolRunTraceDelta) -> ToolRunTraceLayerApply {
        let run = delta.identity.id.run;
        let generation = delta.identity.generation;
        let rebound = self.cursor.is_none_or(|cursor| cursor.run != run || cursor.generation != generation);
        let mut outcome = ToolRunTraceLayerApply { cleared: delta.clear || rebound, ..Default::default() };
        if outcome.cleared {
            self.clear();
        }
        let expected = if outcome.cleared { 0 } else { self.cursor.map_or(0, |cursor| cursor.page) };
        for page in delta.pages.iter().filter(|page| outcome.cleared || page.page >= expected) {
            outcome.ops += self.apply_page(page);
            outcome.pages += 1;
        }
        let page = if outcome.cleared { delta.next } else { delta.next.max(expected) };
        self.cursor = Some(ToolRunTraceCursor { run, generation, page });
        outcome
    }

    fn apply_page(&mut self, page: &ToolRunTracePage) -> u32 {
        for op in &page.ops {
            match *op {
                ToolRunTraceOp::Clear => self.clear(),
                ToolRunTraceOp::Retire { key } => self.retire(key),
                ToolRunTraceOp::Upsert { key, verdict, subject, .. } => self.upsert(key, verdict, subject),
            }
        }
        page.ops.len() as u32
    }

    fn clear(&mut self) {
        self.records.clear();
        self.batches.clear();
        self.newest_testing = None;
    }

    fn upsert(&mut self, key: u64, verdict: ToolRunVerdict, subject: ToolRunTraceSubject) {
        self.retire(key);
        let stamp = self.next_stamp;
        self.next_stamp += 1;
        let batch_key = ToolRunTraceBatchKey::of(subject, verdict);
        let batch = self.batches.entry(batch_key).or_default();
        batch.keys.push(key);
        batch.subjects.push(subject);
        batch.stamps.push(stamp);
        self.records.insert(key, ToolRunTraceSlot { batch: batch_key, index: batch.keys.len() - 1 });
        if verdict == ToolRunVerdict::Testing {
            self.newest_testing = Some((key, stamp));
        }
    }

    fn retire(&mut self, key: u64) {
        let Some(slot) = self.records.remove(&key) else { return };
        let Some(batch) = self.batches.get_mut(&slot.batch) else { return };
        batch.keys.swap_remove(slot.index);
        batch.subjects.swap_remove(slot.index);
        batch.stamps.swap_remove(slot.index);
        if let Some(&moved) = batch.keys.get(slot.index) {
            if let Some(moved_slot) = self.records.get_mut(&moved) {
                moved_slot.index = slot.index;
            }
        }
        if batch.keys.is_empty() {
            self.batches.remove(&slot.batch);
        }
        if self.newest_testing.is_some_and(|(newest, _)| newest == key) {
            self.newest_testing = self
                .batches
                .iter()
                .filter(|(batch_key, _)| batch_key.verdict() == ToolRunVerdict::Testing)
                .flat_map(|(_, batch)| batch.keys.iter().copied().zip(batch.stamps.iter().copied()))
                .max_by_key(|(_, stamp)| *stamp);
        }
    }

    /// 🧭️ What the renderer echoes as `toolRunTraceCursor` in its window instance view state.
    pub fn cursor(&self) -> Option<ToolRunTraceCursor> {
        self.cursor
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// 🔢️ Resident records with `verdict`, over every subject family.
    pub fn count(&self, verdict: ToolRunVerdict) -> usize {
        self.batches.iter().filter(|(key, _)| key.verdict() == verdict).map(|(_, batch)| batch.keys.len()).sum()
    }

    pub fn contains(&self, key: u64) -> bool {
        self.records.contains_key(&key)
    }

    /// 🧺️ Every non-empty batch in draw order.
    pub fn batches(&self) -> impl Iterator<Item = (&ToolRunTraceBatchKey, &ToolRunTraceBatch)> {
        self.batches.iter()
    }

    /// 🔦️ The most recently upserted record still `testing` — the one drawn with the highlight.
    pub fn newest_testing(&self) -> Option<u64> {
        self.newest_testing.map(|(key, _)| key)
    }

    /// 🎨️ One instanced draw per `(mesh, verdict)` for every visible 3d batch — see [`ToolRunTraceDraw`].
    pub fn draws(&self, palette: &ToolRunTracePalette, visibility: ToolRunTraceVisibility) -> Vec<ToolRunTraceDraw> {
        let newest_stamp = self.next_stamp.saturating_sub(1);
        let newest_testing = self.newest_testing();
        self.batches
            .iter()
            .filter(|(key, _)| key.family == ToolRunTraceFamily::Instance3d && visibility.shows(key.verdict()))
            .map(|(key, batch)| {
                let verdict = key.verdict();
                let base = palette.color(verdict);
                let instances = batch
                    .keys
                    .iter()
                    .zip(&batch.subjects)
                    .zip(&batch.stamps)
                    .filter_map(|((record, subject), stamp)| {
                        let ToolRunTraceSubject::Instance3d { position, rotation, scale, .. } = *subject else { return None };
                        let newest = newest_testing == Some(*record);
                        let alpha = if newest { base.a } else { base.a * tool_run_trace_fade(newest_stamp - stamp) };
                        Some(Instance3d { id: format!("toolRunTrace:{record}"), model: Instance3d::model_from_trs(position, rotation, [scale; 3]), color: [base.r, base.g, base.b, alpha], selected: newest, hovered: false })
                    })
                    .collect();
                ToolRunTraceDraw { mesh: key.index, verdict, instances }
            })
            .collect()
    }
}

/// 🌫️ Age fade by sequence distance: 1 for the newest record, down to the `toolRun.fadeFloorOpacity`
/// token after `toolRun.fadeRecords` newer records.
pub fn tool_run_trace_fade(age: u64) -> f32 {
    let floor = tool_run::FADE_FLOOR_OPACITY as f32;
    (1.0 - age as f32 / tool_run::FADE_RECORDS as f32 * (1.0 - floor)).max(floor)
}

fn lane_digest(lane: &str) -> u64 {
    lane.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)) ^ lane.len() as u64
}
//#endregion 🔖️Store

//#region 🔖️Paint
/// 🎨️ Verdict paint from the theme's semantic outcome tokens: `testing` is the progress tone at the
/// `toolRun.testingOpacity` token, `success`/`warning`/`danger` are the theme's success, warning and
/// error tones. No literal colors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolRunTracePalette {
    pub testing: Rgba,
    pub success: Rgba,
    pub warning: Rgba,
    pub danger: Rgba,
}

impl ToolRunTracePalette {
    pub fn from_theme(theme: &Theme) -> Self {
        Self { testing: theme.progress.with_alpha(tool_run::TESTING_OPACITY as f32), success: theme.success, warning: theme.warning, danger: theme.error }
    }

    pub fn color(&self, verdict: ToolRunVerdict) -> Rgba {
        match verdict {
            ToolRunVerdict::Testing => self.testing,
            ToolRunVerdict::Success => self.success,
            ToolRunVerdict::Warning => self.warning,
            ToolRunVerdict::Danger => self.danger,
        }
    }
}

/// 👁️ The legend toggles a window config exposes (show testing / accepted / rejected).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolRunTraceVisibility {
    pub testing: bool,
    pub accepted: bool,
    pub rejected: bool,
}

impl Default for ToolRunTraceVisibility {
    fn default() -> Self {
        Self { testing: true, accepted: true, rejected: true }
    }
}

impl ToolRunTraceVisibility {
    pub fn shows(self, verdict: ToolRunVerdict) -> bool {
        match verdict {
            ToolRunVerdict::Testing => self.testing,
            ToolRunVerdict::Success => self.accepted,
            ToolRunVerdict::Warning | ToolRunVerdict::Danger => self.rejected,
        }
    }
}

/// 🖌️ One instanced draw: every visible record of one mesh index and verdict.
#[derive(Clone, Debug)]
pub struct ToolRunTraceDraw {
    pub mesh: u32,
    pub verdict: ToolRunVerdict,
    pub instances: Vec<Instance3d>,
}

/// 🟩️ The `provisional` style token for a document instance a running tool placed: the theme success
/// tone at the `toolRun.provisionalOpacity` token. wgpu paints it static (no dash animation), which is
/// also what `prefers-reduced-motion` asks of the React target.
pub fn tool_run_provisional_color(theme: &Theme) -> [f32; 4] {
    [theme.success.r, theme.success.g, theme.success.b, tool_run::PROVISIONAL_OPACITY as f32]
}
//#endregion 🔖️Paint

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
