//! 🧰️ Round-trip assertions shared by every technology crate's `Mutation` test suite: demo owners, preparation doubles, scratch directories and the language-neutral mutation report.

#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant};
use semio_framework_os_store_read::{ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactStoreEnvelopeRetirement};
use semio_framework_os_spr::{ActorId, ArtifactId, Edit, Mutation, OpBinary, OpText, SchemaId};
use std::sync::Arc;
use semio_framework_os_store_canonical_json::ArtifactCanonicalJsonTree;
use semio_framework_os_store_space_schema::{ArtifactCommand, ArtifactEnvelope, HistoryLane};
use pack::snapshot::{ArtifactDsl, ArtifactPack};
use semio_framework_os_store_operation_wire::{ArtifactOperationText, ArtifactOperationTextNode, ArtifactPreparedOperationError, ArtifactPreparedOperationSource};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, DslValue, ErasedSnapshotRetirement, FromValue, SnapshotRetirementFactory, ToValue, ValueError};
use semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_one_item_publication::batch_digest::ArtifactStoreBatchDigest;
use semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreCursorDisposer;
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest};
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use semio_framework_os_store_engine_core::lifted_store::CommandReceipt;
use semio_framework_os_store_config::lifted_store::ConfigRecord;
use semio_framework_os_store_engine_owners::{DocumentStoreOwners, complete_unscheduled_catalog};
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_store_text_io::text_format::{ParsedDocumentText, parse_document_pack, parse_document_text, print_document_pack, print_document_text};
use std::marker::PhantomData;
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use serde::Serialize;
use semio_framework_os_vcs::{VcsError, apply_mutation};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_store_replay::apply_outcome;
use semio_framework_os_store_document::create_document_envelope;
use semio_framework_os_store_space::lifted_store::install_unscheduled_catalog;
use semio_framework_os_store_space_member::lifted_document_materialize::materialize_document_snapshot;
use semio_framework_os_store_command::{parse_command, print_command};

//#region 🧪️MutationReport
/// 🧪️ The report every language-neutral mutation case compares against its committed specification vector. The
/// base and expected-after snapshots and the mutation are decoded through the production JSON codec, the mutation is
/// applied through `protocol::apply_diff` (so an apply refusal surfaces as the fatal message production
/// dispatch records), and the mutation's own computed inverse steps are replayed in order onto the applied snapshot.
/// The forward half is `base`, `expectedSnapshot`, `snapshot`, `diff`, `messages`; the inverse half is
/// `inverseSteps`, `inverseSnapshot`, `inverseMessages`. `expectedSnapshot` is decoded through the same path as
/// `base`, so a caller compares like with like.
///
/// @see ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the laws case adapters assert over this report.
pub fn mutation_report_json<S, M>(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String>
where
    S: Clone + ToValue + FromValue,
    M: Mutation<S>,
{
    let decode_snapshot = |text: &str| -> Result<S, String> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: M = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (applied, forward) = apply_outcome(&base, mutation.diff(&base));
    let inverse = mutation.inverse(&base).map_err(ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let (next, step_outcome) = apply_outcome(&undone, step.diff(&undone));
        undone = next;
        inverse_messages.extend(step_outcome.messages().iter().cloned());
    }
    let json = |value: DslValue| semio_framework_pack_json::from_dsl_value(&value);
    let report = semio_framework_pack_json::object([
        ("base".to_string(), json(base.to_value())),
        ("expectedSnapshot".to_string(), json(expected.to_value())),
        ("snapshot".to_string(), json(applied.to_value())),
        ("diff".to_string(), json(forward.diff().to_value())),
        ("messages".to_string(), json(forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), json(inverse.to_value())),
        ("inverseSnapshot".to_string(), json(undone.to_value())),
        ("inverseMessages".to_string(), json(inverse_messages.to_value())),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
//#endregion 🧪️MutationReport

/// 🧬️ Order-preserving test projection for an independent JSON byte encoder.
#[cfg(test)]
pub struct SerdeValue<'a>(pub &'a DslValue);

#[cfg(test)]
impl Serialize for SerdeValue<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::{SerializeMap, SerializeSeq};
        match self.0 {
            semio_framework_value::DslValue::Null => serializer.serialize_unit(),
            semio_framework_value::DslValue::Bool(value) => serializer.serialize_bool(*value),
            semio_framework_value::DslValue::Number(value) => serde_json::Value::from(semio_framework_value::DslValue::Number(*value)).serialize(serializer),
            semio_framework_value::DslValue::String(value) => serializer.serialize_str(value),
            semio_framework_value::DslValue::Bytes(value) => serde::Serialize::serialize(value, serializer),
            semio_framework_value::DslValue::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    sequence.serialize_element(&SerdeValue(value))?;
                }
                sequence.end()
            }
            semio_framework_value::DslValue::Object(values) => {
                let mut map = serializer.serialize_map(Some(values.len()))?;
                for (key, value) in values {
                    map.serialize_entry(key, &SerdeValue(value))?;
                }
                map.end()
            }
        }
    }
}

//#region 📁️ScratchDirectory
static TEMP_DIR_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 📁️ Process-unique test scratch directory removed recursively on drop.
#[derive(Debug)]
pub struct TempDir {
    path: std::path::PathBuf,
}

impl TempDir {
    /// 🧭️ Returns the owned scratch path.
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// 🧪️ Creates a cross-platform process-unique scratch directory using only the standard library.
pub fn tempdir() -> std::io::Result<TempDir> {
    let epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
    for _ in 0..1024 {
        let sequence = TEMP_DIR_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("semio-test-{}-{epoch}-{sequence}", std::process::id()));
        match std::fs::create_dir(&path) {
            Ok(()) => return Ok(TempDir { path }),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "could not allocate a unique Semio scratch directory"))
}

#[cfg(test)]
include!("../🧪️tests/🔬️test-support-scratch-directory/🦀️.rs");
//#endregion 📁️ScratchDirectory

/// 🔁️ Asserts that applying `operation` then applying its reversed `inverse(pre)` restores `pre`.
pub async fn assert_operation_round_trip<P, Mutation>(pre: &P, operation: Mutation)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Mutation: self::Mutation<P>,
{
    let post = apply_mutation(pre, &operation).expect("test operation diff applies").0;
    let mut inverse = operation.inverse(pre).expect("valid retained mutation inverse fixture");
    inverse.reverse();
    let mut restored = post;
    for back_operation in &inverse {
        restored = apply_mutation(&restored, back_operation).expect("test inverse diff applies").0;
    }
    assert_eq!(&restored, pre, "operation inverse did not restore pre-state");
}

/// 🧪️ Dispatches one command under a fixture caller-owned identity authority and returns its original cumulative receipt to the caller.
pub async fn dispatch_test_command<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, command: ArtifactCommand<Mutation>) -> Result<CommandReceipt, VcsError>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    const CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
        assert!(progress.owned_bytes <= CEILING);
        true
    };
    let mut identity: EntityIdentityAuthority<'_> = EntityIdentityAuthority::new(CEILING, &mut observer as &mut semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("declared fixture identity authority");
    let result = store.dispatch(command, &mut identity).await;
    drop(identity.pause().expect("original cumulative fixture identity receipt"));
    result
}

/// 🗄️ Asserts a full store round trip: Apply→Undo restores `initial`, Redo restores the
/// post-apply snapshot, and replay-materialization agrees with the live store snapshot.
pub async fn assert_store_roundtrip<P, Mutation>(initial: P, operation: Mutation)
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + Sync + ArtifactCanonicalJsonTree + 'static,
{
    let envelope = create_document_envelope("test/v1", "test", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = plain_test_store(envelope).await;
    dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![operation], transaction: None }).await.expect("apply");
    let post = store.snapshot().expect("post snapshot");
    dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("undo snapshot"), initial, "undo did not restore initial snapshot");
    dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo");
    assert_eq!(store.snapshot().expect("redo snapshot"), post, "redo did not restore post snapshot");
    let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("replay");
    assert_eq!(replayed, post, "materialization from replay diverged from store snapshot");
    close_plain_test_store(&mut store);
}

/// 📜️ Asserts a DSL round trip: `P::parse_dsl(&snapshot.print_dsl())` recovers an equal
/// snapshot. The compile-time validation ground truth for every technology's `🔖️Dsl` region —
/// call this from a `#[test]` over every `include_str!` fixture.
pub fn assert_dsl_round_trip<P>(snapshot: &P)
where
    P: ArtifactDsl + PartialEq + std::fmt::Debug,
{
    let printed = snapshot.print_dsl();
    let parsed = P::parse_dsl(&printed).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
    assert_eq!(&parsed, snapshot, "dsl round trip diverged;\nprinted:\n{printed}");
}

/// 🧊️ Cold twin of [`assert_dsl_round_trip`] for a projection that REJECTS a bare drop —
/// one owning an `OrderedMap` root or a retirement ladder (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`).
/// The decoded value is handed to `retire` instead of being dropped, so the law can be asserted
/// without aborting the test binary in a destructor.
pub fn assert_dsl_round_trip_cold<P>(snapshot: &P, retire: impl FnOnce(P))
where
    P: ArtifactDsl + PartialEq + std::fmt::Debug,
{
    let printed = snapshot.print_dsl();
    let parsed = P::parse_dsl(&printed).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
    let matches = parsed == *snapshot;
    let report = matches.then(String::new).unwrap_or_else(|| format!("{parsed:?}"));
    retire(parsed);
    assert!(matches, "dsl round trip diverged;\nprinted:\n{printed}\nparsed:\n{report}");
}

/// 🧮️ Config artifact twin of [`assert_dsl_round_trip`] — same law for `ConfigRecord` snapshots.
pub fn assert_config_round_trip<C>(snapshot: &C)
where
    C: ConfigRecord + PartialEq + std::fmt::Debug,
{
    assert_dsl_round_trip(snapshot);
}

/// 🧭️ Non-panicking twin of [`assert_dsl_round_trip`] for a repo-wide fixture-law SWEEP
/// (W6: `.🧬semio/🦑️repo/🎫️tickets/.../DSL-FIXTURE-LAW-SWEEP`): checks BOTH laws directly against real
/// shipped `📚️examples/**` fixture TEXT (not a hand-built in-memory value), which is exactly what
/// a single per-app round-trip test built on its own simpler hardcoded example can miss — a
/// printer/parser asymmetry only a real fixture's actual formatting (comment placement, field
/// order, quoting) would trip.
///
/// **Law 1 — parse→print→reparse fixpoint**: `text` parses to `first`; printing then reparsing
/// `first` must recover an equal value (`second`) — the generic form of [`assert_dsl_round_trip`],
/// but starting from arbitrary fixture text instead of an already-canonical in-memory value.
///
/// **Law 2 — canonicalize idempotence**: `canonicalize(x) := print_dsl(parse_dsl(x))`. For every
/// derive-generated `ArtifactDsl` impl this IS `crate::os_dsl::schema::canonicalize(x, spec, opts)`
/// (`__rt::print_document_record`/`parse_document_record` route straight through
/// `crate::os_dsl::schema::parse`/`print` in `JoinMode::Document`, the exact pair `canonicalize` composes)
/// — and it is the correct generalization for hand-rolled (Route A idiom) `ArtifactDsl` impls
/// that have no `RecordSpec` at all. `canonicalize(text) == printed_once`;
/// `canonicalize(printed_once) == printed_twice`; idempotence is `printed_once == printed_twice`.
///
/// Returns `Ok(())` on success, `Err(description)` on the first law violated — never panics, so
/// a caller sweeping many fixture files can collect every failure before reporting.
pub fn check_dsl_fixture_text_laws<P>(text: &str) -> Result<(), String>
where
    P: ArtifactDsl + PartialEq,
{
    let first = P::parse_dsl(text).map_err(|error| format!("parse failed: {error}"))?;
    let printed_once = first.print_dsl();
    let second = P::parse_dsl(&printed_once).map_err(|error| format!("reparse failed: {error}\nprinted:\n{printed_once}"))?;
    if first != second {
        return Err(format!("parse->print->reparse fixpoint diverged;\nprinted:\n{printed_once}"));
    }
    let printed_twice = second.print_dsl();
    if printed_once != printed_twice {
        return Err(format!("canonicalize is not idempotent;\nonce:\n{printed_once}\ntwice:\n{printed_twice}"));
    }
    Ok(())
}

/// 📦️ Asserts a pack round trip: `P::decode_pack(&snapshot.encode_pack())` recovers an
/// equal snapshot — the pack sibling of `assert_dsl_round_trip`.
pub fn assert_pack_round_trip<P>(snapshot: &P)
where
    P: ArtifactPack + PartialEq + std::fmt::Debug,
{
    let bytes = snapshot.encode_pack();
    let decoded = P::decode_pack(&bytes).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
    assert_eq!(&decoded, snapshot, "pack round trip diverged");
}

/// 🧬️ Asserts a document kind has a structural pack-schema identity: `record_spec()` is
/// declared, its `schema_hash` is nonzero, and `snapshot` round-trips exactly through the pack.
pub fn assert_pack_schema_identity<P>(snapshot: &P) -> [u8; 32]
where
    P: ArtifactPack + PartialEq + std::fmt::Debug,
{
    let spec = P::record_spec().unwrap_or_else(|| panic!("{} declares no pack record spec", std::any::type_name::<P>()));
    let hash = semio_framework_os_pack::schema_hash(&spec);
    assert_ne!(hash, [0u8; 32], "pack schema hash must be nonzero");
    let decoded = P::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
    assert_eq!(&decoded, snapshot, "pack round trip diverged from source snapshot");
    hash
}

/// ⚖️ Asserts dsl and pack are two encodings of the SAME value: `decode_pack(
/// encode_pack(p)) == parse_dsl(print_dsl(p)) == p` — the compile-time validation ground truth
/// for the whole pack rollout's central LAW (see `ArtifactPack`'s doc comment).
pub fn assert_dsl_pack_equivalence<P>(snapshot: &P)
where
    P: ArtifactDsl + ArtifactPack + Clone + PartialEq + std::fmt::Debug,
{
    let via_pack = P::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
    let via_dsl = P::parse_dsl(&snapshot.print_dsl()).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
    assert_eq!(&via_pack, snapshot, "pack round trip diverged from source snapshot");
    assert_eq!(&via_dsl, snapshot, "dsl round trip diverged from source snapshot");
    assert_eq!(via_pack, via_dsl, "pack and dsl round trips diverged from each other");
}

/// 🧊️ Cold twin of [`assert_dsl_pack_equivalence`] for a projection that REJECTS a bare
/// drop — both decoded values are handed to `retire` instead of being dropped.
pub fn assert_dsl_pack_equivalence_cold<P>(snapshot: &P, mut retire: impl FnMut(P))
where
    P: ArtifactDsl + ArtifactPack + Clone + PartialEq + std::fmt::Debug,
{
    let via_pack = P::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
    let via_dsl = P::parse_dsl(&snapshot.print_dsl()).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
    let pack_matches = via_pack == *snapshot;
    let dsl_matches = via_dsl == *snapshot;
    let agree = via_pack == via_dsl;
    retire(via_pack);
    retire(via_dsl);
    assert!(pack_matches, "pack round trip diverged from source snapshot");
    assert!(dsl_matches, "dsl round trip diverged from source snapshot");
    assert!(agree, "pack and dsl round trips diverged from each other");
}

/// 🧊️ Cold twin of [`assert_pack_round_trip`] for a projection that REJECTS a bare drop.
pub fn assert_pack_round_trip_cold<P>(snapshot: &P, retire: impl FnOnce(P))
where
    P: ArtifactPack + PartialEq + std::fmt::Debug,
{
    let bytes = snapshot.encode_pack();
    let decoded = P::decode_pack(&bytes).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
    let matches = decoded == *snapshot;
    retire(decoded);
    assert!(matches, "pack round trip diverged");
}

/// ⚡️ Asserts an op-text round trip for a single operation: `print_op` contains no newline
/// and `Op::parse_op` recovers an equal operation from it. The compile-time validation ground
/// truth for every technology's `🔖️OpText` region — call this once per `Mutation` variant.
pub fn assert_op_line_round_trip<Op>(operation: &Op)
where
    Op: OpText + PartialEq + std::fmt::Debug,
{
    let printed = operation.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
    let parsed = Op::parse_op(&printed).unwrap_or_else(|error| panic!("op parse failed: {error}"));
    assert_eq!(&parsed, operation, "op-text round trip diverged; printed: {printed:?}");
}

pub use semio_framework_value::testing::assert_wire_witness;

/// 🧊️ Cold twin of [`assert_op_line_round_trip`] for an operation that REJECTS a bare drop —
/// one carrying an `OrderedMap` root or a retirement ladder (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`),
/// e.g. a mutation payload holding a `neural_engine::Dictionary`. Every operation this helper
/// decodes is handed to `retire` instead of being dropped, so the law can be asserted without
/// tripping a destructor guard. The op-level mirror of [`assert_dsl_round_trip_cold`].
pub fn assert_op_line_round_trip_cold<Op>(operation: &Op, retire: impl FnOnce(Op))
where
    Op: OpText + PartialEq + std::fmt::Debug,
{
    let printed = operation.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
    let parsed = Op::parse_op(&printed).unwrap_or_else(|error| panic!("op parse failed: {error}"));
    let matches = parsed == *operation;
    let report = matches.then(String::new).unwrap_or_else(|| format!("{parsed:?}"));
    retire(parsed);
    assert!(matches, "op-text round trip diverged; printed: {printed:?}\nparsed:\n{report}");
}

/// 🧊️ Cold twin of [`assert_op_text_binary_equivalence`]; `retire` takes exact ownership of
/// every operation this helper decodes (one from the text lane, one from the binary lane).
pub fn assert_op_text_binary_equivalence_cold<Op>(operation: &Op, mut retire: impl FnMut(Op))
where
    Op: OpText + OpBinary + PartialEq + std::fmt::Debug,
{
    assert_op_line_round_trip_cold(operation, &mut retire);
    let encoded = operation.encode_op().unwrap_or_else(|error| panic!("op encode failed: {error}"));
    let encoded_again = operation.encode_op().unwrap_or_else(|error| panic!("op re-encode failed: {error}"));
    assert_eq!(encoded, encoded_again, "op binary encoding is not deterministic");
    let decoded = Op::decode_op(&encoded).unwrap_or_else(|error| panic!("op decode failed: {error}"));
    let matches = decoded == *operation;
    let report = matches.then(String::new).unwrap_or_else(|| format!("{decoded:?}"));
    retire(decoded);
    assert!(matches, "op-binary round trip diverged from source operation\ndecoded:\n{report}");
}

/// ⚖️ Asserts op text and op binary are two encodings of the SAME operation:
/// `decode_op(encode_op(op)) == parse_op(print_op(op)) == op`, and the binary encoding is
/// deterministic. The compile-time validation ground truth for every technology's `OpBinary`
/// impl — the op-level mirror of {@link assert_dsl_pack_equivalence}.
pub fn assert_op_text_binary_equivalence<Op>(operation: &Op)
where
    Op: OpText + OpBinary + PartialEq + std::fmt::Debug,
{
    assert_op_line_round_trip(operation);
    let encoded = operation.encode_op().unwrap_or_else(|error| panic!("op encode failed: {error}"));
    let encoded_again = operation.encode_op().unwrap_or_else(|error| panic!("op re-encode failed: {error}"));
    assert_eq!(encoded, encoded_again, "op binary encoding is not deterministic");
    let decoded = Op::decode_op(&encoded).unwrap_or_else(|error| panic!("op decode failed: {error}"));
    assert_eq!(&decoded, operation, "op-binary round trip diverged from source operation");
}

/// ⚖️ Asserts command text and command binary are two encodings of the SAME command:
/// `ArtifactCommand::decode_command(&c.encode_command()) == parse_command(print_command(c)) == c`, and the
/// binary encoding is deterministic. The compile-time validation ground truth for
/// `ArtifactCommand`'s text/binary pair — the command-level mirror of
/// `assert_op_text_binary_equivalence`.
pub async fn assert_command_text_binary_equivalence<P, Op>(command: &ArtifactCommand<Op>)
where
    Op: OpText + OpBinary + Mutation<P> + PartialEq + std::fmt::Debug,
{
    let printed = print_command(command).await.unwrap_or_else(|error| panic!("command print failed: {error}"));
    let parsed: ArtifactCommand<Op> = parse_command::<P, Op>(&printed).await.unwrap_or_else(|error| panic!("command parse failed: {error}"));
    assert_eq!(&parsed, command, "command text round trip diverged; printed:\n{printed}");
    let encoded = command.encode_command().unwrap_or_else(|error| panic!("command encode failed: {error}"));
    let encoded_again = command.encode_command().unwrap_or_else(|error| panic!("command re-encode failed: {error}"));
    assert_eq!(encoded, encoded_again, "command binary encoding is not deterministic");
    let decoded: ArtifactCommand<Op> = ArtifactCommand::<Op>::decode_command::<P>(&encoded).unwrap_or_else(|error| panic!("command decode failed: {error}"));
    assert_eq!(&decoded, command, "command binary round trip diverged from source command");
}

/// 📄️ Asserts that printing a store's envelope to text and parsing it back keeps its whole shared log — every edit
/// and every history transition — and hydrates to the canonical trunk tip (the `.ops` text is the shared log and
/// carries no viewer head, ticket 26/10/01 PER-VIEWER-ALTERNATIVE-HEAD); a store on the trunk tip gets back exactly the
/// envelope and live snapshot it holds. The ground truth for {@link print_document_text}/{@link parse_document_text}
/// on any technology once it implements `ArtifactDsl` + `OpText`.
pub async fn assert_document_text_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
where
    P: Clone + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + OpText + self::Mutation<P> + PartialEq + ToValue + FromValue + OpBinary + Send + 'static,
{
    let live = store.snapshot().expect("store snapshot");
    let files = print_document_text(store.envelope()).await.expect("print document text");
    let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
    let shared = parsed.envelope.vcs.edits.iter().map(|edit| (&edit.id, &edit.forwards, &edit.mutation_meta)).eq(store.envelope().vcs.edits.iter().map(|edit| (&edit.id, &edit.forwards, &edit.mutation_meta)));
    assert!(shared && parsed.envelope.transitions == store.envelope().transitions, "document-text round trip lost shared history");
    assert!(parsed.envelope.active_alternative_id.is_none() && parsed.envelope.viewer_checkpoint_id.is_none(), "document text hydrates to the canonical trunk tip");
    if store.envelope().active_alternative_id.is_none() && store.envelope().viewer_checkpoint_id.is_none() {
        assert!(&parsed.envelope == store.envelope(), "document-text round trip lost durable history");
        assert_eq!(parsed.snapshot, live, "document-text round trip diverged from store snapshot");
    }
    retire_parsed_document(parsed);
}

/// 🧹️ Bounded retirement for a parse-produced document that belongs to no store.
/// Every terminal shell below `ParsedDocumentText` (`ArtifactEnvelope`, its `ArtifactVcs`, the
/// history ledger, each `Edit`) asserts in `Drop` that its nested owners were retired through the
/// bounded protocol first, and only an `ArtifactStore` ever runs that protocol — so a helper that
/// parses an envelope purely to compare it must drive `ArtifactStoreEnvelopeRetirement` itself
/// rather than let the value fall out of scope.
pub fn retire_parsed_document<P, Mutation>(parsed: ParsedDocumentText<P, Mutation>)
where
    P: Send + Sync + 'static,
    Mutation: Send + 'static,
{
    let ParsedDocumentText { envelope, snapshot } = parsed;
    let mut retirement = ArtifactStoreEnvelopeRetirement::new(envelope, Arc::new(RoundTripValueRetirementFactory), Arc::new(RoundTripValueRetirementFactory));
    drive_retirement(&mut retirement).expect("parsed document retires within its quoted demands");
    assert!(retirement.terminal_is_empty(), "parsed document must reach its terminal-empty shell");
    drop(snapshot);
}

/// 🧪️ Retirement authorities for a test document whose snapshot and mutations are plain
/// data: every value retires in one bounded step, and the store closes through the real cursor
/// disposer — so a test store records history and closes exactly like a production member.
pub fn plain_document_store_owners<P, Mutation>() -> DocumentStoreOwners<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + Sync + ArtifactCanonicalJsonTree + 'static,
{
    let preparation = RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<PlainAuthoringFactory<P, Mutation>>(), depth: 1 };
    let birth = DocumentStoreOwners::<P, Mutation>::source_birth_bytes::<RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, ArtifactStoreCursorDisposer<P, Mutation>>().expect("plain test catalog source layout") + preparation.capacity_bytes;
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: birth, maximum_depth: 1, ..Default::default() };
    complete_unscheduled_catalog(DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, ArtifactStoreCursorDisposer::<P, Mutation>::new(), Arc::new(PlainAuthoringFactory::<P, Mutation>(PhantomData)) as Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>))).expect("plain test catalog completes its funded constructor tickets")
}

/// 🧾️ A plain test mutation viewed as its `OpBinary` image, one scalar leaf. The image is re-encoded per bounded copy turn
/// and copied out by offset, so the wire owns no storage and nothing outlives the borrow of the original mutation.
#[repr(transparent)]
struct PlainOperationImage<M>(M);

impl<M: OpBinary + Sync> ArtifactOperationText for PlainOperationImage<M> {
    fn operation_text_node(&self, path: &[usize]) -> Result<ArtifactOperationTextNode<'_>, ValueError> {
        if path.is_empty() { Ok(ArtifactOperationTextNode::Scalar) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "plain operation image has no children")) }
    }

    fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), ArtifactPreparedOperationError> {
        if !path.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "plain operation image has no children").into()); }
        let image = self.0.encode_op().map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
        let count = output.len().min(image.len().saturating_sub(offset));
        output[..count].copy_from_slice(&image[offset..offset + count]);
        Ok((count, offset + count >= image.len()))
    }
}

/// 🏭️ Authoring-only semantic authority of a plain test catalog: it names each mutation's `OpBinary` image as its
/// wire, borrowed from the original mutation, and prepares no retained gesture.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct PlainAuthoringFactory<P: 'static, M: 'static>(PhantomData<fn() -> (P, M)>);

impl<P: Send + Sync + 'static, M: OpBinary + Sync + 'static> ArtifactStoreOneItemPreparationFactory<P, M> for PlainAuthoringFactory<P, M> {
    fn begin_batch_digest(&self, _edit: &mut Option<Box<Edit<M>>>, _grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError> {
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plain test catalog prepares no retained gesture"))
    }

    fn operation_wire_source<'a>(&self, mutation: &'a M) -> Option<ArtifactPreparedOperationSource<'a>> {
        let body: &'a PlainOperationImage<M> = unsafe { &*(mutation as *const M).cast::<PlainOperationImage<M>>() };
        Some(ArtifactPreparedOperationSource::Text { header: b"plain", body })
    }

    fn preflight(&self, _mutation: &M, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Err("plain test catalog prepares no retained gesture".into())
    }

    fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plain test catalog prepares no retained gesture"))
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, M, M>, _grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        Err((ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plain test catalog prepares no retained gesture"), request))
    }
}

/// 🧪️ A store over `envelope` carrying [`plain_document_store_owners`].
pub async fn plain_test_store<P, Mutation>(envelope: ArtifactEnvelope<P, Mutation>) -> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + Sync + ArtifactCanonicalJsonTree + 'static,
{
    let mut store = ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("test fixture history is valid");
    install_unscheduled_catalog(&mut store, Ok(plain_document_store_owners())).expect("test store installs its funded plain catalog");
    store
}

/// 🧹️ Releases a test store's backbone and drives its owned close to the terminal-empty
/// witness its `Drop` requires.
pub fn close_plain_test_store<P, Mutation>(store: &mut ArtifactStore<P, Mutation>)
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    drop(store.detach_backbone().expect("a closing test store releases its backbone"));
    store.close_owned_unscheduled().expect("test store reaches its exact terminal-empty witness under its quoted owner demands");
}

impl<P: Send + Sync + 'static> SnapshotRetirementFactory<P> for RoundTripValueRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<P>) -> usize { size_of::<RoundTripValueRetirement<Arc<P>>>() }

    fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)> {
        admit_artifact_retirement(snapshot, grant, |snapshot| RoundTripValueRetirement(Some(snapshot)))
    }
}

/// 🧹️ One-value retirement used only by [`retire_parsed_document`] — a parsed comparison
/// document owns no domain resources, so releasing a value is exactly one bounded step.
struct RoundTripValueRetirement<T>(Option<T>);

impl<T: Send> ErasedSnapshotRetirement for RoundTripValueRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.0.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "plain test retirement requires admitted depth")); }
        drop(self.0.take());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.0.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.0.is_some())) }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RoundTripValueRetirementFactory;

impl<T: Send + 'static> ArtifactOwnedValueRetirementFactory<T> for RoundTripValueRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &T) -> usize { size_of::<RoundTripValueRetirement<T>>() }

    fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)> {
        admit_artifact_retirement(value, grant, |value| RoundTripValueRetirement(Some(value)))
    }
}

/// 🧪️ Drives one erased retirement to its terminal-empty witness, granting each independent axis exactly its quoted demand.
pub fn drive_retirement(owner: &mut dyn ErasedSnapshotRetirement) -> Result<(), ValueError> {
    use semio_framework_value::retained_clone::admit_retained_clone_close;
    for _ in 0..ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS {
        if owner.terminal_is_empty() { return Ok(()); }
        let copy = owner.next_copy_byte_demand()?.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy)?, maximum_release_bytes: owner.next_release_byte_demand()?, maximum_depth: owner.next_depth_demand()?.max(1) };
        let step = owner.close_step(grant)?;
        admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "driven test retirement")?;
        if matches!(step, RetainedCloneStep::Complete(_)) { return Ok(()); }
    }
    Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "driven test retirement did not reach its terminal-empty witness"))
}

/// 🗄️ Asserts a full pack-based document round trip: mirrors
/// `assert_document_text_round_trip` but via `print_document_pack`/`parse_document_pack`. The `.spr` pack persists this
/// replica's viewer head (`REC_VIEWER`), so the parsed pack stands on the store's alternative and checkpoint and shows
/// its live snapshot; on the trunk tip it also agrees with the text path's snapshot — the two storage formats must never
/// diverge on the same store.
pub async fn assert_document_pack_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
where
    P: Clone + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + OpText + OpBinary + self::Mutation<P> + PartialEq + ToValue + FromValue + Send + 'static,
{
    let live = store.snapshot().expect("store snapshot");
    let pack_files = print_document_pack(store.envelope()).await.expect("print document pack");
    let parsed_pack: ParsedDocumentText<P, Mutation> = parse_document_pack(&pack_files.pack, &pack_files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
    assert_eq!(parsed_pack.snapshot, live, "document-pack round trip diverged from store snapshot");
    let viewer = (&parsed_pack.envelope.active_alternative_id, &parsed_pack.envelope.viewer_checkpoint_id);
    assert_eq!(viewer, (&store.envelope().active_alternative_id, &store.envelope().viewer_checkpoint_id), "document-pack round trip lost the viewer head");

    let text_files = print_document_text(store.envelope()).await.expect("print document text");
    let parsed_text: ParsedDocumentText<P, Mutation> = parse_document_text(&text_files.dsl, &text_files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
    if viewer == (&None, &None) {
        assert_eq!(parsed_pack.snapshot, parsed_text.snapshot, "document-pack path diverged from document-text path");
    }
    retire_parsed_document(parsed_pack);
    retire_parsed_document(parsed_text);
}

/// ✉️ Asserts that converting an `Edit<Mutation>` into `crate::os_spr::MutationEnvelope`s
/// (`protocol_causal`'s canonical wire/causal representation, moved from `framework/core` in CW3,
/// via `crate::os_spr::mutation_envelope_from_edit`) preserves every operation's essential facts —
/// the causal-wire sibling of `assert_pack_round_trip`/`assert_dsl_round_trip` for the app
/// fan-out's "pack laws" cluster.
///
/// `MutationEnvelope` is a runtime struct that is never itself re-serialized back into an
/// `Edit` (unlike `encode_pack`/`decode_pack`, there is no `envelope_to_edit` inverse — vcs's OWN
/// `edit_from_operation_envelope` recovers a *whole edit* from vcs's own, differently-shaped,
/// per-edit `semio_framework_replication::MutationEnvelope`, not from this per-operation
/// `protocol_causal` one), so a byte-level encode-then-decode law is not meaningful here.
/// Instead this checks the two LAWS that actually matter for this bridge: (1) whatever
/// `edit.mutation_meta` explicitly recorded for a slot (the ground-truth source
/// `mutation_envelope_from_edit` prefers over its own `Mutation`-trait/structural fallbacks —
/// see that function's own doc comment) survives unchanged onto the envelope's
/// `mutation_id`/`dependencies`/`actor`/`timestamp`; and (2) `envelope.diff.payload`/
/// `envelope.inverse.inverse_diff` decode back (via `Mutation`'s own `Deserialize` impl) into
/// operations equal to `edit.forwards[i]`/`edit.inverse[i]` — the part a hand-rolled
/// `Serialize`/`Deserialize` pair can silently break. Deliberately does NOT recompute the
/// envelope's fallback chain (id/actor/deps when `mutation_meta` is absent) itself, since doing
/// so would just re-run `mutation_envelope_from_edit`'s own logic against itself and always
/// agree — see this function's `🧪️Tests` sibling for a deliberately lossy `Mutation` impl that
/// trips law (2).
pub async fn assert_command_envelope_round_trip<P, Mutation>(edit: &Edit<Mutation>, document_id: &ArtifactId, schema: &SchemaId)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Mutation: self::Mutation<P> + PartialEq + std::fmt::Debug + OpBinary,
{
    let envelopes = semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, document_id, schema).unwrap_or_else(|error| panic!("mutation_envelope_from_edit must succeed for a well-formed edit: {error}"));
    assert_eq!(envelopes.len(), edit.forwards.len(), "one envelope must be produced per forward operation");
    for (index, envelope) in envelopes.iter().enumerate() {
        assert_eq!(envelope.document_id, *document_id, "document id did not survive the envelope conversion");
        if let Some(meta) = edit.mutation_meta.get(index) {
            if let Some(mutation_id) = &meta.mutation_id {
                assert_eq!(&envelope.mutation_id, mutation_id, "explicit operation id did not survive the envelope conversion");
            }
            assert_eq!(envelope.dependencies, meta.dependencies, "explicit dependencies did not survive the envelope conversion");
            if let Some(author_id) = &meta.author_id {
                assert_eq!(&envelope.actor, author_id, "explicit author id did not survive the envelope conversion");
            }
            assert_eq!(envelope.timestamp, meta.timestamp, "explicit timestamp did not survive the envelope conversion");
        }
        let recovered_forward = Mutation::decode_op(&envelope.diff.payload).unwrap_or_else(|error| panic!("envelope diff payload must decode back into an equal operation: {error}"));
        assert_eq!(&recovered_forward, &edit.forwards[index], "envelope diff payload did not decode back into an equal forward operation");
        match edit.inverse.get(index) {
            Some(backward) => {
                let recovered_backward = Mutation::decode_op(&envelope.inverse.payload).unwrap_or_else(|error| panic!("envelope inverse payload must decode back into an equal operation: {error}"));
                assert_eq!(&recovered_backward, backward, "envelope inverse payload did not decode back into an equal backward operation");
            }
            None => assert!(envelope.inverse.payload.is_empty(), "inverse payload must be empty when the edit has no corresponding inverse op"),
        }
    }
}

/// 🩺️ Asserts the store's incrementally-maintained live snapshot agrees with a
/// from-scratch full replay — the differential check for `ArtifactStore`'s stateful `current`
/// field. Call after arbitrary command sequences (apply/amend/undo/redo/checkpoint/switch
/// interleavings) in a tech's own tests to confirm the incremental fast paths never diverge from
/// the replay ground truth.
pub async fn assert_live_equals_replay<P, Mutation>(store: &ArtifactStore<P, Mutation>)
where
    P: Clone + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let live = store.snapshot().expect("store snapshot");
    let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("replay");
    assert_eq!(live, replayed, "store's live snapshot diverged from full-replay materialization");
}

/// 🎚️ Strongest achieved native IO fidelity for export/reimport laws (S8).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoFidelityClass {
    Exact,
    Canonical,
    Semantic,
    Lossy,
}

/// 🗂️ Vendored real-world example input for subset integrated roundtrip harnesses.
pub struct ExampleAsset<'a> {
    pub bytes: &'a [u8],
    pub text: Option<&'a str>,
    pub provenance: &'a str,
}

/// 🪆 Subset-facing surface the integrated harness needs. Implement on a thin adapter per subset.
pub trait SubsetRoundtripSpec {
    type Snapshot: Clone + PartialEq + std::fmt::Debug + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ToValue + FromValue;
    type Mutation: Clone + PartialEq + std::fmt::Debug + ToValue + FromValue + Mutation<Self::Snapshot> + OpText + OpBinary + Send + Sync + ArtifactCanonicalJsonTree + 'static;
    type Inference: Clone + PartialEq + std::fmt::Debug + Default;
    async fn dialect() -> semio_framework_artifact_reference::ArtifactDialect;
    async fn fidelity() -> IoFidelityClass;
    async fn drops() -> &'static [&'static str];
    async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String>;
    async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String>;
    async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String>;
    async fn infer(snapshot: &Self::Snapshot) -> Result<Self::Inference, ValueError>;
    async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation>;
    async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>>;
    async fn validate_negative(bytes: &[u8]) -> Result<Vec<String>, String>;
    async fn is_derived() -> bool {
        false
    }
}

async fn skip_stage(error: &str) -> bool {
    error.starts_with("SKIP:")
}

async fn skip_validation(codes: &[String]) -> bool {
    codes.len() == 1 && skip_stage(&codes[0]).await
}

/// 🧪 Assert import/export byte fidelity matches the declared class at the raw-byte layer.
pub async fn assert_import_export_fidelity_bytes(original: &[u8], exported: &[u8], class: IoFidelityClass) {
    match class {
        IoFidelityClass::Exact => assert_eq!(exported, original, "exact fidelity requires byte-identical export"),
        IoFidelityClass::Canonical | IoFidelityClass::Semantic | IoFidelityClass::Lossy => {}
    }
}

/// 🎯 Assert two inference runs are identical (determinism law S6).
pub async fn assert_inference_determinism<I: PartialEq + std::fmt::Debug>(a: &I, b: &I) {
    assert_eq!(a, b, "inference is not deterministic across two runs on the same snapshot");
}

/// 🔁 Drive S0–S10 subset roundtrip stages. Stages that need unavailable hooks are skipped only when
/// the corresponding trait method returns an explicit Err starting with "SKIP:"; otherwise failures panic with stage id.
pub async fn assert_subset_roundtrip<S: SubsetRoundtripSpec>(example: &ExampleAsset<'_>, negative: Option<&ExampleAsset<'_>>)
where
    S::Snapshot: Send + Sync + 'static,
    S::Mutation: Send + 'static,
{
    assert!(!example.bytes.is_empty(), "S0: example bytes must be non-empty");
    assert!(!example.provenance.is_empty(), "S0: example provenance must be non-empty");

    let dialect = S::dialect().await;
    assert!(!dialect.artifact_kind.is_empty(), "S1: dialect artifact_kind must be non-empty");
    assert!(!dialect.standard.is_empty(), "S1: dialect standard must be non-empty");
    assert!(!dialect.subset.is_empty(), "S1: dialect subset must be non-empty");

    let snapshot = match S::parse_native(example).await {
        Ok(snapshot) => snapshot,
        Err(error) if skip_stage(&error).await => return,
        Err(error) => panic!("S2 failed: {error}"),
    };

    assert_dsl_round_trip(&snapshot);
    assert_pack_round_trip(&snapshot);
    assert_dsl_pack_equivalence(&snapshot);

    let mutations = S::sample_mutations(&snapshot).await;
    for mutation in &mutations {
        assert_operation_round_trip(&snapshot, mutation.clone()).await;
        assert_op_line_round_trip(mutation);
        assert_op_text_binary_equivalence(mutation);
    }

    let inference_a = S::infer(&snapshot).await.expect("valid materialized subset inference");
    let inference_b = S::infer(&snapshot).await.expect("valid materialized subset inference");
    assert_inference_determinism(&inference_a, &inference_b).await;

    if let Some(first) = mutations.first() {
        assert_store_roundtrip(snapshot.clone(), first.clone()).await;
    }

    match S::export_native(&snapshot).await {
        Ok(exported) => {
            assert_import_export_fidelity_bytes(example.bytes, &exported, S::fidelity().await).await;
            match S::reimport_native(&exported).await {
                Ok(reimported) => match S::fidelity().await {
                    IoFidelityClass::Exact => {}
                    IoFidelityClass::Canonical | IoFidelityClass::Semantic => {
                        assert_eq!(&reimported, &snapshot, "S8: canonical/semantic fidelity requires equal snapshot after reimport");
                    }
                    IoFidelityClass::Lossy if S::drops().await.is_empty() => {
                        assert_eq!(&reimported, &snapshot, "S8: lossy fidelity with empty drop set requires equal snapshot after reimport");
                    }
                    IoFidelityClass::Lossy => {}
                },
                Err(error) if skip_stage(&error).await => {}
                Err(error) => panic!("S8 reimport failed: {error}"),
            }
        }
        Err(error) if skip_stage(&error).await => {}
        Err(error) => panic!("S8 export failed: {error}"),
    }

    match S::validate_payload(example.bytes).await {
        Ok(()) => {}
        Err(codes) if skip_validation(&codes).await => {}
        Err(codes) => panic!("S9 validate_payload failed: {codes:?}"),
    }

    if S::is_derived().await {
        if let Some(negative) = negative {
            match S::validate_negative(negative.bytes).await {
                Ok(codes) => assert!(!codes.is_empty(), "S9: derived negative must yield non-empty diagnostic codes"),
                Err(error) if skip_stage(&error).await => {}
                Err(error) => panic!("S9 validate_negative failed: {error}"),
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
