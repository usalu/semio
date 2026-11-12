//! 🔀️ Supersession ledger of the time-travel session: the supersede role, record and ledger with their labels and per-mutation overlay. See [`super`].

use super::*;

/// 🔎️ A bounded prefix selects only transitions that may affect supersession state.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SupersessionCandidate { Revert, Supersede, Other, Decode }

/// 📐️ Quotes the fixed candidate and the original first payload byte.
pub fn supersession_candidate_copy_bytes() -> usize { 2 * std::mem::size_of::<SupersessionCandidate>() + 1 }

/// 🧭️ Borrows the actual envelope; ambiguous varints remain candidates for full decoding.
pub fn original_supersession_candidate(envelope: &store::os_spr::MutationEnvelope, grant: semio_framework_value::RetainedCloneGrant, fuel: u64) -> semio_framework_value::RetainedWorkOutcome<Option<(SupersessionCandidate,semio_framework_value::RetainedCloneProgress)>> {
    use semio_framework_value::{ByteReadSource,RetainedCloneProgress,RetainedWorkOutcome};
    let schema = envelope.diff.schema.0.as_bytes(); let expected = store::os_spr::HISTORY_TRANSITION_SCHEMA.as_bytes();
    let required = if schema.len() == expected.len() { expected.len() as u64 + 2 } else { 1 };
    if fuel < required || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < supersession_candidate_copy_bytes() { return RetainedWorkOutcome { result: Ok(None), work: 0 }; }
    let mut work = 1; let mut same = schema.len() == expected.len();
    if same { for (left,right) in schema.iter().zip(expected) { work += 1; if left != right { same = false; break; } } }
    let candidate = if !same { SupersessionCandidate::Other } else { work += 1; match envelope.diff.payload.byte_at(0) { Some(0) => SupersessionCandidate::Revert, Some(6) => SupersessionCandidate::Supersede, Some(byte) if byte < 128 => SupersessionCandidate::Other, _ => SupersessionCandidate::Decode } };
    RetainedWorkOutcome { result: Ok(Some((candidate,RetainedCloneProgress { copied_items: 1, copied_bytes: supersession_candidate_copy_bytes(), ..Default::default() }))), work }
}

/// 📚️ Visits pinned original transition slots with bounded directory and schema steps.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct OriginalSupersessionScan { stage:u8, schema_offset:usize, schema_checked:bool, complete:bool }
impl OriginalSupersessionScan {
    pub fn new()->Self{Self::default()}
    pub fn complete(&self)->bool{self.complete}
    pub fn copy_bytes()->usize{semio_os_vcs::ArtifactHistoryDirectoryCursor::<store::os_spr::MutationEnvelope>::copy_bytes()+2*std::mem::size_of::<Self>()}
    /// 🪜️ Reads one directory link, original schema byte, or following row per paid turn.
    pub fn advance(&mut self,source:&mut semio_os_vcs::ArtifactHistoryDirectoryCursor<'_,store::os_spr::MutationEnvelope>,revision:u64,grant:semio_framework_value::RetainedCloneGrant,fuel:u64)->semio_framework_value::RetainedWorkOutcome<(Option<(semio_os_vcs::ArtifactHistoryKey,SupersessionCandidate)>,semio_framework_value::RetainedCloneProgress)>{
        use semio_framework_value::{ByteReadSource,RetainedCloneProgress,RetainedWorkOutcome};
        if !source.is_forward(){return RetainedWorkOutcome{result:Err(semio_framework_value::ValueError::invariant("original supersession scan requires chronological history")),work:0}}
        if self.complete||fuel==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<Self::copy_bytes(){return RetainedWorkOutcome{result:Ok((None,RetainedCloneProgress::default())),work:0}}
        if source.exhausted(){let turn=source.current(revision,grant,fuel);return match turn.result{Err(error)=>RetainedWorkOutcome{result:Err(error),work:turn.work},Ok(_)=>{self.complete=true;RetainedWorkOutcome{result:Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes:2*std::mem::size_of::<Self>(),..Default::default()})),work:turn.work}}}}
        if self.stage==0{
            let turn=source.advance(revision,grant,fuel);
            return match turn.result{Err(error)=>RetainedWorkOutcome{result:Err(error),work:turn.work},Ok((entry,mut progress))=>{if entry.is_some(){self.stage=1;progress.copied_bytes+=2*std::mem::size_of::<Self>();}RetainedWorkOutcome{result:Ok((None,progress)),work:turn.work}}}
        }
        if self.stage==2{
            let turn=source.rearm_following(revision,grant);
            return match turn.result{Err(error)=>RetainedWorkOutcome{result:Err(error),work:turn.work},Ok((changed,mut progress))=>{if changed{self.stage=0;progress.copied_bytes+=2*std::mem::size_of::<Self>();}RetainedWorkOutcome{result:Ok((None,progress)),work:turn.work}}}
        }
        let turn=source.current(revision,grant,fuel);
        let work=turn.work;
        let (key,envelope)=match turn.result{Err(error)=>return RetainedWorkOutcome{result:Err(error),work},Ok(Some(entry))=>entry,Ok(None)=>return RetainedWorkOutcome{result:Ok((None,RetainedCloneProgress::default())),work}};
        let schema=envelope.diff.schema.0.as_bytes();let expected=store::os_spr::HISTORY_TRANSITION_SCHEMA.as_bytes();let mut candidate=None;
        if !self.schema_checked{self.schema_checked=true;if schema.len()!=expected.len(){candidate=Some(SupersessionCandidate::Other)}}else if self.schema_offset<expected.len(){if schema[self.schema_offset]!=expected[self.schema_offset]{candidate=Some(SupersessionCandidate::Other)}else{self.schema_offset+=1}}else{candidate=Some(match envelope.diff.payload.byte_at(0){Some(0)=>SupersessionCandidate::Revert,Some(6)=>SupersessionCandidate::Supersede,Some(byte)if byte<128=>SupersessionCandidate::Other,_=>SupersessionCandidate::Decode})}
        let entry=candidate.map(|candidate|(key,candidate));
        if entry.is_some(){self.schema_checked=false;self.schema_offset=0;if source.has_following(){self.stage=2}else{self.complete=true}}
        RetainedWorkOutcome{result:Ok((entry,RetainedCloneProgress{copied_items:1,copied_bytes:2*std::mem::size_of::<Self>(),..Default::default()})),work}
    }
}

#[cfg(test)]
mod original_candidate_tests {
    use super::*;
    #[test]
    fn original_candidate_reads_the_actual_transition_envelope() {
        use store::os_spr::{history_transition_envelope,HistoryTransition,TransitionSupersede};
        use store::os_spr::ids::{ArtifactId,ActorId};
        let document = ArtifactId("doc".into());
        let actor = ActorId("actor".into());
        let tick = HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 0 };
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: supersession_candidate_copy_bytes(), maximum_depth: 1, ..Default::default() };
        for (transition, expected) in [(HistoryTransition::Revert { mutation_ids: Vec::new() }, SupersessionCandidate::Revert), (HistoryTransition::Reinstate { mutation_ids: Vec::new() }, SupersessionCandidate::Other), (HistoryTransition::Supersede(TransitionSupersede { scope: None, inputs: Vec::new() }), SupersessionCandidate::Supersede)] {
            let envelope = history_transition_envelope(&transition, &document, &actor, Vec::new(), tick);
            let refused = original_supersession_candidate(&envelope, grant, 0);
            assert_eq!(refused.work, 0);
            assert!(refused.result.unwrap().is_none());
            let turn = original_supersession_candidate(&envelope, grant, u64::MAX);
            assert_eq!(turn.work, u128::from(store::os_spr::HISTORY_TRANSITION_SCHEMA.len() as u64 + 2));
            let (actual, progress) = turn.result.unwrap().unwrap();
            assert_eq!(actual, expected);
            assert!(progress.fits(grant));
        }
    }
    #[test]
    fn original_supersession_scan_retains_the_real_ledger_source() {
        use store::os_spr::{history_transition_envelope,HistoryTransition};
        use store::os_spr::ids::{ArtifactId,ActorId};
        let mut source=semio_os_vcs::ArtifactHistoryLedger::new();
        let document=ArtifactId("doc".into());let actor=ActorId("actor".into());
        let tick=HybridLogicalTimestamp{actor:1,physical_ms:2,logical:0};
        for transition in [HistoryTransition::Revert{mutation_ids:Vec::new()},HistoryTransition::Reinstate{mutation_ids:Vec::new()}]{source.try_push(history_transition_envelope(&transition,&document,&actor,Vec::new(),tick)).unwrap();}
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:OriginalSupersessionScan::copy_bytes(),maximum_depth:1,..Default::default()};
        let mut loan=None;assert!(semio_os_vcs::ArtifactHistoryDirectoryCursor::admit_head(&mut source,4,grant,&mut loan).result.unwrap().0);
        let mut cursor=loan.take().unwrap();let mut scan=OriginalSupersessionScan::new();
        assert_eq!(scan.advance(&mut cursor,4,grant,0).work,0);
        assert!(scan.advance(&mut cursor,5,grant,1).result.is_err());
        let saved=scan;let denied=scan.advance(&mut cursor,4,semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},1);
        assert_eq!(denied.work,0);assert!(denied.result.unwrap().0.is_none());assert_eq!(scan,saved);
        let mut candidates=Vec::new();for _ in 0..1024{if scan.complete(){break}let turn=scan.advance(&mut cursor,4,grant,1);assert!(turn.work<=1);let(entry,progress)=turn.result.unwrap();assert!(progress.fits(grant));if let Some((_,candidate))=entry{candidates.push(candidate)}}
        assert_eq!(candidates,[SupersessionCandidate::Revert,SupersessionCandidate::Other]);assert!(scan.complete());
        assert!(cursor.close(grant).result.unwrap().0);drop(cursor);drop(loan);
        let mut reverse=None;assert!(semio_os_vcs::ArtifactHistoryDirectoryCursor::admit_tail(&mut source,4,grant,&mut reverse).result.unwrap().0);
        let mut cursor=reverse.take().unwrap();let mut scan=OriginalSupersessionScan::new();
        assert!(scan.advance(&mut cursor,4,grant,1).result.is_err());assert!(!scan.complete());
        assert!(cursor.close(grant).result.unwrap().0);drop(cursor);drop(reverse);
        while source.pop().is_some(){}
    }
    #[test]
    fn original_supersession_scan_observes_the_real_empty_source() {
        let mut source=semio_os_vcs::ArtifactHistoryLedger::new();
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:OriginalSupersessionScan::copy_bytes(),maximum_depth:1,..Default::default()};
        let mut loan=None;assert!(semio_os_vcs::ArtifactHistoryDirectoryCursor::admit_head(&mut source,7,grant,&mut loan).result.unwrap().0);
        let mut cursor=loan.take().unwrap();let mut scan=OriginalSupersessionScan::new();
        assert!(scan.advance(&mut cursor,8,grant,1).result.is_err());assert!(!scan.complete());
        let turn=scan.advance(&mut cursor,7,grant,1);assert_eq!(turn.work,1);assert!(turn.result.unwrap().0.is_none());assert!(scan.complete());
        assert!(cursor.close(grant).result.unwrap().0);drop(cursor);drop(loan);
    }
    #[test]
    fn original_supersession_scan_cancelled_prefix_keeps_its_source() {
        use store::os_spr::{history_transition_envelope,HistoryTransition};
        use store::os_spr::ids::{ArtifactId,ActorId};
        let mut source=semio_os_vcs::ArtifactHistoryLedger::new();
        let envelope=history_transition_envelope(&HistoryTransition::Revert{mutation_ids:Vec::new()},&ArtifactId("doc".into()),&ActorId("actor".into()),Vec::new(),HybridLogicalTimestamp{actor:1,physical_ms:2,logical:0});
        let pointer=envelope.mutation_id.0.as_ptr();let key=source.try_push(envelope).unwrap();
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:OriginalSupersessionScan::copy_bytes(),maximum_depth:1,..Default::default()};
        for cut in 0..64{
            let mut loan=None;assert!(semio_os_vcs::ArtifactHistoryDirectoryCursor::admit_head(&mut source,4,grant,&mut loan).result.unwrap().0);
            let mut cursor=loan.take().unwrap();let mut scan=OriginalSupersessionScan::new();
            for _ in 0..cut{if scan.complete(){break}assert!(scan.advance(&mut cursor,4,grant,1).result.unwrap().1.fits(grant))}
            let denied=cursor.close(semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant});assert!(!denied.result.unwrap().0);assert_eq!(denied.work,0);
            assert!(cursor.close(grant).result.unwrap().0);drop(cursor);drop(loan);
            assert_eq!(source.get_key(key).unwrap().mutation_id.0.as_ptr(),pointer);
        }
        source.pop();
    }

}

//#region 🔖️Supersessions
/// 🔁️ What one `Supersede` transition does to its author's history edits: a new edit, the undo of an earlier one (it
/// restores every input that edit still holds to the input before it, the original input when there was none), or the
/// redo of an undone one (it re-authors that edit's inputs over the state its undo left).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupersedeRole {
    Edit,
    Undo,
    Redo,
}

/// ✏️ One `Supersede` transition of the event log: its id, author, HLC, scope and inputs, its [`SupersedeRole`] and the
/// history edit it acts for (`entry`, an index into [`SupersedeLedger::records`]) — itself for an edit, the undone edit
/// for an undo, the re-authored edit for a redo.
#[derive(Clone, Debug, PartialEq)]
pub struct SupersedeRecord {
    pub transition_id: String,
    pub actor: semio_framework_value::SharedUtf8,
    pub timestamp: HybridLogicalTimestamp,
    pub scope: Option<String>,
    pub inputs: Vec<protocol::SupersededInput>,
    pub role: SupersedeRole,
    pub entry: usize,
}

/// 📚️ Every `Supersede` transition of a document's event log in fold order `(hlc, id)`, classified per author, with each
/// author's history edits that are not undone, the undone ones a redo re-authors (each with the undo that undid it), each
/// author's newest document-edit undo (`Revert`) and the original input of every superseded operation. A pure function
/// of the transitions and the edits: every replica, and every reload, derives the same rows. Rebuilt only when the
/// transitions changed.
///
/// An input's effective supersession in the view of `scope` is the last one naming it whose scope is `None` or `scope` —
/// the store's fold law — and an undo restores in the view of the undone edit's own scope. Every effective input has an
/// owner: the history edit whose input it is (an edit's own, a redo's re-authored edit's, an undo's the owner of the
/// input it restored), or none for an original input.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SupersedeLedger {
    pub key: (usize, Option<String>),
    pub records: Vec<SupersedeRecord>,
    pub index: HashMap<String, usize>,
    pub originals: HashMap<MutationId, protocol::InputReplacement>,
    pub undo: HashMap<semio_framework_value::SharedUtf8, Vec<usize>>,
    pub redo: HashMap<semio_framework_value::SharedUtf8, Vec<(usize, usize)>>,
    reverts: HashMap<semio_framework_value::SharedUtf8, HybridLogicalTimestamp>,
}

impl SupersedeLedger {
    /// 🔄️ Rebuilds from `transitions` when they changed since the last build; `originals` answers the original input,
    /// encoded like a supersede replacement, of every operation a supersession names.
    pub fn refresh<'a,I:ExactSizeIterator<Item=&'a store::os_spr::MutationEnvelope>+DoubleEndedIterator+Clone>(&mut self,transitions:I, originals: impl FnOnce(&BTreeSet<MutationId>) -> HashMap<MutationId, protocol::InputReplacement>) {
        let key = (transitions.len(), transitions.clone().next_back().map(|envelope| envelope.mutation_id.0.clone()));
        if key == self.key {
            return;
        }
        let mut records = Vec::new();
        let mut reverts: HashMap<semio_framework_value::SharedUtf8, HybridLogicalTimestamp> = HashMap::new();
        for envelope in transitions {
            match store::os_spr::history_transition_from_envelope(envelope) {
                Ok(Some(store::os_spr::HistoryTransition::Supersede(supersede))) => {
                    records.push(SupersedeRecord { transition_id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.clone(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supersede.inputs, role: SupersedeRole::Edit, entry: 0 })
                }
                Ok(Some(store::os_spr::HistoryTransition::Revert { .. })) => {
                    let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
                    *newest = (*newest).max(envelope.timestamp);
                }
                _ => {}
            }
        }
        let originals = originals(&records.iter().flat_map(|record| record.inputs.iter().map(|input| input.target.clone())).collect());
        *self = Self { key, ..Self::from_records(records, reverts, originals) };
    }

    /// 🧮️ The ledger of decoded `Supersede` `records` (any order; sorted into fold order here, `role` and `entry`
    /// recomputed), each author's newest document-edit undo and the original input of every operation they name.
    pub fn from_records(mut records: Vec<SupersedeRecord>, reverts: HashMap<semio_framework_value::SharedUtf8, HybridLogicalTimestamp>, originals: HashMap<MutationId, protocol::InputReplacement>) -> Self {
        records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then_with(|| a.transition_id.cmp(&b.transition_id)));
        let index = records.iter().enumerate().map(|(index, record)| (record.transition_id.clone(), index)).collect();
        let mut ledger = Self { key: (0, None), records, index, originals, undo: HashMap::new(), redo: HashMap::new(), reverts };
        for index in 0..ledger.records.len() {
            ledger.classify(index);
        }
        ledger
    }

    /// 🎯️ The replacement effective for `target` over the first `before` records in the view of `scope`, with the record
    /// that installed it.
    pub fn effective(&self, before: usize, scope: Option<&str>, target: &MutationId) -> Option<(usize, &protocol::InputReplacement)> {
        self.records[..before]
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, record)| record.scope.is_none() || record.scope.as_deref() == scope)
            .find_map(|(index, record)| record.inputs.iter().find(|input| input.target == *target).map(|input| (index, &input.replacement)))
    }

    /// 👑️ The history edit whose input record `index` installs for `target`: the edit itself, a redo's re-authored edit,
    /// an undo's owner of the input it restored; `None` for a restored original input.
    pub fn owner(&self, index: usize, target: &MutationId) -> Option<usize> {
        let record = &self.records[index];
        match record.role {
            SupersedeRole::Edit => Some(index),
            SupersedeRole::Redo => Some(record.entry),
            SupersedeRole::Undo => self.effective(record.entry, self.records[record.entry].scope.as_deref(), target).and_then(|(installed, _)| self.owner(installed, target)),
        }
    }

    /// 👑️ The owner of `target`'s input over the first `before` records in the view of `scope`.
    fn owner_at(&self, before: usize, scope: Option<&str>, target: &MutationId) -> Option<usize> {
        self.effective(before, scope, target).and_then(|(installed, _)| self.owner(installed, target))
    }

    /// ✋️ The inputs history edit `entry` still owns over the first `before` records, in the view of its scope.
    pub fn held(&self, entry: usize, before: usize) -> Vec<&MutationId> {
        let scope = self.records[entry].scope.as_deref();
        self.records[entry].inputs.iter().map(|input| &input.target).filter(|target| self.owner_at(before, scope, target) == Some(entry)).collect()
    }

    /// ⏪️ The input `target` had before history edit `entry`: the effective one in the view of its scope, else the original.
    pub fn before(&self, entry: usize, target: &MutationId) -> Option<protocol::InputReplacement> {
        match self.effective(entry, self.records[entry].scope.as_deref(), target) {
            Some((_, replacement)) => Some(replacement.clone()),
            None => self.originals.get(target).cloned(),
        }
    }

    /// ⏩️ Whether the inputs undo `undo` restored still have, over the first `before` records, the owners it left them with.
    fn undo_stands(&self, undo: usize, before: usize) -> bool {
        let scope = self.records[undo].scope.as_deref();
        self.records[undo].inputs.iter().all(|input| self.owner_at(before, scope, &input.target) == self.owner(undo, &input.target))
    }

    /// 🔁️ Classifies record `index` against its author's stacks: the undo of a history edit that still owns exactly its
    /// targets and whose inputs before it it restores, else the redo of the newest undone edit whose undo still stands and
    /// whose inputs it re-authors, else a new history edit (clearing the author's redos).
    fn classify(&mut self, index: usize) {
        let record = &self.records[index];
        let targets: BTreeSet<&MutationId> = record.inputs.iter().map(|input| &input.target).collect();
        let undone = self.undo.get(&record.actor).and_then(|stack| {
            stack.iter().rev().copied().find(|&entry| {
                self.records[entry].scope == record.scope && self.held(entry, index).into_iter().collect::<BTreeSet<_>>() == targets && record.inputs.iter().all(|input| self.before(entry, &input.target).as_ref() == Some(&input.replacement))
            })
        });
        let redone = self.redo.get(&record.actor).and_then(|stack| stack.last().copied()).filter(|&(entry, undo)| {
            undone.is_none()
                && self.records[entry].scope == record.scope
                && self.records[undo].inputs.iter().map(|input| &input.target).collect::<BTreeSet<_>>() == targets
                && self.undo_stands(undo, index)
                && record.inputs.iter().all(|input| self.records[entry].inputs.iter().any(|edited| edited.target == input.target && edited.replacement == input.replacement))
        });
        let actor = record.actor.clone();
        let (role, entry) = match (undone, redone) {
            (Some(entry), _) => {
                if let Some(stack) = self.undo.get_mut(&actor) {
                    stack.retain(|held| *held != entry);
                }
                self.redo.entry(actor).or_default().push((entry, index));
                (SupersedeRole::Undo, entry)
            }
            (None, Some((entry, _))) => {
                if let Some(stack) = self.redo.get_mut(&actor) {
                    stack.pop();
                }
                self.undo.entry(actor).or_default().push(entry);
                (SupersedeRole::Redo, entry)
            }
            (None, None) => {
                self.redo.remove(&actor);
                self.undo.entry(actor).or_default().push(index);
                (SupersedeRole::Edit, index)
            }
        };
        self.records[index].role = role;
        self.records[index].entry = entry;
    }

    /// 🔎️ The record of transition `transition_id`, with its index.
    pub fn record(&self, transition_id: &str) -> Option<(usize, &SupersedeRecord)> {
        self.index.get(transition_id).map(|&index| (index, &self.records[index]))
    }

    /// ✅️ The history edits in effect now: the owners of the inputs the store folds with (`supersessions`, the effective
    /// supersessions of the final alternative).
    pub fn applied_entries<'a>(&self, supersessions: impl IntoIterator<Item = (&'a MutationId, &'a protocol::EffectiveSupersession)>) -> HashSet<usize> {
        supersessions.into_iter().filter_map(|(target, supersession)| self.index.get(supersession.transition_id.as_str()).and_then(|&index| self.owner(index, target))).collect()
    }

    /// ✅️ Whether the history row of record `index` reads as applied: an edit or a redo while its edit is in effect, an
    /// undo while the edit it undid is not redone.
    pub fn row_applied(&self, index: usize, applied: &HashSet<usize>) -> bool {
        let record = &self.records[index];
        match record.role {
            SupersedeRole::Undo => !self.undo.get(&record.actor).is_some_and(|stack| stack.contains(&record.entry)),
            SupersedeRole::Edit | SupersedeRole::Redo => applied.contains(&record.entry),
        }
    }

    /// ⏪️ Whether `actor` may take history edit `entry` back now: it is theirs, not undone, and in effect.
    pub fn undoable(&self, actor: &str, entry: usize, applied: &HashSet<usize>) -> bool {
        applied.contains(&entry) && self.undo.get(actor).is_some_and(|stack| stack.contains(&entry))
    }

    /// ⏪️ The undo of history edit `entry`: its scope and each input it still owns, in the view of that scope, back to
    /// the input before it (the original input when there was none).
    pub fn restore(&self, entry: usize) -> (Option<String>, Vec<protocol::SupersededInput>) {
        let inputs = self.held(entry, self.records.len()).into_iter().filter_map(|target| self.before(entry, target).map(|replacement| protocol::SupersededInput { target: target.clone(), replacement })).collect();
        (self.records[entry].scope.clone(), inputs)
    }

    /// ⏩️ `actor`'s next redo: the newest undone history edit and its undo, while that undo still stands.
    pub fn redo_top(&self, actor: &str) -> Option<(usize, usize)> {
        self.redo.get(actor).and_then(|stack| stack.last().copied()).filter(|&(_, undo)| self.undo_stands(undo, self.records.len()))
    }

    /// ⏩️ The redo of history edit `entry` undone by `undo`: its scope and its own input for every input the undo restored.
    pub fn redo(&self, entry: usize, undo: usize) -> (Option<String>, Vec<protocol::SupersededInput>) {
        let inputs = self.records[undo].inputs.iter().filter_map(|restored| self.records[entry].inputs.iter().find(|edited| edited.target == restored.target).cloned()).collect();
        (self.records[entry].scope.clone(), inputs)
    }

    /// ⏱️ `actor`'s newest undo of a document edit.
    pub fn newest_revert(&self, actor: &str) -> Option<HybridLogicalTimestamp> {
        self.reverts.get(actor).copied()
    }

    /// 🏷️ The history row of record `index` in every locale: its role, its scope (an overwrite, or the alternative
    /// `alternative` names) and how many mutations it supersedes.
    pub fn label(&self, index: usize, alternative: Option<&str>) -> LocalizedLabel {
        let record = &self.records[index];
        let count = record.inputs.len();
        let (en_role, de_role) = match record.role {
            SupersedeRole::Edit => ("History edited", "Verlauf bearbeitet"),
            SupersedeRole::Undo => ("History edit undone", "Verlaufsbearbeitung rückgängig"),
            SupersedeRole::Redo => ("History edit redone", "Verlaufsbearbeitung wiederhergestellt"),
        };
        let (en_scope, de_scope) = match (&record.scope, alternative) {
            (None, _) => ("overwrite".to_string(), "überschrieben".to_string()),
            (Some(_), Some(name)) => (format!("alternative {name}"), format!("Alternative {name}")),
            (Some(id), None) => (format!("alternative {id}"), format!("Alternative {id}")),
        };
        let (en_count, de_count) = if count == 1 { ("1 mutation".to_string(), "1 Mutation".to_string()) } else { (format!("{count} mutations"), format!("{count} Mutationen")) };
        LocalizedLabel::native(&format!("{en_role} — {en_scope}: {en_count}"), &format!("{de_role} — {de_scope}: {de_count}"))
    }
}
//#endregion 🔖️Supersessions
