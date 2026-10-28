//! 🎚️ Actual styled groups preserve whole-source Unicode resolution and match rustybuzz metrics.
use super::*;

#[test]
fn styled_original_runs_match_portable_ranges_and_rustybuzz() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../📄️paragraph/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut unicode_admission=semio_framework_2d_text_font_shape_unicode::UnicodeDataAdmissionJob::new(semio_framework_2d_text_font_shape_unicode::UNICODE_BYTES.to_vec(),1000000);
    while !unicode_admission.advance(4096,&mut semio_framework_2d_text_font::work::FontWorkLedger::new(semio_framework_2d_text_font_testing::work_grant(4096))).unwrap().done{}
    let unicode=unicode_admission.take_result().unwrap();
    let mut admission=FontFaceAdmissionJob::new(bytes(rows["font"].as_str().unwrap()),FontOutlineLimits::default());
    while !admission.advance(4096,&mut semio_framework_2d_text_font::work::FontWorkLedger::new(semio_framework_2d_text_font_testing::work_grant(4096))).unwrap().done{}
    let fonts=[admission.take_result().unwrap()];
    for row in rows["cases"].as_array().unwrap().iter().filter(|row|!row["text"].as_str().unwrap().contains('\n')) {
        let text=row["text"].as_str().unwrap();
        let size=row["size"].as_f64().unwrap();
        let tracking=row["tracking"].as_f64().unwrap();
        let mut spans=vec![crate::FontRunSpan{start:0,end:text.len(),size,tracking,face:0}];
        spans.extend(row["spans"].as_array().unwrap().iter().map(|span|crate::FontRunSpan{start:span["start"].as_u64().unwrap()as usize,end:span["end"].as_u64().unwrap()as usize,size:span["size"].as_f64().unwrap(),tracking:span["tracking"].as_f64().unwrap(),face:span["face"].as_u64().unwrap()as usize}));
        for grant in [1,7,4096] {
            let mut cursor=crate::FontOutlineRunCursor::new(text,1,crate::FontRunStyle{size,origin:[0.0;2],line_height:1.2,kerning:true,outlines:true},FontOutlineLimits::default(),unicode.layout,&unicode.source,&spans,semio_framework_2d_text_font_shape_unicode::bidi::UnicodeBidiDirection::Auto);
            let mut work=0;
            loop {let progress=cursor.advance(text,&fonts,&unicode.source,&spans,grant,&mut semio_framework_2d_text_font::work::FontWorkLedger::new(semio_framework_2d_text_font_testing::work_grant(grant))).unwrap();assert!(progress.work-work<=grant as u64);work=progress.work;if progress.done{break;}}
            let output=cursor.take_result().unwrap();
            let face=rustybuzz::Face::from_slice(&fonts[0].source,0).unwrap();
            for glyph in output.glyphs.iter() {
                let span=spans.iter().enumerate().rev().find(|(_,span)|span.start<=glyph.source_at&&glyph.source_at<span.end).unwrap();
                assert_eq!(glyph.style,Some(span.0));
                assert_eq!(glyph.size,span.1.size);
                let expected=face.glyph_hor_advance(ttf_parser::GlyphId(glyph.glyph)).unwrap_or(0)as f64*glyph.size/f64::from(fonts[0].face.units_per_em);
                assert!((glyph.advance-expected).abs()<1e-9);
            }
            close(cursor);close(output);
        }
        close(spans);
    }
    close(admission);let [font]=fonts;close(font);close(unicode_admission);close(unicode);
}
