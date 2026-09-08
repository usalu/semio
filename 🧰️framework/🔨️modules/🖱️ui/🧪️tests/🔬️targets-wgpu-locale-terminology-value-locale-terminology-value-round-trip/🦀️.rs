
use super::*;
use dsl::{FromValue, ToValue};

#[semio_framework_async_macros::async_test]
async fn locale_round_trips_through_to_value_and_from_value() {
    for locale in Locale::ALL {
        let decoded = Locale::from_value(locale.to_value()).expect("valid DslValue decodes");
        assert_eq!(decoded, locale);
    }
}

#[semio_framework_async_macros::async_test]
async fn terminology_round_trips_through_to_value_and_from_value() {
    for terminology in Terminology::ALL {
        let decoded = Terminology::from_value(terminology.to_value()).expect("valid DslValue decodes");
        assert_eq!(decoded, terminology);
    }
}

#[semio_framework_async_macros::async_test]
async fn to_value_matches_the_existing_serde_wire_string() {
    assert_eq!(Locale::En.to_value(), dsl::DslValue::String("en".to_string()));
    assert_eq!(Terminology::Native.to_value(), dsl::DslValue::String("native".to_string()));
}
