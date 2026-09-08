mod tests {
    use super::*;

    fn archive() -> IsoArchive {
        IsoArchive {
            entries: vec![IsoEntry { name: "bild.jpg".into(), data: b"jpegbytes".to_vec(), method: IsoMethod::Stored, encrypted: false }, IsoEntry { name: "notiz.txt".into(), data: b"text".to_vec(), method: IsoMethod::Deflate, encrypted: false }],
            comment: "Bestand".into(),
        }
    }

    fn spec(kind: &str, params: Vec<(&str, Json)>) -> Json {
        Json::Object(vec![("kind".into(), Json::String(kind.into())), ("params".into(), Json::Object(params.into_iter().map(|(k, v)| (k.to_string(), v)).collect()))])
    }

    #[test]
    fn both_declared_methods_survive_a_reference_write_and_read() {
        let bytes = write_archive(&archive()).expect("writes");
        let read = read_archive(&bytes).expect("reads");
        assert_eq!(read.entries.iter().find(|entry| entry.name == "bild.jpg").expect("member").method, IsoMethod::Stored);
        assert_eq!(read.entries.iter().find(|entry| entry.name == "notiz.txt").expect("member").method, IsoMethod::Deflate);
    }

    #[test]
    fn add_stored_entry_and_add_deflated_entry_are_different_operations() {
        let stored = apply(archive(), &spec("add-stored-entry", vec![("name", Json::String("neu.bin".into())), ("content", Json::String("payload".into()))])).expect("applies");
        let deflated = apply(archive(), &spec("add-deflated-entry", vec![("name", Json::String("neu.bin".into())), ("content", Json::String("payload".into()))])).expect("applies");
        assert_eq!(stored.entries.last().expect("member").method, IsoMethod::Stored);
        assert_eq!(deflated.entries.last().expect("member").method, IsoMethod::Deflate);
    }

    #[test]
    fn unknown_kind_is_an_error_not_a_no_op() {
        assert!(apply(archive(), &spec("add-entry", vec![])).is_err(), "the parent subset's ungated add-entry is not part of this vocabulary");
    }

    #[test]
    fn every_declared_kind_round_trips_through_its_own_inverse() {
        let specs = vec![
            spec("no-mutation", vec![]),
            spec("set-snapshot", vec![("entries", Json::Array(vec![Json::Object(vec![("name".into(), Json::String("x".into())), ("content".into(), Json::String("y".into()))])])), ("comment", Json::String("neu".into()))]),
            spec("set-archive-comment", vec![("comment", Json::String("geaendert".into()))]),
            spec("add-stored-entry", vec![("name", Json::String("a.png".into())), ("content", Json::String("p".into()))]),
            spec("add-deflated-entry", vec![("name", Json::String("a.txt".into())), ("content", Json::String("p".into()))]),
            spec("remove-entry", vec![("name", Json::String("notiz.txt".into()))]),
            spec("rename-entry", vec![("name", Json::String("notiz.txt".into())), ("newName", Json::String("notiz2.txt".into()))]),
            spec("set-entry-data", vec![("name", Json::String("notiz.txt".into())), ("content", Json::String("anders".into()))]),
        ];
        for one in specs {
            let base = archive();
            let mutated = apply(base.clone(), &one).expect("forward applies");
            let restored = invert(&base, mutated, &one).expect("inverse applies");
            assert_eq!(projection(&restored).to_string(), projection(&base).to_string(), "kind {} is not invertible", one.str("kind"));
        }
    }
}
