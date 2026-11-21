//! 🔁️ Note's archive replay and live publication share the original document store issuer.
use super::*;

#[test]
fn note_archive_semantic_issuer_retains_one_source_through_catalog_cancellation() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let demand = note_store_owners_source_demands().unwrap();
    let grant = RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    };
    for currency in law["deniedCurrencies"].as_array().unwrap() {
        let mut short = grant;
        match currency.as_str().unwrap() {
            "items" => short.maximum_items -= 1,
            "copy" => short.maximum_copy_bytes -= 1,
            "capacity" => short.maximum_capacity_bytes -= 1,
            "depth" => short.maximum_depth -= 1,
            other => panic!("unknown native source currency {other}"),
        }
        let refused = admit_note_store_owners(short).err().expect("insufficient original grant");
        assert!(refused.owners.is_none());
        assert_eq!(refused.progress, RetainedCloneProgress::default());
    }

    let grant_for_owned_steps = RetainedCloneGrant {
        maximum_items: 64,
        maximum_copy_bytes: 262_144,
        maximum_capacity_bytes: 262_144,
        maximum_release_bytes: 262_144,
        maximum_depth: 64,
    };
    for prefix in law["constructorCancellationPrefixes"].as_array().unwrap() {
        let (mut owners, receipt) = admit_note_store_owners(grant).unwrap_or_else(|error| panic!("{:?}", error.error));
        assert!(receipt.fits(grant));
        assert_eq!(receipt.copied_bytes, demand.copy_bytes);
        assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);

        let publication = owners.one_item_preparation.as_ref().unwrap();
        let replay = owners.replay_preparation.as_ref().unwrap();
        let aliases = [std::sync::Arc::as_ptr(publication) as *const (), std::sync::Arc::as_ptr(replay) as *const ()];
        assert_eq!(aliases.len(), law["ports"].as_array().unwrap().len());
        assert_eq!(aliases.into_iter().collect::<std::collections::BTreeSet<_>>().len() as u64, law["distinctProducers"].as_u64().unwrap());

        for _ in 0..prefix.as_u64().unwrap() {
            owners.admit_constructor(grant_for_owned_steps).unwrap_or_else(|_| panic!("original Note owner catalog ticket admission"));
            assert!(owners.original_semantic_issuer_is_shared());
        }
        for _ in 0..law["maximumCloseTurns"].as_u64().unwrap() {
            if owners.uninstalled_owners_terminal_is_empty() {
                break;
            }
            let progress = owners.close_uninstalled_owners_step(grant_for_owned_steps).unwrap().progress();
            assert!(progress.fits(grant_for_owned_steps));
        }
        assert!(owners.uninstalled_owners_terminal_is_empty());
    }
}
