mod terminology_tests {
    use super::*;

    app_labels! {
        struct SampleLabels {
            greeting: native_en "Hello", native_de "Hallo", reuse_en "Hi", reuse_de "Servus";
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_labels_is_exhaustive_over_all_four_cells() {
        let native_en = ViewModel { locale: Locale::En, terminology: Terminology::Native, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        let native_de = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        let reuse_en = ViewModel { locale: Locale::En, terminology: Terminology::Reuse, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        let reuse_de = ViewModel { locale: Locale::De, terminology: Terminology::Reuse, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        assert_eq!(resolve_labels::<SampleLabels>(&native_en).greeting.as_str(), "Hello");
        assert_eq!(resolve_labels::<SampleLabels>(&native_de).greeting.as_str(), "Hallo");
        assert_eq!(resolve_labels::<SampleLabels>(&reuse_en).greeting.as_str(), "Hi");
        assert_eq!(resolve_labels::<SampleLabels>(&reuse_de).greeting.as_str(), "Servus");
    }
}
