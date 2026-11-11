//! 🎟️ Actual public return and retirement protocols retain every original contract law.
pub use semio_framework_job::*;
pub use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
pub use semio_framework_job_payload::{RetainedPayloadBuilder,payload_pages_and_ledger::JobPayloadStream};
pub use semio_framework_job_context::testkit::original_return_source::{claim_original_return,receive_original_return_witness,OriginalReturnPhase};
use semio_framework_job_context::testkit;
use semio_framework_value_testing_allocator as _;
/// 🧮️ Sizes a quantum-one turn so it still pays the Source's per-turn return-origin refresh quote.
pub fn quantum(caller:&mut OriginalJobFixtureCaller,stock:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedCloneGrant{let refresh=caller.turn(stock,|cx|Ok(original_return_refresh_quote(cx)?)).expect("return refresh quote");semio_framework_value::RetainedCloneGrant{maximum_items:1+refresh.copied_items,..stock}}
#[path="../../⚠️failure/🧪️tests/🧰️support/🦀️.rs"]
mod original_failure_test_support;
#[path="../../📬️outcome/⏸️continuation/🧪️tests/📦️capsule/🦀️.rs"]
mod law_0;
#[path="../../📬️outcome/⏸️continuation/🧪️tests/🎟️source/🦀️.rs"]
mod law_1;
#[path="../../📬️outcome/⏸️continuation/🧪️tests/🎟️origin/🦀️.rs"]
mod law_2;
#[path="../../♻️retirement/🎟️context/🧪️tests/🎟️work/🦀️.rs"]
mod law_3;
#[path="../../♻️retirement/📄️payload/🎟️work/⚠️posting/🧪️tests/🦀️.rs"]
mod law_4;
#[path="../🎟️source/🎟️return/🧪️tests/🦀️.rs"]
mod law_5;
#[path="../🎟️cancel/🧪️tests/🦀️.rs"]
mod source_law_0;
#[path="../🎟️source/🎟️loan/🧪️tests/🦀️.rs"]
mod source_law_1;
#[path="../🎟️source/👶️child/♻️close/🧪️tests/🦀️.rs"]
mod source_law_2;
#[path="../🎟️source/👶️child/🎟️inherit/🧪️tests/🦀️.rs"]
mod source_law_3;
