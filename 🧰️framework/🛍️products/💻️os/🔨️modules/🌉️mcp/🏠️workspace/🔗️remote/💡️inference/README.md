# Design Notes

## Preserved Source Rationale

<a id="source-notes-184"></a>

### 🦀️.rs:184

📖️ One hub document's own canonical `pack`/`spr` bytes — the SAME verified mount
[`Self::read_canonical_checkpoint`] takes, projected to the bytes instead of to the
checkpoint's descriptive JSON.

🧊️ This is what `artifact_open`/`artifact_snapshot` of a hub document needs. Before ticket
26/09/18 slice M8 the workspace's hub arm answered a flat `canonical artifact bodies remain
unavailable until P4-B`, so an agent bound to a real hub space could list the hub's documents
and read their descriptors but never read one — measured live on hub 7631
(`📓️m8-mcp-agent-third-participant.md` §2.3). Nothing new is fetched or trusted here: the
mount is already digest-verified against the descriptor the authenticated binding published.
