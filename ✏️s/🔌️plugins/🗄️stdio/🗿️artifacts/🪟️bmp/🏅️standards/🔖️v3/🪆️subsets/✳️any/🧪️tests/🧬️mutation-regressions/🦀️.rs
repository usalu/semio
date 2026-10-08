use crate::apply_mutation;
use crate::schema::mutations::{BmpMutation, PaintDirectRegion, PaintIndexedRegion};
use protocol::Mutation;

fn direct() -> crate::BmpSnapshot {
    crate::standards::v_v3::subsets::any::io::decode_bmp(include_bytes!("../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")).unwrap()
}

fn indexed() -> crate::BmpSnapshot {
    crate::standards::v_v3::subsets::any::io::decode_bmp(include_bytes!("../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb4-duplicate-palette.bmp")).unwrap()
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
