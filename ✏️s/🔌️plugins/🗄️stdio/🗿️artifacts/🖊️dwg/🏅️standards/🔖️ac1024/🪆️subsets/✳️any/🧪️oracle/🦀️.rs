//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: none — recorded no-oracle decision `dwg-ac1024-proprietary-container`
//! (`🔣️.json`). DWG is a proprietary, undocumented format. The only independent
//! implementation of any weight is LibreDWG, which is GPL-3.0 C: linking it would put a copyleft C
//! library on this repository's test host, and no owner ruling permits that. No permissively
//! licensed Rust DWG reader exists at all (`dxf`, the crate registered for 🖊️dxf, reads DXF — the
//! published interchange format — and explicitly not DWG).
//!
//! What CAN be read independently is the plain preamble every DWG file since R13 begins with, whose
//! offsets are published (LibreDWG's own `header.spec`) and are already cited by this subset's
//! `DwgSnapshot` doc comments:
//!
//! | bytes       | field                                                       | in the vocabulary? |
//! |-------------|-------------------------------------------------------------|--------------------|
//! | `0x00-0x05` | six ASCII version characters, e.g. `AC1024`                  | yes                |
//! | `0x06-0x0A` | five bytes the specification declares `0x00`                 | read and preserved |
//! | `0x0B`      | maintenance release version (`RC`) — `0x02` in the fixture   | read and preserved |
//! | `0x0C`      | marker byte, `0x00`/`0x01`/`0x03` — `0x03` in the fixture    | read and preserved |
//! | `0x0D-0x10` | preview (image seeker) address (`RL`) — `0x1c0` in the fixture | read and preserved |
//! | `0x11`      | application (DWG) version (`RC`) — `0x1d` in the fixture     | read and preserved |
//! | `0x12`      | `maint_version`, one byte (`RC`)                             | yes                |
//! | `0x13-0x14` | `codepage`, little-endian `u16` (`RS`) — `30` for ANSI_1252  | yes                |
//!
//! The whole 21-byte prefix is modelled, not only the three addressable fields, because
//! [`oracle_round_trip`] zeroes exactly that region before rewriting it: a field the reader does not
//! carry is a field the round trip destroys. Only the addressable three are PROJECTED — projecting
//! the others would claim a discrimination `DwgMutation` cannot exercise.
//!
//! Everything after that is the R2004+ section map: compressed, checksummed and section-encrypted,
//! and nothing in this repository or in the permissively licensed Rust ecosystem can regenerate it.
//! So this oracle is deliberately and visibly narrow: it reads and writes the preamble from the
//! specification directly, and carries the rest of the container through untouched. That narrowness
//! is recorded in the manifest's rationale and in the case's own feature description rather than
//! being hidden behind a projection that looks wider than it is.
//!
//! ⚠️ `../../../../🔖️ac1018/🪆️subsets/✳️any/🦀️oracle.rs` is a `pub use` of THIS module, for
//! the same reason `../../../../🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` is a `pub use` of this
//! standard's schema: the AC1018 subset does not declare a vocabulary of its own. See that file.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Kinds
/// 🏷️ The declared vocabulary of this subset, mirroring the production `KINDS`
/// (`../🧬️schema/🧬️mutations/🦀️.rs`, itself checked there against `DwgMutation::kind()`
/// and against BOTH DWG catalogs) in declaration order. Duplicated rather than imported: the oracle
/// crate must never link the production crate, so this side can only compare STRINGS; the check
/// that a kind exists as a real enum variant is the production-side test's.
pub const KINDS: [&str; 3] = ["no-mutation", "set-snapshot", "set-version-info"];
//#endregion 🔖️Kinds

//#region 🔖️Preamble
/// 📐️ Byte offsets of the plain file-header preamble, sourced from LibreDWG's `header.spec` field
/// order — the same source this subset's own production `DwgSnapshot` doc comments already cite,
/// where `parse_version_header_fields` calls it "the plain file-header preamble shared by every
/// AC1015+ DWG file". ALL of it, not only the three fields this subset's vocabulary addresses — see
/// [`Preamble`] for why that distinction matters — but no further than `0x15`, which is exactly as
/// far as that citation reaches; see
/// [`the_shared_layout_claim_stops_where_the_modelled_preamble_stops`](tests::the_shared_layout_claim_stops_where_the_modelled_preamble_stops).
const VERSION_RANGE: std::ops::Range<usize> = 0..6;
const RESERVED_RANGE: std::ops::Range<usize> = 0x06..0x0B;
const RELEASE_MAINTENANCE_OFFSET: usize = 0x0B;
const MARKER_OFFSET: usize = 0x0C;
const PREVIEW_ADDRESS_RANGE: std::ops::Range<usize> = 0x0D..0x11;
const APPLICATION_VERSION_OFFSET: usize = 0x11;
const MAINTENANCE_OFFSET: usize = 0x12;
const CODEPAGE_RANGE: std::ops::Range<usize> = 0x13..0x15;
/// 📏️ The smallest prefix that carries the whole published header — a document shorter than this
/// has no readable preamble at all.
const PREAMBLE_LEN: usize = 0x15;

/// 🧱️ The DWG preamble, as far as it is publicly specified — EVERY byte of `0x00..0x15`, not only
/// the three fields `DwgMutation` can address.
///
/// ⚠️ Modelling the whole 21 bytes is a correctness requirement, not thoroughness for its own sake:
/// [`oracle_round_trip`] zeroes exactly this region before rewriting it, so any byte the struct does
/// not carry is a byte the round trip DESTROYS. An earlier revision of this module modelled only
/// `version`/`maintenance_version`/`codepage` and silently wiped `0x06..0x12` — the real
/// `architectural.dwg` fixture carries `0x02` at 0x0B, `0x03` at 0x0C, the preview-image seeker
/// `0x000001c0` at 0x0D-0x10 and application version `0x1d` at 0x11, all of them real published
/// fields. `the_round_trip_rebuilds_the_preamble_from_the_parse_alone` caught it, which is exactly
/// what an exact-bytes law is for.
///
/// The four fields beyond the addressable triple are READ and PRESERVED but not projected: no
/// declared mutation kind edits them, so projecting them would claim discrimination the vocabulary
/// cannot exercise. `reserved` is likewise read rather than assumed zero — the specification says
/// five `0x00` bytes there, and a file that disagrees must round-trip as itself rather than as what
/// the specification wishes it were.
struct Preamble {
    version: String,
    reserved: [u8; 5],
    release_maintenance: u8,
    marker: u8,
    preview_address: u32,
    application_version: u8,
    maintenance_version: u8,
    codepage: u16,
}

/// 🔎️ Reads the preamble from the specification's own offsets. A six-character version string that
/// is not `AC` + four digits is refused: this reader must not silently accept a file that is not a
/// DWG at all and then report a projection about it.
fn read_preamble(input: &[u8]) -> Result<Preamble, String> {
    if input.len() < PREAMBLE_LEN {
        return Err(format!("a DWG preamble needs at least {PREAMBLE_LEN} bytes; this document has {}", input.len()));
    }
    let version = String::from_utf8(input[VERSION_RANGE].to_vec()).map_err(|error| format!("the six version bytes are not ASCII: {error}"))?;
    if !(version.starts_with("AC") && version.len() == 6 && version[2..].bytes().all(|byte| byte.is_ascii_digit())) {
        return Err(format!("version string {version:?} is not the `AC` + four digits every DWG file since R13 begins with"));
    }
    let mut reserved = [0u8; 5];
    reserved.copy_from_slice(&input[RESERVED_RANGE]);
    let mut preview = [0u8; 4];
    preview.copy_from_slice(&input[PREVIEW_ADDRESS_RANGE]);
    Ok(Preamble {
        version,
        reserved,
        release_maintenance: input[RELEASE_MAINTENANCE_OFFSET],
        marker: input[MARKER_OFFSET],
        preview_address: u32::from_le_bytes(preview),
        application_version: input[APPLICATION_VERSION_OFFSET],
        maintenance_version: input[MAINTENANCE_OFFSET],
        codepage: u16::from_le_bytes([input[CODEPAGE_RANGE.start], input[CODEPAGE_RANGE.start + 1]]),
    })
}

/// 🏷️ Writes the preamble back into `document` at the specification's own offsets — every byte of
/// `0x00..0x15`, so a zeroed region is fully re-derived rather than partly restored.
fn write_preamble(document: &mut [u8], preamble: &Preamble) -> Result<(), String> {
    if document.len() < PREAMBLE_LEN {
        return Err(format!("a DWG preamble needs at least {PREAMBLE_LEN} bytes; this document has {}", document.len()));
    }
    if preamble.version.len() != 6 || !preamble.version.is_ascii() {
        return Err(format!("version string {:?} is not six ASCII characters", preamble.version));
    }
    document[VERSION_RANGE].copy_from_slice(preamble.version.as_bytes());
    document[RESERVED_RANGE].copy_from_slice(&preamble.reserved);
    document[RELEASE_MAINTENANCE_OFFSET] = preamble.release_maintenance;
    document[MARKER_OFFSET] = preamble.marker;
    document[PREVIEW_ADDRESS_RANGE].copy_from_slice(&preamble.preview_address.to_le_bytes());
    document[APPLICATION_VERSION_OFFSET] = preamble.application_version;
    document[MAINTENANCE_OFFSET] = preamble.maintenance_version;
    document[CODEPAGE_RANGE].copy_from_slice(&preamble.codepage.to_le_bytes());
    Ok(())
}

/// 🌱 A whole DWG document that is nothing but a preamble — byte for byte the shape this artifact's
/// own `📚️examples/🎬️demo/🖼️assets/🖊️example.dwg` already has (22 bytes: six version characters
/// then zeros). This is what `set-snapshot` builds when it is given fields rather than a document: a
/// genuine whole-document replacement, observable as a collapse in `byteLength`, not a field-set
/// dressed up as one.
///
/// The fields the vocabulary cannot address are ZEROED here rather than inherited, and that is the
/// point of the verb: a fresh preamble-only document has no preview image, so carrying the source
/// container's `preview_address` into it would point 0x1c0 bytes past the end of a 22-byte file.
fn stub_document(preamble: &Preamble) -> Result<Vec<u8>, String> {
    let fresh = Preamble {
        version: preamble.version.clone(),
        reserved: [0u8; 5],
        release_maintenance: 0,
        marker: 0,
        preview_address: 0,
        application_version: 0,
        maintenance_version: preamble.maintenance_version,
        codepage: preamble.codepage,
    };
    let mut document = vec![0u8; 22];
    write_preamble(&mut document, &fresh)?;
    Ok(document)
}
//#endregion 🔖️Preamble

//#region 🔖️SpecReaders
fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}

fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(found)) => Some(*found),
        _ => None,
    }
}

/// 🧭️ The preamble a spec's params describe, defaulting each field to the document's current value
/// so a scenario states only what it changes. Only the three fields the vocabulary declares are
/// addressable; the rest of the published header rides along from `current` untouched, which is what
/// makes `set-version-info` a field set rather than a header rewrite.
fn preamble_from(params: &Json, current: &Preamble) -> Preamble {
    Preamble {
        version: match params.get("version") {
            Some(Json::String(found)) => found.clone(),
            _ => current.version.clone(),
        },
        reserved: current.reserved,
        release_maintenance: current.release_maintenance,
        marker: current.marker,
        preview_address: current.preview_address,
        application_version: current.application_version,
        maintenance_version: number(params, "maintenanceVersion").map(|found| found as u8).unwrap_or(current.maintenance_version),
        codepage: number(params, "codepage").map(|found| found as u16).unwrap_or(current.codepage),
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 {
        return Err(format!("hex payload has an odd length ({})", text.len()));
    }
    (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).map_err(|error| format!("hex payload is malformed at pair {index}: {error}"))).collect()
}
//#endregion 🔖️SpecReaders

//#region 🔖️Projection
/// 🎯️ The projection `semantic-dwg-preamble-v1` compares: the three publicly specified preamble
/// fields plus the document's own length. `byteLength` is normative HERE precisely because the rest
/// of the container is opaque — it is the only way a whole-document replacement is distinguishable
/// from a field set, and neither side has any writer freedom over it.
pub fn project_dwg(bytes: &[u8]) -> Result<Json, String> {
    let preamble = read_preamble(bytes)?;
    Ok(Json::Object(vec![
        ("version".to_string(), Json::String(preamble.version)),
        ("maintenanceVersion".to_string(), Json::Number(f64::from(preamble.maintenance_version))),
        ("codepage".to_string(), Json::Number(f64::from(preamble.codepage))),
        ("byteLength".to_string(), Json::Number(bytes.len() as f64)),
    ]))
}
//#endregion 🔖️Projection

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
///
/// * `set-version-info` sets the three preamble fields IN PLACE, leaving the section map and every
///   byte of the body exactly where it was — which is what makes it applicable to a real 148 KB
///   R2010 container this repository can decode but nothing here can rebuild.
/// * `set-snapshot` REPLACES the whole document: with `documentHex` when one is given (the form
///   [`oracle_inverse_spec`] produces, since restoring a proprietary container is not expressible
///   any other way), otherwise with a fresh preamble-only stub carrying the stated fields.
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = params_of(spec);
    let current = read_preamble(input)?;
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-version-info" => {
            let mut document = input.to_vec();
            write_preamble(&mut document, &preamble_from(&params, &current))?;
            Ok(document)
        }
        "set-snapshot" => match params.get("documentHex") {
            Some(Json::String(text)) => hex_decode(text),
            _ => stub_document(&preamble_from(&params, &current)),
        },
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}
//#endregion 🔖️Dispatch

//#region 🔖️Inverse
/// ↩️ The independently computed inverse of `spec` against the UNMUTATED `base`, matching
/// `DwgMutation::inverse()`'s own base-relative semantics: `set-version-info` inverts to the three
/// fields `base` already carried, and `set-snapshot` inverts to `base` itself — which for a
/// proprietary container means its whole byte image, read out of `base` here and never authored by
/// hand.
pub fn oracle_inverse_spec(base: &[u8], spec: &Json) -> Result<Json, String> {
    let preamble = read_preamble(base)?;
    let params = match spec.str("kind").as_str() {
        "no-mutation" => Json::Object(vec![]),
        "set-version-info" => Json::Object(vec![
            ("version".to_string(), Json::String(preamble.version)),
            ("maintenanceVersion".to_string(), Json::Number(f64::from(preamble.maintenance_version))),
            ("codepage".to_string(), Json::Number(f64::from(preamble.codepage))),
        ]),
        "set-snapshot" => Json::Object(vec![("documentHex".to_string(), Json::String(hex_encode(base)))]),
        kind => return Err(format!("mutation kind {kind:?} has no oracle inverse")),
    };
    Ok(Json::Object(vec![("kind".to_string(), Json::String(spec.str("kind"))), ("params".to_string(), params)]))
}
//#endregion 🔖️Inverse

//#region 🔖️RoundTrip
/// 🔁️ Decodes the preamble into typed fields and re-encodes it from those fields alone. The
/// preamble region is ZEROED first, so a byte-identical result cannot come from a `memcpy` that
/// never parsed anything: every one of those 21 bytes has to be re-derived from the parse.
///
/// 🔒️ This carrier is bound by [`carrier_is_exact`](semio_s_plugin_stdio_test_oracle::law::carrier_is_exact),
/// NOT by the no-byte-pass-through law. Reproducing the input exactly is the CORRECT answer here and
/// anything else is the defect: the preamble is fixed-width with no writer freedom whatsoever, and
/// the R2004+ section map that follows is a compressed, checksummed, proprietary structure that
/// neither this repository nor any permissively licensed Rust crate can regenerate, so it is carried
/// through unchanged by construction. Demanding a byte difference would be a fabricated law.
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    let preamble = read_preamble(input)?;
    let mut document = input.to_vec();
    for byte in document[..PREAMBLE_LEN].iter_mut() {
        *byte = 0;
    }
    write_preamble(&mut document, &preamble)?;
    Ok(document)
}
//#endregion 🔖️RoundTrip

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🖊️ The real committed drawing — 148,638 bytes of a genuine architectural DWG. ⚠️ It is
    /// filed under the ac1018 example tree but its version string is `AC1024`: it is an R2010
    /// container. See this subset's `🔣️.json` and both DWG cases' feature descriptions.
    fn fixture() -> Vec<u8> {
        include_bytes!("../../../../🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg").to_vec()
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
        let demo = include_bytes!("../../../../🔖️ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🖊️example.dwg").to_vec();
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
        let vocabulary = include_str!("../🧬️schema/🧬️mutations/🦀️.rs");
        let variants = ["SetSnapshot", "SetVersionInfo"];
        assert_eq!(KINDS.len(), variants.len() + 1, "no-mutation is an oracle-only identity scenario with no DwgMutation variant of its own");
        for manifest in [include_str!("🔣️.json"), include_str!("../../../../🔖️ac1018/🪆️subsets/✳️any/🔣️oracle.json")] {
            for kind in KINDS {
                assert!(manifest.contains(&format!("\"{kind}\"")), "a committed DWG catalog is missing kind {kind:?}");
            }
        }
        for (kind, variant) in KINDS.iter().skip(1).zip(variants.iter()) {
            assert!(vocabulary.contains(&format!("{variant} ")) || vocabulary.contains(&format!("{variant},")) || vocabulary.contains(&format!("{variant} {{")), "DwgMutation is missing variant {variant:?} for kind {kind:?}");
        }
        for feature in [include_str!("../../../../../🧪️tests/mutate-dwg-ac1024/🥒️.feature"), include_str!("../../../../../🧪️tests/mutate-dwg-ac1018/🥒️.feature")] {
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
        for (facet, source) in [
            ("mutations", include_str!("../../../../🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs")),
            ("schema", include_str!("../../../../🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🦀️component.rs")),
            ("snapshot", include_str!("../../../../🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs")),
            ("oracle", include_str!("../../../../🔖️ac1018/🪆️subsets/✳️any/🦀️oracle.rs")),
        ] {
            assert!(source.contains("pub use crate::artifacts::dwg::standards::v_ac1024::subsets::any::"), "the ac1018 {facet} facet is no longer a re-export of ac1024's — the two catalogs can no longer claim to be identical by construction");
        }
    }
}
//#endregion 🧪️Tests
