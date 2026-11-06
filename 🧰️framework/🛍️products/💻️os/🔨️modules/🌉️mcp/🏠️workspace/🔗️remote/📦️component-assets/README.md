# Design Notes

## Preserved Source Rationale

<a id="source-notes-76"></a>

### 🦀️.rs:76

🧩️ How a gateway gets the execution-target components its opens need, as static assets: from the per-user
execution-target store when it holds exactly the lease's bytes (no network at all — a restarted gateway opens every
kind it opened before with zero component bytes), else streamed from the hub over a pool of its own that the per-minute
API budget never meters, one stream at a time, verified against the lease and admitted to the store. The first open of a
plugin is therefore bound by bandwidth alone and reports its bytes as it goes.
