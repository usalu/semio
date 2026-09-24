use super::*;
use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let unique = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir()
            .join(format!("semio-repo-events-{}-{unique}", std::process::id()));
        fs::create_dir_all(&path).expect("temp dir");
        Self(path)
    }
    fn join(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct PhaseCancel {
    phase: String,
    hit: Cell<bool>,
}

impl PhaseCancel {
    fn new(phase: &str) -> Self {
        Self { phase: phase.to_string(), hit: Cell::new(false) }
    }
}

impl Interrupt for PhaseCancel {
    fn cancelled(&self) -> bool {
        self.hit.get()
    }
    fn report(&self, progress: Progress) {
        if progress.step == self.phase {
            self.hit.set(true);
        }
    }
}

struct CancelAtReplay(Cell<bool>);

impl Interrupt for CancelAtReplay {
    fn cancelled(&self) -> bool {
        self.0.get()
    }
    fn report(&self, progress: Progress) {
        if progress.current == 1 {
            self.0.set(true);
        }
    }
}

struct AlwaysCancelled;

impl Interrupt for AlwaysCancelled {
    fn cancelled(&self) -> bool {
        true
    }
}

//#region 📋️EventKindCatalog

#[test]
fn kind_constants_match_schema_catalog() {
    let catalog = kind_catalog();
    assert_eq!(catalog.schema_version, 1);
    assert_eq!(ALL_EVENT_KINDS.len(), catalog.kinds.len());
    for (index, entry) in catalog.kinds.iter().enumerate() {
        assert_eq!(ALL_EVENT_KINDS[index], entry.kind, "kind {index}");
    }
    let mut seen: Vec<&str> = Vec::new();
    for kind in ALL_EVENT_KINDS {
        assert!(!seen.contains(kind), "duplicate kind {kind}");
        assert!(kind.contains('.'), "kind {kind} is not dotted");
        seen.push(kind);
    }
}

//#endregion 📋️EventKindCatalog

//#region ✉️PayloadEncoding

#[derive(Deserialize)]
struct PayloadCase {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    input: serde_json::Value,
    encoded: String,
}

#[derive(Deserialize)]
struct PayloadVectors {
    cases: Vec<PayloadCase>,
}

fn round_trip(kind: &str, input: &serde_json::Value) -> Result<String, String> {
    macro_rules! decode {
        ($($name:literal => $type:ty),* $(,)?) => {
            match kind {
                $($name => {
                    let value: $type = serde_json::from_value(input.clone()).map_err(|error| error.to_string())?;
                    serde_json::to_string(&value).map_err(|error| error.to_string())
                })*
                other => Err(format!("unknown payload type {other}")),
            }
        };
    }
    decode! {
        "TicketPayload" => TicketPayload,
        "TicketOpenPayload" => TicketOpenPayload,
        "TicketClosePayload" => TicketClosePayload,
        "TicketReopenPayload" => TicketReopenPayload,
        "TicketChangePayload" => TicketChangePayload,
        "GoalPayload" => GoalPayload,
        "GoalOpenPayload" => GoalOpenPayload,
        "GoalClosePayload" => GoalClosePayload,
        "GoalReopenPayload" => GoalReopenPayload,
        "GoalChangePayload" => GoalChangePayload,
        "ContributorPayload" => ContributorPayload,
        "CheckpointPayload" => CheckpointPayload,
        "TodoPayload" => TodoPayload,
        "TodoCreatePayload" => TodoCreatePayload,
        "TodoChangePayload" => TodoChangePayload,
        "TodoDeletePayload" => TodoDeletePayload,
        "WorkItem" => WorkItem,
        "ContributorWork" => ContributorWork,
        "DraftPayload" => DraftPayload,
        "FilePayload" => FilePayload,
        "FolderPayload" => FolderPayload,
        "SectionPayload" => SectionPayload,
        "IntegratePayload" => IntegratePayload,
        "ExtractPayload" => ExtractPayload,
    }
}

#[test]
fn payload_golden_encoding() {
    let vectors: PayloadVectors = fixture("✉️payload-vectors.json").expect("payload vectors");
    assert!(!vectors.cases.is_empty());
    for case in &vectors.cases {
        let encoded = round_trip(&case.kind, &case.input)
            .unwrap_or_else(|error| panic!("{}: {error}", case.id));
        assert_eq!(encoded, case.encoded, "{}", case.id);
    }
}

#[test]
fn envelope_encoding() {
    let envelope = Event {
        kind: TICKET_OPEN_STARTING.to_string(),
        source: "repo-cli".to_string(),
        payload: serde_json::json!({"id": "a"}),
    };
    let expected = concat!(
        "{\"kind\":\"ticket.open.starting\",",
        "\"source\":\"repo-cli\",",
        "\"payload\":{\"id\":\"a\"}}"
    );
    assert_eq!(serde_json::to_string(&envelope).unwrap(), expected);
}

#[test]
fn emit_url_and_target() {
    assert_eq!(emit_url(""), "");
    assert_eq!(emit_url("   "), "");
    assert_eq!(emit_url("127.0.0.1:8787"), "http://127.0.0.1:8787/api/v1/events");
    assert_eq!(emit_url("http://host:1/"), "http://host:1/api/v1/events");
    assert_eq!(emit_url("https://host.example"), "https://host.example/api/v1/events");
    assert_eq!(emit_url("http://host.example///"), "http://host.example///api/v1/events");
    let target = parse_target("http://127.0.0.1:8787/api/v1/events").unwrap();
    assert_eq!(target.host, "127.0.0.1");
    assert_eq!(target.port, 8787);
    assert_eq!(target.path, "/api/v1/events");
    assert!(!target.secure);
    assert_eq!(parse_target("https://host/x").unwrap().port, 443);
    assert!(parse_target("ftp://host/x").is_none());
}

//#endregion ✉️PayloadEncoding

//#region 🗄️StoreAppendSequence

#[derive(Deserialize)]
struct StoreVectors {
    inputs: Vec<Input>,
    sequences: Vec<u64>,
    #[serde(rename = "interruptPhases")]
    interrupt_phases: Vec<String>,
}

fn store_vectors() -> StoreVectors {
    fixture("🗄️store-vectors.json").expect("store vectors")
}

#[test]
fn store_fixture_deterministic_replay() {
    let vectors = store_vectors();
    let first_dir = TempDir::new();
    let store = Store::new(first_dir.join("events.jsonl"));
    store.append(&vectors.inputs, &Uninterrupted).expect("append");
    let first = fs::read(&store.path).expect("read");
    let replayed = store.replay(&Uninterrupted).expect("replay");
    let sequences: Vec<u64> = replayed.iter().map(|event| event.sequence).collect();
    assert_eq!(sequences, vectors.sequences);
    let second_dir = TempDir::new();
    let second_store = Store::new(second_dir.join("events.jsonl"));
    second_store.append(&vectors.inputs, &Uninterrupted).expect("append");
    assert_eq!(first, fs::read(&second_store.path).expect("read"));
}

#[test]
fn store_duplicate_interrupted_and_corrupt_event() {
    let vectors = store_vectors();
    let dir = TempDir::new();
    let store = Store::new(dir.join("events.jsonl"));
    store.append(&vectors.inputs[..1], &Uninterrupted).expect("append");
    let before = fs::read(&store.path).expect("read");
    assert!(matches!(
        store.append(&vectors.inputs[..1], &Uninterrupted),
        Err(StoreError::Duplicate(_))
    ));
    let cancel = PhaseCancel::new("encoded");
    assert_eq!(store.append(&vectors.inputs[1..], &cancel), Err(StoreError::Cancelled));
    assert_eq!(before, fs::read(&store.path).expect("read"));
    let mut corrupt = before;
    let middle = corrupt.len() / 2;
    corrupt[middle] ^= 1;
    fs::write(&store.path, &corrupt).expect("write");
    assert!(matches!(store.replay(&Uninterrupted), Err(StoreError::Corrupt(_))));
}

#[test]
fn store_cancellation_and_maximum() {
    let dir = TempDir::new();
    let store = Store::new(dir.join("events.jsonl"));
    let one =
        vec![Input { id: "a".into(), kind: "recorded".into(), data: Payload::of(&serde_json::json!("a")).expect("payload") }];
    assert_eq!(store.append(&one, &AlwaysCancelled), Err(StoreError::Cancelled));
    let maximum = "x".repeat(MAX_EVENT_SIZE - 2);
    store
        .append(
            &[Input {
                id: "max".into(),
                kind: "recorded".into(),
                data: Payload::of(&serde_json::Value::String(maximum)).expect("payload"),
            }],
            &Uninterrupted,
        )
        .expect("maximum input");
    let before = fs::read(&store.path).expect("read");
    let plus_one = "x".repeat(MAX_EVENT_SIZE - 1);
    assert!(matches!(
        store.append(
            &[Input {
                id: "plus-one".into(),
                kind: "recorded".into(),
                data: Payload::of(&serde_json::Value::String(plus_one)).expect("payload"),
            }],
            &Uninterrupted
        ),
        Err(StoreError::TooLarge(_))
    ));
    assert_eq!(before, fs::read(&store.path).expect("read"));
}

#[test]
fn store_interruptions_preserve_committed_log() {
    let vectors = store_vectors();
    for phase in &vectors.interrupt_phases {
        let dir = TempDir::new();
        let store = Store::new(dir.join("events.jsonl"));
        store.append(&vectors.inputs[..1], &Uninterrupted).expect("append");
        let before = fs::read(&store.path).expect("read");
        let cancel = PhaseCancel::new(phase);
        assert_eq!(
            store.append(&vectors.inputs[1..], &cancel),
            Err(StoreError::Cancelled),
            "{phase}"
        );
        assert_eq!(before, fs::read(&store.path).expect("read"), "{phase}");
        assert!(!store.stage_path().exists(), "{phase} left a stage");
        let events = store.replay(&Uninterrupted).expect("replay");
        assert_eq!(events.len(), 1, "{phase}");
        assert_eq!(events[0].id, vectors.inputs[0].id, "{phase}");
    }
}

#[test]
fn store_replay_cancellation_preserves_log() {
    let vectors = store_vectors();
    let dir = TempDir::new();
    let store = Store::new(dir.join("events.jsonl"));
    store.append(&vectors.inputs, &Uninterrupted).expect("append");
    let before = fs::read(&store.path).expect("read");
    let cancel = CancelAtReplay(Cell::new(false));
    assert_eq!(store.replay(&cancel), Err(StoreError::Cancelled));
    assert_eq!(before, fs::read(&store.path).expect("read"));
    assert_eq!(store.replay(&Uninterrupted).expect("replay").len(), vectors.inputs.len());
}

#[test]
fn staged_append_recovery() {
    for (name, numerator, denominator, want_events, want_changed) in [
        ("stage only", 0usize, 1usize, 1usize, false),
        ("partial batch", 1, 2, 1, false),
        ("complete batch", 1, 1, 2, true),
    ] {
        let dir = TempDir::new();
        let store = Store::new(dir.join("events.jsonl"));
        store
            .append(
                &[Input {
                    id: "first".into(),
                    kind: "recorded".into(),
                    data: Payload::of(&serde_json::json!("first")).expect("payload"),
                }],
                &Uninterrupted,
            )
            .expect("append");
        let before = fs::read(&store.path).expect("read");
        let mut event = StoreEvent {
            schema: SCHEMA.into(),
            sequence: 2,
            id: "second".into(),
            kind: "recorded".into(),
            data: Payload::of(&serde_json::json!("second")).expect("payload"),
            checksum: String::new(),
        };
        event.checksum = checksum(&event);
        let mut batch = serde_json::to_string(&event).unwrap();
        batch.push('\n');
        let staged = Stage {
            schema: STAGE_SCHEMA.into(),
            prior_exists: true,
            prior_size: before.len() as u64,
            prior_checksum: digest(&before),
            batch_size: batch.len(),
            batch_checksum: digest(batch.as_bytes()),
        };
        fs::write(store.stage_path(), serde_json::to_vec(&staged).unwrap()).expect("stage");
        let written = &batch.as_bytes()[..batch.len() * numerator / denominator];
        if !written.is_empty() {
            let mut file = fs::OpenOptions::new().append(true).open(&store.path).unwrap();
            file.write_all(written).unwrap();
        }
        let events = store.replay(&Uninterrupted).expect("replay");
        assert_eq!(events.len(), want_events, "{name}");
        let after = fs::read(&store.path).expect("read");
        assert_eq!(before != after, want_changed, "{name}");
        assert!(!store.stage_path().exists(), "{name} left a stage");
    }
}

//#endregion 🗄️StoreAppendSequence

//#region 📤️ExportContentHash

#[derive(Deserialize)]
struct ExportEntityVector {
    kind: String,
    id: String,
    value: serde_json::Value,
}

#[derive(Deserialize)]
struct ExportVectors {
    entities: Vec<ExportEntityVector>,
    #[serde(rename = "sortedIds")]
    sorted_ids: Vec<String>,
    snapshot: String,
    #[serde(rename = "inputIds")]
    input_ids: Vec<String>,
    counts: std::collections::BTreeMap<String, usize>,
}

#[test]
fn export_snapshot_content_hash() {
    let vectors: ExportVectors = fixture("📤️export-vectors.json").expect("export vectors");
    let entities: Vec<ExportEntity> = vectors
        .entities
        .iter()
        .map(|entity| ExportEntity {
            kind: entity.kind.clone(),
            id: entity.id.clone(),
            value: Payload::of(&entity.value).expect("payload"),
        })
        .collect();
    let snapshot = build_export_snapshot(&entities, &Uninterrupted).expect("snapshot");
    assert_eq!(snapshot.snapshot, vectors.snapshot);
    for (kind, count) in &vectors.counts {
        assert_eq!(snapshot.count(kind), *count, "{kind}");
    }
    let ids: Vec<String> = snapshot.inputs.iter().map(|input| input.id.clone()).collect();
    assert_eq!(ids, vectors.input_ids);
    let prefix = format!("snapshot:{}:", vectors.snapshot);
    let stripped: Vec<String> = ids
        .iter()
        .map(|id| id.trim_start_matches(prefix.as_str()).to_string())
        .collect();
    assert_eq!(stripped, vectors.sorted_ids);
    let dir = TempDir::new();
    let store = Store::new(dir.join("export.events.jsonl"));
    store.append(&snapshot.inputs, &Uninterrupted).expect("append");
    assert!(matches!(
        store.append(&snapshot.inputs, &Uninterrupted),
        Err(StoreError::Duplicate(_))
    ));
    assert_eq!(build_export_snapshot(&entities, &AlwaysCancelled), Err(StoreError::Cancelled));
}

#[test]
fn digest_and_checksum_are_stable() {
    assert_eq!(digest(&[]), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(
        digest(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(digest(&[b'a'; 1000]), digest("a".repeat(1000).as_bytes()));
    let event = StoreEvent {
        schema: SCHEMA.into(),
        sequence: 1,
        id: "a".into(),
        kind: "recorded".into(),
        data: Payload::of(&serde_json::json!("a")).expect("payload"),
        checksum: String::new(),
    };
    let mut other = event.clone();
    other.sequence = 2;
    assert_ne!(checksum(&event), checksum(&other));
}

//#endregion 📤️ExportContentHash
