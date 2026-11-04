use crate::apply_mutation;
use crate::schema::mutations::{BmpMutation, PaintDirectRegion, PaintIndexedRegion};
use protocol::{ColdMutation, Mutation};
use semio_framework_test_fixtures::fixture_bytes;

fn direct() -> crate::BmpSnapshot {
    crate::standards::v_v3::subsets::any::io::decode_bmp(fixture_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")).unwrap()
}

fn indexed() -> crate::BmpSnapshot {
    crate::standards::v_v3::subsets::any::io::decode_bmp(fixture_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb4-duplicate-palette.bmp")).unwrap()
}

#[test]
fn direct_region_mutation_is_revision_guarded_and_exactly_invertible() {
    let original = direct();
    let mutation = BmpMutation::PaintDirectRegion(PaintDirectRegion {
        revision: crate::standards::v_v3::subsets::any::schema::operations::bmp_revision(&original),
        x: 1,
        y: 0,
        width: 1,
        height: 1,
        red: 9,
        green: 8,
        blue: 7,
        alpha: 6,
    });
    let inverse = mutation.inverse(&original).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1);
    assert!(matches!(&inverse[0], BmpMutation::ReplaceSamples(restore) if restore.region == crate::schema::snapshot::BmpRegion { x: 1, y: 0, width: 1, height: 1 }));
    let mut edited = original.clone();
    let outcome = apply_mutation(&mut edited, &mutation);
    assert!(outcome.diff().image.is_none());
    assert_eq!(outcome.diff().rects.len(), 1);
    assert_ne!(edited, original);
    apply_mutation(&mut edited, &inverse[0]);
    assert_eq!(edited, original);

    let stale = BmpMutation::PaintDirectRegion(PaintDirectRegion { revision: "stale".into(), ..match mutation { BmpMutation::PaintDirectRegion(payload) => payload, _ => unreachable!() } });
    let mut unchanged = original.clone();
    let refused = apply_mutation(&mut unchanged, &stale);
    assert!(refused.diff().image.is_none() && refused.diff().rects.is_empty());
    assert_eq!(unchanged, original);
}

#[test]
fn indexed_region_mutation_round_trips_text_binary_and_undo() {
    let original = indexed();
    let mutation = BmpMutation::PaintIndexedRegion(PaintIndexedRegion {
        revision: crate::standards::v_v3::subsets::any::schema::operations::bmp_revision(&original),
        x: 0,
        y: 0,
        width: 2,
        height: 1,
        palette_index: 1,
    });
    let text = protocol::OpText::print_op(&mutation);
    assert_eq!(<BmpMutation as protocol::OpText>::parse_op(&text).unwrap(), mutation);
    let binary = protocol::OpBinary::encode_op(&mutation).unwrap();
    assert_eq!(<BmpMutation as protocol::OpBinary>::decode_op(&binary).unwrap(), mutation);
    let inverse = mutation.inverse(&original).expect("valid retained mutation inverse fixture");
    let mut edited = original.clone();
    apply_mutation(&mut edited, &mutation);
    assert_ne!(edited, original);
    apply_mutation(&mut edited, &inverse[0]);
    assert_eq!(edited, original);
}

fn round_trips_wire_forms(mutation: &BmpMutation) {
    let text = protocol::OpText::print_op(mutation);
    assert_eq!(&<BmpMutation as protocol::OpText>::parse_op(&text).unwrap(), mutation);
    let binary = protocol::OpBinary::encode_op(mutation).unwrap();
    assert_eq!(&<BmpMutation as protocol::OpBinary>::decode_op(&binary).unwrap(), mutation);
}

#[test]
fn palette_entry_edits_keep_the_picture_round_trip_wire_forms_and_undo_exactly() {
    use crate::schema::mutations::{InsertPaletteEntry, RemovePaletteEntry, SetPaletteEntry};
    use crate::schema::snapshot::BmpPixels;
    let original = indexed();
    let length = original.image.palette.len() as u32;
    let maximum = 1u32 << original.image.profile.bits_per_pixel();
    assert!(length < maximum, "the fixture palette leaves room for one more entry");
    let colors_used_after_insert = if length + 1 == maximum { 0 } else { length + 1 };

    let set = BmpMutation::SetPaletteEntry(SetPaletteEntry { index: 0, red: 1, green: 2, blue: 3, reserved: 0 });
    round_trips_wire_forms(&set);
    let mut edited = original.clone();
    apply_mutation(&mut edited, &set);
    assert_eq!((edited.image.palette[0].r, edited.image.palette[0].g, edited.image.palette[0].b), (1, 2, 3));
    assert_eq!(edited.image.pixels, original.image.pixels);
    for inverse in set.inverse(&original).expect("set inverse") { apply_mutation(&mut edited, &inverse); }
    assert_eq!(edited, original);

    let insert = BmpMutation::InsertPaletteEntry(InsertPaletteEntry { index: 0, red: 10, green: 20, blue: 30, reserved: 0, colors_used: colors_used_after_insert });
    round_trips_wire_forms(&insert);
    let mut inserted = original.clone();
    apply_mutation(&mut inserted, &insert);
    assert_eq!(inserted.image.palette.len() as u32, length + 1);
    let (BmpPixels::Indexed { indices: before }, BmpPixels::Indexed { indices: after }) = (&original.image.pixels, &inserted.image.pixels) else { panic!("indexed fixture") };
    assert!(before.iter().zip(after).all(|(old, new)| *new == old + 1), "every sample keeps its color after a front insert");
    inserted.validate().expect("the inserted image stays valid");
    for inverse in insert.inverse(&original).expect("insert inverse") { apply_mutation(&mut inserted, &inverse); }
    assert_eq!(inserted, original);

    let remove = BmpMutation::RemovePaletteEntry(RemovePaletteEntry { index: 0, colors_used: length as u32 });
    let mut grown = original.clone();
    apply_mutation(&mut grown, &insert);
    round_trips_wire_forms(&remove);
    let mut shrunk = grown.clone();
    apply_mutation(&mut shrunk, &BmpMutation::RemovePaletteEntry(RemovePaletteEntry { index: 0, colors_used: original.image.colors_used }));
    assert_eq!(shrunk, original);
    for inverse in BmpMutation::RemovePaletteEntry(RemovePaletteEntry { index: 0, colors_used: original.image.colors_used }).inverse(&grown).expect("remove inverse") { apply_mutation(&mut shrunk, &inverse); }
    assert_eq!(shrunk, grown);
}

#[test]
fn palette_entry_edits_refuse_used_absent_and_direct_targets_without_changing_the_image() {
    use crate::schema::mutations::{InsertPaletteEntry, RemovePaletteEntry, SetPaletteEntry};
    let original = indexed();
    let used = match &original.image.pixels { crate::schema::snapshot::BmpPixels::Indexed { indices } => u32::from(indices[0]), _ => panic!("indexed fixture") };
    for refused in [
        BmpMutation::RemovePaletteEntry(RemovePaletteEntry { index: used, colors_used: original.image.colors_used }),
        BmpMutation::RemovePaletteEntry(RemovePaletteEntry { index: 9999, colors_used: 0 }),
        BmpMutation::SetPaletteEntry(SetPaletteEntry { index: 9999, red: 0, green: 0, blue: 0, reserved: 0 }),
        BmpMutation::InsertPaletteEntry(InsertPaletteEntry { index: 9999, red: 0, green: 0, blue: 0, reserved: 0, colors_used: 0 }),
    ] {
        let mut unchanged = original.clone();
        let outcome = apply_mutation(&mut unchanged, &refused);
        assert!(outcome.diff().image.is_none() && outcome.diff().rects.is_empty());
        assert_eq!(unchanged, original);
    }
    let direct_image = direct();
    let mut unchanged = direct_image.clone();
    apply_mutation(&mut unchanged, &BmpMutation::SetPaletteEntry(SetPaletteEntry { index: 0, red: 0, green: 0, blue: 0, reserved: 0 }));
    assert_eq!(unchanged, direct_image);
}
