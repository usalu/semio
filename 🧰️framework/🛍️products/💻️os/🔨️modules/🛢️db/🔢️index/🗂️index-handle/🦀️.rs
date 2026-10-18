//! 🗂️ Index handle: the range step, the index handle over a storage with its run operations and the owned runs. See [`super`].

use super::*;

/// 🧭️ What one entry of a newest-first range walk means for the rest of its run.
enum RangeStep {
    Skip,
    Below,
}

//#region 🔖️IndexHandle
/// 🔍️ One `(document, kind)`'s view onto its sorted runs — every typed wrapper below
/// (`CommandIndex`, `FrontierIndex`, ...) is a thin codec layered on top of one of these. Never
/// interprets key/value bytes itself; that's the typed layer's job.
pub struct IndexHandle<'a, S: IndexStorage> {
    pub storage: &'a S,
    pub document: ArtifactId,
    pub kind: IndexKind,
    pub policy: MergePolicy,
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl<'a, S: IndexStorage> IndexHandle<'a, S> {
    /// 🚀️ Opens a handle with the default `MergePolicy`.
    pub async fn new(storage: &'a S, document: ArtifactId, kind: IndexKind) -> Self {
        Self::with_policy(storage, document, kind, MergePolicy::default()).await
    }

    /// 🚀️ Opens a handle with an explicit `MergePolicy` (e.g. a tighter threshold for a
    /// hot, frequently-scanned kind, or a looser one for a write-heavy, rarely-read kind).
    pub async fn with_policy(storage: &'a S, document: ArtifactId, kind: IndexKind, policy: MergePolicy) -> Self {
        Self { storage, document, kind, policy, cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)) }
    }

    pub fn operation_control(&self, fuel: usize) -> Result<IndexCursorControl, DbError> {
        IndexCursorControl::new(self.cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), fuel)
    }

    /// 🧵️ Mounts a parent job's exact cancellation/deadline authority for retained index work.
    pub fn retained_operation_control(&self, cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<IndexCursorControl, DbError> {
        IndexCursorControl::retained(cancelled, deadline, fuel)
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    /// 📋️ This handle's live run ids, ascending (oldest sequence first) — one storage listing;
    /// every id of another kind or run-id format for the same document is filtered out.
    pub(super) async fn kind_run_ids(&self, control: &mut IndexCursorControl) -> Result<Vec<u64>, DbError> {
        control.grant()?;
        let mut source = self.storage.list_runs(&self.document).await?;
        let namespace = run_namespace(self.kind);
        let mut output = Vec::new();
        for id in source.as_slice() {
            control.grant()?;
            if namespace_of_run_id(*id) == namespace {
                output.push(*id);
            }
        }
        while source.close_step() {}
        Ok(output)
    }

    async fn view_run(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<RunView, DbError> {
        let pages = self.storage.read_run(&self.document, run_id).await?;
        view_run_pages(pages, self.kind, control).await
    }

    /// 🔀️ Folds two adjacent runs (`older` directly beneath `newer`) into one run under
    /// `older`'s sequence and deletes both inputs: written before deleted, so a crash in between
    /// leaves copies whose values agree, and the newer copy of every key still wins. The caller only
    /// picks a pair whose entries fit one run.
    async fn merge_adjacent(&self, older_id: u64, newer_id: u64, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let older = self.view_run(older_id, control).await?;
        let newer = match self.view_run(newer_id, control).await {
            Ok(newer) => newer,
            Err(error) => {
                older.close()?;
                return Err(error);
            }
        };
        let encoded = match merge_run_views(&[&older, &newer], drop_tombstones, control) {
            Ok(picks) if picks.is_empty() => Ok(None),
            Ok(picks) => match make_run_id(self.kind, sequence_of_run_id(older_id), 0, picks.len()) {
                Ok(merged_id) => encode_run_from_views(self.kind, &[&older, &newer], &picks, control).await.map(|pages| Some((merged_id, pages))),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        };
        older.close()?;
        newer.close()?;
        if let Some((merged_id, pages)) = encoded? {
            self.storage.write_run(&self.document, merged_id, pages).await?;
            if merged_id != older_id {
                self.storage.delete_run(&self.document, older_id).await?;
            }
        } else {
            self.storage.delete_run(&self.document, older_id).await?;
        }
        self.storage.delete_run(&self.document, newer_id).await
    }

    /// 🥇️ The greatest live entry whose key starts with `prefix` and is at most `upper`, walking
    /// runs newest-first and keeping only the best candidate plus the tombstoned keys above it —
    /// never every entry of the kind. The newest occurrence of a key decides whether it is live.
    pub async fn last_live_in_range(&self, prefix: &IndexBytes, upper: Option<&IndexBytes>, control: &mut IndexCursorControl) -> Result<Option<(IndexBytes, IndexBytes)>, DbError> {
        const DEAD_KEYS_MAX: usize = MAX_RUN_ENTRIES as usize;
        let ids = self.kind_run_ids(control).await?;
        let mut best: Option<(IndexBytes, IndexBytes)> = None;
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..ids.len()).rev() {
            let view = match self.view_run(ids[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    failure = Some(error);
                    break;
                }
            };
            for index in (0..view.entries.len()).rev() {
                let step = async {
                    control.grant()?;
                    if !view.key_starts_with(index, prefix)? {
                        return Ok::<_, DbError>(if view.key_cmp(index, prefix)? == std::cmp::Ordering::Less { RangeStep::Below } else { RangeStep::Skip });
                    }
                    if let Some(upper) = upper {
                        if view.key_cmp(index, upper)? == std::cmp::Ordering::Greater {
                            return Ok(RangeStep::Skip);
                        }
                    }
                    if let Some((best_key, _)) = best.as_ref() {
                        if view.key_cmp(index, best_key)? != std::cmp::Ordering::Greater {
                            return Ok(RangeStep::Below);
                        }
                    }
                    for dead_key in &dead {
                        if view.key_cmp(index, dead_key)? == std::cmp::Ordering::Equal {
                            return Ok(RangeStep::Skip);
                        }
                    }
                    let entry = view.entries[index];
                    match entry.value {
                        None => {
                            if dead.len() == DEAD_KEYS_MAX {
                                return Err(DbError::LimitExceeded("index range scan tombstone window"));
                            }
                            dead.push(view.materialize(entry.key, control).await?);
                            Ok(RangeStep::Skip)
                        }
                        Some(value) => {
                            let key = view.materialize(entry.key, control).await?;
                            let value = view.materialize(value, control).await?;
                            if let Some((previous_key, previous_value)) = best.replace((key, value)) {
                                close_index_bytes(previous_key, control).await?;
                                close_index_bytes(previous_value, control).await?;
                            }
                            Ok(RangeStep::Below)
                        }
                    }
                }
                .await;
                match step {
                    Ok(RangeStep::Skip) => {}
                    Ok(RangeStep::Below) => break,
                    Err(error) => {
                        failure = Some(error);
                        view.close()?;
                        break 'runs;
                    }
                }
            }
            view.close()?;
        }
        for key in dead {
            close_index_bytes(key, control).await?;
        }
        if let Some(error) = failure {
            if let Some((key, value)) = best {
                close_index_bytes(key, control).await?;
                close_index_bytes(value, control).await?;
            }
            return Err(error);
        }
        Ok(best)
    }

    /// ✍️ Durably appends `entries` as one new, newest retained run,
    /// then applies `MergePolicy`. A no-op (no run written) if `entries` is empty.
    pub async fn put_batch(&self, mut entries: RunEntries, control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        for pass in 0..entries.len() {
            for index in 0..entries.len().saturating_sub(pass + 1) {
                entries.sort_step(index, index + 1, control)?;
            }
        }
        let mut unique = RunEntries::new();
        for index in 0..entries.len() {
            let entry = entries.take(index).ok_or_else(|| DbError::Internal("index sorted entry owner lost".to_string()))?;
            if unique.get(unique.len().saturating_sub(1)).is_some_and(|previous| index_bytes_cmp(&previous.key, &entry.key) == std::cmp::Ordering::Equal) {
                close_run_entry(unique.pop().ok_or_else(|| DbError::Internal("index duplicate owner lost".to_string()))?, control).await?;
            }
            if let Err(entry) = unique.push(entry) {
                close_run_entry(entry, control).await?;
                return Err(DbError::LimitExceeded("index unique fixed entry owner"));
            }
        }
        let count = unique.len();
        let pages = encode_run_pages(self.kind, &unique, control).await?;
        control.grant()?;
        let _ = unique.close_step()?;
        drop(unique);
        self.append_run(pages, count, control).await
    }

    pub async fn put(&self, key: IndexBytes, value: IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut entries = RunEntries::new();
        entries.push(RunEntry { key, value: RunValue::Put(value) }).map_err(|_| DbError::LimitExceeded("index entry owner"))?;
        self.put_batch(entries, control).await
    }

    pub async fn delete(&self, key: IndexBytes, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut entries = RunEntries::new();
        entries.push(RunEntry { key, value: RunValue::Tombstone }).map_err(|_| DbError::LimitExceeded("index entry owner"))?;
        self.put_batch(entries, control).await
    }

    /// ➕️ Writes `pages` (holding `count` entries) as the kind's newest run from ONE listing
    /// of its runs, then lets `MergePolicy` fold at most one adjacent pair — sizes come from the run
    /// ids, so an append never reads a run it does not merge.
    async fn append_run(&self, pages: db_io_core::DbIoPages, count: usize, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut ids = match self.kind_run_ids(control).await {
            Ok(ids) => ids,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        let next = ids.last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        let run_id = match make_run_id(self.kind, next, 0, count) {
            Ok(run_id) => run_id,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        self.storage.write_run(&self.document, run_id, pages).await?;
        ids.push(run_id);
        self.merge_one_within_policy(&ids, control).await
    }

    /// 🔎️ Resolves `key` by searching runs newest-to-oldest in place and returning the first match —
    /// `Ok(None)` if the first match is a tombstone, or if no run has ever held `key`.
    pub async fn get(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<IndexBytes>, DbError> {
        let ids = self.kind_run_ids(control).await?;
        let mut result = Ok(None);
        for position in (0..ids.len()).rev() {
            control.grant()?;
            let view = match self.view_run(ids[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    result = Err(error);
                    break;
                }
            };
            let found = match view.find(key, control) {
                Ok(Some(index)) => match view.entries[index].value {
                    Some(value) => Some(view.materialize(value, control).await.map(Some)),
                    None => Some(Ok(None)),
                },
                Ok(None) => None,
                Err(error) => Some(Err(error)),
            };
            view.close()?;
            if let Some(found) = found {
                result = found;
                break;
            }
        }
        result
    }

    /// 🔝️ The kind's newest run read in place, or `None` when the kind holds no run — the
    /// one read an append-only owner needs to learn how far its entries reach.
    pub(super) async fn newest_run(&self, control: &mut IndexCursorControl) -> Result<Option<RunView>, DbError> {
        let ids = self.kind_run_ids(control).await?;
        match ids.last() {
            Some(newest) => self.view_run(*newest, control).await.map(Some),
            None => Ok(None),
        }
    }

    /// 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`, ascending by
    /// key — runs searched in place newest-first, each key decided by its newest occurrence, only the
    /// live matches materialized (at most `MAX_RUN_ENTRIES` of them).
    pub async fn scan_prefix(&self, prefix: &IndexBytes, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
        let run_ids = self.kind_run_ids(control).await?;
        let mut output = RunEntries::new();
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..run_ids.len()).rev() {
            let view = match self.view_run(run_ids[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    failure = Some(error);
                    break;
                }
            };
            for index in 0..view.entries.len() {
                let step = async {
                    control.grant()?;
                    if !view.key_starts_with(index, prefix)? {
                        return Ok::<_, DbError>(());
                    }
                    for seen in (0..output.len()).filter_map(|slot| output.get(slot)).map(|entry| &entry.key).chain(dead.iter()) {
                        if view.key_cmp(index, seen)? == std::cmp::Ordering::Equal {
                            return Ok(());
                        }
                    }
                    let entry = view.entries[index];
                    let key = view.materialize(entry.key, control).await?;
                    match entry.value {
                        None => {
                            if dead.len() == MAX_RUN_ENTRIES as usize {
                                close_index_bytes(key, control).await?;
                                return Err(DbError::LimitExceeded("index scan tombstone window"));
                            }
                            dead.push(key);
                        }
                        Some(value) => {
                            let value = view.materialize(value, control).await?;
                            if let Err(entry) = output.push(RunEntry { key, value: RunValue::Put(value) }) {
                                close_run_entry(entry, control).await?;
                                return Err(DbError::LimitExceeded("index scan result owner"));
                            }
                        }
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = step {
                    failure = Some(error);
                    view.close()?;
                    break 'runs;
                }
            }
            view.close()?;
        }
        for key in dead {
            close_index_bytes(key, control).await?;
        }
        if let Some(error) = failure {
            while output.close_step()? {}
            return Err(error);
        }
        for pass in 0..output.len() {
            for index in 0..output.len().saturating_sub(pass + 1) {
                output.sort_step(index, index + 1, control)?;
            }
        }
        Ok(output)
    }

    /// 🌀️ `MergePolicy`'s enforcement after one append, bounded to ONE merge: while this kind
    /// has more runs than `policy.max_runs_before_merge`, the oldest adjacent pair among its newest
    /// `max_runs_before_merge + 1` runs whose entries fit one run is merged. Sizes come from the run
    /// ids. Tombstones are dropped only when the pair holds the kind's oldest run, since nothing older
    /// can still need shadowing. Full runs never pair, so an owner that appends full runs never merges.
    async fn merge_one_within_policy(&self, ids: &[u64], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if ids.len() <= self.policy.max_runs_before_merge {
            return Ok(());
        }
        let window = ids.len() - (self.policy.max_runs_before_merge + 1);
        let candidates = &ids[window..];
        let Some(pair) = (0..candidates.len() - 1).find(|pair| entries_of_run_id(candidates[*pair]) + entries_of_run_id(candidates[*pair + 1]) <= MAX_RUN_ENTRIES) else { return Ok(()) };
        control.grant()?;
        self.merge_adjacent(candidates[pair], candidates[pair + 1], window + pair == 0, control).await
    }

    /// 🧹️ Folds this kind's runs into the fewest runs `MAX_RUN_ENTRIES` allows: from the oldest,
    /// every adjacent pair that fits one run is merged (tombstones dropped wherever the pair holds the
    /// oldest run, and from a lone oldest run). Returns the post-compaction `stats()`.
    pub async fn compact(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let mut position = 0usize;
        loop {
            let run_ids = self.kind_run_ids(control).await?;
            let pair = (position + 1 < run_ids.len()).then(|| (run_ids[position], run_ids[position + 1]));
            let oldest = run_ids.first().copied();
            let Some((older, newer)) = pair else {
                if let Some(oldest) = oldest {
                    self.drop_run_tombstones(oldest, control).await?;
                }
                break;
            };
            if entries_of_run_id(older) + entries_of_run_id(newer) <= MAX_RUN_ENTRIES {
                self.merge_adjacent(older, newer, position == 0, control).await?;
            } else {
                if position == 0 {
                    self.drop_run_tombstones(older, control).await?;
                }
                position += 1;
            }
        }
        self.stats(control).await
    }

    /// 🧹️ Rewrites the kind's oldest run without its tombstones (nothing older remains for
    /// them to shadow), or deletes it when nothing else is left in it.
    async fn drop_run_tombstones(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let view = self.view_run(run_id, control).await?;
        if view.entries.iter().all(|entry| entry.value.is_some()) {
            return view.close();
        }
        let picks: Vec<RunPick> = (0..view.entries.len()).filter(|index| view.entries[*index].value.is_some()).map(|entry| RunPick { view: 0, entry }).collect();
        let encoded = if picks.is_empty() {
            Ok(None)
        } else {
            match make_run_id(self.kind, sequence_of_run_id(run_id), level_of_run_id(run_id), picks.len()) {
                Ok(rewritten) => encode_run_from_views(self.kind, &[&view], &picks, control).await.map(|pages| Some((rewritten, pages))),
                Err(error) => Err(error),
            }
        };
        view.close()?;
        match encoded? {
            Some((rewritten, pages)) => {
                self.storage.write_run(&self.document, rewritten, pages).await?;
                if rewritten != run_id {
                    self.storage.delete_run(&self.document, run_id).await?;
                }
                Ok(())
            }
            None => self.storage.delete_run(&self.document, run_id).await,
        }
    }

    /// 📊️ Current shape of this kind's runs — see `IndexStats`'s doc for what `entry_count`
    /// does and doesn't count. Run and entry counts come from the listing; bytes from each run.
    pub async fn stats(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let run_ids = self.kind_run_ids(control).await?;
        let mut total_bytes = 0u64;
        for run_id in &run_ids {
            control.grant()?;
            let bytes = self.storage.read_run(&self.document, *run_id).await?;
            total_bytes += bytes.len() as u64;
            close_run_pages(bytes)?;
        }
        Ok(IndexStats { run_count: run_ids.len(), entry_count: run_ids.iter().map(|id| entries_of_run_id(*id)).sum(), total_bytes })
    }

    /// ✅️ Fully decodes (checksum + structural validation) every live run for this kind,
    /// surfacing the first `DbError::Corrupt` found rather than any value — `db_cli verify`'s hook.
    /// A level-0 run whose entry count differs from the one its id declares, or a folded run holding
    /// more than its level's capacity, is corrupt too.
    pub async fn verify(&self, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let ids = self.kind_run_ids(control).await?;
        for id in ids {
            let view = self.view_run(id, control).await?;
            let declared = entries_of_run_id(id);
            let held = view.entries.len() as u64;
            view.close()?;
            if (level_of_run_id(id) == 0 && declared != held) || held > declared {
                return Err(DbError::Corrupt(format!("index run {id:#x} declares {declared} entries and holds {held}")));
            }
        }
        Ok(())
    }

    /// 🗂️ Lists this kind's runs once for an append-only owner ([`OwnedRuns`]).
    pub async fn owned_runs(&self, control: &mut IndexCursorControl) -> Result<OwnedRuns, DbError> {
        let ids = self.kind_run_ids(control).await?;
        let next_sequence = ids.last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        Ok(OwnedRuns { ids, next_sequence, fold_ceiling: RUN_LEVEL_MAX })
    }

    /// 📥️ Appends strictly ascending, unique `(key, value)` puts as `runs`' newest run without listing the
    /// document's runs, then folds while the newest [`LEVEL_FOLD_RUNS`] runs are full runs of one level below
    /// `runs`' fold ceiling — at most [`RUN_LEVEL_MAX`] folds of at most [`LEVEL_FOLD_BYTES_MAX`] each, so an append
    /// costs the same however many runs the kind holds. `runs` names exactly what storage holds after every
    /// successful append; after a failed one the owner lists again ([`Self::owned_runs`]).
    pub async fn append_owned_sorted_run(&self, runs: &mut OwnedRuns, entries: &[(&[u8], &[u8])], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        let pages = encode_sorted_run_pages(self.kind, entries, control).await?;
        let run_id = match make_run_id(self.kind, runs.next_sequence, 0, entries.len()) {
            Ok(run_id) => run_id,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        self.storage.write_run(&self.document, run_id, pages).await?;
        runs.ids.push(run_id);
        runs.next_sequence += 1;
        while runs.ids.len() >= LEVEL_FOLD_RUNS {
            let group: [u64; LEVEL_FOLD_RUNS] = runs.ids[runs.ids.len() - LEVEL_FOLD_RUNS..].try_into().map_err(|_| DbError::Internal("db_index fold group".to_string()))?;
            let level = level_of_run_id(group[0]);
            if level >= runs.fold_ceiling || group.iter().any(|id| level_of_run_id(*id) != level || entries_of_run_id(*id) != level_capacity(level)) {
                return Ok(());
            }
            control.grant()?;
            let Some(folded) = self.fold_runs(&group, level + 1, control).await? else {
                runs.fold_ceiling = level;
                return Ok(());
            };
            runs.ids.truncate(runs.ids.len() - LEVEL_FOLD_RUNS);
            runs.ids.push(folded);
        }
        Ok(())
    }

    /// 🪜️ Folds `group` (adjacent full runs, oldest first) into one run of `level` under the oldest's
    /// sequence, written before its inputs are deleted — a crash between leaves copies whose values agree and the
    /// folded copy sorts right after its oldest input. `None`, with nothing written, when the inputs exceed
    /// [`LEVEL_FOLD_BYTES_MAX`].
    async fn fold_runs(&self, group: &[u64], level: u32, control: &mut IndexCursorControl) -> Result<Option<u64>, DbError> {
        let mut views: Vec<RunView> = Vec::with_capacity(group.len());
        for id in group {
            match self.view_run(*id, control).await {
                Ok(view) => views.push(view),
                Err(error) => {
                    for view in views {
                        view.close()?;
                    }
                    return Err(error);
                }
            }
        }
        let encoded = if views.iter().map(|view| view.pages.len()).sum::<usize>() > LEVEL_FOLD_BYTES_MAX {
            Ok(None)
        } else {
            let inputs: Vec<&RunView> = views.iter().collect();
            match merge_run_views(&inputs, false, control) {
                Ok(picks) => match make_run_id(self.kind, sequence_of_run_id(group[0]), level, picks.len()) {
                    Ok(folded_id) => encode_run_from_views(self.kind, &inputs, &picks, control).await.map(|pages| Some((folded_id, pages))),
                    Err(error) => Err(error),
                },
                Err(error) => Err(error),
            }
        };
        for view in views {
            view.close()?;
        }
        let Some((folded_id, pages)) = encoded? else { return Ok(None) };
        self.storage.write_run(&self.document, folded_id, pages).await?;
        for id in group {
            self.storage.delete_run(&self.document, *id).await?;
        }
        Ok(Some(folded_id))
    }
}
//#endregion 🔖️IndexHandle

//#region 🔖️OwnedRuns
/// 🗂️ The runs of one kind an append-only owner writes, ascending, as it knows them: listed once when the
/// owner mounts ([`IndexHandle::owned_runs`]) and kept current by its own appends and folds, so an append never
/// lists the document's runs again. `fold_ceiling` is the level this owner stopped folding at because a fold
/// would exceed one read operation's credit (entries of that size never fold higher).
#[derive(Debug)]
pub struct OwnedRuns {
    pub ids: Vec<u64>,
    next_sequence: u64,
    pub(super) fold_ceiling: u32,
}

impl OwnedRuns {
    /// 🔢️ How many runs the kind holds.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}
//#endregion 🔖️OwnedRuns
