use super::*;

#[test]
fn worst_case_terminal_windows_never_exceed_their_budget() {
    let len = usize::MAX;
    let camera_end = bounded_terminal_end(17, len, 64);
    let point_end = bounded_terminal_end(23, len, 256);
    let quality_end = bounded_terminal_end(29, len, 256);
    assert_eq!(camera_end - 17, 64);
    assert_eq!(point_end - 23, 256);
    assert_eq!(quality_end - 29, 256);
}
