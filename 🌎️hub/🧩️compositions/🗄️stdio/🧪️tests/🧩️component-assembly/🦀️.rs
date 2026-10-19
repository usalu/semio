//! 🧩️ The stdio component assembly parses every artifact definition at most once and publishes the shared registry schemas.

/// ⏱️ Assembling the whole component parses each artifact definition AT MOST ONCE.
///
/// The bound on what the guest's `describe()` costs. `describe()` runs `plugin()` inside the OWNED
/// INTERPRETER, and `plugin()` used to walk the 36 definitions about eight times over: two full
/// `artifact_assemblies()` passes (`plugin()` itself, plus a second one built from scratch inside
/// `native_codec_factory_receipts()` purely to learn which artifacts are runtime ones), each of
/// which validates the catalog and then re-parses and re-BUILDS every definition, plus the receipt
/// pass and `native_codec_executables`. MEASURED: 196 parse+validate rounds per assembly over
/// 231 KiB of JSON, with a `format!`, a `BTreeSet` and an `ArtifactIdentity::parse` per identity —
/// invisible natively, and the reason `stdio`'s describe measured ~1 764 s against a 1 800 s guest
/// epoch and lost the coordinator's describe pass three times on a loaded machine.
///
/// Two halves. The ABSOLUTE bound: this crate compiles in exactly 36 schema documents and no test
/// parses a bespoke one, so however many sibling tests in this binary have already run, and on
/// however many threads, the process may never have parsed more than 36 — which is why
/// `validated_source` parses with its memo LOCKED instead of beside it (parsing beside it measured
/// 126 parses of 36 definitions under `--test-threads=4`). The DELTA: a complete second assembly,
/// which asks for a definition 196 times, may not parse a single one again.
#[cfg(feature = "component-app-assembly")]
#[test]
fn assembling_the_component_parses_every_artifact_definition_at_most_once() {
    let assemblies = semio_hub_stdio_catalog::artifact_assemblies().expect("warm artifact assemblies");
    assert_eq!(assemblies.len(), 36);
    let warmed = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    assert_eq!(warmed, 36, "the process parsed {warmed} definition documents; stdio compiles in exactly 36 and each is memoized on first use");

    let lookups_before = semio_s_artifact_stdio_contract::artifact_definition_lookup_count();
    let plugin = semio_hub_stdio::plugin::plugin().expect("stdio plugin assembly");
    assert_eq!(plugin.manifest.plugin_id, "stdio");
    let after = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    assert_eq!(after, warmed, "a complete component assembly re-parsed {} artifact definitions that were already memoized", after - warmed);
    let lookups = semio_s_artifact_stdio_contract::artifact_definition_lookup_count() - lookups_before;
    assert!(lookups >= 4 * 36, "one component assembly asks for {lookups} definitions; the multiplier this law bounds cannot have vanished");
}

/// ⏪️ The stdio component assembly publishes the `s.stdio.registry` shared schema documents, so the runtime input reader
/// resolves the store's per-scope documents (a kit `pin` through `framework/schema/artifact-reference/link/schema.json`).
#[cfg(feature = "component-app-assembly")]
#[test]
fn the_runtime_registry_resolves_the_store_link_scope_after_assembly() {
    semio_hub_stdio::plugin::plugin().expect("stdio plugin assembly");
    let bind = include_str!("../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪢️bind-representation/🧬️schema/🔣️.json");
    let inputs = semio_framework::mutation_input_defs(bind, &semio_framework::registered_input_schema_document).expect("the kit pin reads through the framework.schema.artifact-reference.link scope");
    assert!(inputs.iter().any(|input| input.key() == "pin"), "{inputs:?}");
}

/// 📐️ Prints the measured describe-assembly cost: definition lookups, actual parses and wall time.
///
/// Ignored by default — it is a measurement, not a law (the law above is
/// `assembling_the_component_parses_every_artifact_definition_at_most_once`). Run it with
/// `cargo test -p semio-s-plugin-stdio --features component-app-assembly -- --ignored zzz_describe
/// --nocapture --test-threads=1` when the guest's describe budget has to be re-argued with numbers.
#[cfg(feature = "component-app-assembly")]
#[ignore = "measurement, not a law"]
#[test]
fn zzz_describe_assembly_cost_report() {
    let started = std::time::Instant::now();
    let plugin = semio_hub_stdio::plugin::plugin().expect("cold stdio plugin assembly");
    let cold = started.elapsed();
    let lookups = semio_s_artifact_stdio_contract::artifact_definition_lookup_count();
    let parses = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    let warm_started = std::time::Instant::now();
    semio_hub_stdio::plugin::plugin().expect("warm stdio plugin assembly");
    println!("describe assembly: apps={} lookups={lookups} parses={parses} multiplier={:.1}x cold={:?} warm={:?}", plugin.manifest.apps.len(), lookups as f64 / parses.max(1) as f64, cold, warm_started.elapsed());
}
