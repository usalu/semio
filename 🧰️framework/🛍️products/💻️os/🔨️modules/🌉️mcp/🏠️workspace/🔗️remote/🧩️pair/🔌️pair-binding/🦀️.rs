//! 🔌️ Pair binding of the hub-remote crate: Hub remote binding, component assets and the native binding driver.

use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::ArtifactFrontier;
use semio_framework_os_kernel::os_directory::DocumentScope;
use std::sync::PoisonError;
use std::sync::atomic::Ordering;
use crate::remote::{HubRemoteBinding, HubRemoteBindingState};
use semio_framework_os_mcp_foundation::percent_codec::percent_encode;
use semio_framework_os_mcp_hub_pair::binding_errors::HubBindingError;
use semio_framework_os_mcp_hub_pair::pair::{CanonicalPairActorState, CanonicalPairBody, CanonicalPairFetchRequest, CanonicalPairMount, CanonicalPairMountError, CanonicalPairMountIdentity, CanonicalPairMountProgress, CanonicalPairMountStage, CanonicalPairTransport, HUB_PAIR_MEDIA_TYPE, PairBegin, checkpoint, decode_response, maximum_wire_bytes, receipt_now, receipt_wall_now, receive_body, validate_expected_identity, wait_for_published_receipt};

impl HubRemoteBinding {
    pub async fn mount_canonical_pair<T: CanonicalPairTransport>(
        &self,
        transport: &T,
        scope: &DocumentScope,
        catalog_generation: Option<u64>,
        expected: Option<&CanonicalPairMountIdentity>,
        context: &OperationContext,
        wall_now_ms: i64,
        operation_now_ms: u64,
    ) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        let started = std::time::Instant::now();
        checkpoint(context, receipt_now(operation_now_ms, started))?;
        let snapshot = self.ready_snapshot(wall_now_ms).map_err(|_| CanonicalPairMountError::DescriptorUnavailable)?;
        let document = snapshot.documents.get(scope).ok_or(CanonicalPairMountError::DescriptorUnavailable)?;
        let descriptor_digest_v1 = document.descriptor_digest_v1.clone();
        let binding_generation = self.generation.load(Ordering::SeqCst);
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        if authority_generation == 0 {
            return Err(CanonicalPairMountError::DescriptorUnavailable);
        }
        if let Some(expected) = expected {
            validate_expected_identity(expected, &self.hub_origin, authority_generation, scope, &descriptor_digest_v1, catalog_generation)?;
            let cached = { self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cached(expected) };
            if let Some(mount) = cached {
                return self.finish_mount_return(mount, binding_generation, authority_generation, scope, &descriptor_digest_v1, wall_now_ms);
            }
        }
        let receipt = match self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).begin(authority_generation, scope, expected, &context.cancel)? {
            PairBegin::Owner(receipt) => receipt,
            PairBegin::Join(receiver) => {
                let published = wait_for_published_receipt(receiver, context, operation_now_ms, started).await?;
                let mount = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cached(expected.unwrap_or(&published)).ok_or(CanonicalPairMountError::StaleCompletion)?;
                return self.finish_mount_return(mount, binding_generation, authority_generation, scope, &descriptor_digest_v1, wall_now_ms);
            }
        };
        let request_context = OperationContext { cancel: receipt.cancel.clone(), ..context.clone() };
        let request = CanonicalPairFetchRequest {
            hub_origin: self.hub_origin.clone(),
            path: format!("/spaces/{}/documents/{}/active-checkpoint/pair", percent_encode(&scope.space_id), percent_encode(&scope.document_id)),
            accept: HUB_PAIR_MEDIA_TYPE,
            maximum_response_bytes: maximum_wire_bytes(),
        };
        let response = match transport.fetch(&request_context, &request).await {
            Ok(response) => response,
            Err(error) => {
                match &error {
                    CanonicalPairMountError::Unauthorized => self.revoke(HubBindingError::Unauthorized),
                    CanonicalPairMountError::Cancelled | CanonicalPairMountError::DeadlineExceeded => {
                        self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
                    }
                    _ => self.invalidate("canonical pair transport failed"),
                }
                return Err(error);
            }
        };
        if receipt.cancel.is_cancelled_now() {
            self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
            return Err(if context.cancel.is_cancelled_now() { CanonicalPairMountError::Cancelled } else { CanonicalPairMountError::StaleCompletion });
        }
        if let Err(error) = checkpoint(&request_context, receipt_now(operation_now_ms, started)) {
            self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
            return Err(error);
        }
        match response.status {
            200 => {}
            401 | 403 => {
                self.revoke(HubBindingError::Unauthorized);
                return Err(CanonicalPairMountError::Unauthorized);
            }
            _ => {
                self.invalidate("canonical pair response was unavailable");
                return Err(CanonicalPairMountError::Unavailable);
            }
        }
        let content_type = response.content_type;
        let etag = response.etag;
        let wire = match receive_body(response.content_length, response.body, request.maximum_response_bytes, &request_context, operation_now_ms, started, |completed, total| {
            self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).report(CanonicalPairMountStage::Receiving, completed, total);
        })
        .await
        {
            Ok(wire) => wire,
            Err(error @ (CanonicalPairMountError::Cancelled | CanonicalPairMountError::DeadlineExceeded)) => {
                self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
                return Err(error);
            }
            Err(error) => {
                self.invalidate("canonical pair body receipt failed");
                return Err(error);
            }
        };
        let pair = match decode_response(&content_type, &etag, wire, &self.hub_origin, authority_generation, scope, &descriptor_digest_v1, catalog_generation, &request_context, operation_now_ms, started, |completed, total| {
            self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).report(CanonicalPairMountStage::Verifying, completed, total);
        }) {
            Ok(pair) => pair,
            Err(error @ (CanonicalPairMountError::Cancelled | CanonicalPairMountError::DeadlineExceeded)) => {
                self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
                return Err(error);
            }
            Err(error) => {
                self.invalidate("canonical pair integrity verification failed");
                return Err(error);
            }
        };
        if expected.is_some_and(|expected| expected != &pair.identity) {
            self.invalidate("canonical pair response changed its exact expected identity");
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        if let Err(error) = checkpoint(&request_context, receipt_now(operation_now_ms, started)) {
            self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
            return Err(error);
        }
        if self.generation.load(Ordering::SeqCst) != binding_generation || self.authority_generation.load(Ordering::SeqCst) != authority_generation {
            self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        let refreshed = match self.ready_snapshot(receipt_wall_now(wall_now_ms, started)) {
            Ok(snapshot) => snapshot,
            Err(_) => {
                self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).cancel_receipt(&receipt);
                return Err(CanonicalPairMountError::StaleCompletion);
            }
        };
        if refreshed.documents.get(scope).map(|document| document.descriptor_digest_v1.as_str()) != Some(descriptor_digest_v1.as_str()) {
            self.invalidate("canonical pair descriptor changed before mount publication");
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        let published = {
            let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
            let published = actor.publish(&receipt, pair);
            if published.is_err() {
                actor.cancel_receipt(&receipt);
            }
            published
        }?;
        self.finish_mount_return(published, binding_generation, authority_generation, scope, &descriptor_digest_v1, receipt_wall_now(wall_now_ms, started))
    }

    pub(crate) fn finish_mount_return(
        &self,
        mount: CanonicalPairMount,
        binding_generation: u64,
        authority_generation: u64,
        scope: &DocumentScope,
        descriptor_digest_v1: &str,
        wall_now_ms: i64,
    ) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        #[cfg(test)]
        self.pause_mount_return_for_test();
        self.validate_mount_current(&mount, binding_generation, authority_generation, scope, descriptor_digest_v1, wall_now_ms)?;
        Ok(mount)
    }

    pub(crate) fn validate_mount_current(
        &self,
        mount: &CanonicalPairMount,
        binding_generation: u64,
        authority_generation: u64,
        scope: &DocumentScope,
        descriptor_digest_v1: &str,
        wall_now_ms: i64,
    ) -> Result<(), CanonicalPairMountError> {
        let binding_matches = || self.generation.load(Ordering::SeqCst) == binding_generation && self.authority_generation.load(Ordering::SeqCst) == authority_generation;
        if !binding_matches() {
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        let ready = self.state.read().unwrap_or_else(PoisonError::into_inner);
        let descriptor_matches = matches!(
            &*ready,
            HubRemoteBindingState::Ready(snapshot)
                if snapshot.session_expires_at_ms > wall_now_ms
                    && snapshot.documents.get(scope).map(|document| document.descriptor_digest_v1.as_str()) == Some(descriptor_digest_v1)
        );
        if !descriptor_matches {
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        drop(ready);
        let actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        let actor_matches = actor.binding_generation == authority_generation
            && matches!(actor.state, CanonicalPairActorState::Mounted | CanonicalPairActorState::Loading)
            && actor.mounted.as_ref().is_some_and(|active| active.identity == mount.identity && active.mount_id == mount.mount_id);
        if !actor_matches || !binding_matches() {
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        Ok(())
    }

    /// 🧪 Borrows one exact mounted canonical pair only while its private cache actor remains
    /// locked, then repeats the full binding/descriptor/actor fence before publishing the owned
    /// projection. The callback is deliberately synchronous and cannot escape the retained bytes.
    pub(crate) fn project_mounted_canonical_pair<T>(
        &self,
        mount: &CanonicalPairMount,
        wall_now_ms: i64,
        project: impl FnOnce(&CanonicalPairMountIdentity, &ArtifactFrontier, &[u8], &[u8]) -> Result<T, CanonicalPairMountError>,
    ) -> Result<T, CanonicalPairMountError> {
        let binding_generation = self.generation.load(Ordering::SeqCst);
        let authority_generation = mount.identity.authority_generation;
        let scope = mount.identity.scope.clone();
        let descriptor_digest_v1 = mount.identity.descriptor_digest_v1.clone();
        self.validate_mount_current(mount, binding_generation, authority_generation, &scope, &descriptor_digest_v1, wall_now_ms)?;
        let projected = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).project_mounted(mount, project)?;
        #[cfg(test)]
        self.pause_mount_return_for_test();
        self.validate_mount_current(mount, binding_generation, authority_generation, &scope, &descriptor_digest_v1, wall_now_ms)?;
        Ok(projected)
    }

    #[cfg(test)]
    pub(crate) fn pause_mount_return_for_test(&self) {
        let pause = self.pair_mount_return_pause.lock().unwrap_or_else(PoisonError::into_inner).take();
        if let Some((reached, release)) = pause {
            reached.wait();
            release.wait();
        }
    }

    pub fn canonical_pair_progress(&self) -> CanonicalPairMountProgress {
        self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).progress.clone()
    }

    #[cfg(test)]
    pub(crate) fn canonical_pair_test_stats(&self) -> (CanonicalPairActorState, usize, usize, usize, u64, u64) {
        let actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        (actor.state, actor.loadings.len(), actor.cache.len(), actor.cached_bytes, actor.cache_hits, actor.cache_misses)
    }

    #[cfg(test)]
    pub(crate) fn pause_next_canonical_pair_mount_return(&self, reached: std::sync::Arc<std::sync::Barrier>, release: std::sync::Arc<std::sync::Barrier>) {
        *self.pair_mount_return_pause.lock().unwrap_or_else(PoisonError::into_inner) = Some((reached, release));
    }
}
