//! 🧪️ Exact rejected-page grant laws over real Store wrappers and retained field owners.

use super::*;
use serde::Deserialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

//#region 🧪️NeutralFixture
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fixture {
    version: u8,
    page_bytes: usize,
    input_state: String,
    field_owner: FieldVector,
    cases: Vec<Case>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Grant {
    maximum_items: usize,
    maximum_bytes: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FieldVector {
    token_id: u64,
    payload: Vec<u8>,
    close_grant: Grant,
    expected: FieldExpected,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FieldExpected {
    token_drops: usize,
    field_drops: usize,
    registered_close_calls: usize,
    unadmitted_close_calls: usize,
    accept_calls: usize,
    finish_calls: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Page {
    length: usize,
    suffix: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Case {
    id: String,
    pages: Vec<Page>,
    expected_document: serde_json::Value,
    closes: Vec<Close>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Close {
    grant: Grant,
    expected: Expected,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expected {
    kind: String,
    released_items: usize,
    released_bytes: usize,
    remaining_pages: usize,
    remaining_bytes: usize,
    record_present: bool,
    terminal: bool,
}

fn fixture() -> Fixture {
    let fixture: Fixture = serde_json::from_str(include_str!("🔣️.json")).expect("canonical rejected-page vectors");
    assert_eq!(fixture.version, 1);
    assert_eq!(fixture.page_bytes, OWNED_SCHEMA_DECODE_PAGE_BYTES);
    assert_eq!(fixture.input_state, "unstarted-record");
    fixture
}

fn pages(row: &Case) -> Vec<Vec<u8>> {
    let pages = row
        .pages
        .iter()
        .map(|page| {
            let padding = page.length.checked_sub(page.suffix.len()).expect("explicit UTF-8 suffix fits page");
            let mut bytes = vec![b' '; padding];
            bytes.extend_from_slice(page.suffix.as_bytes());
            assert!(!bytes.is_empty() && bytes.len() <= OWNED_SCHEMA_DECODE_PAGE_BYTES);
            bytes
        })
        .collect::<Vec<_>>();
    assert!(pages.iter().take(pages.len() - 1).all(|page| page.len() == OWNED_SCHEMA_DECODE_PAGE_BYTES));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&pages.concat()).expect("neutral page JSON"), row.expected_document);
    pages
}

fn record(pages: &[Vec<u8>]) -> OwnedSchemaRecordCursor {
    let mut admitted = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: pages.len(), maximum_bytes: pages.iter().map(Vec::len).sum() }).expect("exact page credits");
    for bytes in pages {
        let page = OwnedSchemaDecodePage::try_from_slice(bytes).expect("bounded actual page");
        admitted.admit_page(page).unwrap_or_else(|_| panic!("valid full-nonterminal admission"));
    }
    admitted.seal().expect("seal exact pages");
    artifact_envelope_decode_record(semio_framework_job::OperationId(66), semio_framework_job::Generation(1), admitted).unwrap_or_else(|_| panic!("unstarted actual envelope record"))
}

fn note(failures: &mut Vec<String>, label: &str, condition: bool, detail: impl std::fmt::Debug) {
    if !condition {
        failures.push(format!("{label}: {detail:?}"));
    }
}
//#endregion 🧪️NeutralFixture

//#region 🧪️CountedFieldOwnership
#[derive(Default)]
struct Counts {
    close_calls: AtomicUsize,
    accept_calls: AtomicUsize,
    finish_calls: AtomicUsize,
    field_drops: AtomicUsize,
    token_drops: Mutex<Vec<(usize, u64, Vec<u8>)>>,
}

struct CountedToken {
    id: u64,
    payload: Box<[u8]>,
    released: bool,
    counts: Arc<Counts>,
}

impl Drop for CountedToken {
    fn drop(&mut self) {
        assert!(self.released, "counted token dropped without its bounded close");
        self.counts.token_drops.lock().expect("counted token log").push((self as *const Self as usize, self.id, self.payload.to_vec()));
    }
}

struct CountedField {
    token: Option<Box<CountedToken>>,
    counts: Arc<Counts>,
}

impl CountedField {
    fn new(vector: &FieldVector, counts: &Arc<Counts>) -> (Box<Self>, usize) {
        let token = Box::new(CountedToken { id: vector.token_id, payload: vector.payload.clone().into_boxed_slice(), released: false, counts: Arc::clone(counts) });
        let identity = token.as_ref() as *const CountedToken as usize;
        (Box::new(Self { token: Some(token), counts: Arc::clone(counts) }), identity)
    }

    fn unexpected() -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code: "test.rejected-page-unstarted-field", offset: 0, line: 1, column: 1, path: OwnedSchemaPath::ROOT }
    }
}

impl ArtifactEnvelopeFieldDecoder<(), ()> for CountedField {
    fn accept_field_token(
        &mut self,
        _field_id: u16,
        _token: OwnedSchemaToken,
        _terminal: bool,
        _source: &OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        self.counts.accept_calls.fetch_add(1, Ordering::SeqCst);
        Err(Self::unexpected())
    }

    fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        self.counts.finish_calls.fetch_add(1, Ordering::SeqCst);
        Err(Self::unexpected())
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
        self.counts.close_calls.fetch_add(1, Ordering::SeqCst);
        let Some(token) = self.token.as_ref() else { return Ok(SnapshotRetirementStep::Complete) };
        if maximum_items == 0 || maximum_bytes < token.payload.len() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released_bytes = token.payload.len();
        let mut token = self.token.take().expect("same counted token");
        token.released = true;
        drop(token);
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }

    fn terminal_is_empty(&self) -> bool {
        self.token.is_none()
    }
}

impl Drop for CountedField {
    fn drop(&mut self) {
        assert!(self.token.is_none(), "counted field dropped before its token retired");
        self.counts.field_drops.fetch_add(1, Ordering::SeqCst);
    }
}
//#endregion 🧪️CountedFieldOwnership

//#region 🧪️ActualWrapperHarness
const COUNTED_TOKEN_CLOSE_PHASES: usize = 1;
const FIELD_SHELL_CLOSE_PHASES: usize = 1;
const LEASE_RETURN_PHASES: usize = 1;
const REGISTRY_DETACH_PHASES: usize = 1;
const RECORD_COMPLETION_PHASES: usize = 1;
const AUTHORITY_COMPLETION_PHASES: usize = 1;

/// 🧮️ Adds actual remaining pages and explicit finite ownership phases without overflow.
fn checked_close_bound(maximum_pages: usize, phases: &[usize]) -> usize {
    phases.iter().try_fold(maximum_pages, |bound, phase| bound.checked_add(*phase)).expect("fixture close phase bound fits usize")
}

/// 📤️ A returned counted owner closes its single token, then its field shell.
fn returned_owner_close_bound() -> usize {
    checked_close_bound(0, &[COUNTED_TOKEN_CLOSE_PHASES, FIELD_SHELL_CLOSE_PHASES])
}

/// 📦️ Unadmitted rejection closes the token, field shell, pages, and final record shell.
fn unadmitted_close_bound(maximum_pages: usize) -> usize {
    checked_close_bound(maximum_pages, &[COUNTED_TOKEN_CLOSE_PHASES, FIELD_SHELL_CLOSE_PHASES, RECORD_COMPLETION_PHASES])
}

/// 🪪️ Registered rejection additionally returns and detaches its lease before the returned field shell closes.
fn registered_close_bound(maximum_pages: usize) -> usize {
    checked_close_bound(maximum_pages, &[COUNTED_TOKEN_CLOSE_PHASES, LEASE_RETURN_PHASES, REGISTRY_DETACH_PHASES, FIELD_SHELL_CLOSE_PHASES, RECORD_COMPLETION_PHASES])
}

/// 🏁️ The untransferred authority needs one terminal-state turn after its record shell closes.
fn authority_close_bound(maximum_pages: usize) -> usize {
    checked_close_bound(registered_close_bound(maximum_pages), &[AUTHORITY_COMPLETION_PHASES])
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RecordWitness {
    storage: usize,
    capacity: usize,
    pages: usize,
    bytes: usize,
    sealed: bool,
    contents: Vec<Option<u8>>,
}

fn witness(record: Option<&OwnedSchemaRecordCursor>) -> Option<RecordWitness> {
    record.map(|record| {
        let pages = &record.tokens.pages;
        RecordWitness {
            storage: pages.slots.as_ptr() as usize,
            capacity: pages.slots.len(),
            pages: pages.page_count(),
            bytes: pages.byte_count(),
            sealed: pages.is_sealed(),
            contents: (0..pages.byte_count()).map(|index| pages.byte_at(index)).collect(),
        }
    })
}

enum Subject {
    Registered { rejected: ArtifactEnvelopeDecodeRejected<(), ()>, registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<(), ()>>, ticket: ArtifactEnvelopeFieldDecoderTicket },
    Unadmitted(ArtifactEnvelopeUnadmittedDecodeRejected<(), ()>),
}

fn detach_and_close(registry: &Arc<ArtifactEnvelopeFieldDecoderRegistry<(), ()>>, expected_ticket: Option<ArtifactEnvelopeFieldDecoderTicket>, expected_field: Option<usize>, failures: &mut Vec<String>) {
    let Some(ticket) = registry.next_returned_ticket() else { return };
    if let Some(expected) = expected_ticket {
        note(failures, "exact returned ticket", ticket == expected, (ticket, expected));
    }
    match registry.take_returned_ticket(ticket) {
        Ok(mut returned) => {
            let identity = returned.owner.as_deref().map(|owner| owner as *const dyn ArtifactEnvelopeFieldDecoder<(), ()> as *const () as usize);
            if let Some(expected) = expected_field {
                note(failures, "exact detached field address", identity == Some(expected), (identity, expected));
            }
            note(failures, "detach is not close completion", registry.ticket_reclaimed(ticket) && !returned.terminal_is_empty(), returned.terminal_is_empty());
            for _ in 0..returned_owner_close_bound() {
                let result = returned.close_step(1, OWNED_SCHEMA_DECODE_PAGE_BYTES);
                if result.as_ref() == Ok(&SnapshotRetirementStep::Complete) && returned.terminal_is_empty() {
                    break;
                }
                if let Err(error) = result {
                    failures.push(format!("returned close: {error}"));
                }
            }
            note(failures, "returned owner bounded terminal", returned.terminal_is_empty(), returned.terminal_is_empty());
        }
        Err(error) => failures.push(format!("exact returned ticket detach: {error:?}")),
    }
}

impl Subject {
    fn new(registered: bool, record: OwnedSchemaRecordCursor, fields: Box<CountedField>, failures: &mut Vec<String>) -> Option<Self> {
        if !registered {
            return Some(Self::Unadmitted(ArtifactEnvelopeUnadmittedDecodeRejected::new(record, fields)));
        }
        let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
        let authority = match ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(record, &registry, fields) {
            Ok(authority) => authority,
            Err((record, fault, fields)) => {
                failures.push(format!("fresh registry rejected setup: {fault:?}"));
                let close_bound = unadmitted_close_bound(record.tokens.pages.page_count());
                let mut rejected = ArtifactEnvelopeUnadmittedDecodeRejected::new(record, fields);
                for _ in 0..close_bound {
                    let result = rejected.close_step(1, OWNED_SCHEMA_DECODE_PAGE_BYTES);
                    if result.as_ref() == Ok(&SnapshotRetirementStep::Complete) && rejected.terminal_is_empty() {
                        break;
                    }
                }
                note(failures, "refused setup bounded terminal", rejected.terminal_is_empty(), rejected.terminal_is_empty());
                return None;
            }
        };
        let ticket = authority.field_ticket;
        let diagnostic = OwnedSchemaDecodeDiagnostic { code: "test.rejected-page-close", offset: 0, line: 1, column: 1, path: OwnedSchemaPath::ROOT };
        match authority.reject(diagnostic) {
            Ok(rejected) => Some(Self::Registered { rejected, registry, ticket }),
            Err(mut authority) => {
                failures.push("unstarted public reject refused setup".into());
                let close_bound = authority_close_bound(authority.record.as_ref().map_or(0, |record| record.tokens.pages.page_count()));
                for _ in 0..close_bound {
                    detach_and_close(&registry, Some(ticket), None, failures);
                    semio_framework_job::InteractiveJob::close_step(&mut authority, 1, OWNED_SCHEMA_DECODE_PAGE_BYTES);
                    if authority.terminal_is_empty() {
                        break;
                    }
                }
                note(failures, "refused rejection bounded terminal", authority.terminal_is_empty(), authority.terminal_is_empty());
                None
            }
        }
    }

    fn record(&self) -> Option<&OwnedSchemaRecordCursor> {
        match self {
            Self::Registered { rejected, .. } => rejected.record.as_ref(),
            Self::Unadmitted(rejected) => rejected.record.as_ref(),
        }
    }

    fn field_identity(&self) -> Option<usize> {
        match self {
            Self::Registered { rejected, .. } => rejected.fields.as_ref().and_then(|lease| lease.with_owner(|owner| owner as *mut dyn ArtifactEnvelopeFieldDecoder<(), ()> as *mut () as usize).ok()),
            Self::Unadmitted(rejected) => rejected.fields.as_deref().map(|owner| owner as *const dyn ArtifactEnvelopeFieldDecoder<(), ()> as *const () as usize),
        }
    }

    fn close_step(&mut self, grant: Grant) -> Result<SnapshotRetirementStep, String> {
        match self {
            Self::Registered { rejected, .. } => rejected.close_step(grant.maximum_items, grant.maximum_bytes),
            Self::Unadmitted(rejected) => rejected.close_step(grant.maximum_items, grant.maximum_bytes),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Registered { rejected, registry, .. } => rejected.terminal_is_empty() && registry.terminal_is_empty(),
            Self::Unadmitted(rejected) => rejected.terminal_is_empty(),
        }
    }

    fn prepare_pages(&mut self, vector: &FieldVector, counts: &Counts, field_identity: usize, failures: &mut Vec<String>) {
        let before = witness(self.record());
        note(failures, "exact initial field address", self.field_identity() == Some(field_identity), self.field_identity());
        let zero = self.close_step(Grant { maximum_items: 0, maximum_bytes: OWNED_SCHEMA_DECODE_PAGE_BYTES });
        note(failures, "zero-item field phase", zero == Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }), &zero);
        note(failures, "zero-item keeps field", self.field_identity() == Some(field_identity) && counts.close_calls.load(Ordering::SeqCst) == 0, counts.close_calls.load(Ordering::SeqCst));
        let token_close = self.close_step(vector.close_grant);
        note(failures, "actual field token close", token_close == Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: vector.payload.len() }), &token_close);
        note(failures, "field shell remains after token close", self.field_identity() == Some(field_identity) && counts.field_drops.load(Ordering::SeqCst) == 0, self.field_identity());
        let field_close = self.close_step(vector.close_grant);
        note(failures, "actual field shell close", field_close == Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }), &field_close);
        if let Self::Registered { rejected, registry, ticket } = self {
            note(failures, "registry retained until detach", !registry.ticket_reclaimed(*ticket) && !registry.terminal_is_empty(), registry.ticket_reclaimed(*ticket));
            let blocked = rejected.close_step(1, OWNED_SCHEMA_DECODE_PAGE_BYTES);
            note(failures, "pages wait for exact reclamation", blocked == Ok(SnapshotRetirementStep::Blocked), &blocked);
            detach_and_close(registry, Some(*ticket), Some(field_identity), failures);
            note(failures, "exact ticket reclaimed", registry.ticket_reclaimed(*ticket), registry.ticket_reclaimed(*ticket));
        }
        note(failures, "field phase preserves every original page", witness(self.record()) == before, witness(self.record()));
    }

    fn teardown(&mut self, failures: &mut Vec<String>) {
        let maximum_pages = self.record().map_or(0, |record| record.tokens.pages.page_count());
        let close_bound = match self {
            Self::Registered { .. } => registered_close_bound(maximum_pages),
            Self::Unadmitted(_) => unadmitted_close_bound(maximum_pages),
        };
        for _ in 0..close_bound {
            if let Self::Registered { registry, ticket, .. } = self {
                detach_and_close(registry, Some(*ticket), None, failures);
            }
            if self.terminal_is_empty() {
                break;
            }
            if let Err(error) = self.close_step(Grant { maximum_items: 1, maximum_bytes: OWNED_SCHEMA_DECODE_PAGE_BYTES }) {
                failures.push(format!("wrapper bounded teardown: {error}"));
            }
        }
        note(failures, "wrapper bounded terminal", self.terminal_is_empty(), self.terminal_is_empty());
    }
}
//#endregion 🧪️ActualWrapperHarness

//#region 🧪️PageLaws
fn actual(result: &Result<SnapshotRetirementStep, String>, record: Option<&OwnedSchemaRecordCursor>, terminal: bool) -> Expected {
    let (kind, released_items, released_bytes) = match result {
        Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) => ("pending".into(), *released_items, *released_bytes),
        Ok(SnapshotRetirementStep::Complete) => ("complete".into(), 0, 0),
        Ok(SnapshotRetirementStep::Blocked) => ("blocked".into(), 0, 0),
        Err(error) => (format!("error:{error}"), 0, 0),
    };
    Expected {
        kind,
        released_items,
        released_bytes,
        remaining_pages: record.map_or(0, |record| record.tokens.pages.page_count()),
        remaining_bytes: record.map_or(0, |record| record.tokens.pages.byte_count()),
        record_present: record.is_some(),
        terminal,
    }
}

fn check_case(registered: bool, row: &Case, vector: &FieldVector, failures: &mut Vec<String>) {
    let original_pages = pages(row);
    let record = record(&original_pages);
    let original = witness(Some(&record)).expect("original record witness");
    let counts = Arc::new(Counts::default());
    let (fields, token_identity) = CountedField::new(vector, &counts);
    let field_identity = fields.as_ref() as *const CountedField as usize;
    let Some(mut subject) = Subject::new(registered, record, fields, failures) else { return };
    note(failures, "constructor preserves original page storage", witness(subject.record()) == Some(original.clone()), witness(subject.record()));
    subject.prepare_pages(vector, &counts, field_identity, failures);
    for (index, close) in row.closes.iter().enumerate() {
        let result = subject.close_step(close.grant);
        let observed = actual(&result, subject.record(), subject.terminal_is_empty());
        note(failures, &format!("{} close {index}", row.id), observed == close.expected, (&observed, &close.expected));
        if let Some(current) = witness(subject.record()) {
            let expected_contents = original_pages.iter().take(close.expected.remaining_pages).flatten().copied().map(Some).collect::<Vec<_>>();
            note(failures, &format!("{} original retained page prefix {index}", row.id), current.storage == original.storage && current.capacity == original.capacity && current.sealed && current.contents == expected_contents, current);
        } else {
            note(failures, &format!("{} record removal {index}", row.id), !close.expected.record_present, close.expected.record_present);
        }
        note(
            failures,
            &format!("{} grant not exceeded {index}", row.id),
            observed.released_items <= close.grant.maximum_items && observed.released_bytes <= close.grant.maximum_bytes && observed.released_items <= 1,
            (&observed, close.grant.maximum_items, close.grant.maximum_bytes),
        );
    }
    subject.teardown(failures);
    drop(subject);
    let expected = &vector.expected;
    let close_calls = if registered { expected.registered_close_calls } else { expected.unadmitted_close_calls };
    note(failures, "exact field callback count", counts.close_calls.load(Ordering::SeqCst) == close_calls, counts.close_calls.load(Ordering::SeqCst));
    note(failures, "unstarted accept count", counts.accept_calls.load(Ordering::SeqCst) == expected.accept_calls, counts.accept_calls.load(Ordering::SeqCst));
    note(failures, "unstarted finish count", counts.finish_calls.load(Ordering::SeqCst) == expected.finish_calls, counts.finish_calls.load(Ordering::SeqCst));
    note(failures, "exact field drop count", counts.field_drops.load(Ordering::SeqCst) == expected.field_drops, counts.field_drops.load(Ordering::SeqCst));
    let drops = counts.token_drops.lock().expect("counted terminal drops");
    note(failures, "exact original token drop identity and bytes", drops.len() == expected.token_drops && drops.as_slice() == [(token_identity, vector.token_id, vector.payload.clone())], &*drops);
}

#[test]
fn registered_rejected_pages_obey_zero_short_and_exact_grants() {
    let fixture = fixture();
    let mut failures = Vec::new();
    for row in &fixture.cases {
        check_case(true, row, &fixture.field_owner, &mut failures);
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn unadmitted_rejected_pages_obey_zero_short_and_exact_grants() {
    let fixture = fixture();
    let mut failures = Vec::new();
    for row in &fixture.cases {
        check_case(false, row, &fixture.field_owner, &mut failures);
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
//#endregion 🧪️PageLaws
