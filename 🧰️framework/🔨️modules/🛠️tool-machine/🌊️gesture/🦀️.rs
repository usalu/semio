//! 🔧️ Tool gesture drive: gesture phases, host events, the gesture and press drives and the per-window gesture ledger.

use super::*;

//#region 🌊️Gesture
/// 🌊️ Where one dispatch of a streamed window gesture (a gumball drag, a paint stroke) sits: a one-shot `Once`, a
/// `Stream` tick into the window's open transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GesturePhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl GesturePhase {
    /// 🔡️ Reads a gesture verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason`
    /// (`blur`, `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Option<Self> {
        match phase {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => reason.map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }
}

/// 🖐️ One window's streamed tool for ONE dispatch — a [`ToolMachineRunner`] started at rest, or resumed from the gesture
/// its window persisted between dispatches (the runtime's [`GestureLedger`] slot, never history). A statechart tool is
/// [`ChartGesture`] over its [`GestureChart`].
pub trait GestureTool: Sized {
    type Gesture: PartialEq;
    type Tick;
    type Mutation;
    /// 📌️ Whether a gesture is pinned to the document revision it opened on, so a moved base ends it `baseMoved`; a tool
    /// whose leaves fold on any base says `false` and is driven with an empty base revision.
    const BASE_BOUND: bool = true;
    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal>;
    fn resume(gesture: &Self::Gesture) -> Result<Self, ToolRefusal>;
    fn verb(&self) -> &str;
    fn base_revision(&self) -> &str;
    fn abort(&mut self, reason: ToolAbortReason);
    fn send(&mut self, phase: GesturePhase, tick: Option<Self::Tick>) -> Result<ToolStep<Self::Mutation>, ToolRefusal>;
    fn persist(self) -> Option<Self::Gesture>;
}

/// 📬️ What one gesture dispatch did: the transaction it committed (publish it as ONE edit), the window's next persisted
/// gesture when it changed (`Some(None)` clears it), and whether the dispatch `continued` the persisted gesture (resumed
/// it, neither dropped nor interrupted).
pub struct GestureDrive<G, M> {
    pub committed: Option<(TransactionRef, Vec<M>)>,
    pub next: Option<Option<G>>,
    pub continued: bool,
}

/// 🚂️ Drives one window's streamed tool through ONE dispatch (law `🧫️fixtures/🧫️gesture-drive-law`). `Once` commits
/// `tick` as one transaction; `Stream` upserts it into the window's open transaction (opening it on the first tick),
/// `Commit` folds it in and commits the whole gesture, `Abort` drops the open gesture with zero trace. A gesture whose base
/// moved under it is aborted `baseMoved` (a stream tick or commit that found it is dropped with it, a one-shot commits
/// fresh); otherwise another verb or a one-shot interrupts it (`captureLost`). A gesture its tool cannot restore is dropped
/// with zero trace and the dispatch runs from rest; a refused start or tick is the dispatch's refusal, with no effect at all.
pub fn drive_gesture<T: GestureTool>(persisted: Option<&T::Gesture>, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str, base_revision: &str) -> Result<GestureDrive<T::Gesture, T::Mutation>, ToolRefusal> {
    let dropped = || GestureDrive { committed: None, next: persisted.map(|_| None), continued: false };
    let (open, interrupted) = match (persisted.and_then(|gesture| T::resume(gesture).ok()), phase) {
        (Some(mut tool), GesturePhase::Abort(reason)) => {
            tool.abort(reason);
            return Ok(dropped());
        }
        (None, GesturePhase::Abort(_)) => return Ok(dropped()),
        (Some(mut tool), _) if tool.base_revision() != base_revision && phase != GesturePhase::Once => {
            tool.abort(ToolAbortReason::BaseMoved);
            return Ok(dropped());
        }
        (Some(tool), _) if tool.base_revision() != base_revision => (None, Some((tool, ToolAbortReason::BaseMoved))),
        (Some(tool), _) if tool.verb() != verb || phase == GesturePhase::Once => (None, Some((tool, ToolAbortReason::CaptureLost))),
        (open, _) => (open, None),
    };
    let continued = open.is_some();
    let mut tool = match open {
        Some(tool) => tool,
        None => T::start(verb, authoring_seed, base_revision)?,
    };
    let step = tool.send(phase, tick)?;
    if let Some((mut interrupted, reason)) = interrupted {
        interrupted.abort(reason);
    }
    let gesture = tool.persist();
    let next = (gesture.as_ref() != persisted).then_some(gesture);
    let committed = match step {
        ToolStep::Committed(reference, mutations) => Some((reference, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    };
    Ok(GestureDrive { committed, next, continued })
}

/// 📡️ A host fact that may end a window's open gesture (law table `hostEvents` of `🧫️fixtures/🧫️gesture-drive-law`): the
/// window lost focus or its pointer capture, its utility switched, it is closing, a history edit froze the document, or a
/// remote edit moved the base.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GestureHostEvent {
    Blur,
    CaptureLost,
    UtilityChanged,
    Retiring,
    TimeTravelFrozen,
    BaseMoved,
}

impl GestureHostEvent {
    pub const ALL: [Self; 6] = [Self::Blur, Self::CaptureLost, Self::UtilityChanged, Self::Retiring, Self::TimeTravelFrozen, Self::BaseMoved];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::CaptureLost => "captureLost",
            Self::UtilityChanged => "utilityChanged",
            Self::Retiring => "retiring",
            Self::TimeTravelFrozen => "timeTravelFrozen",
            Self::BaseMoved => "baseMoved",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|event| event.as_str() == text)
    }

    /// 🧨️ The reason this fact ends an open gesture with; `None` keeps the gesture: a moved base ends only a gesture pinned
    /// to a base revision (`base_bound`), every other fact ends any gesture.
    pub fn abort_reason(self, base_bound: bool) -> Option<ToolAbortReason> {
        match self {
            Self::Blur => Some(ToolAbortReason::Blur),
            Self::CaptureLost => Some(ToolAbortReason::CaptureLost),
            Self::UtilityChanged | Self::Retiring => Some(ToolAbortReason::Retired),
            Self::TimeTravelFrozen => Some(ToolAbortReason::Frozen),
            Self::BaseMoved => base_bound.then_some(ToolAbortReason::BaseMoved),
        }
    }
}

/// 🫧️ One window's open gesture between dispatches — the framework-owned persisted form of a streamed statechart tool
/// (`$defs/GestureState`; window slot of the plugin runtime: ephemeral, local-only, never history): the configuration by
/// stable ids, the verb, the host press that opened it (empty: none named; stamped by the slot — [`drive_press`] — never by
/// the tool), the admission's authoring seed and the document revision it opened on (empty: pinned to none), the open
/// transaction with its keyed provisional mutations, and the tool context its entries do not already say (`Null`: none).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct GestureState<M> {
    pub states: Vec<String>,
    pub verb: String,
    pub press: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
    pub context: protocol::DslValue,
}

/// 🧭️ A statechart tool on the shared gesture runner: the chart, its host, the event one dispatch sends and how a resumed
/// gesture gets its chart context back. Starting, resuming, persisting and aborting are [`ChartGesture`]'s, the persisted
/// form is [`GestureState`], the window slot is the runtime's [`GestureLedger`] — a plugin declares none of them.
pub trait GestureChart: ToolMachine
where
    Self::Input: Clone,
{
    type Tick;
    type Host: Host<Self>;

    /// 🧲️ Whether a gesture is pinned to the document revision it opened on, so a moved base ends it `baseMoved`; `false`
    /// for a tool whose leaves fold on any base.
    const BASE_BOUND: bool = true;

    /// 🪛️ The authoring tool id `<appId>#<verb>` every transaction of `verb` is stamped with.
    fn tool(verb: &str) -> String;

    /// 🏠️ The host the chart's timers, invokes and foreign effects go to.
    fn host() -> Self::Host;

    /// 🥚️ The input the chart starts from.
    fn input() -> Self::Input;

    /// 🧶️ The chart context of a resumed gesture, rebuilt from its open transaction's entries and the context it persisted;
    /// `None` refuses the gesture (`Closed`), so the drive drops it with zero trace.
    fn restore(entries: &[(String, Self::Mutation)], context: &protocol::DslValue) -> Option<Self::Context>;

    /// 🎒️ The tool context a gesture persists beyond its entries.
    fn context(_context: &Self::Context) -> protocol::DslValue {
        protocol::DslValue::Null
    }

    /// 📣️ The event one dispatch sends (`at_rest`: no gesture is in flight); `None` sends nothing.
    fn event(phase: GesturePhase, at_rest: bool, tick: Option<Self::Tick>) -> Option<Self::Event>;
}

pub(crate) static GESTURE_CLOCK_TICK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 🎛️ The ONE [`GestureTool`] of every statechart tool: a [`ToolMachineRunner`] over a [`GestureChart`], started at rest
/// or resumed from its window's [`GestureState`], each event on the host clock with a process-monotone tick (two gestures
/// opened in one millisecond never mint one transaction id).
pub struct ChartGesture<T: GestureChart>
where
    T::Input: Clone,
{
    runner: ToolMachineRunner<T, T::Host>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

impl<T: GestureChart> GestureTool for ChartGesture<T>
where
    T::Input: Clone,
    T::Mutation: Clone + PartialEq,
{
    type Gesture = GestureState<T::Mutation>;
    type Tick = T::Tick;
    type Mutation = T::Mutation;

    const BASE_BOUND: bool = T::BASE_BOUND;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(T::tool(verb), ActorId(authoring_seed.into()), T::input(), T::host())?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    fn resume(gesture: &GestureState<T::Mutation>) -> Result<Self, ToolRefusal> {
        let context = T::restore(&gesture.entries, &gesture.context).ok_or(ToolRefusal::Closed)?;
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: T::definition().fingerprint, states: gesture.states.clone(), history: Vec::new(), done: false };
        let snapshot = machine::restore::<T, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(gesture.transaction.clone(), gesture.entries.clone());
        let runner = ToolMachineRunner::resume(T::tool(&gesture.verb), ActorId(gesture.authoring_seed.clone().into()), T::input(), snapshot, Some(transaction), T::host())?;
        Ok(Self { runner, verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone() })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base_revision
    }

    fn abort(&mut self, reason: ToolAbortReason) {
        self.runner.abort(reason);
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<T::Tick>) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        match T::event(phase, self.runner.at_rest(), tick) {
            Some(event) => self.runner.send(event, authoring_clock(GESTURE_CLOCK_TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed))),
            None if self.runner.transaction().is_some() => Ok(ToolStep::Open),
            None => Ok(ToolStep::Idle),
        }
    }

    fn persist(self) -> Option<GestureState<T::Mutation>> {
        let context = T::context(&self.runner.snapshot().context);
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(GestureState {
            states: machine::persist(&snapshot).states,
            verb: self.verb,
            press: String::new(),
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            transaction: transaction.reference().clone(),
            entries: transaction.entries().to_vec(),
            context,
        })
    }
}

/// 🎬️ Drives the statechart tool `T` through ONE dispatch from its window's persisted gesture ([`drive_gesture`] over
/// [`ChartGesture`]); a tool that is not [`GestureChart::BASE_BOUND`] pins its gesture to no revision.
pub fn drive_chart_gesture<T: GestureChart>(
    persisted: Option<&GestureState<T::Mutation>>,
    verb: &str,
    phase: GesturePhase,
    tick: Option<T::Tick>,
    authoring_seed: &str,
    base_revision: &str,
) -> Result<GestureDrive<GestureState<T::Mutation>, T::Mutation>, ToolRefusal>
where
    T::Input: Clone,
    T::Mutation: Clone + PartialEq,
{
    drive_gesture::<ChartGesture<T>>(persisted, verb, phase, tick, authoring_seed, if T::BASE_BOUND { base_revision } else { "" })
}

/// 🎫️ What one dispatch did to its window's slot: the transaction it committed (publish it as ONE edit), the slot's next
/// gesture when it changed (`Some(None)` clears it) and the press the window closed with it.
pub struct PressDrive<M> {
    pub committed: Option<(TransactionRef, Vec<M>)>,
    pub next: Option<Option<GestureState<M>>>,
    pub closed: Option<String>,
}

/// 🪪️ Drives one window's slot through ONE dispatch that may name its host `press` (law `slots`): the slot owns the press
/// identity, never the tool. A gesture belongs to the press that opened it. A dispatch of the press the window last
/// `closed` is dropped with zero trace (a late release after a blur commits nothing); a dispatch of another press
/// interrupts the open gesture first; a named gesture that ends — by anything — closes its press, and so does a named
/// one-shot, commit or abort. A refused dispatch changes nothing.
#[expect(clippy::too_many_arguments, reason = "One dispatch against one slot: the slot's two halves, the press, and drive_gesture's own inputs.")]
pub fn drive_press<T: GestureTool<Gesture = GestureState<M>, Mutation = M>, M: PartialEq>(
    held: Option<&GestureState<M>>,
    closed: Option<&str>,
    press: Option<&str>,
    verb: &str,
    phase: GesturePhase,
    tick: Option<T::Tick>,
    authoring_seed: &str,
    base_revision: &str,
) -> Result<PressDrive<M>, ToolRefusal> {
    let press = press.filter(|press| !press.is_empty());
    if press.is_some() && closed == press {
        return Ok(PressDrive { committed: None, next: None, closed: None });
    }
    let owner = held.map(|open| open.press.as_str()).filter(|owner| !owner.is_empty());
    let interrupts = matches!((press, owner), (Some(press), Some(owner)) if press != owner);
    let drive = drive_gesture::<T>(held.filter(|_| !interrupts), verb, phase, tick, authoring_seed, base_revision)?;
    let next = match drive.next {
        Some(Some(mut gesture)) => {
            gesture.press = if drive.continued { owner } else { press }.unwrap_or_default().to_string();
            (Some(&gesture) != held).then_some(Some(gesture))
        }
        Some(None) => Some(None),
        None => interrupts.then_some(None),
    };
    let after = match &next {
        Some(next) => next.as_ref(),
        None => held,
    };
    let ended = owner.filter(|owner| after.is_none_or(|after| after.press != *owner));
    let closed = press.filter(|_| phase != GesturePhase::Stream).or(ended).map(str::to_string);
    Ok(PressDrive { committed: drive.committed, next, closed })
}

/// 🗄️ Every window's open gesture, at most one per window, and the press each window last closed — the ONE framework-owned window slot of a persisted
/// [`GestureTool`] (design §22.10, law `slots`). Pure: the runtime keeps one per app instance, overlays
/// [`Self::provisional`] on the committed document for every render, and ends a window's gesture on its host facts
/// ([`Self::host_event`]) — no editor maps a host fact to an abort of its own. A persisted gesture holds no host timer.
#[derive(Debug, PartialEq)]
pub struct GestureLedger<M> {
    pub(super) source_revision:u64,
    pub(super) live_windows:usize,
    pub(super) windows: std::collections::BTreeMap<semio_framework_value::SharedUtf8, RetainedOriginalSlot<GestureState<M>>>,
    pub(crate) closed: std::collections::BTreeMap<String, String>,
}

impl<M> Default for GestureLedger<M> {
    fn default() -> Self {
        Self { source_revision:0,live_windows:0, windows: std::collections::BTreeMap::new(), closed: std::collections::BTreeMap::new() }
    }
}

impl<M> GestureLedger<M> {
    /// 🪪️ Identifies the original provisional source across a multi-turn complete-prefix refold.
    pub fn source_revision(&self)->u64{self.source_revision}
    fn update_live_windows(&mut self,window:&str,was_open:bool){let open=self.open(window).is_some();if open&&!was_open{self.live_windows+=1;}else if was_open&&!open{self.live_windows-=1;}}
    fn change_source_revision(&mut self){self.source_revision=self.source_revision.checked_add(1).expect("original tool source revision exhausted");}

    /// 🪹️ Whether no window holds an open gesture.
    pub fn is_empty(&self) -> bool {
        self.live_windows==0
    }

    /// 🔦️ The open gesture of `window`.
    pub fn open(&self, window: &str) -> Option<&GestureState<M>> {
        self.windows.get(window).filter(|slot|!slot.is_empty()).map(RetainedOriginalSlot::get)
    }

    /// 🏘️ The windows holding an open gesture, in window id order.
    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.windows.iter().filter(|(_,slot)|!slot.is_empty()).map(|(key,_)|key.as_str())
    }

    /// 🪞️ Every open gesture's provisional mutations, window by window in window id order — the overlay a render applies
    /// on the committed document; never history.
    pub fn provisional(&self) -> impl Iterator<Item = &M> {
        self.windows.values().filter(|gesture|!gesture.is_empty()).flat_map(|gesture| gesture.get().entries.iter().map(|(_, mutation)| mutation))
    }

    /// 🚪️ The press `window` last closed: its late dispatches leave zero trace.
    pub fn closed(&self, window: &str) -> Option<&str> {
        self.closed.get(window).map(String::as_str)
    }

    /// 📕️ Every window's last closed press, in window id order.
    pub fn closed_presses(&self) -> impl Iterator<Item = (&str, &str)> {
        self.closed.iter().map(|(window, press)| (window.as_str(), press.as_str()))
    }

    /// 🔏️ Records the press `window` closed.
    pub fn close(&mut self, window: &str, press: String) {
        self.closed.insert(window.to_string(), press);
    }

    /// 🖋️ Keeps the gesture a dispatch decided for `window` (`None` clears the slot).
    pub fn settle(&mut self, window: &str, next: Option<GestureState<M>>) -> Result<(), Option<GestureState<M>>> {
        if self.windows.get(window).is_some_and(RetainedOriginalSlot::has_read){return Err(next);}
        let was_open=self.open(window).is_some();
        self.change_source_revision();
        match next {
            Some(gesture) => {
                self.windows.insert(window.to_string().into(), RetainedOriginalSlot::new(gesture));
            }
            None => {
                self.windows.remove(window);
            }
        }
        self.update_live_windows(window,was_open);
        Ok(())
    }

    /// 🚃️ Drives `window`'s tool through ONE dispatch of `press` (`None`: the host named none) against its slot
    /// ([`drive_press`]): the slot and the closed press follow the drive and the committed transaction is answered; a
    /// refused dispatch leaves the ledger exactly as it was.
    #[expect(clippy::too_many_arguments, reason = "One dispatch of one window: the window, the press, and drive_gesture's own inputs.")]
    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(
        &mut self,
        window: &str,
        press: Option<&str>,
        verb: &str,
        phase: GesturePhase,
        tick: Option<T::Tick>,
        authoring_seed: &str,
        base_revision: &str,
    ) -> Result<Option<(TransactionRef, Vec<M>)>, ToolRefusal>
    where
        M: PartialEq,
    {
        if self.windows.get(window).is_some_and(RetainedOriginalSlot::has_read){return Err(ToolRefusal::Unclosed);}
        let drive = drive_press::<T, M>(self.windows.get(window).filter(|slot|!slot.is_empty()).map(RetainedOriginalSlot::get), self.closed.get(window).map(String::as_str), press, verb, phase, tick, authoring_seed, base_revision)?;
        if let Some(next) = drive.next {
            self.settle(window, next).unwrap_or_else(|_|unreachable!("exclusive original gesture decision remains write-admitted"));
        }
        if let Some(press) = drive.closed {
            self.close(window, press);
        }
        Ok(drive.committed)
    }

    /// 🧹️ Host cancel of `window`'s open gesture: zero trace, and its press is closed. `Aborted(ref, reason)`, or `Idle`
    /// when none was open.
    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> ToolStep<M> {
        if self.windows.get(window).is_some_and(RetainedOriginalSlot::has_read){return ToolStep::Idle;}
        let Some(slot) = self.windows.remove(window).filter(|slot|!slot.is_empty()) else { return ToolStep::Idle };
        self.live_windows-=1;self.change_source_revision();
        let gesture=slot.try_into_original().unwrap_or_else(|_|unreachable!("original gesture write fence"));
        if !gesture.press.is_empty() {
            self.closed.insert(window.to_string(), gesture.press);
        }
        ToolStep::Aborted(gesture.transaction, reason)
    }

    /// 🛎️ A host fact of `window`: its open gesture ends with the fact's reason ([`GestureHostEvent::abort_reason`]; a
    /// gesture pinned to no revision survives a moved base), `Idle` when it stays or none was open.
    pub fn host_event(&mut self, window: &str, event: GestureHostEvent) -> ToolStep<M> {
        match self.windows.get(window).filter(|slot|!slot.is_empty()).map(RetainedOriginalSlot::get).and_then(|gesture| event.abort_reason(!gesture.base_revision.is_empty())) {
            Some(reason) => self.abort(window, reason),
            None => ToolStep::Idle,
        }
    }

    /// 🌐️ A host fact of every window (a history edit freezing the document): one `(window, Aborted)` per gesture it ended,
    /// in window id order.
    pub fn host_event_all(&mut self, event: GestureHostEvent) -> Vec<(String, ToolStep<M>)> {
        let windows: Vec<String> = self.windows.keys().map(|key|key.to_string()).collect();
        windows.into_iter().map(|window| (self.host_event(&window, event), window)).filter(|(step, _)| !matches!(step, ToolStep::Idle)).map(|(step, window)| (window, step)).collect()
    }

    /// ⚰️ Host cancel (`retired`) of the open gesture of every window `keep` refuses; their closed presses are forgotten.
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<(String, ToolStep<M>)> {
        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).map(|key|key.to_string()).collect();
        let ended = retired.into_iter().map(|window| (self.abort(&window, ToolAbortReason::Retired), window)).map(|(step, window)| (window, step)).collect();
        self.closed.retain(|window, _| keep(window));
        ended
    }
}
//#endregion 🌊️Gesture


impl<M:Send+Sync+'static> GestureLedger<M>{
 /// 🔎️ Locates the next original resident address without scanning preceding gesture slots.
 pub fn next_retained_window(&self,after:Option<&str>)->Option<(&semio_framework_value::SharedUtf8,RetainedOriginalSlotKey)>{let next=match after{Some(after)=>self.windows.range::<str,_>((std::ops::Bound::Excluded(after),std::ops::Bound::Unbounded)).next(),None=>self.windows.iter().next()};next.map(|(key,slot)|(key,slot.key()))}


 /// 🔗️ Captures the exact original authored state slot without cloning its entries.
 pub fn retained_read_key(&self,window:&str)->Option<RetainedOriginalSlotKey>{self.windows.get(window).filter(|slot|!slot.is_empty()).map(RetainedOriginalSlot::key)}
 pub fn retained_read_pending(&self,window:&str)->bool{self.windows.get(window).is_some_and(RetainedOriginalSlot::has_read)}
 pub fn retained_read_demands(&self,window:&str,key:RetainedOriginalSlotKey)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.windows.get(window).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original gesture source window absent"))?.demands(key)}
 pub fn advance_retained_read(&mut self,window:&str,key:RetainedOriginalSlotKey,factory:&std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<GestureState<M>>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.windows.get_mut(window).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original gesture source window absent"))?.advance_read(key,factory,grant)}
 pub fn retained_read(&self,window:&str,key:RetainedOriginalSlotKey)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'_,GestureState<M>>,semio_framework_value::ValueError>{self.windows.get(window).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original gesture source window absent"))?.borrow(key)}
 /// 🎟️ Retains the actual current gesture source under its exact original slot key.
 pub fn retain_original_read(&self,window:&str,key:RetainedOriginalSlotKey,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(semio_framework_value::retained_clone::RetainedOwnedProjection<GestureState<M>>,semio_framework_value::retained_clone::RetainedCloneProgress),semio_framework_value::ValueError>{self.windows.get(window).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original gesture source window absent"))?.retain_read(key,grant)}
 pub fn retained_read_return_demands(&self,window:&str,key:RetainedOriginalSlotKey,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.windows.get(window).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original gesture source window absent"))?.return_demands(key,body)}
 pub fn return_retained_read(&mut self,window:&str,key:RetainedOriginalSlotKey,factory:&mut Option<std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<GestureState<M>>>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.windows.get_mut(window).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original gesture source window absent"))?.return_read(key,factory,grant)}
}
