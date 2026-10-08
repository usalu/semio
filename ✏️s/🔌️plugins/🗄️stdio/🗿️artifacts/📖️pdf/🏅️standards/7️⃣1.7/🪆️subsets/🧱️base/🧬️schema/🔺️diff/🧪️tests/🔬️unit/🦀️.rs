use super::*;
use crate::standards::v1_7::subsets::base::io::{text_document, PdfTextLayout};
use protocol::{DiffBinary,DiffCodec,DiffText};

fn base() -> PdfSnapshot {
    let mut snapshot = text_document(&[(200.0, 300.0, "one"), (200.0, 300.0, "two")]);
    snapshot.images.push(PdfImage::gray8("Im1", 1, 1, vec![7]));
    snapshot.outlines.push(PdfOutlineItem::to_page("Start", 0));
    snapshot.language = Some("en".into());
    snapshot
}

fn edited() -> PdfSnapshot {
    let mut snapshot = base();
    snapshot.pages[0].content.push(PdfOp::Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
    snapshot.pages[0].content.push(PdfOp::Fill);
    snapshot.pages[0].rotate = 90;
    snapshot.pages[0].crop_box = Some([1.0, 1.0, 2.0, 2.0]);
    snapshot.pages.remove(1);
    snapshot.pages.push(PdfPage::new(50.0, 50.0));
    snapshot.fonts[0].to_unicode = Some(PdfToUnicode { byte_width: 1, mappings: vec![PdfToUnicodeMapping::Char { code: 65, text: "A".into() }] });
    snapshot.images.push(PdfImage::gray8("Im2", 1, 1, vec![9]));
    snapshot.images.remove(0);
    snapshot.outlines[0].title = "Begin".into();
    snapshot.language = None;
    snapshot.page_mode = Some(PdfPageMode::UseOutlines);
    snapshot.info.title = Some("Edited".into());
    snapshot.catalog_extra.push(PdfDictEntry::new("Marker", PdfObject::Int(1)));
    snapshot
}

#[test]
fn builders_produce_sparse_patches() {
    let a = base();
    let layout = PdfTextLayout::new(&a.fonts[0],10.0,&crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources::from_objects(&a.objects)).unwrap();
    let ops = layout.show_lines(&["more".to_string()], 10.0, 100.0);
    let diff = diff_append_page_content(&a, 0, &ops);
    let next = protocol::apply_diff(&diff, &a).unwrap();
    assert_eq!(next.pages[0].content.len(), a.pages[0].content.len() + ops.len());
    let removed = protocol::apply_diff(&diff_remove_content(0, 0, 2), &next).unwrap();
    assert_eq!(removed.pages[0].content.len(), next.pages[0].content.len() - 2);
    let font = protocol::apply_diff(&diff_set_font(&a, PdfFont::standard("F2", "Courier"), None), &a).unwrap();
    assert_eq!(font.fonts.len(), 2);
    assert!(diff_set_font(&a, a.fonts[0].clone(), None).is_empty());
    let without = protocol::apply_diff(&diff_remove_font(&font, "F2"), &font).unwrap();
    assert_eq!(without.fonts, a.fonts);
    let boxed = protocol::apply_diff(&diff_set_page_box(0, PdfPageBox::Trim, Some([0.0, 0.0, 5.0, 5.0])), &a).unwrap();
    assert_eq!(boxed.pages[0].trim_box, Some([0.0, 0.0, 5.0, 5.0]));
    let annotated = protocol::apply_diff(&diff_insert_annotation(0, 0, PdfAnnotation::link([0.0, 0.0, 1.0, 1.0], "x")), &a).unwrap();
    assert_eq!(annotated.pages[0].annotations.len(), 1);
    let mode = protocol::apply_diff(&diff_set_page_mode(&a, Some(PdfPageMode::FullScreen)), &a).unwrap();
    assert_eq!(mode.page_mode, Some(PdfPageMode::FullScreen));
    assert!(diff_set_page_mode(&mode, Some(PdfPageMode::FullScreen)).is_empty());
    let cleared = protocol::apply_diff(&diff_set_language(&a, None), &a).unwrap();
    assert_eq!(cleared.language, None);
}

#[test]
fn validation_rejects_out_of_range_targets() {
    let a = base();
    assert!(protocol::apply_diff(&diff_remove_page(7), &a).is_err());
    assert!(protocol::apply_diff(&diff_remove_content(0, 99, 1), &a).is_err());
    assert!(diff_remove_font(&a, "nope").is_empty());
    let mut duplicate = PdfDiff::default();
    duplicate.fonts = Some(PdfKeyedDiff { added: vec![PdfIndexedItem { index: 0, value: a.fonts[0].clone() }], ..Default::default() });
    assert!(protocol::apply_diff(&duplicate, &a).is_err(), "adding an existing key is rejected");
}

#[test]
fn a_graph_edit_carries_the_typed_lanes_it_moves_and_an_unmarked_one_does_not() {
    use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
    let base = support::document();
    let rows = support::set_catalog_entry_rows(&base, "PageMode", PdfObject::Name("UseOutlines".to_string()), None);
    let marked = protocol::apply_diff(&graph_edit(rows.clone()), &base).unwrap();
    assert_eq!(support::catalog_entry(&marked, "PageMode"), Some(&PdfObject::Name("UseOutlines".to_string())));
    assert_eq!(marked.page_mode, Some(PdfPageMode::UseOutlines), "the page-mode lane reads the edited catalog");
    let unmarked = protocol::apply_diff(&rows, &base).unwrap();
    assert_eq!(unmarked.page_mode, base.page_mode, "rows without the mark leave the typed lane alone");
    assert_eq!(protocol::apply_diff(&graph_edit(rows.clone()).inverse(&base), &marked).unwrap(), base);
    assert!(graph_edit(PdfDiff::default()) == PdfDiff::default(), "a diff without graph rows stays unmarked");
    let mut absorbed = graph_edit(rows);
    absorbed.absorb(graph_edit(support::remove_catalog_entry_rows(&marked, "PageMode")));
    assert!(absorbed.is_empty(), "set then remove of the same entry cancels, and the mark with it: {absorbed:?}");
}

#[test]
fn the_info_patch_is_field_wise_and_coalesces_per_field() {
    let base = base();
    let mut title = PdfDiff { info: Some(PdfInfoDiff { title: Some(PdfSet::Set { value: "T".into() }), ..Default::default() }), ..Default::default() };
    let author = PdfDiff { info: Some(PdfInfoDiff { author: Some(PdfSet::Set { value: "A".into() }), ..Default::default() }), ..Default::default() };
    title.absorb(author);
    let next = protocol::apply_diff(&title, &base).unwrap();
    assert_eq!((next.info.title.as_deref(), next.info.author.as_deref()), (Some("T"), Some("A")));
    assert_eq!(protocol::apply_diff(&title.inverse(&base), &next).unwrap(), base);
    assert_eq!(diff_set_info(&base, &base.info), PdfDiff::default(), "an unchanged info record is no change");
}

