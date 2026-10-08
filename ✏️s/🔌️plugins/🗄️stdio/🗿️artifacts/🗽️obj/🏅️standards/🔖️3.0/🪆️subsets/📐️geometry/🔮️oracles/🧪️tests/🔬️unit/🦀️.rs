
use super::*;

const DOCUMENT: &str = "# a retained comment\nmtllib pattern.mtl\nv 0 0 0\nv 1 0 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 0 1\nvn 0 0 1\ng band\no cap\nusemtl pattern\ns 1\nf 1/1/1 2/2/1 3/3/1\n";

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

const TWO_BANDS: &str = "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 1 1 0\ng lower\nf 1 2 3\nf 2 3 4\ng upper\nf 1 3 4\nf 1 2 4\n";

fn band_membership(document: &[u8]) -> Vec<(String, Vec<usize>)> {
    oracle_snapshot_json(document)
        .unwrap()
        .array("groups")
        .iter()
        .map(|entry| {
            (
                entry.str("name"),
                entry
                    .array("faces")
                    .iter()
                    .map(|face| match face {
                        Json::Number(number) => *number as usize,
                        other => panic!("a face index must be a number, found {other:?}"),
                    })
                    .collect(),
            )
        })
        .collect()
}

#[test]
fn removing_an_interior_face_closes_the_membership_index_space() {
    let removed = oracle_apply_mutation(TWO_BANDS.as_bytes(), &spec("remove-face", Json::Object(vec![("index".to_string(), Json::Number(1.0))]))).unwrap();
    assert_eq!(band_membership(&removed), vec![("lower".to_string(), vec![0]), ("upper".to_string(), vec![1, 2])], "the removed face must leave its own band and every later face must close up by one");
}

#[test]
fn inserting_an_interior_face_opens_the_membership_index_space() {
    let face = Json::Object(vec![(
        "vertices".to_string(),
        Json::Array(vec![Json::Object(vec![("vertex".to_string(), Json::Number(0.0))]), Json::Object(vec![("vertex".to_string(), Json::Number(1.0))]), Json::Object(vec![("vertex".to_string(), Json::Number(3.0))])]),
    )]);
    let widened = oracle_apply_mutation(TWO_BANDS.as_bytes(), &spec("insert-face", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("face".to_string(), face)]))).unwrap();
    assert_eq!(band_membership(&widened), vec![("lower".to_string(), vec![0, 2]), ("upper".to_string(), vec![3, 4])], "the inserted face joins no band, and every band member at or after it moves up by one");
}

#[test]
fn the_emitted_snapshot_carries_every_retained_concern() {
    let snapshot = oracle_snapshot_json(DOCUMENT.as_bytes()).unwrap();
    assert_eq!(snapshot.array("vertices").len(), 3);
    assert_eq!(snapshot.array("faces").len(), 1);
    assert_eq!(snapshot.str("mtllib"), "pattern.mtl");
    assert_eq!(snapshot.array("groups").len(), 1);
    assert_eq!(snapshot.array("objects").len(), 1);
    assert_eq!(snapshot.str("schema"), "stdio.obj");
    assert_eq!(snapshot.array("usemtl").len(), 1);
    assert_eq!(snapshot.array("smoothingGroups").len(), 1);
    assert_eq!(snapshot.array("unknownStatements"), vec![Json::Object(vec![("lineIndex".to_string(), Json::Number(0.0)), ("raw".to_string(), Json::String("# a retained comment".to_string()))])]);
}

#[test]
fn set_unknown_statements_and_smoothing_groups_read_the_leaf_wire_root_keys() {
    let statements = Json::Array(vec![Json::Object(vec![("lineIndex".to_string(), Json::Number(0.0)), ("raw".to_string(), Json::String("# replaced".to_string()))])]);
    let replaced = oracle_apply_mutation(DOCUMENT.as_bytes(), &spec("set-unknown-statements", Json::Object(vec![("unknownStatements".to_string(), statements)]))).unwrap();
    assert!(String::from_utf8(replaced).unwrap().contains("# replaced\n"));
    let ranges = Json::Array(vec![Json::Object(vec![("faceIndexFrom".to_string(), Json::Number(0.0)), ("group".to_string(), Json::Number(4.0))])]);
    let smoothed = oracle_apply_mutation(DOCUMENT.as_bytes(), &spec("set-smoothing-groups", Json::Object(vec![("smoothingGroups".to_string(), ranges)]))).unwrap();
    assert_eq!(oracle_snapshot_json(&smoothed).unwrap().array("smoothingGroups")[0].get("group"), Some(&Json::Number(4.0)));
}

#[test]
fn round_trip_re_renders_the_parsed_model() {
    assert_eq!(String::from_utf8(oracle_round_trip(DOCUMENT.as_bytes()).unwrap()).unwrap(), render(&parse(DOCUMENT).unwrap()));
}
