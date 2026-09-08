
use super::*;
use dsl::{FromValue, ToValue};

#[semio_framework_async_macros::async_test]
async fn round_trips_through_to_value_and_from_value() {
    for icon in [IconName::AlertCircle, IconName::AppWindow, IconName::Lock, IconName::CircleDot, IconName::Sparkles, IconName::Folder] {
        let decoded = IconName::from_value(icon.to_value()).expect("valid DslValue decodes");
        assert_eq!(decoded, icon);
    }
}

#[semio_framework_async_macros::async_test]
async fn to_value_matches_the_existing_serde_wire_string() {
    let icon = IconName::AlertCircle;
    assert_eq!(icon.to_value(), dsl::DslValue::String("alert-circle".to_string()));
}
