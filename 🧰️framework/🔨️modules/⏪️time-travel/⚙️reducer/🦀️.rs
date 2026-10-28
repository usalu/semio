//! ⚙️ Reducer of the time-travel session: the single implementation block that applies one event to the session and returns the effects or the refusal. See [`super`].

use super::*;

//#region 🔖️Reducer
impl TimeTravelSession {
    /// 🌱️ A fresh, inactive session observing `base`.
    pub fn new(base: TimeTravelBase) -> Self {
        Self { base, ..Self::default() }
    }

    /// ⚖️ Applies `event` in place: generation fence first, then the §4 law; a refusal leaves `self` untouched.
    pub fn apply(&mut self, event: TimeTravelEvent) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal> {
        use TimeTravelEvent as E;
        use TimeTravelStage as S;
        if event.generation().is_some_and(|generation| generation != self.generation) {
            return Err(TimeTravelRefusal::Stale);
        }
        if !event.is_well_formed() {
            return Err(TimeTravelRefusal::Illegal);
        }
        match (self.stage, event) {
            (_, E::Begin { target, original }) => self.open(target, original, None),
            (_, E::BeginWithdrawn { target, original }) => self.open(target, original, Some(InputReplacement::Withdrawn)),
            (S::Editing, E::Draft { replacement, .. }) => self.redraft(replacement),
            (S::Editing, E::Withdraw { .. }) => self.redraft(InputReplacement::Withdrawn),
            (S::Editing, E::Accept { .. }) => {
                if let Some(refusal) = self.accept_refusal() {
                    return Err(refusal);
                }
                let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                Ok(self.accept(pending))
            }
            (S::Editing, E::Discard { .. }) => {
                let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                Ok(self.resume(pending.return_stage))
            }
            (_, E::Restore { target, .. }) => match self.restore_refusal(&target) {
                Some(refusal) => Err(refusal),
                None => {
                    self.accepted.retain(|draft| draft.target.mutation != target);
                    Ok(if self.accepted.is_empty() { self.close(Vec::new()) } else { self.settle() })
                }
            },
            (S::Replaying, E::ReplayProgressed { done, total, .. }) => {
                self.progress = Some(TimeTravelProgress { done, total });
                Ok(Vec::new())
            }
            (S::Replaying, E::ReplayCompleted { report, .. }) => {
                self.stage = S::Reviewing;
                self.report = Some(report);
                self.progress = None;
                Ok(Vec::new())
            }
            (S::Replaying, E::ReplayCancelled { .. }) => {
                self.stage = S::Reviewing;
                self.progress = None;
                self.fault = Some(TIME_TRAVEL_CANCELLED_CODE.to_string());
                Ok(Vec::new())
            }
            (S::Replaying, E::ReplayFaulted { code, .. }) => {
                self.stage = S::Reviewing;
                self.progress = None;
                self.fault = Some(code);
                Ok(Vec::new())
            }
            (_, E::Rerun { .. }) => match self.rerun_refusal() {
                Some(refusal) => Err(refusal),
                None => Ok(self.settle()),
            },
            (_, E::RequestFinalize { .. }) => match self.finalize_refusal() {
                Some(refusal) => Err(refusal),
                None => {
                    self.stage = S::Choosing;
                    Ok(vec![TimeTravelEffect::OpenFinalizePrompt])
                }
            },
            (S::Choosing, E::Choose { choice, .. }) => {
                self.stage = S::Finalizing;
                self.generation = self.generation.wrapping_add(1);
                Ok(vec![match choice {
                    TimeTravelChoice::Overwrite => TimeTravelEffect::CommitOverwrite,
                    TimeTravelChoice::Alternative { name } => TimeTravelEffect::CommitAlternative { name },
                }])
            }
            (S::Choosing, E::Back { .. }) => {
                self.stage = S::Reviewing;
                Ok(Vec::new())
            }
            (S::Finalizing, E::Finalized { .. }) => Ok(self.close(Vec::new())),
            (S::Finalizing, E::FinalizeFaulted { code, .. }) => {
                let effects = self.settle();
                self.fault = Some(code);
                Ok(effects)
            }
            (stage, E::BaseMoved { base, positions }) => {
                if base == self.base {
                    return Err(TimeTravelRefusal::Stale);
                }
                self.base = base;
                self.reposition(&positions);
                Ok(match stage {
                    S::Inactive | S::Finalizing => Vec::new(),
                    S::Editing => {
                        self.report = None;
                        self.pending.as_ref().map_or_else(Vec::new, |pending| vec![TimeTravelEffect::ShowPreview { target: pending.target.mutation.clone(), replacement: pending.replacement.clone() }])
                    }
                    S::Replaying | S::Reviewing | S::Choosing => self.settle(),
                })
            }
            (S::Inactive, E::Exit) => Err(TimeTravelRefusal::Illegal),
            (S::Finalizing, E::Exit) => Err(TimeTravelRefusal::Blocked),
            (S::Replaying, E::Exit) => Ok(self.close(vec![TimeTravelEffect::CancelReplay])),
            (_, E::Exit) => Ok(self.close(Vec::new())),
            _ => Err(TimeTravelRefusal::Illegal),
        }
    }

    /// 🔎️ The accepted draft of `mutation`, if any.
    pub fn accepted_draft(&self, mutation: &MutationId) -> Option<&TimeTravelDraft> {
        self.accepted.iter().find(|draft| draft.target.mutation == *mutation)
    }

    /// 🧾️ The accepted drafts as `Supersede` inputs, in history order.
    pub fn inputs(&self) -> Vec<SupersededInput> {
        self.accepted.iter().map(|draft| SupersededInput { target: draft.target.mutation.clone(), replacement: draft.replacement.clone() }).collect()
    }

    /// 🧷️ The value a pending draft started from: the target's accepted draft, else its original input.
    pub fn start_of<'session>(&'session self, pending: &'session TimeTravelPending) -> &'session InputReplacement {
        self.accepted_draft(&pending.target.mutation).map_or(&pending.original, |draft| &draft.replacement)
    }

    /// 🟰️ Whether `pending` still equals the value it started from.
    pub fn unchanged(&self, pending: &TimeTravelPending) -> bool {
        pending.replacement == *self.start_of(pending)
    }

    /// 🛑️ Why `Accept` would be refused; an unchanged input keeps the draft open.
    pub fn accept_refusal(&self) -> Option<TimeTravelRefusal> {
        match (self.stage, self.pending.as_ref()) {
            (TimeTravelStage::Editing, Some(pending)) if self.unchanged(pending) => Some(TimeTravelRefusal::Unchanged),
            (TimeTravelStage::Editing, Some(_)) => None,
            _ => Some(TimeTravelRefusal::Illegal),
        }
    }

    /// 🚧️ Why `RequestFinalize` would be refused, `None` when it would open the prompt.
    pub fn finalize_refusal(&self) -> Option<TimeTravelRefusal> {
        if self.stage != TimeTravelStage::Reviewing {
            Some(TimeTravelRefusal::Illegal)
        } else if self.accepted.is_empty() {
            Some(TimeTravelRefusal::Empty)
        } else if self.report.as_ref().is_none_or(ReplayReport::blocks_finalize) {
            Some(TimeTravelRefusal::Blocked)
        } else {
            None
        }
    }

    /// ✏️ Why `Begin` (and `BeginWithdrawn`) would be refused, `None` when it would open (or switch) the draft editor: it is
    /// legal from `Inactive` and `Reviewing`, from `Editing` only while the pending draft still equals its start (`Blocked`
    /// otherwise), and `Illegal` while replaying, choosing or finalizing — what a host disables a row's Edit and Withdraw
    /// controls by.
    pub fn begin_refusal(&self) -> Option<TimeTravelRefusal> {
        match (self.stage, self.pending.as_ref()) {
            (TimeTravelStage::Inactive | TimeTravelStage::Reviewing, _) => None,
            (TimeTravelStage::Editing, Some(pending)) if self.unchanged(pending) => None,
            (TimeTravelStage::Editing, Some(_)) => Some(TimeTravelRefusal::Blocked),
            _ => Some(TimeTravelRefusal::Illegal),
        }
    }

    /// 🔙️ Why `Restore` of `mutation` would be refused, `None` when it would take the mutation's accepted draft back: it
    /// needs `Reviewing` and an accepted draft of that mutation (`Illegal` otherwise) — what a host disables a row's
    /// Restore control by.
    pub fn restore_refusal(&self, mutation: &MutationId) -> Option<TimeTravelRefusal> {
        (self.stage != TimeTravelStage::Reviewing || self.accepted_draft(mutation).is_none()).then_some(TimeTravelRefusal::Illegal)
    }

    /// 🔁️ Why `Rerun` would be refused, `None` when it would start a replay: it needs `Reviewing`,
    /// accepted drafts, and either no report or a fault.
    pub fn rerun_refusal(&self) -> Option<TimeTravelRefusal> {
        if self.stage != TimeTravelStage::Reviewing {
            Some(TimeTravelRefusal::Illegal)
        } else if self.accepted.is_empty() {
            Some(TimeTravelRefusal::Empty)
        } else if self.report.is_some() && self.fault.is_none() {
            Some(TimeTravelRefusal::Illegal)
        } else {
            None
        }
    }

    /// 🧐️ Classification of a `Reviewing` session; `None` in every other stage.
    pub fn review(&self) -> Option<TimeTravelReview> {
        (self.stage == TimeTravelStage::Reviewing).then(|| match &self.report {
            _ if self.accepted.is_empty() => TimeTravelReview::NoChanges,
            None => TimeTravelReview::NeedsReplay,
            Some(report) if report.blocks_finalize() => TimeTravelReview::Blocked,
            Some(_) => TimeTravelReview::Ready,
        })
    }

    /// 🧪️ First violated session invariant (law fixture `invariants`), `None` when coherent.
    pub fn invariant_violation(&self) -> Option<&'static str> {
        use TimeTravelStage as S;
        if (self.stage == S::Editing) != self.pending.is_some() {
            return Some("pending-iff-editing");
        }
        if self.pending.as_ref().is_some_and(|pending| !matches!(pending.return_stage, S::Inactive | S::Reviewing)) {
            return Some("return-stage-inactive-or-reviewing");
        }
        if self.pending.as_ref().is_some_and(|pending| pending.return_stage == S::Inactive && !self.accepted.is_empty()) {
            return Some("return-inactive-means-nothing-accepted");
        }
        if self.stage == S::Inactive && (!self.accepted.is_empty() || self.report.is_some() || self.fault.is_some()) {
            return Some("inactive-holds-nothing");
        }
        if matches!(self.stage, S::Replaying | S::Choosing | S::Finalizing) && self.accepted.is_empty() {
            return Some("work-needs-accepted-drafts");
        }
        if self.progress.is_some() && self.stage != S::Replaying {
            return Some("progress-only-while-replaying");
        }
        if self.stage == S::Replaying && self.report.is_some() {
            return Some("replaying-has-no-report");
        }
        if matches!(self.stage, S::Choosing | S::Finalizing) && self.report.as_ref().is_none_or(ReplayReport::blocks_finalize) {
            return Some("finalize-needs-a-clean-report");
        }
        if (1..self.accepted.len()).any(|index| self.accepted[index-1].target.history_order(&self.accepted[index].target) != Ordering::Less) {
            return Some("accepted-in-history-order");
        }
        if self.accepted.iter().enumerate().any(|(index, draft)| self.accepted.iter().take(index).any(|earlier| earlier.target.mutation == draft.target.mutation)) {
            return Some("accepted-targets-unique");
        }
        if self.fault.is_some() && self.accepted.is_empty() {
            return Some("fault-needs-drafts");
        }
        None
    }

    /// 🔦️ Opens (or retargets) the draft editor on `target` where [`Self::begin_refusal`] admits it: the pending draft starts
    /// as `draft`, else as the target's accepted draft, else as `original`, and returns to the stage the session was opened
    /// from; a session opened from `Inactive` is the next one.
    fn open(&mut self, target: TimeTravelTarget, original: InputReplacement, draft: Option<InputReplacement>) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal> {
        if let Some(refusal) = self.begin_refusal() {
            return Err(refusal);
        }
        let return_stage = self.pending.as_ref().map_or(self.stage, |pending| pending.return_stage);
        if self.stage == TimeTravelStage::Inactive {
            self.id = self.id.wrapping_add(1);
        }
        let replacement = draft.or_else(|| self.accepted_draft(&target.mutation).map(|accepted| accepted.replacement.clone())).unwrap_or_else(|| original.clone());
        let effect = TimeTravelEffect::ShowPreview { target: target.mutation.clone(), replacement: replacement.clone() };
        self.stage = TimeTravelStage::Editing;
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some(TimeTravelPending { target, original, replacement, return_stage });
        Ok(vec![effect])
    }

    fn redraft(&mut self, replacement: InputReplacement) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal> {
        let pending = self.pending.as_mut().ok_or(TimeTravelRefusal::Illegal)?;
        pending.replacement = replacement;
        Ok(vec![TimeTravelEffect::ShowPreview { target: pending.target.mutation.clone(), replacement: pending.replacement.clone() }])
    }

    pub(super) fn accept(&mut self, pending: TimeTravelPending) -> Vec<TimeTravelEffect> {
        self.accepted.retain(|draft| draft.target.mutation != pending.target.mutation);
        if pending.replacement != pending.original {
            let draft = TimeTravelDraft { target: pending.target, replacement: pending.replacement };
            let index = self.accepted.iter().take_while(|existing| existing.target.history_order(&draft.target) == Ordering::Less).count();
            self.accepted.insert(index, draft);
        }
        self.settle()
    }

    fn resume(&mut self, return_stage: TimeTravelStage) -> Vec<TimeTravelEffect> {
        if return_stage != TimeTravelStage::Reviewing {
            return self.close(Vec::new());
        }
        if self.report.is_none() && !self.accepted.is_empty() {
            return self.settle();
        }
        self.stage = TimeTravelStage::Reviewing;
        Vec::new()
    }

    fn settle(&mut self) -> Vec<TimeTravelEffect> {
        self.report = None;
        self.progress = None;
        let Some(first) = self.accepted.first() else {
            self.stage = TimeTravelStage::Reviewing;
            self.fault = None;
            return Vec::new();
        };
        let from = first.target.mutation.clone();
        self.stage = TimeTravelStage::Replaying;
        self.fault = None;
        self.generation = self.generation.wrapping_add(1);
        vec![TimeTravelEffect::StartReplay { drafts: self.inputs(), from }]
    }

    pub(super) fn close(&mut self, mut effects: Vec<TimeTravelEffect>) -> Vec<TimeTravelEffect> {
        self.stage = TimeTravelStage::Inactive;
        self.generation = self.generation.wrapping_add(1);
        self.accepted.clear();
        self.pending = None;
        self.report = None;
        self.progress = None;
        self.fault = None;
        effects.push(TimeTravelEffect::Close);
        effects
    }

    fn reposition(&mut self, positions: &[TimeTravelTarget]) {
        for moved in positions {
            for target in self.accepted.iter_mut().map(|draft| &mut draft.target).chain(self.pending.as_mut().map(|pending| &mut pending.target)) {
                if target.mutation == moved.mutation {
                    target.position = moved.position;
                }
            }
        }
        let length=self.accepted.len();
        for root in (0..length/2).rev(){self.sift_accepted(root,length)}
        for end in (1..length).rev(){self.accepted.swap(0,end);self.sift_accepted(0,end)}
    }

    fn sift_accepted(&mut self,mut root:usize,length:usize){
        while root<length/2{
            let mut child=root*2+1;
            if child+1<length&&self.accepted[child].target.history_order(&self.accepted[child+1].target)==Ordering::Less{child+=1}
            if self.accepted[root].target.history_order(&self.accepted[child].target)!=Ordering::Less{return}
            self.accepted.swap(root,child);root=child;
        }
    }
}
//#endregion 🔖️Reducer
