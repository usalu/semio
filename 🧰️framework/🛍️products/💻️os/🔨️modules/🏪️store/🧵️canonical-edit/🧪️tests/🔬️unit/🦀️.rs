use super::*;

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
    eprintln!("[DEBUG] [canonical-edit] {checked} ScalarBytes f64 values matched serde_json byte-for-byte");
}

#[derive(Clone, Debug, Serialize, ToValue, Deserialize, FromValue)]
enum FixtureMutation {
    Replace { text: String, nested: Vec<String>, enabled: bool, amount: i64 },
}

impl ArtifactCanonicalJson for FixtureMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        use ArtifactCanonicalJsonNode as N;
        let Self::Replace { text, nested, enabled, amount } = self;
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
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<&str, String> {
        match path {
            [] if index == 0 => Ok("Replace"),
            [0] => ["text", "nested", "enabled", "amount"].get(index).copied().ok_or_else(invalid_path),
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
    let oracle = serde_json::to_vec(&test_support::SerdeValue(&edit.to_value())).unwrap();
    assert_eq!(oracle, fixture["expectedJson"].as_str().unwrap().as_bytes());
    for maximum in [1, 2, 7, 256, 4096] {
        let mut encoder = ArtifactCanonicalJsonCursor::default();
        let mut actual = Vec::new();
        let mut output = vec![0; maximum];
        assert_eq!(encoder.encode_chunk(&edit, &mut []).unwrap(), 0);
        while !encoder.is_complete() {
            let count = encoder.encode_chunk(&edit, &mut output).unwrap();
            assert!(count <= maximum.min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES));
            actual.extend_from_slice(&output[..count]);
        }
        assert_eq!(actual, oracle, "grant {maximum}");
    }
    let digest = CursorRevisionAccumulator::edit_digest(&edit);
    let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    assert_eq!(hex, fixture["expectedDigest"].as_str().unwrap());
}

struct FixtureRetirement {
    text: Option<String>,
    nested: Vec<String>,
    active: Option<ArtifactStoreStringRetirement>,
}

impl ErasedSnapshotRetirement for FixtureRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if items == 0 || bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(active) = self.active.as_mut() {
            let step = active.close_step(items, bytes)?;
            if matches!(step, SnapshotRetirementStep::Complete) {
                assert!(active.terminal_is_empty());
                self.active = None;
            }
            return Ok(if matches!(step, SnapshotRetirementStep::Complete) { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(text) = self.text.take().or_else(|| self.nested.pop()) {
            self.active = Some(ArtifactStoreStringRetirement::new(text));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.text.is_none() && self.nested.is_empty() && self.active.is_none()
    }
}

struct FixtureMutationRetirement;
impl ArtifactOwnedValueRetirementFactory<FixtureMutation> for FixtureMutationRetirement {
    fn retire_owned(&self, value: FixtureMutation) -> Box<dyn ErasedSnapshotRetirement> {
        let FixtureMutation::Replace { text, nested, .. } = value;
        Box::new(FixtureRetirement { text: Some(text), nested, active: None })
    }
}

pub(super) struct FixtureSnapshotRetirement;
struct FixtureRootRetirement(Option<Arc<u64>>);
impl SnapshotRetirementFactory<u64> for FixtureSnapshotRetirement {
    fn retire(&self, snapshot: Arc<u64>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FixtureRootRetirement(Some(snapshot)))
    }
}
impl ErasedSnapshotRetirement for FixtureRootRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if items == 0 || bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if self.0.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
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
    })
}

fn sealer(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreOneItemSealer<u64, FixtureMutation> {
    authority.begin_one_item_seal(fixture().0, Arc::new(17), Arc::new(FixtureMutationRetirement), Arc::new(FixtureSnapshotRetirement))
}

fn close(sealer: &mut ArtifactStoreOneItemSealer<u64, FixtureMutation>, bytes: usize) {
    sealer.begin_close();
    assert!(matches!(sealer.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }).unwrap(), SnapshotRetirementStep::Blocked));
    for _ in 0..100_000 {
        match sealer.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap() {
            SnapshotRetirementStep::Complete => {
                assert!(sealer.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("positive grant failed to retire retained owners"),
        }
    }
    panic!("bounded retirement did not terminate");
}

fn finish(sealer: &mut ArtifactStoreOneItemSealer<u64, FixtureMutation>, bytes: usize) -> [u8; 32] {
    let mut previous = sealer.completed_bytes;
    for _ in 0..100_000 {
        let step = sealer.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap();
        assert!(sealer.completed_bytes - previous <= bytes as u64);
        previous = sealer.completed_bytes;
        if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_)) {
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
        assert!(matches!(owner.advance(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: bytes }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
        owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap();
        for _ in 0..19 {
            owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap();
        }
        let checkpoint: ArtifactStoreOneItemSealCheckpoint = serde_json::from_slice(&serde_json::to_vec(&owner.checkpoint()).unwrap()).unwrap();
        let mut replay = sealer(&authority);
        replay.restore_checkpoint(checkpoint).unwrap();
        assert_eq!(finish(&mut replay, 7), oracle);
        authority.validate_prepared(replay.prepared().unwrap()).unwrap();
        close(&mut replay, 1);
        let mut moved = std::thread::spawn(move || {
            assert_eq!(finish(&mut owner, bytes), oracle);
            owner
        })
        .join()
        .unwrap();
        assert_eq!(moved.completed_bytes, 2 * moved.canonical_bytes + moved.header_offset as u64 + 7 + 2 * "edit-✓".len() as u64);
        authority.validate_prepared(moved.prepared().unwrap()).unwrap();
        close(&mut moved, 1);
    }
}

#[test]
fn canonical_sealer_rejects_stale_checkpoint_forged_prefix_and_rebound_owners() {
    let authority = authority();
    let mut owner = sealer(&authority);
    for _ in 0..4 {
        owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap();
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
        if replay.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).is_err() {
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
            owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }).unwrap();
        }
        owner.cancel();
        let checkpoint = owner.checkpoint();
        assert!(matches!(owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
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
    assert!(CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES > 1_024);
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
            let FixtureMutation::Replace { text, .. } = &mut edit.forwards[0];
            *text = "x".repeat(length.as_u64().unwrap() as usize);
            edit.coalesce_key = Some("coalesce-🧵".into());
            edit.finished_at = Some("finished".into());
            edit.mutation_meta[0].origin = crate::os_spr::MutationOrigin::from_value(origin.clone().into()).unwrap();
            edit.mutation_meta[0].payload_hash = Some(crate::os_spr::PayloadHash([23; 32]));
            edit.mutation_meta[0].semantic_kind = Some(SchemaId("fixture#replace".into()));
            let expected = serde_json::to_vec(&test_support::SerdeValue(&edit.to_value())).unwrap();
            let mut actual = Vec::new();
            let mut encoder = ArtifactCanonicalJsonCursor::default();
            let mut chunk = [0; 256];
            while !encoder.is_complete() {
                let count = encoder.encode_chunk(&edit, &mut chunk).unwrap();
                actual.extend_from_slice(&chunk[..count]);
            }
            assert_eq!(actual, expected);
            let digest = CursorRevisionAccumulator::edit_digest(&edit);
            let authority = authority();
            let mut owner = authority.begin_one_item_seal(edit, Arc::new(17), Arc::new(FixtureMutationRetirement), Arc::new(FixtureSnapshotRetirement));
            assert_eq!(finish(&mut owner, 4096), digest);
            close(&mut owner, 4096);
        }
    }
}

#[test]
fn canonical_authority_final_unicode_strings_retire_under_single_byte_grants() {
    let mut authority = authority();
    Arc::get_mut(&mut authority).unwrap().actor = "actor-🧵".into();
    Arc::get_mut(&mut authority).unwrap().group_id = Some("group-✓".into());
    let mut retirement = authority.retire();
    let mut released = 0;
    assert!(matches!(retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked));
    for _ in 0..100 {
        match retirement.close_step(1, 1).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= 1);
                released += released_bytes;
            }
            SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                assert_eq!(released, "actor-🧵".len() + "group-✓".len());
                return;
            }
            SnapshotRetirementStep::Blocked => panic!("positive retirement grant blocked"),
        }
    }
    panic!("final authority strings did not retire");
}
