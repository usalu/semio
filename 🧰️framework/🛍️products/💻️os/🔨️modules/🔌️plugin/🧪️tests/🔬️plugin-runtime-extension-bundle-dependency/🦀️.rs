mod extension_bundle_dependency_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn extends_mismatching_the_first_dependency_panics() {
        let result = std::panic::catch_unwind(|| ExtensionBundle::new("ext-mismatch", "Ext Mismatch", "0.1.0").depends_on("primary-dep", semio_framework::VersionReq::Any).extends("someone-else"));
        assert!(result.is_err(), "extends != dependencies[0].plugin_id must panic");
    }

    #[semio_framework_async_macros::async_test]
    async fn extends_set_before_a_mismatching_dependency_also_panics() {
        let result = std::panic::catch_unwind(|| ExtensionBundle::new("ext-mismatch-2", "Ext Mismatch 2", "0.1.0").extends("primary-dep").depends_on("someone-else", semio_framework::VersionReq::Any));
        assert!(result.is_err(), "extends != dependencies[0].plugin_id must panic regardless of call order");
    }

    #[semio_framework_async_macros::async_test]
    async fn extends_matching_the_first_dependency_is_accepted_regardless_of_call_order() {
        let bundle = ExtensionBundle::new("ext-ok", "Ext Ok", "0.1.0").extends("primary-dep").depends_on("primary-dep", semio_framework::VersionReq::Any).depends_on("secondary-dep", semio_framework::VersionReq::Any);
        assert_eq!(bundle.manifest.dependencies[0].plugin_id, "primary-dep");
        assert_eq!(bundle.manifest.dependencies[1].plugin_id, "secondary-dep");
        assert_eq!(bundle.manifest.extends, "primary-dep");
    }
}
