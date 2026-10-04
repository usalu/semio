//! ✋️ The cross-plugin node-drag history law (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §5, §13.3, audit G1):
//! a committed relative node-drag leaf edited in history — its offset or its targets — previews as the state right before it,
//! its Report replay re-applies every downstream leaf onto the edited drag exactly like a fresh fold of the edited log, and an
//! overwrite commits exactly that head. Every guest feeds its own drag leaf, its base document and its fold; the mounted
//! gesture half (one drag is one transaction row, a release that moves nothing leaves zero trace) reads its rows through
//! [`node_drag_transaction_rows`].

use super::*;

/// 🧾️ The applied history rows of `patch` that a tool transaction keyed — what one released node drag must add exactly one of.
pub fn node_drag_transaction_rows(patch: &semio_framework::kernel::HistoryPatch) -> Vec<&semio_framework::kernel::HistoryEntry> {
    patch.upserts.iter().filter(|entry| entry.applied && entry.transaction.is_some()).collect()
}

/// ⚖️ LAW: for every `(index, edited)` of `edits`, the history `log` on `base` with the leaf at `index` replaced by `edited` (a
/// re-offset or re-targeted drag) previews as the fold of the leaves before it, replays from the edited leaf to the fresh fold
/// `fold` gives of the edited log, never blocks finalizing, and overwrites to exactly that head. Panics naming `guest` and the
/// edit otherwise.
pub async fn assert_node_drag_edits_replay_like_a_fresh_fold<P, Mu>(guest: &str, schema: &str, base: &P, log: &[Mu], edits: &[(usize, Mu)], fold: impl Fn(&mut P, &Mu))
where
    P: Clone + PartialEq + std::fmt::Debug + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + Send + Sync + 'static,
    Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let fresh = |leaves: &[Mu]| {
        let mut state = base.clone();
        for leaf in leaves {
            fold(&mut state, leaf);
        }
        state
    };
    for (index, edited) in edits {
        let mut store = ArtifactStore::<P, Mu>::new(create_document_envelope::<P, Mu>(schema, guest, base.clone(), None)).await.unwrap_or_else(|error| panic!("{guest}: the drag history store opens: {error:?}"));
        store.install_document_store_owners_exact(bounded_document_store_owners::<P, Mu>());
        for leaf in log {
            store.dispatch(ArtifactCommand::Apply { mutations: vec![leaf.clone()], description: None, transaction: None }).await.unwrap_or_else(|error| panic!("{guest}: a logged leaf applies: {error:?}"));
        }
        let ids: Vec<protocol::MutationId> = store.mutation_ops().unwrap_or_else(|error| panic!("{guest}: the applied operations read: {error:?}")).into_iter().map(|operation| operation.mutation_id).collect();
        let target = ids.get(*index).cloned().unwrap_or_else(|| panic!("{guest}: edit #{index} names no logged leaf of {}", ids.len()));
        let drafts = BTreeMap::from([(target.clone(), protocol::InputReplacement::Input { schema: schema.into(), payload: edited.encode_op().unwrap_or_else(|error| panic!("{guest}: edit #{index} encodes: {error:?}")) })]);
        let preview = store.state_before(&target, &drafts).unwrap_or_else(|error| panic!("{guest}: the preview base of edit #{index} folds: {error:?}"));
        assert_eq!(preview.as_ref(), &fresh(&log[..*index]), "{guest}: the preview base of edit #{index} is the state right before the edited drag");
        drop(preview);
        let mut edited_log = log.to_vec();
        edited_log[*index] = edited.clone();
        let expected = fresh(&edited_log);
        let mut replay = store.begin_report_replay(&drafts, Some(&target)).unwrap_or_else(|error| panic!("{guest}: the replay of edit #{index} begins: {error:?}"));
        while let store::ReplayStep::Pending(_) = replay.step(store.replay_edits(), &mut || false).unwrap_or_else(|error| panic!("{guest}: the replay of edit #{index} steps: {error:?}")) {}
        let result = replay.finish().unwrap_or_else(|error| panic!("{guest}: the replay of edit #{index} finishes: {error:?}"));
        assert!(!store.replay_report(&result).unwrap_or_else(|error| panic!("{guest}: the report of edit #{index} reads: {error:?}")).blocks_finalize(), "{guest}: a re-offset or re-targeted drag never blocks finalizing");
        assert_eq!(result.state().unwrap_or_else(|| panic!("{guest}: the replay of edit #{index} reached no state")).as_ref(), &expected, "{guest}: the replay of edit #{index} equals the fresh fold of the edited log");
        store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.unwrap_or_else(|error| panic!("{guest}: the overwrite of edit #{index} commits: {error:?}"));
        assert_eq!(store.snapshot_ref(), &expected, "{guest}: the overwritten history of edit #{index} folds to the edited state");
        let mut disposer = bounded_document_store_disposer::<P, Mu>();
        for _ in 0..65_536 {
            if disposer.terminal_is_empty(&store) {
                break;
            }
            disposer.close_step(&mut store, 1, 64 * 1024).unwrap_or_else(|error| panic!("{guest}: the drag history store retires: {error:?}"));
        }
        assert!(disposer.terminal_is_empty(&store), "{guest}: the drag history store retires to its terminal-empty shell");
    }
}
