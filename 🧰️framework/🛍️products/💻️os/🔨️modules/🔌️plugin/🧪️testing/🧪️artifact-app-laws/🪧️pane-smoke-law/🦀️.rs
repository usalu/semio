//! 🪧️ The pane smoke law: one bundled example document loads into a mounted app through the app's own load-example command, one typed command applies, undo restores the loaded snapshot, redo reapplies it, a second undo restores it again and the app closes through its exact retained close so the store drop witness holds — all under one explicit language. See [`super`].

use super::*;

/// 🌐️ The languages every pane runs the whole cycle under, in order; there is no default language.
pub const PANE_SMOKE_LOCALES: [semio_ui_locale::Locale; 2] = [semio_ui_locale::Locale::En, semio_ui_locale::Locale::De];

/// 🌐️ One language of the pane smoke: the window `view` every dispatch and render carries, the pane's main `body_key`, and the `catalog` strings of that language (never literals of another) the rendered body must contain.
pub struct PaneSmokeLanguage<'a> {
    pub view: semio_framework::ViewModel,
    pub body_key: &'a str,
    pub catalog: Vec<String>,
}

/// 🔎️ The body renders under the language's view and carries every catalog string of that language.
async fn assert_body_in_catalog<A, M>(app: &mut VcsArtifactApp<A, M>, language: &PaneSmokeLanguage<'_>, stage: &str, cx: &mut semio_framework_job::StepContext<'_>)
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let rendered = render_registered_fixture_app(app, language.body_key, &language.view, cx).await.unwrap_or_else(|fault| panic!("pane smoke: render {} {stage} in {:?}: {fault:?}", language.body_key, language.view.locale));
    for needle in &language.catalog {
        assert!(rendered.contains(needle.as_str()), "pane smoke: {stage} body lacks the {:?} catalog string {needle:?}: {rendered}", language.view.locale);
    }
}

/// ‼️ LAW: `example` is the app's own load-example command, whose published `LoadDocument` effect crosses the document archive door; `command` changes `probe`; `undo` restores the loaded probe, `redo` returns the changed probe, a second `undo` restores the loaded probe again and `close_registered_fixture_app` retires every owner. Every dispatch carries `language.view`, and the body rendered after load and after redo contains the catalog strings of that language. Run it once per [`PANE_SMOKE_LOCALES`] entry on a fresh app each. Returns the probe values `(loaded, applied)` so a pane test can pin its own expectations.
pub async fn assert_pane_smoke_by_example<A, M, P>(app: &mut VcsArtifactApp<A, M>, language: &PaneSmokeLanguage<'_>, example: A::Command, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> (P, P)
where
    A: ArtifactApp + Default,
    A::Command: semio_framework_value::ToValue,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
    P: PartialEq + std::fmt::Debug,
{
    assert_heap_witness_installed();
    let _lifetime = arm_host_lifetime(cx);
    let action_meta = ActionMeta { view_state: Some(std::sync::Arc::new(language.view.clone())), ..meta("local") };
    let receiver = action_meta.instance_id;
    dispatch_original_fixture_command(app, &example, &action_meta, mounted_policy, identity, cx).await.expect("pane smoke: dispatch the load-example command");
    let published = settle_registered_typed_operation(app, receiver, mounted_policy, identity, cx).await.expect("pane smoke: load-example publication");
    let (pack, spr) = published.effects.iter().find_map(|effect| match effect {
        semio_framework::kernel::Effect::LoadDocument { pack, spr } => Some((pack.clone(), spr.clone())),
        _ => None,
    }).expect("pane smoke: the load-example command publishes a LoadDocument effect");
    load_document(app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, identity, cx).await.expect("pane smoke: the published example crosses the archive door");
    let loaded = probe(app);
    assert_body_in_catalog(app, language, "loaded", cx).await;
    dispatch_original_fixture_command(app, &command, &action_meta, mounted_policy, identity, cx).await.expect("pane smoke: apply the typed command");
    settle_registered_typed_operation(app, receiver, mounted_policy, identity, cx).await.expect("pane smoke: typed command publication");
    let applied = probe(app);
    assert_ne!(applied, loaded, "pane smoke: the typed command must change the loaded document");
    settle_history_verb_as(app, "undo", &action_meta, receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), loaded, "pane smoke: undo must restore the loaded snapshot");
    settle_history_verb_as(app, "redo", &action_meta, receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), applied, "pane smoke: redo must reapply the typed command");
    assert_body_in_catalog(app, language, "redone", cx).await;
    settle_history_verb_as(app, "undo", &action_meta, receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), loaded, "pane smoke: a second undo must restore the loaded snapshot again");
    close_registered_fixture_app(app, mounted_policy, cx).expect("pane smoke: the exact retained close retires every owner");
    (loaded, applied)
}
