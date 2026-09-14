//! 🦀️ Rust side of the ticket lifecycle case. Everything runs against an in-memory store, a frozen
//! clock, a null issue tracker and a recording event sink — no filesystem, no `gh`, no clock read.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tickets::{
        ManagementProviders, NullManagementProvider,
        join_path, normalize_ticket_file_inputs, parse_change_request, parse_close_request, parse_open_request, parse_reopen_request, FixedClock, MemoryTicketStore, RecordingEventSink, TicketCloseRequest, TicketId, TicketLayout, TicketOutcome,
        TicketService, TicketStore, OVERSIZED_FILE_BYTES,
    };
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "local://🔓️lifecycle.json";

    struct World {
        store: MemoryTicketStore,
        tracker: ManagementProviders,
        clock: FixedClock,
        events: RecordingEventSink,
        layout: TicketLayout,
    }

    fn world(vectors: &Json) -> World {
        let clock = vectors.get("clock").cloned().unwrap_or(Json::Null);
        let number = |key: &str| match clock.get(key) {
            Some(Json::Number(value)) => *value as i64,
            _ => 0,
        };
        World {
            store: MemoryTicketStore::new(),
            tracker: ManagementProviders::Null(NullManagementProvider),
            clock: FixedClock::new(number("year"), number("month"), number("day"), clock.str("stamp")),
            events: RecordingEventSink::new(),
            layout: TicketLayout::new(vectors.str("repoMetaDir")),
        }
    }

    fn service(world: &World) -> TicketService<'_, MemoryTicketStore, ManagementProviders, FixedClock, RecordingEventSink> {
        TicketService::new(world.layout.clone(), &world.store, &world.tracker, &world.clock, &world.events)
    }

    fn text_of(vectors: &Json, key: &str) -> String {
        vectors.get(key).map(Json::to_string).unwrap_or_else(|| "{}".to_string())
    }

    fn strings(values: Vec<String>) -> Json {
        Json::Array(values.into_iter().map(Json::String).collect())
    }

    fn events(world: &World) -> Json {
        Json::Array(
            world
                .events
                .recorded()
                .into_iter()
                .map(|event| Json::Object(vec![("kind".to_string(), Json::String(event.kind)), ("source".to_string(), Json::String(event.source)), ("payload".to_string(), Json::String(event.payload))]))
                .collect(),
        )
    }

    fn outcome_json(outcome: &TicketOutcome) -> Json {
        Json::Object(vec![
            ("id".to_string(), Json::String(outcome.id.clone())),
            ("relPath".to_string(), Json::String(outcome.rel_path.clone())),
            ("status".to_string(), Json::String(outcome.status.clone())),
            ("document".to_string(), Json::String(outcome.document.clone())),
            ("journal".to_string(), strings(outcome.journal.trace())),
            ("warnings".to_string(), strings(outcome.warnings.clone())),
        ])
    }

    fn opened(world: &World, vectors: &Json) -> Result<TicketOutcome, String> {
        service(world).open(&parse_open_request(&text_of(vectors, "open")).map_err(|error| error.message)?).map_err(|error| error.message)
    }

    /// 📬️ An open materialises the folder and emits.
    pub fn an_open_materialises_the_folder_and_emits(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        let outcome = opened(&world, &vectors)?;
        Ok(Outcome::projection(Json::Object(vec![("outcome".to_string(), outcome_json(&outcome)), ("paths".to_string(), strings(world.store.paths())), ("events".to_string(), events(&world))])))
    }

    /// 📪️ A close consumes the important document.
    pub fn a_close_consumes_the_important_document(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        opened(&world, &vectors)?;
        let outcome = service(&world).close(&parse_close_request(&text_of(&vectors, "close")).map_err(|error| error.message)?).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![("outcome".to_string(), outcome_json(&outcome)), ("paths".to_string(), strings(world.store.paths())), ("events".to_string(), events(&world))])))
    }

    /// 🔓️ A reopen restores the important document.
    pub fn a_reopen_restores_the_important_document(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        opened(&world, &vectors)?;
        service(&world).close(&parse_close_request(&text_of(&vectors, "close")).map_err(|error| error.message)?).map_err(|error| error.message)?;
        let outcome = service(&world).reopen(&parse_reopen_request(&text_of(&vectors, "reopen")).map_err(|error| error.message)?).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![("outcome".to_string(), outcome_json(&outcome)), ("paths".to_string(), strings(world.store.paths())), ("events".to_string(), events(&world))])))
    }

    /// ♻️ A change renames the folder.
    pub fn a_change_renames_the_folder(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        opened(&world, &vectors)?;
        let outcome = service(&world).change(&parse_change_request(&text_of(&vectors, "change")).map_err(|error| error.message)?).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![("outcome".to_string(), outcome_json(&outcome)), ("paths".to_string(), strings(world.store.paths())), ("events".to_string(), events(&world))])))
    }

    /// ⚠️ A close of a ticket that is not open is refused.
    pub fn a_close_refuses_a_ticket_that_is_not_open(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        opened(&world, &vectors)?;
        let valid = parse_close_request(&text_of(&vectors, "close")).map_err(|error| error.message)?;
        let mut projected: Vec<(String, Json)> = Vec::new();
        let mut refuse = |name: &str, request: &TicketCloseRequest| {
            let outcome = match service(&world).close(request) {
                Ok(_) => "accepted".to_string(),
                Err(error) => format!("{}:{}", error.class, error.message),
            };
            projected.push((name.to_string(), Json::String(outcome)));
        };
        refuse("no-summary", &TicketCloseRequest { summary: String::new(), ..valid.clone() });
        refuse("no-files", &TicketCloseRequest { files: Vec::new(), ..valid.clone() });
        refuse("unknown-id", &TicketCloseRequest { id: "26/09/06/NO-SUCH-TICKET".to_string(), ..valid.clone() });
        refuse("malformed-id", &TicketCloseRequest { id: "nonsense".to_string(), ..valid.clone() });
        refuse("first-close", &valid);
        refuse("second-close", &valid);
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 📝️ File inputs are normalised before they are recorded.
    pub fn file_inputs_are_normalised_before_they_are_recorded(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let close = parse_close_request(&text_of(&vectors, "close")).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![("input".to_string(), strings(close.files.clone())), ("normalised".to_string(), strings(normalize_ticket_file_inputs(&close.files)))])))
    }

    /// 🧹️ An oversized artifact is purged.
    pub fn an_oversized_artifact_is_purged(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let world = world(&vectors);
        let outcome = opened(&world, &vectors)?;
        let id = TicketId::parse(&outcome.id).map_err(|error| error.message)?;
        let folder = world.layout.ticket_dir(&id);
        let purge = vectors.get("purge").cloned().unwrap_or(Json::Null);
        world.store.seed_sized_file(&join_path(&folder, &purge.str("oversizedFile")), OVERSIZED_FILE_BYTES + 1);
        world.store.seed_sized_file(&join_path(&folder, &purge.str("smallFile")), 128);
        let report = service(&world).purge_artifacts(&id).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("removedFiles".to_string(), strings(report.removed_files)),
            ("removedDirectories".to_string(), strings(report.removed_directories)),
            ("surviving".to_string(), strings(world.store.paths().into_iter().filter(|path| world.store.is_file(path)).collect())),
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
        .subject("an-open-materialises-the-folder-and-emits", subject::an_open_materialises_the_folder_and_emits)
        .subject("a-close-consumes-the-important-document", subject::a_close_consumes_the_important_document)
        .subject("a-reopen-restores-the-important-document", subject::a_reopen_restores_the_important_document)
        .subject("a-change-renames-the-folder", subject::a_change_renames_the_folder)
        .subject("a-close-refuses-a-ticket-that-is-not-open", subject::a_close_refuses_a_ticket_that_is_not_open)
        .subject("file-inputs-are-normalised-before-they-are-recorded", subject::file_inputs_are_normalised_before_they_are_recorded)
        .subject("an-oversized-artifact-is-purged", subject::an_oversized_artifact_is_purged);
    registered
}
//#endregion 🔖️Registration
