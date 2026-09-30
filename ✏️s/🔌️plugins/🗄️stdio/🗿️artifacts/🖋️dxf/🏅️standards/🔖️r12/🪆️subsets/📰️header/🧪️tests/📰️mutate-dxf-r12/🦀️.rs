//! 🦀️ DXF R12 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the derived, committed R12 `🚏️bus-shelter` drawing into the case work
//! directory first; the committed asset is never written to. `oracle` drives the registered `dxf`
//! 0.6 reference implementation (`../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs`'s
//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's
//! own `parse_dxf_document`/`print_dxf_document`/`apply_dxf_mutation` over the full 19-kind
//! `DxfMutation` vocabulary. Each side hands the drawing it produced to the `semantic-dxf-r12-v1`
//! profile's `dxf-r12-reader-compare-v1` pipeline — the oracle's as `expected-dxf`, the subject's as
//! `actual-dxf` — whose `dxf` 0.6 probes read both files independently. The subject half is gated
//! behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation -- §5.3's own role separation, NOT a workaround for anything: the Rust subject
//! phase runs, and wave 14 ran the full differential comparison against the oracle.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::dxf::standards::v_r12::subsets::header::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_dxf_r12};


//#region 🔖️Input
const INPUT: &str = "asset://🚏️bus-shelter/🖊️.dxf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bus-shelter-r12.dxf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 📦️ The drawing one side produced, written as the `role` artifact (`expected-dxf` for the oracle, `actual-dxf` for the
/// subject) the `dxf-r12-reader-compare-v1` pipeline hands to its probes.
fn produced(ctx: &Context, role: &str, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {
    let path = ctx.artifact(role, &format!("{role}.dxf"))?;
    std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
    Ok(Outcome::with_raw(bytes, projection).artifact(role, &path, "image/vnd.dxf"))
}
//#endregion 🔖️Input

//#region 🔖️Laws
/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law
/// violation must name the field that broke it rather than dump two whole documents at the reader.
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    match (expected, actual) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in left {
                match right.iter().find(|(name, _)| name == key) {
                    Some((_, other)) => {
                        if let Some(found) = first_divergence(&format!("{path}.{key}"), value, other) {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{path}.{key} is absent from the result")),
                }
            }
            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!("{path}.{key} appeared in the result out of nowhere"))
        }
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{path} holds {} member(s), expected {}", right.len(), left.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!("{path}[{index}]"), value, other))
        }
        _ if expected == actual => None,
        _ => Some(format!("{path}: expected {} but read {}", expected.to_string(), actual.to_string())),
    }
}

/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise
/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and
/// its scenario passes whenever the reference library merely declined to error.
fn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {
    match first_divergence("projection", expected, actual) {
        Some(divergence) => Err(format!("{law}: {divergence}")),
        None => Ok(()),
    }
}
//#endregion 🔖️Laws

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in
/// role, before any parity comparison exists: every kind must MOVE the semantic projection. A row
/// whose parameters make the mutation a no-op is not a test -- it passes whenever the reference
/// library declined to error, which is exactly the failure this platform exists to prevent. The
/// baseline runs one `dxf` round trip so the comparison isolates the mutation rather than the
/// writer's own normal form.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_dxf_r12(&bytes)?;
    if projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the R12 drawing unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    produced(ctx, "expected-dxf", bytes, projection)
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: `dxf` applies the forward mutation and
/// then its own base-relative inverse (`oracle_apply_mutation_inverse`, one load/save cycle), and the
/// restored drawing MUST project exactly as the untouched drawing does. The baseline is taken
/// through one `dxf` round trip so the two sides carry the same serializer normalisation and the
/// comparison isolates the mutation pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_dxf_r12(&bytes)?;
    assert_same_projection(&format!("inverse law violated for {:?} -- undoing it did not restore the drawing", spec.str("kind")), &baseline, &projection)?;
    produced(ctx, "expected-dxf", bytes, projection)
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: `dxf` fully parses the real
/// document and re-serializes it from its own typed `Drawing` alone, so the re-encoded bytes MUST
/// carry the same semantic projection as the input AND MUST NOT be bit-identical to it. DXF R12 is
/// not a byte-preserving carrier -- `dxf` regenerates the whole group-code stream from its model --
/// so the byte tripwire is real evidence that the document was parsed rather than copied.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_dxf_r12(&input)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed".to_string());
    }
    let projection = project_dxf_r12(&bytes)?;
    assert_same_projection("identity round trip is not semantics-preserving", &before, &projection)?;
    produced(ctx, "expected-dxf", bytes, projection)
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, produced};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::mutations::{apply_dxf_mutation, decode_dxf_mutation_payload, inverse_dxf_mutation, DxfMutation};
    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::snapshot::{parse_dxf_document, print_dxf_document};
    use semio_s_artifact_stdio_dxf::DxfSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::dxf::standards::v_r12::subsets::header::project_dxf_r12;

    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.
    fn mutation_of(spec: &Json) -> Result<DxfMutation, String> {
        decode_dxf_mutation_payload(&spec.str("kind"), &spec.get("params").map_or_else(|| "null".to_string(), Json::to_string))
    }

    fn decode(ctx: &Context) -> Result<DxfSnapshot, String> {
        let input = mutable_input(ctx)?;
        parse_dxf_document(std::str::from_utf8(&input).map_err(|error| error.to_string())?)
    }

    fn outcome(ctx: &Context, snapshot: &DxfSnapshot) -> Result<Outcome, String> {
        let output = print_dxf_document(snapshot).into_bytes();
        let projection = project_dxf_r12(&output)?;
        produced(ctx, "actual-dxf", output, projection)
    }

    /// 🚫️ A REFUSED mutation is a failure, never a silent no-op. `apply_dxf_mutation` leaves the snapshot untouched and
    /// reports the refusal as the outcome's only messages — every `DxfMutation::diff` arm builds its outcome without one —
    /// so a non-empty message list IS a refusal.
    fn applied(snapshot: &mut DxfSnapshot, mutation: &DxfMutation, kind: &str) -> Result<(), String> {
        match apply_dxf_mutation(snapshot, mutation).messages().first() {
            Some(refusal) => Err(format!("{kind}: the mutation was REFUSED and the document left untouched — {refusal:?}")),
            None => Ok(()),
        }
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        let spec = ctx.doc_json()?;
        applied(&mut snapshot, &mutation_of(&spec)?, &spec.str("kind"))?;
        outcome(ctx, &snapshot)
    }

    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let forward = mutation_of(&spec)?;
        let backward = inverse_dxf_mutation(&snapshot, &forward);
        applied(&mut snapshot, &forward, &kind)?;
        for mutation in &backward {
            applied(&mut snapshot, mutation, &format!("the inverse of {kind}"))?;
        }
        outcome(ctx, &snapshot)
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `parse_dxf_document`/
    /// `print_dxf_document` are this subset's ONLY channel from input to output. `print_dxf_document`
    /// regenerates a canonical NORMAL FORM (documented in `📸️snapshot/🦀️.rs`'s own module
    /// doc), never raw byte preservation, so the tripwire is real rather than incidental.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let snapshot = decode(ctx)?;
        if print_dxf_document(&snapshot).into_bytes() == mutable_input(ctx)? {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        outcome(ctx, &snapshot)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
