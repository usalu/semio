//! 🧪️ Drives the registered native Save program on the large original-image PDF through paid output pages and reopens the result.

use crate::standards::v1_7::subsets::base::{io::import::decode_pdf, schema::snapshot::{PdfObject, PdfOp, PdfSnapshot, PdfTextString}};
use crate::{close::ProgramClose, increment::{OriginalBase, objects_digest}, page::admit_pdf_save_page,
    producer::{PdfNativeSaveProgram, PlanDecision}, program::{ClassicPdfSaveProgram, admit_retained_candidate},
    window::write_pdf_original_turn};
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, close_original_effect_step};
use semio_framework_value::{retained_clone::{RetainedCloneClose, RetainedCloneGrant}, retirement::RetireOwned};
use semio_plugin::{ArtifactOutputChunks, MediaConstruction};
use semio_plugin::__job::InteractiveJobCloseStep;

/// 🔁️ Turn ceiling sized for a 9.44 MB stream written in 4 KiB paid pages.
const TURN_LIMIT: usize = 1 << 22;

const SMALL_TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 16, maximum_copy_bytes: 16_384, maximum_capacity_bytes: 1 << 18,
    maximum_release_bytes: 1 << 18, maximum_depth: 4096,
};

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 1 << 25, maximum_capacity_bytes: 1 << 26,
    maximum_release_bytes: 1 << 20, maximum_depth: 4096,
};

fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(884), Generation(7), RetainedCloneGrant {
        maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28,
        maximum_capacity_bytes: 1 << 27, maximum_release_bytes: 1 << 27, maximum_depth: 4096,
    }, semio_framework_job::default_now_us, u64::MAX, 1 << 40)
}

fn retire<T: RetireOwned>(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, value: T) {
    let (mut value, mut close) = (Some(value), RetainedCloneClose::default());
    while value.is_some() || !close.is_empty() {
        caller.turn_with_decoder(TURN, decoder, |cx| {
            let demand = close.next_owner_demands::<T>(value.is_some(), &None, TURN.maximum_copy_bytes)?;
            close_original_effect_step(cx, demand, |effect| semio_plugin_job_receiving::original_steps::begin(&mut close, &mut value, effect))?;
            Ok(())
        }).unwrap();
    }
}

fn close_construction(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, construction: &mut MediaConstruction) {
    for _ in 0..64 {
        if construction.terminal_is_empty() { return; }
        caller.turn_with_decoder(TURN, decoder, |cx| cx.with_original_retirement(|cx| {
            construction.close_step(cx)?;
            Ok(())
        })).unwrap();
    }
    panic!("PDF output construction did not reach terminal");
}

fn finish(mut caller: OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) {
    for _ in 0..semio_framework_job::ORIGINAL_DECODER_TEARDOWN_STEPS {
        let (quote, _) = semio_framework_job::original_decoder_teardown_quote(decoder).unwrap();
        caller.turn(quote, |cx| { decoder.close_on_source(cx)?; Ok(()) }).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(TURN);
}

fn largest_stream(snapshot: &PdfSnapshot) -> Vec<u8> {
    snapshot.objects.iter().filter_map(|object| match &object.value { PdfObject::Stream { data, .. } => Some(data.to_vec_owner()), _ => None }).max_by_key(Vec::len).unwrap()
}

/// 🏞️ Builds the neutral large one-image PDF and its exact decoded image samples from the schema-first law.
fn large_original_image_pdf(law: &serde_json::Value) -> (Vec<u8>, Vec<u8>) {
    let side = law["imageSide"].as_u64().unwrap() as usize;
    let media = law["mediaSide"].as_u64().unwrap();
    let gray = |index: usize| law["grayscale"][index].as_u64().unwrap() as u8;
    let half = side / 2;
    let rows = |left: u8, right: u8| [vec![left; half], vec![right; half]].concat();
    let (top, bottom) = (rows(gray(0), gray(1)), rows(gray(2), gray(3)));
    let image: Vec<u8> = (0..half).flat_map(|_| top.iter()).chain((0..half).flat_map(|_| bottom.iter())).copied().collect();
    let framed = |dictionary: &str, data: &[u8]| [format!("<< {dictionary} /Length {} >>\nstream\n", data.len()).as_bytes(), data, b"\nendstream"].concat();
    let objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>".to_vec(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {media} {media}] /Contents 4 0 R /Resources << /XObject << /FixtureImage 5 0 R >> >> >>").into_bytes(),
        framed("", format!("q {media} 0 0 {media} 0 0 cm /FixtureImage Do Q\n").as_bytes()),
        framed(&format!("/Type /XObject /Subtype /Image /Width {side} /Height {side} /ColorSpace /DeviceGray /BitsPerComponent 8"), &image),
    ];
    let mut out = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
        out.extend_from_slice(object);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).as_bytes());
    for offset in &offsets { out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes()); }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", offsets.len() + 1).as_bytes());
    (out, image)
}

/// 💾️ The outcome of one driven Save: the bytes written before it ended, whether the writer finished and how many funded turns it spent.
struct SaveRun { bytes: Vec<u8>, done: bool, turns: usize }

/// 💾️ Drives the registered native Save program over one snapshot through paid output pages and returns every written byte.
fn native_save(snapshot: &PdfSnapshot) -> Vec<u8> {
    let run = native_save_with(snapshot, false, None);
    assert!(run.done);
    run.bytes
}

/// 💾️ The same drive with an optional per-turn fuel (small turns) and an optional cancellation after a number of funded turns.
fn native_save_with(snapshot: &PdfSnapshot, small: bool, cancel_after: Option<usize>) -> SaveRun {
    drive_save(snapshot, if small { SMALL_TURN } else { TURN }, cancel_after, None)
}

/// 🌐️ The browser host's per-turn grant: 64 items, 512 KiB of copy, 16 MiB of capacity, 1 MiB of release.
const BROWSER_TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 524_288, maximum_capacity_bytes: 16_777_216,
    maximum_release_bytes: 1_048_576, maximum_depth: 4096,
};

/// 📎️ The retained original bytes of an imported document and the digest of its retained graph as it was opened.
fn base_of(bytes: &[u8], opened: &PdfSnapshot) -> OriginalBase {
    OriginalBase { source: std::sync::Arc::new(bytes.to_vec()), objects: objects_digest(&opened.objects) }
}

fn drive_save(snapshot: &PdfSnapshot, turn: RetainedCloneGrant, cancel_after: Option<usize>, original: Option<OriginalBase>) -> SaveRun {
    let revision = [42; 32];
    let mut source = caller();
    let mut turns = 0usize;
    let mut cancelled = false;
    let mut idle_after_cancel = 0usize;
    let mut decoder = source.admitted_decoder();
    source.retain_dispatch().unwrap();
    let mut candidate = None;
    let mut birth = MediaConstruction::default();
    assert!((0..64).any(|_| source.turn_with_decoder(turn, &mut decoder, |cx|
        admit_retained_candidate(&snapshot, revision, &mut candidate, &mut birth, cx)).unwrap()));
    let mut program = ClassicPdfSaveProgram::new(candidate.unwrap(), ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES);
    if let Some(original) = original { program = program.with_original(original); }
    let ready = (0..TURN_LIMIT).find_map(|_| {
        turns += 1;
        if cancel_after == Some(turns) { source.cancel().cancel_now(); cancelled = true; }
        if cancelled { idle_after_cancel += 1; if idle_after_cancel > 64 { return Some(false); } }
        match source.turn_with_decoder(turn, &mut decoder, |cx| program.prepare_original(&snapshot, revision, cx)) {
            Ok(PlanDecision::Pending) | Err(_) => None,
            Ok(PlanDecision::Ready) => Some(true),
            Ok(PlanDecision::ReferenceRequired) => Some(false),
        }
    });
    assert!(ready == Some(true) || cancelled, "the Save plan settles or is cancelled: {ready:?}");
    let mut actual = Vec::new();
    let mut page = None;
    let mut construction = MediaConstruction::default();
    let mut page_ready = false;
    let mut done = false;
    for _ in 0..TURN_LIMIT {
        if ready != Some(true) { break; }
        turns += 1;
        if cancel_after == Some(turns) { source.cancel().cancel_now(); cancelled = true; }
        if cancelled { idle_after_cancel += 1; if idle_after_cancel > 64 { break; } }
        if !page_ready {
            match source.turn_with_decoder(turn, &mut decoder, |cx| admit_pdf_save_page(&mut page, &mut construction, cx)) {
                Ok(ready) => page_ready = ready,
                Err(_) => {}
            }
            if !page_ready { continue; }
        }
        let buffer = page.as_mut().unwrap();
        if let Ok(Some(ready)) = source.turn_with_decoder(turn, &mut decoder, |cx| {
            let result = write_pdf_original_turn(buffer, cx, |sink, copy| {
                let turn = program.emit_original(&snapshot, sink, copy);
                let read = turn.read;
                (turn, read)
            })?;
            let Some(result) = result else { return Ok(None); };
            cx.consume_fuel(result.work as u64);
            Ok(Some(result.result?))
        }) {
            done = ready;
        }
        if done || buffer.len() == ArtifactOutputChunks::CHUNK_BYTES {
            let original = page.take().unwrap();
            actual.extend_from_slice(&original);
            retire(&mut source, &mut decoder, original);
            page_ready = false;
        }
        if done { break; }
    }
    assert!(done || cancelled, "the Save finishes or is cancelled");
    if let Some(original) = page.take() { if cancelled { drop(original); } else { actual.extend_from_slice(&original); retire(&mut source, &mut decoder, original); } }
    let mut close = ProgramClose::new(program.retire());
    let mut last = None;
    for _ in 0..TURN_LIMIT {
        last = Some(source.turn_with_decoder(TURN, &mut decoder, |cx|
            cx.with_original_retirement(|cx| close.close_step(cx))).unwrap());
        if matches!(last, Some(InteractiveJobCloseStep::Complete { .. })) && close.terminal_is_empty() { break; }
    }
    assert!(close.terminal_is_empty(), "Original graph close did not settle: {:?}", last);
    close_construction(&mut source, &mut decoder, &mut construction);
    close_construction(&mut source, &mut decoder, &mut birth);
    finish(source, &mut decoder);
    SaveRun { bytes: actual, done, turns }
}

#[test]
fn registered_segmented_save_writes_the_large_original_image_and_reopens_exactly() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🏞️large-original-image/🔣️.json")).unwrap();
    let (input, image) = large_original_image_pdf(&law);
    let snapshot = decode_pdf(&input).unwrap();
    let actual = native_save(&snapshot);
    if let Some(path) = std::env::var_os("SEMIO_PDF_ORIGINAL_SAVE_OUTPUT") {
        std::fs::write(path, &actual).unwrap();
    }
    assert!(actual.len() > image.len(), "the saved PDF carries the whole image");
    let reopened = decode_pdf(&actual).unwrap();
    assert!(largest_stream(&reopened) == image, "the segmented save keeps exactly the original image samples");
    assert_eq!(reopened.pages, snapshot.pages);
}

const TWO_PAGES: &[u8] = include_bytes!("../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");

fn by_number(snapshot: &PdfSnapshot) -> Vec<crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject> {
    let mut objects = snapshot.objects.clone();
    objects.sort_by_key(|object| object.id.num);
    objects
}

/// 🧭️ An imported roster keeps its file order; the Save writes it ascending, rotates one page and carries every other retained object unchanged.
#[test]
fn imported_file_order_roster_saves_a_rotation_and_carries_every_other_object() {
    let mut snapshot = decode_pdf(TWO_PAGES).unwrap();
    let numbers: Vec<u32> = snapshot.objects.iter().map(|object| object.id.num).collect();
    assert!(numbers.windows(2).any(|pair| pair[0] > pair[1]), "an imported roster keeps its file order: {numbers:?}");
    snapshot.pages[1].rotate = 90;
    let saved = native_save(&snapshot);
    let reopened = decode_pdf(&saved).unwrap();
    assert_eq!(reopened.pages.len(), 2);
    assert_eq!(reopened.pages[1].rotate, 90);
    assert_eq!(reopened.pages[0], snapshot.pages[0]);
    assert_eq!(reopened.info, snapshot.info);
    let (before, after) = (by_number(&snapshot), by_number(&reopened));
    assert_eq!(before.iter().map(|object| object.id).collect::<Vec<_>>(), after.iter().map(|object| object.id).collect::<Vec<_>>());
    for (original, written) in before.iter().zip(&after) {
        if original.id.num != 7 { assert_eq!(original, written, "object {} is carried over unchanged", original.id.num); }
    }
}

/// 🪂️ A typed edit the ordinary graft cannot spell (new page text) reconciles the whole document once and still writes every untouched retained object unchanged.
#[test]
fn imported_text_edit_falls_back_to_the_reference_reconciliation_and_keeps_retained_objects() {
    let mut snapshot = decode_pdf(TWO_PAGES).unwrap();
    snapshot.info.title = Some("Edited Title".into());
    let font = snapshot.fonts[0].id.clone();
    snapshot.pages[0].content.extend([
        PdfOp::BeginText, PdfOp::SetFont { name: font, size: 12.0 },
        PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, 72.0, 700.0] },
        PdfOp::ShowText { text: PdfTextString::text("Hello") }, PdfOp::EndText,
    ]);
    let saved = native_save(&snapshot);
    let reopened = decode_pdf(&saved).unwrap();
    assert_eq!(reopened.pages.len(), 2);
    assert_eq!(reopened.pages[0].content, snapshot.pages[0].content);
    assert_eq!(reopened.pages[1], snapshot.pages[1]);
    assert_eq!(reopened.info.title.as_deref(), Some("Edited Title"));
    assert_eq!(reopened.info.author, snapshot.info.author);
    assert_eq!(reopened.fonts, snapshot.fonts);
    let (before, after) = (by_number(&snapshot), by_number(&reopened));
    let font_number = before.iter().find(|object| object.value.dict_get("Type").and_then(PdfObject::as_name) == Some("Font")).unwrap().id.num;
    assert_eq!(after.iter().find(|object| object.id.num == font_number), before.iter().find(|object| object.id.num == font_number));
}

/// 🧱️ A neutral flat-tree PDF of `pages` pages with `streams` content streams each, laid out in descending object order so the roster is file-ordered; objects: catalog 1, pages 2, font 3, info 4, then `1 + streams` per page.
fn many_pages_pdf(pages: usize, streams: usize) -> Vec<u8> {
    let per = 1 + streams;
    let first = |page: usize| 5 + page * per;
    let kids = (0..pages).map(|page| format!("{} 0 R", first(page))).collect::<Vec<_>>().join(" ");
    let mut rows: Vec<(usize, Vec<u8>)> = vec![
        (1, b"<< /Type /Catalog /Pages 2 0 R >>".to_vec()),
        (2, format!("<< /Type /Pages /Kids [{kids}] /Count {pages} >>").into_bytes()),
        (3, b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec()),
        (4, b"<< /Title (Generated) /Author (Fixture) >>".to_vec()),
    ];
    for page in 0..pages {
        let contents = (1..=streams).map(|stream| format!("{} 0 R", first(page) + stream)).collect::<Vec<_>>().join(" ");
        rows.push((first(page), format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents [{contents}] >>").into_bytes()));
        for stream in 1..=streams {
            let body = if stream == 1 { format!("BT /F1 12 Tf 72 720 Td (Page {}) Tj ET\n", page + 1) } else { format!("0.5 w {} 40 m {} 60 l S\n", 20 + stream, 200 + page % 50) };
            rows.push((first(page) + stream, [format!("<< /Length {} >>\nstream\n", body.len()).as_bytes(), body.as_bytes(), b"endstream"].concat()));
        }
    }
    rows.reverse();
    let mut out = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();
    let mut offsets = std::collections::BTreeMap::new();
    for (number, body) in &rows {
        offsets.insert(*number, out.len());
        out.extend_from_slice(format!("{number} 0 obj\n").as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }
    let size = rows.len() + 1;
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {size}\n0000000000 65535 f \n").as_bytes());
    for offset in offsets.values() { out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes()); }
    out.extend_from_slice(format!("trailer\n<< /Size {size} /Root 1 0 R /Info 4 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
    out
}

fn typed_hello(snapshot: &mut PdfSnapshot, page: usize) {
    let font = snapshot.fonts[0].id.clone();
    snapshot.pages[page].content.extend([
        PdfOp::BeginText, PdfOp::SetFont { name: font, size: 12.0 },
        PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, 72.0, 700.0] },
        PdfOp::ShowText { text: PdfTextString::text("Hello") }, PdfOp::EndText,
    ]);
}

/// 🧪️ lopdf (the independent reader) must agree with what the Save wrote: page count, per-page rotation and the text operators of the edited page.
fn assert_independent_reader(bytes: &[u8], pages: usize, rotated: Option<(usize, i64)>, text_on: Option<usize>) {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    assert_eq!(oracle::reference_page_count(bytes).unwrap(), pages);
    if let Some((page, rotate)) = rotated {
        let projected: serde_json::Value = serde_json::from_str(&oracle::project_pdf_1_7(bytes).unwrap().to_string()).unwrap();
        assert_eq!(projected["pages"][page]["rotate"], rotate);
        assert_eq!(projected["pages"][0]["rotate"], if page == 0 { rotate } else { 0 });
    }
    if let Some(page) = text_on {
        let operators = oracle::reference_page_operator_names(bytes).unwrap();
        assert!(operators[page].iter().filter(|operator| *operator == "Tj").count() >= 2, "lopdf reads both the original and the inserted text on page {page}");
    }
}

fn dump(name: &str, bytes: &[u8]) {
    if let Some(dir) = std::env::var_os("SEMIO_PDF_LARGE_OUTPUT_DIR") { std::fs::write(std::path::Path::new(&dir).join(name), bytes).unwrap(); }
}

/// 📚️ A flat document of thousands of objects in file order opens, rotates a page on the bounded graft, takes inserted text on the reference route and reopens; lopdf agrees.
#[test]
fn thousands_of_objects_rotate_insert_text_and_reopen() {
    const PAGES: usize = 1600;
    let input = many_pages_pdf(PAGES, 1);
    dump("large-input.pdf", &input);
    let mut snapshot = decode_pdf(&input).unwrap();
    assert!(snapshot.objects.len() > 3000 && snapshot.pages.len() == PAGES);
    assert!(snapshot.objects.windows(2).any(|pair| pair[0].id.num > pair[1].id.num), "the roster is file-ordered");
    snapshot.pages[1000].rotate = 90;
    let rotated = native_save(&snapshot);
    dump("large-rotation.pdf", &rotated);
    assert_independent_reader(&rotated, PAGES, Some((1000, 90)), None);
    let reopened = decode_pdf(&rotated).unwrap();
    assert_eq!(reopened.pages, snapshot.pages);
    assert_eq!(reopened.objects.len(), snapshot.objects.len());
    typed_hello(&mut snapshot, 0);
    let texted = native_save(&snapshot);
    dump("large-text.pdf", &texted);
    assert_independent_reader(&texted, PAGES, Some((1000, 90)), Some(0));
    let reopened = decode_pdf(&texted).unwrap();
    assert_eq!(reopened.pages, snapshot.pages);
    assert_eq!(reopened.pages[1].content, snapshot.pages[1].content);
}

/// 📚️ The existing large Type1 fixture law (48 pages, two streams per page) rotates, takes text and reopens.
#[test]
fn the_large_type1_fixture_rotates_inserts_text_and_reopens() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let pages = law["largeType1"]["pageCount"].as_u64().unwrap() as usize;
    let input = many_pages_pdf(pages, 2);
    let mut snapshot = decode_pdf(&input).unwrap();
    snapshot.pages[10].rotate = 270;
    typed_hello(&mut snapshot, 3);
    let saved = native_save(&snapshot);
    assert_independent_reader(&saved, pages, Some((10, 270)), Some(3));
    assert_eq!(decode_pdf(&saved).unwrap().pages, snapshot.pages);
}

/// 🪜️ Small turn grants (16 KiB of copy instead of 1 MiB) still finish the same Save byte for byte, over many more funded turns.
#[test]
fn a_save_over_many_small_turns_writes_the_same_bytes_as_one_over_large_turns() {
    let mut snapshot = decode_pdf(&many_pages_pdf(40, 1)).unwrap();
    snapshot.pages[7].rotate = 180;
    let large = native_save_with(&snapshot, false, None);
    let small = native_save_with(&snapshot, true, None);
    assert!(large.done && small.done);
    assert_eq!(small.bytes, large.bytes);
    assert!(small.turns >= large.turns, "small turns ({}) never spend fewer turns than large ({})", small.turns, large.turns);
    assert!(small.turns > 200, "a 40 page Save needs many funded turns, took {}", small.turns);
}

/// 🛑️ Cancelling while the plan is being built and while the file is being written ends the Save: it never completes, never writes a closing xref, and every owner retires.
#[test]
fn cancelling_a_large_save_midway_leaves_no_complete_file() {
    let mut snapshot = decode_pdf(&many_pages_pdf(300, 1)).unwrap();
    snapshot.pages[5].rotate = 90;
    let full = native_save_with(&snapshot, false, None);
    assert!(full.done && full.bytes.ends_with(b"%%EOF\n"));
    let during_plan = native_save_with(&snapshot, false, Some(full.turns / 8));
    assert!(!during_plan.done && during_plan.bytes.is_empty(), "no byte is written before the plan is ready");
    let during_write = native_save_with(&snapshot, false, Some(full.turns - 60));
    assert!(!during_write.done && !during_write.bytes.is_empty(), "the cancel lands while pages are being written");
    assert!(during_write.bytes.len() < full.bytes.len());
    assert!(!during_write.bytes.windows(5).any(|window| window == b"%%EOF"), "a cancelled Save never writes its closing trailer");
}


fn lopdf_operators(bytes: &[u8], page: usize) -> Vec<String> {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    oracle::reference_page_operator_names(bytes).unwrap()[page].clone()
}

fn lopdf_pages(bytes: &[u8]) -> usize {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    oracle::reference_page_count(bytes).unwrap()
}

/// 🔢️ The object number of the `index`-th kid of a flat page tree.
fn kid_number(snapshot: &PdfSnapshot, index: usize) -> u32 {
    let find = |number: u32| snapshot.objects.iter().find(|object| object.id.num == number).unwrap();
    let root = snapshot.trailer.iter().find(|entry| entry.key == "Root").and_then(|entry| entry.value.as_ref()).unwrap();
    let pages = find(root.num).value.dict_get("Pages").and_then(PdfObject::as_ref).unwrap();
    match find(pages.num).value.dict_get("Kids") {
        Some(PdfObject::Array(kids)) => kids[index].as_ref().unwrap().num,
        _ => panic!("a flat page tree"),
    }
}

/// 🧪️ Every object of `before` except the `changed` numbers is value-identical in `after`; `after` may add objects.
fn assert_objects_unchanged(before: &PdfSnapshot, after: &PdfSnapshot, changed: &[u32]) {
    let (before, after) = (by_number(before), by_number(after));
    for object in &before {
        if changed.contains(&object.id.num) { continue; }
        assert_eq!(after.iter().find(|candidate| candidate.id == object.id), Some(object), "object {} is carried over unchanged", object.id.num);
    }
}

/// 📎️ A title edit plus inserted text on a two page file append to the original: the original bytes stay an exact prefix, lopdf reads the edit, no other page or object moves.
#[test]
fn incremental_two_pages_title_and_text_append_to_the_original() {
    let opened = decode_pdf(TWO_PAGES).unwrap();
    let mut snapshot = opened.clone();
    snapshot.info.title = Some("Edited Title".into());
    typed_hello(&mut snapshot, 0);
    let run = drive_save(&snapshot, BROWSER_TURN, None, Some(base_of(TWO_PAGES, &opened)));
    assert!(run.done);
    let saved = run.bytes;
    dump("incremental-two-pages.pdf", &saved);
    assert!(saved.starts_with(TWO_PAGES), "the original bytes are an exact prefix of the saved file");
    assert!(saved.len() > TWO_PAGES.len() && saved.len() - TWO_PAGES.len() < 4096, "only the edit is appended, {} bytes", saved.len() - TWO_PAGES.len());
    assert_eq!(lopdf_pages(&saved), 2);
    let before = lopdf_operators(TWO_PAGES, 0).iter().filter(|operator| *operator == "Tj").count();
    assert_eq!(lopdf_operators(&saved, 0).iter().filter(|operator| *operator == "Tj").count(), before + 1);
    assert_eq!(lopdf_operators(&saved, 1), lopdf_operators(TWO_PAGES, 1));
    let reopened = decode_pdf(&saved).unwrap();
    assert_eq!(reopened.info.title.as_deref(), Some("Edited Title"));
    assert_eq!(reopened.info.author, opened.info.author);
    assert_eq!(reopened.pages[0].content, snapshot.pages[0].content);
    assert_eq!(reopened.pages[1], opened.pages[1]);
    assert_eq!(reopened.fonts, opened.fonts);
    let page = kid_number(&opened, 0);
    let info = opened.trailer.iter().find(|entry| entry.key == "Info").and_then(|entry| entry.value.as_ref()).map_or(0, |reference| reference.num);
    assert_objects_unchanged(&opened, &reopened, &[page, info]);
}

/// 🔄️ A rotation is a geometry edit of one page and appends too.
#[test]
fn incremental_rotation_appends_one_page_row() {
    let opened = decode_pdf(TWO_PAGES).unwrap();
    let mut snapshot = opened.clone();
    snapshot.pages[1].rotate = 90;
    let saved = drive_save(&snapshot, BROWSER_TURN, None, Some(base_of(TWO_PAGES, &opened))).bytes;
    assert!(saved.starts_with(TWO_PAGES));
    assert_independent_reader(&saved, 2, None, None);
    let reopened = decode_pdf(&saved).unwrap();
    assert_eq!(reopened.pages[1].rotate, 90);
    assert_eq!(reopened.pages[0], opened.pages[0]);
}

/// 🚪️ A document nothing changed in is saved as the original bytes alone.
#[test]
fn incremental_without_edits_writes_the_original_bytes() {
    let opened = decode_pdf(TWO_PAGES).unwrap();
    let saved = drive_save(&opened, BROWSER_TURN, None, Some(base_of(TWO_PAGES, &opened))).bytes;
    assert_eq!(saved, TWO_PAGES);
}

/// 🧱️ A raw object edit the typed lanes do not carry changes the retained graph's digest, so the Save is written compactly instead of silently losing it.
#[test]
fn a_raw_object_edit_falls_back_to_the_compact_route() {
    let opened = decode_pdf(TWO_PAGES).unwrap();
    let mut snapshot = opened.clone();
    snapshot.objects.push(crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject { id: crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef { num: 99, gen: 0 }, value: PdfObject::Int(7) });
    let saved = drive_save(&snapshot, BROWSER_TURN, None, Some(base_of(TWO_PAGES, &opened))).bytes;
    assert!(!saved.starts_with(TWO_PAGES), "a compact write does not append");
    assert_eq!(decode_pdf(&saved).unwrap().pages.len(), 2);
}

/// 🗜️ An original that spells its table as a stream gets a stream section appended.
#[test]
fn incremental_over_an_xref_stream_original_appends_a_stream_section() {
    use crate::standards::v1_7::subsets::base::{io::export::{EncodeOptions, encode_pdf_with}, modules::writer::WriteOptions};
    let seed = decode_pdf(TWO_PAGES).unwrap();
    let original = encode_pdf_with(&seed, &EncodeOptions { write: WriteOptions { xref_stream: true, encryption: None }, ..Default::default() }).unwrap();
    assert!(original.windows(5).any(|window| window == b"/XRef"));
    let opened = decode_pdf(&original).unwrap();
    let mut snapshot = opened.clone();
    typed_hello(&mut snapshot, 1);
    let saved = drive_save(&snapshot, BROWSER_TURN, None, Some(base_of(&original, &opened))).bytes;
    dump("incremental-xref-stream.pdf", &saved);
    assert!(saved.starts_with(&original));
    assert!(saved[original.len()..].windows(5).any(|window| window == b"/XRef"), "a stream original continues with a stream section");
    assert_eq!(lopdf_pages(&saved), 2);
    assert!(lopdf_operators(&saved, 1).iter().filter(|operator| *operator == "Tj").count() > lopdf_operators(&original, 1).iter().filter(|operator| *operator == "Tj").count());
    assert_eq!(decode_pdf(&saved).unwrap().pages[1].content, snapshot.pages[1].content);
}

/// 📚️ 1,600 pages under the browser's 512 KiB copy grant: a text insert appends a few hundred bytes after the unchanged original over many turns.
#[test]
fn incremental_text_insert_on_1600_pages_completes_under_the_browser_grant() {
    const PAGES: usize = 1600;
    let input = many_pages_pdf(PAGES, 1);
    let opened = decode_pdf(&input).unwrap();
    let mut snapshot = opened.clone();
    typed_hello(&mut snapshot, 0);
    let run = drive_save(&snapshot, BROWSER_TURN, None, Some(base_of(&input, &opened)));
    assert!(run.done);
    dump("incremental-large.pdf", &run.bytes);
    assert!(run.bytes.starts_with(&input));
    assert!(run.bytes.len() - input.len() < 4096, "appended {} bytes", run.bytes.len() - input.len());
    assert!(run.turns > 1600, "one funded turn per page at least, took {}", run.turns);
    assert_independent_reader(&run.bytes, PAGES, None, Some(0));
    let reopened = decode_pdf(&run.bytes).unwrap();
    assert_eq!(reopened.pages, snapshot.pages);
}

/// 🛑️ Cancelling during the page plan or the write publishes nothing: the run never completes and never reaches the final byte.
#[test]
fn cancelling_an_incremental_save_midway_publishes_nothing() {
    let input = many_pages_pdf(300, 1);
    let opened = decode_pdf(&input).unwrap();
    let mut snapshot = opened.clone();
    typed_hello(&mut snapshot, 2);
    let full = drive_save(&snapshot, BROWSER_TURN, None, Some(base_of(&input, &opened)));
    assert!(full.done && full.bytes.starts_with(&input));
    let during_plan = drive_save(&snapshot, BROWSER_TURN, Some(full.turns / 4), Some(base_of(&input, &opened)));
    assert!(!during_plan.done && during_plan.bytes.is_empty(), "no byte is written while pages are still being planned");
    let during_write = drive_save(&snapshot, BROWSER_TURN, Some(full.turns - 20), Some(base_of(&input, &opened)));
    assert!(!during_write.done && during_write.bytes.len() < full.bytes.len());
}

/// 🔁️ A second incremental Save on top of the first appends a second section; both edits survive and the first file is an exact prefix of the second.
#[test]
fn a_second_incremental_save_stacks_on_the_first() {
    let opened = decode_pdf(TWO_PAGES).unwrap();
    let mut first = opened.clone();
    typed_hello(&mut first, 0);
    let one = drive_save(&first, BROWSER_TURN, None, Some(base_of(TWO_PAGES, &opened))).bytes;
    assert!(one.starts_with(TWO_PAGES));
    let reopened = decode_pdf(&one).unwrap();
    let mut second = reopened.clone();
    second.info.title = Some("Second".into());
    typed_hello(&mut second, 1);
    let two = drive_save(&second, BROWSER_TURN, None, Some(base_of(&one, &reopened))).bytes;
    dump("incremental-stacked.pdf", &two);
    assert!(two.starts_with(&one) && two.len() > one.len());
    assert_eq!(lopdf_pages(&two), 2);
    let tj = |bytes: &[u8], page| lopdf_operators(bytes, page).iter().filter(|operator| *operator == "Tj").count();
    assert_eq!(tj(&two, 0), tj(&one, 0));
    assert_eq!(tj(&two, 1), tj(TWO_PAGES, 1) + 1);
    let last = decode_pdf(&two).unwrap();
    assert_eq!(last.info.title.as_deref(), Some("Second"));
    assert_eq!(last.pages[0].content, first.pages[0].content);
    assert_eq!(last.pages[1].content, second.pages[1].content);
}
