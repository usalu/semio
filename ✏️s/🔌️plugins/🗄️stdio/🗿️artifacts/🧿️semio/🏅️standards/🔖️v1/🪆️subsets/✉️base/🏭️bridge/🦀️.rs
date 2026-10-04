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
    let rows: Vec<semio_framework_pack_json::Value> = <SemioMutation as Mutation<SemioSnapshot>>::DESCRIPTORS
        .iter()
        .map(|descriptor| {
            semio_framework_pack_json::object([
                ("id".to_string(), semio_framework_pack_json::Value::from(descriptor.semantic_kind)),
                ("variant".to_string(), semio_framework_pack_json::Value::from(descriptor.aggregate_variant)),
                ("outcomes".to_string(), semio_framework_pack_json::array(descriptor.outcome_classes.iter().map(|class| semio_framework_pack_json::Value::from(class.as_str())))),
            ])
        })
        .collect();
    let out = semio_framework_pack_json::object([
        ("schema".to_string(), semio_framework_pack_json::Value::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), semio_framework_pack_json::Value::from("s.stdio.semio")),
        ("standard".to_string(), semio_framework_pack_json::Value::from("v1")),
        ("subset".to_string(), semio_framework_pack_json::Value::from("base")),
        ("bridgeVersion".to_string(), semio_framework_pack_json::Value::from(1_i64)),
        ("mutations".to_string(), semio_framework_pack_json::array(rows)),
    ]);
    println!("{}", semio_framework_pack_json::to_string(&out));
}
