//! 🫧️ S Home app-transient mutation aggregate — one verb: fold one authenticated directory page into the projection.

use super::HomeTransient;

#[path = "📬️apply-directory-page/🦀️.rs"]
mod apply_directory_page;
pub use apply_directory_page::ApplyDirectoryPage;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = HomeTransient, diff = HomeTransient, schema = "s.space.home.transient")]
pub enum HomeTransientMutation {
    #[dsl(key = "apply-directory-page")]
    ApplyDirectoryPage(ApplyDirectoryPage),
}





/// 🫧️ The directory projection is derived hub state published one bounded page at a time, so its diff IS the folded projection
/// (copy-on-write rows behind `Arc`s) and is never inverted: the page lane declares itself non-invertible.
impl protocol::DiffAlgebra<HomeTransient> for HomeTransient {
    fn inverse(&self, base: &HomeTransient) -> Self {
        base.clone()
    }
    fn between(_base: &HomeTransient, other: &HomeTransient) -> Self {
        other.clone()
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl protocol::MutationDiff<HomeTransient> for HomeTransient {
    fn apply(&self, _base: &HomeTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<HomeTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
