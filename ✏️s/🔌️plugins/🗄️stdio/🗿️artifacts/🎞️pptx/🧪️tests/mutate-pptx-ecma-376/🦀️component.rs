//! 🦀️ PPTX ECMA-376/✳️any exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.
//!
//! Every scenario copies the real, committed `🎞️semio-talk.pptx` fixture (a real 7-slide subset
//! derived once from a real 62-slide, 16 MB 2020 conference deck — see the feature file's own
//! header for the full provenance) into the case work directory first; the committed fixture is
//! never written to. `oracle` drives the registered `zip`+`quick-xml` composition
//! (`../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_pptx`/`encode_pptx`/`apply_pptx_mutation` over the full 9-kind `PptxMutation`
//! vocabulary. Both results are read back by the SAME independent `project_pptx_mutation` (the
//! `zip`+`quick-xml` composition) before the `semantic-pptx-mutate-v1` profile compares them. The
//! subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! links `semio-s-plugin-stdio`, whose subject phase is peer-blocked right now (concurrent
//! os-kernel refactor).
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law`
//! module, so a scenario cannot pass merely because `zip`+`quick-xml` declined to error:
//! `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>` must land back on the
//! untouched deck's projection, and `identity-round-trip` must both preserve the projection and
//! rebuild an archive that differs from the input. There is no carve-out of any kind: the profile
//! declares no writer freedom, no kind is exempt from observability, and no axis — slide order
//! included — is dropped from the inverse law.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pptx::standards::v_ecma_376::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_pptx_mutation, KINDS};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const INPUT: &str = "shared://🎞️semio-talk.pptx";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("semio-talk.pptx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: the reference composition
/// applies the kind to the real seven-slide deck and the result has to differ from the untouched
/// presentation. Returning the projection uncompared is what made these nine scenarios pass whenever
/// `zip`+`quick-xml` merely did not error. NOTHING is exempt — every declared kind is defined on the
/// ordered slide list or on a shape inside it, which is exactly what `semantic-pptx-mutate-v1`
/// reports, and that profile declares no writer freedom at all (`ignoreKeys: []`).
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pptx_mutation(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_pptx_mutation(&input)?, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The INVERSE law, asserted in role without needing the subject: `apply(inverse(m), apply(m,
/// base))` must land back on the ORIGINAL presentation's own projection, read through the same
/// independent reader. No axis is dropped and no tolerance is allowed — slide ORDER included, which
/// is the whole point of a vocabulary that declares `move-slide`.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_pptx_mutation(&bytes)?;
    inverse_restores(&kind, &projection, &project_pptx_mutation(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity law, both halves asserted: the `zip`+`quick-xml` composition
/// unzips the real deck, parses every slide, regenerates every slide-related OPC part from its own
/// typed slide/shape list and rezips — the same rebuild every kind goes through. The result must
/// differ from the input, because two independent writers agree on neither compression nor part
/// layout, and the projection must survive intact.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_pptx_mutation(&bytes)?;
    round_trip_preserves(&projection, &project_pptx_mutation(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::{build_minimal_pptx, encode_pptx};
    use semio_s_plugin_stdio::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_pptx;
    use semio_s_plugin_stdio::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::mutations::apply_pptx_mutation;
    use semio_s_plugin_stdio::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::snapshot::{PptxParagraph, PptxPresentation, PptxShape, PptxSlide, PptxTransform};
    use semio_s_plugin_stdio::artifacts::pptx::{PptxMutation, PptxSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::pptx::standards::v_ecma_376::subsets::any::project_pptx_mutation;

    //#region 🔖️SpecCodec
    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }
    fn usize_field(value: &Json, key: &str) -> usize {
        number_field(value, key).max(0.0) as usize
    }
    fn i64_field(value: &Json, key: &str) -> i64 {
        number_field(value, key) as i64
    }

    /// 🔎️ The same `{"x":...,"y":...,"cx":...,"cy":...}` shape the oracle side's `Transform` speaks,
    /// decoded into the PRODUCTION `PptxTransform` here instead.
    fn json_to_transform(value: &Json) -> PptxTransform {
        match value.get("position") {
            Some(position) => PptxTransform { x: i64_field(position, "x"), y: i64_field(position, "y"), cx: i64_field(position, "cx"), cy: i64_field(position, "cy") },
            None => PptxTransform::default(),
        }
    }

    /// 🔎️ The same owned shape-spec JSON grammar the oracle side speaks
    /// (`{"kind":"textBox"|"placeholder"|"picture", ...}`), decoded into the PRODUCTION `PptxShape`
    /// here instead of the oracle's own independent `PShape` type.
    fn json_to_shape(value: &Json) -> Result<PptxShape, String> {
        let position = json_to_transform(value);
        match value.str("kind").as_str() {
            "textBox" => Ok(PptxShape::TextBox { text_frame: vec![PptxParagraph::text(value.str("text"))], position }),
            "placeholder" => Ok(PptxShape::Placeholder { kind: value.str("phKind"), text_frame: vec![PptxParagraph::text(value.str("text"))], position }),
            "picture" => Ok(PptxShape::Picture { blip_rel_id: value.str("blipRelId"), position }),
            other => Err(format!("unknown shape kind {other:?}")),
        }
    }

    fn json_to_slide(value: &Json) -> Result<PptxSlide, String> {
        Ok(PptxSlide { shapes: value.array("shapes").iter().map(json_to_shape).collect::<Result<Vec<_>, _>>()? })
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `PptxMutation` this subset
    /// declares for it. `set-snapshot` builds a brand-new, fully valid `PptxSnapshot` through
    /// `build_minimal_pptx` (real OPC/XML scaffolding synthesis, the same helper this subset's own
    /// mutation-law tests use), replacing the base snapshot outright.
    fn mutation_from_spec(spec: &Json) -> Result<PptxMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => PptxMutation::NoMutation,
            "set-snapshot" => {
                let slides = params.array("slides").iter().map(json_to_slide).collect::<Result<Vec<_>, _>>()?;
                PptxMutation::SetSnapshot { snapshot: build_minimal_pptx(PptxPresentation { slides }) }
            }
            "insert-slide" => PptxMutation::InsertSlide { index: usize_field(&params, "index"), slide: json_to_slide(params.get("slide").ok_or("insert-slide: missing slide")?)? },
            "remove-slide" => PptxMutation::RemoveSlide { index: usize_field(&params, "index") },
            "move-slide" => PptxMutation::MoveSlide { from: usize_field(&params, "from"), to: usize_field(&params, "to") },
            "insert-shape" => PptxMutation::InsertShape { slide_index: usize_field(&params, "slideIndex"), shape_index: usize_field(&params, "shapeIndex"), shape: json_to_shape(params.get("shape").ok_or("insert-shape: missing shape")?)? },
            "remove-shape" => PptxMutation::RemoveShape { slide_index: usize_field(&params, "slideIndex"), shape_index: usize_field(&params, "shapeIndex") },
            "set-shape-text" => PptxMutation::SetShapeText { slide_index: usize_field(&params, "slideIndex"), shape_index: usize_field(&params, "shapeIndex"), text_frame: vec![PptxParagraph::text(params.str("text"))] },
            "set-shape-position" => PptxMutation::SetShapePosition { slide_index: usize_field(&params, "slideIndex"), shape_index: usize_field(&params, "shapeIndex"), position: json_to_transform(&params) },
            other => return Err(format!("mutation kind {other:?} has no subject implementation")),
        })
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// ↩️ `PptxMutation::inverse` in closed form -- every variant's own `Mutation::inverse` arm,
    /// transplanted rather than called through the trait, same precedent `mutate-pdf-1-7`'s own
    /// `inverse_of` gives: written in closed form so this adapter needs no extra crate dependency
    /// beyond `semio-s-plugin-stdio` itself.
    fn inverse_of(mutation: &PptxMutation, base: &PptxSnapshot) -> PptxMutation {
        match mutation {
            PptxMutation::NoMutation => PptxMutation::NoMutation,
            PptxMutation::SetSnapshot { .. } => PptxMutation::SetSnapshot { snapshot: base.clone() },
            PptxMutation::InsertSlide { index, .. } => PptxMutation::RemoveSlide { index: *index },
            PptxMutation::RemoveSlide { index } => match base.presentation.slides.get(*index) {
                Some(slide) => PptxMutation::InsertSlide { index: *index, slide: slide.clone() },
                None => PptxMutation::NoMutation,
            },
            PptxMutation::MoveSlide { from, to } => {
                let len = base.presentation.slides.len();
                let final_pos = (*to).min(len.saturating_sub(1));
                PptxMutation::MoveSlide { from: final_pos, to: *from }
            }
            PptxMutation::InsertShape { slide_index, shape_index, .. } => PptxMutation::RemoveShape { slide_index: *slide_index, shape_index: *shape_index },
            PptxMutation::RemoveShape { slide_index, shape_index } => match base.presentation.slides.get(*slide_index).and_then(|slide| slide.shapes.get(*shape_index)) {
                Some(shape) => PptxMutation::InsertShape { slide_index: *slide_index, shape_index: *shape_index, shape: shape.clone() },
                None => PptxMutation::NoMutation,
            },
            PptxMutation::SetShapeText { slide_index, shape_index, .. } => {
                let old = base.presentation.slides.get(*slide_index).and_then(|slide| slide.shapes.get(*shape_index)).and_then(|shape| match shape {
                    PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => Some(text_frame.clone()),
                    _ => None,
                });
                match old {
                    Some(text_frame) => PptxMutation::SetShapeText { slide_index: *slide_index, shape_index: *shape_index, text_frame },
                    None => PptxMutation::NoMutation,
                }
            }
            PptxMutation::SetShapePosition { slide_index, shape_index, .. } => {
                let old = base.presentation.slides.get(*slide_index).and_then(|slide| slide.shapes.get(*shape_index)).and_then(|shape| match shape {
                    PptxShape::TextBox { position, .. } | PptxShape::Picture { position, .. } | PptxShape::Placeholder { position, .. } => Some(*position),
                    PptxShape::Other { .. } => None,
                });
                match old {
                    Some(position) => PptxMutation::SetShapePosition { slide_index: *slide_index, shape_index: *shape_index, position },
                    None => PptxMutation::NoMutation,
                }
            }
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode_pptx(&input).map_err(|error| format!("decode_pptx failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_pptx_mutation(&mut snapshot, &mutation);
        let bytes = encode_pptx(&snapshot).map_err(|error| format!("encode_pptx failed: {error}"))?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".into());
        }
        let projection = project_pptx_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = decode_pptx(&input).map_err(|error| format!("decode_pptx failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_pptx_mutation(&mut snapshot, &mutation);
        apply_pptx_mutation(&mut snapshot, &undo);
        let bytes = encode_pptx(&snapshot).map_err(|error| format!("encode_pptx failed: {error}"))?;
        let projection = project_pptx_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real presentation into its
    /// typed snapshot and re-serialize from the model alone -- `decode_pptx`/`encode_pptx` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_pptx(&input).map_err(|error| format!("decode_pptx failed: {error}"))?;
        let output = encode_pptx(&snapshot).map_err(|error| format!("encode_pptx failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_pptx_mutation(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 9-kind sweep for the subject role
    /// from the one list the subset's own oracle module declares.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 9 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
