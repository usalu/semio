//! 🎞️ Event sourced history reprojection: the remote transition adoption, the reprojection, the live prefix state, the replayed projection adoption, the convergence checkpoints and the prefix snapshot ring. See [`super`].

use super::*;
use semio_framework_os_spr::ColdMutation;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreLive<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    //#region 🔖️EventSourcedHistory
    /// 📥️ Records admitted remote transitions in the log and materializes the projection they fold to — adopting `replayed`
    /// when it is their finished Report replay — or records none of them.
    pub(super) async fn adopt_remote_transitions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, transitions: Vec<semio_framework_os_spr::MutationEnvelope>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let admitted: Vec<MutationId> = transitions.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions {
            self.insert_transition(store_log, envelope);
        }
        let projected = match replayed {
            Some(result) => self.reproject_replayed(store_log, store_owners, store_cursor, result),
            None => self.reproject(store_log, store_owners, store_cursor).await,
        };
        projected.inspect_err(|_| store_log.envelope.transitions.retain(|known| !admitted.contains(&known.mutation_id)))
    }

    /// 📎️ Records `envelope` in the transition log at its `(hlc, id)` position.
    pub fn insert_transition(&mut self, store_log: &mut StoreLog<P, Mutation>, envelope: semio_framework_os_spr::MutationEnvelope) {
        self.fold_frontier = None;
        let key = envelope.timestamp.cmp_key();
        let position = store_log.envelope.transitions.partition_point(|known| (known.timestamp.cmp_key(), known.mutation_id.0.as_str()) < (key, envelope.mutation_id.0.as_str()));
        store_log.envelope.transitions.insert(position, envelope);
    }

    /// ♻️ Re-derives every history position from the event log (`crate::os_spr::fold_history`)
    /// and materializes the payload projection for it. The single path local commands and remote
    /// ingest converge through: the document is always the fold of its events, never a merge.
    /// Transactional: the projection is computed and every capacity its adoption needs is checked before the first fact
    /// is adopted. Answers the messages of every edit a replay re-evaluated.
    pub async fn reproject(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        self.reproject_now(store_log, store_owners, store_cursor)
    }

    /// ♻️ [`Self::reproject`]'s body, which never suspends: for the history steps that must not.
    pub(super) fn reproject_now(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        self.fold_frontier = None;
        let fold = store_log.fold_live_log()?;
        let replay = self.reprojection_replay(store_log, store_cursor, &fold)?;
        self.adopt_reprojection(store_log, store_owners, store_cursor, fold, replay)
    }

    /// 🧷️ Adopts a local edit already appended as the fold's tail ([`Self::folded_tail`]) without folding the log: the fold
    /// keeps exactly the redo entries `actor` did not author and changes nothing else.
    pub(super) fn adopt_folded_tail(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, actor: Option<&str>, key: (u64, u64, u64)) -> Result<(), VcsError> {
        let kept: Vec<String> = store_log.redo_edit_ids.iter().filter(|redo| store_log.envelope.vcs.edits.find_near(|edit| edit.id == **redo).and_then(|edit| edit.actor.as_deref()) != actor).cloned().collect();
        if kept.len() != store_log.redo_edit_ids.len() {
            self.replace_redo_edit_ids_retained(store_log, store_owners, store_cursor, kept)?;
        }
        self.fold_frontier = Some(key);
        Ok(())
    }

    /// 🔭️ The projection after the first `length` live applied edits: the head, the tail snapshot, or the nearest retained
    /// prefix — exact, since every bump evicts each entry past the first changed position ([`Self::prune_prefix_ring`]) — or
    /// the genesis, folded forward over the live order (one stride at most once the ring holds the prefix). Answers the
    /// prefixes it folded through on the ring's stride with their forwards-only digests, chained from the base's digest
    /// instead of hashing the whole history.
    pub fn live_prefix_state(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, length: usize) -> Result<(Arc<P>, Vec<(usize, Option<[u8; 32]>, Arc<P>)>), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let applied = store_log.applied_edit_ids.len();
        if !self.current_detached && length == applied {
            return Ok((Arc::clone(&*self.current), Vec::new()));
        }
        if let Some((_, snapshot)) = self.tail_undo_cache.as_ref().filter(|(edit_id, _)| length + 1 == applied && store_log.applied_edit_ids.last() == Some(edit_id)) {
            return Ok((Arc::clone(snapshot), Vec::new()));
        }
        let (start, mut digest, mut running) = match self.prefix_ring.iter().rev().find(|entry| entry.length <= length) {
            Some(entry) => {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(store_log, store_cursor, entry);
                (entry.length, entry.digest, Arc::clone(&entry.snapshot))
            }
            None => (0, forward_prefix_genesis(store_cursor.revision_accumulator.identity_digest), store_log.envelope.vcs.genesis.facts().share_snapshot()),
        };
        let stride = Self::prefix_stride(applied);
        let last_recorded = length.saturating_sub(1) / stride * stride;
        let mut recorded = Vec::new();
        for (offset, edit_id) in store_log.applied_edit_ids.iter_from(start).take(length - start).enumerate() {
            let Some(edit) = store_log.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id) else {
                retire_shared_projection::<P, Mutation>(running);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                return Err(VcsError::UnknownEdit(edit_id.clone()));
            };
            fold_effective_edit(&mut running, edit, &store_log.envelope.schema, &self.supersessions, usize::MAX);
            let reached = start + offset + 1;
            if reached <= last_recorded {
                digest = forward_prefix_step::<P, Mutation>(&digest, edit, &self.supersessions);
                if reached % stride == 0 {
                    recorded.push((reached, Some(digest), Arc::clone(&running)));
                }
            }
        }
        Ok((running, recorded))
    }

    /// 🧱️ Where a replay of the live order from `length` starts: the head, the tail snapshot, or the nearest retained prefix
    /// at or before `length` (exact: [`Self::prune_prefix_ring`]), else the genesis; the replay folds the rest of the prefix
    /// itself ([`EditReplay::new_after_prefix`]).
    pub(super) fn live_base(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, length: usize) -> (usize, Arc<P>) {
        let applied = store_log.applied_edit_ids.len();
        if !self.current_detached && length == applied {
            return (applied, Arc::clone(&*self.current));
        }
        if let Some((_, snapshot)) = self.tail_undo_cache.as_ref().filter(|(edit_id, _)| applied > 0 && length + 1 >= applied && store_log.applied_edit_ids.last() == Some(edit_id)) {
            return (applied - 1, Arc::clone(snapshot));
        }
        match self.prefix_ring.iter().rev().find(|entry| entry.length <= length) {
            Some(entry) => {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(store_log, store_cursor, entry);
                (entry.length, Arc::clone(&entry.snapshot))
            }
            None => (0, store_log.envelope.vcs.genesis.facts().share_snapshot()),
        }
    }

    /// 🔬️ Test-build oracle of [`Self::live_prefix_state`]'s trust in the pruned ring: `entry` names the live prefix of its
    /// length (checked outside the hot-path census).
    #[cfg(test)]
    fn assert_prefix_entry_is_live(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, entry: &PrefixSnapshot<P>) {
        let saved = hot_path_census::take();
        let digests = forward_prefix_digests::<P, Mutation, _, _>(store_cursor.revision_accumulator.identity_digest, store_log.applied_edit_ids.iter().take(entry.length), &store_log.envelope.vcs.edits, &self.supersessions).expect("the live prefix digests");
        hot_path_census::count(|census| *census = saved);
        assert_eq!(digests.last(), Some(&entry.digest), "a pruned prefix ring holds only entries of the live prefix (length {})", entry.length);
    }

    /// 🩺️ The Report replay `fold` needs: none when its applied order and effective inputs are live or it only drops applied
    /// edits from the tail (every remaining edit keeps its prefix, so its durable messages and inverses stay
    /// replay-consistent), otherwise a replay from the first position whose order or effective input differs — an interior
    /// undo, redo or checkout rewrites every downstream edit's messages and inverses exactly as a supersession does.
    /// Synchronous: one command or ingest replays the applied history in place. A remote change under
    /// [`Self::defer_remote_replays`] and a local step under [`Self::defer_local_replays`] replay resumably instead
    /// ([`Self::step_reprojection`]).
    fn reprojection_replay(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold) -> Result<Option<EditReplayResult<P, Mutation>>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        if fold.supersessions == *self.supersessions && store_log.applied_edit_ids.starts_with(&fold.applied) {
            return Ok(None);
        }
        let (from, horizon) = self.replay_window(store_log, &fold.applied, &fold.supersessions)?;
        self.report_replay(store_log, store_cursor, &fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len()))?.run(&store_log.envelope.vcs.edits).map(Some)
    }

    /// 🚀️ Adopts an already finished Report replay of exactly the folded event log instead of replaying again — the fast path
    /// of a supersession whose replay was computed before it was authored (a dry run, or a time-travel session). A replay of
    /// another order or other effective inputs is dropped and the log is replayed as usual.
    pub fn reproject_replayed(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, result: EditReplayResult<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        self.fold_frontier = None;
        let fold = match store_log.fold_live_log() {
            Ok(fold) => fold,
            Err(error) => {
                let _ = store_owners.retire_replay_result(result);
                return Err(error);
            }
        };
        if fold.applied != result.order || !same_effective_inputs(&fold.supersessions, &result.supersessions) {
            store_owners.retire_replay_result(result)?;
            let replay = self.reprojection_replay(store_log, store_cursor, &fold)?;
            return self.adopt_reprojection(store_log, store_owners, store_cursor, fold, replay);
        }
        self.adopt_reprojection(store_log, store_owners, store_cursor, fold, Some(result))
    }

    /// 🧷️ Adopts `fold` and the replay it needed ([`Self::reprojection_replay`]): a tail-only removal re-projects through
    /// the tail snapshot or the nearest retained prefix, a replay publishes its projection, inverses and messages. Nothing
    /// is adopted unless [`Self::preflight_reprojection`] admits all of it.
    fn adopt_reprojection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, fold: semio_framework_os_spr::HistoryFold, replay: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let prefix = match &replay {
            None if fold.applied != *store_log.applied_edit_ids => Some(self.live_prefix_state(store_log, store_cursor, fold.applied.len())?),
            _ => None,
        };
        let recorded = prefix.as_ref().map_or(0, |(_, recorded)| recorded.len()) + replay.as_ref().map_or(0, |result| result.recorded.len());
        if let Err(error) = store_owners.preflight_reprojection(store_log, &fold, replay.as_ref(), recorded) {
            if let Some((current, recorded)) = prefix {
                retire_shared_projection::<P, Mutation>(current);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
            }
            if let Some(result) = replay {
                let _ = store_owners.retire_replay_result(result);
            }
            return Err(error);
        }
        if let Err(error) = store_owners.adopt_history_facts(store_log, &fold) {
            if let Some((current, recorded)) = prefix {
                retire_shared_projection::<P, Mutation>(current);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
            }
            if let Some(result) = replay {
                let _ = store_owners.retire_replay_result(result);
            }
            return Err(error);
        }
        let semio_framework_os_spr::HistoryFold { applied, redo, checkpoint, alternative, supersessions, .. } = fold;
        let replayed = match (replay, prefix) {
            (Some(result), _) => self.adopt_replayed_projection(store_log, store_owners, store_cursor, applied, supersessions, result)?,
            (None, Some((current, recorded))) => {
                self.replace_current_retained(store_log, store_owners, current)?;
                self.replace_tail_undo_cache_retained(store_log, store_owners, None)?;
                self.replace_applied_edit_ids_retained(store_log, store_owners, store_cursor, applied)?;
                self.admit_prefix_snapshots(store_log, store_owners, recorded)?;
                Vec::new()
            }
            (None, None) => Vec::new(),
        };
        self.adopt_history_positions(store_log, store_owners, store_cursor, redo, checkpoint, alternative)?;
        self.fold_frontier = Some(store_log.event_frontier());
        Ok(replayed)
    }

    /// 🪄️ Publishes a finished Report replay of `applied` under `supersessions`. A replay that only appended one edit to the
    /// live tail keeps the live head as that edit's tail snapshot (an undo answers from it again).
    fn adopt_replayed_projection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, applied: Vec<String>, supersessions: EffectiveSupersessions, mut result: EditReplayResult<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let from = result.from;
        let appended = from == store_log.applied_edit_ids.len() && from + 1 == applied.len() && store_log.applied_edit_ids.is_prefix_of_slice(&applied) && supersessions == *self.supersessions && !self.current_detached;
        let tail = appended.then(|| (applied[from].clone(), Arc::clone(&*self.current)));
        let recorded = result.take_recorded();
        let replayed = self.adopt_report_replay(store_log, store_owners, store_cursor, result, tail)?;
        self.replace_applied_edit_ids_retained(store_log, store_owners, store_cursor, applied)?;
        self.replace_supersessions_retained(store_owners, supersessions)?;
        self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(from);
        self.retain_prefix_snapshots(store_log, store_owners, store_cursor, recorded)?;
        store_cursor.revision_dirty_from = Some(store_cursor.revision_dirty_from.map_or(from, |dirty| dirty.min(from)));
        store_cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
        Ok(replayed)
    }

    /// 📌️ Adopts the fold's redo stack, checkpoint and active alternative.
    fn adopt_history_positions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, redo: Vec<String>, checkpoint: Option<String>, alternative: Option<String>) -> Result<(), VcsError> {
        if redo != *store_log.redo_edit_ids {
            self.replace_redo_edit_ids_retained(store_log, store_owners, store_cursor, redo)?;
        }
        if checkpoint != *store_log.current_checkpoint_id {
            self.replace_current_checkpoint_retained(store_log, store_owners, checkpoint)?;
        }
        store_log.envelope.active_alternative_id = alternative;
        Ok(())
    }

    /// 🪟️ Where a replay of `applied` under `supersessions` must start (the first position whose order or effective
    /// input differs from the live projection) and, when the order is unchanged, the prefix length from which it may
    /// converge onto the live projection (just past the last changed input).
    pub(super) fn replay_window(&self, store_log: &StoreLog<P, Mutation>, applied: &[String], supersessions: &EffectiveSupersessions) -> Result<(usize, Option<usize>), VcsError> {
        let changed = superseded_positions::<P, Mutation>(applied, &store_log.envelope.vcs.edits, &self.supersessions, supersessions)?;
        let common = applied.iter().zip(store_log.applied_edit_ids.iter()).take_while(|(next, live)| next == live).count();
        let reordered = common != applied.len() || common != store_log.applied_edit_ids.len();
        let from = changed.first().copied().unwrap_or(applied.len()).min(if reordered { common } else { applied.len() });
        let horizon = if reordered { None } else { changed.last().map(|position| position + 1) };
        Ok((from, horizon))
    }

    /// 📽️ A Report replay of `applied[from..]` under `supersessions`, starting from the live projection nearest before
    /// `from` ([`Self::live_base`]; every caller's `from` lies at or before the first position where `applied` or
    /// `supersessions` leave the live history, so the live prefix is the replayed one). With snapshot equality enabled and the live order unchanged it may converge onto the live
    /// projection from prefix length `horizon` on.
    pub(super) fn report_replay(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, applied: &[String], from: usize, supersessions: EffectiveSupersessions, horizon: Option<usize>, stride: usize) -> Result<EditReplay<P, Mutation>, VcsError> {
        let (start, base) = self.live_base(store_log, store_cursor, from);
        let replay = EditReplay::new_after_prefix(ReplayMode::Report, start, base, applied.to_vec(), from, &store_log.envelope.schema, supersessions, &store_log.envelope.vcs.edits)?.with_base(store_cursor.generation, store_cursor.content_revision).recording(stride, Vec::new());
        let convergence = match (horizon, self.snapshot_equality) {
            (Some(horizon), Some(equal)) if store_log.applied_edit_ids.eq_slice(applied) => match self.convergence_checkpoints(store_log, store_cursor, horizon, equal) {
                Ok(checkpoints) => Some(checkpoints),
                Err(error) => {
                    let _ = replay.retire_unscheduled();
                    return Err(error);
                }
            },
            _ => None,
        };
        Ok(replay.with_convergence(convergence))
    }

    /// 🚩️ The live projection's retained states from prefix length `horizon` on — the pruned ring's entries (exact:
    /// [`Self::prune_prefix_ring`]), the tail snapshot and the head — as a convergence probe. Every state is an alias of a root this
    /// store keeps, so the probe never owns a last reference.
    pub(super) fn convergence_checkpoints(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, horizon: usize, equal: fn(&P, &P) -> bool) -> Result<ReplayConvergence<P>, VcsError> {
        let length = store_log.applied_edit_ids.len();
        let mut checkpoints: Vec<(usize, Arc<P>)> = self
            .prefix_ring
            .iter()
            .filter(|entry| entry.length >= horizon && entry.length < length)
            .map(|entry| {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(store_log, store_cursor, entry);
                (entry.length, Arc::clone(&entry.snapshot))
            })
            .collect();
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() {
            if length > horizon && store_log.applied_edit_ids.last() == Some(edit_id) && !checkpoints.iter().any(|(known, _)| *known == length - 1) {
                checkpoints.push((length - 1, Arc::clone(snapshot)));
            }
        }
        checkpoints.push((length, Arc::clone(&self.current)));
        checkpoints.sort_by_key(|(prefix, _)| *prefix);
        Ok(ReplayConvergence { horizon, checkpoints, equal })
    }

    /// 🔭️ The projection after `applied[..from]` under `supersessions` ([`Self::prefix_state_recorded`]).
    fn prefix_state(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, applied: &[String], from: usize, supersessions: &EffectiveSupersessions) -> Result<Arc<P>, VcsError>  where P: Clone, Mutation: ColdMutation<P> {
        Ok(self.prefix_state_recorded(store_log, store_cursor, applied, from, supersessions, usize::MAX)?.0)
    }

    /// 🔬️ The projection after `applied[..from]` under `supersessions`: the live head, the tail snapshot or the nearest
    /// retained prefix whose digest names the same prefix, folded forward — the genesis when none lies before `from` —
    /// with the projections it folded through at every multiple of `stride`.
    pub(super) fn prefix_state_recorded(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, applied: &[String], from: usize, supersessions: &EffectiveSupersessions, stride: usize) -> Result<(Arc<P>, Vec<(usize, Arc<P>)>), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let identity = store_cursor.revision_accumulator.identity_digest;
        let digests = forward_prefix_digests::<P, Mutation, _, _>(identity, &applied[..from], &store_log.envelope.vcs.edits, supersessions)?;
        let live = forward_prefix_digests::<P, Mutation, _, _>(identity, &*store_log.applied_edit_ids, &store_log.envelope.vcs.edits, &self.supersessions)?;
        let length = store_log.applied_edit_ids.len();
        let mut candidates: Vec<(usize, [u8; 32], &Arc<P>)> = self.prefix_ring.iter().map(|entry| (entry.length, entry.digest, &entry.snapshot)).collect();
        if !self.current_detached {
            candidates.push((length, live[length], &*self.current));
        }
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() {
            if store_log.applied_edit_ids.last() == Some(edit_id) {
                candidates.push((length - 1, live[length - 1], snapshot));
            }
        }
        let best = candidates.into_iter().filter(|(prefix, digest, _)| *prefix <= from && digests.get(*prefix) == Some(digest)).max_by_key(|(prefix, _, _)| *prefix).map(|(prefix, _, snapshot)| (prefix, snapshot));
        let (length, base) = match best {
            Some((length, snapshot)) => (length, Arc::clone(snapshot)),
            None => (0, store_log.envelope.vcs.genesis.facts().share_snapshot()),
        };
        if length == from {
            return Ok((base, Vec::new()));
        }
        fold_recorded::<P, Mutation>(base, applied, length, from, &store_log.envelope.vcs.edits, &store_log.envelope.schema, supersessions, stride)
    }

    /// 📣️ Publishes a finished Report replay: every replayed edit gets its rebased inverse and its messages (bounded by
    /// the replay itself, [`bound_edit_messages`]), `tail` becomes the tail snapshot and the reached projection becomes
    /// `current`.
    fn adopt_report_replay(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, mut result: EditReplayResult<P, Mutation>, tail: Option<(String, Arc<P>)>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let taken = store_cursor.take_report_replay(store_log, &mut result);
        let retired = store_owners.retire_replay_result(result);
        let (replayed, state) = taken?;
        if let Err(error) = retired {
            retire_shared_projection::<P, Mutation>(state);
            return Err(error);
        }
        self.replace_tail_undo_cache_retained(store_log, store_owners, tail)?;
        self.replace_current_retained(store_log, store_owners, state)?;
        for entry in &replayed {
            store_owners.replace_edit_messages(store_log, &entry.edit_id, entry.messages.clone())?;
        }
        Ok(replayed)
    }

    /// 🔄️ Replaces the supersessions `current` is projected with; the displaced map retires one byte grant at a time.
    pub fn replace_supersessions_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: EffectiveSupersessions) -> Result<(), VcsError> {
        if *self.supersessions == next {
            return Ok(());
        }
        store_owners.displaced_retirements.reserve(usize::from(!self.supersessions.is_empty()))?;
        self.fold_frontier = None;
        let previous = std::mem::replace(&mut *self.supersessions, next);
        if !previous.is_empty() {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreSupersessionRetirement::new(previous)));
        }
        Ok(())
    }

    /// 📏️ The prefix-ring stride for `length` applied edits: [`Self::prefix_ring_capacity`] entries span them all.
    pub(super) fn prefix_stride(length: usize) -> usize {
        length.div_ceil(Self::prefix_ring_capacity(length)).max(1)
    }

    /// 📐️ How many prefix projections a history of `length` applied edits retains: about √length, so a `state_before` or a
    /// replay start folds at most one stride of about √length edits while the retained projections grow only with the square
    /// root — never fewer than [`PREFIX_SNAPSHOT_RING_FLOOR`], never more than [`PREFIX_SNAPSHOT_RING_CAPACITY`].
    fn prefix_ring_capacity(length: usize) -> usize {
        length.isqrt().clamp(PREFIX_SNAPSHOT_RING_FLOOR, PREFIX_SNAPSHOT_RING_CAPACITY)
    }

    /// 🗃️ Admits prefix projections `(length, projection)` of the live applied history into the ring at the current
    /// stride, evicting whatever a stale digest or a coarser stride no longer justifies. Without a snapshot retirement
    /// factory nothing is retained; a refused admission retires every offered projection.
    pub(super) fn retain_prefix_snapshots(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, snapshots: Vec<(usize, Arc<P>)>) -> Result<(), VcsError> {
        if snapshots.is_empty() || (*store_owners.snapshot_retirement_factory).is_none() {
            for (_, snapshot) in snapshots {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            return Ok(());
        }
        match forward_prefix_digests::<P, Mutation, _, _>(store_cursor.revision_accumulator.identity_digest, &*store_log.applied_edit_ids, &store_log.envelope.vcs.edits, &self.supersessions) {
            Ok(digests) => self.admit_prefix_snapshots(store_log, store_owners, snapshots.into_iter().map(|(length, snapshot)| (length, digests.get(length).copied(), snapshot)).collect()),
            Err(error) => {
                for (_, snapshot) in snapshots {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                Err(error)
            }
        }
    }

    /// 🗃️ Admits prefix projections `(length, digest, projection)` of the live applied history whose forwards-only digests
    /// the caller already holds ([`Self::retain_prefix_snapshots`], [`Self::live_prefix_state`]); one without a digest, off
    /// the stride, or offered without a snapshot retirement factory retires at once.
    pub fn admit_prefix_snapshots(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, snapshots: Vec<(usize, Option<[u8; 32]>, Arc<P>)>) -> Result<(), VcsError> {
        if snapshots.is_empty() {
            return Ok(());
        }
        let admitted = match self.prune_prefix_ring(store_owners) {
            Err(error) => Err(error),
            Ok(()) => match (*store_owners.snapshot_retirement_factory).clone() {
                Some(factory) => store_owners.displaced_retirements.reserve(2 * snapshots.len()).map(|()| Some(factory)),
                None => Ok(None),
            },
        };
        let factory = match admitted {
            Ok(Some(factory)) => factory,
            refused => {
                for (_, _, snapshot) in snapshots {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                return refused.map(|_| ());
            }
        };
        let stride = Self::prefix_stride(store_log.applied_edit_ids.len());
        for (length, digest, snapshot) in snapshots {
            let Some(digest) = digest.filter(|_| length != 0 && length % stride == 0) else {
                retire_shared_projection::<P, Mutation>(snapshot);
                continue;
            };
            if let Some(position) = self.prefix_ring.iter().position(|entry| entry.length == length) {
                if self.prefix_ring[position].digest == digest {
                    retire_shared_projection::<P, Mutation>(snapshot);
                    continue;
                }
                self.evict_prefix_snapshot_reserved(store_owners, position, &factory);
            }
            if self.prefix_ring.len() >= Self::prefix_ring_capacity(store_log.applied_edit_ids.len()) {
                let victim = self.prefix_ring.iter().position(|entry| entry.length % stride != 0).unwrap_or(0);
                self.evict_prefix_snapshot_reserved(store_owners, victim, &factory);
            }
            let position = self.prefix_ring.partition_point(|entry| entry.length < length);
            self.prefix_ring.insert(position, PrefixSnapshot { length, digest, snapshot });
        }
        Ok(())
    }

    /// 🧹️ Evicts ring entry `position` into a displaced slot the caller reserved: an alias another owner holds only
    /// drops its count, the last owner — decided atomically by [`Arc::try_unwrap`] — retires through `factory`.
    fn evict_prefix_snapshot_reserved(&mut self, store_owners: &mut StoreOwners<P, Mutation>, position: usize, factory: &Arc<dyn SnapshotRetirementFactory<P>>) {
        let PrefixSnapshot { snapshot, .. } = self.prefix_ring.remove(position);
        if let Ok(unique) = Arc::try_unwrap(snapshot) {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(Arc::new(unique), Arc::clone(factory)));
        }
    }

    /// 🪓️ Evicts every ring entry past [`Self::prefix_ring_dirty_from`] — the first applied position whose id, forwards or
    /// effective input changed since the last prune — in O(ring); every reader still checks an entry's digest before folding
    /// from it.
    pub(super) fn prune_prefix_ring(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> Result<(), VcsError> {
        let dirty = std::mem::replace(&mut self.prefix_ring_dirty_from, usize::MAX);
        let stale = self.prefix_ring.len() - self.prefix_ring.partition_point(|entry| entry.length <= dirty);
        if stale == 0 {
            return Ok(());
        }
        store_owners.displaced_retirements.reserve(stale)?;
        let factory = (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store prefix ring eviction requires its exact snapshot retirement factory".into()))?;
        for _ in 0..stale {
            self.evict_prefix_snapshot_reserved(store_owners, self.prefix_ring.len() - 1, &factory);
        }
        Ok(())
    }

    /// 🧼️ Evicts every ring entry (a reload adopts a different history).
    pub fn clear_prefix_ring(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> Result<(), VcsError> {
        if self.prefix_ring.is_empty() {
            return Ok(());
        }
        store_owners.displaced_retirements.reserve(self.prefix_ring.len())?;
        let factory = (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store prefix ring eviction requires its exact snapshot retirement factory".into()))?;
        while !self.prefix_ring.is_empty() {
            self.evict_prefix_snapshot_reserved(store_owners, self.prefix_ring.len() - 1, &factory);
        }
        Ok(())
    }

    //#endregion 🔖️EventSourcedHistory
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
