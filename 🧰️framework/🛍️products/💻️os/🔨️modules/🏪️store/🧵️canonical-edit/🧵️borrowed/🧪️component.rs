//! 🌳️ Language-neutral borrowed-map, lifetime, and worker replay laws.

use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

//#region 🪪️LifetimeOracle
#[derive(Default, Debug)]
pub(super) struct MapLifetime { pub(super) active_iterators: AtomicUsize, pub(super) iterator_drops: AtomicUsize, pub(super) root_drops: AtomicUsize }

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum MapValue { Text(String), Array(Vec<MapValue>), Object(BTreeMap<String, MapValue>) }

#[derive(Debug, Serialize, Deserialize)]
pub(super) enum MapMutation {
    ReplaceMap {
        map: BTreeMap<String, MapValue>,
        #[serde(skip)]
        lifetime: Arc<MapLifetime>,
        #[serde(skip)]
        tracked: bool,
    },
}

impl Drop for MapMutation {
    fn drop(&mut self) {
        let Self::ReplaceMap { lifetime, tracked, .. } = self;
        if *tracked {
            assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
            lifetime.root_drops.fetch_add(1, Ordering::SeqCst);
        }
    }
}

struct MapIterator<'a> { entries: std::collections::btree_map::Iter<'a, String, MapValue>, lifetime: &'a MapLifetime }
impl<'a> Iterator for MapIterator<'a> {
    type Item = (&'a str, ArtifactCanonicalJsonValue<'a>);
    fn next(&mut self) -> Option<Self::Item> { self.entries.next().map(|(key, value)| (key.as_str(), map_value(value, self.lifetime))) }
}
impl Drop for MapIterator<'_> {
    fn drop(&mut self) {
        assert_eq!(self.lifetime.root_drops.load(Ordering::SeqCst), 0);
        self.lifetime.active_iterators.fetch_sub(1, Ordering::SeqCst);
        self.lifetime.iterator_drops.fetch_add(1, Ordering::SeqCst);
    }
}
//#endregion 🪪️LifetimeOracle

//#region 🧬️TypedTraversal
fn map_object<'a>(map: &'a BTreeMap<String, MapValue>, lifetime: &'a MapLifetime) -> ArtifactCanonicalJsonValue<'a> {
    lifetime.active_iterators.fetch_add(1, Ordering::SeqCst);
    ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new(MapIterator { entries: map.iter(), lifetime }))
}

fn map_value<'a>(value: &'a MapValue, lifetime: &'a MapLifetime) -> ArtifactCanonicalJsonValue<'a> {
    match value {
        MapValue::Text(text) => ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String(text)),
        MapValue::Array(values) => ArtifactCanonicalJsonValue::Array(ArtifactCanonicalJsonArray::new(values.iter().map(move |value| map_value(value, lifetime)))),
        MapValue::Object(map) => map_object(map, lifetime),
    }
}

impl ArtifactCanonicalJson for MapMutation {
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, String> {
        let Self::ReplaceMap { map, lifetime, .. } = self;
        let fields = ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new([("map", map_object(map, lifetime))].into_iter()));
        Ok(Some(ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new([("ReplaceMap", fields)].into_iter()))))
    }
}

struct IndexedDepth(usize);
impl ArtifactCanonicalJson for IndexedDepth {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        Ok(if path.len() == self.0 { ArtifactCanonicalJsonNode::Null } else { ArtifactCanonicalJsonNode::Array(1) })
    }
}
//#endregion 🧬️TypedTraversal

//#region 🧹️OwnedRetirement
struct MapRetirement { owner: Option<MapMutation>, nodes: Vec<MapValue>, active: Option<Vec<u8>> }
impl ErasedSnapshotRetirement for MapRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if items == 0 || bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(active) = self.active.as_mut() {
            let released_bytes = active.len().min(bytes);
            active.truncate(active.len() - released_bytes);
            if active.is_empty() { self.active = None; }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        if let Some(node) = self.nodes.last_mut() {
            match node {
                MapValue::Text(text) => { self.active = Some(std::mem::take(text).into_bytes()); self.nodes.pop(); }
                MapValue::Array(values) => if let Some(value) = values.pop() { self.nodes.push(value); } else { self.nodes.pop(); },
                MapValue::Object(map) => if let Some((key, value)) = map.pop_first() { self.active = Some(key.into_bytes()); self.nodes.push(value); } else { self.nodes.pop(); },
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.owner.take().is_some() { return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.nodes.is_empty() && self.active.is_none() }
}

pub(super) struct MapRetirementFactory;
impl ArtifactOwnedValueRetirementFactory<MapMutation> for MapRetirementFactory {
    fn retire_owned(&self, mut value: MapMutation) -> Box<dyn ErasedSnapshotRetirement> {
        let MapMutation::ReplaceMap { map, lifetime, .. } = &mut value;
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        let mut nodes = Vec::with_capacity(ARTIFACT_CANONICAL_JSON_DEPTH * 2);
        nodes.push(MapValue::Object(std::mem::take(map)));
        Box::new(MapRetirement { owner: Some(value), nodes, active: None })
    }
}
//#endregion 🧹️OwnedRetirement

//#region 📦️FixtureOwners
pub(super) fn fixture() -> (Edit<MapMutation>, serde_json::Value, Arc<MapLifetime>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🔣️canonical-borrowed-map.json")).unwrap();
    let mut edit: Edit<MapMutation> = serde_json::from_value(fixture["edit"].clone()).unwrap();
    let MapMutation::ReplaceMap { lifetime, tracked, .. } = &mut edit.forwards[0];
    *tracked = true;
    let lifetime = Arc::clone(lifetime);
    (edit, fixture, lifetime)
}

fn owner() -> (ArtifactStoreOneItemSealer<u64, MapMutation>, serde_json::Value, Arc<MapLifetime>) {
    let (edit, fixture, lifetime) = fixture();
    (super::tests::authority().begin_one_item_seal(edit, Arc::new(17), Arc::new(MapRetirementFactory), Arc::new(super::tests::FixtureSnapshotRetirement)), fixture, lifetime)
}

fn close(owner: &mut ArtifactStoreOneItemSealer<u64, MapMutation>, lifetime: &MapLifetime) {
    owner.begin_close();
    assert!(matches!(owner.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }).unwrap(), SnapshotRetirementStep::Blocked));
    for _ in 0..100_000 {
        match owner.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).unwrap() {
            SnapshotRetirementStep::Complete => {
                assert!(owner.terminal_is_empty());
                assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
                assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 1);
                return;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 1); }
            SnapshotRetirementStep::Blocked => panic!("positive grant failed to retire borrowed root"),
        }
    }
    panic!("borrowed root retirement did not finish");
}
//#endregion 📦️FixtureOwners

//#region 🧪️BorrowedMapLaws
#[test]
fn borrowed_map_long_unicode_keys_nested_and_empty_maps_match_serde_under_tiny_grants() {
    for maximum in [1, 7, 256, 4096] {
        let (mut owner, fixture, lifetime) = owner();
        let expected = serde_json::to_vec(owner.edit.as_ref().unwrap().as_ref()).unwrap();
        assert_eq!(expected, fixture["expectedJson"].as_str().unwrap().as_bytes());
        let before = owner.checkpoint();
        for (maximum_items, maximum_bytes) in [(0, maximum), (1, 0)] {
            assert!(matches!(owner.advance(ArtifactStoreOneItemGrant { maximum_items, maximum_bytes }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
            assert_eq!(owner.checkpoint(), before);
            assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        }
        let mut prior = 0;
        let mut canonical = Vec::new();
        for _ in 0..100_000 {
            let phase = owner.phase;
            let step = owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: maximum }).unwrap();
            if phase == 3 { canonical.extend_from_slice(owner.canonical_chunk()); }
            assert!(owner.completed_bytes - prior <= maximum as u64);
            prior = owner.completed_bytes;
            if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_)) { break; }
        }
        assert_eq!(canonical, expected);
        let digest = owner.prepared().unwrap().edit_digest();
        let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(hex, fixture["expectedDigest"].as_str().unwrap());
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        assert!(lifetime.iterator_drops.load(Ordering::SeqCst) > 0);
        close(&mut owner, &lifetime);
    }
}

#[test]
fn borrowed_map_cancel_every_phase_retires_iterators_before_exact_root_once() {
    for phase in 0..=6 {
        let (mut owner, _, lifetime) = owner();
        while owner.phase < phase { owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap(); }
        owner.cancel();
        assert!(matches!(owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
        close(&mut owner, &lifetime);
    }
    let (mut owner, fixture, lifetime) = owner();
    let key_start = fixture["expectedJson"].as_str().unwrap().find("key-").unwrap() as u64;
    while owner.canonical_bytes <= key_start + 128 { owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).unwrap(); }
    assert_eq!(owner.phase, 1);
    assert!(owner.canonical_bytes < key_start + fixture["longKeyBytes"].as_u64().unwrap());
    assert!(lifetime.active_iterators.load(Ordering::SeqCst) > 0);
    assert!(lifetime.iterator_drops.load(Ordering::SeqCst) > 0);
    owner.cancel();
    close(&mut owner, &lifetime);
}

#[test]
fn borrowed_map_checkpoint_replays_fresh_root_and_live_owner_moves_workers() {
    let (mut first, fixture, lifetime) = owner();
    for _ in 0..100 { first.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap(); }
    let checkpoint: ArtifactStoreOneItemSealCheckpoint = serde_json::from_slice(&serde_json::to_vec(&first.checkpoint()).unwrap()).unwrap();
    let (mut replay, _, replay_lifetime) = owner();
    replay.restore_checkpoint(checkpoint).unwrap();
    let mut replay = std::thread::spawn(move || {
        for _ in 0..100_000 { if matches!(replay.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(_)) { return replay; } }
        panic!("checkpoint did not finish");
    }).join().unwrap();
    let digest = replay.prepared().unwrap().edit_digest();
    assert_eq!(digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), fixture["expectedDigest"].as_str().unwrap());
    close(&mut replay, &replay_lifetime);
    let mut first = std::thread::spawn(move || { first.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap(); first }).join().unwrap();
    close(&mut first, &lifetime);
}

#[test]
fn borrowed_map_rebound_root_and_depth_overflow_fail_before_references_escape() {
    let mut indexed = ArtifactCanonicalJsonCursor { maximum_depth: 2, ..ArtifactCanonicalJsonCursor::default() };
    assert_eq!(indexed.encode_chunk(&IndexedDepth(2), &mut [0; 256]).unwrap_err(), "canonical-edit.depth-limit");
    let (mut owner, _, lifetime) = owner();
    for _ in 0..100 { owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap(); }
    let (replacement, _, _) = fixture();
    let original = owner.edit.replace(Box::new(replacement)).unwrap();
    assert_eq!(owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap_err(), "canonical-edit.borrowed-root-rebound");
    owner.edit = Some(original);
    close(&mut owner, &lifetime);

    let (mut owner, _, lifetime) = self::owner();
    let MapMutation::ReplaceMap { map, .. } = &mut owner.edit.as_mut().unwrap().forwards[0];
    let mut value = MapValue::Text("leaf".into());
    for _ in 0..ARTIFACT_CANONICAL_JSON_DEPTH + 1 { value = MapValue::Object(BTreeMap::from([("nested".into(), value)])); }
    *map = BTreeMap::from([("root".into(), value)]);
    let mut rejected = false;
    for _ in 0..100_000 {
        match owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }) {
            Err(reason) => { assert_eq!(reason, "canonical-edit.depth-limit"); rejected = true; break; }
            Ok(ArtifactStoreOneItemPreparationStep::Prepared(_)) => panic!("over-depth map sealed"),
            _ => {}
        }
    }
    assert!(rejected);
    close(&mut owner, &lifetime);
}
//#endregion 🧪️BorrowedMapLaws
