//! 🧭️ The GisMapApprovalCommitterV1 port implementation and the retained inference job types.

use super::*;

impl GisMapApprovalCommitterV1 for RetainedGisMapApprovalCommitterV1 {
    fn commit<'a>(&'a self, request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        if let Err(error) = self.validate_prepared_request(&request) {
            return Box::pin(async move { Err(error) });
        }
        let preflight = match Self::preflight(&request) {
            Ok(preflight) => preflight,
            Err(error) => return Box::pin(async move { Err(error) }),
        };
        if let Err(error) = self.reserve_cleanup_job(request.job_id) {
            return Box::pin(async move { Err(error) });
        }
        let key = document_key(request.scope);
        let request_token = Arc::new(GisMapApprovalRequestTokenV1);
        let owner = GisMapApprovalRequestOwnerV1 {
            committer: self.clone(),
            key,
            identity: None,
            prepared: Some(GisMapPreparedApprovalV1 {
                scope: request.scope.clone(),
                job_id: request.job_id.to_owned(),
                mutation_id: request.mutation_id.to_owned(),
                command_hash: request.command_hash.to_owned(),
                proposal_hash: request.proposal_hash.to_owned(),
                ingress: request.ingress.clone(),
            }),
            request: request_token,
        };
        Box::pin(self.commit_retained(request, owner, preflight))
    }

    fn undo<'a>(&'a self, request: GisMapApprovalUndoCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        let preflight = match Self::preflight_undo(&request) {
            Ok(preflight) => preflight,
            Err(error) => return Box::pin(async move { Err(error) }),
        };
        if let Err(error) = self.reserve_cleanup_job(request.operation_id) {
            return Box::pin(async move { Err(error) });
        }
        let owner = GisMapApprovalRequestOwnerV1 { committer: self.clone(), key: document_key(request.scope), identity: None, prepared: None, request: Arc::new(GisMapApprovalRequestTokenV1) };
        Box::pin(self.commit_undo_retained(request, owner, preflight))
    }

    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(RetainedGisMapApprovalCommitterV1::close(self))
    }

    fn document_clock<'a>(&'a self, scope: &'a DocumentScope) -> std::pin::Pin<Box<dyn Future<Output = Option<protocol::HybridLogicalTimestamp>> + Send + 'a>> {
        Box::pin(async move {
            let documents = self.documents.lock().await;
            match documents.get(&document_key(scope)) {
                Some(RetainedGisMapDocumentStateV1::Ready { owners, .. }) | Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => owners.parent.as_ref().map(directory::os_store::ArtifactStore::clock_now),
                _ => None,
            }
        })
    }
}

/// 🗺️ The server-materialized Map base one job is frozen against.
pub struct InferenceMapBaseV1 {
    pub frontier: ArtifactFrontier,
    pub descriptor_digest: String,
    pub pack: InferencePrivateBytesV1,
}

impl InferenceMapBaseV1 {
    /// 🔐️ Returns the exact base-pack digest the identity froze and every recheck compares.
    pub fn digest(&self) -> String {
        sha256(self.pack.as_slice())
    }

    /// 🕸️ Returns the composed child members this Map owns, in stable-member order.
    pub fn composed_children(&self) -> Result<Vec<String>, InferenceRouteErrorV1> {
        let snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(self.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mut members = Vec::with_capacity(3);
        if !snapshot.drawing.child_id.is_empty() {
            members.push(snapshot.drawing.child_id.clone());
        }
        if let Some(image) = snapshot.image.as_ref() {
            members.push(image.child_id.clone());
        }
        if !snapshot.value.child_id.is_empty() {
            members.push(snapshot.value.child_id.clone());
        }
        Ok(members)
    }
}

/// 🧮️ One completed deterministic Map inference plus its sole typed proposal.
pub struct InferenceProposalV1 {
    pub result: InferencePrivateBytesV1,
    pub proposal: InferencePrivateBytesV1,
    pub proposal_hash: String,
}

/// 🪪️ The exact durable execution claim one retained worker owns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct InferenceRunKeyV1 {
    pub(super) job_id: String,
    pub(super) identity_digest: String,
    pub(super) run_epoch: u64,
}

impl InferenceRunKeyV1 {
    pub(super) fn same_run(&self, job_id: &str, identity_digest: &str) -> bool {
        self.job_id == job_id && self.identity_digest == identity_digest && self.run_epoch != 0
    }
}

/// 🧵️ One installed worker whose blocking child is joined by its outer task.
pub(super) struct RetainedInferenceRunV1 {
    pub(super) key: InferenceRunKeyV1,
    pub(super) control: Arc<InferenceOperationControlV1>,
    pub(super) task: JoinHandle<()>,
}

/// 🛡️ The process-owned admission, cancellation and join authority for GIS inference.
pub(super) struct RetainedInferenceRunSupervisorV1 {
    pub(super) closing: AtomicBool,
    pub(super) capacity: Arc<Semaphore>,
    pub(super) operations: Mutex<HashMap<String, RetainedInferenceRunV1>>,
}

impl RetainedInferenceRunSupervisorV1 {
    pub(super) fn new() -> Self {
        Self { closing: AtomicBool::new(false), capacity: Arc::new(Semaphore::new(OPERATION_CAPACITY)), operations: Mutex::new(HashMap::new()) }
    }

    pub(super) async fn reap_finished_operations(&self) -> Result<(), InferenceRouteErrorV1> {
        let finished = {
            let mut operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
            let job_ids = operations.iter().filter_map(|(job_id, operation)| operation.task.is_finished().then(|| job_id.clone())).collect::<Vec<_>>();
            job_ids.into_iter().filter_map(|job_id| operations.remove(&job_id)).collect::<Vec<_>>()
        };
        for operation in finished {
            operation.task.await.map_err(|_| InferenceRouteErrorV1::Storage)?;
        }
        Ok(())
    }

    pub(super) fn interrupt_operation(&self, job_id: &str, identity_digest: &str) -> Result<(), InferenceRouteErrorV1> {
        let operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if let Some(operation) = operations.get(job_id) {
            if !operation.key.same_run(job_id, identity_digest) {
                return Err(InferenceRouteErrorV1::Denied);
            }
            operation.control.cancel();
        }
        Ok(())
    }

    pub(super) async fn signal_and_join_operations(&self) -> Result<(), InferenceRouteErrorV1> {
        self.closing.store(true, Ordering::Release);
        let operations = {
            let mut operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
            std::mem::take(&mut *operations).into_values().collect::<Vec<_>>()
        };
        for operation in &operations {
            operation.control.cancel();
        }
        for operation in operations {
            operation.task.await.map_err(|_| InferenceRouteErrorV1::Storage)?;
        }
        Ok(())
    }
}

pub(super) enum InferenceOperationAdmissionV1 {
    Existing,
    Installed(oneshot::Sender<()>),
    Unclaimed,
}

/// 🔒️ The private input and authorities retained until the blocking worker is joined.
pub(super) struct RetainedInferenceRunOwnerV1 {
    pub(super) runtime: Arc<HubInferenceRuntimeV1>,
    pub(super) directory: Arc<crate::directory::HubDirectories>,
    pub(super) rebootstrap: Arc<crate::lag_rebootstrap::VerifiedRebootstrapSource>,
    pub(super) gate: Arc<tokio::sync::Mutex<()>>,
    pub(super) scope: DocumentScope,
    pub(super) identity: InferenceIdentityV1,
    pub(super) job_id: String,
    pub(super) expires_at_ms: u64,
    pub(super) claim: super::super::sqlite::InferenceRunClaimV1,
    pub(super) base: InferenceMapBaseV1,
    pub(super) control: Arc<InferenceOperationControlV1>,
    pub(super) _permit: OwnedSemaphorePermit,
}
