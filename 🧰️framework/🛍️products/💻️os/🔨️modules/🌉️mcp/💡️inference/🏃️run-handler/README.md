# Design Notes

## Preserved Source Rationale

<a id="source-notes-89"></a>

### 🦀️.rs:89

💡️ Runs one declared inference to completion in a single call — the synchronous sibling of
`inference_submit`, resolved through the same service model. A guest service runs here, in its
plugin's guest; a hub-executed service is submitted to the hub and its events are followed to a
terminal page. Nothing is committed: an offered proposal is returned, never applied.

🎫️ The job is minted BEFORE the contract is checked, and that ordering is load-bearing:
`JobRegistry::begin` binds the id to this call's `_meta.progressToken`, so every step from
here on — including a REFUSAL — is pushed to the client as `notifications/progress` and is
readable afterwards with `job_get`/`job_cancel`.
