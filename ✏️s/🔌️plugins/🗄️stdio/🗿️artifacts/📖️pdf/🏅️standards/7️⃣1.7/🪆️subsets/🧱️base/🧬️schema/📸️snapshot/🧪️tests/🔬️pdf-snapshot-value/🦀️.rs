
    use super::*;
    use pack::value::{DslValue, FromValue, ToValue};

    fn populated_snapshot() -> PdfSnapshot {
        let root = ObjRef { num: 1, gen: 0 };
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.to_string(),
            declared_version: "1.7".to_string(),
            pages: vec![PdfPage { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: Some([12.0, 12.0, 600.0, 780.0]), rotate: 90, text: "Semio".to_string() }],
            info: PdfInfo { title: Some("Value path".to_string()), producer: Some("semio".to_string()), ..PdfInfo::default() },
            objects: vec![PdfIndirectObject { id: root, value: PdfObject::Name("Catalog".to_string()) }],
            trailer: vec![PdfDictEntry { key: "Root".to_string(), value: PdfObject::Ref(root) }],
        }
    }

    #[test]
    fn populated_snapshot_round_trips_through_value() {
        let snapshot = populated_snapshot();
        assert_eq!(PdfSnapshot::from_value(snapshot.to_value()), Ok(snapshot));
    }

    #[test]
    fn missing_optional_fields_keep_the_derived_defaults() {
        let value = DslValue::object([("schema".to_string(), DslValue::String("stdio.pdf.1.7".to_string()))]);
        assert_eq!(
            PdfSnapshot::from_value(value),
            Ok(PdfSnapshot { schema: "stdio.pdf.1.7".to_string(), declared_version: String::new(), pages: Vec::new(), info: PdfInfo::default(), objects: Vec::new(), trailer: Vec::new() })
        );
    }

    #[test]
    fn missing_schema_reports_the_exact_field() {
        let error = PdfSnapshot::from_value(DslValue::object([])).unwrap_err();
        assert_eq!(error.to_string(), "missing field `schema`");
    }

    #[test]
    fn camel_case_json_shape_agrees_with_serde_json_oracle() {
        let snapshot = PdfSnapshot::default();
        let actual: serde_json::Value = snapshot.to_value().into();
        let oracle = serde_json::json!({
            "schema": "stdio.pdf.1.7",
            "declaredVersion": "1.7",
            "pages": [],
            "info": {},
            "objects": [],
            "trailer": []
        });
        assert_eq!(actual, oracle);
        let json = serde_json::to_string(&oracle).expect("serde_json oracle encodes");
        assert_eq!(pack::json::from_json_str::<PdfSnapshot>(&json), Ok(snapshot));
    }
