
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

//#region 🌐️AxesParity
/// 🌐️ The `Locale`/`Terminology` axes this target resolves labels through are generated from the
/// SAME `🎚️axes/🔣️.json` that emits React's `SHELL_LOCALES`/`SHELL_TERMINOLOGIES`
/// (`🎚️axes/📽️projection/🟦️.ts`), so a label tier can never exist on one renderer only. Asserted
/// against the JSON itself rather than against a second hardcoded list — ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o.
#[semio_framework_async_macros::async_test]
async fn the_generated_axes_match_the_shared_axes_source() {
    let axes: serde_json::Value = serde_json::from_str(include_str!("../../🎚️axes/🔣️.json")).expect("the axes source is valid JSON");
    let locales: Vec<&str> = axes["locales"].as_array().expect("locales array").iter().map(|entry| entry["id"].as_str().expect("locale id")).collect();
    let terminologies: Vec<&str> = axes["terminologies"].as_array().expect("terminologies array").iter().map(|entry| entry["id"].as_str().expect("terminology id")).collect();
    assert_eq!(Locale::ALL.iter().map(|locale| locale.as_str()).collect::<Vec<_>>(), locales);
    assert_eq!(Terminology::ALL.iter().map(|terminology| terminology.as_str()).collect::<Vec<_>>(), terminologies);
    assert_eq!(Locale::COUNT, locales.len());
    assert_eq!(Terminology::COUNT, terminologies.len());
}

/// 🗺️ The resolution matrix really is two independent axes: `native` text translates per locale and
/// ignores terminology, `data` is invariant on both, and a full matrix answers a distinct cell per
/// `(terminology, locale)` pair — the tiering every wgpu label goes through before it is painted.
#[semio_framework_async_macros::async_test]
async fn label_tiers_resolve_per_locale_and_per_terminology() {
    let native = LocalizedLabel::native("Open", "Öffnen");
    for terminology in Terminology::ALL {
        assert_eq!(native.resolve(terminology, Locale::En), "Open");
        assert_eq!(native.resolve(terminology, Locale::De), "Öffnen");
    }
    let data = LocalizedLabel::data("kitchen.spk");
    for terminology in Terminology::ALL {
        for locale in Locale::ALL {
            assert_eq!(data.resolve(terminology, locale), "kitchen.spk", "runtime data is never translated");
        }
    }
    let matrix = LocalizedLabel::from_fn(|terminology, locale| format!("{}/{}", terminology.as_str(), locale.as_str()));
    let mut seen: Vec<&str> = Vec::new();
    for terminology in Terminology::ALL {
        for locale in Locale::ALL {
            let cell = matrix.resolve(terminology, locale);
            assert!(!seen.contains(&cell), "every (terminology, locale) cell is addressed exactly once");
            seen.push(cell);
        }
    }
    assert_eq!(seen.len(), Terminology::COUNT * Locale::COUNT);
}
//#endregion 🌐️AxesParity
