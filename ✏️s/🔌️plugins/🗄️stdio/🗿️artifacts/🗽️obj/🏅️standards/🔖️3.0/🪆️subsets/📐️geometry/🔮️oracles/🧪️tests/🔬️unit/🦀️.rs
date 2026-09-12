
use super::*;

const DOCUMENT: &str = "# a retained comment\nmtllib pattern.mtl\nv 0 0 0\nv 1 0 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 0 1\nvn 0 0 1\ng band\no cap\nusemtl pattern\ns 1\nf 1/1/1 2/2/1 3/3/1\n";

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

#[test]
fn the_emitted_snapshot_is_what_set_snapshot_consumes() {
    let bytes = DOCUMENT.as_bytes();
    let snapshot = oracle_snapshot_json(bytes).unwrap();
    let replaced = oracle_apply_mutation(bytes, &spec("set-snapshot", Json::Object(vec![("snapshot".to_string(), snapshot)]))).unwrap();
    assert_eq!(String::from_utf8(replaced).unwrap(), render(&parse(DOCUMENT).unwrap()), "set-snapshot fed the document's own emitted snapshot must reproduce that document");
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
    assert_eq!(snapshot.array("usemtlRanges").len(), 1);
    assert_eq!(snapshot.array("smoothingGroups").len(), 1);
    assert_eq!(snapshot.array("unknownStatements").len(), 1);
}
