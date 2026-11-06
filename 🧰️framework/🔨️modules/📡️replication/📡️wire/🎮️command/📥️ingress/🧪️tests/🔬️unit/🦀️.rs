use super::*;
use std::mem::size_of;

#[test]
fn retained_batch_descriptor_has_no_destructor() {
    assert!(!std::mem::needs_drop::<CommandBatchEntry>());
}

//#region 📥️CommandIngressPages
/// 📥️ The cross-language paged command-ingress contract — the same rows the TypeScript host writer's
/// twin reads (`🎭️actor/📮️shard-client/🧪️tests/📥️command-ingress-pages/🟦️.ts`).
#[derive(serde::Deserialize)]
struct CommandIngressPagesFixture {
    #[serde(rename = "pageBytes")]
    page_bytes: usize,
    #[serde(rename = "commandMaximumBytes")]
    command_maximum_bytes: usize,
    #[serde(rename = "maximumPages")]
    maximum_pages: usize,
    rows: Vec<CommandIngressPagesRow>,
    refusals: Vec<CommandIngressPagesRefusal>,
}

#[derive(serde::Deserialize)]
struct CommandIngressPagesRow {
    name: String,
    bytes: usize,
    pages: usize,
}

#[derive(serde::Deserialize)]
struct CommandIngressPagesRefusal {
    name: String,
    bytes: usize,
}

fn command_ingress_pages_fixture() -> CommandIngressPagesFixture {
    serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/📄️pages/🔣️.json")).expect("📥️command-ingress-pages fixture parses")
}

/// 🧱️ A deterministic command body — every byte a function of its offset, so a reassembly that
/// duplicates, drops or reorders one page cannot compare equal.
fn command_ingress_body(bytes: usize) -> Vec<u8> {
    (0..bytes).map(|offset| (offset % 251 + 1) as u8).collect()
}

/// ⚖️ LAW: the transport's page and byte authorities are DERIVED from the guest linear-memory budget,
/// and the spine that assembles a ceiling-sized command stays inside one guest growth unit.
///
/// 🧨️ This is the law the 64-page constant failed on both sides at once: it refused a 272 089-byte
/// contributions pack outright, and the reservation it bought was 262 272 contiguous bytes — four
/// times what a fragmented guest can be relied on to serve (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_command_ingress_authorities_are_derived_from_the_guest_memory_budget() {
    let fixture = command_ingress_pages_fixture();
    assert_eq!(COMMAND_PAGE_MAXIMUM_BYTES, fixture.page_bytes);
    assert_eq!(COMMAND_MAXIMUM_BYTES, fixture.command_maximum_bytes);
    assert_eq!(COMMAND_MAXIMUM_PAGES, fixture.maximum_pages);
    assert_eq!(COMMAND_MAXIMUM_BYTES, semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES, "a command is an assembled host answer and is bound by that budget, not by one of its own");
    assert_eq!(COMMAND_MAXIMUM_PAGES, COMMAND_MAXIMUM_BYTES / COMMAND_PAGE_MAXIMUM_BYTES);
    assert!(
        CommandPageSet::reservation_bytes(COMMAND_MAXIMUM_PAGES) <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "a ceiling-sized page authority reserves {} B — over the {} B a routine per-command guest path may ask a fixed linear memory for",
        CommandPageSet::reservation_bytes(COMMAND_MAXIMUM_PAGES),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
    );
    eprintln!(
        "command ingress authorities: pages={COMMAND_MAXIMUM_PAGES} bytes={COMMAND_MAXIMUM_BYTES} slot={} B spine={} B ceiling={} B",
        size_of::<FixedCommandPage>(),
        CommandPageSet::reservation_bytes(COMMAND_MAXIMUM_PAGES),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
    );
    assert!(
        size_of::<FixedCommandPage>() < COMMAND_PAGE_MAXIMUM_BYTES,
        "a page slot must be a handle to its block, never the block — an inline page is what forced a fixed page ceiling"
    );
}

/// 🔪️ LAW: every fixture row splits into exactly the pages it declares and reassembles to the EXACT
/// bytes it was cut from, with no page of a multi-page command left short of the page extent.
#[test]
fn every_command_ingress_row_reassembles_to_the_exact_bytes_it_was_cut_from() {
    let fixture = command_ingress_pages_fixture();
    for row in &fixture.rows {
        let body = command_ingress_body(row.bytes);
        let cut: Vec<&[u8]> = body.chunks(COMMAND_PAGE_MAXIMUM_BYTES).collect();
        assert_eq!(cut.len(), row.pages, "row {} cuts into {} pages, not the {} it declares", row.name, cut.len(), row.pages);
        let mut pages = CommandPageSet::try_new(row.pages).unwrap_or_else(|fault| panic!("row {} declares a page authority: {fault:?}", row.name));
        assert!(
            CommandPageSet::reservation_bytes(row.pages) <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
            "row {} reserves {} B for its spine",
            row.name,
            CommandPageSet::reservation_bytes(row.pages)
        );
        for (index, chunk) in cut.iter().enumerate() {
            assert!(index + 1 == cut.len() || chunk.len() == COMMAND_PAGE_MAXIMUM_BYTES, "row {} left a nonterminal page short", row.name);
            pages.try_push(FixedCommandPage::try_copy_from(chunk).expect("a cut page")).unwrap_or_else(|(fault, _)| panic!("row {} admits page {index}: {fault:?}", row.name));
        }
        let command = PagedCommand::try_from_pages(pages).unwrap_or_else(|(fault, _)| panic!("row {} assembles: {fault:?}", row.name));
        assert_eq!(command.page_len(), row.pages);
        assert_eq!(command.byte_len(), row.bytes);
        assert_eq!(command.kind(), body[0]);
        let mut reader = PagedCommandReader::new(command);
        let mut read = Vec::with_capacity(row.bytes);
        for offset in 0..row.bytes {
            read.push(reader.read_byte().unwrap_or_else(|fault| panic!("row {} ends inside byte {offset}: {fault:?}", row.name)));
        }
        assert_eq!(read, body, "row {} did not reassemble to its own bytes", row.name);
        assert!(reader.terminal_is_empty(), "row {} left pages behind after its last byte", row.name);
        assert_eq!(reader.read_byte().expect_err("a fully read command has no further byte").code.0, "plugin.command-decode-truncated");
    }
}

/// 📃️ LAW: an over-declaration and an over-push are refused with a `Fault` the host can display, and
/// the page a saturated authority refuses is handed BACK rather than dropped.
#[test]
fn every_command_ingress_refusal_answers_a_fault_and_keeps_its_page() {
    let fixture = command_ingress_pages_fixture();
    for refusal in &fixture.refusals {
        let declared = refusal.bytes.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES);
        assert_eq!(
            CommandPageSet::try_new(declared).expect_err(&format!("refusal {} has no page authority", refusal.name)).code.0,
            "plugin.command-page-count",
            "refusal {} must be refused by the declaration, before one page is copied",
            refusal.name
        );
    }
    let mut pages = CommandPageSet::try_new(2).expect("a two-page authority");
    for _ in 0..2 {
        pages.try_push(FixedCommandPage::try_copy_from(&[7; COMMAND_PAGE_MAXIMUM_BYTES]).expect("a full page")).expect("the declared pages are admitted");
    }
    let (fault, returned) = pages.try_push(FixedCommandPage::try_copy_from(b"third").expect("a page")).expect_err("a third page is over the declared authority");
    assert_eq!(fault.code.0, "plugin.command-page-count");
    assert_eq!(returned.as_slice(), b"third", "a refused page is handed back, never dropped");
}

//#endregion 📥️CommandIngressPages

#[test]
fn command_page_input_moves_only_after_actual_page_admission(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📥️borrow/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let mut pages=CommandPageSet::try_new(case["declared"].as_u64().unwrap()as usize).unwrap();
  for _ in 0..case["occupied"].as_u64().unwrap(){pages.try_push(FixedCommandPage::try_copy_from(&[1]).unwrap()).unwrap();}
  let cursor=CommandPageCursor{owner:7,generation:11,command_index:0,command_count:1,instance:13,seq:17,kind:19,page_index:0,page_count:1,item_count:0,metadata:0};
  let bytes=vec![19;case["bytes"].as_u64().unwrap()as usize];
  let mut input=case["incoming"].as_bool().unwrap().then(||(cursor,FixedCommandPage::try_copy_from(&bytes).unwrap()));
  let original=input.as_ref().map(|(_,page)|page.as_slice().as_ptr());
  let accepted=pages.try_push_input(&mut input).is_ok_and(|cursor|cursor.is_some());
  assert_eq!(accepted,case["accepted"].as_bool().unwrap());
  assert_eq!(input.is_some(),case["remaining"].as_bool().unwrap());
  if accepted{assert_eq!(pages.pages.back().unwrap().as_slice().as_ptr(),original.unwrap());assert_eq!(pages.pages.back().unwrap().as_slice(),bytes)}
  else if let Some((_,page))=input.as_ref(){assert_eq!(Some(page.as_slice().as_ptr()),original);assert_eq!(page.as_slice(),bytes)}
 }
}

#[test]
fn command_page_sources_return_real_backing_only_under_funded_retirement(){
 use semio_framework_value::{RetainedCloneGrant,retirement::controlled::ControlledRetirement};
 for count in 0..=2{
  let mut pages=CommandPageSet::try_new(2).unwrap();
  for _ in 0..count{pages.try_push(FixedCommandPage::try_copy_from(&[19]).unwrap()).unwrap();}
  let existing=pages.pages.capacity()*size_of::<FixedCommandPage>()+count*COMMAND_PAGE_MAXIMUM_BYTES;
  let mut owner=ControlledRetirement::retain(pages);let mut born=0;let mut released=0;
  for at in 0..10000{
   if owner.terminal_is_empty(){break}
   let d=owner.next_demand(4096).unwrap();
   let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(g));let p=result.unwrap().progress();
   assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(at<9999);
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,existing+born);
 }
}
