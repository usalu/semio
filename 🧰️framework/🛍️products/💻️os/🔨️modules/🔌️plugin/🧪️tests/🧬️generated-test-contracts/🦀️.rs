#[doc(hidden)]
#[macro_export]
macro_rules! __semio_plugin_descriptor_fresh_test {
    ($describe:path) => {
        #[cfg(test)]
        #[test]
        fn descriptor_is_fresh() {
            __semio_install_plugin_bundle();
            let plugin_id = __SEMIO_PLUGIN_RUNTIME.with(|runtime| $crate::app::resolve_ready($crate::plugin_runtime::plugin_manifest(runtime))).plugin_id;
            let assembled = __SEMIO_PLUGIN_RUNTIME.with(|runtime| $crate::app::resolve_ready($describe(runtime)));
            let expected_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🛂️.descriptor.semio");
            const DESCRIPTOR_MIGRATED_PLUGINS: &[&str] = &["note", "sequence", "vcs", "forms", "sourcing", "dag", "mathematical", "writer", "reasoning", "animate", "draw", "energy", "layout"];
            match std::fs::read(expected_path) {
                Ok(expected) => {
                    if let (Some(assembled), Some(expected)) = ($crate::plugin_runtime::descriptor_bytes_with_blank_hashes(&assembled), $crate::plugin_runtime::descriptor_bytes_with_blank_hashes(&expected)) {
                        assert_eq!(assembled, expected, "{expected_path} is stale — re-run `describe` (📓️design-abi.md §3) and commit the refreshed 🛂️.descriptor.semio + 🔣️.json");
                    }
                }
                Err(_) => {
                    assert!(!DESCRIPTOR_MIGRATED_PLUGINS.contains(&plugin_id.as_str()), "{expected_path} is missing but {plugin_id:?} is listed in DESCRIPTOR_MIGRATED_PLUGINS — run `describe` and commit 🛂️.descriptor.semio + 🔣️.json");
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __semio_extension_descriptor_fresh_test {
    () => {
        #[cfg(test)]
        #[semio_framework_async_macros::async_test]
        async fn descriptor_is_fresh() {
            __semio_install_extension_bundle();
            let extension_id = $crate::plugin_runtime::extension_manifest().await.extension_id;
            let assembled = $crate::describe::describe_extension().await;
            let expected_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🛂️.descriptor.semio");
            const DESCRIPTOR_MIGRATED_EXTENSIONS: &[&str] = &[];
            match std::fs::read(expected_path) {
                Ok(expected) => {
                    if let (Some(assembled), Some(expected)) = ($crate::plugin_runtime::descriptor_bytes_with_blank_hashes(&assembled), $crate::plugin_runtime::descriptor_bytes_with_blank_hashes(&expected)) {
                        assert_eq!(assembled, expected, "{expected_path} is stale — re-run `describe` (📓️design-abi.md §3) and commit the refreshed 🛂️.descriptor.semio + 🔣️.json");
                    }
                }
                Err(_) => {
                    assert!(!DESCRIPTOR_MIGRATED_EXTENSIONS.contains(&extension_id.as_str()), "{expected_path} is missing but {extension_id:?} is listed in DESCRIPTOR_MIGRATED_EXTENSIONS — run `describe` and commit 🛂️.descriptor.semio + 🔣️.json");
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __semio_subset_owning_conformance_tests {
    ($composition:ty) => {
        #[cfg(test)]
        mod conformance {
            use super::*;

            #[semio_framework_async_macros::async_test]
            async fn subset_macro_owning_dialect_matches_spec() {
                assert_eq!(SUBSET_DIALECT, <$composition as $crate::ArtifactComposition>::WRITES);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __semio_subset_derived_conformance_tests {
    ($validator:ty) => {
        #[cfg(test)]
        mod conformance {
            use super::*;

            #[semio_framework_async_macros::async_test]
            async fn subset_macro_derived_dialect_is_non_any() {
                assert_ne!(SUBSET_DIALECT.subset, $crate::SubsetId::ANY);
            }

            #[semio_framework_async_macros::async_test]
            async fn subset_macro_derived_validator_registers() {
                register().await;
                let registered = semio_framework::io::list_registered_subset_validator_dialects().await.expect("registered subset observation");
                assert_eq!(registered.iter().filter(|dialect| **dialect == SUBSET_DIALECT).count(), 1);
                let payload = $crate::IoPayload::Text(String::new());
                let _ = <$validator as $crate::SubsetValidator>::validate(&payload).await;
            }
        }
    };
}
