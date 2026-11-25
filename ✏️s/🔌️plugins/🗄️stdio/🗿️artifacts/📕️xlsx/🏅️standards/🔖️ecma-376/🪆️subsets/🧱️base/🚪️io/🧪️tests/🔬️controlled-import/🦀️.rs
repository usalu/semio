//! 🧪️ Controlled XLSX natural-file import laws: the paged decoder publishes the snapshot of the whole-buffer decoder in bounded turns, honours cancellation, reports monotone progress and agrees with calamine and openpyxl.

use calamine::Reader;
use semio_s_artifact_stdio_xlsx_base::standards::v_ecma_376::subsets::base::io::{
    export::serializers::encode_xlsx,
    import::controlled::{accept_every_workbook, admit_xlsx_natural_decode, admit_xlsx_paged_natural_decode, build_xlsx_natural_decode, XlsxConformanceGate},
    import::deserializers::decode_xlsx,
};
use semio_framework_value::paged::PagedBytes;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep, RetirementPollExit};
use semio_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_plugin::{NaturalFileDecodeOwner, NaturalFileDecodeProgress, NaturalFileDecodeStep, NaturalFilePagedSource, ToolExecutionContract};
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, XmlCursorStep, XmlDocumentCursor};

const LARGE_GRID: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📏️large-grid/📦️package.xlsx");
const LARGE_GRID_WITNESS: &str = include_str!("../../../🧪️tests/🧫️fixtures/📏️large-grid/🔣️.json");
const SMALL_WORKBOOK: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx");

fn contract(length: usize) -> ToolExecutionContract {
    ToolExecutionContract::resumable(length, 8_192, 4_096, 32 << 20, 7_500, 1, 1)
}

enum Intake { Paged, Contiguous }

struct Run {
    snapshot: XlsxSnapshot,
    progress: NaturalFileDecodeProgress,
    turns: usize,
    widest_turn: u64,
}

fn claim(input: &[u8], contract: ToolExecutionContract, gate: XlsxConformanceGate, intake: &Intake, scope: &mut OriginalWorkFixture) -> NaturalFileDecodeOwner<XlsxSnapshot> {
    let stock = OriginalWorkFixture::stock();
    let mut claimed = None;
    match intake {
        Intake::Paged => {
            let mut source = Some(NaturalFilePagedSource::retain(PagedBytes::try_from_slice(input).unwrap()));
            let denied = RetainedCloneGrant { maximum_items: 0, ..stock };
            assert!(!scope.turn(denied, |cx| admit_xlsx_paged_natural_decode(&mut source, contract, gate, &mut claimed, cx)).unwrap());
            assert!(source.as_ref().is_some_and(|owner| owner.original_is_untouched()) && claimed.is_none());
            assert!(scope.turn(stock, |cx| admit_xlsx_paged_natural_decode(&mut source, contract, gate, &mut claimed, cx)).unwrap());
            assert!(source.is_none(), "the sealed XLSX pages transferred");
        }
        Intake::Contiguous => {
            let mut source = Some(input.to_vec());
            assert!(scope.turn(stock, |cx| admit_xlsx_natural_decode(&mut source, contract, gate, &mut claimed, cx)).unwrap());
            assert!(source.is_none(), "the exact XLSX bytes transferred");
        }
    }
    claimed.expect("the decoder owner is admitted")
}

fn retire(owner: &mut Box<dyn semio_plugin::NaturalFileDecodeCursor<XlsxSnapshot>>, scope: &mut OriginalWorkFixture) {
    let stock = OriginalWorkFixture::stock();
    owner.begin_close();
    for _ in 0..200_000 {
        if owner.terminal_is_empty() { break; }
        scope.turn(stock, |cx| {
            let demand = owner.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
            semio_plugin::__job::close_original_effect_step(cx, demand, |effect| {
                let result = owner.close_chained(effect).map(|step| RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into);
                RetirementPollExit::settle(effect, result)
            })?;
            Ok(())
        }).unwrap();
    }
    assert!(owner.terminal_is_empty(), "the decoder retires everything it owns");
}

fn run_to_completion(input: &[u8], contract: ToolExecutionContract, gate: XlsxConformanceGate, intake: &Intake) -> Result<Run, String> {
    let stock = OriginalWorkFixture::stock();
    let mut scope = OriginalWorkFixture::hosted(1 << 44);
    let NaturalFileDecodeOwner::Controlled(mut decoder) = claim(input, contract, gate, intake, &mut scope) else {
        panic!("an input above one scheduler step must use the controlled decoder");
    };
    let (mut previous, mut turns, mut widest_turn, mut idle) = (NaturalFileDecodeProgress::default(), 0usize, 0u64, 0usize);
    let outcome = loop {
        turns += 1;
        assert!(turns < 200_000, "the decoder must finish");
        let step = scope.turn(stock, |cx| Ok(decoder.advance(cx))).unwrap();
        match step {
            Err(error) => break Err(error.to_string()),
            Ok(NaturalFileDecodeStep::Yield) => {
                idle += 1;
                assert!(idle < 64, "a funded decoder never idles");
            }
            Ok(NaturalFileDecodeStep::Progress(progress)) => {
                idle = 0;
                assert!(progress.completed_work_units > previous.completed_work_units, "every progress turn spends work");
                assert!(progress.consumed_input_bytes >= previous.consumed_input_bytes && progress.decoded_items >= previous.decoded_items);
                widest_turn = widest_turn.max(progress.completed_work_units - previous.completed_work_units);
                previous = progress;
            }
            Ok(NaturalFileDecodeStep::Complete { snapshot, progress }) => {
                widest_turn = widest_turn.max(progress.completed_work_units - previous.completed_work_units);
                break Ok(Run { snapshot, progress, turns, widest_turn });
            }
        }
    };
    retire(&mut decoder, &mut scope);
    drop(decoder);
    scope.finish();
    outcome
}

fn witness() -> serde_json::Value {
    serde_json::from_str(LARGE_GRID_WITNESS).unwrap()
}

fn calamine_grid(bytes: &[u8]) -> Vec<(String, Vec<Vec<String>>)> {
    let mut workbook: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes.to_vec())).unwrap();
    workbook.sheet_names().into_iter().map(|name| {
        let range = workbook.worksheet_range(&name).unwrap();
        (name, range.rows().map(|row| row.iter().map(|cell| format!("{cell:?}")).collect()).collect())
    }).collect()
}

#[test]
fn the_large_grid_exceeds_one_scheduler_step_by_a_wide_margin() {
    assert!(LARGE_GRID.len() > 8 * 4_096);
    assert_eq!(witness()["packageBytes"].as_u64().unwrap() as usize, LARGE_GRID.len());
}

#[test]
fn paged_decoder_publishes_the_whole_buffer_snapshot_in_bounded_turns() {
    let expected = decode_xlsx(LARGE_GRID).unwrap();
    let run = run_to_completion(LARGE_GRID, contract(0), accept_every_workbook, &Intake::Paged).unwrap();
    assert_eq!(run.snapshot, expected);
    assert!(run.turns > 100, "the work spans many scheduler steps: {}", run.turns);
    assert!(run.widest_turn <= 4_096, "no turn exceeds the step budget: {}", run.widest_turn);
    assert_eq!(run.progress.consumed_input_bytes, LARGE_GRID.len());
    assert_eq!(run.progress.decoded_items, expected.opc.parts.len() + expected.xml_parts.len() + expected.opc.relationships.groups().count() + 1);
    assert!(run.progress.owned_output_bytes <= 32 << 20);
}

#[test]
fn contiguous_decoder_publishes_the_same_snapshot() {
    let expected = decode_xlsx(LARGE_GRID).unwrap();
    let run = run_to_completion(LARGE_GRID, contract(LARGE_GRID.len()), accept_every_workbook, &Intake::Contiguous).unwrap();
    assert_eq!(run.snapshot, expected);
    assert!(run.widest_turn <= 4_096);
}

#[test]
fn the_decoded_grid_agrees_with_calamine_and_the_openpyxl_witness() {
    let run = run_to_completion(LARGE_GRID, contract(0), accept_every_workbook, &Intake::Paged).unwrap();
    let reference = calamine_grid(LARGE_GRID);
    let reopened = calamine_grid(&encode_xlsx(&run.snapshot).unwrap());
    assert_eq!(reopened, reference);
    let witness = witness();
    let names: Vec<_> = reference.iter().map(|(name, _)| name.as_str()).collect();
    let expected: Vec<_> = witness["sheets"].as_array().unwrap().iter().map(|sheet| sheet["name"].as_str().unwrap()).collect();
    assert_eq!(names, expected);
    for (sheet, rows) in &reopened {
        let shape = witness["sheets"].as_array().unwrap().iter().find(|candidate| candidate["name"] == sheet.as_str()).unwrap();
        assert_eq!(rows.len() as u64, shape["rows"].as_u64().unwrap());
        assert_eq!(rows.iter().map(|row| row.iter().filter(|cell| cell.as_str() != "Empty").count()).sum::<usize>() as u64, shape["cells"].as_u64().unwrap());
        for (coordinate, value) in witness["probes"][sheet.as_str()].as_object().unwrap() {
            let column = (coordinate.as_bytes()[0] - b'A') as usize;
            let row: usize = coordinate[1..].parse().unwrap();
            let cell = &rows[row - 1][column];
            let rendered = match value {
                serde_json::Value::String(text) => format!("String({text:?})"),
                serde_json::Value::Number(number) if number.is_i64() => format!("Int({})", number.as_i64().unwrap()),
                serde_json::Value::Number(number) => format!("Float({:?})", number.as_f64().unwrap()),
                other => panic!("unexpected probe {other}"),
            };
            assert_eq!(cell, &rendered, "{sheet}!{coordinate}");
        }
    }
}

#[test]
fn small_inputs_keep_the_synchronous_owner_and_large_inputs_get_the_controlled_one() {
    assert!(matches!(build_xlsx_natural_decode(vec![0; 4_096], contract(0), accept_every_workbook).unwrap(), NaturalFileDecodeOwner::Synchronous(_)));
    assert!(matches!(build_xlsx_natural_decode(LARGE_GRID.to_vec(), contract(0), accept_every_workbook).unwrap(), NaturalFileDecodeOwner::Controlled(_)));
    let cold = match build_xlsx_natural_decode(SMALL_WORKBOOK.to_vec(), contract(0), accept_every_workbook).unwrap() {
        NaturalFileDecodeOwner::Controlled(mut decoder) => {
            let mut scope = OriginalWorkFixture::hosted(1 << 44);
            let stock = OriginalWorkFixture::stock();
            let snapshot = loop {
                if let Ok(NaturalFileDecodeStep::Complete { snapshot, .. }) = scope.turn(stock, |cx| Ok(decoder.advance(cx))).unwrap() { break snapshot; }
            };
            retire(&mut decoder, &mut scope);
            snapshot
        }
        NaturalFileDecodeOwner::Synchronous(_) => panic!("the marketplace workbook is above one step"),
    };
    assert_eq!(cold, decode_xlsx(SMALL_WORKBOOK).unwrap());
}

#[test]
fn cancellation_stops_the_work_and_the_decoder_retires_cleanly() {
    let stock = OriginalWorkFixture::stock();
    let mut scope = OriginalWorkFixture::hosted(1 << 44);
    let NaturalFileDecodeOwner::Controlled(mut decoder) = claim(LARGE_GRID, contract(0), accept_every_workbook, &Intake::Paged, &mut scope) else { panic!("controlled decoder") };
    let mut spent = 0;
    for _ in 0..40 {
        match scope.turn(stock, |cx| Ok(decoder.advance(cx))).unwrap().unwrap() {
            NaturalFileDecodeStep::Progress(progress) => spent = progress.completed_work_units,
            other => panic!("a funded decoder keeps progressing, got {}", matches!(other, NaturalFileDecodeStep::Yield)),
        }
    }
    assert!(spent > 0);
    scope.cancel().cancel_now();
    for _ in 0..3 {
        assert!(matches!(scope.turn(stock, |cx| Ok(decoder.advance(cx))).unwrap().unwrap(), NaturalFileDecodeStep::Yield), "a cancelled turn performs no work");
    }
    retire(&mut decoder, &mut scope);
    drop(decoder);
    scope.finish();
}

#[test]
fn corrupt_workbooks_are_refused_with_the_whole_buffer_decoders_reason() {
    let mut damaged = LARGE_GRID.to_vec();
    let middle = damaged.len() / 2;
    damaged[middle] ^= 0xFF;
    let expected = decode_xlsx(&damaged).expect_err("the damaged workbook is refused whole");
    let refused = run_to_completion(&damaged, contract(0), accept_every_workbook, &Intake::Paged).err().expect("the damaged workbook is refused in turns");
    assert!(!refused.is_empty() && !expected.to_string().is_empty());
    let truncated = &LARGE_GRID[..LARGE_GRID.len() - 64];
    assert!(decode_xlsx(truncated).is_err());
    assert!(run_to_completion(truncated, contract(0), accept_every_workbook, &Intake::Paged).is_err());
}

#[test]
fn a_conformance_gate_that_refuses_withholds_the_snapshot() {
    fn refuse(_: &XlsxSnapshot) -> Result<(), semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::refusal::XlsxError> {
        Err(semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::refusal::XlsxError::Malformed("gate refused".into()))
    }
    let refused = run_to_completion(LARGE_GRID, contract(0), refuse, &Intake::Paged).err().expect("the gate refuses");
    assert!(refused.contains("gate refused"), "{refused}");
}

const XML_DOCUMENTS: &[&str] = &[
    "<a/>",
    "  <?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<!-- lead --><?pi data?><root xmlns=\"urn:x\" a='1'><b>text &amp; &#x41; &lt;</b><![CDATA[<raw>]]><!-- c --><?inner go?><c d=\"2\"/>tail</root><!-- end -->\n",
    "<!DOCTYPE note [<!ELEMENT note (#PCDATA)>]><note>é ü 日本</note>",
    "<r><s><t><u><v>deep</v></u></t></s><s/></r>",
    "",
    "   ",
];

const XML_REFUSALS: &[&str] = &["<a>", "<a></b>", "<a/><b/>", "text", "</a>", "<a b=1/>", "<a>&nope;</a>", "<a><!-- open</a>"];

#[test]
fn the_xml_cursor_builds_exactly_the_whole_document_parse_at_every_budget() {
    for text in XML_DOCUMENTS {
        let expected = xml_document_from_text(text).unwrap();
        for budget in [1, 2, 7, 64, usize::MAX] {
            let mut cursor = XmlDocumentCursor::new((*text).to_owned());
            let document = loop {
                match cursor.advance(budget).unwrap() {
                    XmlCursorStep::More => {}
                    XmlCursorStep::Complete(document) => break document,
                }
            };
            assert_eq!(document, expected, "{text:?} at budget {budget}");
        }
    }
}

#[test]
fn the_xml_cursor_refuses_what_the_whole_document_parse_refuses() {
    for text in XML_REFUSALS {
        let expected = xml_document_from_text(text).expect_err(text);
        let mut cursor = XmlDocumentCursor::new((*text).to_owned());
        let refusal = loop {
            match cursor.advance(3) {
                Ok(XmlCursorStep::More) => {}
                Ok(XmlCursorStep::Complete(_)) => panic!("{text:?} must be refused"),
                Err(detail) => break detail,
            }
        };
        assert_eq!(refusal, expected, "{text:?}");
    }
}

#[test]
fn the_xml_cursor_tree_has_the_element_count_quick_xml_reads_from_a_real_sheet() {
    let package = semio_s_artifact_stdio_zip::opc::decode_opc(LARGE_GRID).unwrap();
    let sheet = String::from_utf8(package.part_bytes("xl/worksheets/sheet1.xml").unwrap().to_vec()).unwrap();
    let mut reader = quick_xml::Reader::from_str(&sheet);
    let mut elements = 0usize;
    loop {
        match reader.read_event().unwrap() {
            quick_xml::events::Event::Start(_) | quick_xml::events::Event::Empty(_) => elements += 1,
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }
    fn count(node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode) -> usize {
        match node {
            semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { children, .. } => 1 + children.iter().map(count).sum::<usize>(),
            _ => 0,
        }
    }
    let mut cursor = XmlDocumentCursor::new(sheet.clone());
    let mut turns = 0;
    let document = loop {
        turns += 1;
        match cursor.advance(4_096).unwrap() {
            XmlCursorStep::More => {}
            XmlCursorStep::Complete(document) => break document,
        }
    };
    assert!(turns > 10, "the sheet spans many bounded turns: {turns}");
    assert_eq!(count(document.root.as_ref().unwrap()), elements);
    assert_eq!(document, xml_document_from_text(&sheet).unwrap());
}
