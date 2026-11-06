# Design Notes

## Preserved Source Rationale

<a id="source-notes-25"></a>

### 🦀️.rs:25

💡️ Why a bare READ of a declared inference still cannot answer a value: `inference_get` and the
`semio://artifact/{id}/inference/{field}` resource carry no canonical request body, and an
inference is a computation over one — the guest has nothing to run against. Execution itself is
no longer missing: `inference_run` (`AppCommand::Infer` →
`semio_plugin_host::ArtifactInferenceRouter`) runs any declared service for real. This
stays retryable because a later packet that caches a document-derived canonical payload per
artifact turns this exact read into a hit with no change to the discovery path above.

🔗️ An artifact-bound inference HAS a request body for this artifact — the gateway can
build it — but building it means running the guest, which a READ must not do on its
own budget. So the refusal is now an instruction with everything in it.

📜️ The published contract travels WITH the refusal, so a client that asked for a value
and got a gap learns in the same round trip what to call and what to send — instead of
having to find `inference_list` and correlate the row itself.

<a id="source-notes-67"></a>

### 🦀️.rs:67

#region 🔖️JobSeam
💡️ The `job_` handle payload an expensive inference execution would mint via
`crate::handles::HandleTable::mint(HandleKind::Job, ..)` once a real `artifact-infer` route
exists — `job_get`/`job_cancel` (`🦀️.rs`'s own `DECLARED_STUB_TOOL_NAMES`, still
unimplemented) resolve/`mark_terminal` that SAME shared `HandleTable` by the id `mint` returns.
This facet never mints one itself: no execution route exists to mint FOR yet, and an inert job
handle nobody could ever progress would be its own kind of fabrication. Pinning this shape now
means wiring the two sides together later is additive, not a redesign.
