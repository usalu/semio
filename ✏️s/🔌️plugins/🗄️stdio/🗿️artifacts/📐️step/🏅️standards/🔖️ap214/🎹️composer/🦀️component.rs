//! 🎹️ StepComposer (ap214 standard) — aggregates its subsets' composer entries value-level:
//! `✳️any` plus the six real ISO 10303-214 conformance-class subsets `✳️cc1`..`✳️cc6`
//! (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::step::standards::v_ap214::subsets::any::composer::StepComposer as StepRawAnyComposer;
use crate::artifacts::step::standards::v_ap214::subsets::cc1::composer::StepCc1Composer;
use crate::artifacts::step::standards::v_ap214::subsets::cc2::composer::StepCc2Composer;
use crate::artifacts::step::standards::v_ap214::subsets::cc3::composer::StepCc3Composer;
use crate::artifacts::step::standards::v_ap214::subsets::cc4::composer::StepCc4Composer;
use crate::artifacts::step::standards::v_ap214::subsets::cc5::composer::StepCc5Composer;
use crate::artifacts::step::standards::v_ap214::subsets::cc6::composer::StepCc6Composer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| {
        vec![
            composer_entry_of::<StepRawAnyComposer>(),
            composer_entry_of::<StepCc1Composer>(),
            composer_entry_of::<StepCc2Composer>(),
            composer_entry_of::<StepCc3Composer>(),
            composer_entry_of::<StepCc4Composer>(),
            composer_entry_of::<StepCc5Composer>(),
            composer_entry_of::<StepCc6Composer>(),
        ]
    })
    .as_slice()
}
