//! ⏱️ Criterion benchmarks for the `protocol_*` crate family's hot paths: `HistoryLogGen`
//! generation cost, whole-file `encode_history`/`decode_history`, `HistoryAppender` streaming
//! append, `protocol_format::recover`, `protocol_wire` frame codec, `protocol_crdt::
//! merge_concurrent_diffs`, and `OpDag` insertion — each group parameterized over a scaling value
//! to reveal where a cost curve goes superlinear, not just a single-point timing. Run via
//! `nx run @protocol/testkit-rs:bench` (`bun ./📜️script.ts bench`).

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use protocol_testkit::{GenProfile, HistoryLogGen, OpDagGen};

//#region 🔖️Fixtures
fn profile_for(edit_count: usize) -> GenProfile {
    GenProfile { edit_count, max_ops_per_edit: 4, checkpoint_every: 8, adversarial: false }
}

fn generated_log(seed: u64, edit_count: usize) -> protocol::HistoryLog {
    HistoryLogGen::new(seed).generate(&profile_for(edit_count))
}

fn encoded_bytes(seed: u64, edit_count: usize) -> Vec<u8> {
    let log = generated_log(seed, edit_count);
    protocol_history::encode_history(&log, &protocol_history::EncodeOptions::default()).expect("encode_history")
}
//#endregion 🔖️Fixtures

//#region 🔖️Gen
fn bench_history_log_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("history_log_gen");
    for &edit_count in &[8usize, 64, 256] {
        group.bench_with_input(BenchmarkId::new("generate", edit_count), &edit_count, |b, &edit_count| {
            b.iter(|| black_box(HistoryLogGen::new(1).generate(&profile_for(edit_count))));
        });
    }
    group.finish();
}
//#endregion 🔖️Gen

//#region 🔖️Codec
fn bench_encode_decode_history(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_decode_history");
    for &edit_count in &[8usize, 64, 256] {
        let log = generated_log(2, edit_count);
        group.bench_with_input(BenchmarkId::new("encode_history", edit_count), &log, |b, log| {
            b.iter(|| black_box(protocol_history::encode_history(log, &protocol_history::EncodeOptions::default()).expect("encode_history")));
        });

        let bytes = encoded_bytes(2, edit_count);
        group.bench_with_input(BenchmarkId::new("decode_history", edit_count), &bytes, |b, bytes| {
            b.iter(|| black_box(protocol_history::decode_history(bytes, &protocol_history::DecodeOptions::default()).expect("decode_history")));
        });
    }
    group.finish();
}
//#endregion 🔖️Codec

//#region 🔖️Append
fn bench_history_appender(c: &mut Criterion) {
    let mut group = c.benchmark_group("history_appender");
    for &edit_count in &[8usize, 64, 256] {
        let log = generated_log(3, edit_count);
        group.bench_with_input(BenchmarkId::new("append_edit_commit_once", edit_count), &log, |b, log| {
            b.iter(|| {
                let options = protocol::WriteOptions::default();
                let mut appender = protocol::HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).expect("HistoryAppender::begin");
                for edit in &log.edits {
                    appender.append_edit(edit).expect("append_edit");
                }
                appender.commit().expect("commit");
                black_box(appender.into_sink())
            });
        });
    }
    group.finish();
}
//#endregion 🔖️Append

//#region 🔖️Recover
fn bench_recover(c: &mut Criterion) {
    let mut group = c.benchmark_group("recover");
    for &edit_count in &[8usize, 64, 256] {
        let bytes = encoded_bytes(4, edit_count);
        let limits = protocol::ProtocolLimits::default();
        group.bench_with_input(BenchmarkId::new("last_commit", edit_count), &bytes, |b, bytes| {
            b.iter(|| black_box(protocol_format::recover(bytes, &limits, protocol::RecoveryMode::LastCommit).expect("recover")));
        });
    }
    group.finish();
}
//#endregion 🔖️Recover

//#region 🔖️Wire
fn bench_wire_frame_codec(c: &mut Criterion) {
    let mut group = c.benchmark_group("wire_frame_codec");
    for &envelope_count in &[1usize, 16, 64] {
        let envelopes: Vec<protocol::OperationEnvelope> = OpDagGen::new(5).generate(envelope_count);
        let frame = protocol::ClientFrame::Commands { batch_id: 1, envelopes };
        group.bench_with_input(BenchmarkId::new("encode_client_frame", envelope_count), &frame, |b, frame| {
            b.iter(|| black_box(protocol::encode_client_frame(frame, protocol::Lane::Command)));
        });

        let bytes = protocol::encode_client_frame(&frame, protocol::Lane::Command);
        group.bench_with_input(BenchmarkId::new("decode_client_frame", envelope_count), &bytes, |b, bytes| {
            b.iter(|| black_box(protocol::decode_client_frame(bytes).expect("decode_client_frame")));
        });
    }
    group.finish();
}
//#endregion 🔖️Wire

//#region 🔖️Crdt
fn bench_crdt_merge(c: &mut Criterion) {
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct BenchDiff {
        field_a: Option<i64>,
        field_b: Option<i64>,
    }
    impl protocol::OperationDiff<(i64, i64)> for BenchDiff {
        fn apply(&self, base: &(i64, i64)) -> (i64, i64) {
            (self.field_a.unwrap_or(base.0), self.field_b.unwrap_or(base.1))
        }
        fn absorb(&mut self, other: Self) {
            if other.field_a.is_some() {
                self.field_a = other.field_a;
            }
            if other.field_b.is_some() {
                self.field_b = other.field_b;
            }
        }
    }
    fn meta_at(actor: u64, physical_ms: u64) -> protocol::OperationMeta {
        protocol::OperationMeta {
            operation_id: None,
            dependencies: Vec::new(),
            base_version: 0,
            author_id: None,
            timestamp: protocol::HybridLogicalTimestamp::new(actor, physical_ms),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
        }
    }

    let mut group = c.benchmark_group("crdt_merge");
    let a = BenchDiff { field_a: Some(1), field_b: Some(2) };
    let b = BenchDiff { field_a: Some(3), field_b: None };
    let ma = meta_at(1, 10);
    let mb = meta_at(2, 20);
    for strategy in [
        protocol::MergeStrategyKind::LwwRegister,
        protocol::MergeStrategyKind::OrderedSequence,
        protocol::MergeStrategyKind::TextSequence,
        protocol::MergeStrategyKind::TombstonedGraphSet,
        protocol::MergeStrategyKind::ContentAddressedBlob,
    ] {
        group.bench_with_input(BenchmarkId::new("merge_concurrent_diffs", format!("{strategy:?}")), &strategy, |bencher, &strategy| {
            bencher.iter(|| black_box(protocol::merge_concurrent_diffs(strategy, a.clone(), b.clone(), &ma, &mb)));
        });
    }
    group.finish();
}
//#endregion 🔖️Crdt

//#region 🔖️OpDag
fn bench_op_dag_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("op_dag_insert");
    for &node_count in &[8usize, 64, 256] {
        let envelopes = OpDagGen::new(6).generate(node_count);
        group.bench_with_input(BenchmarkId::new("insert_in_topological_order", node_count), &envelopes, |b, envelopes| {
            b.iter(|| {
                let mut dag = protocol::OpDag::new();
                for envelope in envelopes {
                    dag.insert(envelope.clone()).expect("insert");
                }
                black_box(dag.drain_applied_envelopes())
            });
        });
    }
    group.finish();
}
//#endregion 🔖️OpDag

criterion_group!(protocol_benches, bench_history_log_gen, bench_encode_decode_history, bench_history_appender, bench_recover, bench_wire_frame_codec, bench_crdt_merge, bench_op_dag_insert);
criterion_main!(protocol_benches);
