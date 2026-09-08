
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
