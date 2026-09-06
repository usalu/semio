//! 🔋️ `s.energy.model` exhaustive mutation case — Rust adapter. Every declared kind carries two
//! committed specification vectors, one that really moves the document and one that is really
//! refused, so `UNOBSERVABLE` is empty: the weak single-no-op evidence this case used to record
//! disappeared together with the `replace-model` whole-document swap it was about (ticket
//! 26/09/06/ENERGY-PLUGIN-END-TO-END; whole-model load is now `store::ArtifactStore::reset`).
//!
//! **Where the assertions live.** The oracle role is the Python second implementation beside this
//! file, reached through the feature's `@oracle-` tag; the subject role is this repository's own
//! `energy_model_mutation_report_json`. Each asserts the forward and inverse laws in role, through
//! the shared law module `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs`, before the two are compared
//! byte for byte. The subject half is gated behind the generated host's `sut` feature so an
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. That
/// file's own `direct_owner_descriptors_and_catalog_correspond` keeps the list honest against both
/// the enum and the catalog.
const KINDS: &[&str] = &[
    "rename-model",
    "change-model-version",
    "update-site",
    "update-ground-temperature",
    "update-run-period",
    "replace-airflow-network",
    "add-output-variable",
    "remove-output-variable",
    "bind-weather-file",
    "unbind-weather-file",
    "connect-referenced-model",
    "disconnect-referenced-model",
    "rename-zone",
    "change-zone-volume",
    "change-zone-multiplier",
    "change-zone-conditioned",
    "change-zone-floor-area-participation",
];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect. Empty: every kind
/// owns a vector that moves the document.
const UNOBSERVABLE: &[&str] = &[];

/// 🗣️ The real committed document this artifact ships as its own example.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One committed `(before, mutation, after, diff, outcome)` specification vector, read literally
/// via `include_str!` — never recomputed here, never restated as a Rust literal.
struct Vector {
    id: &'static str,
    kind: &'static str,
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: &'static str,
    outcome: &'static str,
}

const VECTORS: &[Vector] = &[
        Vector {
            id: "rename-model-renames-the-model",
            kind: "rename-model",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "rename-model-refuses-a-blank-name",
            kind: "rename-model",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-model-version-bumps-the-version",
            kind: "change-model-version",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-model-version-refuses-a-blank-version",
            kind: "change-model-version",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "update-site-relocates-to-denver",
            kind: "update-site",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "update-site-refuses-a-bad-latitude",
            kind: "update-site",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "update-ground-temperature-sets-denver-ground",
            kind: "update-ground-temperature",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "update-ground-temperature-refuses-a-short-year",
            kind: "update-ground-temperature",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "update-run-period-shortens-to-january",
            kind: "update-run-period",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "update-run-period-refuses-month-13",
            kind: "update-run-period",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "replace-airflow-network-attaches-a-network",
            kind: "replace-airflow-network",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "replace-airflow-network-refuses-unpaired-nodes",
            kind: "replace-airflow-network",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "add-output-variable-adds-zone-air-temp",
            kind: "add-output-variable",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "add-output-variable-refuses-a-duplicate",
            kind: "add-output-variable",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "remove-output-variable-drops-zone-air-temp",
            kind: "remove-output-variable",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "remove-output-variable-refuses-an-absent-one",
            kind: "remove-output-variable",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "bind-weather-file-binds-hannover-epw",
            kind: "bind-weather-file",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "bind-weather-file-refuses-a-bad-uri",
            kind: "bind-weather-file",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "unbind-weather-file-unbinds-the-weather",
            kind: "unbind-weather-file",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "unbind-weather-file-refuses-when-unbound",
            kind: "unbind-weather-file",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "connect-referenced-model-connects-the-geometry",
            kind: "connect-referenced-model",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "connect-referenced-model-refuses-a-bad-uri",
            kind: "connect-referenced-model",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "disconnect-referenced-model-disconnects-the-geometry",
            kind: "disconnect-referenced-model",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "disconnect-referenced-model-refuses-when-absent",
            kind: "disconnect-referenced-model",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "rename-zone-renames-zone-one",
            kind: "rename-zone",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "rename-zone-refuses-a-missing-zone",
            kind: "rename-zone",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-volume-resizes-zone-one",
            kind: "change-zone-volume",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-volume-refuses-zero-volume",
            kind: "change-zone-volume",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-multiplier-stacks-four-storeys",
            kind: "change-zone-multiplier",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-multiplier-refuses-zero-instances",
            kind: "change-zone-multiplier",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-conditioned-frees-the-zone",
            kind: "change-zone-conditioned",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-conditioned-refuses-a-missing-zone",
            kind: "change-zone-conditioned",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-floor-area-participation-excludes-the-zone",
            kind: "change-zone-floor-area-participation",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone/🎯️outcome/🔣️.json"),
        },
        Vector {
            id: "change-zone-floor-area-participation-refuses-a-missing-zone",
            kind: "change-zone-floor-area-participation",
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone/🎯️outcome/🔣️.json"),
        },
];

fn vector(id: &str) -> &'static Vector {
    VECTORS.iter().find(|vector| vector.id == id).unwrap_or_else(|| panic!("mutate-energy-model-1: no committed specification vector is registered for {id:?}"))
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-energy-model-1: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed after-snapshot, read literally.
fn mutate_oracle_for(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let after = vector(id).after;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed before-snapshot.
fn inverse_oracle_for(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let before = vector(id).before;
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DSL_ASSET, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_energy::artifacts::model::standards::v1::subsets::any::schema::mutations::energy_model_mutation_report_json;
    use semio_s_plugin_energy::artifacts::model::standards::v1::subsets::any::schema::snapshot::energy_model_identity_report_json;
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️Report
    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }

    fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
        match member(report, key)? {
            Json::Array(items) => Ok(items.clone()),
            other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
        }
    }

    fn text(report: &Json, key: &str) -> Result<String, String> {
        match member(report, key)? {
            Json::String(value) => Ok(value.clone()),
            other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
        }
    }

    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .map(|segment| match segment {
                Json::String(text) => text.clone(),
                other => other.to_string(),
            })
            .collect()
    }

    fn level_of(word: &str) -> String {
        if word == "warn" {
            "warning".to_string()
        } else {
            word.to_string()
        }
    }

    /// 🎯️ Checks the produced diagnostics against the ones the committed `🎯️outcome` declares.
    fn declared_outcome_holds(id: &str, produced: &[Json], outcome: &Json) -> Result<(), String> {
        let codes: Vec<String> = produced.iter().map(|message| message.str("code")).collect();
        let levels: Vec<String> = produced.iter().map(|message| level_of(&message.str("level"))).collect();
        if outcome.str("status") == "rejected" {
            let expected = outcome.str("code");
            if codes != vec![expected.clone()] {
                return Err(format!("mutate-{id}: the vector declares a rejection with code {expected:?}, the implementation raised {codes:?}"));
            }
            if !levels.iter().any(|level| level == "error" || level == "fatal") {
                return Err(format!("mutate-{id}: the vector declares a rejection, but the implementation raised it at {levels:?}"));
            }
            let path = strings(outcome, "path");
            let target = strings(&produced[0], "target");
            if !path.is_empty() && target != path {
                return Err(format!("mutate-{id}: the vector declares the offending address {path:?}, the implementation reported {target:?}"));
            }
            return Ok(());
        }
        let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
        if codes != expected {
            return Err(format!("mutate-{id}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
        }
        match levels.iter().find(|level| level.as_str() == "error" || level.as_str() == "fatal") {
            Some(level) => Err(format!("mutate-{id}: the vector declares an applied outcome, but the implementation raised a {level}")),
            None => Ok(()),
        }
    }
    //#endregion 🔖️Report

    //#region 🔖️Handlers
    /// 🎯️ Applies the vector and asserts the resulting document, the produced delta and the declared
    /// diagnostics all match what the vector commits to.
    pub fn mutate(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(id);
            let report = parse_json(&energy_model_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("mutate-{id}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let applied = member(&report, "snapshot")?;
            let expected = member(&report, "expectedSnapshot")?;
            if let Some(first) = law::divergence(applied, expected) {
                return Err(format!("mutate-{id}: the applied document is not the committed after-snapshot — {first}"));
            }
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
                return Err(format!("mutate-{id}: the produced delta is not the committed 🔺️diff — {first}"));
            }
            declared_outcome_holds(id, &members(&report, "messages")?, &canonical(committed.outcome))?;
            if canonical(committed.outcome).str("status") != "rejected" {
                law::mutation_is_observable(committed.kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            }
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the vector and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly.
    pub fn inverse(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(id);
            let report = parse_json(&energy_model_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("inverse-{id}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?
                .iter()
                .filter(|message| {
                    let level = message.str("level");
                    level == "error" || level == "fatal"
                })
                .map(|message| message.str("code"))
                .collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{id}: an inverse step was rejected with {faults:?}, so the document never got the chance to return"));
            }
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(committed.kind, restored, member(&report, "base")?)?;
            Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored.clone()))
        }
    }

    /// 🔁️ The real committed document through this subset's own two codecs.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text_bytes = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let report = parse_json(&energy_model_identity_report_json(&text_bytes).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed.clone()))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so this loop mirrors the feature's `Examples`
/// tables exactly; `identity-round-trip` is subject-only because turning the committed example's DSL
/// bytes into a document needs this subset's own codec.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    debug_assert!(KINDS.iter().all(|kind| VECTORS.iter().any(|vector| vector.kind == *kind)));
    for vector in VECTORS {
        built = built.oracle(&format!("mutate-{}", vector.id), mutate_oracle_for(vector.id)).oracle(&format!("inverse-{}", vector.id), inverse_oracle_for(vector.id));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{}", vector.id), subject::mutate(vector.id)).subject(&format!("inverse-{}", vector.id), subject::inverse(vector.id));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
