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





impl protocol::MutationDiff<HomeTransient> for HomeTransient {
    fn apply(&self, _base: &HomeTransient) -> protocol::MutationApplyResult<HomeTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
