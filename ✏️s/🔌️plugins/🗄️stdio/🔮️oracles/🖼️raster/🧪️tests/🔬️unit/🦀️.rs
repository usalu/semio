
use super::*;

/// 🧵️ GIF §20's worked example: an 8-row image stores rows 0,8… then 4,12… then 2,6… then odd.
#[test]
fn the_four_pass_order_matches_the_specification() {
    assert_eq!(gif_interlace_row_order(8), vec![0, 4, 2, 6, 1, 3, 5, 7]);
    assert_eq!(gif_interlace_row_order(1), vec![0]);
    assert_eq!(gif_interlace_row_order(0), Vec::<usize>::new());
}

#[test]
fn reordering_rows_is_its_own_inverse() {
    let natural: Vec<u8> = (0..8u8).flat_map(|row| [row, row, row]).collect();
    let interlaced = gif_reorder_rows(&natural, 3, 8, true);
    assert_eq!(&interlaced[0..3], &[0, 0, 0], "the first stored row is row 0");
    assert_eq!(&interlaced[3..6], &[4, 4, 4], "the second stored row is row 4");
    assert_eq!(gif_reorder_rows(&interlaced, 3, 8, false), natural);
}

#[test]
fn a_buffer_that_does_not_match_the_geometry_is_returned_untouched() {
    assert_eq!(gif_reorder_rows(&[1, 2, 3], 4, 4, true), vec![1, 2, 3]);
}

/// 🚩️ A hand-built two-image 87a stream: image 0 interlaced, image 1 not, with a comment
/// extension between them that the walk must skip structurally.
#[test]
fn interlace_flags_are_recovered_per_image_block() {
    let mut data = Vec::new();
    data.extend_from_slice(b"GIF87a");
    data.extend_from_slice(&2u16.to_le_bytes());
    data.extend_from_slice(&2u16.to_le_bytes());
    data.extend_from_slice(&[0x00, 0x00, 0x00]);
    data.extend_from_slice(&[0x2C, 0, 0, 0, 0, 2, 0, 2, 0, 0x40, 0x02, 0x01, 0x00, 0x00]);
    data.extend_from_slice(&[0x21, 0xFE, 0x03, b'h', b'e', b'y', 0x00]);
    data.extend_from_slice(&[0x2C, 0, 0, 0, 0, 2, 0, 2, 0, 0x00, 0x02, 0x01, 0x00, 0x00]);
    data.push(0x3B);
    assert_eq!(gif_image_interlace_flags(&data).expect("walk the hand-built stream"), vec![true, false]);
}

#[test]
fn a_stream_that_is_not_a_gif_is_rejected_rather_than_guessed() {
    assert!(gif_image_interlace_flags(b"not a gif at all, really").is_err());
}
