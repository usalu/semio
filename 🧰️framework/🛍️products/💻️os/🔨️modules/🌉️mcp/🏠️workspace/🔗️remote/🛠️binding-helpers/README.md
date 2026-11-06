# Design Notes

## Preserved Source Rationale

<a id="source-notes-283"></a>

### 🦀️.rs:283

🔁️ Runs one canonical pair `attempt` until it stops failing for a reason a wait cures: an authority refresh in flight
(the attempt raced a directory event: waits for it to settle, `wait_ms` in all) or a spent network byte budget (waits
through `await_budget`). Every other outcome is the attempt's own. The first `artifact_open` right after a gateway
starts used to answer "canonical pair descriptor is unavailable" while the refresh a directory event started was still
running (ticket 26/09/23, session 14c).
