//! 🖐️ The gesture slot a streamed tool drives inside its window: capture, slot state and decisions.
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework::Fault;
use semio_framework_value::shared_issuance::SealedShared;
use semio_framework_tool_machine::GesturePhase;
use semio_framework_tool_machine::GestureState;
use semio_framework_tool_machine::GestureTool;

//#endregion 🔖️Tag

//#region 🖐️Gesture
/// 🎰️ The dispatching window's gesture slot as ONE admitted dispatch sees it (design §22.10): the gesture the window held
/// at admission with the press it last closed, the document revision the dispatch runs on, and what the dispatch decided — kept by the runtime only when
/// the dispatch publishes, and only while the window still holds the gesture it was admitted on. Shared with the dispatch's
/// retained job, so its state sits behind one lock.
#[derive(semio_framework_value::RetireOwned)]
pub struct GestureSlot<M> {
    window: String,
    base_revision: String,
    pub state: std::sync::Mutex<GestureSlotState<M>>,
}

#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct GestureSlotState<M> {
    pub admitted: Option<GestureState<M>>,
    closed: Option<String>,
    pub decided: Option<Option<GestureState<M>>>,
    pub closing: Option<String>,
    pub driven: bool,
}

/// 📮️ What a driven slot hands the runtime at publication: the gesture it was admitted on, the gesture it decided (when it
/// changed) and the press it closed.
#[derive(semio_framework_value::RetireOwned)]
pub struct GestureSlotDecision<M> {
    pub admitted: Option<GestureState<M>>,
    pub decided: Option<Option<GestureState<M>>>,
    pub closing: Option<String>,
}

impl<M> GestureSlot<M> {
    /// 🕳️ The slot of a dispatch outside a live window (a preview, a view without command authority): it holds no gesture
    /// and nothing it decides is kept.
    pub fn detached() -> Self {
        Self::of(String::new(), String::new(), None, None)
    }

    pub fn of(window: String, base_revision: String, admitted: Option<GestureState<M>>, closed: Option<String>) -> Self {
        Self { window, base_revision, state: std::sync::Mutex::new(GestureSlotState { admitted, closed, decided: None, closing: None, driven: false }) }
    }

    /// 🪟️ The window this slot belongs to.
    pub fn window(&self) -> &str {
        &self.window
    }

    pub fn take(&self) -> Option<GestureSlotDecision<M>> {
        let mut state = self.state.lock().ok()?;
        std::mem::take(&mut state.driven).then(|| GestureSlotDecision { admitted: state.admitted.take(), decided: state.decided.take(), closing: state.closing.take() })
    }

    /// 🔦️ The window's gesture as this dispatch sees it: what it decided so far, else the gesture the window held when the
    /// dispatch was admitted.
    pub fn open(&self) -> Option<GestureState<M>>
    where
        M: Clone,
    {
        let state = self.state.lock().ok()?;
        match &state.decided {
            Some(decided) => decided.clone(),
            None => state.admitted.clone(),
        }
    }

    /// 🚃️ Drives the window's streamed tool `T` through ONE dispatch of `verb` ([`semio_framework_tool_machine::drive_press`])
    /// against this slot — a second drive of the same dispatch continues from the first — and answers the transaction it
    /// committed (publish it as ONE edit: [`gesture_emit`]). `press` is the host press the dispatch names (the verb's
    /// `gesture` argument; `None`: none): a dispatch of the press the window last closed is dropped with zero trace, a
    /// dispatch of another press interrupts the open gesture first. A tool that is not base-bound pins its gesture to no revision. A
    /// refused start or tick is the dispatch's fault (`toolTransaction.closed` | `toolTransaction.unclosed`) and decides nothing.
    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(&self, press: Option<&str>, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str) -> Result<Option<(protocol::TransactionRef, Vec<M>)>, Fault>
    where
        M: PartialEq,
    {
        let mut state = self.state.lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("toolTransaction.slot-poisoned"), format!("the gesture slot of window {:?} is poisoned", self.window)))?;
        let held = match state.decided.as_ref() {
            Some(decided) => decided.as_ref(),
            None => state.admitted.as_ref(),
        };
        let closed = state.closing.as_deref().or(state.closed.as_deref());
        let drive = semio_framework_tool_machine::drive_press::<T, M>(held, closed, press, verb, phase, tick, authoring_seed, if T::BASE_BOUND { self.base_revision.as_str() } else { "" })
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("gesture tool {verb:?} refused its dispatch")))?;
        state.driven = true;
        if let Some(decided) = drive.next {
            state.decided = Some(decided);
        }
        if let Some(press) = drive.closed {
            state.closing = Some(press);
        }
        Ok(drive.committed)
    }
}

#[derive(semio_framework_value::RetireOwned)]
pub enum GestureCapture<M:Send+'static>{Detached(GestureSlot<M>),Issued(SealedShared<GestureSlot<M>>)}

impl<M:Send+'static> GestureCapture<M>{
 pub fn detached()->Self{Self::Detached(GestureSlot::detached())}
 pub fn get(&self)->&GestureSlot<M>{match self{Self::Detached(slot)=>slot,Self::Issued(slot)=>slot.get()}}
}

impl<M:Send+'static> std::ops::Deref for GestureCapture<M>{type Target=GestureSlot<M>;fn deref(&self)->&Self::Target{self.get()}}
