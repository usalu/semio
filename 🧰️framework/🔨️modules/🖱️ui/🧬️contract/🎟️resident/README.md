# Design Notes

## Preserved Source Rationale

<a id="source-notes-17"></a>

### 🦀️.rs:17

💰️ The process-wide byte budget, derived from sixty-four full-document record baselines.
Static backing and each reservation's additional owners consume this budget too, so its byte
ceiling and the slot ceiling remain independent.

🐛️ ticket 26/09/09/PROCEDURAL-3D-END-TO-END lane `react-example-switch-regression`: this was
`4 * UI_RESIDENT_SURFACE_BYTES` — four times the per-surface CEILING, the imagined concurrency of a
four-surface session — while [`UI_RESIDENT_SLOTS`] admits sixty-four holders at once and the React
generation3d editor peaks at TWENTY-SIX, `33 483 491` of `33 554 432` bytes
(`🗑️generated/example-switch-census/console.txt`). The fifth example switch asked the flow window to
grow to `2 517 394` bytes, `5 549` were left, and the refusal is a TERMINAL
`SurfaceReconcileFault::ResidentCredit`: `window:procedural-main` never published a graph again and
fourteen further switches rendered the same stale example. Wave B58 already established that the
per-surface ceiling is "a maximum a surface may reach, never a price" — deriving the AGGREGATE from
that ceiling repeated the same category error one level up.

<a id="source-notes-235"></a>

### 🦀️.rs:235

⚖️ One live reservation moves to new limits in place — up or down — so a holder priced from what it
has actually used can climb toward the per-surface ceiling instead of reserving that ceiling up front,
and can hand unused credit back when its census completes.

🐛️ A reconcile reservation asked for [`UI_RESIDENT_SURFACE_BYTES`] — the per-surface MAXIMUM — of a
[`UI_RESIDENT_AGGREGATE_BYTES`] budget four times its size, so exactly THREE surfaces could hold one
process-wide while a single app session mounts thirteen; every further surface was refused, deferred
and re-dirtied for ever (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56 §6, wave B58). An increase the
aggregate cannot take is refused [`UiResidentFault::Capacity`] — the same named refusal a first
reservation answers — so an over-budget aggregate can never become a silent defer loop. The exact
ownership of the whole reservation is required in either direction, so a reservation whose output
obligation has already been split off can no longer be repriced.
