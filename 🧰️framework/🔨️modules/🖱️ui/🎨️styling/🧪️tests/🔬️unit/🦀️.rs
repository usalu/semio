
use super::appearance::AppearanceName;

#[test]
fn appearance_name_parse() {
    assert_eq!(AppearanceName::parse("dark"), AppearanceName::Dark);
    assert_eq!(AppearanceName::parse("light"), AppearanceName::Light);
    assert_eq!(AppearanceName::parse("DARK"), AppearanceName::Dark);
    assert_eq!(AppearanceName::parse("anything-else"), AppearanceName::Light);
}
