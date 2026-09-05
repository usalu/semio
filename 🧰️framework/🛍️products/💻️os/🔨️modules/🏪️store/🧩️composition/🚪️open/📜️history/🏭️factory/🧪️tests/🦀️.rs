//! 🧪️ Static fixture factories retain the actual paged request without calling open/create.

use super::super::{MemberHistoryInputStep, MemberHistoryVerification};
use super::*;
use crate::os_io::{ArtifactDialect, ArtifactRef};
use crate::os_spr::format::retained::RetainedSprLimits;
use crate::os_store::{MemberOpenRequest, OwnedSchemaDecodeCredits, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OwnerRef, VcsError, OWNED_SCHEMA_DECODE_PAGE_BYTES};
use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};
use serde_json::Value;

const fn declaration(subset: &'static str) -> MemberOpenDeclaration {
    MemberOpenDeclaration { kind: "s.stdio.semio", standard: "v1", subset, schema: "stdio.semio" }
}
const DECLARATIONS: [MemberOpenDeclaration; 18] = [
    declaration("animation"),
    declaration("audio"),
    declaration("brep"),
    declaration("cad"),
    declaration("document"),
    declaration("drawing"),
    declaration("flow"),
    declaration("graph"),
    declaration("image"),
    declaration("kit"),
    declaration("mesh"),
    declaration("model"),
    declaration("object"),
    declaration("presentation"),
    declaration("table"),
    declaration("text"),
    declaration("value"),
    declaration("video"),
];
const fn changed(mode: u8) -> [MemberOpenDeclaration; 18] {
    let mut rows = DECLARATIONS;
    if mode == 0 {
        let mut i = 0;
        while i < 18 {
            rows[i] = DECLARATIONS[17 - i];
            i += 1;
        }
    }
    if mode == 1 {
        rows[17].kind = "";
    }
    if mode == 2 {
        rows[17].schema = "";
    }
    if mode == 3 {
        rows[17].schema = "stdio.\u{85}semio";
    }
    if mode == 4 {
        rows[17].schema = concat!(
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        );
    }
    if mode == 5 {
        rows[17].kind = "stdio.semio";
    }
    if mode == 6 {
        rows[17].standard = "";
    }
    if mode == 7 {
        rows[17].subset = "";
    }
    if mode == 8 {
        rows[17].standard = changed(4)[17].schema;
    }
    if mode == 9 {
        rows[17].subset = "vi\u{85}deo";
    }
    rows
}
const fn duplicated(foreign: bool) -> [MemberOpenDeclaration; 19] {
    let mut rows = [DECLARATIONS[0]; 19];
    let mut i = 0;
    while i < 18 {
        rows[i] = DECLARATIONS[i];
        i += 1;
    }
    rows[18] = if foreign { DECLARATIONS[0] } else { DECLARATIONS[6] };
    if foreign {
        rows[18].schema = "different.schema";
    }
    rows
}
const fn missing() -> [MemberOpenDeclaration; 17] {
    let mut rows = [DECLARATIONS[0]; 17];
    let mut i = 0;
    while i < 17 {
        rows[i] = DECLARATIONS[if i < 6 { i } else { i + 1 }];
        i += 1;
    }
    rows
}

thread_local! { static FACTORY_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
macro_rules! factory {
    ($name:ident, $rows:expr) => {
        struct $name;
        impl MemberFactory for $name {
            const OPEN_DECLARATIONS: &'static [MemberOpenDeclaration] = &$rows;
            type Open = crate::os_store::UnsupportedMemberFactoryOpen<Self>;
            fn begin_open(request: crate::os_store::MemberOpenRequest) -> Result<Self::Open, crate::os_store::MemberOpenAdmissionError> {
                crate::os_store::UnsupportedMemberFactoryOpen::begin(request)
            }
            async fn create(_: &str, _: &ArtifactDialect, _: &[u8]) -> Result<Self, VcsError> {
                FACTORY_CALLS.set(FACTORY_CALLS.get() + 1);
                Err(VcsError::ValidationFailed("selection cannot create".into()))
            }
            async fn open(_: &ArtifactRef, _: Option<&OwnerRef>, _: &[u8]) -> Result<Self, VcsError> {
                FACTORY_CALLS.set(FACTORY_CALLS.get() + 1);
                Err(VcsError::ValidationFailed("selection cannot hydrate".into()))
            }
        }
    };
}
factory!(SemioFixture, DECLARATIONS);
factory!(Reversed, changed(0));
factory!(InvalidKind, changed(1));
factory!(InvalidSchema, changed(2));
factory!(ControlSchema, changed(3));
factory!(OverlongSchema, changed(4));
factory!(NoncanonicalKind, changed(5));
factory!(EmptyStandard, changed(6));
factory!(EmptySubset, changed(7));
factory!(OverlongStandard, changed(8));
factory!(ControlSubset, changed(9));
factory!(Duplicate, duplicated(false));
factory!(ForeignDuplicate, duplicated(true));
factory!(Missing, missing());
factory!(TooMany, [DECLARATIONS[0]; 65]);

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap()
}
fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}
fn selection_history() -> Vec<u8> {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixture/🔣️.json")).unwrap();
    hex(fixture["historyHex"].as_str().unwrap())
}
fn expected_error(row: &Value) -> Option<MemberOpenDiagnostic> {
    row["error"].as_str().map(|value| match value {
        "identity" => MemberOpenDiagnostic::Identity,
        "capacity" => MemberOpenDiagnostic::Capacity,
        "cancelled" => MemberOpenDiagnostic::Cancelled,
        "stale" => MemberOpenDiagnostic::Stale,
        "expired" => MemberOpenDiagnostic::Expired,
        _ => panic!("unrecognized diagnostic"),
    })
}

fn input(fixture: &Value, dialect: &[String], history: &[u8]) -> VerifiedMemberHistoryInput {
    let mut fields: Vec<String> = fixture["requestIdentity"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().into()).collect();
    fields[1..4].clone_from_slice(dialect);
    let expected = ArtifactRef { artifact_id: fields[0].clone(), dialect: ArtifactDialect { artifact_kind: fields[1].clone(), standard: fields[2].clone(), subset: fields[3].clone() } };
    let owner =
        OwnerRef { parent: ArtifactRef { artifact_id: fields[4].clone(), dialect: ArtifactDialect { artifact_kind: fields[5].clone(), standard: fields[6].clone(), subset: fields[7].clone() } }, slot: fields[8].clone(), child_id: fields[9].clone() };
    let mut bytes = vec![1, 170];
    bytes.extend_from_slice(history);
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.len() }).unwrap();
    for chunk in bytes.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let request = MemberOpenRequest::new(OperationId(7), Generation(11), 1000, expected, Some(owner), pages).admit(1).unwrap_or_else(|_| panic!("request admission"));
    let mut verifier = MemberHistoryVerification::new(request, RetainedSprLimits::default()).unwrap_or_else(|_| panic!("verification admission"));
    let mut sequence = 0;
    for _ in 0..10000 {
        let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(4096, 999), root_cancel_token(), || Some(1), &mut sequence);
        match verifier.step(&mut cx) {
            MemberHistoryInputStep::Pending(_) => {}
            MemberHistoryInputStep::Ready => {
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
                return verifier.take_ready(&mut cx).unwrap().unwrap();
            }
            MemberHistoryInputStep::Rejected(error) => panic!("valid framing rejected: {error:?}"),
        }
    }
    panic!("verification did not converge");
}

fn retire(owner: &mut dyn ErasedSnapshotRetirement, grant: usize) -> usize {
    let mut retired = 0;
    assert!(matches!(owner.close_step(0, grant).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    for _ in 0..20000 {
        match owner.close_step(1, grant).unwrap() {
            SnapshotRetirementStep::Complete => {
                assert!(owner.terminal_is_empty());
                return retired;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= grant);
                retired += released_bytes;
            }
            SnapshotRetirementStep::Blocked => panic!("private owner close cannot block"),
        }
    }
    panic!("retirement did not converge");
}

fn run_case<M: MemberFactory>(fixture: &Value, row: &Value, grant: usize) {
    let dialect: Vec<String> = row["dialect"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().into()).collect();
    let input = input(fixture, &dialect, &selection_history());
    assert_eq!(input.retained_input_bytes(), fixture["inputBytes"].as_u64().unwrap() as usize);
    let mut sequence = 0;
    let cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
    let mut owner = match MemberFactorySelection::<M>::begin(input, &cx) {
        Ok(owner) => owner,
        Err(mut rejected) => {
            let diagnostic = rejected.diagnostic;
            let retired = retire(&mut rejected.input, grant);
            assert_eq!(Some(diagnostic), expected_error(row));
            assert_eq!(retired, fixture["caseRetirement"][row["id"].as_str().unwrap()].as_u64().unwrap() as usize);
            return;
        }
    };
    let mut zero = StepContext::new(OperationId(7), Generation(11), StepBudget::new(0, 999), root_cancel_token(), || Some(1), &mut sequence);
    assert!(matches!(owner.step(&mut zero), MemberFactorySelectionStep::Pending(_)));
    assert_eq!(owner.completed, 0);
    let mut error = None;
    let mut terminal = false;
    for _ in 0..10000 {
        let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(grant as u64, 999), root_cancel_token(), || Some(1), &mut sequence);
        let before = owner.completed;
        let step = owner.step(&mut cx);
        assert!(owner.completed - before <= grant as u64 - cx.fuel_remaining());
        match step {
            MemberFactorySelectionStep::Pending(_) => {}
            MemberFactorySelectionStep::Ready => {
                terminal = true;
                break;
            }
            MemberFactorySelectionStep::Rejected(found) => {
                error = Some(found);
                terminal = true;
                break;
            }
        }
    }
    assert!(terminal);
    let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
    let (selected, retired) = if error.is_none() {
        let mut selected = owner.take_ready(&mut cx).unwrap().unwrap();
        let row = serde_json::json!([selected.declaration.kind, selected.declaration.standard, selected.declaration.subset, selected.declaration.schema]);
        assert!(owner.terminal_is_empty());
        assert!(matches!(owner.take_ready(&mut cx), Err(MemberOpenDiagnostic::Stale)));
        (row, retire(&mut selected, grant))
    } else {
        assert!(matches!(owner.take_ready(&mut cx), Err(found) if Some(found) == error));
        (Value::Null, retire(&mut owner, grant))
    };
    assert_eq!(error, expected_error(row), "{}", row["id"]);
    assert_eq!(selected, row["selected"]);
    assert_eq!(retired, fixture["caseRetirement"][row["id"].as_str().unwrap()].as_u64().unwrap() as usize);
}

#[test]
fn member_factory_selection_uses_only_complete_closed_declarations() {
    let fixture = fixture();
    FACTORY_CALLS.set(0);
    let rows = SemioFixture::OPEN_DECLARATIONS.iter().map(|row| serde_json::json!([row.kind, row.standard, row.subset, row.schema])).collect::<Vec<_>>();
    assert_eq!(rows, *fixture["declarations"].as_array().unwrap());
    for row in fixture["cases"].as_array().unwrap() {
        for grant in [1, 7, 4096] {
            match row["mutation"].as_str().unwrap() {
                "none" => run_case::<SemioFixture>(&fixture, row, grant),
                "reverse" => run_case::<Reversed>(&fixture, row, grant),
                "missing" => run_case::<Missing>(&fixture, row, grant),
                "empty" => run_case::<crate::os_store::NoMembers>(&fixture, row, grant),
                "capacity" => run_case::<TooMany>(&fixture, row, grant),
                "duplicate" => run_case::<Duplicate>(&fixture, row, grant),
                "duplicate-foreign" => run_case::<ForeignDuplicate>(&fixture, row, grant),
                "late-kind" => run_case::<InvalidKind>(&fixture, row, grant),
                "late-schema" => run_case::<InvalidSchema>(&fixture, row, grant),
                "late-control" => run_case::<ControlSchema>(&fixture, row, grant),
                "late-length" => run_case::<OverlongSchema>(&fixture, row, grant),
                "late-noncanonical" => run_case::<NoncanonicalKind>(&fixture, row, grant),
                "late-standard" => run_case::<EmptyStandard>(&fixture, row, grant),
                "late-subset" => run_case::<EmptySubset>(&fixture, row, grant),
                "late-standard-length" => run_case::<OverlongStandard>(&fixture, row, grant),
                "late-subset-control" => run_case::<ControlSubset>(&fixture, row, grant),
                _ => panic!("unbound table mutation"),
            }
        }
    }
    for row in fixture["declarations"].as_array().unwrap() {
        let mut complete = fixture.clone();
        complete["caseRetirement"]["all-arm"] = serde_json::json!(fixture["inputBytes"].as_u64().unwrap() + 68 + row[2].as_str().unwrap().len() as u64);
        let row = serde_json::json!({ "id": "all-arm", "dialect": [row[0], row[1], row[2]], "selected": row, "error": null });
        run_case::<SemioFixture>(&complete, &row, 1);
    }
    type Generated = super::super::super::super::tests::RetainedTestMembers;
    let generated = Generated::OPEN_DECLARATIONS.iter().map(|row| serde_json::json!([row.kind, row.standard, row.subset, row.schema])).collect::<Vec<_>>();
    assert_eq!(generated, *fixture["generatedDeclarations"].as_array().unwrap());
    for row in fixture["generatedDeclarations"].as_array().unwrap() {
        let mut complete = fixture.clone();
        complete["caseRetirement"]["generated-arm"] = serde_json::json!(fixture["inputBytes"].as_u64().unwrap() + 68 + row[2].as_str().unwrap().len() as u64);
        let row = serde_json::json!({ "id": "generated-arm", "dialect": [row[0], row[1], row[2]], "selected": row, "error": null });
        run_case::<Generated>(&complete, &row, 1);
    }
    assert_eq!(FACTORY_CALLS.get(), 0);
    println!("[DEBUG] selected factory:18 source-cohort fixture declarations,21 hostile selection cases x3 grants,2 actual space_members macro rows; full table before one retained handoff; open/create calls0");
}

async fn semantic_history(fixture: &Value, row: &Value) -> Vec<u8> {
    use crate::os_spr::history::{encode_history, EncodeOptions, HistoryComposition, HistoryLog};
    let fields = fixture["requestIdentity"].as_array().unwrap();
    let string = |index: usize| fields[index].as_str().unwrap().to_owned();
    let mut log = HistoryLog {
        doc_id: string(0),
        schema: SemioFixture::OPEN_DECLARATIONS[6].schema.into(),
        composition: Some(HistoryComposition { owner: Some((format!("{}!{}@{}/{}", string(4), string(5), string(6), string(7)), string(8), string(9))), dialect: Some((string(1), string(2), string(3))), checkpoint_pins: Vec::new() }),
        ..HistoryLog::default()
    };
    let value = row["value"].as_str().unwrap();
    let composition = log.composition.as_mut().unwrap();
    match row["field"].as_str().unwrap() {
        "none" => {}
        "schema" => log.schema = value.into(),
        "document" => log.doc_id = value.into(),
        "parent" => composition.owner.as_mut().unwrap().0 = value.into(),
        "slot" => composition.owner.as_mut().unwrap().1 = value.into(),
        "child" => composition.owner.as_mut().unwrap().2 = value.into(),
        "kind" => composition.dialect.as_mut().unwrap().0 = value.into(),
        "standard" => composition.dialect.as_mut().unwrap().1 = value.into(),
        "subset" => composition.dialect.as_mut().unwrap().2 = value.into(),
        _ => panic!("unbound persisted identity"),
    }
    encode_history(&log, &EncodeOptions::default()).await.unwrap()
}

fn selected(input: VerifiedMemberHistoryInput) -> SelectedMemberHistoryInput<SemioFixture> {
    let mut sequence = 0;
    let cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
    let mut owner = MemberFactorySelection::<SemioFixture>::begin(input, &cx).unwrap_or_else(|_| panic!("closed factory admission"));
    for _ in 0..10000 {
        let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 999), root_cancel_token(), || Some(1), &mut sequence);
        match owner.step(&mut cx) {
            MemberFactorySelectionStep::Pending(_) => {}
            MemberFactorySelectionStep::Ready => {
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
                return owner.take_ready(&mut cx).unwrap().unwrap();
            }
            MemberFactorySelectionStep::Rejected(error) => panic!("valid selection rejected: {error:?}"),
        }
    }
    panic!("selection did not converge");
}

#[semio_framework_async_macros::async_test]
async fn member_factory_selection_retains_input_through_denial_and_handoff() {
    let fixture = fixture();
    FACTORY_CALLS.set(0);
    for row in fixture["lifecycle"].as_array().unwrap() {
        for grant in [1, 7, 4096] {
            let dialect = ["s.stdio.semio".into(), "v1".into(), "flow".into()];
            let input = input(&fixture, &dialect, &selection_history());
            if row["at"] == "begin" {
                let mut sequence = 0;
                let cancel = root_cancel_token();
                cancel.cancel_now();
                let cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), cancel, || Some(1), &mut sequence);
                let mut rejected = match MemberFactorySelection::<SemioFixture>::begin(input, &cx) {
                    Ok(mut accepted) => {
                        retire(&mut accepted, grant);
                        panic!("cancelled factory admitted");
                    }
                    Err(rejected) => rejected,
                };
                assert_eq!(Some(rejected.diagnostic), expected_error(row));
                assert_eq!(retire(&mut rejected.input, grant), row["retiredBytes"].as_u64().unwrap() as usize);
                continue;
            }
            let mut sequence = 0;
            let cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
            let mut owner = MemberFactorySelection::<SemioFixture>::begin(input, &cx).unwrap_or_else(|_| panic!("fixture factory admission"));
            let at = row["at"].as_str().unwrap();
            if at != "begin" {
                for _ in 0..10000 {
                    let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
                    assert!(!matches!(owner.step(&mut cx), MemberFactorySelectionStep::Rejected(_)));
                    if (at == "selected-unpublished" && owner.selected.is_some()) || (at != "selected-unpublished" && owner.phase == Phase::Complete) {
                        break;
                    }
                }
            }
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
            if at == "selected-unpublished" {
                assert!(owner.take_ready(&mut cx).unwrap().is_none());
                assert!(owner.row < 18);
            }
            let mut witness = if at == "witness" { owner.take_ready(&mut cx).unwrap() } else { None };
            let event = row["event"].as_str().unwrap();
            let cancel = root_cancel_token();
            if event == "cancel" {
                cancel.cancel_now();
            }
            let clock: fn() -> Option<u64> = match event {
                "expired" => || Some(1000),
                "clock-absent" => || None,
                _ => || Some(1),
            };
            let mut cx = StepContext::new(OperationId(if event == "operation" { 8 } else { 7 }), Generation(if event == "generation" { 12 } else { 11 }), StepBudget::new(7, 999), cancel, clock, &mut sequence);
            let before = owner.completed;
            let diagnostic;
            if let Some(witness) = witness.as_mut() {
                diagnostic = witness.check(&cx).err();
            } else if event == "none" {
                witness = owner.take_ready(&mut cx).unwrap();
                diagnostic = None;
            } else {
                diagnostic = match owner.step(&mut cx) {
                    MemberFactorySelectionStep::Rejected(error) => Some(error),
                    _ => None,
                };
                assert_eq!(cx.fuel_remaining(), 7);
            }
            assert_eq!(owner.completed, before);
            assert_eq!(diagnostic, expected_error(row));
            assert_eq!(usize::from(witness.is_some()), row["handoffs"].as_u64().unwrap() as usize);
            let mut retry_sequence = 0;
            let retry = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 999), root_cancel_token(), || Some(1), &mut retry_sequence);
            let retired = if let Some(witness) = witness.as_mut() {
                assert_eq!(witness.check(&retry).err(), diagnostic);
                retire(witness, grant)
            } else {
                assert_eq!(owner.check(&retry).err(), diagnostic);
                retire(&mut owner, grant)
            };
            assert_eq!(retired, row["retiredBytes"].as_u64().unwrap() as usize);
        }
    }
    for row in fixture["semantic"].as_array().unwrap() {
        let history = semantic_history(&fixture, row).await;
        for grant in [1, 7, 4096] {
            let dialect = ["s.stdio.semio".into(), "v1".into(), "flow".into()];
            let input = input(&fixture, &dialect, &history);
            assert_eq!(input.retained_input_bytes(), row["inputBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
            let mut selected = selected(input);
            let mut sequence = 0;
            let mut zero = StepContext::new(OperationId(7), Generation(11), StepBudget::new(0, 999), root_cancel_token(), || Some(1), &mut sequence);
            assert!(selected.begin_dictionary(MemberHistoryDictionaryLimits::default(), &mut zero).unwrap().is_none());
            assert!(!selected.terminal_is_empty());
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
            let mut owner = selected.begin_dictionary(MemberHistoryDictionaryLimits::default(), &mut cx).unwrap().unwrap();
            assert!(selected.terminal_is_empty());
            assert!(matches!(selected.begin_dictionary(MemberHistoryDictionaryLimits::default(), &mut cx), Err(MemberOpenDiagnostic::Stale)));
            let mut error = None;
            let mut terminal = false;
            for _ in 0..20000 {
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(grant as u64, 999), root_cancel_token(), || Some(1), &mut sequence);
                match owner.step(&mut cx) {
                    MemberHistoryDictionaryStep::Pending(_) => {}
                    MemberHistoryDictionaryStep::Ready => {
                        terminal = true;
                        break;
                    }
                    MemberHistoryDictionaryStep::Rejected(found) => {
                        error = Some(found);
                        terminal = true;
                        break;
                    }
                }
            }
            assert!(terminal);
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
            let (handoffs, retired) = if error.is_none() {
                let mut verified = owner.take_ready(&mut cx).unwrap().unwrap();
                assert_eq!(verified.declaration.schema, "stdio.semio");
                assert_eq!(verified.declaration.subset, "flow");
                assert!(owner.terminal_is_empty());
                assert!(matches!(owner.take_ready(&mut cx), Err(MemberOpenDiagnostic::Stale)));
                (1, retire(&mut verified, grant))
            } else {
                assert!(matches!(owner.take_ready(&mut cx), Err(found) if Some(found) == error));
                (0, retire(&mut owner, grant))
            };
            assert_eq!(error, expected_error(row), "{}", row["id"]);
            assert_eq!(handoffs, row["handoffs"].as_u64().unwrap());
            assert_eq!(retired, row["retiredBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
        }
    }
    let history = semantic_history(&fixture, &fixture["semantic"][0]).await;
    for row in fixture["semanticLifecycle"].as_array().unwrap() {
        for grant in [1, 7, 4096] {
            let dialect = ["s.stdio.semio".into(), "v1".into(), "flow".into()];
            let mut selected = selected(input(&fixture, &dialect, &history));
            let mut sequence = 0;
            let event = row["event"].as_str().unwrap();
            let at = row["at"].as_str().unwrap();
            let mut dictionary = None;
            if at == "ready" {
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
                dictionary = selected.begin_dictionary(MemberHistoryDictionaryLimits::default(), &mut cx).unwrap();
                let mut ready = false;
                for _ in 0..20000 {
                    let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(grant as u64, 999), root_cancel_token(), || Some(1), &mut sequence);
                    match dictionary.as_mut().unwrap().step(&mut cx) {
                        MemberHistoryDictionaryStep::Ready => {
                            ready = true;
                            break;
                        }
                        MemberHistoryDictionaryStep::Pending(_) => {}
                        MemberHistoryDictionaryStep::Rejected(error) => panic!("valid semantic history rejected: {error:?}"),
                    }
                }
                assert!(ready);
            }
            let cancel = root_cancel_token();
            if event == "cancel" {
                cancel.cancel_now();
            }
            let mut cx = StepContext::new(OperationId(if event == "operation" { 8 } else { 7 }), Generation(if event == "generation" { 12 } else { 11 }), StepBudget::new(7, 999), cancel, || Some(1), &mut sequence);
            let retired = if let Some(dictionary) = dictionary.as_mut() {
                assert!(matches!(dictionary.take_ready(&mut cx), Err(error) if Some(error) == expected_error(row)));
                assert_eq!(cx.fuel_remaining(), 7);
                let mut retry_sequence = 0;
                let mut retry = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 999), root_cancel_token(), || Some(1), &mut retry_sequence);
                assert!(matches!(dictionary.step(&mut retry), MemberHistoryDictionaryStep::Rejected(error) if Some(error) == expected_error(row)));
                retire(dictionary, grant)
            } else {
                let mut limits = MemberHistoryDictionaryLimits::default();
                if event == "capacity" {
                    limits.dictionary_entries = 8193;
                }
                assert!(matches!(selected.begin_dictionary(limits, &mut cx), Err(error) if Some(error) == expected_error(row)));
                assert_eq!(selected.input.as_ref().unwrap().retained_input_bytes(), 274);
                let mut retry_sequence = 0;
                let mut retry = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 999), root_cancel_token(), || Some(1), &mut retry_sequence);
                assert!(matches!(selected.begin_dictionary(MemberHistoryDictionaryLimits::default(), &mut retry), Err(error) if Some(error) == expected_error(row)));
                retire(&mut selected, grant)
            };
            assert_eq!(retired, row["retiredBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
        }
    }
    assert_eq!(FACTORY_CALLS.get(), 0);
    println!(
        "[DEBUG] selected factory lifecycle:7 selection +5 semantic authority transitions,9 production HistoryLog schema/document/owner/dialect cases x3 grants; selected static schema only, exact close, private one-use handoff; open/create calls0"
    );
}
