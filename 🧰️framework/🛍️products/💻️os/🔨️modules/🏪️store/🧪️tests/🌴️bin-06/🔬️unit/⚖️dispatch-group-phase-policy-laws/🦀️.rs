/// ⚖️ Dispatch group phase policy laws: the original envelope cancel retirement, the group tool transaction stamping, the phase one rejection and acceptance under the normal, laissez faire and vigilant policies and the group receipt messages.

fn retire_original_envelope_cancel(cancel:semio_framework_job::CancelToken,source:RetainedCloneGrant){
    let mut owner=semio_framework_async::CancelTokenRetirement::from_token(cancel);
    for _ in 0..256{if owner.terminal_is_empty(){break}let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(source).unwrap());assert!(step.progress().fits(source));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));}
    assert!(owner.terminal_is_empty());let(_,born,freed)=semio_framework_trace_memory_testing::observe_backing(||drop(owner));assert_eq!((born,freed),(0,0));
}


/// ⚖️ LAW (design §12 composed children): a group carrying a committed tool transaction lands the SAME `TransactionRef`
/// on every operation of every member it touches — the parent and its owned child — because the ref rides each member's
/// own `Apply` command, so the ledger, the revision and the outbound envelopes all see it at apply time.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_stamps_one_tool_transaction_on_parent_and_owned_child() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-transaction-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-transaction-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");
    let parent_ops = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_ops = vec![DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode child op"), DemoMutation::SetN(SetN { n: 3 }).encode_op().expect("encode child op")];
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &child_ops, op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];
    let transaction = semio_framework_os_spr::TransactionRef { id: "tx-0123456789abcdef".into(), tool: "s.stdio.demoparent@1/*#editor#drag".into() };

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta { transaction: Some(transaction.clone()), ..GroupMeta::default() }, &mut identity).await.expect("owned dispatch");

    assert_eq!(receipt.member_edits.len(), 2, "one edit per touched member");
    let parent_edit = parent_store.envelope().vcs.edits.last().expect("parent edit");
    let child_edit = child_store.envelope().vcs.edits.last().expect("child edit");
    assert_eq!(child_edit.mutation_meta.len(), 2, "both child ops land in one edit");
    for (member, edit) in [("parent", parent_edit), ("child", child_edit)] {
        assert!(edit.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&transaction)), "every {member} operation carries the group's transaction");
        assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(receipt.invocation_id.as_str())), "every {member} operation keeps the group identity");
    }
    let plain = coordinator.dispatch_group(
            &parent_ref,
            &mut parent_store,
            &mut [(&mut child_store, ChildDispatch { child: child_ref.clone(), ops: &[DemoMutation::SetN(SetN { n: 4 }).encode_op().expect("encode")], op_schema: &SchemaId("demo/v1".into()), labels: &[] })],
            Vec::new(),
            Vec::new(),
            GroupMeta::default(),
        &mut identity)
        .await
        .expect("plain dispatch");
    assert_eq!(plain.member_edits.len(), 1);
    assert!(child_store.envelope().vcs.edits.last().expect("child edit").mutation_meta.iter().all(|meta| meta.transaction.is_none()), "a group without a transaction stamps none");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


//#region 🔖️PhasePolicyTests
/// 🧪️ Normal (the default policy) rejects an Error-level message exactly like it already
/// rejects Fatal (`dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing` above)
/// — the SAME all-or-nothing law, now driven by `reject_if_policy_rejects` off the UNIONED
/// `preview_wire` messages rather than an immediate per-member `Err`. Exercises lane 1-A's REAL
/// `ArtifactStore::set_merge_policy` (§C6, landed) end to end — no test-only policy fixture
/// needed now that it exists.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_rejects_under_normal_when_a_member_yields_an_error_and_nothing_applies() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-normal".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-normal".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    assert_eq!(parent_store.merge_policy().await, semio_framework_os_spr::MergePolicy::Normal, "Normal is also the default, but set it explicitly so this test does not rely on that");
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected Normal policy to reject an Error-level message, but the dispatch succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, semio_framework_os_spr::MergePolicy::Normal, "the rejection must name the policy that actually rejected it");
            assert!(messages.iter().any(|message| message.level == semio_framework_diagnostic::Severity::Error), "the rejection must carry the Error message that triggered it");
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a policy-rejected group dispatch");
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ The SAME Error-level scenario `dispatch_group_phase1_rejects_under_normal_when_a_
/// member_yields_an_error_and_nothing_applies` rejects is accepted end-to-end under
/// `LaissezFaire` (only `Fatal` is rejected) — both members get a real edit, and the child's own
/// diff stays empty (§C2 LAW 2: an `Error` message's diff carries no change for the target) even
/// though the group as a whole succeeded.
///
/// 🎯️ Sets `LaissezFaire` on BOTH members, not just the parent: `reject_if_policy_rejects` is
/// only the coordinator's GROUP-level gate (checked once, against the parent's own policy,
/// before Phase 2 starts). Phase 2 still dispatches through each member's REAL `dispatch_wire`,
/// which (lane 1-A's now-landed C6 `ArtifactStore::dispatch`) independently enforces THAT
/// member's own `merge_policy()` — a child left at the Normal default would still reject its own
/// Error-level op right here, even though the coordinator's gate already accepted the group.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-lf".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-lf".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity).await.expect("LaissezFaire accepts an Error-level message");

    assert_eq!(receipt.member_edits.len(), 2, "both parent and child got a real edit despite the child's Error message");
    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(1), "the parent's own clean op still applied");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0), "the child's Error-level op's diff carries no change for its target (LAW 2), even though the group was accepted");
    assert!(receipt.messages.iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == semio_framework_diagnostic::Severity::Error), "the child's Error message must still be reported, even on acceptance");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `Vigilant` rejects a plain `Warning` — the strictest of the three policies, and the
/// one level `Normal` (the OTHER two policy tests above/below use) would have accepted.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_rejects_under_vigilant_on_a_members_warning() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-vigilant".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-vigilant".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::Vigilant);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let child_op = SeverityMutation::SetWarningN(SetWarningN { n: 5 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected Vigilant policy to reject a Warning-level message, but the dispatch succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, semio_framework_os_spr::MergePolicy::Vigilant, "the rejection must name the policy that actually rejected it");
            assert!(messages.iter().any(|message| message.level == semio_framework_diagnostic::Severity::Warning), "the rejection must carry the Warning message that triggered it");
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `GroupReceipt.messages` carries the FULL union (both parent's own and the child's),
/// each `target` prefixed with the ORIGINATING member's own `semio_framework_artifact_reference::ArtifactRef::
/// to_uri()` — the discipline that lets a caller with several members in flight tell messages
/// apart. Parent-first ordering matches phase 1's own collection order.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn group_receipt_messages_contains_the_union_with_member_path_prefixed_targets() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-union".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-union".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetWarningN(SetWarningN { n: 5 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity).await.expect("LaissezFaire accepts a Warning-level message");

    assert_eq!(receipt.messages.len(), 2, "one Warning from the parent, one from the child");
    assert_eq!(receipt.messages[0].level, semio_framework_diagnostic::Severity::Warning);
    assert_eq!(receipt.messages[0].code.0, "mutation.clamped");
    assert_eq!(receipt.messages[0].target, vec![parent_ref.to_uri()], "the parent's own message is prefixed with the parent's own path, collected before any child's");
    assert_eq!(receipt.messages[1].level, semio_framework_diagnostic::Severity::Warning);
    assert_eq!(receipt.messages[1].code.0, "mutation.clamped");
    assert_eq!(receipt.messages[1].target, vec![child_ref.to_uri()], "the child's message is prefixed with the CHILD's own path, not the parent's");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#endregion 🔖️PhasePolicyTests
