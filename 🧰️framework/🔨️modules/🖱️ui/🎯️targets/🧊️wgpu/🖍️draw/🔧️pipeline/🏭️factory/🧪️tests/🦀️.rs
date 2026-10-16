use super::*;
#[test]
fn world_globals_slot_size_is_aligned() {
    const { assert!(WORLD_GLOBALS_SLOT_SIZE >= 240) };
    assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
}
