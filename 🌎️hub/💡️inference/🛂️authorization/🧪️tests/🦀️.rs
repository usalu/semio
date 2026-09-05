use super::*;
use crate::directory::{
    model::{AuthSessionIssue, AuthSessionKind},
    sqlite::SqliteDirectory,
    DirectoryService,
};
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility};
use std::sync::Arc;

#[tokio::test]
async fn inference_live_author_rechecks_real_sqlite_session_scope_role_revocation_and_cancellation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🛂️inference-author-v1/🔣️.json")).unwrap();
    let base: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let directory = Arc::new(HubDirectories::Sqlite(SqliteDirectory::connect(":memory:").await.unwrap()));
        let owner = directory.create_user("owner@example.test", "Owner", None, None, None).await.unwrap();
        let author = directory.create_user("author@example.test", "Author", None, None, None).await.unwrap();
        let service = DirectoryService::new(directory.clone(), 8);
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#owner", owner.id) };
        let (events, _) = service.execute(actor.clone(), DirectoryCommand::CreateSpace { name: "Inference".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await.unwrap();
        let space_id = events[0].space_id.clone().unwrap();
        service.execute(actor.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: author.email.clone(), role: DirectorySpaceRole::Author }).await.unwrap();
        let issued = directory
            .issue_auth_session(&AuthSessionIssue {
                user_id: author.id.clone(),
                identity_provider: "inference-test".into(),
                identity_subject_digest: crate::directory::identity_subject_digest("inference-test", &author.id).unwrap(),
                ttl_secs: 60,
                device_instance_id: "inference-device".into(),
                session_kind: AuthSessionKind::DevelopmentLocal,
                correlation_id: "inference-author-law".into(),
                peer_class: "loopback-test".into(),
            })
            .await
            .unwrap();
        let mut identity: InferenceIdentityV1 = serde_json::from_value(base["identity"].clone()).unwrap();
        identity.user_id = author.id.clone();
        identity.session_id = issued.record.id.clone();
        identity.authorization_generation = issued.record.authorization_generation;
        identity.space_id = space_id.clone();
        let mut scope = DocumentScope::new(&space_id, &identity.document_id);
        let mut now = issued.record.issued_at;
        let operation = row["operation"].as_str().unwrap();
        let control = InferenceOperationControlV1::new(if operation == "deadline" { 1 } else { 10_000 }, 2).unwrap();
        match operation {
            "author" | "expiry-after-read" | "clock-regressed" => {}
            "cross-space" => scope.space_id = "fa".repeat(16),
            "cross-document" => scope.document_id = "fb".repeat(16),
            "wrong-user" => identity.user_id = owner.id.clone(),
            "wrong-session" => identity.session_id = "fc".repeat(16),
            "rotated-generation" => identity.authorization_generation += 1,
            "spectator" => {
                service.execute(actor.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: author.email.clone(), role: DirectorySpaceRole::Spectator }).await.unwrap();
            }
            "removed-member" => {
                service.execute(actor.clone(), DirectoryCommand::RemoveMember { space_id: space_id.clone(), user_id: author.id.clone() }).await.unwrap();
            }
            "revoked" => {
                directory.revoke_auth_session(&issued.record.id, "inference-law", Some(&owner.id), "inference-revoke").await.unwrap().unwrap();
            }
            "expiry-exact" => now = issued.record.expires_at,
            "expiry-past" => now = issued.record.expires_at + 1,
            "cancelled" => control.cancel(),
            "deadline" => tokio::time::sleep(std::time::Duration::from_millis(2)).await,
            "negative-clock" => now = -1,
            _ => panic!("unhandled neutral author operation"),
        }
        let head = directory.head_seq().await.unwrap();
        let clock_reads = std::cell::Cell::new(0);
        let clock = || {
            let reads = clock_reads.get();
            clock_reads.set(reads + 1);
            match (operation, reads) {
                ("expiry-after-read", 1..) => issued.record.expires_at,
                ("clock-regressed", 1..) => now - 1,
                _ => now,
            }
        };
        let result = check_live_inference_author(directory.as_ref(), &identity, &scope, clock, &control).await;
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{operation}: {result:?}");
        assert_eq!(directory.head_seq().await.unwrap(), head, "authorization is read-only: {operation}");
        if operation == "cancelled" {
            assert_eq!(result, Err(InferenceErrorV1::Cancelled));
        }
        if operation == "deadline" {
            assert_eq!(result, Err(InferenceErrorV1::Expired));
        }
    }
}
