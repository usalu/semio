//! 🧪️ Drives the registered native Save program on the large original-image PDF through paid output pages and reopens the result.

use crate::standards::v1_7::subsets::base::{io::import::decode_pdf, schema::snapshot::{PdfObject, PdfSnapshot}};
use crate::{close::ProgramClose, page::admit_pdf_save_page,
    producer::{PdfNativeSaveProgram, PlanDecision}, program::{ClassicPdfSaveProgram, admit_retained_candidate},
    window::write_pdf_original_turn};
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, close_original_effect_step};
use semio_framework_value::{retained_clone::{RetainedCloneClose, RetainedCloneGrant}, retirement::RetireOwned};
use semio_plugin::{ArtifactOutputChunks, MediaConstruction};
use semio_plugin::__job::InteractiveJobCloseStep;

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 1 << 20,
    maximum_release_bytes: 1 << 20, maximum_depth: 4096,
};

fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(884), Generation(7), RetainedCloneGrant {
        maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28,
        maximum_capacity_bytes: 1 << 27, maximum_release_bytes: 1 << 27, maximum_depth: 4096,
    }, semio_framework_job::default_now_us, u64::MAX, 1 << 25)
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
    snapshot.objects.iter().filter_map(|object| match &object.value { PdfObject::Stream { data, .. } => Some(data.clone()), _ => None }).max_by_key(Vec::len).unwrap()
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

#[test]
fn registered_segmented_save_writes_the_large_original_image_and_reopens_exactly() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🏞️large-original-image/🔣️.json")).unwrap();
    let (input, image) = large_original_image_pdf(&law);
    let snapshot = decode_pdf(&input).unwrap();
    let revision = [42; 32];
    let mut source = caller();
    let mut decoder = source.admitted_decoder();
    source.retain_dispatch().unwrap();
    let mut candidate = None;
    let mut birth = MediaConstruction::default();
    assert!((0..64).any(|_| source.turn_with_decoder(TURN, &mut decoder, |cx|
        admit_retained_candidate(&snapshot, revision, &mut candidate, &mut birth, cx)).unwrap()));
    let mut program = ClassicPdfSaveProgram::new(candidate.unwrap(), ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES);
    let ready = (0..16384).find_map(|_| {
        match source.turn_with_decoder(TURN, &mut decoder, |cx| program.prepare_original(&snapshot, revision, cx)).unwrap() {
            PlanDecision::Pending => None,
            PlanDecision::Ready => Some(true),
            PlanDecision::ReferenceRequired => Some(false),
        }
    });
    assert_eq!(ready, Some(true));
    let mut actual = Vec::new();
    let mut page = None;
    let mut construction = MediaConstruction::default();
    let mut page_ready = false;
    let mut done = false;
    for _ in 0..16384 {
        if !page_ready {
            page_ready = source.turn_with_decoder(TURN, &mut decoder, |cx|
                admit_pdf_save_page(&mut page, &mut construction, cx)).unwrap();
            if !page_ready { continue; }
        }
        let buffer = page.as_mut().unwrap();
        if let Some(ready) = source.turn_with_decoder(TURN, &mut decoder, |cx| {
            let result = write_pdf_original_turn(buffer, cx, |sink, copy| {
                let turn = program.emit_original(&snapshot, sink, copy);
                let read = turn.read;
                (turn, read)
            })?;
            let Some(result) = result else { return Ok(None); };
            cx.consume_fuel(result.work as u64);
            Ok(Some(result.result?))
        }).unwrap() {
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
    assert!(done);
    let mut close = ProgramClose::new(program.retire());
    let mut last = None;
    for _ in 0..16384 {
        last = Some(source.turn_with_decoder(TURN, &mut decoder, |cx|
            cx.with_original_retirement(|cx| close.close_step(cx))).unwrap());
        if matches!(last, Some(InteractiveJobCloseStep::Complete { .. })) && close.terminal_is_empty() { break; }
    }
    assert!(close.terminal_is_empty(), "Original graph close did not settle: {:?}", last);
    close_construction(&mut source, &mut decoder, &mut construction);
    close_construction(&mut source, &mut decoder, &mut birth);
    finish(source, &mut decoder);
    if let Some(path) = std::env::var_os("SEMIO_PDF_ORIGINAL_SAVE_OUTPUT") {
        std::fs::write(path, &actual).unwrap();
    }
    if let Some(path) = std::env::var_os("SEMIO_PDF_ORIGINAL_SAVE_OUTPUT") {
        std::fs::write(path, &actual).unwrap();
    }
    assert!(actual.len() > image.len(), "the saved PDF carries the whole image");
    let reopened = decode_pdf(&actual).unwrap();
    assert!(largest_stream(&reopened) == image, "the segmented save keeps exactly the original image samples");
    assert_eq!(reopened.pages, snapshot.pages);
    println!("[DEBUG] segmented native Save wrote and retired {} bytes", actual.len());
}
