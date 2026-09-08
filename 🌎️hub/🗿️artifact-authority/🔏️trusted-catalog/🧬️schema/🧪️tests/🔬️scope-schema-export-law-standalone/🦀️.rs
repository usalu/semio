use semio_framework_schema_registry::{resolve_schema_export, scope_schema_exports_registered, SchemaFormat};

    /// 🗂️ The formats an export exists in, read from the module document rather than restated: an
    /// unannotated export exists in every format the module carries a file for.
    fn annotated(export: &str, provided: &[SchemaFormat]) -> Vec<SchemaFormat> {
        let module: serde_json::Value = serde_json::from_str(super::MODULE_JSON).expect("module json");
        match module["$defs"][export]["x-semio-formats"].as_array() {
            None => provided.to_vec(),
            Some(entries) => entries.iter().map(|entry| SchemaFormat::parse(entry.as_str().expect("format id")).expect("declared format id")).collect(),
        }
    }

    #[test]
    fn registers_and_resolves_exactly_the_annotated_formats() {
        super::register_scope_exports();
        assert!(scope_schema_exports_registered(super::SCHEMA_SCOPE));
        for export in super::EXPORTS.map(|declaration| declaration.id) {
            let declared = annotated(export, &PROVIDED);
            for format in SchemaFormat::ALL {
                let resolved = resolve_schema_export(super::SCHEMA_SCOPE, export, format);
                assert_eq!(resolved.is_ok(), declared.contains(&format), "{export} resolves {format} but x-semio-formats declares {declared:?}");
            }
        }
    }

    /// 🗂️ The formats this module carries a file for.
    const PROVIDED: [SchemaFormat; 2] = [SchemaFormat::JsonSchema, SchemaFormat::Rust];
