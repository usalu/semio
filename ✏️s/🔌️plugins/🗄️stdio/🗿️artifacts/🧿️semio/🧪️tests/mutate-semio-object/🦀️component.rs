//! 🦀️ Semio OBJECT exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-object-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️object/🧪️oracle/🔣️component.json`): `s.stdio.semio.object` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed, independently
//! handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/✳️object/
//! 🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_semio_object_mutation`, the entry point this ticket added, over the full 9-kind
//! `SemioObjectMutation` vocabulary. Both sides project the snapshot to structural JSON and
//! `ordered-json-v1` compares them. The oracle-only build must never link the subject crate (fleet
//! brief §5.3), so the fixtures' BEFORE snapshot and MUTATION payload are transcribed once, by
//! hand, as `SemioObjectSnapshot`/`SemioObjectMutation` Rust literals inside the `sut`-gated
//! `subject` module below — mechanically identical to the committed JSON, never independently
//! invented (compare against the JSON embedded via `include_str!` in `oracle_fixture`). The
//! generated test-host crate carries no `serde_json` dependency (only `semio-repo-test-host` and,
//! behind `sut`, this subset's own crate), so parsing committed JSON straight into typed structs is
//! not an option here; the framework's own dependency-free `protocol::Json`/`parse_json` carries the
//! oracle side instead. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation; the Rust SUBJECT phase is blocked this
//! wave by a concurrent os-kernel refactor (see the fleet brief), so it is written and gated but not
//! run.
//!
//! **Known subject gap — `store::os_io::ArtifactRef`.** `object` is the FIRST composite subset in
//! this wave (real owned CHILD slots, `📸️snapshot/🦀️component.rs`'s own doc comment), and 6 of its
//! 9 kinds (`create-brep`/`delete-brep`/`create-mesh`/`delete-mesh`/`create-properties`/
//! `delete-properties`) need a `store::os_io::ArtifactRef` value — either directly as the `create-*`
//! mutation payload's `target` field, or indirectly to populate a `delete-*` fixture's BEFORE
//! snapshot with a real child handle. `ArtifactRef`/`ArtifactChild<S>` live in
//! `semio_framework_os_kernel`, reached inside the plugin crate only through its own PRIVATE
//! `extern crate semio_framework_os_kernel as store;` (`📦️glue.rs:15`, no `pub`) — confirmed by
//! exhaustively grepping the whole stdio plugin tree for any `pub use`/`pub extern crate`
//! re-export (none) and any already-public, non-test helper that turns a URI string into an
//! `ArtifactRef` (none). The generated test-host `Cargo.toml` (`materializeRustHost`,
//! `🧰️framework/…/🧪️test/📜️script.ts`) links ONLY `semio-repo-test-host` and, behind `sut`,
//! `semio-s-plugin-stdio` itself with `default-features = false` — no `serde`, no
//! `semio-framework-os-kernel` — so there is no nameable path, generic-deserialize bridge, or
//! `Default` impl (neither `ArtifactRef` nor `ArtifactDialect` derives `Default`) that reaches the
//! type without editing `Cargo.toml`/`📦️glue.rs`, both out of scope for this ticket. This is the
//! same CLASS of gap this wave's own status notes already record for `store::ArtifactPack`/
//! `ArtifactDsl` (private trait aliases, unreachable from outside) — not a new kind of limitation,
//! just the first subset with a MUTATION PAYLOAD (not only an identity round-trip) that needs one.
//! `oracle` still covers all 9 kinds (it never constructs typed values, only compares JSON text);
//! `subject` registers real handlers for the 3 kinds that never touch a child slot
//! (`move-object`/`rotate-object`/`scale-object`) and, for the other 6, a handler that returns a
//! clear, self-documenting `Err` naming this exact blocker — a registered, honest failure rather
//! than an unregistered scenario (the runner treats a missing registration as "no {role}
//! registration for scenario X", strictly less informative). The projection encoder
//! (`snapshot_json` below) still encodes populated `brep`/`mesh`/`properties` slots correctly by
//! READING their already-`pub` fields directly (`child.target.artifact_id`, `child.target.dialect.
//! artifact_kind`, …) — reading an opaque type's public fields needs no nameable path to the type
//! itself, only CONSTRUCTING one does — so the encoder is not itself limited by this gap, only
//! `fixture_for` is.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioObjectMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["move-object", "rotate-object", "scale-object", "create-brep", "delete-brep", "create-mesh", "delete-mesh", "create-properties", "delete-properties"];
//#endregion 🔖️Kinds

//#region 🔖️OracleFixtures
/// 🧫️ The committed `(before, after)` snapshot JSON for one kind, read literally — this IS the
/// independently handcrafted specification vector the no-oracle decision rests on, never
/// recomputed.
fn oracle_fixture(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "move-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/🧪️tests/moves-the-object-to-a-new-translation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/🧪️tests/moves-the-object-to-a-new-translation/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "rotate-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/🧪️tests/rotates-the-object-a-half-turn-about-z/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/🧪️tests/rotates-the-object-a-half-turn-about-z/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "scale-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/🧪️tests/scales-the-object-non-uniformly/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/🧪️tests/scales-the-object-non-uniformly/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-brep" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/🧪️tests/attaches-a-brep-child-to-an-object-that-has-none/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/🧪️tests/attaches-a-brep-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-brep" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/🧪️tests/detaches-the-brep-child-and-leaves-the-mesh-child-alone/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/🧪️tests/detaches-the-brep-child-and-leaves-the-mesh-child-alone/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-mesh" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/attaches-a-mesh-child-to-an-object-that-has-none/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/attaches-a-mesh-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-mesh" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/🧪️tests/detaches-the-mesh-child-and-leaves-the-brep-child-alone/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/🧪️tests/detaches-the-mesh-child-and-leaves-the-brep-child-alone/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-properties" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-an-object-that-has-none/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-properties" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-the-mesh-child-alone/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-the-mesh-child-alone/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-object: no fixture registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️OracleFixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::mutations::{move_object, rotate_object, scale_object, SemioObjectMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
    use protocol::Mutation;

    //#region 🔖️HandcraftedFixtures
    /// 🧫️ The SAME specification vector `../🦀️component.rs::oracle_fixture` embeds as JSON,
    /// transcribed once by hand into real `SemioObjectSnapshot`/`SemioObjectMutation` values — the
    /// oracle-only build must never link this crate, so there is no way to share one physical
    /// source between the two roles; committed side by side under the same kind's `🧪️tests/`
    /// directory, so a drift between them is a one-file diff away from being caught by eye. Every
    /// one of the 3 kinds registered below leaves `brep`/`mesh`/`properties` at `None` — the
    /// committed BEFORE/AFTER fixtures for `move-object`/`rotate-object`/`scale-object` never
    /// populate those slots either (only the `transform` field changes), so `identity_snapshot()`
    /// needs no `store::os_io::ArtifactRef` (see this file's parent module doc comment for why the
    /// other 6 kinds cannot be transcribed at all).
    fn identity_snapshot() -> SemioObjectSnapshot {
        SemioObjectSnapshot {
            schema: "stdio.semio.object".into(),
            transform: SemioTransform { translation: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, rotation: SemioQuaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
            brep: None,
            mesh: None,
            properties: None,
        }
    }

    fn fixture_for(kind: &str) -> Result<(SemioObjectSnapshot, SemioObjectMutation), String> {
        match kind {
            "move-object" => Ok((identity_snapshot(), SemioObjectMutation::MoveObject(move_object::mutation::MoveObject { translation: SemioPoint3 { x: 2.0, y: -0.5, z: 4.0 } }))),
            "rotate-object" => Ok((identity_snapshot(), SemioObjectMutation::RotateObject(rotate_object::mutation::RotateObject { rotation: SemioQuaternion { x: 0.0, y: 0.0, z: 1.0, w: 0.0 } }))),
            "scale-object" => Ok((identity_snapshot(), SemioObjectMutation::ScaleObject(scale_object::mutation::ScaleObject { scale: SemioPoint3 { x: 2.0, y: 0.5, z: 4.0 } }))),
            "create-brep" | "delete-brep" | "create-mesh" | "delete-mesh" | "create-properties" | "delete-properties" => {
                Err(format!("mutate-semio-object subject: {kind} needs a `store::os_io::ArtifactRef` value (either as the create-* mutation's own `target` field, or to populate a delete-*'s BEFORE child slot), and that type is unreachable from this adapter — it lives in `semio_framework_os_kernel`, reached inside the plugin crate only via its own PRIVATE `extern crate semio_framework_os_kernel as store;` (📦️glue.rs:15), with no public re-export anywhere in the stdio plugin and no `serde`/`semio-framework-os-kernel` dependency on the generated test-host crate. See ../../../../🦀️component.rs's module doc comment for the full investigation."))
            }
            other => Err(format!("mutate-semio-object: no fixture registered for kind {other:?}")),
        }
    }
    //#endregion 🔖️HandcraftedFixtures

    //#region 🔖️Projection
    fn point3_json(p: &SemioPoint3) -> Json {
        Json::Object(vec![("x".to_string(), Json::Number(p.x)), ("y".to_string(), Json::Number(p.y)), ("z".to_string(), Json::Number(p.z))])
    }
    fn quaternion_json(q: &SemioQuaternion) -> Json {
        Json::Object(vec![("x".to_string(), Json::Number(q.x)), ("y".to_string(), Json::Number(q.y)), ("z".to_string(), Json::Number(q.z)), ("w".to_string(), Json::Number(q.w))])
    }
    fn transform_json(t: &SemioTransform) -> Json {
        Json::Object(vec![("translation".to_string(), point3_json(&t.translation)), ("rotation".to_string(), quaternion_json(&t.rotation)), ("scale".to_string(), point3_json(&t.scale))])
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (mut base, mutation) = fixture_for(kind)?;
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::mutations::apply_semio_object_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            let projection = snapshot_json(&base);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (base, mutation) = fixture_for(kind)?;
            let mut current = base.clone();
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::mutations::apply_semio_object_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            let undo = mutation.inverse(&base);
            for step in &undo {
                let step_outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::mutations::apply_semio_object_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            let projection = snapshot_json(&current);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }

    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field. Encodes populated
    /// `brep`/`mesh`/`properties` slots by reading their `pub` fields directly (never spelling
    /// `store::ArtifactChild<S>`/`store::os_io::ArtifactRef` — see the parent module doc comment) so
    /// the encoder itself stays correct even though none of the 3 registered kinds ever populate
    /// those slots.
    fn snapshot_json(snapshot: &SemioObjectSnapshot) -> Json {
        let mut fields = vec![("schema".to_string(), Json::String(snapshot.schema.clone())), ("transform".to_string(), transform_json(&snapshot.transform))];
        if let Some(brep) = &snapshot.brep {
            fields.push(("brep".to_string(), Json::Object(vec![("childId".to_string(), Json::String(brep.child_id.clone())), ("target".to_string(), Json::Object(vec![("artifactId".to_string(), Json::String(brep.target.artifact_id.clone())), ("dialect".to_string(), Json::Object(vec![("artifactKind".to_string(), Json::String(brep.target.dialect.artifact_kind.clone())), ("standard".to_string(), Json::String(brep.target.dialect.standard.clone())), ("subset".to_string(), Json::String(brep.target.dialect.subset.clone()))]))]))])));
        }
        if let Some(mesh) = &snapshot.mesh {
            fields.push(("mesh".to_string(), Json::Object(vec![("childId".to_string(), Json::String(mesh.child_id.clone())), ("target".to_string(), Json::Object(vec![("artifactId".to_string(), Json::String(mesh.target.artifact_id.clone())), ("dialect".to_string(), Json::Object(vec![("artifactKind".to_string(), Json::String(mesh.target.dialect.artifact_kind.clone())), ("standard".to_string(), Json::String(mesh.target.dialect.standard.clone())), ("subset".to_string(), Json::String(mesh.target.dialect.subset.clone()))]))]))])));
        }
        if let Some(properties) = &snapshot.properties {
            fields.push(("properties".to_string(), Json::Object(vec![("childId".to_string(), Json::String(properties.child_id.clone())), ("target".to_string(), Json::Object(vec![("artifactId".to_string(), Json::String(properties.target.artifact_id.clone())), ("dialect".to_string(), Json::Object(vec![("artifactKind".to_string(), Json::String(properties.target.dialect.artifact_kind.clone())), ("standard".to_string(), Json::String(properties.target.dialect.standard.clone())), ("subset".to_string(), Json::String(properties.target.dialect.subset.clone()))]))]))])));
        }
        Json::Object(fields)
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built
}
//#endregion 🔖️Registration
