//! 🌳️ Language-neutral borrowed-map, lifetime, and worker replay laws.

use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

//#region 🪪️LifetimeOracle
#[derive(Default, Debug)]
pub(super) struct MapLifetime {
    pub(super) active_iterators: AtomicUsize,
    pub(super) iterator_drops: AtomicUsize,
    pub(super) root_drops: AtomicUsize,
}

#[derive(Debug, Serialize, Deserialize, semio_framework_value::RetireOwned)]
#[serde(untagged)]
pub(super) enum MapValue {
    Text(String),
    Array(Vec<MapValue>),
    Object(MapFields),
}

#[derive(Debug, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(retire_with = "std::mem::drop")]
pub(super) enum MapMutation { ReplaceMap(MapBody) }
#[derive(Debug, Serialize, Deserialize, ToValue, FromValue)]
#[value(retire_with = "std::mem::drop")]
pub(super) struct MapBody {
    map: MapFields,
    #[serde(skip)]
    #[value(skip)]
    lifetime: Arc<MapLifetime>,
    #[serde(skip)]
    #[value(skip)]
    tracked: bool,
}
#[derive(Debug, semio_framework_value::RetireOwned)]
pub(super) struct MapFields(semio_framework_value::ordered::OrderedMap<MapValue>);
impl Serialize for MapFields{
 fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeMap;let mut map=serializer.serialize_map(Some(self.0.len()))?;for(key,value)in self.0.iter(){map.serialize_entry(key,value)?;}map.end()}
}
impl<'de> Deserialize<'de> for MapFields{
 fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{
  struct OriginalMapVisitor;
  impl<'de> serde::de::Visitor<'de> for OriginalMapVisitor{type Value=MapFields;fn expecting(&self,formatter:&mut std::fmt::Formatter)->std::fmt::Result{formatter.write_str("original ordered object")}
   fn visit_map<A:serde::de::MapAccess<'de>>(self,mut map:A)->Result<MapFields,A::Error>{let mut entries=Vec::new();while let Some(entry)=map.next_entry::<String,MapValue>()?{entries.push(entry);}Ok(MapFields(entries.into_iter().collect()))}
  }
  deserializer.deserialize_map(OriginalMapVisitor)
 }
}
impl ToValue for MapFields { fn to_value(&self) -> DslValue { self.0.to_value() } }
impl FromValue for MapFields { fn from_value(value: DslValue) -> Result<Self, ValueError> { semio_framework_value::ordered::OrderedMap::from_value(value).map(Self) } }
impl ArtifactCanonicalJsonTree for MapFields {
    fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(ArtifactCanonicalJsonNode::Object(self.0.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { self.0.entry_at_rank(ordinal).map(|(_, value)| value as &dyn ArtifactCanonicalJsonTree).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "map fixture original child ordinal missing")) }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { self.0.entry_at_rank(ordinal).map(|(key, _)| key.as_str().into()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "map fixture original key ordinal missing")) }
}
impl ArtifactCanonicalJsonTree for MapValue {
 fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> { match self { Self::Text(value) => value.canonical_tree_node(), Self::Array(value) => value.canonical_tree_node(), Self::Object(value) => value.canonical_tree_node() } }
 fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { match self { Self::Text(value) => value.canonical_tree_child(ordinal), Self::Array(value) => value.canonical_tree_child(ordinal), Self::Object(value) => value.canonical_tree_child(ordinal) } }
 fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { match self { Self::Text(value) => value.canonical_tree_key(ordinal), Self::Array(value) => value.canonical_tree_key(ordinal), Self::Object(value) => value.canonical_tree_key(ordinal) } }
}
impl ArtifactCanonicalJsonTree for MapBody {
 fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(ArtifactCanonicalJsonNode::Object(1)) }
 fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { if ordinal == 0 { Ok(&self.map) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "map fixture original ordinal missing")) } }
 fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { if ordinal == 0 { Ok("map".into()) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "map fixture original key missing")) } }
}
impl ArtifactCanonicalJsonTree for MapMutation {
 fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(ArtifactCanonicalJsonNode::Object(1)) }
 fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { let Self::ReplaceMap(body) = self; if ordinal == 0 { Ok(body) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "map fixture original ordinal missing")) } }
 fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { if ordinal == 0 { Ok("ReplaceMap".into()) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "map fixture original key missing")) } }
}

impl ToValue for MapValue {
    fn to_value(&self) -> DslValue {
        match self {
            Self::Text(value) => value.to_value(),
            Self::Array(value) => value.to_value(),
            Self::Object(value) => value.to_value(),
        }
    }
}

impl FromValue for MapValue {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(value) => Ok(Self::Text(value)),
            DslValue::Array(value) => value.into_iter().map(Self::from_value).collect::<Result<_, _>>().map(Self::Array),
            DslValue::Object(value) => value.into_iter().map(|(key, value)| Ok((key, Self::from_value(value)?))).collect::<Result<semio_framework_value::ordered::OrderedMap<_>, ValueError>>().map(|map| Self::Object(MapFields(map))),
            _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "map fixture requires text, array, or object")),
        }
    }
}

impl Drop for MapBody {
    fn drop(&mut self) {
        let Self { lifetime, tracked, .. } = self;
        if *tracked {
            assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
            lifetime.root_drops.fetch_add(1, Ordering::SeqCst);
        }
    }
}

struct MapIterator<'a> {
    entries: semio_framework_value::ordered::Iter<'a, MapValue>,
    lifetime: &'a MapLifetime,
}
impl<'a> Iterator for MapIterator<'a> {
    type Item = (&'a str, ArtifactCanonicalJsonValue<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next().map(|(key, value)| (key.as_str(), map_value(value, self.lifetime)))
    }
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
fn map_object<'a>(map: &'a MapFields, lifetime: &'a MapLifetime) -> ArtifactCanonicalJsonValue<'a> {
    lifetime.active_iterators.fetch_add(1, Ordering::SeqCst);
    ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new(MapIterator { entries: map.0.iter(), lifetime }))
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
        let Self::ReplaceMap(MapBody { map, lifetime, .. }) = self;
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
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct MapRetirementFactory;
impl ArtifactOwnedValueRetirementFactory<MapMutation> for MapRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &MapMutation) -> usize {
        semio_framework_value::retirement::owned_retirement_birth_bytes::<MapMutation>()
    }
    fn retire_owned(&self, value: MapMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, MapMutation)> {
        let MapMutation::ReplaceMap(MapBody { lifetime, .. }) = &value;
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}
impl semio_framework_value::retirement::RetireOwned for MapLifetime {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        use semio_framework_value::retirement::RetireOwned;
        [self.active_iterators.into_inner(), self.iterator_drops.into_inner(), self.root_drops.into_inner()].retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&[0usize; 3])
    }
    fn controlled_retirement_supported() -> bool { true }
}
struct MapTerminalLifetime { lifetime: Arc<MapLifetime>, tracked: bool }
impl semio_framework_value::retirement::RetireOwned for MapTerminalLifetime {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        use semio_framework_value::retirement::RetireOwned;
        if self.tracked {
            assert_eq!(self.lifetime.active_iterators.load(Ordering::SeqCst), 0);
            self.lifetime.root_drops.fetch_add(1, Ordering::SeqCst);
        }
        self.lifetime.retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.lifetime)
    }
    fn controlled_retirement_supported() -> bool { true }
}
impl semio_framework_value::retirement::RetireOwned for MapBody {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        use semio_framework_value::retirement::{deferred, sequence};
        let original = std::mem::ManuallyDrop::new(self);
        let Self { map, lifetime, tracked } = &*original;
        let (map, lifetime) = unsafe { (std::ptr::read(map), std::ptr::read(lifetime)) };
        sequence(vec![deferred(MapTerminalLifetime { lifetime, tracked: *tracked }), deferred(map)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        use semio_framework_value::retirement::{deferred_birth_bytes, sequence_birth_bytes};
        sequence_birth_bytes(&[deferred_birth_bytes::<MapFields>(), deferred_birth_bytes::<MapTerminalLifetime>()])
    }
    fn controlled_retirement_supported() -> bool { true }
}
pub(super) fn fixture_retirement_grant() -> RetainedCloneGrant {
    let input: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📖️canonical-reader.json")).unwrap();let row=&input["retirementGrant"];
    RetainedCloneGrant { maximum_items: row["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: row["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: row["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: row["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: row["maximumDepth"].as_u64().unwrap() as usize }
}
fn fixture_encoding_grant(items: usize, copy: usize) -> ArtifactStoreOneItemGrant {
    let policy=fixture_retirement_grant();
    ArtifactStoreOneItemGrant { maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:policy.maximum_capacity_bytes,maximum_release_bytes:policy.maximum_release_bytes,maximum_depth:policy.maximum_depth }
}
fn retire_fixture_owned<T: semio_framework_value::retirement::RetireOwned>(original: T) {
    let grant = fixture_retirement_grant();
    let (admitted, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::retirement::admit_owned_retirement(original, grant));
    let (owner, progress) = admitted.unwrap_or_else(|_| panic!("fixture map original frame was refused"));
    assert!(progress.fits(grant)); assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
    let mut owner = Some(owner);
    for _ in 0..100_000 {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| super::super::artifact_retirement_box_close_step(&mut owner, grant).unwrap());
        assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        if owner.is_none() { return; }
    }
    panic!("fixture original map did not retire");
}
//#endregion 🧹️OwnedRetirement

//#region 📦️FixtureOwners
pub(super) fn fixture() -> (Edit<MapMutation>, serde_json::Value, Arc<MapLifetime>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🗺️canonical-borrowed-map.json")).unwrap();
    let mut edit = Edit::<MapMutation>::from_value(fixture["edit"].clone().into()).unwrap();
    let MapMutation::ReplaceMap(MapBody { lifetime, tracked, .. }) = &mut edit.forwards[0];
    *tracked = true;
    let lifetime = Arc::clone(lifetime);
    (edit, fixture, lifetime)
}

fn owner() -> (ArtifactStoreOneItemSealer<u64, MapMutation>, serde_json::Value, Arc<MapLifetime>) {
    let (edit, fixture, lifetime) = fixture();
    (tests::admit_sealer(tests::authority(), edit, Arc::new(17), Arc::new(MapRetirementFactory), Arc::new(tests::FixtureSnapshotRetirement)), fixture, lifetime)
}

fn close(owner: &mut ArtifactStoreOneItemSealer<u64, MapMutation>, lifetime: &MapLifetime) {
    let grant = fixture_retirement_grant();
    owner.begin_close();
    assert_eq!(owner.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap().progress(), RetainedCloneProgress::default());
    for _ in 0..100_000 {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty());assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst),0);assert_eq!(lifetime.root_drops.load(Ordering::SeqCst),1);return; }
    }
    panic!("borrowed root retirement did not finish");
}
//#endregion 📦️FixtureOwners

//#region 🧪️BorrowedMapLaws

fn map_payload_capacity(map: &MapFields) -> usize {
    fn capacity(value: &MapValue) -> usize {
        match value { MapValue::Text(text) => text.capacity(), MapValue::Array(values) => values.iter().map(capacity).sum(), MapValue::Object(map) => map_payload_capacity(map) }
    }
    map.0.iter().map(|(key,value)|key.capacity()+capacity(value)).sum()
}
#[test]
fn member_canonical_reader_map_retirement_preserves_refused_original_and_paid_cancel_frontiers() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    let plain: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🗺️canonical-borrowed-map.json")).unwrap();
    let policy=fixture_retirement_grant();
    for cancel_cut in [0,1,3] {
        let mut original: MapMutation=serde_json::from_value(plain["edit"]["forwards"][0].clone()).unwrap();
        let MapMutation::ReplaceMap(body)=&mut original;body.tracked=true;
        let lifetime=Arc::clone(&body.lifetime);let key_pointer=body.map.0.entry_at_rank(0).unwrap().0.as_ptr();let payload=map_payload_capacity(&body.map);
        for denied in [RetainedCloneGrant{maximum_items:0,..policy},RetainedCloneGrant{maximum_capacity_bytes:0,..policy},RetainedCloneGrant{maximum_depth:0,..policy}] {
            let (result,heap)=observe_heap_allocations_on_this_thread(||MapRetirementFactory.retire_owned(original,denied));
            original=match result { Err((_,original))=>original,Ok(_)=>panic!("denied original map moved") };
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            let MapMutation::ReplaceMap(body)=&original;assert!(Arc::ptr_eq(&body.lifetime,&lifetime));assert_eq!(body.map.0.entry_at_rank(0).unwrap().0.as_ptr(),key_pointer);assert_eq!(lifetime.root_drops.load(Ordering::SeqCst),0);
            assert_eq!(serde_json::to_value(&original).unwrap(),plain["edit"]["forwards"][0]);
        }
        let (result,heap)=observe_heap_allocations_on_this_thread(||MapRetirementFactory.retire_owned(original,policy));
        let (owner,birth)=result.unwrap_or_else(|_|panic!("independently funded original map refused"));
        assert!(birth.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,birth.released_bytes));
        let mut owner=Some(owner);let mut released=birth.released_bytes;
        for turn in 0..100_000 {
            if turn==cancel_cut {
                let (step,heap)=observe_heap_allocations_on_this_thread(||super::super::artifact_retirement_box_close_step(&mut owner,RetainedCloneGrant{maximum_items:0,..policy}).unwrap());
                assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(owner.is_some());
            }
            let (step,heap)=observe_heap_allocations_on_this_thread(||super::super::artifact_retirement_box_close_step(&mut owner,policy).unwrap());
            let progress=step.progress();assert!(progress.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));released+=progress.released_bytes;
            if lifetime.root_drops.load(Ordering::SeqCst)==1 {assert!(released>=payload);}
            if owner.is_none(){break;}assert!(turn+1<100_000);
        }
        assert!(owner.is_none());assert_eq!(lifetime.root_drops.load(Ordering::SeqCst),1);assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst),0);
        let (_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    println!("[DEBUG] member reader map retains original key pointer/serde bytes on denied admission; independent grant pays every System birth/release and cancel frontier, map backing retires before exact lifetime notification");
}
#[test]
fn borrowed_map_long_unicode_keys_nested_and_empty_maps_match_serde_under_tiny_grants() {
    for maximum in [1, 7, 256, 4096] {
        let (mut owner, fixture, lifetime) = owner();
        let expected = serde_json::to_vec(&test_support::SerdeValue(&crate::os_store::component::CursorRevisionAccumulator::revision_value(owner.edit.as_ref().unwrap().as_ref()))).unwrap();
        assert_eq!(expected, fixture["expectedJson"].as_str().unwrap().as_bytes());
        let before = owner.checkpoint();
        for (maximum_items, maximum_bytes) in [(0, maximum), (1, 0)] {
            assert!(matches!(owner.advance(fixture_encoding_grant(maximum_items, maximum_bytes)).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
            assert_eq!(owner.checkpoint(), before);
            assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        }
        let mut prior = 0;
        let mut canonical = Vec::new();
        for _ in 0..100_000 {
            let phase = owner.phase;
            let step = owner.advance(fixture_encoding_grant(1, maximum)).unwrap();
            if phase == 3 {
                canonical.extend_from_slice(owner.canonical_chunk());
            }
            assert!(owner.completed_bytes - prior <= maximum as u64);
            prior = owner.completed_bytes;
            if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(..)) {
                break;
            }
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
        while owner.phase < phase {
            owner.advance(fixture_encoding_grant(1, 7)).unwrap();
        }
        owner.cancel();
        assert!(matches!(owner.advance(fixture_encoding_grant(1, 4096)).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
        close(&mut owner, &lifetime);
    }
    let (mut owner, fixture, lifetime) = owner();
    let key_start = fixture["expectedJson"].as_str().unwrap().find("key-").unwrap() as u64;
    while owner.canonical_bytes <= key_start + 128 {
        owner.advance(fixture_encoding_grant(1, 1)).unwrap();
    }
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
    for _ in 0..100 {
        first.advance(fixture_encoding_grant(1, 7)).unwrap();
    }
    let checkpoint: ArtifactStoreOneItemSealCheckpoint = serde_json::from_slice(&serde_json::to_vec(&first.checkpoint()).unwrap()).unwrap();
    let (mut replay, _, replay_lifetime) = owner();
    replay.restore_checkpoint(checkpoint).unwrap();
    let mut replay = std::thread::spawn(move || {
        for _ in 0..100_000 {
            if matches!(replay.advance(fixture_encoding_grant(1, 1)).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(..)) {
                return replay;
            }
        }
        panic!("checkpoint did not finish");
    })
    .join()
    .unwrap();
    let digest = replay.prepared().unwrap().edit_digest();
    assert_eq!(digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), fixture["expectedDigest"].as_str().unwrap());
    close(&mut replay, &replay_lifetime);
    let mut first = std::thread::spawn(move || {
        first.advance(fixture_encoding_grant(1, 7)).unwrap();
        first
    })
    .join()
    .unwrap();
    close(&mut first, &lifetime);
}

#[test]
fn borrowed_map_rebound_root_and_depth_overflow_fail_before_references_escape() {
    let mut indexed = ArtifactCanonicalJsonCursor { maximum_depth: 2, ..ArtifactCanonicalJsonCursor::default() };
    assert_eq!(indexed.encode_chunk(&IndexedDepth(2), &mut [0; 256]).unwrap_err(), ArtifactCanonicalJsonEncodeError { written_bytes: 2, reason: "canonical-edit.depth-limit".into() });
    let (mut owner, _, lifetime) = owner();
    for _ in 0..100 {
        owner.advance(fixture_encoding_grant(1, 7)).unwrap();
    }
    let (replacement, _, _) = fixture();
    let original = owner.edit.replace(Box::new(replacement)).unwrap();
    assert_eq!(owner.advance(fixture_encoding_grant(1, 7)).unwrap_err(), "canonical-edit.borrowed-root-rebound");
    let replacement = owner.edit.replace(original).unwrap();
    retire_fixture_owned(replacement);
    close(&mut owner, &lifetime);

    let (mut owner, _, lifetime) = self::owner();
    let MapMutation::ReplaceMap(MapBody { map, .. }) = &mut owner.edit.as_mut().unwrap().forwards[0];
    let mut value = MapValue::Text("leaf".into());
    for _ in 0..ARTIFACT_CANONICAL_JSON_DEPTH + 1 {
        value = MapValue::Object(MapFields(semio_framework_value::ordered::OrderedMap::from([("nested".into(), value)])));
    }
    let original = std::mem::replace(map, MapFields(semio_framework_value::ordered::OrderedMap::from([("root".into(), value)])));
    retire_fixture_owned(original);
    let mut rejected = false;
    for _ in 0..100_000 {
        match owner.advance(fixture_encoding_grant(1, 1)) {
            Err(reason) => {
                assert_eq!(reason, "canonical-edit.depth-limit");
                rejected = true;
                break;
            }
            Ok(ArtifactStoreOneItemPreparationStep::Prepared(..)) => panic!("over-depth map sealed"),
            _ => {}
        }
    }
    assert!(rejected);
    close(&mut owner, &lifetime);
}
//#endregion 🧪️BorrowedMapLaws
