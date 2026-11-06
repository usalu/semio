# Clock Stride And Job Protocol

🪜️ A step's clock, read for real only every `stride` calls. A job that checks its deadline after
every unit of work used to read the host clock just as often; inside a Wasm guest that read is a
component-model host call, and measured 2026-09-24 (ticket 26/09/23 slice G6) the `🀄️wfc` genesis
solve made 6.7 million of them in one `inference_run`. The stride is recalibrated at every real
read so reads land about [`ClockStride::TARGET_READ_INTERVAL_US`] apart, which is also the most a
deadline is overshot by. Every [`StepContext`] a driver builds carries one: the step begins at the
driver's own entry reading ([`ClockStride::begin`]), so a new step costs no read of its own, and the
calibrated stride carries from step to step. Law: `🧫️fixtures/🪜️clock-stride-law.json`.

📸️ A pause point where work is resumable but not yet committed — `state` is opaque, pack-encoded
(or, for a dependency-free job like [`TortureJob`], hand-rolled little-endian) bytes the job alone
interprets; `applied_progress` is the Puzzle 3D `FillBuilder.applied_count` pattern generalized: how
much of `state` is COMMITTED versus merely planned, so a caller can show "these N are done" without
decoding `state` itself.

🧵️ Whether a job may be handed to another thread. Every threaded target elaborates this to
`Send`, so `J: InteractiveJob` still proves `J: Send` for [`WorkerJobSessionInner`]'s
`unsafe impl` and every pool submission. The browser build (`wasm32`, not `wasip2`) has no
second thread to hand a job to at all — its jobs own `Rc<JsValue>` browser handles by design
(`🎯️targets/🧊️wgpu/🧵️frame-job`'s frame worker is the single JS worker thread) — so requiring
`Send` there would only forbid the one ownership model that target can have.

🧩️ The protocol every interactive operation implements instead of a run-to-completion function
call — see the module doc's governing rule. `step` is bounded (checks [`StepContext::should_yield`]
and returns before the hard ceiling), cancellable ([`StepContext::is_cancelled`]) and explicitly
resumable (a fresh [`StepContext`] each call, job-owned state carries everything between calls).


## Protocol details

🏁️ Begins a step at `start_us`, the reading its driver made at entry: the step's first
`stride - 1` reads answer it, and the calibrated stride is kept.

⏱️ The clock as of the last real read of `clock`, which is at most one stride of calls old.

📖️ How many calls the next real read is apart — the law's observable.

🖥️ Terminal success payload: the job's final persisted `state` plus its `output` — both opaque
bytes, so the runtime stays completely job-agnostic (design doc Decision 2).

💥️ Opaque, job-specific error payload — never interpreted by the runtime, same reasoning as
[`CommitCandidate`]'s fields.

☎️ What one [`InteractiveJob::step`] call reports. [`StepOutcome::Yield`]/[`StepOutcome::PreviewReady`]/
[`StepOutcome::CheckpointReady`] all mean "call `step` again"; [`StepOutcome::is_terminal`] marks
the other three.

📊️ Keeps every admitted turn's physical and logical receipt, including terminal release.

🛡️ Checks independent authority and the original owner's terminal witness before publication.

🎟️ Consumes physical and work effects once under the same original caller; successful progress reports that debit.

📦️ Requests the actual natural frontier allocation for this bounded payload grant.

♻️ Requests complete physical backing release independently of payload copying.

📪️ Requests the exact original frontier depth without enlarging the caller's limit.

🔎️ Refuses undeclared retained demand instead of guessing a zero-credit owner.

🔔️ Registers the wake source that can advance a genuinely blocked close.


## Overview

🚦️ Clock stride and job protocol: the clock stride, the checkpoint, commit candidate, fault and step outcome records, the close step, the thread transfer traits and the interactive job trait. See [`super`].

## Return Cancellation Source Notes

🪜️ Original clock stride; see `README.md`.

🕰️ The last reading, without reading the clock.

📸️ Original checkpoint; see `README.md`.

🧵️ Job thread ownership; see `README.md`.

💼️ Browser twin of the marker above — see its docstring.

🧩️ Interactive job protocol; see `README.md`.

🧮️ Requests the next original payload work without granting it.
