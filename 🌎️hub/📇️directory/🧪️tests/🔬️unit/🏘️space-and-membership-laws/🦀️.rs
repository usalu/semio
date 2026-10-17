/// 🏘️ Laws of document descriptors, space authorship and archive, membership and invites.

#[tokio::test]
async fn document_descriptor_is_immutable_space_scoped_and_survives_restart() {
    let mut root = std::env::temp_dir();
    root.push(format!("semio-document-descriptor-{}", directory::os_identity::time_ordered_id()));
    std::fs::create_dir_all(&root).expect("create test directory");
    let path = root.join("directory.sqlite3");
    let path_text = path.to_str().expect("utf8 test path").to_string();

    let persisted = descriptor("default", "shared-document");
    let mut zero_identity = persisted.clone();
    zero_identity.pack_schema_hash = "0".repeat(64);
    assert!(matches!(validate_document_descriptor(&zero_identity), Err(DirectoryError::Conflict(_))));
    {
        let directory = SqliteDirectory::connect(&path_text).await.expect("connect");
        directory.seed().await.expect("seed");
        let directories = Arc::new(HubDirectories::from(directory));
        let service = DirectoryService::new(directories.clone(), 16);
        let (events, _) = service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: Box::new(persisted.clone()) }).await.expect("announce");
        assert!(matches!(&events[0].body, DirectoryEventBody::DocumentAnnounced { descriptor } if descriptor == &persisted));

        let mut conflict = persisted.clone();
        conflict.pack_schema_hash = "44".repeat(32);
        assert!(matches!(service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: Box::new(conflict) }).await, Err(DirectoryError::Conflict(_))));

        let other = descriptor("other-space", "shared-document");
        let actor = user_actor("seed");
        let (created, _) = service.execute(actor.clone(), DirectoryCommand::CreateSpace { name: "Other".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await.expect("create other space");
        let other_space = created[0].space_id.clone().expect("space id");
        let mut other = other;
        other.space_id = other_space.clone();
        service.execute(actor, DirectoryCommand::AnnounceDocument { descriptor: Box::new(other.clone()) }).await.expect("announce same document id in other space");
        assert_eq!(directories.list_document_descriptors(&other_space).await.expect("other descriptors"), vec![other]);
    }

    let reopened = SqliteDirectory::connect(&path_text).await.expect("reopen");
    assert_eq!(reopened.get_document_descriptor(&DocumentScope::new("default", "shared-document")).await.expect("read after restart"), Some(persisted.clone()));
    let before = reopened.list_document_descriptors("default").await.expect("before rebuild");
    reopened.rebuild_projections().await.expect("rebuild");
    assert_eq!(reopened.list_document_descriptors("default").await.expect("after rebuild"), before);

    drop(reopened);
    for target in [&path, &root.join("directory.sqlite3-wal"), &root.join("directory.sqlite3-shm")] {
        if target.exists() {
            std::fs::remove_file(target).expect("remove exact sqlite test file");
        }
    }
    std::fs::remove_dir(&root).expect("remove empty test directory");
}

// 🔬️ Decider law: an atelier rejects a second, distinct author (re-upserting the sole existing
// author is not exercised here — that is the sqlite backend's own membership round-trip test).
#[tokio::test]
async fn atelier_rejects_a_second_distinct_author() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Atelier).await;
    let err = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "other@example.com".into(), role: DirectorySpaceRole::Author }).await.unwrap_err();
    assert!(matches!(err, DirectoryError::Conflict(_)));
}

// 🔬️ Decider law: `archive-space` first demotes every current author to spectator, then
// archives — nobody is left an author of a frozen space.
#[tokio::test]
async fn archive_space_demotes_every_author_to_spectator() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "second@example.com".into(), role: DirectorySpaceRole::Author }).await.expect("second author");
    service.execute(owner.clone(), DirectoryCommand::ArchiveSpace { space_id: space_id.clone() }).await.expect("archive-space");
    let members = dir.list_members(&space_id).await.expect("list members");
    assert_eq!(members.len(), 2);
    assert!(members.iter().all(|(_, role)| *role == SpaceRole::Spectator));
    assert_eq!(dir.get_space(&space_id).await.expect("get space").expect("space exists").kind, "archive");
}

// 🔬️ Decider law: the owner's membership can never be removed via `remove-member`.
#[tokio::test]
async fn owner_membership_can_never_be_removed() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    let err = service.execute(owner, DirectoryCommand::RemoveMember { space_id, user_id: "u-owner".into() }).await.unwrap_err();
    assert!(matches!(err, DirectoryError::Conflict(_)));
}

// 🔬️ Decider law: `delete-space` revokes every membership (owner included) before it deletes the space, all under one
// decision — the member-only readers learn the deletion through their own `member.removed`.
#[tokio::test]
async fn delete_space_revokes_every_membership_before_the_space() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "member@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
    let (events, _) = service.execute(owner, DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete-space");
    let (last, removals) = events.split_last().expect("delete-space events");
    assert!(matches!(&last.body, DirectoryEventBody::SpaceDeleted { space_id: deleted } if *deleted == space_id), "the space is deleted last");
    let mut removed: Vec<&str> = removals.iter().map(|event| match &event.body {
        DirectoryEventBody::MemberRemoved { space_id: from, user_id } if *from == space_id && event.user_id.as_deref() == Some(user_id.as_str()) => user_id.as_str(),
        other => panic!("only member removals precede the deletion: {other:?}"),
    }).collect();
    removed.sort_unstable();
    assert_eq!(removed.len(), 2, "the owner and the added member are both removed: {removed:?}");
    assert!(removed.contains(&"u-owner"));
}

// 🔬️ Decider law: any command naming a deleted (or otherwise missing) space is `NotFound`.
#[tokio::test]
async fn command_naming_a_deleted_space_is_not_found() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete-space");
    let err = service.execute(owner, DirectoryCommand::RenameSpace { space_id, name: "Nope".into() }).await.unwrap_err();
    assert!(matches!(err, DirectoryError::NotFound(_)));
}

// 🔬️ Decider law: `upsert-member` with an email that has no `UserRecord` yet emits
// `user.created` before `member.upserted`, both under one decision.
#[tokio::test]
async fn upsert_member_with_unknown_email_creates_the_user_first() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    let (events, _) = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "new@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0].body, DirectoryEventBody::UserCreated { .. }));
    assert!(matches!(events[1].body, DirectoryEventBody::MemberUpserted { .. }));
}

// 🔬️ Invite create -> redeem -> revoke round-trip: redemption grants membership and emits
// `invite.redeemed`; a revoked invite still round-trips its `revoked_at`.
#[tokio::test]
async fn invite_create_redeem_revoke_round_trip() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;

    let (_, result) = service.execute(owner.clone(), DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3600 }).await.expect("create-invite");
    let token = result.expect("command result").invite_token.expect("invite token");
    let capability = InviteCapability::parse(&token).expect("typed invite capability");
    let invited = dir.create_user("invited@example.com", "Invited", None, None, None).await.expect("create invited user");

    let redeemed = service.redeem_invite(user_actor(&invited.id), &capability, &invited.id).await.expect("redeem");
    assert!(matches!(redeemed.event().body, DirectoryEventBody::InviteRedeemed { .. }));
    let members = dir.list_members(&space_id).await.expect("list members");
    assert!(members.iter().any(|(user, role)| user.email == "invited@example.com" && *role == SpaceRole::Spectator));

    let invites = dir.list_invites(&space_id).await.expect("list invites");
    assert_eq!(invites.len(), 1);
    assert_eq!(invites[0].accepted_at, Some(redeemed.event().recorded_at_ms));
    assert_eq!(invites[0].accepted_event_id.as_deref(), Some(redeemed.event().id.as_str()));
    let retried = service.redeem_invite(user_actor(&invited.id), &capability, &invited.id).await.expect("idempotent same-user retry");
    assert!(matches!(redeemed, InviteRedemptionCommit::NewlyCommitted { .. }));
    assert!(matches!(retried, InviteRedemptionCommit::AlreadyCommitted { .. }));
    assert_eq!(retried.event(), redeemed.event());
    assert!(matches!(dir.revoke_invite(&space_id, &invites[0].id, "test-revoke", "invite-round-trip").await, Err(DirectoryError::Conflict(message)) if message == "invite already accepted"));
}
