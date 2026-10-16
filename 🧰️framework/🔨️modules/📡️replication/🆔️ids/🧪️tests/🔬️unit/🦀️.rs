//! 🧪️ Hybrid logical timestamp and identifier laws.

use super::*;

//#region 🔖️HybridLogicalTimestamp
#[test]
fn hlc_tick_advances_on_newer_physical_time() {
    let mut hlc = HybridLogicalTimestamp::new(1, 100);
    hlc.tick(200);
    assert_eq!(hlc.physical_ms, 200);
    assert_eq!(hlc.logical, 0);
}

#[test]
fn hlc_tick_bumps_logical_on_equal_or_older_physical_time() {
    let mut hlc = HybridLogicalTimestamp::new(1, 100);
    hlc.tick(100);
    assert_eq!(hlc.logical, 1);
    hlc.tick(50);
    assert_eq!(hlc.logical, 2);
}

#[test]
fn hlc_merge_adopts_the_greater_remote_tick_then_bumps() {
    let mut local = HybridLogicalTimestamp::new(1, 100);
    let remote = HybridLogicalTimestamp { actor: 2, physical_ms: 150, logical: 3 };
    local.merge(&remote);
    assert_eq!(local.physical_ms, 150);
    assert_eq!(local.logical, 4);
}

#[test]
fn hlc_ordering_uses_actor_as_final_tiebreak() {
    let a = HybridLogicalTimestamp { actor: 1, physical_ms: 100, logical: 5 };
    let b = HybridLogicalTimestamp { actor: 2, physical_ms: 100, logical: 5 };
    assert!(a < b, "equal physical_ms/logical must tiebreak by actor, not compare Equal");
    assert_ne!(a.cmp_key(), b.cmp_key());
}

#[test]
fn hlc_ordering_prioritizes_physical_then_logical_then_actor() {
    let older = HybridLogicalTimestamp { actor: 9, physical_ms: 100, logical: 0 };
    let newer_physical = HybridLogicalTimestamp { actor: 0, physical_ms: 101, logical: 0 };
    let newer_logical = HybridLogicalTimestamp { actor: 0, physical_ms: 100, logical: 1 };
    assert!(older < newer_physical);
    assert!(older < newer_logical);
    assert!(newer_logical < newer_physical);
}
//#endregion 🔖️HybridLogicalTimestamp

//#region 🔖️Identifiers
/// 🌱️ Rewritten off `serde_json` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): asserts the same transparent shape directly on `DslValue`.
#[test]
fn identifier_newtypes_to_value_round_trip_transparently() {
    let op = MutationId("op-1".to_string());
    let value = semio_framework_value::ToValue::to_value(&op);
    assert_eq!(value, semio_framework_value::DslValue::String("op-1".to_string()));
    assert_eq!(<MutationId as semio_framework_value::FromValue>::from_value(value).unwrap(), op);

    let version = ArtifactVersion(42);
    let value = semio_framework_value::ToValue::to_value(&version);
    assert_eq!(value, semio_framework_value::DslValue::uint(42));
    assert_eq!(<ArtifactVersion as semio_framework_value::FromValue>::from_value(value).unwrap(), version);
}

#[test]
fn document_version_orders_numerically() {
    assert!(ArtifactVersion(1) < ArtifactVersion(2));
}
//#endregion 🔖️Identifiers
