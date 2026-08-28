//! 🦀️ Remodel-scene exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `remodel-mutation-semantics` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`):
//! `s.remodel.remodel` is a semio-NATIVE reconstruction JOB document, not a point cloud or a mesh
//! file, so no photogrammetry library is a reference for it. `oracle` reads the committed per-kind
//! specification vectors literally and `subject` drives all 35 `RemodelMutation` variants.
//!
//! 34 of the 35 vectors are the leaves' own committed `(before, mutation, after)` triples.
//! `commit-reconstruction` is the exception and the reason is structural: its diff reads
//! process-global staging state that a triple cannot carry, so its vector lives in this case's own
//! `🧫️fixtures/` — assembled ONCE out of two committed sibling payloads, with the provenance written
//! into `component.feature` — and exercises the kind's documented refusal path instead.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law this case claims is asserted INSIDE the subject handler, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module: `law::divergence` names the first divergence by
//! JSON path, `law::mutation_is_observable` refuses a kind that moved nothing it is compared
//! through, `law::inverse_restores` is the inverse law itself, and `law::round_trip_preserves` plus
//! `law::carrier_is_exact` are the identity law's two halves. A handler that merely returned `Ok`
//! would report a pass having checked nothing at all.
//!
//! **How the fixture reaches typed values.** The generated host links only `semio-repo-test-host`,
//! the law module and — behind `sut` — this plugin's crate, whose `protocol`/`store`/`serde_json`
//! extern-crate aliases are private (`📦️glue.rs`). The oracle role therefore reads the committed
//! bytes with `include_str!` and the platform's own JSON reader, and the subject role hands the SAME
//! bytes to the production bridges `apply_remodel_mutation_json`, `undo_remodel_mutation_json` and
//! `round_trip_remodel_dsl` that this subset's `🧬️schema/🧬️mutations/🦀️component.rs` exports for it.
//! The subject half is gated behind the generated host's `sut` feature so an oracle-only run never
//! compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in
/// `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog and
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "create-stream",
    "delete-stream",
    "change-stream-sync",
    "add-stream-frame",
    "remove-stream-frame",
    "replace-stream-source",
    "create-asset",
    "delete-asset",
    "create-camera-calibration",
    "update-camera-calibration",
    "delete-camera-calibration",
    "create-rig-extrinsic",
    "delete-rig-extrinsic",
    "update-rig-extrinsic",
    "create-gcp",
    "delete-gcp",
    "add-gcp-observation",
    "remove-gcp-observation",
    "update-ingest-params",
    "update-feature-params",
    "update-match-params",
    "update-sfm-params",
    "update-dense-params",
    "update-mesh-params",
    "update-motion-params",
    "update-geo-params",
    "replace-job",
    "replace-sparse",
    "replace-dense",
    "replace-mesh-result",
    "replace-trajectory",
    "replace-tracks",
    "replace-geo-products",
    "replace-qc",
    "commit-reconstruction",
];

/// 👁️ Kinds whose committed specification vector declares NO movement — a refusal or an accepted
/// no-op — so the observability law must not be claimed for them. Each one is named in
/// `component.feature`'s description with the reason, and each is still asserted, through
/// [`DECLARED_CODE`], to raise exactly the diagnostic its leaf declares.
const UNOBSERVABLE: &[&str] = &["commit-reconstruction"];

/// 🚨️ The diagnostic code a declared no-op or refusal must raise, from the leaf's own committed
/// `🎯️outcome/🔣️component.json`. A vector that stopped raising it would otherwise be
/// indistinguishable from a mutation that quietly did nothing. Read only by the subject role —
/// the oracle role answers with the committed after-document, which already IS the declared outcome.
#[cfg(feature = "sut")]
const DECLARED_CODE: &[(&str, &str)] = &[("commit-reconstruction", "mutation.invalid-reconstruction-sparse")];

/// 🗣️ The real committed example this artifact ships — the identity law's input.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after)` specification vector for one kind, read literally
/// via `include_str!` — this IS the independently handcrafted evidence the no-oracle decision rests
/// on, never recomputed here.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "create-stream" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/adds-stream-c-bound-to-cam-b/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/adds-stream-c-bound-to-cam-b/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/adds-stream-c-bound-to-cam-b/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-stream" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/removes-stream-b-and-cascades-its-gcp-observation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/removes-stream-b-and-cascades-its-gcp-observation/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/removes-stream-b-and-cascades-its-gcp-observation/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-stream-sync" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/shifts-stream-a-sync-offset-to-minus-seven-and-a-half/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/shifts-stream-a-sync-offset-to-minus-seven-and-a-half/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/shifts-stream-a-sync-offset-to-minus-seven-and-a-half/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-stream-frame" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/appends-a-third-frame-to-stream-a/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/appends-a-third-frame-to-stream-a/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/appends-a-third-frame-to-stream-a/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-stream-frame" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/removes-the-last-frame-of-stream-a/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/removes-the-last-frame-of-stream-a/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/removes-the-last-frame-of-stream-a/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-stream-source" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/clears-the-video-source-of-stream-a/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/clears-the-video-source-of-stream-a/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/clears-the-video-source-of-stream-a/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-asset" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/stores-a-new-jpeg-frame-asset/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/stores-a-new-jpeg-frame-asset/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/stores-a-new-jpeg-frame-asset/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-asset" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/removes-asset-a-and-reports-its-stale-references/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/removes-asset-a-and-reports-its-stale-references/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/removes-asset-a-and-reports-its-stale-references/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-camera-calibration" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/adds-the-cam-c-fisheye-calibration/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/adds-the-cam-c-fisheye-calibration/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/adds-the-cam-c-fisheye-calibration/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-camera-calibration" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/refines-the-cam-a-focal-length-and-rms/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/refines-the-cam-a-focal-length-and-rms/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/refines-the-cam-a-focal-length-and-rms/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-camera-calibration" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/removes-the-cam-b-calibration/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/removes-the-cam-b-calibration/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/removes-the-cam-b-calibration/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-rig-extrinsic" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/adds-a-rig-extrinsic-for-cam-b/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/adds-a-rig-extrinsic-for-cam-b/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/adds-a-rig-extrinsic-for-cam-b/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-rig-extrinsic" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/drops-the-cam-a-rig-extrinsic/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/drops-the-cam-a-rig-extrinsic/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/drops-the-cam-a-rig-extrinsic/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-rig-extrinsic" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/retunes-the-cam-a-rig-translation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/retunes-the-cam-a-rig-translation/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/retunes-the-cam-a-rig-translation/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-gcp" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/adds-gcp-tower-with-one-observation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/adds-gcp-tower-with-one-observation/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/adds-gcp-tower-with-one-observation/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-gcp" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/removes-gcp-corner-and-cascades-its-observation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/removes-gcp-corner-and-cascades-its-observation/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/removes-gcp-corner-and-cascades-its-observation/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-gcp-observation" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/adds-the-first-observation-to-gcp-ridge/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/adds-the-first-observation-to-gcp-ridge/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/adds-the-first-observation-to-gcp-ridge/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-gcp-observation" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/removes-the-only-observation-of-gcp-corner/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/removes-the-only-observation-of-gcp-corner/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/removes-the-only-observation-of-gcp-corner/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-ingest-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/tightens-the-ingest-sharpness-gate/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/tightens-the-ingest-sharpness-gate/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/tightens-the-ingest-sharpness-gate/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-feature-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/switches-the-detector-to-akaze/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/switches-the-detector-to-akaze/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/switches-the-detector-to-akaze/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-match-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/switches-the-matcher-to-a-kd-tree/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/switches-the-matcher-to-a-kd-tree/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/switches-the-matcher-to-a-kd-tree/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-sfm-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/switches-the-robust-loss-to-cauchy/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/switches-the-robust-loss-to-cauchy/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/switches-the-robust-loss-to-cauchy/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-dense-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/raises-the-dense-resolution-and-confidence-gate/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/raises-the-dense-resolution-and-confidence-gate/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/raises-the-dense-resolution-and-confidence-gate/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-mesh-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/doubles-the-texture-size-and-drops-the-watertight-guarantee/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/doubles-the-texture-size-and-drops-the-watertight-guarantee/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/doubles-the-texture-size-and-drops-the-watertight-guarantee/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-motion-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/enables-motion-tracking/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/enables-motion-tracking/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/enables-motion-tracking/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-geo-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/enables-georeferencing-with-an-origin/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/enables-georeferencing-with-an-origin/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/enables-georeferencing-with-an-origin/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-job" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/advances-the-job-to-texturing/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/advances-the-job-to-texturing/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/advances-the-job-to-texturing/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-sparse" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/swaps-in-an-uncolored-four-point-sparse-cloud/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/swaps-in-an-uncolored-four-point-sparse-cloud/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/swaps-in-an-uncolored-four-point-sparse-cloud/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-dense" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/swaps-in-a-two-point-classified-dense-cloud/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/swaps-in-a-two-point-classified-dense-cloud/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/swaps-in-a-two-point-classified-dense-cloud/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-mesh-result" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/swaps-in-an-imported-untextured-mesh/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/swaps-in-an-imported-untextured-mesh/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/swaps-in-an-imported-untextured-mesh/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-trajectory" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/clears-the-camera-trajectory/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/clears-the-camera-trajectory/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/clears-the-camera-trajectory/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-tracks" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/replaces-the-moving-track-with-two-static-tracks/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/replaces-the-moving-track-with-two-static-tracks/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/replaces-the-moving-track-with-two-static-tracks/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-geo-products" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/adds-the-dtm-and-ortho-rasters/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/adds-the-dtm-and-ortho-rasters/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/adds-the-dtm-and-ortho-rasters/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-qc" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/records-a-qc-report-carrying-a-watertight-summary/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/records-a-qc-report-carrying-a-watertight-summary/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/records-a-qc-report-carrying-a-watertight-summary/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "commit-reconstruction" => (
            include_str!("🧫️fixtures/commit-reconstruction-before.json"),
            include_str!("🧫️fixtures/commit-reconstruction-mutation.json"),
            include_str!("🧫️fixtures/commit-reconstruction-after.json"),
        ),
        other => panic!("mutate-remodel-1: {other:?} is not a declared kind of this subset"),
    }
}

/// 🔣️ A committed fixture parsed through the platform's own JSON reader.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-remodel-1: a committed fixture must be valid JSON: {error}"))
}

/// 🚨️ The diagnostic a kind's vector declares, if it declares one.
#[cfg(feature = "sut")]
fn declared_code(kind: &str) -> Option<&'static str> {
    DECLARED_CODE.iter().find(|(name, _)| *name == kind).map(|(_, code)| *code)
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER document, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, after) = fixture_text(kind);
        law::mutation_is_observable(kind, &canonical(after), &canonical(before), UNOBSERVABLE)?;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE document — undoing any mutation must land
/// exactly where its specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_remodel::artifacts::remodel::standards::v1::subsets::any::schema::mutations::{apply_remodel_mutation_json, round_trip_remodel_dsl, undo_remodel_mutation_json};

    /// 📥️ Splits a bridge answer into the resulting document and the diagnostic codes it raised.
    fn answer(text: &str) -> Result<(Json, Vec<String>), String> {
        let value = parse_json(text)?;
        let document = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let codes = value.array("messages").iter().map(|code| match code {
            Json::String(text) => text.clone(),
            other => other.to_string(),
        }).collect();
        Ok((document, codes))
    }

    /// 🚨️ A declared no-op or refusal must raise exactly the code its leaf's committed outcome names.
    fn raised(kind: &str, codes: &[String]) -> Result<(), String> {
        match super::declared_code(kind) {
            None => Ok(()),
            Some(code) if codes.iter().any(|raised| raised == code) => Ok(()),
            Some(code) => Err(format!("mutate-{kind}: the committed vector declares the diagnostic {code:?}, but applying it raised {codes:?}")),
        }
    }

    /// 🎯️ Applies the kind to its committed before-document and asserts the result IS the committed
    /// after-document, that the mutation moved the compared projection unless its own vector declares
    /// otherwise, and that a declared refusal really was refused.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after) = super::fixture_text(kind);
            let (document, codes) = answer(&apply_remodel_mutation_json(before, mutation)?)?;
            let expected = super::canonical(after);
            if let Some(first) = law::divergence(&document, &expected) {
                return Err(format!("mutate-{kind}: the applied document does not match the committed after-document — {first}"));
            }
            law::mutation_is_observable(kind, &document, &super::canonical(before), super::UNOBSERVABLE)?;
            raised(kind, &codes)?;
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }

    /// ↩️ The inverse law in role: applying the kind and then EVERY step of its own computed inverse
    /// must restore the committed before-document — member positions included, which is what a
    /// delete undone by re-appending would fail.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after) = super::fixture_text(kind);
            let (document, _codes) = answer(&undo_remodel_mutation_json(before, mutation)?)?;
            law::inverse_restores(kind, &document, &super::canonical(before))?;
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }

    /// 🔁️ The identity law in role, on the real committed example. Its two halves are asserted
    /// separately: the reparsed document must agree with the first parse, and the reprinted text must
    /// reproduce the committed bytes. The byte half is `carrier_is_exact` rather than the wave's
    /// usual no-pass-through tripwire because the committed `🗣️example.dsl.semio` is this codec's OWN
    /// canonical output, committed as the artifact's example — reproducing it exactly is the correct
    /// answer here and any divergence is codec drift this case exists to catch.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(super::DSL_ASSET)?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let value = parse_json(&round_trip_remodel_dsl(&text)?)?;
        let parsed = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let reparsed = value.get("reparsed").cloned().ok_or_else(|| "the bridge answer carries no reparsed document".to_string())?;
        law::round_trip_preserves(&reparsed, &parsed)?;
        law::carrier_is_exact(value.str("printed").as_bytes(), &input)?;
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors `component.feature`'s `Examples` tables exactly. `identity-round-trip` is
/// subject-only: the reference answer for every other scenario is a committed JSON document the
/// oracle role can read literally, but the real artifact is committed as DSL text ONLY and turning
/// that into a document needs this subset's own codec, which the oracle-only build must not link.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
