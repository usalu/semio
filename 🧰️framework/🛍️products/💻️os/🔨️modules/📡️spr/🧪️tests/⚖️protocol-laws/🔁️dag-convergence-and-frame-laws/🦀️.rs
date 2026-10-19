//! 🔁️ DAG convergence and frame laws of the protocol laws: the shuffle and retire helpers, the operation DAG convergence law, the wire and channel frame samples and the outcome laws. See [`super`].

use super::*;

//#region 🔖️Laws
pub(super) async fn fisher_yates_shuffle(rng: &mut SplitMix64, items: &mut [usize]) {
    for i in (1..items.len()).rev() {
        let j = rng.next_range(i as u64 + 1).await as usize;
        items.swap(i, j);
    }
}

/// 🧹️ Walks a law's own dag down to its exact terminal-empty witness. `take_next_applied` hands back
/// applied ENVELOPES but leaves the applied and pending identity ledgers standing, and
/// `MutationDag::drop` asserts all three are empty, so a law that builds a dag retires it the way a
/// store's close cursor does — one owner per `take_one_close_owner` turn.
pub(super) fn retire_dag(mut dag: crate::MutationDag) {
    while dag.take_one_close_owner().is_some() {}
    assert!(dag.terminal_is_empty(), "a law's causal dag must reach its exact terminal-empty witness");
}

/// 🧺️ Drains every applied envelope out of `dag` via [`crate::os_spr::MutationDag::take_next_applied`],
/// mirroring the one-at-a-time cursor contract (`Envelope` collects, `SeededIdentity` is a no-op,
/// `Complete` ends the loop).
pub(super) fn take_all_applied(dag: &mut crate::MutationDag) -> Vec<crate::MutationEnvelope> {
    let mut envelopes = Vec::new();
    loop {
        match dag.take_next_applied() {
            crate::MutationDagAppliedStep::Envelope(envelope) => envelopes.push(envelope),
            crate::MutationDagAppliedStep::SeededIdentity => {}
            crate::MutationDagAppliedStep::Complete => return envelopes,
        }
    }
}

/// ✅️ LAW: `MutationDag` converges to the same fully-applied set regardless of insertion order, for
/// `permutation_count` random shuffles of `envelopes` (which must form a closed dependency set —
/// see `OpDagGen::generate`).
pub async fn assert_op_dag_convergence(envelopes: &[crate::MutationEnvelope], seed: u64, permutation_count: usize) {
    let expected: std::collections::BTreeSet<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
    let mut rng = SplitMix64(seed);
    for _ in 0..permutation_count.max(1) {
        let mut order: Vec<usize> = (0..envelopes.len()).collect();
        fisher_yates_shuffle(&mut rng, &mut order).await;

        let mut dag = crate::MutationDag::new();
        for index in order {
            dag.insert(envelopes[index].clone()).expect("a closed dependency set inserted with unique ids must never duplicate");
        }
        let applied: std::collections::BTreeSet<String> = take_all_applied(&mut dag).iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        retire_dag(dag);
        assert_eq!(applied, expected, "MutationDag must converge to the same fully-applied set regardless of insertion order");
    }
}

/// 🎞️ Which side of the semio_hub wire protocol an `assert_wire_frame_round_trip` sample represents —
/// `ClientFrame`/`ServerFrame` are distinct enums with distinct encode/decode fn pairs, so this
/// crate's own choice (the contract names one law fn, not two) is a single sum type covering both.
pub enum WireFrameSample {
    Client(crate::ClientFrame, crate::Lane),
    Server(crate::ServerFrame, crate::Lane),
}

/// ✅️ LAW: `decode(encode(frame)) == frame`, for either wire direction.
pub async fn assert_wire_frame_round_trip(sample: &WireFrameSample) {
    match sample {
        WireFrameSample::Client(frame, lane) => {
            let bytes = crate::encode_client_frame(frame, *lane).await;
            let (decoded_lane, decoded_frame) = crate::decode_client_frame(&bytes).await.expect("decode_client_frame must succeed on its own encode_client_frame output");
            assert_eq!(decoded_lane, *lane, "decoded Lane must match the encoded Lane");
            assert_eq!(&decoded_frame, frame, "decode_client_frame(encode_client_frame(frame)) must equal frame");
        }
        WireFrameSample::Server(frame, lane) => {
            let bytes = crate::encode_server_frame(frame, *lane).await;
            let (decoded_lane, decoded_frame) = crate::decode_server_frame(&bytes).await.expect("decode_server_frame must succeed on its own encode_server_frame output");
            assert_eq!(decoded_lane, *lane, "decoded Lane must match the encoded Lane");
            assert_eq!(&decoded_frame, frame, "decode_server_frame(encode_server_frame(frame)) must equal frame");
        }
    }
}

/// 🧵️ Which side of the app-engine channel an `assert_channel_frame_round_trip` sample
/// represents — `AppCommand`/`AppFrame` are distinct enums with distinct encode/decode fn pairs,
/// same rationale as `WireFrameSample` above.
pub enum ChannelFrameSample<'a> {
    Command(&'a crate::AppCommand),
    Frame(&'a crate::AppFrame),
}

/// ✅️ LAW: `decode(encode(frame)) == frame`, for either channel direction.
pub async fn assert_channel_frame_round_trip(sample: &ChannelFrameSample<'_>) {
    match sample {
        ChannelFrameSample::Command(command) => {
            let encoded = crate::encode_app_command(command).await.expect("encode_app_command must succeed");
            assert_eq!(encoded.page_len(), 1, "testkit command is one page");
            let bytes = encoded.front_page().expect("testkit command has one page").as_slice();
            let decoded = crate::channel::decode_app_command(bytes).await.expect("decode_app_command must succeed on its own encode_app_command output");
            assert_eq!(&decoded, *command, "decode_app_command(encode_app_command(command)) must equal command");
        }
        ChannelFrameSample::Frame(frame) => {
            let bytes = crate::encode_app_frame(frame).await;
            let decoded = crate::decode_app_frame(&bytes).await.expect("decode_app_frame must succeed on its own encode_app_frame output");
            assert_eq!(&decoded, *frame, "decode_app_frame(encode_app_frame(frame)) must equal frame");
        }
    }
}

//#region 🔖️Outcome
/// ✅️ LAW (§C2): a mutation whose target is absent must yield an outcome carrying an `Error`
/// message with code `mutation.target-missing`, and the diff must carry no change (`D::default()`
/// — the `MutationOutcome::error` constructor's own empty-diff guarantee every missing-target
/// `diff` leaf is expected to route through).
pub async fn assert_missing_target_is_error<P, Op>(base: &P, mutation: &Op)
where
    Op: crate::Mutation<P> + crate::ColdMutation<P>,
    Op::ReferenceDiff: PartialEq + std::fmt::Debug + Default,
{
    let outcome = mutation.diff(base);
    let has_missing_target_error = outcome.messages().iter().any(|message| message.level == semio_framework_diagnostic::Severity::Error && message.code.0 == "mutation.target-missing");
    assert!(has_missing_target_error, "a mutation targeting an absent element must carry an Error message with code 'mutation.target-missing', got {:?}", outcome.messages());
    assert_eq!(outcome.diff(), &Op::ReferenceDiff::default(), "a mutation.target-missing outcome must carry no change (diff == Diff::default())");
}

/// ✅️ LAW (§C2 law 1): whenever `outcome.worst_level()` is `Fatal`, `outcome.diff() ==
/// D::default()`.
pub async fn assert_fatal_never_applies<D>(outcome: &crate::MutationOutcome<D>)
where
    D: PartialEq + std::fmt::Debug + Default,
{
    if outcome.worst_level() == Some(semio_framework_diagnostic::Severity::Fatal) {
        assert_eq!(outcome.diff(), &D::default(), "a Fatal outcome must carry diff == D::default()");
    }
}

/// ✅️ LAW (§C2 law 3): equal `(op, base)` produces equal messages and an equal diff, across
/// repeated invocations.
pub async fn assert_outcome_deterministic<P, Op>(base: &P, mutation: &Op)
where
    Op: crate::Mutation<P> + crate::ColdMutation<P>,
    Op::ReferenceDiff: PartialEq + std::fmt::Debug,
{
    let a = mutation.diff(base);
    let b = mutation.diff(base);
    assert_eq!(a.diff(), b.diff(), "diff(op, base) must be deterministic across repeated invocations");
    assert_eq!(a.messages(), b.messages(), "messages(op, base) must be deterministic across repeated invocations");
}
//#endregion 🔖️Outcome
//#endregion 🔖️Laws
