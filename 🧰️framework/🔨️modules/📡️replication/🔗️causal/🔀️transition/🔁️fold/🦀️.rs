//! ⏳️ Granted history folding with cooperative decoding and exact cancellation retirement.

use super::*;
use semio_framework_value::{ErasedSnapshotRetirement, SnapshotRetirementStep};
use semio_framework_value::retirement::{RetireOwned, owned_retirement};
use std::{collections::{BTreeMap, BTreeSet, VecDeque}, future::Future, ops::{Deref, DerefMut}, pin::Pin, sync::{Arc, Mutex, atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering}}, task::{Context, Poll, Wake, Waker}};

struct FoldControlState {
    cancelled: AtomicBool,
    completed: AtomicU64,
    page_bytes: AtomicUsize,
    retirements: Mutex<VecDeque<Box<dyn ErasedSnapshotRetirement>>>,
}

/// ⏱️ The shared pulse and retirement authority of one immutable-source fold.
#[derive(Clone)]
pub struct HistoryFoldControl(Arc<FoldControlState>);

impl HistoryFoldControl {
    /// 🌱️ Creates fixed retirement queue capacity without inspecting any history.
    pub fn new() -> Self { Self(Arc::new(FoldControlState { cancelled: AtomicBool::new(false), completed: AtomicU64::new(0), page_bytes: AtomicUsize::new(4096), retirements: Mutex::new(VecDeque::with_capacity(128)) })) }
    /// 📊️ Completed cooperative work units, excluding cancellation retirement.
    pub fn completed(&self) -> u64 { self.0.completed.load(Ordering::Relaxed) }
    /// ♻️ Transfers an owned local to exact retirement when its coroutine scope ends.
    pub fn track<T: RetireOwned>(&self, value: T) -> HistoryFoldOwned<T> { HistoryFoldOwned { value: Some(value), control: self.clone() } }
    /// ⏭️ One pulse before each bounded work unit; cancellation is checked before publication.
    pub async fn pulse(&self) -> Result<(), crate::ProtocolError> { FoldPulse { control: self, yielded: false }.await }
    fn page_bytes(&self) -> usize { self.0.page_bytes.load(Ordering::Relaxed).clamp(1, 4096) }
    fn retire<T: RetireOwned>(&self, value: T) { self.0.retirements.lock().expect("fold retirement authority remains available").push_back(owned_retirement(value)); }
    fn empty(&self) -> bool { self.0.retirements.lock().expect("fold retirement authority remains available").is_empty() }
    fn close_one(&self, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        let mut queue = self.0.retirements.lock().expect("fold retirement authority remains available");
        let Some(owner) = queue.front_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        match owner.close_step(1, maximum_bytes)? {
            SnapshotRetirementStep::Complete if owner.terminal_is_empty() => { queue.pop_front(); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }) }
            SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "fold retirement reported a false terminal")),
            step => Ok(step),
        }
    }
}

impl Default for HistoryFoldControl { fn default() -> Self { Self::new() } }

/// 🧳️ Coroutine-local owned values enqueue their registered retirement instead of deep-dropping on cancellation.
pub struct HistoryFoldOwned<T: RetireOwned> { value: Option<T>, control: HistoryFoldControl }
impl<T: RetireOwned> HistoryFoldOwned<T> { pub fn take(mut self) -> T { self.value.take().expect("fold local remains owned") } }
impl<T: RetireOwned> Deref for HistoryFoldOwned<T> { type Target = T; fn deref(&self) -> &T { self.value.as_ref().expect("fold local remains owned") } }
impl<T: RetireOwned> DerefMut for HistoryFoldOwned<T> { fn deref_mut(&mut self) -> &mut T { self.value.as_mut().expect("fold local remains owned") } }
impl<T: RetireOwned> Drop for HistoryFoldOwned<T> { fn drop(&mut self) { if let Some(value) = self.value.take() { self.control.retire(value); } } }

struct FoldPulse<'a> { control: &'a HistoryFoldControl, yielded: bool }
impl Future for FoldPulse<'_> {
    type Output = Result<(), crate::ProtocolError>;
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.control.0.cancelled.load(Ordering::Acquire) { return Poll::Ready(Err(fold_error("history fold cancelled"))); }
        if !self.yielded { self.yielded = true; return Poll::Pending; }
        self.control.0.completed.fetch_add(1, Ordering::Relaxed);
        Poll::Ready(Ok(()))
    }
}

struct FoldWake;
impl Wake for FoldWake { fn wake(self: Arc<Self>) {} }

/// 📊️ One granted step of a history fold, including the retirement required before handoff.
pub enum HistoryFoldJobStep<T = HistoryFold> { Pending { completed: u64 }, Ready(T), Rejected(crate::ProtocolError) }

/// 🧮️ A pinned fold state machine; its immutable input is captured by the caller-owned runner.
pub struct HistoryFoldJob<'a, T: RetireOwned = HistoryFold> {
    control: HistoryFoldControl,
    future: Option<Pin<Box<dyn Future<Output = Result<T, crate::ProtocolError>> + Send + 'a>>>,
    result: Option<Result<T, crate::ProtocolError>>,
    waker: Waker,
    terminal: bool,
}

impl<'a, T: RetireOwned> HistoryFoldJob<'a, T> {
    /// 🌱️ Captures immutable owners without scanning their contents.
    pub fn new<F: Future<Output = Result<T, crate::ProtocolError>> + Send + 'a>(runner: impl FnOnce(HistoryFoldControl) -> F) -> Self {
        let control = HistoryFoldControl::new();
        let future = Box::pin(runner(control.clone()));
        Self { control, future: Some(future), result: None, waker: Waker::from(Arc::new(FoldWake)), terminal: false }
    }
    /// 📊️ Completed semantic and decoding work units.
    pub fn completed(&self) -> u64 { self.control.completed() }
    /// ⏭️ Polls at most one coroutine work unit or one exact retired child per granted item.
    pub fn step(&mut self, maximum_items: usize, maximum_bytes: usize, should_yield: &mut impl FnMut() -> bool) -> Result<HistoryFoldJobStep<T>, semio_framework_value::ValueError> {
        self.control.0.page_bytes.store(maximum_bytes.clamp(1,4096), Ordering::Relaxed);
        if maximum_bytes == 0 { return Ok(HistoryFoldJobStep::Pending { completed: self.completed() }); }
        for _ in 0..maximum_items {
            if should_yield() { break; }
            if !self.control.empty() { self.control.close_one(maximum_bytes)?; continue; }
            let Some(future) = self.future.as_mut() else { break };
            if let Poll::Ready(result) = future.as_mut().poll(&mut Context::from_waker(&self.waker)) { self.result = Some(result); self.future.take(); }
        }
        if self.future.is_none() && self.control.empty() {
            if let Some(result) = self.result.take() { self.terminal = true; return Ok(match result { Ok(fold) => HistoryFoldJobStep::Ready(fold), Err(error) => HistoryFoldJobStep::Rejected(error) }); }
        }
        Ok(HistoryFoldJobStep::Pending { completed: self.completed() })
    }
    /// 🧊️ Runs the same cursor to completion for explicit cold callers; interactive owners grant individual turns.
    pub fn finish_cold(mut self) -> Result<T, crate::ProtocolError> {
        loop {
            match self.step(64, 4096, &mut || false).expect("registered history owners must retire within their grants") {
                HistoryFoldJobStep::Pending { .. } => {},
                HistoryFoldJobStep::Ready(value) => return Ok(value),
                HistoryFoldJobStep::Rejected(error) => return Err(error),
            }
        }
    }

    /// 🛑️ Cancels without executing further fold work; tracked locals transfer to exact retirement.
    pub fn request_cancel(&mut self) {
        self.control.0.cancelled.store(true, Ordering::Release);
        self.future.take();
        if let Some(Ok(fold)) = self.result.take() { self.control.retire(fold); }
    }
}

impl<T: RetireOwned> ErasedSnapshotRetirement for HistoryFoldJob<'_, T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        self.request_cancel();
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if !self.control.empty() { return self.control.close_one(maximum_bytes); }
        self.terminal = true;
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.terminal && self.future.is_none() && self.result.is_none() && self.control.empty() }
}
impl<T: RetireOwned> Drop for HistoryFoldJob<'_, T> { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "history fold job released before exact handoff or retirement"); } }

/// 📝️ Copies verified UTF-8 in fixed pages, yielding before each page allocation or write.
pub async fn copy_history_text(text: &str, control: &HistoryFoldControl) -> Result<String, crate::ProtocolError> { copy_history_text_parts(&[text], control).await }

pub async fn copy_history_text_parts(parts: &[&str], control: &HistoryFoldControl) -> Result<String, crate::ProtocolError> {
    let mut copied = control.track(String::with_capacity(parts.iter().map(|part| part.len()).sum()));

    let mut scalar = [0;4];
    let mut width = 0;
    let mut needed = 0;
    control.pulse().await?;
    for text in parts {
    let mut at = 0;
    while at < text.len() {
        control.pulse().await?;
        let end = (at + control.page_bytes()).min(text.len());
        for &byte in &text.as_bytes()[at..end] {
            if width == 0 { needed = match byte { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, _ => 4 }; }
            scalar[width] = byte; width += 1;
            if width == needed { copied.push_str(std::str::from_utf8(&scalar[..width]).expect("source text preserves complete UTF-8")); width = 0; }
        }
        at = end;
    }
    }
    Ok(copied.take())
}

async fn read_fold_string(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<String, crate::ProtocolError> {
    control.pulse().await?;
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let length = usize::try_from(count).map_err(|_| malformed(*pos, "string length exceeds address space"))?;
    let end = pos.checked_add(length).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(*pos, "truncated string"))?;
    let mut text = control.track(String::with_capacity(length));
    let mut scalar = [0; 4];
    let mut width = 0;
    let mut needed = 0;
    while *pos < end {
        control.pulse().await?;
        let limit = (*pos + control.page_bytes()).min(end);
        while *pos < limit {
            let byte = bytes[*pos];
            if width == 0 { needed = match byte { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => return Err(malformed(*pos, "invalid UTF-8")) }; }
            scalar[width] = byte;
            width += 1;
            *pos += 1;
            if width == needed { let value = std::str::from_utf8(&scalar[..width]).map_err(|_| malformed(*pos-width, "invalid UTF-8"))?; text.push_str(value); width = 0; }
        }
    }
    if width != 0 { return Err(malformed(*pos, "truncated UTF-8")); }
    Ok(text.take())
}

async fn read_fold_optional(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Option<String>, crate::ProtocolError> {
    control.pulse().await?;
    match read_u8(bytes, pos)? { 0 => Ok(None), 1 => read_fold_string(bytes, pos, control).await.map(Some), other => Err(malformed(*pos, format!("invalid option tag {other}"))) }
}

async fn read_fold_ids(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Vec<MutationId>, crate::ProtocolError> {
    control.pulse().await?;
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    if count > bytes.len().saturating_sub(*pos) as u64 { return Err(malformed(*pos, "id count exceeds payload")); }
    let mut ids = control.track(Vec::with_capacity(count as usize));
    for _ in 0..count { ids.push(MutationId(read_fold_string(bytes, pos, control).await?)); }
    Ok(ids.take())
}

async fn read_fold_payload(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Vec<u8>, crate::ProtocolError> {
    control.pulse().await?;
    let length = crate::wire::read_varint_u64(bytes, pos)?;
    if length > SUPERSEDE_PAYLOAD_MAX_BYTES as u64 { return Err(malformed(*pos, format!("supersede payload exceeds {SUPERSEDE_PAYLOAD_MAX_BYTES} bytes"))); }
    let end = pos.checked_add(length as usize).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(*pos, "truncated replacement payload"))?;
    let mut payload = control.track(Vec::with_capacity(length as usize));
    while *pos < end { control.pulse().await?; let limit = (*pos+control.page_bytes()).min(end); payload.extend_from_slice(&bytes[*pos..limit]); *pos = limit; }
    Ok(payload.take())
}

/// 🔀️ Decodes every transition field and repeated member cooperatively; malformed and cancelled partial owners retire.
pub async fn decode_history_transition_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<HistoryTransition, crate::ProtocolError> {
    let mut pos = 0;
    control.pulse().await?;
    let transition = match crate::wire::read_varint_u64(bytes, &mut pos)? {
        0 => HistoryTransition::Revert { mutation_ids: read_fold_ids(bytes, &mut pos, control).await? },
        1 => HistoryTransition::Reinstate { mutation_ids: read_fold_ids(bytes, &mut pos, control).await? },
        2 => {
            let checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let parent_id = control.track(read_fold_optional(bytes, &mut pos, control).await?);
            let change_id = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let mutation_ids = control.track(read_fold_ids(bytes, &mut pos, control).await?);
            let description = control.track(read_fold_optional(bytes, &mut pos, control).await?);
            let saved_at = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "author count exceeds payload")); }
            let mut authors = control.track(Vec::with_capacity(count as usize));
            for _ in 0..count {
                let id = control.track(read_fold_string(bytes, &mut pos, control).await?);
                let name = control.track(read_fold_string(bytes, &mut pos, control).await?);
                let avatar = read_fold_optional(bytes, &mut pos, control).await?;
                authors.push(TransitionAuthor { id: id.take(), name: name.take(), avatar });
            }
            let message = control.track(read_fold_optional(bytes, &mut pos, control).await?);
            let timestamp = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let line_id = read_fold_optional(bytes, &mut pos, control).await?;
            HistoryTransition::Commit(TransitionCheckpoint { checkpoint_id: checkpoint_id.take(), parent_id: parent_id.take(), change_id: change_id.take(), mutation_ids: mutation_ids.take(), description: description.take(), saved_at: saved_at.take(), authors: authors.take(), message: message.take(), timestamp: timestamp.take(), line_id })
        }
        3 => {
            let alternative_id = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let name = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let checkpoint_id = read_fold_string(bytes, &mut pos, control).await?;
            HistoryTransition::Branch { alternative_id: alternative_id.take(), name: name.take(), checkpoint_id }
        }
        4 => { let checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?); let alternative_id = read_fold_optional(bytes, &mut pos, control).await?; HistoryTransition::Checkout { checkpoint_id: checkpoint_id.take(), alternative_id } }
        5 => {
            let checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let pinned_checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?);
            let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "pin count exceeds payload")); }
            let mut pins = control.track(Vec::with_capacity(count as usize));
            for _ in 0..count { let child_uri = control.track(read_fold_string(bytes, &mut pos, control).await?); let checkpoint_id = read_fold_string(bytes, &mut pos, control).await?; pins.push(TransitionPin { child_uri: child_uri.take(), checkpoint_id }); }
            HistoryTransition::Repin { checkpoint_id: checkpoint_id.take(), pinned_checkpoint_id: pinned_checkpoint_id.take(), pins: pins.take() }
        }
        6 => {
            let scope = control.track(read_fold_optional(bytes, &mut pos, control).await?);
            if scope.as_ref().is_some_and(|scope| scope.len() > SUPERSEDE_SCOPE_MAX_BYTES) { return Err(malformed(pos, format!("supersede scope exceeds {SUPERSEDE_SCOPE_MAX_BYTES} bytes"))); }
            let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if count == 0 { return Err(malformed(pos, "supersede names no input")); }
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "input count exceeds payload")); }
            let mut inputs = control.track(Vec::with_capacity(count as usize));
            let mut seen = control.track(BTreeSet::new());
            for _ in 0..count {
                let target = control.track(MutationId(read_fold_string(bytes, &mut pos, control).await?));
                if !seen.insert(copy_history_text(&target.0, control).await?) { return Err(malformed(pos, format!("supersede repeats target {}", target.0))); }
                let replacement = match read_u8(bytes, &mut pos)? {
                    0 => { let schema = control.track(read_fold_string(bytes, &mut pos, control).await?); let payload = read_fold_payload(bytes, &mut pos, control).await?; InputReplacement::Input { schema: schema.take(), payload } }
                    1 => InputReplacement::Withdrawn,
                    tag => return Err(malformed(pos-1, format!("invalid replacement tag {tag}"))),
                };
                inputs.push(SupersededInput { target: target.take(), replacement });
            }
            HistoryTransition::Supersede(TransitionSupersede { scope: scope.take(), inputs: inputs.take() })
        }
        tag => return Err(malformed(0, format!("unknown transition tag {tag}"))),
    };
    let transition = control.track(transition);
    if pos != bytes.len() { return Err(malformed(pos, "trailing bytes")); }
    Ok(transition.take())
}

async fn owned_positions(ids: &[MutationId], owners: &BTreeMap<String, usize>, control: &HistoryFoldControl) -> Result<Vec<usize>, crate::ProtocolError> {
    let mut positions = control.track(Vec::with_capacity(ids.len()));
    let mut seen = control.track(BTreeSet::new());
    for id in ids { control.pulse().await?; let position = *owners.get(&id.0).ok_or_else(|| fold_error(format!("transition references unknown operation {}", id.0)))?; if seen.insert(position) { positions.push(position); } }
    Ok(positions.take())
}

/// 🧮️ The shared semantic fold, cooperatively ordered and indexed without whole-history collect, sort or retain.
pub async fn fold_history_for_controlled(document_id: &ArtifactId, edits: &[FoldEdit], transitions: &[super::super::MutationEnvelope], excluded: &std::collections::HashSet<String>, head: &ViewerHead, control: &HistoryFoldControl) -> Result<HistoryFold, crate::ProtocolError> {
    let mut owners = control.track(BTreeMap::<String, usize>::new());
    let mut identities = control.track(BTreeSet::<String>::new());
    let mut events = control.track(BTreeMap::new());
    for (index, edit) in edits.iter().enumerate() {
        let id = control.track(copy_history_text(&edit.id, control).await?);
        if !identities.insert(copy_history_text(&id, control).await?) { return Err(fold_error("history repeats an edit")); }
        for mutation in &edit.mutation_ids { owners.insert(copy_history_text(&mutation.0, control).await?, index); }
        events.insert((edit.timestamp.cmp_key(), id.take()), (false, index));
    }
    for (index, event) in transitions.iter().enumerate() {
        let id = control.track(copy_history_text(&event.mutation_id.0, control).await?);
        if !identities.insert(copy_history_text(&id, control).await?) { return Err(fold_error(format!("history repeats transition {}", id.as_str()))); }
        if event.diff.schema.0 != HISTORY_TRANSITION_SCHEMA { return Err(fold_error(format!("{} is not a history transition", id.as_str()))); }
        events.insert((event.timestamp.cmp_key(), id.take()), (true, index));
    }
    control.pulse().await?;
    let mut fold = control.track(HistoryFold { trunk: trunk_alternative_id(document_id), applied: Vec::with_capacity(edits.len()), redo: Vec::with_capacity(edits.len()), refused: Vec::with_capacity(transitions.len()), changes: Vec::with_capacity(transitions.len()), checkpoints: Vec::with_capacity(transitions.len()), alternatives: Vec::with_capacity(transitions.len()+1), ..HistoryFold::default() });
    let mut trunk_chain = control.track(Vec::<String>::with_capacity(transitions.len()));
    let mut active = control.track(BTreeSet::<usize>::new());
    let mut redo = control.track(BTreeMap::<u64, usize>::new());
    let mut redo_lookup = control.track(BTreeMap::<usize, u64>::new());
    let mut redo_actors = control.track(BTreeMap::<Option<String>, BTreeSet<u64>>::new());
    let mut renamed = control.track(BTreeMap::<String, String>::new());
    let mut checkpoint_lookup = control.track(BTreeMap::<String, usize>::new());
    let mut change_lookup = control.track(BTreeMap::<String, usize>::new());
    let mut alternative_lookup = control.track(BTreeMap::<String, usize>::new());
    let mut edit_lookup = control.track(BTreeMap::<String, usize>::new());
    let mut serial = 0;
    for (index, edit) in edits.iter().enumerate() { edit_lookup.insert(copy_history_text(&edit.id, control).await?, index); }
    while let Some((key, (structural, index))) = events.pop_first() {
        let key = control.track(key);
        control.pulse().await?;
        if !structural {
            let edit = &edits[index];
            if excluded.contains(&edit.id) { continue; }
            active.insert(index);
            while let Some(position) = redo_actors.get_mut(&edit.actor).and_then(BTreeSet::pop_first) { control.pulse().await?; if let Some(index) = redo.remove(&position) { redo_lookup.remove(&index); } }
            continue;
        }
        let event = &transitions[index];
        let mut transition = control.track(decode_history_transition_controlled(&event.diff.payload, control).await?);
        let reinstate = matches!(&*transition, HistoryTransition::Reinstate { .. });
        match &mut *transition {
            HistoryTransition::Revert { mutation_ids } | HistoryTransition::Reinstate { mutation_ids } => {
                let positions = control.track(owned_positions(mutation_ids, &owners, control).await?);
                let mut foreign = false;
                for position in positions.iter() { control.pulse().await?; foreign |= edits[*position].actor.as_deref().is_some_and(|actor| actor != event.actor.0); }
                if foreign { fold.refused.push(copy_history_text(&event.mutation_id.0, control).await?); continue; }
                for position in positions.iter().copied() {
                    control.pulse().await?;
                    if reinstate {
                        if let Some(order) = redo_lookup.remove(&position) { redo.remove(&order); if let Some(orders) = redo_actors.get_mut(&edits[position].actor) { orders.remove(&order); } active.insert(position); }
                    } else if active.remove(&position) {
                        redo.insert(serial, position);
                        redo_lookup.insert(position, serial);
                        let actor = match &edits[position].actor { Some(actor) => Some(copy_history_text(actor, control).await?), None => None };
                        redo_actors.entry(actor).or_default().insert(serial);
                        serial += 1;
                    }
                }
            }
            HistoryTransition::Commit(checkpoint) => {
                let parent = match &checkpoint.parent_id { Some(parent) => Some(*checkpoint_lookup.get(parent).ok_or_else(|| fold_error(format!("checkpoint {} names unknown parent {parent}", checkpoint.checkpoint_id)))?), None => None };
                let count = parent.map_or(0, |parent| fold.checkpoints[parent].change_ids.len());
                let mut change_ids = control.track(Vec::with_capacity(count+1));
                if let Some(parent) = parent { for change in &fold.checkpoints[parent].change_ids { change_ids.push(copy_history_text(change, control).await?); } }
                change_ids.push(copy_history_text(&checkpoint.change_id, control).await?);
                let positions = control.track(owned_positions(&checkpoint.mutation_ids, &owners, control).await?);
                let mut edit_ids = control.track(Vec::with_capacity(positions.len()));
                for position in positions.iter() { edit_ids.push(copy_history_text(&edits[*position].id, control).await?); }
                change_lookup.entry(copy_history_text(&checkpoint.change_id, control).await?).or_insert(fold.changes.len());
                checkpoint_lookup.entry(copy_history_text(&checkpoint.checkpoint_id, control).await?).or_insert(fold.checkpoints.len());
                let checkpoint_id = control.track(copy_history_text(&checkpoint.checkpoint_id, control).await?);
                let line = checkpoint.line_id.as_ref().filter(|line| *line != &fold.trunk);
                match line {
                    Some(line) => { let index = *alternative_lookup.get(line).ok_or_else(|| fold_error(format!("commit {} names unknown alternative {line}", checkpoint.checkpoint_id)))?; fold.alternatives[index].checkpoint_ids.push(copy_history_text(&checkpoint_id, control).await?); }
                    None => trunk_chain.push(copy_history_text(&checkpoint_id, control).await?),
                }
                fold.changes.push(FoldChange { id: std::mem::take(&mut checkpoint.change_id), edit_ids: edit_ids.take(), description: checkpoint.description.take(), saved_at: std::mem::take(&mut checkpoint.saved_at) });
                fold.checkpoints.push(FoldCheckpoint { id: std::mem::take(&mut checkpoint.checkpoint_id), change_ids: change_ids.take(), parent_id: checkpoint.parent_id.take(), authors: std::mem::take(&mut checkpoint.authors), message: checkpoint.message.take(), timestamp: std::mem::take(&mut checkpoint.timestamp), pins: Vec::new() });
            }
            HistoryTransition::Branch { alternative_id, name, checkpoint_id } => {
                if *alternative_id == fold.trunk { return Err(fold_error(format!("branch claims the trunk alternative {alternative_id}"))); }
                if !checkpoint_lookup.contains_key(checkpoint_id) { return Err(fold_error(format!("branch names unknown checkpoint {checkpoint_id}"))); }
                alternative_lookup.entry(copy_history_text(alternative_id, control).await?).or_insert(fold.alternatives.len());
                fold.alternatives.push(FoldAlternative { id: std::mem::take(alternative_id), name: std::mem::take(name), checkpoint_ids: vec![std::mem::take(checkpoint_id)] });
            }
            HistoryTransition::Checkout { checkpoint_id, .. } => { if !checkpoint_lookup.contains_key(checkpoint_id) { return Err(fold_error(format!("checkout names unknown checkpoint {checkpoint_id}"))); } }
            HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins } => {
                let index = *checkpoint_lookup.get(checkpoint_id).ok_or_else(|| fold_error(format!("repin names unknown checkpoint {checkpoint_id}")))?;
                control.track(std::mem::replace(&mut fold.checkpoints[index].id, copy_history_text(pinned_checkpoint_id, control).await?));
                control.track(std::mem::replace(&mut fold.checkpoints[index].pins, std::mem::take(pins)));
                checkpoint_lookup.remove(checkpoint_id);
                checkpoint_lookup.insert(copy_history_text(pinned_checkpoint_id, control).await?, index);
                for alternative in &mut fold.alternatives { for id in &mut alternative.checkpoint_ids { control.pulse().await?; if *id == *checkpoint_id { control.track(std::mem::replace(id, copy_history_text(pinned_checkpoint_id, control).await?)); } } }
                for id in trunk_chain.iter_mut() { control.pulse().await?; if *id == *checkpoint_id { control.track(std::mem::replace(id, copy_history_text(pinned_checkpoint_id, control).await?)); } }
                if let Some(previous) = renamed.insert(std::mem::take(checkpoint_id), std::mem::take(pinned_checkpoint_id)) { control.track(previous); }
            }
            HistoryTransition::Supersede(supersede) => {
                for input in &supersede.inputs { control.pulse().await?; if !owners.contains_key(&input.target.0) { return Err(fold_error(format!("transition references unknown operation {}", input.target.0))); } }
                if supersede.scope.as_ref().is_none_or(|scope| *scope == head.line_id) {
                    for input in &mut supersede.inputs {
                        control.pulse().await?;
                        let actor = control.track(copy_history_text(&event.actor.0, control).await?);
                        let transition_id = control.track(copy_history_text(&event.mutation_id.0, control).await?);
                        let scope = match &supersede.scope { Some(scope) => Some(copy_history_text(scope, control).await?), None => None };
                        let target = MutationId(std::mem::take(&mut input.target.0));
                        let replacement = std::mem::replace(&mut input.replacement, InputReplacement::Withdrawn);
                        if let Some(previous) = fold.supersessions.insert(target, EffectiveSupersession { transition_id: transition_id.take(), actor: actor.take(), timestamp: event.timestamp, scope, replacement }) { control.track(previous); }
                    }
                }
            }
        }
    }
    let on_trunk = head.line_id == fold.trunk;
    let mut chain = control.track(Vec::new());
    let source = if on_trunk { &*trunk_chain } else { let index = *alternative_lookup.get(&head.line_id).ok_or_else(|| fold_error(format!("viewer head names unknown alternative {}", head.line_id)))?; &fold.alternatives[index].checkpoint_ids };
    chain.reserve_exact(source.len());
    for id in source { chain.push(copy_history_text(id, control).await?); }
    let mut viewed = control.track(match &head.checkpoint_id { Some(id) => Some(copy_history_text(id, control).await?), None => None });
    if let Some(id) = viewed.as_mut() {
        let mut guard = 0;
        while let Some(next) = renamed.get(id) { control.pulse().await?; control.track(std::mem::replace(id, copy_history_text(next, control).await?)); guard += 1; if guard == 64 { return Err(fold_error("repin cycle")); } }
        let mut known = false;
        for candidate in chain.iter() { control.pulse().await?; known |= candidate == id; }
        if !known { return Err(fold_error(format!("viewer head names unknown checkpoint {id}"))); }
    }
    let at_tip = viewed.is_none();
    let checkpoint = match viewed.take() { Some(id) => Some(id), None => match chain.last() { Some(id) => Some(copy_history_text(id, control).await?), None => None } };
    let checkpoint = control.track(checkpoint);
    let mut visible = control.track(BTreeSet::<usize>::new());
    if let Some(id) = checkpoint.as_ref() {
        let position = *checkpoint_lookup.get(id).ok_or_else(|| fold_error(format!("viewer head names unknown checkpoint {id}")))?;
        for change_id in &fold.checkpoints[position].change_ids {
            control.pulse().await?;
            let change = *change_lookup.get(change_id).ok_or_else(|| fold_error(format!("checkpoint {id} names unknown change {change_id}")))?;
            for edit_id in &fold.changes[change].edit_ids { control.pulse().await?; if let Some(position) = edit_lookup.get(edit_id) { if active.contains(position) { visible.insert(*position); } } }
        }
    }
    if at_tip {
        let mut committed = control.track(BTreeSet::<usize>::new());
        for change in &fold.changes { for edit_id in &change.edit_ids { control.pulse().await?; if let Some(position) = edit_lookup.get(edit_id) { committed.insert(*position); } } }
        for (position, edit) in edits.iter().enumerate() { control.pulse().await?; if !active.contains(&position) || committed.contains(&position) { continue; } let on_line = match &edit.line { Some(line) if line != &fold.trunk => line == &head.line_id, _ => on_trunk }; if on_line { visible.insert(position); } }
    }
    let mut ordered = control.track(BTreeMap::new());
    for position in visible.iter().copied() { let edit = &edits[position]; ordered.insert((edit.timestamp.cmp_key(), copy_history_text(&edit.id, control).await?), position); }
    while let Some((key, position)) = ordered.pop_first() { control.track(key); fold.applied.push(copy_history_text(&edits[position].id, control).await?); }
    for position in redo.values() { fold.redo.push(copy_history_text(&edits[*position].id, control).await?); }
    if !trunk_chain.is_empty() { let trunk = copy_history_text(&fold.trunk, control).await?; fold.alternatives.push(FoldAlternative { id: trunk, name: String::new(), checkpoint_ids: trunk_chain.take() }); for index in (1..fold.alternatives.len()).rev() { control.pulse().await?; fold.alternatives.swap(index, index-1); } }
    fold.checkpoint = checkpoint.take();
    fold.alternative = if on_trunk { None } else { Some(copy_history_text(&head.line_id, control).await?) };
    Ok(fold.take())
}

/// 📄️ Copies immutable protocol bytes through the same granted decoder pages.
pub async fn copy_history_bytes(bytes: &[u8], control: &HistoryFoldControl) -> Result<Vec<u8>, crate::ProtocolError> {
    let mut copied = control.track(Vec::with_capacity(bytes.len()));
    let mut at = 0;
    control.pulse().await?;
    while at < bytes.len() { control.pulse().await?; let end = (at+control.page_bytes()).min(bytes.len()); copied.extend_from_slice(&bytes[at..end]); at = end; }
    Ok(copied.take())
}

/// 🪪️ Validates an opaque quarantined envelope while copying only its operation identity.
pub async fn history_envelope_id_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<MutationId, crate::ProtocolError> {
    let mut pos = 0;
    let identity = control.track(MutationId(read_fold_string(bytes, &mut pos, control).await?));
    control.track(read_fold_string(bytes, &mut pos, control).await?);
    control.track(read_fold_string(bytes, &mut pos, control).await?);
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "dependency count exceeds envelope")); }
    for _ in 0..count { control.track(read_fold_string(bytes, &mut pos, control).await?); }
    match crate::wire::read_varint_u64(bytes, &mut pos)? { 0 => {}, 1 => { control.track(read_fold_string(bytes, &mut pos, control).await?); }, flag => return Err(malformed(pos, format!("observed flag {flag}"))) }
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "target count exceeds envelope")); }
    for _ in 0..count { control.track(read_fold_string(bytes, &mut pos, control).await?); }
    for _ in 0..2 {
        control.track(read_fold_string(bytes, &mut pos, control).await?);
        control.pulse().await?;
        let length = crate::wire::read_varint_u64(bytes, &mut pos)?;
        pos = usize::try_from(length).ok().and_then(|length| pos.checked_add(length)).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(pos, "truncated envelope payload"))?;
    }
    control.pulse().await?;
    super::super::decode_hlc(bytes, &mut pos)?;
    let flags = crate::wire::read_varint_u64(bytes, &mut pos)?;
    if flags > 0b111 { return Err(malformed(pos, "invalid envelope flags")); }
    if flags & 1 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?); control.track(read_fold_string(bytes, &mut pos, control).await?); }
    if flags & 2 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?); }
    if flags & 4 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?); }
    if pos != bytes.len() { return Err(malformed(pos, "quarantined envelope has trailing bytes")); }
    Ok(identity.take())
}

async fn read_fold_blob(bytes: &[u8], position: &mut usize, control: &HistoryFoldControl) -> Result<Vec<u8>, crate::ProtocolError> {
    control.pulse().await?;
    let length = usize::try_from(crate::wire::read_varint_u64(bytes, position)?).map_err(|_| malformed(*position, "envelope payload length exceeds address space"))?;
    let end = position.checked_add(length).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(*position, "truncated envelope payload"))?;
    let copied = copy_history_bytes(&bytes[*position..end], control).await?;
    *position = end;
    Ok(copied)
}

/// 📦️ Decodes quarantined envelopes in byte pages, preserving every opaque payload and exact header fact.
pub async fn decode_history_envelope_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<super::super::MutationEnvelope, crate::ProtocolError> {
    let mut position = 0;
    let mutation_id = control.track(MutationId(read_fold_string(bytes, &mut position, control).await?));
    let document_id = control.track(ArtifactId(read_fold_string(bytes, &mut position, control).await?));
    let actor = control.track(ActorId(read_fold_string(bytes, &mut position, control).await?));
    let dependencies = control.track(read_fold_ids(bytes, &mut position, control).await?);
    control.pulse().await?;
    let observed = control.track(match crate::wire::read_varint_u64(bytes, &mut position)? { 0 => None, 1 => Some(MutationId(read_fold_string(bytes, &mut position, control).await?)), other => return Err(malformed(position, format!("invalid observed flag {other}"))) });
    let count = crate::wire::read_varint_u64(bytes, &mut position)?;
    if count > bytes.len().saturating_sub(position) as u64 { return Err(malformed(position, "target count exceeds envelope")); }
    let mut target = control.track(Vec::with_capacity(count as usize));
    for _ in 0..count { target.push(read_fold_string(bytes, &mut position, control).await?); }
    let diff_schema = control.track(SchemaId(read_fold_string(bytes, &mut position, control).await?));
    let diff_payload = control.track(read_fold_blob(bytes, &mut position, control).await?);
    let inverse_schema = control.track(SchemaId(read_fold_string(bytes, &mut position, control).await?));
    let inverse_payload = control.track(read_fold_blob(bytes, &mut position, control).await?);
    control.pulse().await?;
    let timestamp = super::super::decode_hlc(bytes, &mut position)?;
    let flags = crate::wire::read_varint_u64(bytes, &mut position)?;
    if flags > 0b111 { return Err(malformed(position, "invalid envelope flags")); }
    let transaction = control.track(if flags & 1 != 0 {
        let id = control.track(read_fold_string(bytes, &mut position, control).await?);
        let tool = read_fold_string(bytes, &mut position, control).await?;
        Some(crate::TransactionRef { id: id.take(), tool })
    } else { None });
    let verb = control.track(if flags & 2 != 0 { Some(read_fold_string(bytes, &mut position, control).await?) } else { None });
    let line = control.track(if flags & 4 != 0 { Some(read_fold_string(bytes, &mut position, control).await?) } else { None });
    if position != bytes.len() { return Err(malformed(position, "quarantined envelope has trailing bytes")); }
    Ok(super::super::MutationEnvelope { mutation_id: mutation_id.take(), document_id: document_id.take(), actor: actor.take(), dependencies: dependencies.take(), observed: observed.take(), target: target.take(), diff: super::super::ArtifactDiff { schema: diff_schema.take(), payload: diff_payload.take() }, inverse: super::super::InverseMutation { schema: inverse_schema.take(), payload: inverse_payload.take() }, timestamp, transaction: transaction.take(), verb: verb.take(), line: line.take() })
}
