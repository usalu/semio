# Design Notes

## Preserved Source Rationale

<a id="source-notes-24"></a>

### 🦀️.rs:24

🛡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the security property of
the whole extension-activation design — an extension must never hold a capability its host plugin
lacks. A REQUESTED grant survives only when `granted` (the parent's own already-granted set)
carries a grant of the SAME `capability` name; an ungranted request is silently dropped, never
escalated or substituted. This is an actual intersection, not "grant what was asked for": the
output is always a subset of `requested`, bounded by `granted`. Matched by capability name only
(this crate's `CapabilityGrant` is a minimal pack-codeable stand-in — see its own doc comment — so
`scope` reconciliation belongs to the real broker at integration time, not this pure function).
