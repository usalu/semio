//! 📥️ Items of `initialization_replay` that belong to `os-store-initialization-runtime` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🏗️initialization/🔁️replay/🦀️.rs`.
use semio_framework_value::ArtifactOwnedValueRetirementFactory;
use crate::lifted_store::ArtifactStoreInitializationRuntime;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use semio_framework_os_spr::{Edit, Mutation, OpBinary};
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_store_replay::fold_operation;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::sync::Arc;
/// ▶️ What folding one forward of an applied edit onto the initialization runtime's current projection produced.
pub enum ArtifactStoreInitializationNext<P> {
    /// 🏁️ The edit has no forward at that index.
    Exhausted,
    /// 🪙️ The operation's encoded bytes exceed the copy bytes the caller granted; nothing changed.
    Unfunded,
    /// ⏭️ The forward folded with keep-and-record semantics: `next` is the projection it produced (`None` when it folded as a
    /// no-op — withdrawn, faulted or refused) and `copy_bytes` the operation's encoded size.
    Folded { next: Option<P>, copy_bytes: usize },
}

impl<P> ArtifactStoreInitializationRuntime<P> {
    /// ▶️ Folds forward `index` of the applied `edit` onto the shared current projection through its effective input
    /// (supersessions, [`admit_replacement`](super::admit_replacement) and keep-and-record [`fold_operation`](super::fold_operation)),
    /// handing the produced projection to the caller instead of mutating a uniquely owned workspace: the genesis alias is never
    /// cloned, the one `diff` and `apply` of the operation is the whole traversal of the turn. Refuses an operation whose encoding
    /// fails or exceeds `maximum_bytes`; answers [`ArtifactStoreInitializationNext::Unfunded`] when it exceeds `granted_copy_bytes`.
    pub fn fold_next_forward<M>(&mut self, edit: &Edit<M>, index: usize, schema: &str, maximum_bytes: usize, granted_copy_bytes: usize) -> Result<ArtifactStoreInitializationNext<P>, &'static str>
    where
        M: Mutation<P> + OpBinary,
    {
        let Some(effective) = self.effective_forward(edit, index, schema) else { return Ok(ArtifactStoreInitializationNext::Exhausted) };
        let operation = effective.operation();
        let copy_bytes = match operation.map(OpBinary::encode_op) {
            None => 0,
            Some(Ok(encoded)) if encoded.len() <= maximum_bytes => encoded.len(),
            Some(_) => return Err("artifact store initialization forward encoding exceeds its page"),
        };
        if copy_bytes > granted_copy_bytes {
            return Ok(ArtifactStoreInitializationNext::Unfunded);
        }
        if let Some(original) = edit.forwards.get(index).filter(|op| op.may_emit_foreign_steps()) {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            let source = self.current.as_deref().ok_or("artifact store initialization lost its current projection")?;
            let presence = original.foreign_step_source(source, 0).map_err(|_| "artifact store original foreign source refused")?.is_some();
            self.revision.unit_flags.insert((digest, index), presence);
        }
        let next = match operation {
            None => None,
            Some(operation) => fold_operation::<P, M>(self.current.as_deref().ok_or("artifact store initialization lost its current projection")?, operation, index as u32).0,
        };
        Ok(ArtifactStoreInitializationNext::Folded { next, copy_bytes })
    }

    /// 📏️ Quotes [`Self::adopt_next_funded`]: the returned-read frame that retires the displaced projection and the shared
    /// backing of its successor.
    pub fn adopt_next_demand() -> RetirementDemand
    where
        P: Send + Sync + 'static,
    {
        RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes() + semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<P>(), depth: 2, ..Default::default() }
    }

    /// 🎟️ Makes `next` the current projection under one funded turn: the displaced projection moves into a returned-read
    /// retirement frame that [`Self::settle_current_retirement_step`] drains, and `next` takes its shared backing. Refusal
    /// hands `next` back and leaves the runtime unchanged.
    pub fn adopt_next_funded(&mut self, next: P, factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, P)>
    where
        P: Send + Sync + 'static,
    {
        let demand = Self::adopt_next_demand();
        if self.close_phase != 0 || self.close_active.is_some() || self.current.is_none() {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "initialization adoption requires a settled open current projection"), next));
        }
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes {
            return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "initialization adoption exceeds its admitted grant"), next));
        }
        let previous = self.current.take().expect("open initialization retains its current projection");
        match ReturnedSnapshotReadRetirement::admit(previous, Arc::clone(factory), grant) {
            Ok((owner, mut receipt)) => {
                *self.close_active = Some(owner);
                *self.current = Some(Arc::new(next));
                receipt.retained_capacity_bytes += semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<P>();
                Ok(receipt)
            }
            Err((error, previous, alias)) => {
                *self.current = Some(previous);
                drop(alias);
                Err((error, next))
            }
        }
    }
}
