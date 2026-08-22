//! 🏅️ Standard root for `s.sourcing.curate` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime registration exists for
//! this artifact anywhere in the codebase — the old `definition()` capability rows only ever claimed
//! a codec id (`sourcing.curate/v1:curate`) and an extension (`curate`), never a mime type, so
//! `mimes` here is a documented synthesis, not a literal carry-over (see
//! `📓️w4-sourcing-report.md` `## openQuestions`); `extensions` IS the real, carried-over value.

use crate::artifacts::curate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard() -> StandardDeclaration<crate::SourcingApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.sourcing.curate+json"], extensions: &["curate"] }, subsets: vec![subsets::any::subset()] }
}
