use super::*;

#[test]
fn canonical_sealer_constructor_refusal_keeps_every_original_owner_without_heap_work() {
    let demand = ArtifactStoreOneItemSealer::<u64, FixtureMutation>::constructor_demand();
    assert_eq!(demand.capacity_bytes, ArtifactStoreCanonicalSource::<FixtureMutation>::source_constructor_demand().capacity_bytes);
    assert!(demand.capacity_bytes <= 4096);
    let mut original = (authority(), fixture().0, Arc::new(17u64), Arc::new(FixtureMutationRetirement) as Arc<dyn ArtifactOwnedValueRetirementFactory<FixtureMutation>>, Arc::new(FixtureSnapshotRetirement) as Arc<dyn SnapshotRetirementFactory<u64>>);
    let authority_address = Arc::as_ptr(&original.0);
    let post_address = Arc::as_ptr(&original.2);
    let id_address = original.1.id.as_ptr();
    for grant in [
        RetainedCloneGrant { maximum_items: 0, maximum_capacity_bytes: demand.capacity_bytes, maximum_depth: demand.depth, ..Default::default() },
        RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: demand.capacity_bytes - 1, maximum_depth: demand.depth, ..Default::default() },
        RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: demand.capacity_bytes, maximum_depth: 0, ..Default::default() },
    ] {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreOneItemSealer::admit(original.0, original.1, original.2, original.3, original.4, grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        original = match result {
            Err((_, authority, edit, post, mutation, snapshot)) => (authority, edit, post, mutation, snapshot),
            Ok(_) => panic!("canonical sealer admitted an incomplete constructor grant"),
        };
        assert_eq!(Arc::as_ptr(&original.0), authority_address);
        assert_eq!(Arc::as_ptr(&original.2), post_address);
        assert_eq!(original.1.id.as_ptr(), id_address);
    }
    println!("[DEBUG] canonical sealer zero/one-below/depth refusal preserves exact original edit/post/authority/factory owners with zero heap work");
}

/// 🔬️ `ScalarBytes::from_node`'s serde-free arms (`Null`/`Bool`/`I64`/`U64`/`I128`/`U128`/
/// `F64`), byte-for-byte against `serde_json` — the direct proof this ticket's own
/// `float-format-parity.md` calls for on the second real call site (`F32` intentionally
/// excluded, still routed through `serde_json`, see `from_node`'s own docstring). The `f64`
/// half reuses the same LCG this crate's other property tests use, not a new dependency.
#[test]
fn scalar_bytes_from_node_matches_serde_json_byte_for_byte() {
    fn bytes_of(node: ArtifactCanonicalJsonNode<'_>) -> Vec<u8> {
        let scalar = ScalarBytes::from_node(node).unwrap();
        scalar.bytes[..scalar.length].to_vec()
    }
    assert_eq!(bytes_of(ArtifactCanonicalJsonNode::Null), serde_json::to_vec(&()).unwrap());
    for value in [true, false] {
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::Bool(value)), serde_json::to_vec(&value).unwrap());
    }
    for value in [0i64, -1, 1, i64::MIN, i64::MAX, -17] {
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::I64(value)), serde_json::to_vec(&value).unwrap());
    }
    for value in [0u64, 1, u64::MAX] {
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::U64(value)), serde_json::to_vec(&value).unwrap());
    }
    for value in [0i128, -1, i128::MIN, i128::MAX] {
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::I128(value)), serde_json::to_vec(&value).unwrap());
    }
    for value in [0u128, u128::MAX] {
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::U128(value)), serde_json::to_vec(&value).unwrap());
    }
    let mut state: u64 = 0xC0DE_CAFE_1234_5678;
    let mut next_u64 = || {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };
    let mut checked = 0usize;
    for value in [0.0, -0.0, 1.0, -1.0, 0.1, 1e21, 1e-7, f64::MIN_POSITIVE, f64::MAX] {
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::F64(value)), serde_json::to_vec(&value).unwrap(), "mismatch for {value:e}");
        checked += 1;
    }
    for _ in 0..50_000u32 {
        let value = f64::from_bits(next_u64());
        if !value.is_finite() {
            continue;
        }
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::F64(value)), serde_json::to_vec(&value).unwrap(), "mismatch for {value:e}");
        checked += 1;
    }
}

#[derive(Clone, Debug, Serialize, ToValue, Deserialize, FromValue, semio_framework_value_derive::RetireOwned)]
enum FixtureMutation {
    Replace(FixtureReplace),
}

#[derive(Clone, Debug, Serialize, ToValue, Deserialize, FromValue, semio_framework_value_derive::RetireOwned)]
struct FixtureReplace { text:String,nested:Vec<String>,enabled:bool,amount:i64 }
impl ArtifactCanonicalJsonTree for FixtureMutation {
    fn canonical_tree_node(&self)->Result<ArtifactCanonicalJsonNode<'_>,ValueError>{Ok(ArtifactCanonicalJsonNode::Object(1))}
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{let Self::Replace(body)=self;if ordinal==0{Ok(body)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"fixture original variant ordinal missing"))}}
    fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{if ordinal==0{Ok("Replace".into())}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"fixture original variant key missing"))}}
}
impl ArtifactCanonicalJsonTree for FixtureReplace {
    fn canonical_tree_node(&self)->Result<ArtifactCanonicalJsonNode<'_>,ValueError>{Ok(ArtifactCanonicalJsonNode::Object(4))}
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{Ok(match ordinal{0=>&self.text,1=>&self.nested,2=>&self.enabled,3=>&self.amount,_=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"fixture original field ordinal missing"))})}
    fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{["text","nested","enabled","amount"].get(ordinal).map(|key|ArtifactCanonicalJsonText::from(*key)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"fixture original field key missing"))}
}

impl ArtifactCanonicalJson for FixtureMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        use ArtifactCanonicalJsonNode as N;
        let Self::Replace(FixtureReplace { text, nested, enabled, amount }) = self;
        Ok(match path {
            [] => N::Object(1),
            [0] => N::Object(4),
            [0, 0] => N::String(text),
            [0, 1] => N::Array(nested.len()),
            [0, 1, index] => N::String(nested.get(*index).ok_or_else(invalid_path)?),
            [0, 2] => N::Bool(*enabled),
            [0, 3] => N::I64(*amount),
            _ => return Err(invalid_path()),
        })
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        match path {
            [] if index == 0 => Ok("Replace".into()),
            [0] => ["text", "nested", "enabled", "amount"].get(index).copied().map(Into::into).ok_or_else(invalid_path),
            _ => Err(invalid_path()),
        }
    }
}

fn fixture() -> (Edit<FixtureMutation>, serde_json::Value) {
    let value: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔏️canonical-edit-sealer.json")).unwrap();
    (Edit::from_value(value["edit"].clone().into()).unwrap(), value)
}

#[test]
fn canonical_edit_large_unicode_bytes_match_serde_and_language_neutral_oracle() {
    let (edit, fixture) = fixture();
    let oracle = serde_json::to_vec(&test_support::SerdeValue(&CursorRevisionAccumulator::revision_value(&edit))).unwrap();
    assert_eq!(oracle, fixture["expectedJson"].as_str().unwrap().as_bytes());
    for maximum in [1, 2, 7, 256, 4096] {
        let mut encoder = ArtifactCanonicalJsonCursor::default();
        let mut actual = Vec::new();
        let mut output = vec![0; maximum];
        assert_eq!(encoder.encode_chunk_admitted(&edit, &mut [],RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}).unwrap().written_bytes, 0);
        while !encoder.is_complete() {
            let count = encoder.encode_chunk_admitted(&edit, &mut output,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}).unwrap().written_bytes;
            assert!(count <= maximum.min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES));
            actual.extend_from_slice(&output[..count]);
        }
        assert_eq!(actual, oracle, "grant {maximum}");
    }
    let digest = CursorRevisionAccumulator::edit_digest(&edit);
    let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    assert_eq!(hex, fixture["expectedDigest"].as_str().unwrap());
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct FixtureMutationRetirement;
impl ArtifactOwnedValueRetirementFactory<FixtureMutation> for FixtureMutationRetirement {
    fn retirement_birth_bytes(&self, _: &FixtureMutation) -> usize { semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<FixtureMutation>() }
    fn retire_owned(&self, value: FixtureMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, FixtureMutation)> {
        semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(value, grant).map(|(owner, progress)| (owner as Box<dyn ErasedSnapshotRetirement>, progress))
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct FixtureSnapshotRetirement;
impl SnapshotRetirementFactory<u64> for FixtureSnapshotRetirement {
    fn retirement_birth_bytes(&self, _: &Arc<u64>) -> usize { semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<u64>() }
    fn retire(&self, snapshot: Arc<u64>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<u64>)> {
        semio_framework_value::retirement::shared::admit_shared_retirement(snapshot, grant, false)
    }
}

pub(super) fn authority() -> Arc<ArtifactStoreOneItemLiveAuthority> {
    Arc::new(ArtifactStoreOneItemLiveAuthority {
        operation: semio_framework_job::OperationId(11),
        generation: semio_framework_job::Generation(7),
        base_revision: [9; 32],
        base_applied_edit_count: 3,
        next_sequence_number: 4,
        next_clock: HybridLogicalTimestamp { actor: 1, physical_ms: 42, logical: 2 },
        actor: "actor-1".into(),
        group_id: Some("group-1".into()),
        stamped_edit_id: None,
        line: None,
    })
}

pub(super) fn admit_sealer<P, M:semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree>(authority: Arc<ArtifactStoreOneItemLiveAuthority>, edit: Edit<M>, post: Arc<P>, mutation: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshot: Arc<dyn SnapshotRetirementFactory<P>>) -> ArtifactStoreOneItemSealer<P, M> {
    let demand = ArtifactStoreOneItemSealer::<P, M>::constructor_demand();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes:4096, maximum_capacity_bytes: demand.capacity_bytes, maximum_depth: demand.depth, ..Default::default() };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| authority.begin_one_item_seal(edit, post, mutation, snapshot, grant));
    let (owner, receipt) = result.unwrap_or_else(|_| panic!("canonical sealer exact constructor admission"));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, 0));
    assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
    assert!(receipt.fits(grant));
    owner
}

fn sealer(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreOneItemSealer<u64, FixtureMutation> {
    admit_sealer(Arc::clone(authority), fixture().0, Arc::new(17), Arc::new(FixtureMutationRetirement), Arc::new(FixtureSnapshotRetirement))
}

fn close(sealer: &mut ArtifactStoreOneItemSealer<u64, FixtureMutation>, bytes: usize) {
    sealer.begin_close();
    assert_eq!(sealer.close_step(RetainedCloneGrant::default()).unwrap().progress(), RetainedCloneProgress::default());
    let mut last = None;
    for _ in 0..100_000 {
        let demand = sealer.retirement_demands(bytes).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let step = sealer.close_step(grant).unwrap();
        last = Some((demand, step));
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(sealer.terminal_is_empty());
            return;
        }
    }
    panic!("bounded retirement did not terminate: {last:?}");
}

/// 🎟️ The turn grant that funds exactly the next quoted preparation demand, with `bytes` of canonical streaming copy on top.
fn funded(sealer: &ArtifactStoreOneItemSealer<u64, FixtureMutation>, bytes: usize) -> ArtifactStoreOneItemGrant {
    let demand = sealer.preparation_demands().expect("the sealer quotes its next preparation demand");
    ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: bytes.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(64) }
}

fn finish(sealer: &mut ArtifactStoreOneItemSealer<u64, FixtureMutation>, bytes: usize) -> [u8; 32] {
    let mut previous = sealer.completed_bytes;
    for _ in 0..100_000 {
        let grant = funded(sealer, bytes);
        let step = sealer.advance(grant).unwrap();
        assert!(sealer.completed_bytes - previous <= (grant.maximum_copy_bytes + grant.maximum_capacity_bytes + grant.maximum_release_bytes) as u64);
        previous = sealer.completed_bytes;
        if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(..)) {
            return sealer.prepared().unwrap().edit_digest();
        }
    }
    panic!("positive byte grant failed to seal");
}

#[test]
fn canonical_sealer_tiny_grants_replay_and_cross_worker_transfer_preserve_exact_digest() {
    let oracle = CursorRevisionAccumulator::edit_digest(&fixture().0);
    for bytes in [1, 2, 7, 256, 4096] {
        let authority = authority();
        let mut owner = sealer(&authority);
        assert!(matches!(owner.advance(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
        owner.advance(funded(&owner, bytes)).unwrap();
        for _ in 0..19 {
            owner.advance(funded(&owner, bytes)).unwrap();
        }
        let checkpoint: ArtifactStoreOneItemSealCheckpoint = serde_json::from_slice(&serde_json::to_vec(&owner.checkpoint()).unwrap()).unwrap();
        let mut replay = sealer(&authority);
        replay.restore_checkpoint(checkpoint).unwrap();
        let refused = replay.advance(funded(&replay, 7)).unwrap_err();
        assert_eq!(refused.message, "canonical-edit.native-checkpoint-unsupported", "a native source cannot resume a serialized prefix");
        assert!(replay.prepared().is_none());
        close(&mut replay, 1);
        let mut moved = std::thread::spawn(move || {
            assert_eq!(finish(&mut owner, bytes), oracle);
            owner
        })
        .join()
        .unwrap();
        assert!(moved.completed_bytes >= 2 * moved.canonical_bytes, "the identity and hash passes each stream the whole canonical edit");
        authority.validate_prepared(moved.prepared().unwrap()).unwrap();
        close(&mut moved, 1);
    }
}

#[test]
fn canonical_sealer_rejects_stale_checkpoint_forged_prefix_and_rebound_owners() {
    let authority = authority();
    let mut owner = sealer(&authority);
    for _ in 0..4 {
        owner.advance(funded(&owner, 7)).unwrap();
    }
    let checkpoint = owner.checkpoint();
    for hostile in 0..5 {
        let mut altered = checkpoint;
        match hostile {
            0 => altered.operation += 1,
            1 => altered.generation += 1,
            2 => altered.base_revision[0] ^= 1,
            3 => altered.authority_digest[0] ^= 1,
            _ => altered.version += 1,
        }
        let mut replay = sealer(&authority);
        assert!(replay.restore_checkpoint(altered).is_err());
        close(&mut replay, 7);
    }
    let mut altered = checkpoint;
    altered.prefix_digest[0] ^= 1;
    let mut replay = sealer(&authority);
    replay.restore_checkpoint(altered).unwrap();
    let mut rejected = false;
    for _ in 0..100 {
        if replay.advance(funded(&replay, 1)).is_err() {
            rejected = true;
            break;
        }
    }
    assert!(rejected && replay.prepared().is_none());
    close(&mut replay, 1);
    finish(&mut owner, 4096);
    let prepared = owner.prepared.as_mut().unwrap();
    prepared.edit_digest[0] ^= 1;
    assert!(authority.validate_prepared(prepared).is_err());
    prepared.edit_digest[0] ^= 1;
    let replacement = Box::new(prepared.edit.as_ref().clone());
    let original = std::mem::replace(&mut prepared.edit, replacement);
    assert!(authority.validate_prepared(prepared).is_err());
    prepared.edit = original;
    let original = std::mem::replace(&mut prepared.post_snapshot, Arc::new(17));
    assert!(authority.validate_prepared(prepared).is_err());
    prepared.post_snapshot = original;
    assert!(tests::authority().validate_prepared(prepared).is_err());
    authority.validate_prepared(prepared).unwrap();
    close(&mut owner, 1);
}

#[test]
fn canonical_sealer_cancellation_at_every_phase_retires_exact_owners_and_allows_retry() {
    let authority = authority();
    for phase in 0..=6 {
        let mut owner = sealer(&authority);
        while owner.phase < phase {
            owner.advance(funded(&owner, 4096)).unwrap();
        }
        owner.cancel();
        let checkpoint = owner.checkpoint();
        assert!(matches!(owner.advance(funded(&owner, 4096)).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
        assert_eq!(owner.checkpoint(), checkpoint);
        close(&mut owner, 1);
    }
    let mut retry = sealer(&authority);
    assert_eq!(finish(&mut retry, 1), CursorRevisionAccumulator::edit_digest(&fixture().0));
    close(&mut retry, 1);
}

#[test]
fn canonical_sealer_checkpoint_maximum_accepts_exact_framing_and_identity_overhead_only() {
    let authority = authority();
    let mut owner = sealer(&authority);
    let mut checkpoint = owner.checkpoint();
    checkpoint.phase = 6;
    checkpoint.canonical_bytes = CANONICAL_EDIT_MAXIMUM_BYTES;
    checkpoint.completed_bytes = 2 * CANONICAL_EDIT_MAXIMUM_BYTES + CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES;
    const { assert!(CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES > 1_024) };
    owner.restore_checkpoint(checkpoint).unwrap();
    checkpoint.completed_bytes += 1;
    assert!(owner.restore_checkpoint(checkpoint).is_err());
    close(&mut owner, 1);
}

#[test]
fn canonical_sealer_preserves_large_domains_and_all_wire_metadata_origins() {
    let (base, fixture) = fixture();
    for length in fixture["largeTextBytes"].as_array().unwrap() {
        for origin in fixture["origins"].as_array().unwrap() {
            let mut edit = base.clone();
            let FixtureMutation::Replace(FixtureReplace { text, .. }) = &mut edit.forwards[0];
            *text = "x".repeat(length.as_u64().unwrap() as usize);
            edit.finished_at = Some("finished".into());
            edit.mutation_meta[0].origin = crate::os_spr::MutationOrigin::from_value(origin.clone().into()).unwrap();
            edit.mutation_meta[0].payload_hash = Some(crate::os_spr::PayloadHash([23; 32]));
            edit.mutation_meta[0].semantic_kind = Some(SchemaId("fixture#replace".into()));
            let expected = serde_json::to_vec(&test_support::SerdeValue(&CursorRevisionAccumulator::revision_value(&edit))).unwrap();
            let mut actual = Vec::new();
            let mut encoder = ArtifactCanonicalJsonCursor::default();
            let mut chunk = [0; 256];
            while !encoder.is_complete() {
                let count = encoder.encode_chunk_admitted(&edit, &mut chunk,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}).unwrap().written_bytes;
                actual.extend_from_slice(&chunk[..count]);
            }
            assert_eq!(actual, expected);
            let digest = CursorRevisionAccumulator::edit_digest(&edit);
            let authority = authority();
            let mut owner = admit_sealer(Arc::clone(&authority), edit, Arc::new(17), Arc::new(FixtureMutationRetirement), Arc::new(FixtureSnapshotRetirement));
            assert_eq!(finish(&mut owner, 4096), digest);
            close(&mut owner, 4096);
        }
    }
}

#[test]
fn canonical_authority_final_unicode_strings_retire_under_single_byte_grants() {
    use semio_framework_value::{factory_ticket_demands, close_factory_ticket, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔏️canonical-edit-sealer.json")).unwrap();
    let law = &fixture["authorityRetirement"];
    let mut authority = authority();
    let fields = ["actor", "group", "edit", "line"].map(|name| law[name].as_str().unwrap().to_owned());
    let payload_bytes = fields.iter().map(String::len).sum::<usize>();
    let [actor, group, edit, line] = fields;
    let original = Arc::get_mut(&mut authority).unwrap();
    original.actor = actor.into(); original.group_id = Some(group); original.stamped_edit_id = Some(edit); original.line = Some(line);
    let pointer = Arc::as_ptr(&authority);
    let birth = authority.retirement_birth_demand();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: law["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: birth.capacity_bytes, maximum_release_bytes: 0, maximum_depth: birth.depth };
    for refusal in law["refusals"].as_array().unwrap() {
        let denied = match refusal.as_str().unwrap() { "items" => RetainedCloneGrant { maximum_items: 0, ..grant }, "capacity" => RetainedCloneGrant { maximum_capacity_bytes: birth.capacity_bytes - 1, ..grant }, "depth" => RetainedCloneGrant { maximum_depth: 0, ..grant }, _ => unreachable!() };
        let (result, allocated, released) = crate::test_allocation::observe_backing(|| authority.retire(denied));
        let (_, returned) = result.err().unwrap(); authority = returned;
        assert_eq!((allocated, released), (0, 0)); assert_eq!(Arc::as_ptr(&authority), pointer);
    }
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| authority.retire(grant));
    let (owner, receipt) = result.unwrap_or_else(|_| panic!("original authority frame admission"));
    assert_eq!((allocated, released), (receipt.retained_capacity_bytes, 0)); assert!(receipt.fits(grant));
    let mut owner = Some(owner); let mut physical_release = 0;
    for _ in 0..1000 {
        if owner.is_none() { assert!(physical_release >= payload_bytes + birth.capacity_bytes); eprintln!("[DEBUG] original authority birth/refusal preserves Arc custody; one-byte copy and whole physical releases match System"); return; }
        let demand = factory_ticket_demands(owner.as_ref().unwrap(), grant.maximum_copy_bytes).unwrap();
        let actual = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..grant };
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| close_factory_ticket(&mut owner, RetainedCloneGrant { maximum_items: 0, ..actual }).unwrap());
        assert_eq!((allocated, released), (0, 0)); assert_eq!(step.progress(), RetainedCloneProgress::default());
        if demand.release_bytes > 0 {
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| close_factory_ticket(&mut owner, RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..actual }).unwrap());
            assert_eq!((allocated, released), (0, 0)); assert_eq!(step.progress(), RetainedCloneProgress::default());
            assert_eq!(factory_ticket_demands(owner.as_ref().unwrap(), grant.maximum_copy_bytes).unwrap(), demand);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| close_factory_ticket(&mut owner, actual).unwrap());
        assert!(step.progress().fits(actual)); assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes)); physical_release += released;
    }
    panic!("final authority strings did not retire");
}

/// 🔗️ LAW (ticket 26/09/23 C12): the revision digest rule — a single-operation edit hashes its canonical JSON, an edit grown
/// past one operation hashes its header fields and one running chain per operation list — matches the language-neutral
/// vectors (derived by a third implementation, replayed by the TS oracle too), and extending the chains of a grown edit by the
/// operations an amend appended equals its from-scratch digest.
#[test]
fn edit_digest_chains_match_the_neutral_vectors_and_extend_incrementally() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️edit-digest-chains.json")).unwrap();
    let edit_of = |case: &serde_json::Value| match case.get("edit") {
        Some(edit) => Edit::<DslValue>::from_value(edit.clone().into()).unwrap(),
        None => {
            let mut header = case["header"].clone();
            let count = case["generatedOperations"].as_u64().unwrap();
            header["forwards"] = (0..count).map(|index| serde_json::json!({ "SetN": { "n": index + 1 } })).collect();
            header["inverse"] = (0..count).map(|index| serde_json::json!({ "SetN": { "n": index } })).collect();
            Edit::<DslValue>::from_value(header.into()).unwrap()
        }
    };
    for case in fixture["cases"].as_array().unwrap() {
        let digest = super::super::CursorRevisionAccumulator::edit_digest(&edit_of(case));
        assert_eq!(semio_framework_hash::hex_lower(&digest), case["expectedDigest"].as_str().unwrap(), "edit digest vector {}", case["name"]);
    }
    let mut grown = edit_of(&fixture["cases"][1]);
    let (_, chains) = super::super::CursorRevisionAccumulator::edit_digest_extending(&grown, None);
    grown.forwards.push(serde_json::json!({ "SetN": { "n": 3 } }).into());
    grown.inverse.push(serde_json::json!({ "SetN": { "n": 2 } }).into());
    let (extended, _) = super::super::CursorRevisionAccumulator::edit_digest_extending(&grown, chains);
    assert_eq!(extended, super::super::CursorRevisionAccumulator::edit_digest(&grown), "an amend's incremental digest equals the from-scratch digest");
}

#[test]
fn canonical_json_original_supplied_depth_and_prefix_conserve_every_real_turn() {
    struct DepthSource(serde_json::Value);
    impl ArtifactCanonicalJson for DepthSource {
        fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
            let mut value = &self.0;
            for ordinal in path { value = value.as_array().and_then(|values| values.get(*ordinal)).ok_or_else(invalid_path)?; }
            Ok(match value {
                serde_json::Value::Bool(value) => ArtifactCanonicalJsonNode::Bool(*value),
                serde_json::Value::Array(values) => ArtifactCanonicalJsonNode::Array(values.len()),
                _ => return Err(invalid_path()),
            })
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚧️canonical-error-progress.json")).unwrap();
    let limits = &fixture["retainedPolicy"];
    let original = RetainedCloneGrant {
        maximum_items: limits["maximumItems"].as_u64().unwrap() as usize,
        maximum_copy_bytes: limits["maximumCopyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: limits["maximumCapacityBytes"].as_u64().unwrap() as usize,
        maximum_release_bytes: limits["maximumReleaseBytes"].as_u64().unwrap() as usize,
        maximum_depth: limits["maximumDepth"].as_u64().unwrap() as usize,
    };
    for row in fixture["depthRows"].as_array().unwrap() {
        let source = DepthSource(row["source"].clone());
        let source_address = &source.0 as *const _;
        let expected = serde_json::to_vec(&source.0).unwrap();
        let grant = RetainedCloneGrant { maximum_depth: row["maximumDepth"].as_u64().unwrap() as usize, ..original };
        let mut encoder = ArtifactCanonicalJsonCursor::default();
        let mut actual = Vec::with_capacity(expected.len());
        let mut refused = false;
        for _ in 0..64 {
            let mut output = [0xa5; 8];
            for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }] {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_chunk_admitted(&source, &mut output, denied).unwrap());
                assert_eq!(step.written_bytes, 0); assert_eq!(step.ownership.progress(), RetainedCloneProgress::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(output, [0xa5; 8]);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_chunk_admitted(&source, &mut output, grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            match step {
                Ok(step) => {
                    let receipt = step.ownership.progress();
                    assert!(receipt.copied_items <= grant.maximum_items && receipt.copied_bytes <= grant.maximum_copy_bytes);
                    assert_eq!((receipt.retained_capacity_bytes, receipt.released_bytes), (0, 0));
                    assert!(output[step.written_bytes..].iter().all(|byte| *byte == 0xa5));
                    actual.extend_from_slice(&output[..step.written_bytes]);
                    if encoder.is_complete() { break; }
                }
                Err(error) => { assert_eq!(error.written_bytes, 0); assert_eq!(error.reason.retained_progress(), RetainedCloneProgress::default()); assert_eq!(output, [0xa5; 8]); refused = true; break; }
            }
        }
        assert_eq!(&source.0 as *const _, source_address);
        if row["refused"].as_bool().unwrap_or(false) { assert!(refused); assert_eq!(actual, row["expectedPrefix"].as_str().unwrap().as_bytes()); }
        else { assert!(!refused && encoder.is_complete()); assert_eq!(actual, expected); }
        println!("[DEBUG] canonical JSON original supplied depth={} prefix={} refused={} actual System0 full-grant per-event receipts", grant.maximum_depth, actual.len(), refused);
    }
}
