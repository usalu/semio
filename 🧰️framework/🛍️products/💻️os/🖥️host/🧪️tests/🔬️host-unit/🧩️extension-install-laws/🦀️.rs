    /// 🧩️ Extension install laws: the test executor bridge, the sample extension package, install, mismatch refusal, capability scoping and uninstall.

    //#region 🔖️ExtensionInstall
    /// 🚫️async: E5 executor bridge — test-only (`📌️important.md` R4 clause 5: a `#[test] fn`
    /// body is a sanctioned executor entry point). Separate from `🎠️activation/🦀️.rs`'s one
    /// PRODUCTION bridge in this same crate — R2's "at most one per crate" governs production
    /// code, and R4 clause 5 explicitly does not count a test bridge against that census.
    /// `store::extension::pack`/`verify`/`content_hash` and this module's own
    /// `install_extension_package` are I/O-free (pure zip/hash work over in-memory bytes), so
    /// they resolve on the first poll by construction.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        let mut future = std::pin::pin!(future);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("block_on: extension install/pack futures are documented I/O-free"),
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn sample_extension_sxt(extension_id: &str, extends: &str, capabilities: Vec<String>) -> Vec<u8> {
        let manifest = store::extension::ExtensionPackageManifest {
            extension_id: extension_id.into(),
            directory_name: "🧩️sample-extension".into(),
            label: extension_id.into(),
            version: "0.1.0".into(),
            extends: extends.into(),
            capabilities,
            topic_contributions: semio_framework_pack_json::Value::Array(Vec::new()),
            dependencies: vec![store::extension::PackagePluginDependency { plugin_id: extends.into(), version: "^1.0.0".into() }],
            contributions: semio_framework_pack_json::Value::Array(Vec::new()),
            package_format: store::extension::EXTENSION_PACKAGE_FORMAT,
        };
        let component = b"\0asm\x01\x00\x00\x00fake-component".to_vec();
        block_on(store::extension::pack(&manifest, &component, &[])).expect("pack a valid sample .sxt")
    }

    /// 🧫️ Installs 30 real `.sxt` packages (real zip bytes, real blake3 hash — not a mock)
    /// across 3 distinct parent plugins and proves `extensions_extending_plugin` returns
    /// exactly the matching subset for each, with none for a plugin that installed nothing —
    /// the actual install→query pipeline, not just the pure filter (that claim, at the scale
    /// fixture's full 2,500-descriptor shape, is proven separately by
    /// `🧰️framework/🔨️modules/🎠️kernel`'s own `extensions_extending` test and this packet's
    /// standalone verification script — see the report).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn install_extension_package_registers_descriptors_queryable_by_extends() {
        let mut host = PluginHost::new();
        for i in 0..30 {
            let extends = format!("plugin-{}", i % 3);
            let bytes = sample_extension_sxt(&format!("ext-{i}"), &extends, vec!["storage.read".into()]);
            let installed = block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");
            assert_eq!(installed.manifest.extension_id, format!("ext-{i}"));
            assert!(!installed.content_hash.is_empty());
        }

        for plugin_index in 0..3 {
            let plugin_id = format!("plugin-{plugin_index}");
            let matched = host.extensions_extending_plugin(&plugin_id);
            assert_eq!(matched.len(), 10, "10 of 30 synthetic extensions extend {plugin_id}");
            assert!(matched.iter().all(|installed| installed.manifest.extends == plugin_id));
        }
        assert!(host.extensions_extending_plugin("plugin-nonexistent").is_empty());
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn install_extension_package_rejects_extends_mismatch() {
        // 🩹️ `sample_extension_sxt` always writes a matching dependency; to exercise the
        // rejection path this test packs a manifest whose `extends` disagrees with its own
        // first dependency directly, rather than corrupting bytes (which `verify`'s own
        // envelope/zip checks would reject for an unrelated reason).
        let mismatched_manifest = store::extension::ExtensionPackageManifest {
            extension_id: "ext-mismatch".into(),
            directory_name: "⚖️ext-mismatch".into(),
            label: "Ext Mismatch".into(),
            version: "0.1.0".into(),
            extends: "flow".into(),
            capabilities: vec![],
            topic_contributions: semio_framework_pack_json::Value::Array(Vec::new()),
            dependencies: vec![store::extension::PackagePluginDependency { plugin_id: "cad".into(), version: "^1.0.0".into() }],
            contributions: semio_framework_pack_json::Value::Array(Vec::new()),
            package_format: store::extension::EXTENSION_PACKAGE_FORMAT,
        };
        let manifest_bytes_source = block_on(store::extension::pack(&mismatched_manifest, b"\0asm\x01\x00\x00\x00x", &[])).expect("pack a structurally-valid but contract-violating .sxt");

        let mut host = PluginHost::new();
        let error = block_on(host.install_extension_package(&manifest_bytes_source)).expect_err("extends != dependencies[0].plugin_id must be rejected at install time");
        assert!(matches!(error, ExtensionInstallError::ExtendsMismatch { extension_id, extends, actual } if extension_id == "ext-mismatch" && extends == "flow" && actual == "cad"));
        assert!(host.extensions_extending_plugin("flow").is_empty(), "a rejected install must not register a descriptor");
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn extension_capabilities_scoped_to_parent_drops_what_the_parent_lacks() {
        let mut host = PluginHost::new();
        let bytes = sample_extension_sxt("ext-caps", "cad", vec!["storage.read".into(), "storage.write".into()]);
        let installed = block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");

        let parent_effective = vec!["storage.read".to_string(), "http:example.com".to_string()];
        let scoped = host.extension_capabilities_scoped_to_parent(&installed, &parent_effective);
        assert_eq!(scoped, vec!["storage.read"], "storage.write is not in the parent's effective set");
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn uninstall_extension_package_removes_it_from_the_query() {
        let mut host = PluginHost::new();
        let bytes = sample_extension_sxt("ext-removable", "cad", vec![]);
        block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");
        assert_eq!(host.extensions_extending_plugin("cad").len(), 1);

        let removed = host.uninstall_extension_package("ext-removable").expect("previously installed");
        assert_eq!(removed.manifest.extension_id, "ext-removable");
        assert!(host.extensions_extending_plugin("cad").is_empty());
        assert!(host.uninstall_extension_package("ext-removable").is_none(), "uninstall is not idempotent-returning on a second call");
    }
    //#endregion 🔖️ExtensionInstall
