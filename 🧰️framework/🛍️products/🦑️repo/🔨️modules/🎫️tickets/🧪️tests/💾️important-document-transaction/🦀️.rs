//! 🦀️ Rust side of the important-document transaction case. Every wrong bundle and the injected
//! write failure are produced through the in-memory store, so nothing touches a real filesystem.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tickets::{
        ManagementProviders, NullManagementProvider,
        ensure_important_document, inspect_important_document, join_path, rollback_important_creation, FixedClock, MemoryTicketStore, RecordingEventSink, TicketCloseRequest, TicketId, TicketLayout, TicketOpenRequest, TicketReopenRequest,
        TicketService, TicketStore, TransactionJournal,
    };
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "shared://💾️important-document-transaction/💾️cases.json";

    fn strings(values: Vec<String>) -> Json {
        Json::Array(values.into_iter().map(Json::String).collect())
    }

    fn open_request() -> TicketOpenRequest {
        TicketOpenRequest {
            emoji: "💾️".to_string(),
            title: "Important Document Transaction".to_string(),
            prompt: "Prove the transaction".to_string(),
            client: "claude-code".to_string(),
            goal: "🎯️aioptimizedrepo".to_string(),
            no_issue: true,
            no_management: true,
            ..Default::default()
        }
    }

    fn close_request(id: &str) -> TicketCloseRequest {
        TicketCloseRequest { id: id.to_string(), summary: "done".to_string(), files: vec!["a/b.rs".to_string()], no_management: true, bulk: false }
    }

    fn reopen_request(id: &str) -> TicketReopenRequest {
        TicketReopenRequest { id: id.to_string(), prompt: "again".to_string(), client: "claude-code".to_string(), no_management: true, ..Default::default() }
    }

    struct World {
        store: MemoryTicketStore,
        tracker: ManagementProviders,
        clock: FixedClock,
        events: RecordingEventSink,
        layout: TicketLayout,
    }

    fn world(vectors: &Json) -> World {
        World {
            store: MemoryTicketStore::new(),
            tracker: ManagementProviders::Null(NullManagementProvider),
            clock: FixedClock::new(26, 9, 6, "2026-09-06 12:00:00"),
            events: RecordingEventSink::new(),
            layout: TicketLayout::new(vectors.str("repoMetaDir")),
        }
    }

    fn service(world: &World) -> TicketService<'_, MemoryTicketStore, ManagementProviders, FixedClock, RecordingEventSink> {
        TicketService::new(world.layout.clone(), &world.store, &world.tracker, &world.clock, &world.events)
    }

    /// ⚠️ A wrong bundle is refused before anything is removed.
    pub fn a_wrong_bundle_is_refused_before_anything_is_removed(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let mut projected: Vec<(String, Json)> = Vec::new();
        for mode in vectors.array("modes") {
            let mode = match mode {
                Json::String(text) => text,
                other => other.to_string(),
            };
            let world = world(&vectors);
            let opened = service(&world).open(&open_request()).map_err(|error| error.message)?;
            let id = TicketId::parse(&opened.id).map_err(|error| error.message)?;
            let important = world.layout.important_path(&id);
            match mode.as_str() {
                "missing" => world.store.remove_file(&important).map_err(|error| error.message)?,
                "not-empty" => world.store.write(&important, "not empty at all").map_err(|error| error.message)?,
                "not-a-file" => {
                    world.store.remove_file(&important).map_err(|error| error.message)?;
                    world.store.seed_symlink(&important);
                }
                _ => world.store.seed_file(&join_path(&world.layout.important_dir(&id), "🪤️stray.md"), "stray"),
            }
            let outcome = match service(&world).close(&close_request(&opened.id)) {
                Ok(_) => "accepted".to_string(),
                Err(error) => format!("{}:{}", error.class, error.message),
            };
            projected.push((mode, Json::Object(vec![("refusal".to_string(), Json::String(outcome)), ("documentStillThere".to_string(), Json::String(if world.store.exists(&world.layout.document_path(&id)) { "yes" } else { "no" }.to_string()))])));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ↩️ A failed save rolls the creation back.
    pub fn a_failed_save_rolls_the_creation_back(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        let opened = service(&world).open(&open_request()).map_err(|error| error.message)?;
        let id = TicketId::parse(&opened.id).map_err(|error| error.message)?;
        service(&world).close(&close_request(&opened.id)).map_err(|error| error.message)?;
        world.store.fail_write(&vectors.str("failurePath"), &vectors.str("failureMessage"));
        let failure = service(&world).reopen(&reopen_request(&opened.id)).err().map(|error| format!("{}:{}", error.class, error.message)).unwrap_or_else(|| "accepted".to_string());
        let mut journal = TransactionJournal::default();
        let creation = ensure_important_document(&world.store, &world.layout.important_path(&id), &mut journal).map_err(|error| error.message)?;
        rollback_important_creation(&world.store, &creation, &mut journal).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("failure".to_string(), Json::String(failure)),
            ("replayJournal".to_string(), strings(journal.trace())),
            ("importantDocument".to_string(), Json::String(if world.store.exists(&world.layout.important_path(&id)) { "present" } else { "absent" }.to_string())),
            ("importantDirectory".to_string(), Json::String(if world.store.exists(&world.layout.important_dir(&id)) { "present" } else { "absent" }.to_string())),
        ])))
    }

    /// 🛡️ A preserved document is not rolled back.
    pub fn a_preserved_document_is_not_rolled_back(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        let opened = service(&world).open(&open_request()).map_err(|error| error.message)?;
        let id = TicketId::parse(&opened.id).map_err(|error| error.message)?;
        service(&world).close(&close_request(&opened.id)).map_err(|error| error.message)?;
        world.store.write(&world.layout.important_path(&id), "").map_err(|error| error.message)?;
        world.store.fail_write(&vectors.str("failurePath"), &vectors.str("failureMessage"));
        let failure = service(&world).reopen(&reopen_request(&opened.id)).err().map(|error| format!("{}:{}", error.class, error.message)).unwrap_or_else(|| "accepted".to_string());
        let mut journal = TransactionJournal::default();
        let creation = ensure_important_document(&world.store, &world.layout.important_path(&id), &mut journal).map_err(|error| error.message)?;
        rollback_important_creation(&world.store, &creation, &mut journal).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("failure".to_string(), Json::String(failure)),
            ("replayJournal".to_string(), strings(journal.trace())),
            ("importantDocument".to_string(), Json::String(if world.store.exists(&world.layout.important_path(&id)) { "present" } else { "absent" }.to_string())),
        ])))
    }

    /// 📓️ The journal records every step in order.
    pub fn the_journal_records_every_step_in_order(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        let opened = service(&world).open(&open_request()).map_err(|error| error.message)?;
        let id = TicketId::parse(&opened.id).map_err(|error| error.message)?;
        let closed = service(&world).close(&close_request(&opened.id)).map_err(|error| error.message)?;
        let reopened = service(&world).reopen(&reopen_request(&opened.id)).map_err(|error| error.message)?;
        let mut inspection = TransactionJournal::default();
        let inspected = inspect_important_document(&world.store, &world.layout.important_path(&id), &mut inspection).map(|preimage| preimage.path).unwrap_or_else(|error| error.message);
        Ok(Outcome::projection(Json::Object(vec![
            ("open".to_string(), strings(opened.journal.trace())),
            ("close".to_string(), strings(closed.journal.trace())),
            ("reopen".to_string(), strings(reopened.journal.trace())),
            ("inspection".to_string(), strings(inspection.trace())),
            ("inspected".to_string(), Json::String(inspected)),
        ])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("a-wrong-bundle-is-refused-before-anything-is-removed", subject::a_wrong_bundle_is_refused_before_anything_is_removed)
        .subject("a-failed-save-rolls-the-creation-back", subject::a_failed_save_rolls_the_creation_back)
        .subject("a-preserved-document-is-not-rolled-back", subject::a_preserved_document_is_not_rolled_back)
        .subject("the-journal-records-every-step-in-order", subject::the_journal_records_every_step_in_order);
    registered
}
//#endregion 🔖️Registration
