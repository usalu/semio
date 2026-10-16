//! 📥️ Trait impls of `🏪️store` whose trait or type belongs to `os-store-ledger` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🦀️.rs`.
use std::sync::Arc;
use crate::lifted_replay::FoldProjection;
use semio_framework_os_store_projection::retire_shared_projection;
use semio_framework_os_spr::Mutation;


impl<P, Mutation> FoldProjection<P, Mutation> for Arc<P>
where
    Mutation: self::Mutation<P>,
{
    fn projection(&self) -> &P {
        self
    }

    fn advance(&mut self, next: P) {
        retire_shared_projection::<P, Mutation>(std::mem::replace(self, Arc::new(next)));
    }
}
