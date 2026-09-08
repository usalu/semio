
use super::*;

#[test]
fn opening_preferences_default_is_empty() {
    assert_eq!(OpeningPreferences::default(), OpeningPreferences { defaults: Vec::new() });
}
