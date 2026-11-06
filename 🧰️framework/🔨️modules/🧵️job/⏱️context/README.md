# Original Job Context

🧭️ Everything one [`InteractiveJob::step`] call needs: identity ([`OperationId`]/[`Generation`]),
the two-bound budget, cancellation, the clock, and the running preview-sequence cursor. Fields are
private with accessor methods (a deliberate narrowing from the design doc's Decision 1 sketch,
which exposed `pub fuel: &mut u64`/`pub cancel: CancelToken` directly) so [`StepContext::is_cancelled`]
can own the the canonical cancellation observation seam in exactly one place instead of every job reimplementing it.

🎨️ Whether this step's [`CancelToken`] (or an ancestor's) is cancelled — checked via a single
non-blocking the canonical cancellation observation, see the module doc. A job MUST check this on entry and after
every bounded unit of work (design doc Decision 6): return [`StepOutcome::Cancelled`] without
doing further work once true.

👣️ Records a `semio_framework_trace::StageChanged` event and updates [`StepContext::stage`] —
the job's own instrumentation call for switching between internal lanes/phases (Puzzle 3D's
brush → fill switch is the template). Terminal per-call events (preview/checkpoint/commit/
cancel/fail) are recorded once by [`drive_step`] from the returned [`StepOutcome`] instead —
see the module doc's "trace, not a second instrumentation layer" section.

🕰️ The event is stamped with the step's latest reading ([`StepContext::latest_us`]), at most
one stride old, instead of a read of its own.

▶️ Runs exactly one [`InteractiveJob::step`] call under a [`Watchdog`] (so an 8 ms-plus step is
ALWAYS caught — never eyeballed, see this ticket's exit gate), pre-checks cancellation so an
already-cancelled operation never even enters the job, and is the ONE place a returned
[`StepOutcome`] becomes a `semio_framework_trace::record_*` call (module doc). `site` is the
`&'static str` label `Watchdog`/the trace ring key on; `stage` is which [`InteractiveStage`]
contract family this call belongs to (mirrors the caller's `semio_framework_async::Lane`, kept a
separate parameter rather than converted from `Lane` since this crate must not depend on the actor
crate's lane-to-stage mapping). `preview_sequence` is threaded across an entire run — see
[`StepContext::next_preview_sequence`].


## Original Loans

🛬️ Captures the owning operation's original decoder without creating replacement capacity or cancellation authority.

🫴️ Loans original cancellation, clock, independent remaining credits and the exact shared construction receipt.

♻️ Loans retirement-only authority after cancellation while retaining the original deadline, fuel and physical credits.

🧾️ Debits the enclosing semantic receipt once after its original construction child already accepted physical spend.

📩️ Closes one receiving scope whose enclosing semantic receipt equals its accepted physical spend: no further debit, no open obligation.

⏳️ Narrows actual CPU and absolute deadline on the same original context, preserving clock observations and charging actual child work even through unwind.

📹️ Records actual child effects in the external recipient before refusing exceeded authority.

🛑️ Borrows the admitted external cancellation for native controls without an alias or context replacement.


## Source Context Accessors

🤝️ Borrows the original paid ledger and cancellation while preserving the caller's exact recipient.

🎟️ Preserves the original caller depth and independently unspent physical credits.

🪙️ Narrows the same original recipient and restores ceilings only while its initiating context remains intact.

📊️ Exposes actual effects recorded by this same original step context, including failed turns.

✅️ Checks the original cumulative recipient against the issued turn grant.

🪪️ Identifies the actual external recipient without creating a replacement or alias.

🔑️ Borrows only the equality identity of this turn's live original payload source.

🕰️ Borrows the caller's original clock source for a retained child authority.

⏰️ Returns the same monotonic observation state after one worker turn.

🕰️ Advances only the observed clock after the actual callback returns.

📖️ Reads the original supplied source turn without deriving it from trace sequences.

🕐️ Reads the opaque clock authority supplied by this actual retained source.

💼️ Quotes the same source's total remaining CPU while its semantic work loan stays narrowed.

⌛️ The supplied source deadline remains immutable while a semantic view may yield sooner.

💲️ Quotes total original remaining currencies without changing the active semantic loan.

🔏️ Binds one opaque source clock label without changing its actual callback or absolute deadline.

🪢️ The genuine parent issuer binds one input epoch before children borrow this context.

✖️ Borrows this caller's original cancellation witness without issuing another alias.

🏡️ Borrows the genuine supplier parent for its source lifetime without creating another strong loan.

📸️ Borrows the genuine external ledger and cancellation roots without creating authority.

❌️ Issues one alias only after this original context admits its metadata item and depth.

💰️ Raw work remains visible even when a returned guest exceeds the caller's finite wallet.

🧾️ Borrows only the separately claimed original return receipt from this current callback.

⛽️ Reports actual return CPU funded by a prior original claim, independently of current semantic fuel.

🚩️ Observes the initiating supplier cancellation without spending an outstanding original loan.


## Additional Context APIs

🌐️ Admits actor Core-wire return delivery on the actual original Source.

🌱️ Admits the actual Source return escrow table under its paid original budget.

🪪️ Reads the installed original decoder continuation without creating a decoder or borrowing construction credit.

🌉️ Runs one receiving step; see `🧪️testing/README.md`.

🏷️ The label passed to the most recent [`StepContext::set_stage`] call (`"initial"` before the

⏱️ The step's clock, read for real only every [`ClockStride`] calls — see there. A job may

call this after every unit of work; inside a Wasm guest each real read is a host call.

🪜️ The latest reading this step holds, without reading the clock.

⛽️ Decrements the remaining fuel by `units`, saturating at zero — a job calls this after doing

🚦️ Whether the job must return NOW (before the hard 8 ms ceiling) — either bound crossed.

🧹️ Physical close spends the same finite source even when its semantic deadline expired.

🛂️ Reports the authenticated original Source retirement loan for restricted cleanup ports.

🛂️ Only a genuine original source can lend close permission; semantic yield remains unchanged.

👶️ A clone of this step's [`CancelToken`] — `Arc`-cheap — for a job that wants to derive a

child scope (see [`JobScope::child_of`]) or hand the token to work it submits elsewhere.

🔢️ The next preview-sequence number for this operation, advancing a cursor that survives

across every [`StepContext`] built for the same retained session — one call per

🔢️ Reads the original caller cursor without admitting another preview.

🔁️ Opens the next fixture turn; see `🧪️testing/README.md`.

🔁️ Opens the next Source turn; see `🧪️testing/README.md`.

🪙️ Reads the genuine parent's currently borrowable source ledger.

🔐️ Observes the genuine gate using its actual nonblocking borrow.

🪪️ Checks the live defining claim without issuing another credit.


## Further API Details

🧭️ Original step authority; see `README.md`.

📦️ Births the original Core-wire capsule under its Source budget.

🚪️ Quotes normal work; see `🧪️testing/README.md`.

🔬️ White-box Source laws; see `🧪️testing/README.md`.


## Context Overview

StepContext retains the initiating clock, absolute deadline, cancellation, recipient and funded stock.

Results borrow their paid payload source through descriptors and close through exact physical receipts.

Core owns the private payload/context/retirement knot; worker, scope, registry and progress have distinct owners.


## Return Cancellation Source Notes

📤️ Publishes the actual Source-owned return escrow.

🛬️ Original loan; see `README.md`.

🫴️ Original loan; see `README.md`.

♻️ Original loan; see `README.md`.

🧾️ Original loan; see `README.md`.

📩️ Original loan; see `README.md`.

🤝️ Source context; see `README.md`.

🎟️ Source context; see `README.md`.

🪙️ Source context; see `README.md`.

⏳️ Original loan; see `README.md`.

📹️ Original loan; see `README.md`.

📊️ Source context; see `README.md`.

✅️ Source context; see `README.md`.

🪪️ Source context; see `README.md`.

🔑️ Source context; see `README.md`.

🕰️ Source context; see `README.md`.

⏰️ Source context; see `README.md`.

🕰️ Source context; see `README.md`.

📖️ Source context; see `README.md`.

🕐️ Source context; see `README.md`.

💼️ Source context; see `README.md`.

⌛️ Source context; see `README.md`.

💲️ Source context; see `README.md`.

🔏️ Source context; see `README.md`.

🪢️ Source context; see `README.md`.

✖️ Source context; see `README.md`.

🛑️ Original loan; see `README.md`.

🏡️ Source context; see `README.md`.

📸️ Source context; see `README.md`.

❌️ Source context; see `README.md`.

💰️ Source context; see `README.md`.

🧾️ Source context; see `README.md`.

⛽️ Source context; see `README.md`.

`units` worth of its own work, never before.

🎨️ Reads cancellation; see `README.md`.

🚩️ Source context; see `README.md`.

👣️ Records the stage; see `README.md`.

▶️ Runs one watched step; see `README.md`.

📊️ See `🧪️testing/README.md`.

🧪️ See `🧪️testing/README.md`.

🧪️ See `🧪️testing/README.md`.

👶️ See `🧪️testing/README.md`.

🪢️ See `🧪️testing/README.md`.

🧪️ See `🧪️testing/README.md`.

🧪️ See `🧪️testing/README.md`.

🧪️ See `🧪️testing/README.md`.

📊️ See `🧪️testing/README.md`.

🪙️ See `🧪️testing/README.md`.

🎟️ See `🧪️testing/README.md`.

🎟️ See `🧪️testing/README.md`.

🧪️ See `🧪️testing/README.md`.

🧪️ See `🧪️testing/README.md`.

🚫️ See `🧪️testing/README.md`.
