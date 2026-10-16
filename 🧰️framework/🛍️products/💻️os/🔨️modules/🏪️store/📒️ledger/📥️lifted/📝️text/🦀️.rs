//! 📥️ Items of `text_format` that belong to `os-store-ledger` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🚪️io/📝️text/🦀️.rs`.
use semio_framework_os_spr::MutationId;
pub(crate) fn mutation_ids(ids: Vec<String>) -> Vec<MutationId> {
    ids.into_iter().map(MutationId).collect()
}
