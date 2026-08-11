//! ⚙️ Ifc2x3Engine — real IFC2X3 SPF (ISO 10303-21 Part-21) decode/encode. IFC2X3 is buildingSMART
//! Coordination View 2.0-era IFC, ISO/PAS 16739:2005 schema, physically-encoded identically to
//! `📐️step`'s AP214 (`FILE_SCHEMA(('IFC2X3'))` in place of `FILE_SCHEMA(('AUTOMOTIVE_DESIGN'))`).
//! Reuses `step::engine::part21`'s tokenizer/writer functions directly (already `pub` — no
//! visibility change needed, unlike the ticket's contingency plan) — that is PARSING-CODE reuse,
//! explicitly allowed; what's NOT reused is `Part21Document`'s type IDENTITY as this standard's
//! snapshot type (see `🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`'s own doc comment).
//! This file owns the standard-specific layer parsing code alone can't provide: FILE_SCHEMA
//! validation (decode refuses non-IFC2X3 input) and the genuine round-trip test this ticket's law
//! requires (`POLICY_ROUND_TRIP_TEST_ALLOWLIST` is shrink-only — new standards never get added).

use crate::artifacts::step::engine::part21::{parse_part21, write_part21};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::Ifc2x3Mutation;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::{Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::Ifc2x3Artifact;

//#region 🔖️Codec
/// 📐️ The IFC2X3 FILE_SCHEMA name a conforming Part-21 file must declare.
pub const IFC2X3_SCHEMA_NAME: &str = "IFC2X3";

/// 📥️ Decodes IFC2X3 SPF bytes into an [`Ifc2x3Snapshot`]. Real standard-specific validation
/// beyond generic Part-21 parsing: rejects any file whose `FILE_SCHEMA` doesn't declare
/// `IFC2X3` (so this decoder never silently accepts an IFC4 or plain STEP AP214 file).
pub fn decode_ifc2x3(bytes: &[u8]) -> Result<Ifc2x3Snapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("ifc2x3: not valid utf-8: {e}"))?;
    let document = parse_part21(text).map_err(|e| format!("ifc2x3 parse: {e}"))?;
    let declares_ifc2x3 = document.header.file_schema.iter().any(|v| {
        v.as_list()
            .map(|items| items.iter().any(|item| item.as_str() == Some(IFC2X3_SCHEMA_NAME)))
            .unwrap_or(false)
    });
    if !declares_ifc2x3 {
        return Err(format!("ifc2x3: FILE_SCHEMA does not declare {IFC2X3_SCHEMA_NAME}"));
    }
    Ok(Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document })
}

/// 📤️ Regenerates valid IFC2X3 SPF bytes from a snapshot. Losslessness is `write_part21`'s job
/// (shared with `step`/`4`); this function's only own contribution is the byte encoding.
pub fn encode_ifc2x3(snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
    Ok(write_part21(&snapshot.document).into_bytes())
}

pub fn empty_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    Ifc2x3Snapshot::default()
}
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers this standard's schema descriptor, document codec, and (via each real subset's
/// own composer) its `SubsetValidator`s. Does NOT call the artifact-level `ifc::composer::register()`
/// (that union is already invoked once from `4`'s own `engine::register()`, extended by this
/// ticket to also union `v2x3::composer::entries()` — calling it a second time here would be a
/// redundant registration, same reasoning gif's `89a::engine::register` doc comment gives).
pub fn register() {
    ::schema::register_artifact_schema_descriptor(
        crate::artifacts::ifc::standards::v2x3::subsets::any::schema::ifc2x3_artifact_schema_descriptor(),
    );
    store::register_document_codec(store::ArtifactCodec::of::<Ifc2x3Snapshot, Ifc2x3Mutation>(STDIO_IFC2X3_DOCUMENT_SCHEMA));
    // 🛡️ D5's generic validate-on-build hook: registers each real subset's `SubsetValidator` so
    // `io_dispatch`/`wire_artifact_compose` re-check them for free. Each subset's `ComposerEntry`
    // is registered separately via this standard's own `composer::entries()` aggregation.
    crate::artifacts::ifc::standards::v2x3::subsets::cv20::composer::register();
    crate::artifacts::ifc::standards::v2x3::subsets::sav::composer::register();
    crate::artifacts::ifc::standards::v2x3::subsets::cobie::composer::register();
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
pub struct Ifc2x3Engine {
    artifact_state: Ifc2x3Artifact,
    snapshot_state: Ifc2x3Snapshot,
}

impl Ifc2x3Engine {
    pub fn new(snapshot: Ifc2x3Snapshot) -> Self {
        let artifact_state = Ifc2x3Artifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const IFC2X3_FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-11T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('0YvctVUKr0kugbFTf53O9L',$,'Project',$,$,$,$,(#20),#30);\n#20=IFCUNITASSIGNMENT((#21));\n#21=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#30=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#31,$);\n#31=IFCAXIS2PLACEMENT3D(#32,$,$);\n#32=IFCCARTESIANPOINT((0.,0.,0.));\n#40=IFCBUILDING('0YvctVUKr0kugbFTf53O9M',$,'Building',$,$,#41,$,$,.ELEMENT.,$,$,$);\n#41=IFCLOCALPLACEMENT($,#31);\nENDSEC;\nEND-ISO-10303-21;\n";

    #[test]
    fn decode_rejects_non_ifc2x3_schema() {
        let step_ap214 = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert!(decode_ifc2x3(step_ap214.as_bytes()).is_err(), "must reject a non-IFC2X3 FILE_SCHEMA");
        let ifc4 = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert!(decode_ifc2x3(ifc4.as_bytes()).is_err(), "must reject IFC4 too -- 2x3 is a distinct standard, not a superset reader");
    }

    /// 🧪️ THE genuine decode→encode→decode round-trip law this ticket's own policy requires
    /// (`POLICY_ROUND_TRIP_TEST_ALLOWLIST` is shrink-only for new standards).
    #[test]
    fn decode_encode_decode_round_trip_is_lossless() {
        let once = decode_ifc2x3(IFC2X3_FIXTURE.as_bytes()).expect("decode fixture");
        // 🩹 8 distinct instance ids in IFC2X3_FIXTURE: #1, #20, #21, #30, #31, #32, #40, #41.
        assert_eq!(once.document.instances.len(), 8);
        assert!(once.document.by_type("IFCPROJECT").next().is_some());
        let bytes = encode_ifc2x3(&once).expect("encode");
        let twice = decode_ifc2x3(&bytes).expect("decode re-encoded bytes");
        assert_eq!(once, twice, "decode -> encode -> decode must be lossless at the snapshot level");
    }

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_ifc2x3_snapshot();
        assert_eq!(snapshot.schema, STDIO_IFC2X3_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip_through_store_traits() {
        let snap = decode_ifc2x3(IFC2X3_FIXTURE.as_bytes()).expect("decode");
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse_dsl");
        assert_eq!(parsed, snap);
        let packed = store::ArtifactPack::encode_pack(&snap);
        let decoded = <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode_pack");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
