
use super::*;

#[semio_framework_async_macros::async_test]
async fn round_trips_through_to_value_and_from_value() {
    let label = LocalizedLabel::from_fn(|terminology, locale| format!("{}-{}", terminology.as_str(), locale.as_str()));
    let decoded = LocalizedLabel::from_value(label.to_value()).expect("valid DslValue decodes");
    assert_eq!(decoded, label);
}

#[semio_framework_async_macros::async_test]
async fn to_value_uses_the_same_keys_the_hand_written_serde_impl_emits() {
    let label = LocalizedLabel::native("Hello", "Hallo");
    let entries = label.to_value().into_object().expect("LocalizedLabel::to_value is an object");
    for terminology in Terminology::ALL {
        let inner = entries.iter().find(|(key, _)| key == terminology.as_str()).map(|(_, value)| value.clone()).expect("terminology key present").into_object().expect("inner value is an object");
        for locale in Locale::ALL {
            let text = inner.iter().find(|(key, _)| key == locale.as_str()).map(|(_, value)| value.clone()).expect("locale key present");
            assert_eq!(text, DslValue::String(label.resolve(terminology, locale).to_string()));
        }
    }
}
