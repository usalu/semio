//! 🔬️ Verify the classic cross-reference cursor at every byte boundary.

use super::{ClassicXrefCursor, ClassicXrefEvent};

#[test]
fn reads_all_live_offsets_under_small_grants() {
    let pdf = include_bytes!("../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    for budget in [1, 2, 5, 19, 256] {
        let mut cursor = ClassicXrefCursor::new(703);
        let mut live = Vec::new();
        loop {
            let before = cursor.position();
            let receipt = cursor.advance(pdf, budget);
            assert!(receipt.work <= budget);
            let event = receipt.event.unwrap();
            assert!(cursor.position() - before <= budget);
            match event {
                ClassicXrefEvent::More => assert!(cursor.position() > before),
                ClassicXrefEvent::Entry { number, offset, generation, live: true } => {
                    let header = format!("{number} {generation} obj");
                    assert!(pdf[offset..].starts_with(header.as_bytes()));
                    live.push(number);
                }
                ClassicXrefEvent::Entry { live: false, .. } => {}
                ClassicXrefEvent::Trailer => break,
            }
        }
        assert_eq!(live, (1..9).collect::<Vec<_>>());
        assert!(pdf[cursor.position()..].starts_with(b"\n<<"));
    }
}
