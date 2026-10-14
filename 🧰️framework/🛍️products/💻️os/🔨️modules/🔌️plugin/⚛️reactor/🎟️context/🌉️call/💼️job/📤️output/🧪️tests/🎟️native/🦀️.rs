//! 🎟️ Focused integration tests call the same production actor entry and canonical neutral law bodies.
use semio_framework_plugin::reactor::original_actor_context::original_laws;
#[global_allocator]
static ORIGINAL_OUTPUT_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[test]
fn original_fixed_metadata(){original_laws::original_owned_job_header_matches_portable_oracle_without_allocation()}
#[test]
fn original_header_loan(){original_laws::original_owned_job_metadata_loan_denies_overwrite_before_body_without_heap()}
#[test]
fn original_owned_output(){original_laws::original_owned_output_loan_keeps_paid_producer_and_exact_handback()}
#[test]
fn original_wit_output(){original_laws::original_job_output_paged_source_identity_cpu_denials_system_and_paid_ack()}
#[test]
fn original_surface_retry(){semio_framework_plugin::reactor::patches::original_retry_laws::pending_mounted_render_survives_denied_original_retry_without_heap_or_lost_schedule()}

#[test]
fn original_manifest_receiving(){semio_framework_plugin::plugin_runtime::original_manifest_receiving_law()}
