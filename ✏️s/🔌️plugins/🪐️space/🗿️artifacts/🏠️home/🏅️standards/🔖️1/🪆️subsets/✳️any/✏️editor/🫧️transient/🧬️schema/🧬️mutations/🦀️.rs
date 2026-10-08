//! 🫧️ S Home app-transient mutation aggregate — one verb: fold one authenticated directory page into the projection.

use super::HomeTransient;

#[path = "📬️apply-directory-page/🦀️.rs"]
mod apply_directory_page;
pub use apply_directory_page::ApplyDirectoryPage;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = HomeTransient, diff = HomeTransientDiff, schema = "s.space.home.transient")]
pub enum HomeTransientMutation {
    #[dsl(key = "apply-directory-page")]
    ApplyDirectoryPage(ApplyDirectoryPage),
}

/// 🔺️ Sparse delta over [`HomeTransient`]: the directory slot, when present, is the projection the folded page leaves behind
/// (copy-on-write rows behind `Arc`s, so carrying it costs the touched rows, not the directory).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct HomeTransientDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub directory: Option<HomeTransient>,
}

/// 🫧️ The page lane declares itself non-invertible at the mutation level (derived hub state the host re-reads from the origin);
/// the diff itself still inverts to the held projection so the generic sum law holds.
impl protocol::DiffAlgebra<HomeTransient> for HomeTransientDiff {
    fn inverse(&self, base: &HomeTransient) -> Self {
        Self { directory: self.directory.as_ref().map(|_| base.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.directory.is_none()
    }
}

impl protocol::MutationDiff<HomeTransient> for HomeTransientDiff {
    fn apply(&self, base: &HomeTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<HomeTransient> {
        Ok(self.directory.clone().unwrap_or_else(|| base.clone()))
    }
    fn absorb(&mut self, other: Self) {
        if other.directory.is_some() {
            self.directory = other.directory;
        }
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
