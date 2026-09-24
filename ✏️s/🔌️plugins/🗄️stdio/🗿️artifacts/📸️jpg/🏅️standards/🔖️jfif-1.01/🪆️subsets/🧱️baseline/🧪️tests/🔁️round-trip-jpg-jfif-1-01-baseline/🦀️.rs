//! 🦀️ JFIF 1.01 🧱️baseline decode/re-encode case — Rust adapter.
//!
//! The round trip is judged by laws and by the INDEPENDENT `image` reader the sibling `🧾️document`
//! subset registers, inside the subject handler: this repository's encoder writes a conforming baseline
//! file with its own Annex K tables, so no third party can re-derive its bytes (recorded as the
//! `jpg-jfif-1-01-baseline-round-trip-normalization` no-oracle decision).

use semio_repo_test_host::Adapter;

//#region 🔖️Input
/// 🖼️ The real 2275x2560 architectural scan, shared with the `🧾️document` case rather than copied.
#[cfg(feature = "sut")]
const SCAN: &str = "shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg";
//#endregion 🔖️Input

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_stdio_jpg::io::{decode_jpg, encode_jpg};
    use semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::{encode_jpg_baseline_projection_json, jpg_baseline_conformance_codes};
    use semio_s_artifact_stdio_jpg::JpgSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::jpg::standards::v_jfif_1_01::subsets::document::project_jpg_mutation;
    use semio_s_plugin_stdio_test_oracle::law;

    /// 🖼️ The real scan, decoded into the snapshot the whole case reasons about. The decode is
    /// required to have retained a frame header: `check_baseline_conformance` reports
    /// `stdio.jpg.baseline.no-frame` and certifies nothing without one, so a case that let a
    /// frameless snapshot through would be measuring the absence of the document.
    fn decoded(ctx: &Context) -> Result<JpgSnapshot, String> {
        let snapshot = decode_jpg(&ctx.fixture_bytes(super::SCAN)?).map_err(|error| format!("mutate-jpg-jfif-1-01-baseline: the committed scan must decode: {error:?}"))?;
        let frame = snapshot.frame.as_ref().ok_or("mutate-jpg-jfif-1-01-baseline: the decode retained no SOF0 frame header, so no conformance axis exists to move")?;
        if frame.components.len() != 3 || snapshot.huffman_tables.len() != 4 {
            return Err(format!(
                "mutate-jpg-jfif-1-01-baseline: this case's params are addressed at the committed scan's own SOF0 (three components) and four DHT tables, but the decode read {} component(s) and {} table(s)",
                frame.components.len(),
                snapshot.huffman_tables.len()
            ));
        }
        Ok(snapshot)
    }

    fn projection(snapshot: &JpgSnapshot) -> Result<Json, String> {
        parse_json(&encode_jpg_baseline_projection_json(snapshot))
    }

    /// 🔁️ The one scenario in this case that touches bytes. The scan is decoded into the typed
    /// snapshot and re-serialized from that snapshot ALONE — no splice, no copy — and the result
    /// must differ from the input, which it genuinely does: `encode_jpg` regenerates fresh Annex K
    /// DQT/DHT tables at its own re-encode quality rather than carrying the source's forward, so
    /// bit-identical output here would mean the input was smuggled rather than parsed. The geometry
    /// claim is then made by the INDEPENDENT `image`-backed reader on both sides, not by this
    /// repository's codec agreeing with itself, and the class verdict is required to survive the
    /// round trip — which is the strongest statement this subset can make about its own encoder.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(super::SCAN)?;
        let base = decoded(ctx)?;
        let bytes = encode_jpg(&base).map_err(|error| format!("identity-round-trip: re-serializing the decoded scan failed: {error:?}"))?;
        law::reparsed_not_copied(&bytes, &input)?;
        let reparsed = decode_jpg(&bytes).map_err(|error| format!("identity-round-trip: the re-encoded scan must decode again: {error:?}"))?;
        let verdict = jpg_baseline_conformance_codes(&reparsed);
        if !verdict.is_empty() {
            return Err(format!("identity-round-trip: a decode/re-encode of a conforming scan must stay inside the class, but the result reports {verdict:?}"));
        }
        let (was, now) = (project_jpg_mutation(&input)?, project_jpg_mutation(&bytes)?);
        if was.str("dimensions") != now.str("dimensions") {
            return Err(format!("identity-round-trip: the independent reader sees {} in and {} out — the re-serialization changed the image geometry", was.str("dimensions"), now.str("dimensions")));
        }
        let (was, now) = (projection(&base)?, projection(&reparsed)?);
        law::round_trip_preserves(&now, &was)?;
        Ok(Outcome::with_raw(bytes, now))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls: the one plain scenario, subject only.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
