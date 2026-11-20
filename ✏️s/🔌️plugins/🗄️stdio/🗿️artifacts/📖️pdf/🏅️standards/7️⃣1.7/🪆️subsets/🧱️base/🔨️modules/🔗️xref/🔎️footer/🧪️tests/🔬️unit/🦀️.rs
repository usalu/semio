//! 🔬️ Verify footer discovery with interrupted marker, comment and numeric tokens.

use super::{FooterCursor, FooterStep};

#[test]
fn each_footer_phase_resumes_on_a_one_byte_grant() {
    let pdf = include_bytes!("../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    for (file, expected) in [
        (&pdf[..], Some(703)), (&b"startxref 1 startxref % last update\r\n+2"[..], Some(2)),
        (&b"startxref 4000\n%%EOF"[..], None), (&b"%PDF-1.7\n%%EOF"[..], None),
        (&b"startxref 1.5\n%%EOF"[..], None),
        (&b"startxref 2\n%%EOF\nxstartxref 4"[..], Some(2)),
        (&b"startxref 3\n%%EOF\nstartxref123"[..], Some(3)),
        (&b"startxref 4\n%%EOF\nstartxref 9999999999999999999999999999999"[..], Some(4)),
        (&b"startxref 5\n%%EOF\nstartxref bogus"[..], Some(5)),
    ] {
        for budget in [1, 3, 8, 32] {
            let mut cursor = FooterCursor::new(file.len());
            assert!(matches!(cursor.advance(file, 0), FooterStep::More));
            let found = (0..file.len() * 2 + 2).find_map(|_| match cursor.advance(file, budget) {
                FooterStep::More => None,
                FooterStep::Complete(value) => Some(value),
            }).expect("footer finishes within the finite input");
            assert_eq!(found, expected);
        }
    }
}
