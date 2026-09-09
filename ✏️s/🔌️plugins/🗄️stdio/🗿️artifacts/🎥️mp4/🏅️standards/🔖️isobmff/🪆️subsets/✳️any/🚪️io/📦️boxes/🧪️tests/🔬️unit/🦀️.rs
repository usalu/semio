use super::*;

#[semio_framework_async_macros::async_test]
async fn iterates_two_sibling_boxes_and_resolves_sizes() {
    let ftyp = write_box(b"ftyp", b"isom");
    let free = write_box(b"free", &[0, 0]);
    let mut bytes = ftyp.clone();
    bytes.extend_from_slice(&free);
    let boxes: Vec<_> = iter_boxes(&bytes).collect::<Result<Vec<_>, _>>().expect("iterate");
    assert_eq!(boxes.len(), 2);
    assert_eq!(boxes[0].kind.0, *b"ftyp");
    assert_eq!(boxes[0].payload, b"isom");
    assert_eq!(boxes[1].kind.0, *b"free");
    assert_eq!(find_box(&bytes, b"free").unwrap(), Some(&[0u8, 0][..]));
    assert_eq!(find_box(&bytes, b"nope").unwrap(), None);
}

#[semio_framework_async_macros::async_test]
async fn require_box_errors_when_absent() {
    let bytes = write_box(b"ftyp", b"isom");
    assert!(require_box(&bytes, b"moov", "missing moov").is_err());
}
