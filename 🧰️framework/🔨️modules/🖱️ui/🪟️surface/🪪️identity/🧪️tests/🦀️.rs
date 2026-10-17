use super::*;

#[test]
fn original_surface_identity_preserves_slot_and_generation_without_aba_aliasing() {
    for row in include_str!("🧫️fixtures/📐️identity.csv").lines().skip(1) {
        let values: Vec<_> = row.split(',').collect();
        let left = UiSurfaceToken::new(values[0].parse().unwrap(), values[1].parse().unwrap());
        let right = UiSurfaceToken::new(values[2].parse().unwrap(), values[3].parse().unwrap());
        assert_eq!(left.slot().to_string(), values[0]);
        assert_eq!(left.generation().to_string(), values[1]);
        assert_eq!(left == right, values[4] == "true");
        assert_eq!(left, left.clone());
    }
    assert_eq!(UI_LAYOUT_SURFACE_SLOTS, 64);
}
