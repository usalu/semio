# Design Notes

## Preserved Source Rationale

<a id="source-notes-132"></a>

### 🦀️.rs:132

🧵️ The plugin-agnostic job seam `job_get`/`job_cancel` read/act through, and every job producer
(W5's inference-as-a-job, a future tessellation/export job, …) mints ids into and reports
progress/results into — see [`job_registry`] for the one process-wide instance every tool
handler and resource reader in this file shares. Cancellation is cooperative: `request_cancel`
flips `cancel_requested` (and, for a `Pending` job with no producer running yet, finishes it
immediately — there is nothing to interrupt); a `Running` job's own producer is expected to poll
[`JobRegistry::is_cancel_requested`] and call [`JobRegistry::mark_cancelled`] once it has
actually stopped. This facet never force-kills work it does not own.

<a id="source-notes-161"></a>

### 🦀️.rs:161

🆕️ Registers `job_id` `Pending` under an id the caller already minted elsewhere — the seam a
producer that ALSO wants `crate::handles::HandleTable`'s session-owned/expiring handle (for
its authorization/TTL semantics, which this registry does not provide) uses to track the SAME
id's mutable progress/result here: mint the `HandleTable` entry first with
`HandleKind::Job`/`mint_id`, then call this with that exact id.

📈️ Every producer in this crate mints through here, so binding the job to the `tools/call`
progress token currently active on this thread covers all of them by construction — no
producer has to opt in, and none can silently forget to.
