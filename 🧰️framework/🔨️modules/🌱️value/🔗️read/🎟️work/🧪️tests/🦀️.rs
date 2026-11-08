//! 🎟️ Original read Work laws retain actual Source, ownership and physical receipts.
use semio_framework_value_read::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,NativeReceivingEffect,NativeReceivingEffectPoll};
use semio_framework_value_retirement::{OriginalRetirementCloseLoan,OriginalRetirementFailure};
use semio_framework_value_read::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_retirement::retirement::arc_custody::original::SharedOriginalStorage;
use std::mem::ManuallyDrop;
#[path="../../../🛬️decode/🏭️construction/🧪️tests/source-bank.rs"]mod source_bank;
#[path="../../🏗️birth/🧪️tests/shared.rs"]mod shared;
use shared::*;
#[path="../../🏗️birth/🧪️tests/sparse.rs"]mod sparse;
#[path="../../🏗️birth/🧪️tests/work.rs"]mod work;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn real_read_alias_child_handoff_and_empty_header_have_distinct_quantum_one_work(){work::run_read_work()}
#[test]
fn canonical_borrowed_source_null_retains_the_same_read_and_direct_attempt_work(){work::run_source_null()}
