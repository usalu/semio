use super::*;

#[test]
fn celsius_kelvin_roundtrip() {
    assert!((k_to_c(c_to_k(20.0)) - 20.0).abs() < 1e-9);
}
