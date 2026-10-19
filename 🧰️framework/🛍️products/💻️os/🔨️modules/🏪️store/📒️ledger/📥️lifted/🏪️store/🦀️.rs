//! 📥️ Trait impls of `🏪️store` whose trait or type belongs to `os-store-ledger` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🦀️.rs`.
use std::sync::Arc;
use crate::lifted_replay::FoldProjection;
use semio_framework_os_store_projection::retire_shared_projection;
use semio_framework_os_spr::Mutation;


impl<P, Mutation> ColdFoldProjection<P, Mutation> for Arc<P>
where
    Mutation: self::Mutation<P>,
{
    fn fold_operation(&mut self, operation: &Mutation, op_index: u32) {
        if let (Some(next), _) = fold_operation::<P, Mutation>(self.as_ref(), operation, op_index) { <Self as ColdFoldProjection<P, Mutation>>::advance(self, next); }
    }

    fn advance(&mut self, next: P) {
        match Arc::try_unwrap(std::mem::replace(self,Arc::new(next))){Ok(original)=><Mutation::ReferenceDiff as MutationDiff<P>>::retire_projection(original),Err(alias)=>drop(alias)}
    }
}
