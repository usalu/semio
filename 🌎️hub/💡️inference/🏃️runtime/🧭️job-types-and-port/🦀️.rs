//! 🧭️ The GisMapApprovalCommitterV1 port implementation and the retained inference job types.

use std::sync::{Arc, Mutex};
use directory::os_directory::ArtifactFrontier;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use semio_hub_inference::{InferenceOperationControlV1, InferencePrivateBytesV1, sha256};
use crate::approval_contract::InferenceRouteErrorV1;
use tokio::task::JoinHandle;
use semio_hub_inference_binding::MapInferenceProposalBindingV1;
use crate::OPERATION_CAPACITY;
use tokio::sync::Semaphore;

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

    /// 🕸️ Returns the composed child members this Map owns, in stable-member order, as the registered binding reads them.
    pub fn composed_children(&self, proposals: &dyn MapInferenceProposalBindingV1) -> Result<Vec<String>, InferenceRouteErrorV1> {
        proposals.composed_children(self.pack.as_slice()).map_err(InferenceRouteErrorV1::from)
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
pub struct InferenceRunKeyV1 {
    pub job_id: String,
    pub identity_digest: String,
    pub run_epoch: u64,
}

impl InferenceRunKeyV1 {
    pub fn same_run(&self, job_id: &str, identity_digest: &str) -> bool {
        self.job_id == job_id && self.identity_digest == identity_digest && self.run_epoch != 0
    }
}

/// 🧵️ One installed worker whose blocking child is joined by its outer task.
pub struct RetainedInferenceRunV1 {
    pub key: InferenceRunKeyV1,
    pub control: Arc<InferenceOperationControlV1>,
    pub task: JoinHandle<()>,
}

/// 🛡️ The process-owned admission, cancellation and join authority for GIS inference.
pub struct RetainedInferenceRunSupervisorV1 {
    pub closing: AtomicBool,
    pub capacity: Arc<Semaphore>,
    pub operations: Mutex<HashMap<String, RetainedInferenceRunV1>>,
}

impl RetainedInferenceRunSupervisorV1 {
    pub fn new() -> Self {
        Self { closing: AtomicBool::new(false), capacity: Arc::new(Semaphore::new(OPERATION_CAPACITY)), operations: Mutex::new(HashMap::new()) }
    }

    pub async fn reap_finished_operations(&self) -> Result<(), InferenceRouteErrorV1> {
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

    pub fn interrupt_operation(&self, job_id: &str, identity_digest: &str) -> Result<(), InferenceRouteErrorV1> {
        let operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if let Some(operation) = operations.get(job_id) {
            if !operation.key.same_run(job_id, identity_digest) {
                return Err(InferenceRouteErrorV1::Denied);
            }
            operation.control.cancel();
        }
        Ok(())
    }

    pub async fn signal_and_join_operations(&self) -> Result<(), InferenceRouteErrorV1> {
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

pub enum InferenceOperationAdmissionV1 {
    Existing,
    Installed(oneshot::Sender<()>),
    Unclaimed,
}

