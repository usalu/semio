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

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pptx::standards::v_ecma_376::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_pptx_mutation};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `PptxMutation` variant, mirrored from
/// `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own
/// `KINDS` — duplicated rather than imported because the ORACLE-only build of this adapter must
/// never link `semio-s-plugin-stdio` (see this file's own header); `kinds_matches_enum_variants_
/// and_manifest` on the production side and the framework's own catalog-completeness gate on this
/// side are what keep the two lists honest against each other.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🎞️semio-talk.pptx";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("semio-talk.pptx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pptx_mutation(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔮️ One handler shared by every `inverse-<kind>` scenario id.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_pptx_mutation(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law: the `zip`+`quick-xml` composition fully
/// parses the real presentation and re-serializes it from its own typed slide/shape list alone (the
/// same "no-mutation" routing `oracle_apply_mutation` already gives every other kind — this oracle's
/// own header explains why every call, including `no-mutation`, is a genuine re-serialization),
/// independent evidence that a full parse/re-serialize is possible before the SUBJECT is held to the
/// same standard below.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    let projection = project_pptx_mutation(&bytes)?;
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
    /// without duplicating `KINDS` a third time.
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
