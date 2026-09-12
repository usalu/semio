
use super::*;

#[test]
fn mip_extent_halves_each_level_and_floors_at_one() {
    assert_eq!(mip_extent(800, 600, 0), (800, 600));
    assert_eq!(mip_extent(800, 600, 1), (400, 300));
    assert_eq!(mip_extent(800, 600, 2), (200, 150));
    assert_eq!(mip_extent(3, 3, 3), (1, 1));
    assert_eq!(mip_extent(3, 3, 10), (1, 1));
}

#[test]
fn mip_extent_never_reaches_zero() {
    for level in 0..SCENE_MIP_LEVELS {
        let (width, height) = mip_extent(1, 1, level);
        assert!(width >= 1 && height >= 1);
    }
}
