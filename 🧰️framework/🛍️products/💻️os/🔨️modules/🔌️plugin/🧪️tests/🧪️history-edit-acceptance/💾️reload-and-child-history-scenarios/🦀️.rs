//! 💾️ Reload and child history scenarios: the archive operation, the reloaded documents and heads, the document view and its difference, the reload identity assertion and the child history scenario with its assertion. See [`super`].

use super::*;

/// 🎟️ The archive operation id a reload uses on its fresh instance.
const ACCEPTANCE_ARCHIVE_OPERATION: u64 = 1;

/// 📀️ `app`'s document saved as its recursive archive (`PluginApp::document_archive`: the parent and every owned member) and loaded
/// into a fresh registered instance through the archive load (`begin_document_archive_load`, polled to `Ready`); `Err` names why,
/// after closing the fresh instance.
pub(super) async fn acceptance_reloaded<A, M>(app: &VcsArtifactApp<A, M>, manifest: fn() -> App,cx:&mut semio_framework_job::StepContext<'_>) -> Result<VcsArtifactApp<A, M>, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let archive = PluginApp::document_archive(app).await.map_err(|fault| AcceptanceError::from(fault).context("the document does not archive".into()))?;
    let mut reloaded = semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into()), acceptance_policy(), &mut identity).await);
    reloaded.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let loaded = async {
        assert_eq!(reloaded.stores.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.into()));
        PluginApp::begin_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION, archive).map_err(|fault| AcceptanceError::from(fault).context("the archive is refused".into()))?;
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            let status = semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| PluginApp::poll_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION, &mut identity,cx).await).map_err(|fault| AcceptanceError::from(fault).context("the archive load faulted".into()))?;
            match status.state {
                protocol::DocumentArchiveLoadState::Ready => break,
                protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault => return Err((format!("the archive load ends {:?}: {}", status.state, String::from_utf8_lossy(&status.fault))).into()),
                _ if cx.should_yield() => return Err((format!("the archive load never settles ({}/{})", status.completed, status.total)).into()),
                _ => {
                    PluginApp::maintenance_step(&mut reloaded,cx,acceptance_policy().maintenance)?;
                }
            }
        }
        PluginApp::acknowledge_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION).map_err(|fault| AcceptanceError::from(fault).context("the archive load is not acknowledged".into()))?;
        reloaded.render_ctx().refresh_cache().await.map_err(|fault| AcceptanceError::from(fault).context("the reloaded history does not backfill".into()))
    }
    .await;
    match loaded {
        Ok(()) => Ok(reloaded),
        Err(reason) => {
            acceptance_close(&mut reloaded,cx).await;
            Err(reason)
        }
    }
}

/// 💽️ The head `app`'s document reloads to: saved as its recursive archive and loaded into a fresh instance through the app's own
/// store initializer ([`acceptance_reloaded`]).
pub(super) async fn acceptance_reloaded_head<A, M>(app: &VcsArtifactApp<A, M>, manifest: fn() -> App,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(DslValue, String), AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut reloaded = acceptance_reloaded(app, manifest,cx).await?;
    let head = acceptance_head(&reloaded);
    acceptance_close(&mut reloaded,cx).await;
    head
}

/// 🖼️ What a reload must preserve of a document: its head (value and document text), its owned children (slot, child id, dialect,
/// envelope bytes, sorted) and the projected render of every declared window body (a refusal compared as its fault text).
struct AcceptanceDocumentView {
    pub head: (DslValue, String),
    pub children: Vec<(String, String, String, Vec<u8>)>,
    renders: Vec<(String, Result<String, String>)>,
}

/// 📸️ The [`AcceptanceDocumentView`] of `app`.
async fn acceptance_document_view<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App,cx:&mut semio_framework_job::StepContext<'_>) -> Result<AcceptanceDocumentView, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let head = acceptance_head(app)?;
    let mut children: Vec<(String, String, String, Vec<u8>)> =
        PluginApp::child_packs(app).await.map_err(|fault| AcceptanceError::from(fault).context("the owned children do not pack".into()))?.into_iter().map(|entry| (entry.slot, entry.child_id, entry.dialect, entry.envelope_pack)).collect();
    children.sort();
    let view = ViewModel::new(Locale::En, Terminology::Native);
    let mut renders = Vec::new();
    let bodies: Vec<String> = (&manifest().definition.window_kinds).into_iter().map(|window| window.body_key.clone()).collect();
    for body in bodies {
        let render = match artifact_app_laws::render_registered_fixture_app(app, &body, &view,cx).await {
            Ok(projected) => Ok(projected),
            Err(original) => return Err(original.into()),
        };
        renders.push((body, render));
    }
    Ok(AcceptanceDocumentView { head, children, renders })
}

/// 🧐️ Where a reloaded document's view first departs from the saved one, or `None` when it reloads identically.
fn acceptance_view_difference(saved: &AcceptanceDocumentView, reloaded: &AcceptanceDocumentView) -> Option<String> {
    if saved.head != reloaded.head {
        return Some(format!("the head differs at {}", acceptance_text_difference(&saved.head.1, &reloaded.head.1)));
    }
    if saved.children != reloaded.child_composition.children {
        let names = |children: &[(String, String, String, Vec<u8>)]| children.iter().map(|(slot, child, dialect, bytes)| format!("{slot}/{child} {dialect} ({} bytes)", bytes.len())).collect::<Vec<_>>();
        return Some(format!("the owned children differ: {:?} saved, {:?} reloaded", names(&saved.children), names(&reloaded.child_composition.children)));
    }
    saved.renders.iter().zip(&reloaded.renders).find(|(left, right)| left != right).map(|((body, left), (_, right))| match (left, right) {
        (Ok(left), Ok(right)) => format!("window body {body} renders differently at {}", acceptance_text_difference(left, right)),
        (left, right) => format!("window body {body} renders {left:?} saved, {right:?} reloaded"),
    })
}

/// 🔂️ LAW (design §20.15): a document survives save → fresh load. Every document the app ships (its initial document and each of
/// `examples`, loaded through the app's own example route `setActiveExample{exampleId}`) saved as its recursive archive and loaded into a fresh instance has the same head (value and text), the same owned
/// children and the same render of every declared window body; and a representative history edit runs end to end on a reloaded
/// document (the [`assert_history_edits_end_to_end`] search, each scenario on the reloaded instance; an app without a parent-lane
/// leaf says so). Answers the summary; panics naming `plugin` and the document otherwise.
pub async fn assert_documents_reload_identically<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("original acceptance caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut documents = vec![("initial".to_string(), None)];
    documents.extend(examples.iter().map(|example| (format!("example {}", example.id()), Some(example.id().to_string()))));
    let mut reloaded_documents = 0;
    for (name, example) in documents {
        let mut saved = semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into()), acceptance_policy(), &mut identity).await);
        saved.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
        let outcome = async {
            if let Some(example) = example {
                assert_eq!(saved.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.into()));
                acceptance_verb(&mut saved, CATALOGUE_EXAMPLE_ACTION_ID, vec![("exampleId", DslValue::String(example))],cx).await.map_err(|refusal|refusal.context("the app's own example route does not load it".into()))?;
                semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut saved, acceptance_meta().instance_id, acceptance_policy(), &mut identity,cx).await)?;
            }
            saved.render_ctx().refresh_cache().await.map_err(|fault| AcceptanceError::from(fault).context("the history does not backfill".into()))?;
            let before = acceptance_document_view(&mut saved, manifest,cx).await?;
            let mut reloaded = acceptance_reloaded(&saved, manifest,cx).await?;
            let after = acceptance_document_view(&mut reloaded, manifest,cx).await;
            acceptance_close(&mut reloaded,cx).await;
            Ok::<_, AcceptanceError>(acceptance_view_difference(&before, &after?))
        }
        .await;
        acceptance_close(&mut saved,cx).await;
        match outcome {
            Ok(None) => reloaded_documents += 1,
            Ok(Some(difference)) => panic!("{plugin}: the {name} document does not reload identically: {difference}"),
            Err(reason) => panic!("{plugin}: the {name} document does not survive save and load: {reason}"),
        }
    }
    let edit = match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, true,cx).await {
        AcceptanceSearch::Passed(summary) => summary,
        AcceptanceSearch::Failed(failure) => panic!("{failure} (on a reloaded document)"),
        AcceptanceSearch::Exhausted(report) if report.contains("none of the 0 committed and 0 derived") => format!("{plugin}: no parent-lane leaf to edit on a reloaded document"),
        AcceptanceSearch::Exhausted(report) => panic!("{report} (on a reloaded document)"),
    };
    format!("{reloaded_documents} document(s) reload identically; {edit}")

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original acceptance caller paid retirement");result
}

/// 🧷️ The first applied, editable, unedited child-lane mutation of `app`'s history (`HistoryMutationEntry.store` names its owned
/// member, `<slot>/<childId>`): its id, its store, and its label in every locale (refused when a locale resolves empty).
async fn acceptance_child_row<A, M>(app: &mut VcsArtifactApp<A, M>) -> Result<Option<(String, String, String)>, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let patch = app.render_ctx().history_patch(true).await.map_err(|fault| AcceptanceError::from(fault).context("the history does not project".into()))?;
    let Some(row) = patch.upserts.iter().filter(|entry| entry.applied).flat_map(|entry| entry.mutations.iter()).find(|row| row.store.is_some() && row.editable && !row.superseded && !row.withdrawn && !row.pending) else {
        return Ok(None);
    };
    let (en, de) = (row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string());
    if en.trim().is_empty() || de.trim().is_empty() {
        return Err((format!("the child-lane row {} is labelled {en:?} / {de:?}, not in every locale", row.mutation_id)).into());
    }
    Ok(Some((row.mutation_id.clone(), row.store.clone().unwrap_or_default(), format!("{en} / {de}"))))
}

/// 🌰️ A registered instance on the app's initial document after the plugin's `seed` gestures (`(action id, JSON object args)`, each
/// dispatched as the acceptance actor and published to completion) — the gestures that land a child-lane edit in an owned member.
async fn acceptance_child_seeded<A, M>(manifest: fn() -> App, seed: &[(&str, &str)],cx:&mut semio_framework_job::StepContext<'_>) -> Result<VcsArtifactApp<A, M>, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into()), acceptance_policy(), &mut identity).await);
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.into()));
        for (action, args) in seed {
            let args = semio_framework_pack_json::parse(args, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the seed args of {action} are not JSON: {error:?}"))?;
            let DslValue::Object(args) = semio_framework_pack_json::to_dsl_value(&args) else {
                return Err((format!("the seed args of {action} are not a JSON object")).into());
            };
            acceptance_verb(&mut app, action, args.iter().map(|(key, value)| (key.as_str(), value.clone())).collect(),cx).await?;
            semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, acceptance_meta().instance_id, acceptance_policy(), &mut identity,cx).await).map_err(|fault|AcceptanceError::from(fault).context(format!("the seed gesture {action} does not publish")))?;
        }
        app.render_ctx().refresh_cache().await.map_err(|fault| AcceptanceError::from(fault).context("the history does not backfill".into()))
    }
    .await;
    match seeded {
        Ok(()) => Ok(app),
        Err(reason) => {
            acceptance_close(&mut app,cx).await;
            Err(reason)
        }
    }
}

/// 🧵️ The head snapshot pack of every owned member of `app`, sorted by slot and child id.
async fn acceptance_child_heads<A, M>(app: &VcsArtifactApp<A, M>) -> Result<Vec<(String, String, Vec<u8>)>, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    Ok(PluginApp::child_head_packs(app).await.map_err(|fault| AcceptanceError::from(fault).context("the owned children do not pack their heads".into()))?.into_iter().map(|entry| (entry.slot, entry.child_id, entry.head_pack)).collect())
}

/// 🔄️ Where `app`'s document, saved as its recursive archive and loaded into a fresh instance through the app's own store initializer
/// ([`acceptance_reloaded`]), departs from the live one — its head, owned children or window renders ([`acceptance_view_difference`]), or its
/// member head packs (every member folds its own log, edits included, from scratch) — else `None`; the fresh instance is closed on every path.
async fn acceptance_child_reload_difference<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<String>, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let saved = acceptance_document_view(app, manifest,cx).await?;
    let heads = acceptance_child_heads(app).await?;
    let mut reloaded = acceptance_reloaded(app, manifest,cx).await?;
    let fresh = acceptance_document_view(&mut reloaded, manifest,cx).await;
    let fresh_heads = acceptance_child_heads(&reloaded).await;
    acceptance_close(&mut reloaded,cx).await;
    if let Some(difference) = acceptance_view_difference(&saved, &fresh?) {
        return Ok(Some(difference));
    }
    Ok((fresh_heads? != heads).then(|| "the member head packs differ from the live members'".to_string()))
}

/// 🎞️ The child-lane scenario behind [`assert_child_history_edits_end_to_end`]; every instance is closed on every path.
async fn acceptance_child_scenario<A, M>(manifest: fn() -> App, seed: &[(&str, &str)],cx:&mut semio_framework_job::StepContext<'_>) -> Result<String, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut overwrite = acceptance_child_seeded::<A, M>(manifest, seed,cx).await.map_err(|reason|reason.context("the seed does not land".into()))?;
    let outcome = async {
        if let Some(difference) = acceptance_child_reload_difference(&mut overwrite, manifest,cx).await? {
            return Err((format!("the seeded document with child-lane history does not reload identically: {difference}")).into());
        }
        let (target, store, label) = acceptance_child_row(&mut overwrite).await?.ok_or("the seed lands no editable child-lane mutation in the history")?;
        let before = acceptance_child_heads(&overwrite).await?;
        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None, None, None,cx).await {
            Ok(change) => change,
            Err(AcceptanceVerdict::Fail(error)) => return Err(error.context(format!("the overwrite of {target} in {store}"))),
            Err(verdict) => return Err((format!("the overwrite of {target} in {store}: {}", verdict.text())).into()),
        };
        let heads = acceptance_child_heads(&overwrite).await?;
        if heads == before {
            return Err((format!("the overwrite {} = {:?} of {target} leaves every member head unchanged", change.0, change.1)).into());
        }
        if let Some(difference) = acceptance_child_reload_difference(&mut overwrite, manifest,cx).await? {
            return Err((format!("the overwritten document does not fold freshly to the same state after save and load: {difference}")).into());
        }
        Ok::<_, AcceptanceError>((change, store, label, heads))
    }
    .await;
    acceptance_close(&mut overwrite,cx).await;
    let (change, store, label, heads) = outcome?;
    let mut seeded = acceptance_child_seeded::<A, M>(manifest, seed,cx).await.map_err(|reason|reason.context("the alternative instance does not seed".into()))?;
    let reloaded = acceptance_reloaded(&seeded, manifest,cx).await;
    acceptance_close(&mut seeded,cx).await;
    let mut branched = reloaded.map_err(|reason|reason.context("the seeded document does not reload for the alternative".into()))?;
    let alternative = async {
        let (target, _, _) = acceptance_child_row(&mut branched).await?.ok_or("the reloaded seeded document lists no editable child-lane mutation")?;
        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE), None, None,cx).await {
            Ok(_) => {}
            Err(AcceptanceVerdict::Fail(error)) => return Err(error.context("the change the overwrite accepted fails as a new alternative on the reloaded document".into())),
            Err(verdict) => return Err((format!("the change the overwrite accepted fails as a new alternative on the reloaded document: {}", verdict.text())).into()),
        }
        if acceptance_child_heads(&branched).await? != heads {
            return Err("the new alternative's member heads differ from the overwrite's".into());
        }
        if let Some(difference) = acceptance_child_reload_difference(&mut branched, manifest,cx).await? {
            return Err((format!("the new alternative does not fold freshly to the same state after save and load: {difference}")).into());
        }
        Ok::<(), AcceptanceError>(())
    }
    .await;
    acceptance_close(&mut branched,cx).await;
    alternative?;
    Ok(format!("{store}: {} = {:?}, row \"{label}\", seeded, overwritten and alternative documents reload identically", change.0, change.1))
}

/// 👶️ LAW (design §12, §20.15 — D20, audit F22): a child-lane mutation, the edit the plugin's `seed` gestures land in an owned member
/// store, is edited in history end to end through the generic member path (`historyEditBegin{mutationId, store}`). The seeded document
/// (carrying child-lane history) saved and freshly loaded through the app's own store initializer has the same head, owned children,
/// renders and member heads; the overwrite changes the member's head and the overwritten document reloads identically; the same change
/// committed as a new alternative on a second instance — whose session runs on the seeded document after a save → fresh load — reaches the
/// same member heads and reloads identically; the row is labelled in every locale. Answers the summary; panics naming `plugin` otherwise.
pub async fn assert_child_history_edits_end_to_end<A, M>(plugin: &str, manifest: fn() -> App, seed: &[(&str, &str)]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("original acceptance caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    match acceptance_child_scenario::<A, M>(manifest, seed,cx).await {
        Ok(summary) => format!("{plugin}: child lane {summary}"),
        Err(reason) => panic!("{plugin}: the child-lane history edit breaks the generic mechanism: {reason}"),
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original acceptance caller paid retirement");result
}
