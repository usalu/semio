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
//! - Replaceable samples (pointer move and metrics/resize) — [`CoalesceSlot`]: two fixed `Copy`
//!   fields, no heap allocation ever, latest-wins.
//! - Ordered, lossless events (scroll, pointer down/up, key down/up, ime, paste) — a bounded `VecDeque` sized
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
    pub fn checked_next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
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
    pub modifiers: ui_render::EventModifiers,
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

/// 🖱️ Fixed, `Copy`-only coalescing state for replaceable pointer and metrics samples. No heap
/// allocation on any path — every field is a plain `Option<Copy struct>`.
#[derive(Clone, Copy, Debug, Default)]
pub struct CoalesceSlot {
    pointer_move: Option<PointerMoveSample>,
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

    /// 📐️ Overwrites any pending metrics sample — only the latest size/scale matters.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn coalesce_metrics(&mut self, sample: MetricsSample) {
        self.metrics = Some(sample);
    }

    /// 🚿️ Takes the retained pointer sample once its generation can enter the current ordered page.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn take_pointer_move(&mut self) -> Option<PointerMoveSample> {
        self.pointer_move.take()
    }

    /// 🚿️ Takes the latest metrics sample independently of input dispatch order.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn take_metrics(&mut self) -> Option<MetricsSample> {
        self.metrics.take()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_empty(&self) -> bool {
        self.pointer_move.is_none() && self.metrics.is_none()
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
/// Backing is admitted with the first discrete event and retained until explicit terminal retirement.
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
        Self { #[cfg(test)] root: None, coalesced: CoalesceSlot::new(), discrete: std::collections::VecDeque::new(), generation: InputGeneration::default(), overflow_count: 0, discrete_bytes: 0 }
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
        let Some(generation) = self.generation.checked_next() else {
            self.overflow_count = self.overflow_count.saturating_add(1);
            return EnqueueOutcome::Overflow;
        };
        let outcome = match event {
            DispatchEvent::PointerMove { pointer, x, y, modifiers } => {
                self.coalesced.coalesce_pointer_move(PointerMoveSample { pointer, x, y, modifiers, generation });
                EnqueueOutcome::Accepted
            }
            other => self.push_discrete(other, generation),
        };
        if outcome == EnqueueOutcome::Accepted {
            self.generation = generation;
        }
        outcome
    }

    /// 📐️ [`crate::window::WindowMetrics`] funnels through here rather than [`Self::enqueue`] — it is
    /// not a [`ui_render::DispatchEvent`] variant at all (see `window.rs`'s own `WindowDelegate::
    /// handle_metrics`), but is exactly as replaceable as pointer move/scroll.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn enqueue_metrics(&mut self, _ui: UiThreadToken, physical_width: u32, physical_height: u32, scale_factor: f32) -> EnqueueOutcome {
        let Some(generation) = self.generation.checked_next() else {
            self.overflow_count = self.overflow_count.saturating_add(1);
            return EnqueueOutcome::Overflow;
        };
        self.coalesced.coalesce_metrics(MetricsSample { physical_width, physical_height, scale_factor, generation });
        self.generation = generation;
        EnqueueOutcome::Accepted
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn push_discrete(&mut self, event: DispatchEvent, generation: InputGeneration) -> EnqueueOutcome {
        let bytes = event_owned_bytes(&event);
        if bytes > DISCRETE_EVENT_BYTE_CAPACITY || self.discrete.len() >= DISCRETE_QUEUE_CAPACITY || self.discrete_bytes.saturating_add(bytes) > DISCRETE_QUEUE_BYTE_CAPACITY {
            self.overflow_count = self.overflow_count.saturating_add(1);
            return EnqueueOutcome::Overflow;
        }
        if self.discrete.capacity() < DISCRETE_QUEUE_CAPACITY && self.discrete.try_reserve_exact(DISCRETE_QUEUE_CAPACITY - self.discrete.len()).is_err() {
            self.overflow_count = self.overflow_count.saturating_add(1);
            return EnqueueOutcome::Overflow;
        }
        self.discrete.push_back(DiscreteEvent { event, generation });
        self.discrete_bytes += bytes;
        EnqueueOutcome::Accepted
    }

    /// 🚿️ Drains one bounded ordered page and only exposes a retained pointer sample when no older
    /// ordered event remains for a later page. Requires [`WorkerContext`]: draining feeds a frame
    /// build, which is worker-only work per this ticket's capability split.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_page(&mut self, _worker: WorkerContext) -> DrainedEvents {
        let metrics = self.coalesced.take_metrics();
        let mut discrete = std::array::from_fn(|_| None);
        for slot in &mut discrete {
            let Some(event) = self.discrete.pop_front() else { break };
            self.discrete_bytes = self.discrete_bytes.saturating_sub(event_owned_bytes(&event.event));
            *slot = Some(event);
        }
        let pointer_move = match (self.coalesced.pointer_move.as_ref(), self.discrete.front()) {
            (Some(pointer), Some(next)) if next.generation < pointer.generation => None,
            (Some(_), _) => self.coalesced.take_pointer_move(),
            (None, _) => None,
        };
        DrainedEvents { pointer_move, metrics, discrete }
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
            self.coalesced = CoalesceSlot::new();
            return false;
        }
        if let Some(event) = self.discrete.pop_front() {
            self.discrete_bytes = self.discrete_bytes.saturating_sub(event_owned_bytes(&event.event));
            return false;
        }
        if self.discrete.capacity() != 0 {
            self.discrete = std::collections::VecDeque::new();
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.is_empty() && self.discrete.capacity() == 0
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
    pub metrics: Option<MetricsSample>,
    pub discrete: [Option<DiscreteEvent>; DISCRETE_DRAIN_PAGE_CAPACITY],
}

fn event_owned_bytes(event: &DispatchEvent) -> usize {
    match event {
        DispatchEvent::KeyDown { key, .. } | DispatchEvent::KeyUp { key, .. } => key.len(),
        DispatchEvent::TextInput { text } | DispatchEvent::Paste { text } | DispatchEvent::TextEditChunk { text, .. } => text.len(),
        DispatchEvent::Ime(ui_render::ImeEvent::Update { text, .. }) | DispatchEvent::Ime(ui_render::ImeEvent::Commit { text }) => text.len(),
        DispatchEvent::Accessibility { target, event } => {
            target.window_id.len()
                + target.node_key.len()
                + match event {
                    ui_render::AccessibilityEvent::Value(value) => value.len(),
                    ui_render::AccessibilityEvent::Focus | ui_render::AccessibilityEvent::Blur | ui_render::AccessibilityEvent::Activate => 0,
                }
        }
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
