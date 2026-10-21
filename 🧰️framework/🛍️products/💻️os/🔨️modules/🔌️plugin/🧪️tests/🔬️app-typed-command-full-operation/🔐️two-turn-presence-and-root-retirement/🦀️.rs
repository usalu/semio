    /// 🔐️ Two-turn presence and root retirement: the two-turn presence preparation, the local root retirement with its factory and the mounted close and registry helpers.

    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_os_store_testing_doubles::store;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
#[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct TwoTurnPublicationPresencePreparationFactory;

    struct TwoTurnPublicationPresencePreparation {
        pub request: Option<store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>>,
        pub prepared: Option<store::ArtifactEphemeralOneItemPrepared<PublicationPresence>>,
        pub checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
        pub turn: u8,
        pub closing: bool,
    }

    impl store::ArtifactEphemeralOneItemPreparationFactory<PublicationPresence, PublicationPresenceMutation> for TwoTurnPublicationPresencePreparationFactory {
        fn preflight(&self, _mutation: &PublicationPresenceMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
            Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 64 })
        }

        fn begin(
            &self,
            request: store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>,
        ) -> Result<Box<dyn store::ArtifactEphemeralOneItemPreparation<PublicationPresence, PublicationPresenceMutation>>, store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>> {
            Ok(Box::new(TwoTurnPublicationPresencePreparation { request: Some(request), prepared: None, checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), turn: 0, closing: false }))
        }
    }

    impl store::ArtifactEphemeralOneItemPreparation<PublicationPresence, PublicationPresenceMutation> for TwoTurnPublicationPresencePreparation {
        fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
            if !grant.permits_one() {
                return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            if self.turn == 0 {
                self.turn = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [1; 32] };
                return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            if self.prepared.is_none() {
                let request = self.request.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "two-turn publication-presence preparation lost its owner bundle"))?;
                let next_root = protocol::apply_diff_cold(request.mutation.diff(request.base.as_ref()).diff(), request.base.as_ref()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
                self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 2, digest: [2; 32] };
            }
            Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }

        fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
            self.checkpoint
        }

        fn prepared(&self) -> Option<&store::ArtifactEphemeralOneItemPrepared<PublicationPresence>> {
            self.prepared.as_ref()
        }

        fn take_prepared(&mut self) -> Option<store::ArtifactEphemeralOneItemPrepared<PublicationPresence>> {
            self.prepared.take()
        }

        fn cancel(&mut self) {}

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
            use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
            if !self.closing || !grant.permits_one() {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            if self.prepared.take().is_some() || self.request.take().is_some() {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(usize::from(!self.terminal_is_empty()))
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.request.is_none() && self.prepared.is_none()
        }
    }

    struct PublicationPresenceLocalRootRetirement {
        pub root: Option<std::sync::Arc<PublicationPresence>>,
    }

    impl store::ErasedSnapshotRetirement for PublicationPresenceLocalRootRetirement {
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;Ok(semio_framework_value::RetirementDemand{depth:usize::from(self.root.is_some()),..Default::default()})}

        fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
            use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
            if self.root.is_none() {
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
            }
            if grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.root.take();
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }

        fn terminal_is_empty(&self) -> bool {
            self.root.is_none()
        }

        fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(usize::from(self.root.is_some()))
        }
    }

    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct PublicationPresenceLocalRootRetirementFactory;

    impl store::SnapshotRetirementFactory<PublicationPresence> for PublicationPresenceLocalRootRetirementFactory {
        fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<PublicationPresence>) -> usize { semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<PublicationPresence>() }
        fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<PublicationPresence>)> {
            semio_framework_value::retirement::arc_custody::admit_shared_retirement(snapshot, grant, true)
        }
    }

    fn close_mounted<A: ArtifactApp>(mounted: &mut MountedTypedCommandFullOperation<A>, context: &str, original_grant:RetainedCloneGrant) {
        for _ in 0..100_000 {
            if mounted.terminal_is_empty() {
                return;
            }
            let quoted = original_grant;
            assert!(quoted.maximum_items>0 && quoted.maximum_depth>0, "{context}");
            match mounted.retirement_step(quoted).unwrap() {
                PluginLifecycleStep::Progress(progress) => assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}"),
                PluginLifecycleStep::Complete(progress) => {
                    assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}");
                    return;
                }
                PluginLifecycleStep::AwaitingInput { reason } => panic!("{context} awaited input: {reason}"),
                PluginLifecycleStep::Blocked { reason } => panic!("{context} blocked: {reason}"),
            }
        }
    }

    fn close_app<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, context: &str) {
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                return;
            }
            let quoted = app.mounted_policy.close;
            assert!(quoted.maximum_items>0 && quoted.maximum_depth>0, "{context}");
            match app.close_step(quoted).unwrap() {
                PluginLifecycleStep::Progress(progress) => assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}"),
                PluginLifecycleStep::Complete(progress) => {
                    assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}");
                    return;
                }
                PluginLifecycleStep::AwaitingInput { reason } => panic!("{context} awaited input: {reason}"),
                PluginLifecycleStep::Blocked { reason } => panic!("{context} blocked: {reason}"),
            }
        }
    }

    fn advance_registry(registry: &mut ToolLatestWinsRegistry, context: &str) {
        let quoted = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        assert!(quoted.maximum_items>0 && quoted.maximum_depth>0, "{context}");
        let (semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) | semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)) = registry.advance(quoted).unwrap();
        assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}");
    }
