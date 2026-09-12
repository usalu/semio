//! 📃️ LAW: how a per-frame retained-document READ may reach its lease.
//!
//! `UiDocumentLease::try_alias` mints an arena alias and only `close_read_step_with_grant` gives one
//! back, so a reader that aliases per frame step and drops the alias exhausts the slot's
//! [`UI_DOCUMENT_LEASE_ALIASES`] credit and every later read answers `AliasCapacity`. The wgpu chrome
//! walk did exactly that: its window body silently vanished from the walk after the seventh step of
//! the very first frame, its page ingress froze mid-document, and the shell presented an empty tree
//! for the rest of the session (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️wgpu-blank-paint-2026-09-12.md`). Moving the one owner out and back mints nothing and therefore
//! cannot run out — that is the rule this file pins, against the real arena, with the neutral fixture
//! `🖱️ui/🧫️fixtures/📃️document-lease-owner-move/🔣️.json` as the shared oracle
//! (its TypeScript twin is `📺️renderer/🧑‍🎨engine/🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts`).

use super::*;

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📃️document-lease-owner-move/🔣️.json")).expect("document lease owner-move fixture")
}

fn retire(lease: &mut UiDocumentLease) {
    for _ in 0..100_000 {
        if lease.close_read_step_with_grant(1, 32768).expect("exact retirement authority").complete {
            return;
        }
    }
    panic!("document lease did not retire");
}

fn published_lease() -> UiDocumentLease {
    let mut owner = UiDocumentAssembly::default();
    let mut surface = Some(SurfaceId::try_from("owner-move").expect("fixture surface id"));
    let identity = UiDocumentAssemblyIdentity { generation: 7, revision: UiRevision(1), root: Some(UiNodeId(1)), layout_epoch: 0 };
    for _ in 0..64 {
        if owner.open_into(&mut surface, identity.clone(), 1, 32768).expect("open admits").progressed {
            break;
        }
    }
    assert!(surface.is_none(), "assembly claimed the exact surface identity");
    let mut record = Some(tests::leaf_record(1, "root"));
    for _ in 0..64 {
        owner.place_one(&mut record, 1, 32768).expect("record placement admits");
        if record.is_none() {
            break;
        }
    }
    let mut target = None;
    for _ in 0..64 {
        if owner.finish_into(&mut target, UiRevision(1), 1, 32768).expect("finish admits").complete {
            break;
        }
    }
    target.expect("published document lease")
}

/// 🔒️ `UI_DOCUMENT_ARENA` is one fixed, process-wide slot table shared by every test in this binary,
/// so the two halves of this law take their leases in ONE test rather than racing each other for a
/// slot — a live lease elsewhere in the binary is what `ArenaFull` means.
#[test]
fn a_per_frame_read_moves_the_exact_owner_because_an_alias_credit_runs_out() {
    let law = law();
    let capacity = law["aliasCapacity"].as_u64().expect("alias capacity");
    let admitted = law["aliasReads"]["admitted"].as_u64().expect("admitted reads") as usize;
    let refused_at = law["aliasReads"]["refusedAt"].as_u64().expect("refusal index") as usize;
    assert_eq!(capacity, UI_DOCUMENT_LEASE_ALIASES, "the fixture is pinned to the ratified alias capacity");
    assert_eq!(admitted + law["publishedAliases"].as_u64().expect("published aliases") as usize, capacity as usize, "a published document already holds one of the slot's aliases");
    assert_eq!(refused_at, admitted + 1, "the refusal lands on the read after the last admitted one");

    let mut lease = published_lease();

    // ✅️ The rule: lend the ONE owner for the step and take it back. The same read repeats without
    // bound because it mints no alias at all.
    let reads = law["ownerMove"]["reads"].as_u64().expect("owner-move reads");
    let mut owner = Some(lease);
    for read in 0..reads {
        let moved = owner.take().expect("the one owner is lent for exactly one step");
        assert!(moved.root_identity().is_some(), "owner-move read {read} still addresses its exact root");
        owner = Some(moved);
    }
    assert_eq!(law["ownerMove"]["refusals"].as_u64(), Some(0), "an owner move mints no alias and therefore never refuses");
    lease = owner.take().expect("the owner comes back from its last loan");

    // ♻️ A retired alias hands its credit straight back and never retires the published owner.
    let mut returned = lease.try_alias().expect("first alias");
    assert!(returned.close_read_step_with_grant(1, 4096).expect("alias retirement is exact").complete, "releasing a non-final alias completes in one step");
    assert!(lease.root_identity().is_some(), "the published owner outlives every alias it lent");

    // 🩸️ The defect, reproduced on the SAME owner: alias per read and drop it instead of moving it.
    let mut aliases = Vec::new();
    for read in 1..=admitted {
        aliases.push(lease.try_alias().unwrap_or_else(|_| panic!("read {read} of {admitted} must still be admitted")));
    }
    assert_eq!(lease.try_alias().err(), Some(UiDocumentLeaseError::AliasCapacity), "read {refused_at} exhausts the slot's alias credit");
    for mut alias in aliases {
        retire(&mut alias);
    }
    retire(&mut lease);
}
