mod artifact_fixed_registry_tests {
    use super::*;

    #[derive(Debug)]
    struct ActiveMediaExportSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
    #[derive(Debug)]
    struct SnapshotRetentionSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
    #[derive(Debug)]
    struct SegmentedDownloadSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);

    macro_rules! sentinel_drop {
        ($owner:ty) => {
            impl Drop for $owner {
                fn drop(&mut self) {
                    self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }
        };
    }

    sentinel_drop!(ActiveMediaExportSentinel);
    sentinel_drop!(SnapshotRetentionSentinel);
    sentinel_drop!(SegmentedDownloadSentinel);

    fn reject_duplicate<T: std::fmt::Debug>(first: T, duplicate: T, drops: &std::sync::Arc<std::sync::atomic::AtomicUsize>) {
        let mut registry = ArtifactFixedRegistry::new();
        let mut close_registry = ArtifactFixedRegistry::new();
        registry.insert(17, first).expect("first exact owner");
        let duplicate = registry.insert(17, duplicate).expect_err("an occupied same-id slot must reject replacement ownership");
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(close_registry.can_insert(17));
        close_registry.insert_admitted(17, duplicate);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        let first = registry.remove(17).expect("exact live owner");
        let duplicate = close_registry.remove(17).expect("exact rejected-owner quarantine");
        drop(first);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        drop(duplicate);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[test]
    fn duplicate_id_never_drops_active_media_snapshot_or_segmented_download_ownership() {
        let media_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        reject_duplicate(ActiveMediaExportSentinel(media_drops.clone()), ActiveMediaExportSentinel(media_drops.clone()), &media_drops);
        let snapshot_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        reject_duplicate(SnapshotRetentionSentinel(snapshot_drops.clone()), SnapshotRetentionSentinel(snapshot_drops.clone()), &snapshot_drops);
        let download_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        reject_duplicate(SegmentedDownloadSentinel(download_drops.clone()), SegmentedDownloadSentinel(download_drops.clone()), &download_drops);
    }

    #[test]
    fn media_construction_failure_at_each_fallible_seam_keeps_exact_close_authority() {
        struct ConstructionOwner {
            remaining_steps: u8,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }
        impl Drop for ConstructionOwner {
            fn drop(&mut self) {
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut closures = ArtifactFixedRegistry::new();
        for (operation_id, _) in ["builder", "dispatch", "session", "worker-submit"].into_iter().enumerate() {
            closures.insert_admitted(operation_id as u64, ConstructionOwner { remaining_steps: operation_id as u8 + 1, drops: drops.clone() });
        }
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            let Some(operation_id) = closures.id_at(index) else { continue };
            while closures.get(operation_id).is_some_and(|owner| owner.remaining_steps != 0) {
                closures.get_mut(operation_id).expect("exact construction owner").remaining_steps -= 1;
            }
            drop(closures.remove(operation_id));
        }
        assert!(closures.is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 4);
    }

    #[test]
    fn media_registry_detach_preserves_unrelated_live_owner_and_exact_close_handoff() {
        struct MediaOwner {
            identity: std::sync::Arc<()>,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }
        impl Drop for MediaOwner {
            fn drop(&mut self) {
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let first_identity = std::sync::Arc::new(());
        let second_identity = std::sync::Arc::new(());
        let mut live = ArtifactFixedRegistry::new();
        let mut closing = ArtifactFixedRegistry::new();
        live.insert_admitted(41, MediaOwner { identity: first_identity.clone(), drops: drops.clone() });
        live.insert_admitted(42, MediaOwner { identity: second_identity.clone(), drops: drops.clone() });
        let first = live.remove(41).expect("exact media owner detaches once");
        assert!(std::sync::Arc::ptr_eq(&first.identity, &first_identity));
        assert!(std::sync::Arc::ptr_eq(&live.get(42).expect("unrelated media owner remains live").identity, &second_identity));
        closing.insert_admitted(41, first);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        drop(closing.remove(41).expect("exact detached owner transfers to close"));
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        drop(live.remove(42).expect("unrelated owner remains independently removable"));
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    fn ingress_with_page(byte: u8) -> ActiveArtifactEnvelopeIngress {
        let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }).expect("one exact ingress page credit");
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[0] = byte;
        store::ArtifactEnvelopeDecodePage::try_from_array(bytes, 1).expect("one-byte exact page").admit_into(&mut pages).unwrap_or_else(|_| panic!("pre-admitted ingress page"));
        ActiveArtifactEnvelopeIngress::new(pages)
    }

    fn close_ingress(mut ingress: ActiveArtifactEnvelopeIngress) {
        assert_eq!(ingress.close_step(0, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(ingress.close_step(1, 0), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(ingress.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), PluginCloseStep::Pending { released_items: 1, released_bytes: 1 });
        assert_eq!(ingress.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), PluginCloseStep::Complete);
        assert!(ingress.terminal_is_empty());
        drop(ingress);
    }

    #[test]
    fn artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots() {
        let mut live = ArtifactFixedRegistry::new();
        for operation in 0..ARTIFACT_LIVE_OUTPUT_SLOTS as u64 {
            live.insert(operation, ingress_with_page(operation as u8)).unwrap_or_else(|_| panic!("fixed ingress slot"));
        }
        let rejected = live.insert(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, ingress_with_page(255)).err().unwrap_or_else(|| panic!("modulo-colliding plus-one ingress returns its exact owner"));
        close_ingress(rejected);
        for operation in 0..ARTIFACT_LIVE_OUTPUT_SLOTS as u64 {
            close_ingress(live.remove(operation).expect("exact ingress slot closes in admission order"));
        }
        assert!(live.is_empty());
    }

    #[test]
    fn artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant() {
        let mut ingress = ingress_with_page(7);
        ingress.closing = true;
        close_ingress(ingress);
    }
}
