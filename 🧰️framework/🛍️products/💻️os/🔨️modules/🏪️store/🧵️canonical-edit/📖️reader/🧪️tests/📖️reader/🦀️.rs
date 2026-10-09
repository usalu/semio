//! 🧪️ Typed reader byte parity, exact Arc ownership, cancellation, and worker laws.

use super::super::borrowed_tests::{MapLifetime, MapMutation, MapRetirementFactory, fixture};
use super::*;
use std::sync::atomic::Ordering;

//#region 📦️ReaderFixtures
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RootRetirementFactory;
impl SnapshotRetirementFactory<Edit<MapMutation>> for RootRetirementFactory {
 fn retirement_birth_bytes(&self, root: &Arc<Edit<MapMutation>>) -> usize { semio_framework_value::retirement::SharedValueRetirementFactory::<Edit<MapMutation>>::default().retirement_birth_bytes(root) }
 fn retire(&self, root: Arc<Edit<MapMutation>>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<Edit<MapMutation>>)> { semio_framework_value::retirement::SharedValueRetirementFactory::<Edit<MapMutation>>::default().retire(root, grant) }
}
static RETIREMENT_POLICY: std::sync::LazyLock<RetainedCloneGrant> = std::sync::LazyLock::new(super::super::borrowed_tests::fixture_retirement_grant);
fn encoding_grant(items: usize, copy: usize) -> ArtifactStoreOneItemGrant {
 ArtifactStoreOneItemGrant { maximum_items: items, maximum_copy_bytes: copy, maximum_capacity_bytes: RETIREMENT_POLICY.maximum_capacity_bytes, maximum_release_bytes: RETIREMENT_POLICY.maximum_release_bytes, maximum_depth: RETIREMENT_POLICY.maximum_depth }
}

fn make_reader() -> (ArtifactCanonicalJsonReader<Edit<MapMutation>>, serde_json::Value, Arc<MapLifetime>) {
    let (edit, fixture, lifetime) = fixture();
    (ArtifactCanonicalJsonReader::new(Arc::new(edit), Arc::new(RootRetirementFactory)), fixture, lifetime)
}

fn finish(reader: &mut ArtifactCanonicalJsonReader<Edit<MapMutation>>, bytes: usize) -> Vec<u8> {
    let mut result = Vec::new();
    for _ in 0..100_000 {
        let prior = reader.completed_bytes();
        let mut output = [0; 512];
        let count = reader.encode_chunk(encoding_grant(1, bytes), &mut output).unwrap();
        assert!(count <= bytes.min(256));
        assert_eq!(reader.completed_bytes() - prior, count as u64);
        result.extend_from_slice(&output[..count]);
        if reader.is_complete() {
            return result;
        }
    }
    panic!("canonical reader did not finish");
}

fn close(reader: &mut ArtifactCanonicalJsonReader<Edit<MapMutation>>) {
 reader.begin_close();
 assert_eq!(reader.close_step(RetainedCloneGrant { maximum_items: 0, ..*RETIREMENT_POLICY }).unwrap().progress(), RetainedCloneProgress::default());
 for _ in 0..100_000 {
  let (step, heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reader.close_step(*RETIREMENT_POLICY).unwrap());let receipt=step.progress();assert!(receipt.fits(*RETIREMENT_POLICY));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
  if matches!(step,RetainedCloneStep::Complete(_)) { assert!(reader.terminal_is_empty());return; }
 }
 panic!("canonical reader did not retire");
}
fn retire(root: Arc<Edit<MapMutation>>, lifetime: &MapLifetime) {
 let (owner,progress)=RootRetirementFactory.retire(root,*RETIREMENT_POLICY).unwrap_or_else(|_|panic!("original reader root was refused"));assert!(progress.fits(*RETIREMENT_POLICY));let mut owner=Some(owner);
 for _ in 0..100_000 {
  let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||super::super::super::artifact_retirement_box_close_step(&mut owner,*RETIREMENT_POLICY).unwrap());let receipt=step.progress();assert!(receipt.fits(*RETIREMENT_POLICY));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
  if owner.is_none() { assert_eq!(lifetime.root_drops.load(Ordering::SeqCst),1);return; }
 }
 panic!("transferred reader root did not retire");
}
//#endregion 📦️ReaderFixtures

//#region 🧪️ReaderLifecycle
#[test]
fn canonical_reader_large_borrowed_map_matches_serde_and_transfers_exact_root() {
    let reader_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📖️canonical-reader.json")).unwrap();
    for bytes in [1, 7, 4096] {
        let (mut reader, fixture, lifetime) = make_reader();
        let address = Arc::as_ptr(reader.owned.root.as_ref().unwrap());
        let expected = serde_json::to_vec(&test_support::SerdeValue(&crate::os_store::component::CursorRevisionAccumulator::revision_value(reader.owned.root.as_ref().unwrap().as_ref()))).unwrap();
        assert_eq!(expected, fixture["expectedJson"].as_str().unwrap().as_bytes());
        assert_eq!(expected.len() as u64, reader_fixture["expectedByteLength"].as_u64().unwrap());
        assert!(reader.take_root().is_none());
        assert_eq!(reader.encode_chunk(encoding_grant(0, bytes), &mut [0; 256]).unwrap(), 0);
        assert_eq!(reader.encode_chunk(encoding_grant(1, 0), &mut [0; 256]).unwrap(), 0);
        assert_eq!(reader.completed_bytes(), 0);
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        let actual = finish(&mut reader, bytes);
        assert_eq!(actual, expected);
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(&actual);
        assert_eq!(hash.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>(), reader_fixture["expectedJsonSha256"].as_str().unwrap());
        let root = reader.take_root().unwrap();
        assert_eq!(Arc::as_ptr(&root), address);
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 0);
        close(&mut reader);
        retire(root, &lifetime);
    }
}

#[test]
fn canonical_reader_cancel_before_poll_mid_key_and_after_completion_retires_exact_root() {
    for stage in 0..3 {
        let (mut reader, fixture, lifetime) = make_reader();
        if stage == 1 {
            let target = fixture["expectedJson"].as_str().unwrap().find("key-").unwrap() as u64 + 128;
            while reader.completed_bytes() < target {
                reader.encode_chunk(encoding_grant(1, 1), &mut [0; 1]).unwrap();
            }
            assert!(lifetime.active_iterators.load(Ordering::SeqCst) > 0);
            assert!(reader.take_root().is_none());
            reader = std::thread::spawn(move || {
                reader.encode_chunk(encoding_grant(1, 1), &mut [0; 1]).unwrap();
                reader
            })
            .join()
            .unwrap();
        } else if stage == 2 {
            finish(&mut reader, 7);
        }
        reader.cancel();
        let prior = reader.completed_bytes();
        assert_eq!(reader.encode_chunk(encoding_grant(1, 4096), &mut [0; 256]).unwrap(), 0);
        assert_eq!(reader.completed_bytes(), prior);
        close(&mut reader);
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn canonical_reader_rebound_root_rejected_before_borrowed_reference_use() {
    let (mut reader, _, lifetime) = make_reader();
    for _ in 0..100 {
        reader.encode_chunk(encoding_grant(1, 7), &mut [0; 7]).unwrap();
    }
    let (replacement, _, replacement_lifetime) = fixture();
    let original = reader.owned.root.replace(Arc::new(replacement)).unwrap();
    assert_eq!(reader.encode_chunk(encoding_grant(1, 7), &mut [0; 7]).unwrap_err(), ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason: "canonical-edit.borrowed-root-rebound".into() });
    let replacement = reader.owned.root.replace(original).unwrap();
    retire(replacement, &replacement_lifetime);
    close(&mut reader);
    assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 1);

    let (mut reader, _, lifetime) = make_reader();
    for _ in 0..100 {
        reader.encode_chunk(encoding_grant(1, 7), &mut [0; 7]).unwrap();
    }
    reader.cancel();
    reader.begin_close();
    while !reader.owned.encoder.terminal_is_empty() {
        assert!(!reader.is_complete());
        assert!(reader.take_root().is_none());
        reader.close_step(*RETIREMENT_POLICY).unwrap();
    }
    let root = reader.take_root().unwrap();
    assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
    close(&mut reader);
    retire(root, &lifetime);
}

#[test]
fn canonical_reader_unclosed_drop_preserves_owned_root_and_does_not_double_panic() {
    let (mut reader, _, lifetime) = make_reader();
    for _ in 0..100 {
        reader.encode_chunk(encoding_grant(1, 7), &mut [0; 7]).unwrap();
    }
    assert!(lifetime.active_iterators.load(Ordering::SeqCst) > 0);
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(reader))).is_err());
    assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 0);
    assert!(lifetime.active_iterators.load(Ordering::SeqCst) > 0);
    let (reader, _, lifetime) = make_reader();
    assert!(
        std::thread::spawn(move || {
            let _reader = reader;
            panic!("primary reader failure");
        })
        .join()
        .is_err()
    );
    assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 0);
}
//#endregion 🧪️ReaderLifecycle

//#region ⚠️ErrorProgress
#[derive(semio_framework_value::RetireOwned)]
struct ErrorLeaf;
#[derive(semio_framework_value::RetireOwned)]
struct ErrorRoot {
    text: String,
    borrowed: bool,
    error: ErrorLeaf,
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ErrorRootRetirement(#[factory_child] Arc<std::sync::atomic::AtomicUsize>);

impl ArtifactCanonicalJson for ErrorLeaf {
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, String> {
        Err("canonical-reader.fixture-child".into())
    }
}

impl ArtifactCanonicalJson for ErrorRoot {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        match path {
            [] => Ok(ArtifactCanonicalJsonNode::Array(2)),
            [0] => Ok(ArtifactCanonicalJsonNode::String(&self.text)),
            _ => Err("canonical-reader.fixture-child".into()),
        }
    }
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, String> {
        Ok(self.borrowed.then(|| ArtifactCanonicalJsonValue::Array(ArtifactCanonicalJsonArray::new([ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String(&self.text)), ArtifactCanonicalJsonValue::Source(&self.error)].into_iter()))))
    }
}

impl SnapshotRetirementFactory<ErrorRoot> for ErrorRootRetirement {
 fn retirement_birth_bytes(&self,root:&Arc<ErrorRoot>)->usize {semio_framework_value::retirement::SharedValueRetirementFactory::<ErrorRoot>::default().retirement_birth_bytes(root)}
 fn retire(&self,root:Arc<ErrorRoot>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<ErrorRoot>)> {
  let unique=Arc::strong_count(&root)==1;let admitted=semio_framework_value::retirement::SharedValueRetirementFactory::<ErrorRoot>::default().retire(root,grant);if admitted.is_ok()&&unique{self.0.fetch_add(1,Ordering::SeqCst);}admitted
 }
}
impl ArtifactOwnedValueRetirementFactory<ErrorRoot> for ErrorRootRetirement {
 fn retirement_birth_bytes(&self,root:&ErrorRoot)->usize {semio_framework_value::retirement::OwnedValueRetirementFactory::<ErrorRoot>::default().retirement_birth_bytes(root)}
 fn retire_owned(&self,root:ErrorRoot,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,ErrorRoot)> {
  let admitted=semio_framework_value::retirement::OwnedValueRetirementFactory::<ErrorRoot>::default().retire_owned(root,grant);if admitted.is_ok(){self.0.fetch_add(1,Ordering::SeqCst);}admitted
 }
}
impl ArtifactCanonicalJsonTree for ErrorLeaf {
 fn canonical_tree_node(&self)->Result<ArtifactCanonicalJsonNode<'_>,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-reader.fixture-child"))}
}
impl ArtifactCanonicalJsonTree for ErrorRoot {
 fn canonical_tree_node(&self)->Result<ArtifactCanonicalJsonNode<'_>,ValueError>{Ok(ArtifactCanonicalJsonNode::Array(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{match ordinal{0=>Ok(&self.text),1=>Ok(&self.error),_=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-reader.fixture-child"))}}
}

#[test]
fn canonical_reader_error_after_partial_unicode_output_accounts_every_initialized_byte() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🚧️canonical-error-progress.json")).unwrap();
    let oracle = serde_json::to_string(&serde_json::json!([fixture["text"], null])).unwrap();
    let prefix = oracle.strip_suffix("null]").unwrap().as_bytes();
    assert_eq!(prefix, fixture["expectedPrefix"].as_str().unwrap().as_bytes());
    assert_eq!(prefix.len() as u64, fixture["expectedBytes"].as_u64().unwrap());
    for mode in fixture["modes"].as_array().unwrap() {
        for bytes in fixture["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).filter(|bytes| *bytes != 0) {
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let root = ErrorRoot { text: fixture["text"].as_str().unwrap().into(), borrowed: mode == "borrowed", error: ErrorLeaf };
            let snapshot_bytes = root.text.as_bytes().len();
            let mut reader = ArtifactCanonicalJsonReader::new(Arc::new(root), Arc::new(ErrorRootRetirement(count.clone())));
            let sentinel = fixture["sentinel"].as_u64().unwrap() as u8;
            let mut actual = Vec::new();
            let mut failure = None;
            let mut empty = [sentinel; 512];
            assert_eq!(reader.encode_chunk(encoding_grant(1, 0), &mut empty).unwrap(), 0);
            assert!(empty.iter().all(|byte| *byte == sentinel));
            for _ in 0..128 {
                let mut output = [sentinel; 512];
                let result = reader.encode_chunk(encoding_grant(1, bytes), &mut output);
                let initialized = output.iter().position(|byte| *byte == sentinel).unwrap();
                assert!(initialized <= bytes.min(256));
                assert!(output[initialized..].iter().all(|byte| *byte == sentinel));
                actual.extend_from_slice(&output[..initialized]);
                match result {
                    Ok(written) => assert_eq!(written, initialized),
                    Err(error) => {
                        assert_eq!(error.written_bytes, initialized);
                        failure = Some(error.reason);
                        break;
                    }
                }
            }
            let reported = reader.completed_bytes();
            let complete = reader.is_complete();
            assert!(reader.take_root().is_none());
            assert_eq!(reader.encode_chunk(encoding_grant(1, 4096), &mut empty).unwrap(), 0);
            assert!(empty.iter().all(|byte| *byte == sentinel));
            reader.begin_close();
            let retired=snapshot_bytes;let mut physical_released=0;
            for _ in 0..256 {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| reader.close_step(*RETIREMENT_POLICY).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                match step {
                    RetainedCloneStep::Progress(progress) => { assert!(progress.fits(*RETIREMENT_POLICY));physical_released+=progress.released_bytes; }
                    RetainedCloneStep::Complete(progress) => { assert!(progress.fits(*RETIREMENT_POLICY));physical_released+=progress.released_bytes;break; },
                }
            }
            assert!(reader.terminal_is_empty());
            assert_eq!(failure.as_deref(), fixture["expectedError"].as_str());
            assert_eq!(actual, prefix);
            assert_eq!(reported, actual.len() as u64, "{mode}: grant{bytes} must include initialized bytes from its failed write");
            assert_eq!(complete, fixture["expectedComplete"].as_bool().unwrap());
            assert_eq!(count.load(Ordering::SeqCst) as u64, fixture["expectedRootRetirements"].as_u64().unwrap());
            assert_eq!(retired as u64, fixture["expectedSnapshotBytes"].as_u64().unwrap());
            eprintln!("[DEBUG] canonical reader failed mode={mode} grant={bytes} with all{reported} initialized prefix bytes owned, original UTF8 snapshot bytes={retired}, physical closure bytes={physical_released}");
        }
    }
}

#[test]
fn canonical_reader_indexed_cursor_error_preserves_actual_chunk_prefix() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🚧️canonical-error-progress.json")).unwrap();
    for maximum in [1, 7, 256, 4096] {
        let root = ErrorRoot { text: fixture["text"].as_str().unwrap().into(), borrowed: false, error: ErrorLeaf };
        let mut cursor = ArtifactCanonicalJsonCursor::default();
        let mut actual = Vec::new();
        let mut failure = None;
        for _ in 0..128 {
            let mut output = [165; 512];
            let result = cursor.encode_chunk(&root, &mut output[..maximum.min(512)]);
            let written = match result {
                Ok(written) => written,
                Err(error) => {
                    failure = Some(error.reason);
                    error.written_bytes
                }
            };
            assert!(written <= maximum.min(256));
            assert!(output[written..].iter().all(|byte| *byte == 165));
            actual.extend_from_slice(&output[..written]);
            if failure.is_some() {
                break;
            }
        }
        assert_eq!(failure.as_deref(), fixture["expectedError"].as_str());
        assert_eq!(actual, fixture["expectedPrefix"].as_str().unwrap().as_bytes());
        assert!(!cursor.is_complete());
    }
}

#[test]
fn canonical_reader_sealer_failed_prefix_is_accounted_without_minting_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🚧️canonical-error-progress.json")).unwrap();
    for borrowed in [false, true] {
        for maximum_bytes in [1, 7, 4096] {
            let mut value: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔏️canonical-edit-sealer.json")).unwrap();
            value["edit"]["forwards"] = serde_json::json!([]);
            value["edit"]["inverse"] = serde_json::json!([]);
            let mut oracle = Edit::<DslValue>::from_value(value["edit"].take().into()).unwrap();
            oracle.forwards.push(serde_json::json!([fixture["text"], null]).into());
            let expected = serde_json::to_string(&test_support::SerdeValue(&crate::os_store::component::CursorRevisionAccumulator::revision_value(&oracle))).unwrap();
            let prefix = &expected.as_bytes()[..expected.find("null]").unwrap()];
            let edit = Edit {
                line: oracle.line,
                id: oracle.id,
                actor: oracle.actor,
                forwards: vec![ErrorRoot { text: fixture["text"].as_str().unwrap().into(), borrowed, error: ErrorLeaf }],
                inverse: Vec::new().into(),
                mutation_meta: oracle.mutation_meta,
                verb: oracle.verb,
                sequence_number: oracle.sequence_number,
                started_at: oracle.started_at,
                finished_at: oracle.finished_at,
            };
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let authority = super::super::tests::authority();
            let mut owner = super::super::tests::admit_sealer(Arc::clone(&authority), edit, Arc::new(17u64), Arc::new(ErrorRootRetirement(count.clone())), Arc::new(super::super::tests::FixtureSnapshotRetirement));
            let grant = encoding_grant(1, maximum_bytes);
            let mut actual = Vec::new();
            let mut failure = None;
            for _ in 0..1024 {
                let result = owner.advance(grant);
                actual.extend_from_slice(owner.canonical_chunk());
                if let Err(error) = result {
                    failure = Some(error);
                    break;
                }
            }
            assert_eq!(failure.as_deref(), fixture["expectedError"].as_str());
            assert_eq!(actual, prefix);
            assert_eq!(owner.checkpoint().completed_bytes, actual.len() as u64);
            assert_eq!(owner.checkpoint().canonical_bytes, actual.len() as u64);
            assert_eq!(owner.checkpoint().prefix_digest, {
                let mut hash = semio_framework_hash::Sha256::new();
                hash.update(&actual);
                hash.finalize()
            });
            assert!(owner.prepared().is_none() && owner.take_prepared().is_none());
            assert_eq!(owner.advance(grant).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked);
            owner.begin_close();
            for _ in 0..100_000 {
                if matches!(owner.close_step(*RETIREMENT_POLICY).unwrap(), RetainedCloneStep::Complete(_)) {
                    break;
                }
            }
            assert!(owner.terminal_is_empty());
            assert_eq!(count.load(Ordering::SeqCst), 1);
            eprintln!("failed canonical sealer retained {} prefix bytes, no prepared authority, exact close borrowed={borrowed} grant={maximum_bytes}", actual.len());
        }
    }
}
//#endregion ⚠️ErrorProgress
