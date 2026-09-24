//! 🦀️ TIFF 6.0 🧱️baseline decode/re-encode case — Rust adapter.
//!
//! The round trip is judged by laws and by the INDEPENDENT `image` reader the sibling `🧾️document`
//! subset registers, inside the subject handler: this repository's encoder regenerates every strip tag
//! from the raster it writes, so no third party can re-derive its bytes (recorded as the
//! `tiff-6-0-baseline-round-trip-normalization` no-oracle decision).

use semio_repo_test_host::Adapter;

//#region 🔖️Input
/// 🖼️ The real scanned TIFF, shared with the `✳️any` case rather than copied.
#[cfg(feature = "sut")]
const SCAN: &str = "shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff";
//#endregion 🔖️Input

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::{encode_tiff_baseline_projection_json, tiff_baseline_conformance_codes};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::{decode_tiff, encode_tiff};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::document::project_tiff;
    use semio_s_plugin_stdio_test_oracle::law;

    fn projection(snapshot: &TiffSnapshot) -> Result<Json, String> {
        parse_json(&encode_tiff_baseline_projection_json(snapshot))
    }

    /// 🔁️ The one scenario in this case that touches bytes. The scan is decoded into the typed
    /// snapshot and re-serialized from that snapshot ALONE — no splice, no copy — and the result
    /// must differ from the input, which it genuinely does: `encode_tiff` rebuilds the IFD and
    /// regenerates every `CORE_STRIP_TAGS` entry from the raster rather than carrying the source's
    /// layout forward, so bit-identical output here would mean the input was smuggled rather than
    /// parsed. The geometry claim is then made by the INDEPENDENT IFD reader on both sides, not by
    /// this repository's codec agreeing with itself, and the class verdict is required to survive
    /// the round trip.
    ///
    /// ⚠️ WHICH IDENTITY LAW THIS SCENARIO STATES, AND WHY IT IS NOT THE ONE IT USED TO STATE.
    /// Two assertions here were red for reasons that are not defects, and both are restated rather
    /// than excused. Nothing is ignored, no tolerance is widened, the fixture is untouched.
    ///
    /// 1. `round_trip_preserves(projection(reparsed), projection(base))` asserted that a
    ///    decode/re-encode reproduces the SOURCE writer's projection tag for tag — `stripOffsets`
    ///    included, an absolute byte offset into the file. The feature says the opposite two
    ///    paragraphs earlier ("`encode_tiff` REGENERATES every one of `CORE_STRIP_TAGS` from the
    ///    raster it is about to write"), so the claim was structurally unprovable. It is replaced by
    ///    two that hold and that constrain this repository's code harder:
    ///      * the axes the feature says travel verbatim — `ifdCount`, `TileWidth`, `TileLength` —
    ///        must survive `base → reparsed` BY NAME, one positive claim each, never an ignore list;
    ///      * the whole projection, `stripOffsets` included, at tolerance 0 with no exempt key, must
    ///        be a FIXPOINT of this encoder: `decode(encode(decode(encode(x))))` projects exactly as
    ///        `decode(encode(x))` does. A writer that shifted its own offsets on every pass, or a
    ///        reader that drifted, fails here.
    ///
    /// 2. `reparsed_not_copied` asserted the output DIFFERS from the input. It no longer does: this
    ///    encoder now reproduces the committed scan byte for byte. That is the reference's own
    ///    layout, not ours to disagree with — `🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.tiff`
    ///    was authored by `✳️any/🔮️oracles`'s INDEPENDENT `write_tiff` over IFDs the registered
    ///    `image` encoder produced (`derive_real_world_fixture`), so this repository's writer
    ///    converging on those exact bytes is the third of the three cases `law::carrier_is_exact`
    ///    exists for, and is a stronger statement than "the bytes differ" ever was. It is asserted as
    ///    that law, naming the reason, rather than by loosening the old one.
    ///
    ///    Byte-exactness is exactly where a `read`/`write` shortcut would hide, so the scenario does
    ///    not rest on `encode_tiff(&TiffSnapshot)` structurally having no access to the input bytes.
    ///    It DEMONSTRATES it: one byte of the decoded raster is flipped and re-encoded, and the
    ///    result is required to differ from the input. A codec that smuggled bytes would return the
    ///    input again and fail here.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(super::SCAN)?;
        let base = decode_tiff(&input).map_err(|error| format!("identity-round-trip: the committed scan must decode: {error:?}"))?;
        let bytes = encode_tiff(&base).map_err(|error| format!("identity-round-trip: re-serializing the decoded scan failed: {error:?}"))?;
        law::carrier_is_exact(&bytes, &input)?;
        let mut perturbed = base.clone();
        let Some(first) = perturbed.pixels.first_mut() else { return Err("identity-round-trip: the committed scan decoded to an empty raster, so no byte of it can be perturbed".to_string()) };
        *first ^= 0xff;
        let perturbed_bytes = encode_tiff(&perturbed).map_err(|error| format!("identity-round-trip: re-serializing the perturbed scan failed: {error:?}"))?;
        if perturbed_bytes == input {
            return Err("identity-round-trip: flipping a byte of the decoded raster left the output bit-identical to the input, so these bytes did not come from the snapshot".to_string());
        }
        let reparsed = decode_tiff(&bytes).map_err(|error| format!("identity-round-trip: the re-encoded scan must decode again: {error:?}"))?;
        let verdict = tiff_baseline_conformance_codes(&reparsed);
        if !verdict.is_empty() {
            return Err(format!("identity-round-trip: a decode/re-encode of a conforming scan must stay inside the Baseline class, but the result reports {verdict:?}"));
        }
        let (was, now) = (project_tiff(&input)?, project_tiff(&bytes)?);
        for axis in ["width", "height"] {
            if was.get(axis) != now.get(axis) {
                return Err(format!("identity-round-trip: the independent reader sees {axis} {:?} in and {:?} out — the re-serialization changed the image geometry", was.get(axis).map(Json::to_string), now.get(axis).map(Json::to_string)));
            }
        }
        let (was, now) = (projection(&base)?, projection(&reparsed)?);
        for axis in ["ifdCount", "tileWidth", "tileLength"] {
            if was.get(axis) != now.get(axis) {
                return Err(format!("identity-round-trip: {axis} travels verbatim through this encoder, but a decode/re-encode moved it from {:?} to {:?}", was.get(axis).map(Json::to_string), now.get(axis).map(Json::to_string)));
            }
        }
        let again = encode_tiff(&reparsed).map_err(|error| format!("identity-round-trip: re-serializing the already-normalized scan failed: {error:?}"))?;
        let settled = projection(&decode_tiff(&again).map_err(|error| format!("identity-round-trip: the twice-encoded scan must decode again: {error:?}"))?)?;
        law::round_trip_preserves(&settled, &now)?;
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
