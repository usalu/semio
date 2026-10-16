//! 🔋️ `s.energy.model` exhaustive mutation case — Rust adapter. Every declared kind carries two
//! committed specification vectors, one that really moves the document and one that is really
//! refused, so `UNOBSERVABLE` is empty: the weak single-no-op evidence this case used to record
//! disappeared together with the `replace-model` whole-document swap it was about (ticket
//! 26/09/06/ENERGY-PLUGIN-END-TO-END; whole-model load is now `store::ArtifactStore::reset`).
//!
//! **Where the assertions live.** The oracle role is the Python second implementation beside this
//! file, reached through the feature's `@oracle-` tag; the subject role is this repository's own
//! `energy_model_mutation_report_json`. Each asserts the forward and inverse laws in role, through
//! the shared law module `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law/🦀️.rs`, before the two are compared
//! byte for byte. The subject half is gated behind the generated host's `sut` feature so an
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

#[path = "🏷️kinds/🦀️.rs"]
mod kinds;
use self::kinds::*;

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

#[path = "🏠️building/🦀️.rs"]
mod building;
#[path = "🪟️openings/🦀️.rs"]
mod openings;
#[path = "🪨️fabric-and-gains/🦀️.rs"]
mod fabric;
#[path = "💨️airflow-and-controls/🦀️.rs"]
mod airflow;
#[path = "🔧️zone-systems/🦀️.rs"]
mod zone_systems;
#[path = "🔌️power-and-plant/🦀️.rs"]
mod power;
#[path = "📆️schedules-and-layers/🦀️.rs"]
mod schedules;
use self::airflow::AIRFLOW_VECTORS;
use self::building::BUILDING_VECTORS;
use self::fabric::FABRIC_VECTORS;
use self::openings::OPENING_VECTORS;
use self::power::POWER_VECTORS;
use self::schedules::SCHEDULE_VECTORS;
use self::zone_systems::ZONE_SYSTEM_VECTORS;

/// 🗂️ Every committed specification vector in catalog order, chained across the domain tables.
fn vectors() -> impl Iterator<Item = &'static Vector> {
    BUILDING_VECTORS.iter().chain(OPENING_VECTORS).chain(FABRIC_VECTORS).chain(AIRFLOW_VECTORS).chain(ZONE_SYSTEM_VECTORS).chain(POWER_VECTORS).chain(SCHEDULE_VECTORS)
}

fn vector(id: &str) -> &'static Vector {
    vectors().find(|vector| vector.id == id).unwrap_or_else(|| panic!("mutate-energy-model-1: no committed specification vector is registered for {id:?}"))
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-energy-model-1: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

#[path = "🔮️oracle-role/🦀️.rs"]
mod oracle;
use self::oracle::*;

#[cfg(feature = "sut")]
#[path = "🎯️subject-role/🦀️.rs"]
mod subject;

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so this loop mirrors the feature's `Examples`
/// tables exactly; `identity-round-trip` is subject-only on this side because turning the committed
/// example's DSL bytes into a document needs this subset's own codec — its oracle is the Python
/// implementation's independent carrier reader (`🐍️.py` `🔖️TextCarrier`).
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    debug_assert!(KINDS.iter().all(|kind| vectors().any(|vector| vector.kind == *kind)));
    for vector in vectors() {
        built = built.oracle(&format!("mutate-{}", vector.id), mutate_oracle_for(vector.id)).oracle(&format!("inverse-{}", vector.id), inverse_oracle_for(vector.id));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{}", vector.id), subject::mutate(vector.id)).subject(&format!("inverse-{}", vector.id), subject::inverse(vector.id).expect("valid retained mutation inverse fixture"));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
