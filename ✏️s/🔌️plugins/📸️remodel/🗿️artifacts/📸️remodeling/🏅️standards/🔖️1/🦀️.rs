//! 🏅️ Standard root for `s.remodel.remodeling@1` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM design.md §2). Exports `standard() -> StandardDeclaration`, mounting subset `any` —
//! this artifact's only subset.

use crate::artifacts::remodeling::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🎯️ `extensions` is the real carried-over value — the old `definition()` codec row claims
/// `codec-extension: "16:remodeling.scene:remodeling"`, i.e. the `.remodeling` document extension the
/// five `dsl::LanguageSpec`s also declare. `mimes` is a documented synthesis: no MIME registration for
/// this artifact exists anywhere in the pre-declaration code, matching `🗒️note`/`🔱️trinity`'s
/// identical deviation.
pub fn standard() -> StandardDeclaration<crate::RemodelApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.remodeling+json"], extensions: &["remodeling"] }, subsets: vec![subsets::any::subset()] }
}
