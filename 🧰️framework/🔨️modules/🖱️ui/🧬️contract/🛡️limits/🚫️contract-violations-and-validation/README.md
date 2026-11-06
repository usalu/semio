# Design Notes

## Preserved Source Rationale

<a id="source-notes-92"></a>

### 🦀️.rs:92

🌲️ Validates `snapshot` against `limits`, collecting every [`UiContractViolation`] found rather
than stopping at the first — a fuzz corpus or a UI diagnostic wants the whole picture. Returns
`Ok(())` only when the document is entirely clean. Short-circuits to just [`UiContractViolation::NodeQuota`]
when the node count alone already exceeds the quota, since walking an already-oversized untrusted
document is itself part of the attack surface `max_nodes` exists to cut off.
🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
