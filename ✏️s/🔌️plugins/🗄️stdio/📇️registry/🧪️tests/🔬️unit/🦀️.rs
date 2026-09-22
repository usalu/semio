use super::*;

#[test]
fn selected_contribution_identities_are_unique_and_schema_owned() {
    let contributions = selected_contributions();
    validate_catalog(&contributions).expect("selected contribution catalog");
    assert_eq!(contributions.iter().map(|item| item.identity).collect::<BTreeSet<_>>().len(), expected_artifact_count());
}

#[cfg(feature = "full-artifact-catalog")]
#[test]
fn full_catalog_preserves_definition_codec_and_ledger_counts() {
    assert_eq!(artifact_assemblies().expect("artifact assemblies").len(), 36);
    assert_eq!(native_codec_factory_receipts().expect("native codec receipts").len(), 26);
    let ledger = capability_ledger().expect("capability ledger");
    assert_eq!(ledger.declared, CapabilityCounts { codecs: 32, mutations: 3, inferences: 67 });
    assert_eq!(ledger.registered, CapabilityCounts { codecs: 26, mutations: 3, inferences: 67 });
    assert_eq!(ledger.implemented, CapabilityCounts { codecs: 26, mutations: 0, inferences: 0 });
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
/// pass and `native_codec_executables`. That is ~290 parse+validate rounds over 231 KiB of JSON,
/// with a `format!`, a `BTreeSet` and an `ArtifactIdentity::parse` per identity inside `validate` —
/// invisible natively, and the reason `stdio`'s describe measured ~1 764 s against a 1 800 s guest
/// epoch and lost the coordinator's describe pass three times on a loaded machine.
///
/// The law is stated as a DELTA and not as "36 parses", because a schema is memoized per process
/// and a sibling test in this binary may already have warmed it. The first block warms every one of
/// the 36; after that, a complete second assembly may not parse a single definition again. The
/// absolute bound still holds — this crate compiles in exactly 36 schema documents and no test
/// parses a bespoke one — so the count may never exceed 36 either.
#[cfg(feature = "component-app-assembly")]
#[test]
fn assembling_the_component_parses_every_artifact_definition_at_most_once() {
    let assemblies = artifact_assemblies().expect("warm artifact assemblies");
    assert_eq!(assemblies.len(), 36);
    let warmed = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    assert!(warmed <= 36, "assembly parsed {warmed} definition documents, but stdio compiles in only 36");

    let plugin = crate::plugin::plugin().expect("stdio plugin assembly");
    assert_eq!(plugin.manifest.plugin_id, "stdio");
    let after = semio_s_artifact_stdio_contract::artifact_definition_parse_count();
    assert_eq!(after, warmed, "a complete component assembly re-parsed {} artifact definitions that were already memoized", after - warmed);
}
