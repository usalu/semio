//! 🪧️ The pane smoke law: one bundled example document loads into a mounted app, one typed command applies, undo restores the loaded snapshot, redo reapplies it and the app closes through its exact retained close so the store drop witness holds. See [`super`].

use super::*;

/// ⚖️ LAW: `example_dsl` loads through the document archive door (the genesis op log of the booted app rides along), `command` changes `probe`, `undo` restores the loaded probe, `redo` returns the changed probe, and `close_registered_fixture_app` retires every owner. Returns the probe values `(loaded, applied)` so a pane test can pin its own expectations.
pub async fn assert_pane_smoke<A, M, P>(app: &mut VcsArtifactApp<A, M>, example_dsl: &str, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> (P, P)
where
    A: ArtifactApp,
    A::Command: semio_framework_value::ToValue,
    A::Snapshot: pack::snapshot::ArtifactDsl,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
    P: PartialEq + std::fmt::Debug,
{
    let receiver = meta("local").instance_id;
    let booted = app.document_text().await.expect("pane smoke: booted document text");
    load_document_text(app, &store::os_store::text_format::ArtifactTextFiles { dsl: example_dsl.into(), ops: booted.ops }, identity, cx).await.expect("pane smoke: the bundled example loads");
    let loaded = probe(app);
    dispatch_original_fixture_command(app, &command, &meta("local"), mounted_policy, identity, cx).await.expect("pane smoke: apply the typed command");
    settle_registered_typed_operation(app, receiver, mounted_policy, identity, cx).await.expect("pane smoke: typed command publication");
    let applied = probe(app);
    assert_ne!(applied, loaded, "pane smoke: the typed command must change the loaded document");
    settle_history_verb(app, "undo", receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), loaded, "pane smoke: undo must restore the loaded snapshot");
    settle_history_verb(app, "redo", receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), applied, "pane smoke: redo must reapply the typed command");
    settle_history_verb(app, "undo", receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), loaded, "pane smoke: a second undo must restore the loaded snapshot again");
    close_registered_fixture_app(app, mounted_policy, cx).expect("pane smoke: the exact retained close retires every owner");
    (loaded, applied)
}

/// ‼️ LAW: like [`assert_pane_smoke`], but the example arrives the way the shell asks for it — `example` is the app's own load-example command, whose published `LoadDocument` effect crosses the document archive door — so the pane's example catalogue, not a text constant, is what the law exercises.
pub async fn assert_pane_smoke_by_example<A, M, P>(app: &mut VcsArtifactApp<A, M>, example: A::Command, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> (P, P)
where
    A: ArtifactApp,
    A::Command: semio_framework_value::ToValue,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
    P: PartialEq + std::fmt::Debug,
{
    let receiver = meta("local").instance_id;
    dispatch_original_fixture_command(app, &example, &meta("local"), mounted_policy, identity, cx).await.expect("pane smoke: dispatch the load-example command");
    let published = settle_registered_typed_operation(app, receiver, mounted_policy, identity, cx).await.expect("pane smoke: load-example publication");
    let (pack, spr) = published.effects.iter().find_map(|effect| match effect {
        semio_framework::kernel::Effect::LoadDocument { pack, spr } => Some((pack.clone(), spr.clone())),
        _ => None,
    }).expect("pane smoke: the load-example command publishes a LoadDocument effect");
    load_document(app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, identity, cx).await.expect("pane smoke: the published example crosses the archive door");
    let loaded = probe(app);
    dispatch_original_fixture_command(app, &command, &meta("local"), mounted_policy, identity, cx).await.expect("pane smoke: apply the typed command");
    settle_registered_typed_operation(app, receiver, mounted_policy, identity, cx).await.expect("pane smoke: typed command publication");
    let applied = probe(app);
    assert_ne!(applied, loaded, "pane smoke: the typed command must change the loaded document");
    settle_history_verb(app, "undo", receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), loaded, "pane smoke: undo must restore the loaded snapshot");
    close_registered_fixture_app(app, mounted_policy, cx).expect("pane smoke: the exact retained close retires every owner");
    (loaded, applied)
}
