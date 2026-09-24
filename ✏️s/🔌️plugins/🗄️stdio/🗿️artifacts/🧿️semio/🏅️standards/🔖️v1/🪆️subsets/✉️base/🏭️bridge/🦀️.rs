//! 🏭️ Production mutation bridge for `s.stdio.semio@v1/✉️base`.
//!
//! `test inventory` runs this and compares what it prints against the owner manifest and the claimed
//! test catalog. The answer is read out of `SemioMutation::DESCRIPTORS`, which the `dsl::Mutations`
//! derive generates from the envelope's own mutation leaves, so a verb reachable in production and
//! absent from the manifest shows up as a breach rather than as a coverage footnote.
//!
//! @see ../🔮️oracles/🔣️.json — the manifest this inventory is compared with.

extern crate semio_framework_os_kernel as protocol;

use protocol::Mutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::mutations::SemioMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::snapshot::SemioSnapshot;

/// 🎯️ Maps production's outcome severities onto the protocol's outcome classes: `Info`/`Warning`
/// ride on an applied outcome, `Error`/`Fatal` are the refusal.
fn protocol_outcomes(classes: &[protocol::MutationOutcomeClass]) -> Vec<&'static str> {
    let mut seen: Vec<&'static str> = Vec::new();
    for outcome in classes {
        let mapped = match outcome {
            protocol::MutationOutcomeClass::Applied | protocol::MutationOutcomeClass::Info | protocol::MutationOutcomeClass::Warning => "applied",
            protocol::MutationOutcomeClass::Error | protocol::MutationOutcomeClass::Fatal => "rejected",
        };
        if !seen.contains(&mapped) {
            seen.push(mapped);
        }
    }
    if seen.is_empty() {
        seen.push("applied");
    }
    seen
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [command, artifact, standard, subset] = args.as_slice() else {
        eprintln!("usage: list-mutations <artifact> <standard> <subset>");
        std::process::exit(2);
    };
    if command != "list-mutations" || artifact != "s.stdio.semio" || standard != "v1" || subset != "base" {
        eprintln!("this bridge answers only `list-mutations s.stdio.semio v1 base`, not {command} {artifact} {standard} {subset}");
        std::process::exit(2);
    }
    let rows: Vec<pack::JsonValue> = <SemioMutation as Mutation<SemioSnapshot>>::DESCRIPTORS
        .iter()
        .map(|descriptor| {
            pack::json_object([
                ("id".to_string(), pack::JsonValue::from(descriptor.semantic_kind)),
                ("variant".to_string(), pack::JsonValue::from(descriptor.aggregate_variant)),
                ("outcomes".to_string(), pack::json_array(protocol_outcomes(descriptor.outcome_classes).into_iter().map(pack::JsonValue::from))),
            ])
        })
        .collect();
    let out = pack::json_object([
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from("s.stdio.semio")),
        ("standard".to_string(), pack::JsonValue::from("v1")),
        ("subset".to_string(), pack::JsonValue::from("base")),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{}", pack::json_to_string(&out));
}
