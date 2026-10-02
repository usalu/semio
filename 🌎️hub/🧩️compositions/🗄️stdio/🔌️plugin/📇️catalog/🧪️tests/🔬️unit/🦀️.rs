use super::*;

#[test]
fn selected_contribution_identities_are_unique_and_schema_owned() {
    let contributions = selected_contributions();
    semio_s_artifact_stdio_contract::registry::ContributionRegistry::new(contributions.clone()).map(|_| ()).expect("selected contribution catalog");
    assert_eq!(contributions.iter().map(|item| item.identity).collect::<BTreeSet<_>>().len(), contributions.len());
}

#[cfg(feature = "full-artifact-catalog")]
#[test]
fn full_catalog_preserves_definition_codec_and_ledger_counts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️catalog/🧫️fixtures/📇️native-catalog-surface/🧪️full-roster.json")).unwrap();
    assert_eq!(artifact_assemblies().expect("artifact assemblies").len(), 36);
    let receipts = native_codec_factory_receipts().expect("native codec receipts");
    let expected: BTreeSet<_> = fixture["identities"].as_array().unwrap().iter().map(|identity| (identity[0].as_str().unwrap(), identity[1].as_str().unwrap(), identity[2].as_str().unwrap())).collect();
    let actual: BTreeSet<_> = receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str(), receipt.factory_id.as_str())).collect();
    assert_eq!(expected.len(), 30);
    assert_eq!(receipts.len(), expected.len());
    assert_eq!(actual, expected);
    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<BTreeSet<_>>().len(), 30);
    assert_eq!(receipts.iter().map(|receipt| receipt.artifact_kind.as_str()).collect::<BTreeSet<_>>().len(), fixture["artifactKinds"].as_u64().unwrap() as usize);
    assert_eq!(fixture["artifactKinds"].as_u64().unwrap(), 29);
    let ledger = capability_ledger().expect("capability ledger");
    assert_eq!(ledger.declared, CapabilityCounts { codecs: 36, mutations: 3, inferences: 67 });
    assert_eq!(ledger.registered, CapabilityCounts { codecs: 30, mutations: 3, inferences: 67 });
    assert_eq!(ledger.implemented, CapabilityCounts { codecs: 30, mutations: 0, inferences: 0 });
    assert_eq!(ledger.verified, CapabilityCounts::default());
}

/// 🏷️ Every published format resolves by its SHORT id — the extension spelling the mesh codecs
/// answer with (`SolidExporter::format_kind` → `"step"`/`"obj"`/`"stl"`/`"glb"`) and that
/// `export_process3d_model` hands straight to `format_descriptor`. While the descriptors carried
/// the fully qualified representation id in `short_id` too, every one of those lookups failed with
/// `unknown process export format kind`; the mesh row is spelled out because process3d exports
/// exactly it, `glb` included (gltf's binary representation).
#[cfg(feature = "full-artifact-catalog")]
#[test]
fn every_published_format_owns_its_short_id_and_the_mesh_lane_is_complete() {
    let formats = format_descriptors().expect("stdio format descriptors");
    for descriptor in &formats {
        let extension = descriptor.extensions.first().expect("a published format claims an extension");
        assert_eq!(descriptor.short_id, extension.trim_start_matches('.'), "{} publishes a short id that is not its own extension", descriptor.kind_id);
        assert_ne!(descriptor.short_id, descriptor.kind_id, "{} publishes its qualified representation id as its short id", descriptor.kind_id);
    }
    let shorts = formats.iter().map(|descriptor| descriptor.short_id.as_str()).collect::<BTreeSet<_>>();
    assert_eq!(shorts.len(), formats.len(), "short format ids collide across the catalog");
    for mesh in ["step", "obj", "stl", "ply", "gltf", "glb"] {
        assert!(shorts.contains(mesh), "the mesh export lane owes a `{mesh}` format row, got {shorts:?}");
    }
}

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
    let assemblies = artifact_assemblies().expect("warm artifact assemblies");
    assert_eq!(assemblies.len(), 36);
    let warmed = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    assert_eq!(warmed, 36, "the process parsed {warmed} definition documents; stdio compiles in exactly 36 and each is memoized on first use");

    let lookups_before = semio_s_artifact_stdio_contract::artifact_definition_lookup_count();
    let plugin = crate::plugin::plugin().expect("stdio plugin assembly");
    assert_eq!(plugin.manifest.plugin_id, "stdio");
    let after = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    assert_eq!(after, warmed, "a complete component assembly re-parsed {} artifact definitions that were already memoized", after - warmed);
    let lookups = semio_s_artifact_stdio_contract::artifact_definition_lookup_count() - lookups_before;
    assert!(lookups >= 4 * 36, "one component assembly asks for {lookups} definitions; the multiplier this law bounds cannot have vanished");
}

/// ⏪️ The stdio component assembly publishes the `s.stdio.registry` shared schema documents, so the runtime input reader the
/// time-travel editor uses (`mutation_input_defs` over `registered_input_schema_document`) resolves `$defs/SnapshotPatch`
/// of every stdio `patch-snapshot` leaf — csv and json run here, png/jpg/tiff in `stdio-image`, mp4/wav in `stdio-media`,
/// and each of those packages declares the same documents — and the store's per-scope documents (a kit `pin` through
/// `os/store/link/schema.json`).
#[cfg(feature = "component-app-assembly")]
#[test]
fn the_runtime_registry_resolves_every_patch_snapshot_leaf_after_assembly() {
    crate::plugin::plugin().expect("stdio plugin assembly");
    let leaves = [
        include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json"),
        include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json"),
        include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json"),
        include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json"),
        include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json"),
        include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json"),
        include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json"),
    ];
    for leaf in leaves {
        let inputs = semio_framework::mutation_input_defs(leaf, &semio_framework::registered_input_schema_document).expect("every patch-snapshot leaf reads through the runtime registry");
        let patch = inputs.iter().find(|input| input.key() == "patch").expect("a patch input");
        assert!(matches!(&patch.schema, semio_framework::ArgSchema::Object { fields } if fields.iter().any(|field| field.key() == "edits")), "{:?}", patch.schema);
    }
    let bind = include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪢️bind-representation/🧬️schema/🔣️.json");
    let inputs = semio_framework::mutation_input_defs(bind, &semio_framework::registered_input_schema_document).expect("the kit pin reads through the os.store.link scope");
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
    let plugin = crate::plugin::plugin().expect("cold stdio plugin assembly");
    let cold = started.elapsed();
    let lookups = semio_s_artifact_stdio_contract::artifact_definition_lookup_count();
    let parses = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    let warm_started = std::time::Instant::now();
    crate::plugin::plugin().expect("warm stdio plugin assembly");
    println!("describe assembly: apps={} lookups={lookups} parses={parses} multiplier={:.1}x cold={:?} warm={:?}", plugin.manifest.apps.len(), lookups as f64 / parses.max(1) as f64, cold, warm_started.elapsed());
}

#[cfg(feature = "full-artifact-catalog")]
#[test]
fn reduced_catalog_commits_only_registered_definitions_and_codec_receipts() {
    let mut registry = ContributionRegistry::new(vec![semio_s_artifact_stdio_binary::contribution(), semio_s_artifact_stdio_txt::contribution(), semio_s_artifact_stdio_csv::contribution()]).unwrap();
    for removed in [None, Some("csv"), Some("txt"), Some("binary")] {
        if let Some(identity) = removed {
            assert!(registry.remove(identity).unwrap());
        }
        let assemblies = registry.artifact_assemblies().unwrap();
        let contribution = artifact_catalog_contribution_for(&registry, &assemblies).unwrap();
        let decoded: NativeArtifactCatalogV1 = contribution.decode().unwrap();
        let oracle = serde_json::to_value(&contribution).unwrap()["payload"].clone();
        let expected: BTreeSet<_> = registry.contributions().iter().map(|item| format!("s.stdio.{}", item.identity)).collect();
        assert_eq!(decoded.definitions.iter().map(|item| item.identity.clone()).collect::<BTreeSet<_>>(), expected);
        assert_eq!(oracle["definitions"].as_array().unwrap().iter().map(|item| item["identity"].as_str().unwrap().to_string()).collect::<BTreeSet<_>>(), expected);
        let receipts = registry.native_codec_factory_receipts("stdio", "semio:stdio", env!("CARGO_PKG_VERSION")).unwrap();
        assert_eq!(decoded.codecs.iter().map(|item| item.factory_id.clone()).collect::<BTreeSet<_>>(), receipts.iter().map(|item| item.factory_id.clone()).collect());
        for receipt in receipts {
            receipt.instantiate().unwrap();
        }
    }
    let foreign = selected_registry().unwrap().artifact_assemblies().unwrap();
    assert!(artifact_catalog_contribution_for(&registry, &foreign).is_err());
}
