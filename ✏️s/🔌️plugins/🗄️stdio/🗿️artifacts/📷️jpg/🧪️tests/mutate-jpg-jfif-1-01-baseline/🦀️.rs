//! 🦀️ JFIF 1.01 ✳️baseline conformance-class mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR. Recorded no-oracle decision
//! `jpg-jfif-1-01-baseline-conformance-class-semantics`
//! (`../../🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🔣️oracle.json`).
//!
//! **Why there is no oracle here when the sibling subset has one.** `image` 0.25 is registered by
//! `✳️any` and is the right reference for a JPEG raster, but its public surface is pixels,
//! dimensions and a colour type. It cannot read a SOF marker, a sample precision, a DAC flag or a
//! DHT table, and it offers no way to write any of them — so it can neither perform nor judge a
//! single row of this vocabulary. And even a reference that could would be measuring nothing at the
//! byte level, because `encode_jpg` normalizes every one of these axes away on re-serialization.
//! The evidence is therefore T.81's own tables read against this subset's `check_baseline_conformance`,
//! plus the inverse law over the decoded snapshot.
//!
//! **Where the assertion lives.** No oracle role is dispatched, so every law is asserted INSIDE the
//! subject handler — and, unlike the semio-native cases in this wave, this one CAN reach the shared
//! `⚖️law` module, because its owner sits under `✏️s/🔌️plugins/🗄️stdio` and the generated host
//! therefore links `semio-s-plugin-stdio-test-oracle`. `inverse-<kind>` goes through
//! `law::inverse_restores` and `identity-round-trip` through `law::reparsed_not_copied` and
//! `law::round_trip_preserves`, so the wording of the laws is shared with the rest of the fleet
//! rather than restated.
//!
//! **The one scenario that does touch bytes.** `identity-round-trip` decodes the real scan,
//! re-serializes from the snapshot alone, and reads BOTH sides through `project_jpg_mutation` — the
//! `✳️any` oracle module's independent `image`-backed reader — so the geometry claim is made by a
//! third party rather than by this repository's own codec agreeing with itself.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `JpgBaselineMutation::KINDS` (`../../🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/
/// 🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not imported, because the oracle-only build
/// must not link the subject crate. `kinds_match_enum_variants_in_declaration_order` and
/// `kinds_match_the_committed_catalog` in that production file keep it honest at both ends.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-sof-marker", "set-sample-precision", "set-arithmetic", "insert-huffman-table", "remove-huffman-table", "insert-frame-component", "remove-frame-component", "set-component-sampling"];

/// 🖼️ The real 2275x2560 architectural scan, shared with the `✳️any` case rather than copied: two
/// DQT, an SOF0 with three components, four DHT and SOS.
#[cfg(feature = "sut")]
const SCAN: &str = "shared://🖼️abbau-aufbau-masterarbeit-grundriss.jpg";
//#endregion 🔖️Kinds

//#region 🔖️Oracle
/// 🚫️ This case records a no-oracle decision, so the runner dispatches no oracle role at all and
/// nothing is registered for one. The reference answers this vocabulary would need — "is this
/// decoded document inside ITU-T T.81 Annex F's baseline class" — are the standard's own tables,
/// read by `check_baseline_conformance` on the subject side; there is no package that holds them.
/// Stating that here rather than registering a handler that would never run keeps the file honest
/// about what exists.
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::jpg::io::{decode_jpg, encode_jpg};
    use semio_s_plugin_stdio::artifacts::jpg::schema::snapshot::{JpgFrameComponent, JpgHuffmanClass, JpgHuffmanTable};
    use semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::{apply_jpg_baseline_mutation, encode_jpg_baseline_projection_json, inverse_jpg_baseline_mutation, jpg_baseline_conformance_codes, JpgBaselineMutation};
    use semio_s_plugin_stdio::artifacts::jpg::schema::diff::JpgHuffmanTableKey;
    use semio_s_plugin_stdio::artifacts::jpg::JpgSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::jpg::standards::v_jfif_1_01::subsets::document::project_jpg_mutation;
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️Json
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    fn class_of(params: &Json) -> JpgHuffmanClass {
        match params.str("class").as_str() {
            "ac" => JpgHuffmanClass::Ac,
            _ => JpgHuffmanClass::Dc,
        }
    }

    /// 🧾️ A Huffman table whose bit-length counts and values are irrelevant to every axis this
    /// vocabulary reads — the class only counts tables per class, it never inspects one. Spelled out
    /// rather than defaulted so a reader can see that the payload is deliberately inert.
    fn inert_table(class: JpgHuffmanClass, id: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [0u8; 16], values: Vec::new() }
    }
    //#endregion 🔖️Json

    //#region 🔖️MutationFromSpec
    /// 🧬️ The feature's ten-kind params grammar, translated into the REAL typed
    /// `JpgBaselineMutation` this subset applies. `set-snapshot` is built from the decoded document
    /// with the three hard axes stamped at once, because a conformance class is a whole-document
    /// property and that variant is the class stamp in its total form.
    fn mutation_from_spec(kind: &str, params: &Json, base: &JpgSnapshot) -> Result<JpgBaselineMutation, String> {
        match kind {
            "set-snapshot" => {
                let mut snapshot = base.clone();
                snapshot.sof_marker = number(params, "sofMarker", 194.0) as u8;
                snapshot.arithmetic = matches!(params.get("arithmetic"), Some(Json::Bool(true)));
                if let Some(frame) = snapshot.frame.as_mut() {
                    frame.precision = number(params, "precision", 12.0) as u8;
                }
                Ok(JpgBaselineMutation::SetSnapshot(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_snapshot::SetSnapshot { snapshot }))
            }
            "set-sof-marker" => Ok(JpgBaselineMutation::SetSofMarker(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_sof_marker::SetSofMarker { marker: number(params, "marker", 194.0) as u8 })),
            "set-sample-precision" => Ok(JpgBaselineMutation::SetSamplePrecision(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_sample_precision::SetSamplePrecision { precision: number(params, "precision", 12.0) as u8 })),
            "set-arithmetic" => Ok(JpgBaselineMutation::SetArithmetic(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_arithmetic::SetArithmetic { arithmetic: matches!(params.get("arithmetic"), Some(Json::Bool(true))) })),
            "insert-huffman-table" => Ok(JpgBaselineMutation::InsertHuffmanTable(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::insert_huffman_table::InsertHuffmanTable { index: number(params, "index", 0.0) as usize, table: inert_table(class_of(params), number(params, "id", 2.0) as u8) })),
            "remove-huffman-table" => Ok(JpgBaselineMutation::RemoveHuffmanTable(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::remove_huffman_table::RemoveHuffmanTable { key: JpgHuffmanTableKey { class: class_of(params), id: number(params, "id", 0.0) as u8 } })),
            "insert-frame-component" => Ok(JpgBaselineMutation::InsertFrameComponent(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::insert_frame_component::InsertFrameComponent {
                index: number(params, "index", 0.0) as usize,
                component: JpgFrameComponent { id: number(params, "id", 4.0) as u8, h_sampling: number(params, "hSampling", 1.0) as u8, v_sampling: number(params, "vSampling", 1.0) as u8, quant_table_id: 0 },
            })),
            "remove-frame-component" => Ok(JpgBaselineMutation::RemoveFrameComponent(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::remove_frame_component::RemoveFrameComponent { id: number(params, "id", 3.0) as u8 })),
            "set-component-sampling" => Ok(JpgBaselineMutation::SetComponentSampling(semio_s_plugin_stdio::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_component_sampling::SetComponentSampling { id: number(params, "id", 1.0) as u8, h_sampling: number(params, "hSampling", 5.0) as u8, v_sampling: number(params, "vSampling", 1.0) as u8 })),
            other => Err(format!("mutate-jpg-jfif-1-01-baseline: no params grammar for kind {other:?}")),
        }
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Decode
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
    //#endregion 🔖️Decode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the real decoded scan and asserts, in role, that the class verdict
    /// moved exactly as the feature's `code` column declares and that the kind's own axis moved. An
    /// empty `code` is a positive claim, not an absence: three kinds move their axis in the
    /// direction that stays INSIDE the class, and those must raise nothing while still being
    /// observable.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let base = decoded(ctx)?;
            let before = jpg_baseline_conformance_codes(&base);
            if !before.is_empty() {
                return Err(format!("mutate-{kind}: the committed scan must start INSIDE the baseline class for a departure from it to mean anything, but it already reports {before:?}"));
            }
            let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
            let mutation = mutation_from_spec(kind, &params, &base)?;
            let mut current = base.clone();
            apply_jpg_baseline_mutation(&mut current, &mutation);
            let after = jpg_baseline_conformance_codes(&current);
            let expected = row.str("code");
            if expected.is_empty() {
                if !after.is_empty() {
                    return Err(format!("mutate-{kind}: this row moves its axis in the direction that stays inside the class, so the verdict must stay clean, but it reports {after:?}"));
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

    /// ↩️ The metamorphic inverse law over the real scan: applying the kind and then its OWN
    /// computed inverse must land back on the original conformance projection — every axis, and the
    /// verdict with them. The two counting kinds carry the weight, because their inverse has to
    /// restore a table or a component at the INDEX it was removed from, which is why both variants
    /// carry an index at all.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let base = decoded(ctx)?;
            let original = projection(&base)?;
            let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
            let mutation = mutation_from_spec(kind, &params, &base)?;
            let mut current = base.clone();
            apply_jpg_baseline_mutation(&mut current, &mutation);
            if kind != "no-mutation" && projection(&current)? == original {
                return Err(format!("inverse-{kind}: the forward mutation left the conformance projection untouched, so restoring it proves nothing"));
            }
            for step in inverse_jpg_baseline_mutation(&mutation, &base) {
                apply_jpg_baseline_mutation(&mut current, &step);
            }
            let restored = projection(&current)?;
            law::inverse_restores(kind, &restored, &original)?;
            Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
        }
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
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Every handler is subject-only:
/// the recorded no-oracle decision means the runner dispatches no oracle role, and the reference
/// answer this vocabulary would need is a reading of ITU-T T.81 rather than a package.
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
