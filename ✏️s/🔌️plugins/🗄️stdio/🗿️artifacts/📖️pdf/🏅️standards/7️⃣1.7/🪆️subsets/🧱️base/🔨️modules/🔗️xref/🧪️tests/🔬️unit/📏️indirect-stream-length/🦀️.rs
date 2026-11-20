use super::xref::{Resolver, build_xref};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;

#[test]
fn indirect_stream_length_preserves_embedded_endstream_word() {
    let pdf = include_bytes!("../../../../../🧪️tests/🧫️fixtures/🔗️indirect-length.pdf");
    let index = build_xref(pdf).unwrap();
    let mut resolver = Resolver::new(pdf, index.entries);
    let Some(PdfObject::Stream { data, .. }) = resolver.resolve(4) else { panic!("expected page content stream") };
    assert_eq!(data, b"BT /F1 12 Tf 10 70 Td (prefix endstream suffix) Tj ET");
}
