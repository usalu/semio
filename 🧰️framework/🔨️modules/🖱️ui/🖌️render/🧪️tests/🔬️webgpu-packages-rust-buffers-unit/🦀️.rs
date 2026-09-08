
use super::*;

#[test]
fn capacity_stays_when_required_fits() {
    assert_eq!(grown_capacity(256, 100), 256);
    assert_eq!(grown_capacity(256, 256), 256);
}

#[test]
fn capacity_grows_to_next_power_of_two() {
    assert_eq!(grown_capacity(256, 257), 512);
    assert_eq!(grown_capacity(0, 1), 256);
    assert_eq!(grown_capacity(0, 1000), 1024);
}

#[test]
fn world_globals_slot_size_is_uniform_alignment_safe() {
    assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
    assert!(WORLD_GLOBALS_SLOT_SIZE >= std::mem::size_of::<World3dGlobals>() as u64);
}
