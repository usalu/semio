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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️catalog/🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️full-roster.json")).unwrap();
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
