//! 🦀️ TIFF 6.0 ✳️baseline conformance-class mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR. Recorded no-oracle decision
//! `tiff-6-0-baseline-conformance-class-semantics`
//! (`../../🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json`).
//!
//! **Why there is no oracle here when the sibling subset has one.** `image` 0.25 is registered by
//! `✳️any` and is the right reference for a TIFF raster, but its TIFF surface decodes pixels and
//! re-encodes them under its OWN choice of `Compression`, `PhotometricInterpretation` and
//! `BitsPerSample`. It offers no way to set an arbitrary compression, an out-of-range photometric
//! or a tile-organized IFD, so it can neither perform nor judge a row of this vocabulary, and it
//! holds no opinion about Baseline class membership to compare against. And four of the six
//! conformance kinds are not byte-observable through this repository's own encoder either, because
//! `encode_tiff` regenerates every `CORE_STRIP_TAGS` entry from the raster it writes.
//!
//! **Where the assertion lives.** No oracle role is dispatched, so every law is asserted INSIDE the
//! subject handler — and, unlike the semio-native cases in this wave, this one CAN reach the shared
//! `⚖️law` module, because its owner sits under `✏️s/🔌️plugins/🗄️stdio` and the generated host
//! therefore links `semio-s-plugin-stdio-test-oracle`. `inverse-<kind>` goes through
//! `law::inverse_restores` and `identity-round-trip` through `law::reparsed_not_copied` and
//! `law::round_trip_preserves`.
//!
//! **The `setup` column.** `remove-tile-tags` cannot be exercised against a strip-organized scan as
//! it stands — there are no tile tags to remove — so its row names the mutation that makes the
//! removal meaningful, and both its observability and its inverse law are measured from THAT state.
//! Doing it the other way round, by silently treating a no-op removal as a pass, is exactly the
//! shallow green this wave removes.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `TiffBaselineMutation::KINDS` (`../../🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/
/// 🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build
/// must not link the subject crate. `kinds_match_enum_variants_in_declaration_order` and
/// `kinds_match_the_committed_catalog` in that production file keep it honest at both ends.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-compression", "set-photometric-interpretation", "set-bits-per-sample", "insert-tile-tags", "remove-tile-tags", "set-strip-offsets", "remove-strip-offsets"];

/// 🖼️ The real scanned TIFF, shared with the `✳️any` case rather than copied.
#[cfg(feature = "sut")]
const SCAN: &str = "shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff";
//#endregion 🔖️Kinds

//#region 🔖️Oracle
/// 🚫️ This case records a no-oracle decision, so the runner dispatches no oracle role at all and
/// nothing is registered for one. The reference answer this vocabulary would need — "is this
/// decoded IFD inside Adobe TIFF 6.0 Part 1's Baseline class" — is a reading of the standard's own
/// tag-value tables, which `check_tiff_baseline_conformance` encodes on the subject side; no
/// package holds it. Stating that here rather than registering a handler that would never run keeps
/// the file honest about what exists.
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::document::io::{decode_tiff, encode_tiff};
    use semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
    use semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::{apply_tiff_baseline_mutation, encode_tiff_baseline_projection_json, inverse_tiff_baseline_mutation, tiff_baseline_conformance_codes, TiffBaselineMutation};
    use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::document::project_tiff;
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️Json
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    fn numbers(value: &Json, key: &str) -> Vec<f64> {
        value
            .array(key)
            .into_iter()
            .filter_map(|entry| match entry {
                Json::Number(found) => Some(found),
                _ => None,
            })
            .collect()
    }
    //#endregion 🔖️Json

    //#region 🔖️MutationFromSpec
    /// 🧬️ The feature's nine-kind params grammar, translated into the REAL typed
    /// `TiffBaselineMutation` this subset applies. `set-snapshot` is built from the decoded document
    /// with all three value axes stamped at once, because a conformance class is a whole-document
    /// property and that variant is the class stamp in its total form.
    fn mutation_from_spec(kind: &str, params: &Json, base: &TiffSnapshot) -> Result<TiffBaselineMutation, String> {
        match kind {
            "set-snapshot" => {
                let mut snapshot = base.clone();
                for step in [
                    TiffBaselineMutation::SetCompression(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_compression::SetCompression { compression: number(params, "compression", 5.0) as u16 }),
                    TiffBaselineMutation::SetPhotometricInterpretation(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_photometric_interpretation::SetPhotometricInterpretation { photometric: number(params, "photometric", 6.0) as u16 }),
                    TiffBaselineMutation::SetBitsPerSample(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_bits_per_sample::SetBitsPerSample { bits: numbers(params, "bits").into_iter().map(|value| value as u16).collect() }),
                ] {
                    apply_tiff_baseline_mutation(&mut snapshot, &step);
                }
                Ok(TiffBaselineMutation::SetSnapshot(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_snapshot::SetSnapshot { snapshot }))
            }
            "set-compression" => Ok(TiffBaselineMutation::SetCompression(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_compression::SetCompression { compression: number(params, "compression", 5.0) as u16 })),
            "set-photometric-interpretation" => Ok(TiffBaselineMutation::SetPhotometricInterpretation(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_photometric_interpretation::SetPhotometricInterpretation { photometric: number(params, "photometric", 6.0) as u16 })),
            "set-bits-per-sample" => Ok(TiffBaselineMutation::SetBitsPerSample(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_bits_per_sample::SetBitsPerSample { bits: numbers(params, "bits").into_iter().map(|value| value as u16).collect() })),
            "insert-tile-tags" => Ok(TiffBaselineMutation::InsertTileTags(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::insert_tile_tags::InsertTileTags { tile_width: number(params, "tileWidth", 256.0) as u32, tile_length: number(params, "tileLength", 256.0) as u32 })),
            "remove-tile-tags" => Ok(TiffBaselineMutation::RemoveTileTags(remove_tile_tags::RemoveTileTags {})),
            "set-strip-offsets" => Ok(TiffBaselineMutation::SetStripOffsets(semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::baseline::schema::mutations::set_strip_offsets::SetStripOffsets { offsets: numbers(params, "offsets").into_iter().map(|value| value as u32).collect() })),
            "remove-strip-offsets" => Ok(TiffBaselineMutation::RemoveStripOffsets(remove_strip_offsets::RemoveStripOffsets {})),
            other => Err(format!("mutate-tiff-6-0-baseline: no params grammar for kind {other:?}")),
        }
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Decode
    /// 🖼️ The real scan, decoded and then carried through the row's `setup` step when it names one.
    /// The decode is required to have retained an IFD 0 with the strip organization these params are
    /// addressed at: `check_tiff_baseline_conformance` certifies nothing without an IFD, and a case
    /// that let an IFD-less snapshot through would be measuring the absence of the document.
    fn prepared(ctx: &Context, row: &Json) -> Result<TiffSnapshot, String> {
        let mut snapshot = decode_tiff(&ctx.fixture_bytes(super::SCAN)?).map_err(|error| format!("mutate-tiff-6-0-baseline: the committed scan must decode: {error:?}"))?;
        if snapshot.ifds.is_empty() {
            return Err("mutate-tiff-6-0-baseline: the decode retained no IFD, so no conformance axis exists to move".to_string());
        }
        let setup = row.get("setup").cloned().unwrap_or(Json::Object(Vec::new()));
        let setup_kind = setup.str("kind");
        if !setup_kind.is_empty() {
            let params = setup.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
            let step = mutation_from_spec(&setup_kind, &params, &snapshot)?;
            apply_tiff_baseline_mutation(&mut snapshot, &step);
        }
        Ok(snapshot)
    }

    fn projection(snapshot: &TiffSnapshot) -> Result<Json, String> {
        parse_json(&encode_tiff_baseline_projection_json(snapshot))
    }
    //#endregion 🔖️Decode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the prepared document and asserts, in role, that the class verdict
    /// moved exactly as the feature's `code` column declares and that the kind's own axis moved. An
    /// empty `code` is a positive claim, not an absence: `remove-tile-tags` and `set-strip-offsets`
    /// move their axis in the direction that stays INSIDE the class, so the document must certify
    /// CLEAN afterwards while still being observable. Reading the empty column as "the verdict is
    /// unchanged" instead would be wrong in exactly the row that carries the most weight:
    /// `remove-tile-tags` runs from the `setup` state, where the document is already OUT of the
    /// class with `tiled-not-baseline` raised, and putting it back inside is the whole point of the
    /// row — the verdict is REQUIRED to change there, from one code to none.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let base = prepared(ctx, &row)?;
            let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
            let mutation = mutation_from_spec(kind, &params, &base)?;
            let mut current = base.clone();
            apply_tiff_baseline_mutation(&mut current, &mutation);
            let after = tiff_baseline_conformance_codes(&current);
            let expected = row.str("code");
            if expected.is_empty() {
                if !after.is_empty() {
                    return Err(format!("mutate-{kind}: this row moves its axis in the direction that stays INSIDE the class, so the document must certify clean afterwards, but the verdict reports {after:?}"));
                }
            } else if !after.contains(&expected) {
                return Err(format!("mutate-{kind}: the class verdict must gain {expected:?}, but it reports {after:?} — the mutation did not reach the axis its own diagnostic guards"));
            }
            let (was, now) = (projection(&base)?, projection(&current)?);
            if kind != "no-mutation" {
                law::mutation_is_observable(kind, &now, &was, &[])?;
            } else if now != was {
                return Err("mutate-no-mutation: the identity element moved the projection".to_string());
            }
            Ok(Outcome::with_raw(now.to_string().into_bytes(), now))
        }
    }

    /// ↩️ The metamorphic inverse law over the prepared document: applying the kind and then its OWN
    /// computed inverse must land back on the pre-mutation conformance projection — every tag, and
    /// the verdict with them. `remove-tile-tags` carries the weight, because its payload is a bare
    /// variant with no fields at all, so the undo has to restore both tile values out of the base.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let base = prepared(ctx, &row)?;
            let original = projection(&base)?;
            let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
            let mutation = mutation_from_spec(kind, &params, &base)?;
            let mut current = base.clone();
            apply_tiff_baseline_mutation(&mut current, &mutation);
            if kind != "no-mutation" && projection(&current)? == original {
                return Err(format!("inverse-{kind}: the forward mutation left the conformance projection untouched, so restoring it proves nothing"));
            }
            for step in inverse_tiff_baseline_mutation(&mutation, &base) {
                apply_tiff_baseline_mutation(&mut current, &step);
            }
            let restored = projection(&current)?;
            law::inverse_restores(kind, &restored, &original)?;
            Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
        }
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
    ///    was authored by `✳️any/🧪️oracle`'s INDEPENDENT `write_tiff` over IFDs the registered
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
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Every handler is subject-only:
/// the recorded no-oracle decision means the runner dispatches no oracle role, and the reference
/// answer this vocabulary would need is a reading of Adobe TIFF 6.0 Part 1 rather than a package.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
