use super::project_dxf_r12;
use dxf::entities::EntityType;
use semio_repo_test_host::Json;

#[test]
fn semantic_golden_entities_match_the_independent_dxf_reader() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖊️semantic/🔣️.json")).expect("declared semantic cases");
    for case in cases.as_array().expect("closed case array") {
        let bytes = case["input"].as_str().expect("declared DXF input").as_bytes();
        if let Some(prefix) = case["errorPrefix"].as_str() {
            let error = project_dxf_r12(bytes).expect_err("malformed input refuses");
            assert!(error.starts_with(prefix), "{}: {error}", case["id"]);
            assert!(dxf::Drawing::load(&mut &bytes[..]).is_err());
            continue;
        }
        let drawing = dxf::Drawing::load(&mut &bytes[..]).expect("independent library parses fixture");
        let independent: Vec<serde_json::Value> = drawing.entities().map(|entity| match &entity.specific {
            EntityType::Line(line) => serde_json::json!({ "start": [line.p1.x, line.p1.y, line.p1.z], "end": [line.p2.x, line.p2.y, line.p2.z], "entityKind": "line", "layer": entity.common.layer }),
            _ => panic!("golden cases declare only line entities"),
        }).collect();
        assert_eq!(serde_json::Value::Array(independent), case["entities"], "{}", case["id"]);
        let projected = project_dxf_r12(bytes).expect("owned semantic reader parses fixture");
        let entities = projected.array("entities");
        let expected = case["entities"].as_array().expect("declared entity projection");
        assert_eq!(entities.len(), expected.len(), "{}", case["id"]);
        for (entity, expected) in entities.iter().zip(expected) {
            assert_eq!(entity.str("entityKind"), expected["entityKind"].as_str().unwrap());
            assert_eq!(entity.str("layer"), expected["layer"].as_str().unwrap());
            for key in ["start", "end"] {
                let actual: Vec<f64> = entity.array(key).iter().map(|value| match value { Json::Number(number) => *number, _ => panic!("coordinate must be numeric") }).collect();
                assert_eq!(serde_json::to_value(&actual).unwrap(), expected[key], "the owned projection retains the literal f64 representation: {key}");
                let expected: Vec<f64> = expected[key].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect();
                assert_eq!(actual, expected, "{key}");
            }
        }
    }
}
