use semio_os_channel_archive::{archive_reopen_route::*, archive_schema::DOCUMENT_ARCHIVE_MAXIMUM_BYTES};

#[test]
fn reopen_route_matches_the_language_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧭️reopen-route/🔣️.json")).unwrap();
    assert_eq!(law["authority"].as_u64().unwrap() as usize, DOCUMENT_ARCHIVE_MAXIMUM_BYTES);
    assert_eq!(law["contiguousMaximum"].as_u64().unwrap() as usize, DOCUMENT_ARCHIVE_CONTIGUOUS_REOPEN_MAXIMUM_BYTES);
    for row in law["cases"].as_array().unwrap() {
        let route = select_document_archive_reopen_route(row["extent"].as_u64().unwrap() as usize, row["first"].as_u64().unwrap() as u8, row["last"].as_u64().unwrap() as u8);
        let expected = match row["route"].as_str().unwrap() { "contiguous" => DocumentArchiveReopenRoute::Contiguous, "paged" => DocumentArchiveReopenRoute::Paged, _ => DocumentArchiveReopenRoute::Refused };
        assert_eq!(route, expected, "{}", row["name"]);
    }
}
