use super::{decode_clipboard_content, ClipboardContent};

#[test]
fn native_clipboard_page_decodes_text_and_exact_rgba_without_conflating_them() {
    match decode_clipboard_content(&[1, b'H', b'i']).expect("text clipboard page") {
        ClipboardContent::Text(text) => assert_eq!(text, "Hi"),
        ClipboardContent::ImageRgba8 { .. } => panic!("text page decoded as image"),
    }
    let mut image = vec![2];
    image.extend_from_slice(&1u32.to_le_bytes());
    image.extend_from_slice(&2u32.to_le_bytes());
    image.extend_from_slice(&[255, 0, 0, 255, 0, 255, 0, 255]);
    match decode_clipboard_content(&image).expect("RGBA clipboard page") {
        ClipboardContent::ImageRgba8 { width, height, bytes } => {
            assert_eq!((width, height), (1, 2));
            assert_eq!(bytes, [255, 0, 0, 255, 0, 255, 0, 255]);
        }
        ClipboardContent::Text(_) => panic!("image page decoded as text"),
    }
    image.pop();
    assert!(decode_clipboard_content(&image).is_none(), "truncated RGBA is an explicit refusal");
}
