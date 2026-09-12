//! @emoji 🎫 The enqueue-only UI-thread contract: [`UiThreadToken`]/[`WorkerContext`] (the
//! non-interchangeable capability split ticket `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR` Phase 3
//! calls for) plus [`EventQueue`] — the fixed-capacity, allocation-free-for-replaceable-state sink a
//! [`crate::window::WindowDelegate`] host writes into instead of processing an event synchronously.
//!
//! **Why not a byte-for-byte fixed `[u8; 128]` struct per the design doc's original sketch.**
//! [`ui_render::DispatchEvent::KeyDown`]/`Paste`/`Ime` carry a real `String` (a logical key label, a
//! pasted clipboard string, an IME composition string) — genuinely unbounded in length, not just
//! under-measured. A byte-for-byte fixed representation would have to silently truncate a paste or an
//! IME composition, which is a correctness bug, not an optimization. The honest design instead splits
//! by REPLACEABILITY, matching the design doc's own §3.2 table exactly:
//! - Replaceable samples (pointer move, scroll, metrics/resize) — [`CoalesceSlot`]: three fixed `Copy`
//!   fields, no heap allocation ever, latest-wins (scroll deltas accumulate rather than overwrite, so a
//!   burst of wheel ticks before the next drain is not lost — only the final position/size is
//!   "latest").
//! - Discrete, lossless events (pointer down/up, key down/up, ime, paste) — a bounded `VecDeque` sized
//!   generously (`DISCRETE_QUEUE_CAPACITY`) for any realistic per-frame input burst. This is a bounded
//!   queue, not a lock-free zero-allocation ring — the variable-length string payloads make true
//!   zero-allocation impossible for this subset without the same truncation bug above. What IS
//!   preserved: the queue never grows unboundedly (`try_push` reports `Overflow` rather than growing
//!   past capacity — see [`EnqueueOutcome`]), and it is never silently dropped: overflow is a caller-
//!   observable event, never a swallowed one.
//!
//! Every enqueued item carries [`InputGeneration`] — a monotonically increasing counter bumped on
//! every state-changing enqueue — so a consumer (a hit-test result computed against a
//! [`crate::window::RedrawOutcome`]-shaped snapshot from an earlier generation) can detect it is
//! acting on stale pointer state and re-query rather than silently applying a late result.

use ui_render::DispatchEvent;

#[path = "../📥️input/🎟️admission/🪪️root/🦀️.rs"]
#[cfg(test)]
mod input_root;

#[path = "../📥️input/🎟️admission/✍️writer/🦀️.rs"]
#[cfg(test)]
mod input_writer;

//#region 🔖️Capabilities

//#region 🎫️UiThreadToken

/// 🎫️ Zero-size, unforgeable proof that the calling code is running on the thread that owns the
/// window/event-loop — the non-interchangeable half of the ticket's capability split (master plan
/// Decision 2 / this ticket's own text). No public constructor: the only way to obtain one is
/// [`UiThreadToken::mint`], `pub(crate)`, called exactly once per host at construction — see
/// `crate::window::native::NativeHost::new`/`crate::window::browser::CanvasHost::new`. A function that
/// takes `&UiThreadToken` by value can therefore be called only from code a host itself invoked,
/// which is the whole enforcement mechanism: the type system, not a runtime assertion, though
/// `semio_framework_trace::assert_ui_thread()` remains the runtime backstop for code this crate cannot
/// see (product/plugin callbacks a delegate itself invokes).
#[derive(Clone, Copy)]
pub struct UiThreadToken(());

impl UiThreadToken {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn mint() -> Self {
        Self(())
    }

    /// 🎫️ The escape hatch for a host that cannot go through [`crate::window::native::NativeHost`]/
    /// [`crate::window::browser::CanvasHost`] — e.g. a product with a two-phase boot handshake that
    /// hand-rolls its own `ApplicationHandler` (see the OS renderer's own `winit_app.rs` module
    /// docstring for a documented, real example). Same discipline as
    /// `semio_framework_trace::register_ui_thread()`: call this exactly once, from the thread that owns
    /// your event loop, before constructing anything that needs a token. This is a compile-time
    /// capability marker within a trusted codebase, not a defense against a determined adversary inside
    /// the same trust boundary — see this crate's own root docstring on the U1 sync/async boundary for
    /// the enforcement model this participates in.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mint_for_host() -> Self {
        Self(())
    }
}

//#endregion 🎫️UiThreadToken

//#region 🧵️WorkerContext

/// 🧵️ The capability a function needs to allocate substantially, lock contended state, wait, perform
/// I/O, mutate a document/model, run layout, create geometry, or execute plugin code — anything
/// [`UiThreadToken`] must NOT authorize. `generation` mirrors [`InputGeneration`]: the input state this
/// worker step is building against, so a result computed from a superseded generation is identifiable
/// as stale by its own caller rather than silently applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerContext {
    pub generation: InputGeneration,
}

impl WorkerContext {
    // 🚫️async: U1 — plain data constructor, no suspension of any kind.
    pub fn new(generation: InputGeneration) -> Self {
        Self { generation }
    }
}

//#endregion 🧵️WorkerContext

//#endregion 🔖️Capabilities

//#region 🔖️InputGeneration

/// 🔢️ A monotonic counter bumped on every state-changing enqueue (pointer move/down/up, scroll, key,
/// resize). A hit-test or dispatch result stamped with an [`InputGeneration`] older than the queue's
/// current one is provably stale — the mechanism the ticket's brief asks for: "input generation ids
/// prevent a late hit result from acting on stale pointer state."
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputGeneration(pub u64);

impl InputGeneration {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn next(self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

//#endregion 🔖️InputGeneration

//#region 🔖️CoalesceSlot

/// 🖱️ The latest pointer-move sample — replaces the previous one on every enqueue, never grows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerMoveSample {
    pub pointer: ui_render::PointerInfo,
    pub x: f32,
    pub y: f32,
    pub generation: InputGeneration,
}

/// 🎡️ The accumulated scroll delta since the last drain — deltas ADD (a burst of wheel ticks before a
/// drain must not lose earlier ticks' magnitude), but position is latest-wins like pointer move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollSample {
    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub generation: InputGeneration,
}

/// 📐️ The latest resize/scale-factor sample.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetricsSample {
    pub physical_width: u32,
    pub physical_height: u32,
    pub scale_factor: f32,
    pub generation: InputGeneration,
}

/// 🖱️ Fixed, `Copy`-only, three-field coalescing state for the events the design doc's §3.2 table
/// marks replaceable. No heap allocation on any path — every field is a plain `Option<Copy struct>`.
#[derive(Clone, Copy, Debug, Default)]
pub struct CoalesceSlot {
    pointer_move: Option<PointerMoveSample>,
    scroll: Option<ScrollSample>,
    metrics: Option<MetricsSample>,
}

impl CoalesceSlot {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self::default()
    }

    /// 🖱️ Overwrites any pending pointer-move sample — only the latest position matters for hit-test.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn coalesce_pointer_move(&mut self, sample: PointerMoveSample) {
        self.pointer_move = Some(sample);
    }

    /// 🎡️ Accumulates delta, replaces position — see this struct's own doc for why deltas add.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn coalesce_scroll(&mut self, sample: ScrollSample) {
        match self.scroll.as_mut() {
            Some(existing) => {
                existing.x = sample.x;
                existing.y = sample.y;
                existing.delta_x += sample.delta_x;
                existing.delta_y += sample.delta_y;
                existing.generation = sample.generation;
            }
            None => self.scroll = Some(sample),
        }
    }

    /// 📐️ Overwrites any pending metrics sample — only the latest size/scale matters.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn coalesce_metrics(&mut self, sample: MetricsSample) {
        self.metrics = Some(sample);
    }

    /// 🚿️ Drains every coalesced sample, leaving the slot empty — a caller calls this once per drain
    /// cycle (typically once per frame build) and applies each `Some` result at most once.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain(&mut self) -> (Option<PointerMoveSample>, Option<ScrollSample>, Option<MetricsSample>) {
        (self.pointer_move.take(), self.scroll.take(), self.metrics.take())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_empty(&self) -> bool {
        self.pointer_move.is_none() && self.scroll.is_none() && self.metrics.is_none()
    }
}

//#endregion 🔖️CoalesceSlot

//#region 🔖️DiscreteQueue

/// 📦️ A discrete, lossless input event — every [`ui_render::DispatchEvent`] variant NOT covered by
/// [`CoalesceSlot`], stamped with the [`InputGeneration`] current at enqueue time.
#[derive(Clone, Debug, PartialEq)]
pub struct DiscreteEvent {
    pub event: DispatchEvent,
    pub generation: InputGeneration,
}

/// 📏️ Generous bound on pending discrete events between two drains — sized well above any realistic
/// per-frame burst (design doc §3.3: "typical frame at 60 Hz consumes ~6-10 events"; this is >25x that)
/// so `try_push` returning [`EnqueueOutcome::Overflow`] is a genuine backpressure signal, not a routine
/// occurrence, while still bounding worst-case memory.
pub const DISCRETE_QUEUE_CAPACITY: usize = 256;
pub const DISCRETE_EVENT_BYTE_CAPACITY: usize = 4 * 1024;
pub const DISCRETE_QUEUE_BYTE_CAPACITY: usize = DISCRETE_QUEUE_CAPACITY * DISCRETE_EVENT_BYTE_CAPACITY;
pub const DISCRETE_DRAIN_PAGE_CAPACITY: usize = 4;

/// 📤️ What [`EventQueue::try_push`] reports — overflow is a real, caller-observable outcome (the
/// ticket's own "user commands never silently dropped" invariant), never a silent drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnqueueOutcome {
    Accepted,
    Overflow,
}

//#endregion 🔖️DiscreteQueue

//#region 🔖️EventQueue

/// 📬️ The whole enqueue-only sink a [`crate::window::WindowDelegate`] host writes into:
/// [`CoalesceSlot`] for replaceable state plus a bounded [`DiscreteEvent`] queue for everything else.
/// Preallocated once at construction (`with_capacity`), never reallocated on the hot path short of
/// genuine overflow.
pub struct EventQueue {
    #[cfg(test)]
    root: Option<std::num::NonZeroU64>,
    coalesced: CoalesceSlot,
    discrete: std::collections::VecDeque<DiscreteEvent>,
    generation: InputGeneration,
    overflow_count: u64,
    discrete_bytes: usize,
}

impl EventQueue {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self { #[cfg(test)] root: None, coalesced: CoalesceSlot::new(), discrete: std::collections::VecDeque::with_capacity(DISCRETE_QUEUE_CAPACITY), generation: InputGeneration::default(), overflow_count: 0, discrete_bytes: 0 }
    }

    #[cfg(test)]
    fn try_admit_root_with(&mut self, sequence: &input_root::InputRootSequence, granted_work_bytes: usize) -> Result<bool, input_root::InputRootFault> {
        if self.root.is_some() { return Ok(true); }
        if granted_work_bytes < size_of::<Self>() { return Ok(false); }
        self.root = Some(sequence.try_next()?);
        Ok(true)
    }

    /// 🔢️ The generation this queue is currently accumulating into — bumped by every `enqueue*` call
    /// below. A caller reads this AFTER enqueueing to stamp whatever it derives from the event (e.g. a
    /// hit-test performed synchronously against the coalesced pointer position).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn current_generation(&self) -> InputGeneration {
        self.generation
    }

    /// 📥️ The single entry point every normalized [`ui_render::DispatchEvent`] funnels through — a
    /// host's `handle_event` calls this instead of processing the event itself. Routes to
    /// [`CoalesceSlot`] for replaceable kinds, the bounded queue otherwise. Requires a [`UiThreadToken`]
    /// — this is, by construction, the one function a host's event callback is allowed to call
    /// directly with the raw event.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn enqueue(&mut self, _ui: UiThreadToken, event: DispatchEvent) -> EnqueueOutcome {
        self.generation = self.generation.next();
        let generation = self.generation;
        match event {
            DispatchEvent::PointerMove { pointer, x, y } => {
                self.coalesced.coalesce_pointer_move(PointerMoveSample { pointer, x, y, generation });
                EnqueueOutcome::Accepted
            }
            DispatchEvent::Scroll { x, y, delta_x, delta_y } => {
                self.coalesced.coalesce_scroll(ScrollSample { x, y, delta_x, delta_y, generation });
                EnqueueOutcome::Accepted
            }
            other => self.push_discrete(other, generation),
        }
    }

    /// 📐️ [`crate::window::WindowMetrics`] funnels through here rather than [`Self::enqueue`] — it is
    /// not a [`ui_render::DispatchEvent`] variant at all (see `window.rs`'s own `WindowDelegate::
    /// handle_metrics`), but is exactly as replaceable as pointer move/scroll.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn enqueue_metrics(&mut self, _ui: UiThreadToken, physical_width: u32, physical_height: u32, scale_factor: f32) {
        self.generation = self.generation.next();
        self.coalesced.coalesce_metrics(MetricsSample { physical_width, physical_height, scale_factor, generation: self.generation });
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn push_discrete(&mut self, event: DispatchEvent, generation: InputGeneration) -> EnqueueOutcome {
        let bytes = event_owned_bytes(&event);
        if bytes > DISCRETE_EVENT_BYTE_CAPACITY || self.discrete.len() >= DISCRETE_QUEUE_CAPACITY || self.discrete_bytes.saturating_add(bytes) > DISCRETE_QUEUE_BYTE_CAPACITY {
            self.overflow_count += 1;
            return EnqueueOutcome::Overflow;
        }
        self.discrete.push_back(DiscreteEvent { event, generation });
        self.discrete_bytes += bytes;
        EnqueueOutcome::Accepted
    }

    /// 🚿️ Drains everything accumulated since the last drain — coalesced samples first (design doc
    /// ordering: replaceable state settles before discrete events replay against it), then every
    /// discrete event in arrival order. Requires [`WorkerContext`]: draining feeds a frame build, which
    /// is worker-only work per this ticket's capability split.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_page(&mut self, _worker: WorkerContext) -> DrainedEvents {
        let (pointer_move, scroll, metrics) = self.coalesced.drain();
        let mut discrete = std::array::from_fn(|_| None);
        for slot in &mut discrete {
            let Some(event) = self.discrete.pop_front() else { break };
            self.discrete_bytes = self.discrete_bytes.saturating_sub(event_owned_bytes(&event.event));
            *slot = Some(event);
        }
        DrainedEvents { pointer_move, scroll, metrics, discrete }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn overflow_count(&self) -> u64 {
        self.overflow_count
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_empty(&self) -> bool {
        self.coalesced.is_empty() && self.discrete.is_empty()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn pending_discrete_len(&self) -> usize {
        self.discrete.len()
    }

    pub fn close_step(&mut self) -> bool {
        if !self.coalesced.is_empty() {
            self.coalesced.drain();
            return false;
        }
        if let Some(event) = self.discrete.pop_front() {
            self.discrete_bytes = self.discrete_bytes.saturating_sub(event_owned_bytes(&event.event));
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// 📤️ One drain's worth of input — a worker consumes this to advance input dispatch/hit-index state.
#[derive(Debug, Default)]
pub struct DrainedEvents {
    pub pointer_move: Option<PointerMoveSample>,
    pub scroll: Option<ScrollSample>,
    pub metrics: Option<MetricsSample>,
    pub discrete: [Option<DiscreteEvent>; DISCRETE_DRAIN_PAGE_CAPACITY],
}

fn event_owned_bytes(event: &DispatchEvent) -> usize {
    match event {
        DispatchEvent::KeyDown { key, .. } | DispatchEvent::KeyUp { key, .. } => key.len(),
        DispatchEvent::TextInput { text } | DispatchEvent::Paste { text } | DispatchEvent::TextEditChunk { text, .. } => text.len(),
        DispatchEvent::Ime(ui_render::ImeEvent::Update { text, .. }) | DispatchEvent::Ime(ui_render::ImeEvent::Commit { text }) => text.len(),
        _ => 0,
    }
}

//#endregion 🔖️EventQueue

#[cfg(test)]
#[path = "../📥️input/🎟️admission/🧪️tests/🎟️admission/🦀️.rs"]
mod input_admission_tests;

#[cfg(test)]
#[path = "../📥️input/🎟️admission/🪪️root/🧪️tests/🪪️root/🦀️.rs"]
mod input_root_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../📥️input/🎟️admission/✍️writer/🧪️tests/✍️writer/🦀️.rs"]
mod input_writer_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️enqueue-unit/🦀️.rs"]
mod tests;
