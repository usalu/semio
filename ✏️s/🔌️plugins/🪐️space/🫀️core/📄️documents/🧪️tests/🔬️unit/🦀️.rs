use super::*;

fn codec(format: SpaceDocumentFormat, text: &str) -> Result<String, String> {
    match format {
        SpaceDocumentFormat::Json => Ok(text.into()),
        SpaceDocumentFormat::Dsl if text == "shape id=owned" => Ok("{\"schema\":\"shape.v1\",\"id\":\"owned\"}".into()),
        SpaceDocumentFormat::Dsl => Err("fixture codec refuses malformed source".into()),
    }
}

fn source(text: &str, format: SpaceDocumentFormat) -> SpaceDocumentSource {
    SpaceDocumentSource { slug: "fixture:shape".into(), format, codec: "shape-v1".into(), text: text.into() }
}

#[test]
fn typed_source_formats_match_the_language_neutral_vectors_and_independent_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for vector in vectors["vectors"].as_array().unwrap() {
        let row = &vector["source"];
        let format = row["format"].as_str().unwrap().parse::<SpaceDocumentFormat>().unwrap();
        let input = SpaceDocumentSource { slug: row["slug"].as_str().unwrap().into(), format, codec: row["codec"].as_str().unwrap().into(), text: row["text"].as_str().unwrap().into() };
        let documents = prepare_space_document_sources(&[input], &[SpaceDocumentCodec { id: "shape-v1", decode: codec }]).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&documents[0].1).unwrap();
        assert_eq!(actual, vector["expected"]);
    }
}

#[test]
fn hostile_sources_fail_closed_without_dropping_any_original_category() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for vector in vectors["invalid"].as_array().unwrap() {
        let inputs = vector["sources"].as_array().unwrap().iter().map(|row| {
            let format = row["format"].as_str().unwrap().parse::<SpaceDocumentFormat>()?;
            Ok(SpaceDocumentSource { slug: row["slug"].as_str().unwrap().into(), format, codec: row["codec"].as_str().unwrap().into(), text: row["text"].as_str().unwrap().into() })
        }).collect::<Result<Vec<_>, String>>();
        let result = inputs.and_then(|inputs| prepare_space_document_sources(&inputs, &[SpaceDocumentCodec { id: "shape-v1", decode: codec }]));
        assert!(result.is_err(), "{}", vector["id"]);
    }
}

#[test]
fn repeated_identity_and_second_source_failure_refuse_the_whole_batch() {
    assert!(prepare_space_document_sources(&[source("{}", SpaceDocumentFormat::Json)], &[SpaceDocumentCodec { id: "shape-v1", decode: codec }, SpaceDocumentCodec { id: "shape-v1", decode: codec }]).is_err());
    let first = source("{\"schema\":\"shape.v1\"}", SpaceDocumentFormat::Json);
    let second = source("{\"schema\":\"shape.v1\"}", SpaceDocumentFormat::Json);
    assert!(prepare_space_document_sources(&[first, second], &[SpaceDocumentCodec { id: "shape-v1", decode: codec }]).is_err());
    let mut first = source("{\"schema\":\"shape.v1\"}", SpaceDocumentFormat::Json);
    first.slug = "fixture:atomic-refusal".into();
    assert!(prepare_space_document_sources(&[first, source("{", SpaceDocumentFormat::Json)], &[SpaceDocumentCodec { id: "shape-v1", decode: codec }]).is_err());
}
