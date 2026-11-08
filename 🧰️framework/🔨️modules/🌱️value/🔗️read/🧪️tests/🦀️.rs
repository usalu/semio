//! 🧪️ Original sparse visitation and physical read ownership share one portable fixture.
use semio_framework_value_read::*;
use semio_framework_value_read::ErasedSnapshotRetirement;

use semio_framework_value_control_contract::*;
use semio_framework_value_native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,NativeReceivingEffect,NativeReceivingEffectPoll};
use semio_framework_value_retirement::{OriginalRetirementCloseLoan,OriginalRetirementFailure};
use semio_framework_value_read::{IssuedReadRegistry,ReadAuthority,ReadLease,ReadLeaseIssue,ReadVisit,ErasedReadLease,OriginalReadSource,OriginalErasedReadSource,ReadSourceCursor};
use semio_framework_value_read::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_retirement::retirement::arc_custody::original::SharedOriginalStorage;
use std::mem::ManuallyDrop;
#[path="../../🛬️decode/🏭️construction/🧪️tests/source-bank.rs"]mod source_bank;
#[path="../🏗️birth/🧪️tests/shared.rs"]mod shared;
use shared::*;
#[path="../🏗️birth/🧪️tests/sparse.rs"]mod sparse;
#[path="../🏗️birth/🧪️tests/source-owner.rs"]mod source_owner;
#[path="../🏗️birth/🧪️tests/alias.rs"]mod alias;
#[path="../🏗️birth/🧪️tests/erase.rs"]mod erase;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}

#[test]
fn typed_read_registry_retains_every_original_sparse_wrap_and_starvation_vector(){sparse::run_sparse()}

#[test]
fn typed_read_registry_ctor_return_source_and_cancel_conserve_actual_heap_with_full_grants(){source_owner::run_typed_source()}

#[test]
fn erased_read_authority_retains_typed_roots_and_every_alias_until_full_physical_closure(){alias::run_authority_aliases()}

#[test]
fn erased_read_admission_preserves_original_pointer_and_cancelled_full_frame_receipts(){erase::run_erased_cancellation()}

#[test]
fn erased_read_authority_close_order_preserves_original_root_and_full_physical_receipts(){erase::run_close_orders()}

#[test]
fn erased_read_source_retains_original_id_commit_authority_and_root_after_read_return(){source_owner::run_erased_source()}
