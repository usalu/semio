use super::*;

fn reference(num: u32) -> PdfObject {
    PdfObject::Ref(ObjRef { num, gen: 0 })
}

fn id(num: u32) -> ObjRef {
    ObjRef { num, gen: 0 }
}

#[test]
fn an_insertion_lands_each_object_at_the_next_free_number_and_the_end_of_the_list() {
    let base = document_of(vec![PdfObject::Int(1), PdfObject::Int(2)]);
    let mut insertion = Insertion::after(&base, &[]);
    let first = insertion.push(PdfObject::Int(3));
    let second = insertion.push(PdfObject::Int(4));
    assert_eq!((first, second), (id(3), id(4)));
    let next = protocol::apply_diff(&insertion.rows(), &base).expect("the insertion applies");
    assert_eq!(next.objects.iter().map(|object| object.id).collect::<Vec<_>>(), vec![id(1), id(2), id(3), id(4)]);
    assert_eq!(Insertion::after(&base, &[]).rows(), PdfDiff::default(), "nothing pushed is no change");
}

#[test]
fn owned_objects_stop_at_anything_another_object_still_references() {
    let base = document_of(vec![dict(vec![("Root", reference(2))]), dict(vec![("Kid", reference(3)), ("Own", reference(5))]), PdfObject::Int(9), dict(vec![("Also", reference(3))]), PdfObject::Int(7)]);
    let owned = owned_objects(&base, &reference(2));
    assert_eq!(owned, vec![id(2), id(5)], "the shared child 3 stays, the exclusive child 5 goes with its parent");
}

#[test]
fn removing_a_catalog_entry_takes_what_it_exclusively_installed_with_it() {
    let catalog = document_of(vec![catalog_object()]);
    let base = after_rows(&catalog, output_intent_rows(&catalog, "GTS_PDFA1", "sRGB", true, &[], None));
    assert_eq!(base.objects.len(), 3);
    let next = after_rows(&base, remove_catalog_entry_owned_rows(&base, "OutputIntents"));
    assert_eq!(next, catalog, "the intent, its profile stream and the entry leave together");
    assert_eq!(remove_catalog_entry_owned_rows(&catalog, "OutputIntents"), PdfDiff::default(), "an absent entry removes nothing");
}

#[test]
fn embedding_a_font_program_leaves_exactly_one_program_key() {
    let base = document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string())), ("FontFile", reference(1))])]);
    let next = after_rows(&base, embed_font_file_rows(&base, id(2), "FontFile2", id(1), None));
    assert_eq!(font_program(&next, id(2)), Some(("FontFile2".to_string(), id(1))));
    assert!(object(&next, id(2)).and_then(|value| value.dict_get("FontFile")).is_none(), "the previous program key is dropped");
    assert_eq!(embed_font_file_rows(&next, id(2), "FontFile2", id(1), None), PdfDiff::default(), "embedding what is already there is no change");
}

#[test]
fn an_insertion_honours_the_placements_it_is_given_in_final_position_order() {
    let base = document_of(vec![PdfObject::Int(1), PdfObject::Int(2)]);
    let mut insertion = Insertion::after(&base, &[placed(4, 2), placed(3, 1)]);
    assert_eq!((insertion.push(PdfObject::Int(4)), insertion.push(PdfObject::Int(3))), (id(4), id(3)));
    let next = protocol::apply_diff(&insertion.rows(), &base).expect("the insertion applies");
    assert_eq!(next.objects.iter().map(|object| object.id.num).collect::<Vec<_>>(), vec![1, 3, 4, 2]);
}

#[test]
fn placements_name_the_current_position_or_nothing() {
    let base = document_of(vec![PdfObject::Int(1), PdfObject::Int(2), PdfObject::Int(3)]);
    assert_eq!(placements_of(&base, &[id(3), id(1)]), vec![placed(3, 2), placed(1, 0)]);
    assert_eq!(placements_of(&base, &[id(3), id(9)]), Vec::new(), "one missing object leaves the builder on fresh placements");
}

#[test]
fn an_entry_lands_at_the_position_it_is_given_and_reports_its_position() {
    let base = document_of(vec![dict(vec![("A", PdfObject::Int(1)), ("C", PdfObject::Int(3))])]);
    let next = after_rows(&base, set_entry_rows(&base, id(1), "B", PdfObject::Int(2), Some(1)));
    assert_eq!(entry_position(&next, id(1), "B"), Some(1));
    assert_eq!(entry_position(&next, id(1), "C"), Some(2));
    assert_eq!(entry_position(&next, id(1), "Z"), None);
    let appended = after_rows(&base, set_entry_rows(&base, id(1), "B", PdfObject::Int(2), None));
    assert_eq!(entry_position(&appended, id(1), "B"), Some(2));
}

#[test]
fn creation_ids_follow_the_order_their_builders_create_in() {
    let catalog = document_of(vec![catalog_object()]);
    let intent = after_rows(&catalog, output_intent_rows(&catalog, "GTS_PDFA1", "sRGB", true, &[], None));
    assert_eq!(output_intent_creation_ids(&intent), vec![id(2), id(3)], "profile stream first, intent second");
    let spec = after_rows(&catalog, insert_file_spec_rows(&catalog, "a.csv", &[]));
    assert_eq!(file_spec_creation_ids(&spec, id(3)), vec![id(2), id(3)], "attached stream first, spec second");
    let root = after_rows(&catalog, dpart_root_rows(&catalog, "job", &[], None));
    assert_eq!(dpart_root_creation_ids(&root), vec![id(2), id(3)], "node first, root second");
}
