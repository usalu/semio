
use super::*;

/// 🖊️ The real committed drawing — 148,638 bytes of a genuine architectural DWG. ⚠️ It is
/// filed under the ac1018 example tree but its version string is `AC1024`: it is an R2010
/// container. See this subset's `🔣️.json` and both DWG cases' feature descriptions.
fn fixture() -> Vec<u8> {
    include_bytes!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🖼️assets/🏛️architectural/🏛️architectural.dwg").to_vec()
}

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

fn object(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

#[test]
fn the_committed_drawing_reads_the_values_the_published_offsets_predict() {
    let projection = project_dwg(&fixture()).unwrap();
    assert_eq!(projection.get("version").unwrap().clone(), Json::String("AC1024".to_string()));
    assert_eq!(projection.get("maintenanceVersion").unwrap().clone(), Json::Number(2.0), "LibreDWG's header.spec puts maint_version at 0x12; this file carries 0x02 there");
    assert_eq!(projection.get("codepage").unwrap().clone(), Json::Number(30.0), "0x13-0x14 is the codepage RS; 30 is ANSI_1252");
    assert_eq!(projection.get("byteLength").unwrap().clone(), Json::Number(148_638.0));
}

/// 📐️ Every published field of the R2004+ header prefix, read off the real fixture at the
/// specification's own offsets. This is the test that would have caught the earlier revision's
/// silent wipe of `0x06..0x12`, and it is what entitles [`oracle_round_trip`] to zero the whole
/// region: a field asserted here is a field the writer is required to put back.
#[test]
fn the_whole_published_header_prefix_reads_the_values_the_fixture_carries() {
    let preamble = read_preamble(&fixture()).unwrap();
    assert_eq!(preamble.version, "AC1024");
    assert_eq!(preamble.reserved, [0u8; 5], "the specification declares 0x06-0x0A as five zero bytes");
    assert_eq!(preamble.release_maintenance, 0x02, "maintenance release version at 0x0B");
    assert_eq!(preamble.marker, 0x03, "the 0x00/0x01/0x03 marker at 0x0C");
    assert_eq!(preamble.preview_address, 0x0000_01c0, "preview (image seeker) address at 0x0D-0x10, little-endian");
    assert_eq!(preamble.application_version, 0x1d, "application (DWG) version at 0x11");
    assert_eq!(preamble.maintenance_version, 0x02);
    assert_eq!(preamble.codepage, 30);
}

/// 🌱 `set-snapshot`'s stub is byte-identical to this artifact's own committed 22-byte demo
/// example when it carries that file's own fields — the shape is read off a real committed file
/// rather than invented, and the non-addressable fields are reset rather than inherited.
#[test]
fn the_whole_document_replacement_matches_the_committed_preamble_only_example() {
    let demo = include_bytes!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🧪️example/🖊️.dwg").to_vec();
    let built = oracle_apply_mutation(&fixture(), &spec("set-snapshot", object(vec![("maintenanceVersion", Json::Number(0.0)), ("codepage", Json::Number(0.0))]))).unwrap();
    assert_eq!(built, demo, "the stub must reproduce the committed preamble-only example, including the fields no mutation kind addresses");
}

/// 🚧 WHERE the shared-layout claim stops, pinned as bytes rather than left as prose. The two
/// DWG cases rest on AC1018 and AC1024 sharing a header layout, and that claim is made ONLY for
/// `0x00..0x15` — the region [`Preamble`] models — because that is the region this repository's
/// own production conformance code cites a source for: `DwgSnapshot`'s doc comments name
/// LibreDWG's `header.spec` field order (`dwg_version@0x11`, `maint_version@0x12`,
/// `codepage@0x13`) and `parse_version_header_fields` calls it "the plain file-header preamble
/// shared by every AC1015+ DWG file".
///
/// Immediately past it the ground is NOT established, and this test records why rather than
/// letting a wider claim ride along unchecked. The R2004 file header is customarily documented
/// with three `0x00` bytes at `0x15`; the real AC1024 fixture carries `00 1d 02` there, which
/// repeats the application version/maintenance pair from `0x11`-`0x12`. Whatever those bytes
/// are, they are not three zeros, so nothing here is entitled to say the two releases share a
/// header layout BEYOND the preamble — and the oracle deliberately stops at `PREAMBLE_LEN`
/// instead. If a future revision widens the modelled region, this test fails first and the
/// claim has to be re-sourced before the code can move.
#[test]
fn the_shared_layout_claim_stops_where_the_modelled_preamble_stops() {
    let input = fixture();
    assert_eq!(PREAMBLE_LEN, 0x15, "the modelled region ends immediately after the codepage");
    assert_eq!(&input[0x15..0x18], &[0x00, 0x1d, 0x02], "the real AC1024 fixture does not carry the three 0x00 bytes the R2004 header layout is customarily documented with");
    assert_eq!(input[0x16], input[0x11], "0x16 repeats the application version byte");
    assert_eq!(input[0x17], input[0x12], "0x17 repeats the application maintenance byte");
}

#[test]
fn a_document_that_is_not_a_dwg_is_refused_rather_than_projected() {
    assert!(project_dwg(b"not a drawing at all, but long enough").is_err());
    assert!(project_dwg(b"AC10").unwrap_err().contains("at least"));
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let input = fixture();
    assert_eq!(oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).unwrap(), input);
}

#[test]
fn every_kind_is_observable_and_its_own_inverse_restores_the_projection() {
    let input = fixture();
    let original = project_dwg(&input).unwrap();
    let cases = vec![
        spec("no-mutation", Json::Object(vec![])),
        spec("set-version-info", object(vec![("version", Json::String("AC1032".to_string())), ("maintenanceVersion", Json::Number(7.0)), ("codepage", Json::Number(29.0))])),
        spec("set-snapshot", object(vec![("version", Json::String("AC1018".to_string())), ("maintenanceVersion", Json::Number(0.0)), ("codepage", Json::Number(0.0))])),
    ];
    for case in cases {
        let kind = case.str("kind");
        let mutated = oracle_apply_mutation(&input, &case).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
        let after = project_dwg(&mutated).unwrap();
        if kind != "no-mutation" {
            assert_ne!(after, original, "{kind} left the projection unchanged — a mutation that is not observable proves nothing");
        }
        let inverse = oracle_inverse_spec(&input, &case).unwrap();
        let restored = oracle_apply_mutation(&mutated, &inverse).unwrap_or_else(|error| panic!("{kind} inverse failed: {error}"));
        assert_eq!(project_dwg(&restored).unwrap(), original, "applying {kind} and then its own inverse must restore the original projection");
    }
}

#[test]
fn set_snapshot_is_a_whole_document_replacement_and_set_version_info_is_not() {
    let input = fixture();
    let fields = object(vec![("version", Json::String("AC1018".to_string()))]);
    let snapshot = oracle_apply_mutation(&input, &spec("set-snapshot", fields.clone())).unwrap();
    let version_info = oracle_apply_mutation(&input, &spec("set-version-info", fields)).unwrap();
    assert_eq!(snapshot.len(), 22, "set-snapshot replaces the container outright");
    assert_eq!(version_info.len(), input.len(), "set-version-info leaves the section map exactly where it was");
    assert_ne!(project_dwg(&snapshot).unwrap(), project_dwg(&version_info).unwrap(), "the two verbs must be distinguishable in the projection, not two names for one edit");
}

#[test]
fn the_round_trip_rebuilds_the_preamble_from_the_parse_alone() {
    let input = fixture();
    let output = oracle_round_trip(&input).unwrap();
    assert_eq!(output, input, "the preamble region is zeroed before it is written back, so equality here proves the parse/write pair is exact — not that the bytes were copied");
    assert_eq!(project_dwg(&output).unwrap(), project_dwg(&input).unwrap());
}

#[test]
fn an_unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(&fixture(), &spec("set-header-var", Json::Object(vec![]))).is_err());
}

/// 🏷️ `KINDS` must equal both DWG standards' committed catalogs AND the committed production
/// vocabulary. The framework never parses Rust, so the catalog is what the contract gate counts
/// against; this reads the files as text and fails the moment any of them drift apart. Both
/// catalogs are checked from here because the AC1018 subset re-exports THIS vocabulary — see
/// `every_ac1018_facet_is_a_re_export_of_this_one` below.
#[test]
fn kinds_match_both_catalogs_and_the_vocabulary() {
    let vocabulary = include_str!("../../../🧬️schema/🧬️mutations/🦀️.rs");
    let variants = ["SetSnapshot", "SetVersionInfo"];
    assert_eq!(KINDS.len(), variants.len() + 1, "no-mutation is an oracle-only identity scenario with no DwgMutation variant of its own");
    for manifest in [include_str!("../../🔣️.json"), include_str!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🔮️oracle/🔣️.json")] {
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "a committed DWG catalog is missing kind {kind:?}");
        }
    }
    for (kind, variant) in KINDS.iter().skip(1).zip(variants.iter()) {
        assert!(vocabulary.contains(&format!("{variant} ")) || vocabulary.contains(&format!("{variant},")) || vocabulary.contains(&format!("{variant} {{")), "DwgMutation is missing variant {variant:?} for kind {kind:?}");
    }
    for feature in [include_str!("../../../🧪️tests/🖊️mutate-dwg-ac1024/🥒️.feature"), include_str!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🧪️tests/🖊️mutate-dwg-ac1018/🥒️.feature")] {
        for kind in KINDS {
            assert!(feature.contains(&format!("| {kind} ")) || feature.contains(&format!("| {kind}  ")), "a DWG case's Examples table is missing kind {kind:?}");
        }
    }
}

/// 🧬️ The claim both DWG cases rest on, checked instead of asserted in prose: AC1018 declares
/// no vocabulary, schema or snapshot of its own — every one of those facets is a `pub use` of
/// this standard's. The two catalogs are therefore identical BY CONSTRUCTION, not by a
/// copy-paste that could silently rot.
#[test]
fn every_ac1018_facet_is_a_re_export_of_this_one() {
    for (facet, source, root) in [
        ("mutations", include_str!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"), "crate::standards"),
        ("schema", include_str!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🦀️.rs"), "crate::standards"),
        ("snapshot", include_str!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"), "crate::standards"),
        ("oracle", include_str!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🔮️oracle/🦀️.rs"), "crate::artifacts::dwg::standards"),
    ] {
        assert!(source.contains(&format!("pub use {root}::v_ac1024::subsets::any::")), "the ac1018 {facet} facet is no longer a re-export of ac1024's — the two catalogs can no longer claim to be identical by construction");
    }
}
