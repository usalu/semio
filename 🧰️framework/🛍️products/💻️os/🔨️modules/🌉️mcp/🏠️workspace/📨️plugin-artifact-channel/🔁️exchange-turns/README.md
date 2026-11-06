# Design Notes

## Preserved Source Rationale

<a id="source-notes-44"></a>

### 🦀️.rs:44

🔁️ ONE guest turn, or one bounded host-side bookkeeping step, of the command above. Returns
[`CommandTurn::MoreWork`] when the command genuinely owes another one — the guest has more
retained pages to read, or a fixed-capacity registry is mid-teardown — and a `Fault` only
when the command is over. The two used to be the same value: every "call me again" was minted
as a `budget.exceeded` fault and the loop above re-entered on the fault's CODE, so a real
budget failure and normal paging were indistinguishable both to the loop and to the agent
reading the message.

📄️ One retained page per turn. Under a compiled runtime every turn completes, so the
cursor always advances here; when the driver has no page left the guest is woken instead,
which is how it gets to publish its terminal acknowledgement.

⏱️ The suspension window is "while this guest turn runs, nobody touches the driver", so
it is closed on every arm, not just the `Ok` one: leaving the driver `Suspended` (and
linked for close) made the next turn read `!is_active`, tear the whole command down and
re-send it under a BRAND NEW `seq` while the guest still retained the owner of the
first attempt — the whole of `plugin.command-cursor-mismatch: owner is 1, the retained
owner's is 554` (ticket 26/09/18 slice A2 §6.1). The instance itself is gone either
way: a turn cut by its ceiling leaves the component mid-call and unusable.

🧾️ The guest's OWN reason for refusing the ingress rides in `command_ingress`, and
the cleanup below takes further turns — so it is captured on the exchange the
moment it is seen, not read again at the terminal where it no longer exists. It
used to be dropped entirely, and "faulted for seq N after bounded exact-owner
cleanup" told the agent (and every earlier slice of this ticket) nothing at all.

🧹️ An exchange left behind by an ABANDONED command — its wall budget ran out, or its guest turn faulted and the
instance was discarded — is closed step by step before the next command on the instance is admitted. It used to
stay active, and the next command's turn met it with `assert_eq!(pending.seq, seq)`: the gateway's main thread
panicked and every MCP session on it died (live hub coverage on 7800/p33, ticket 26/09/23 G12 session 14c).

<a id="source-notes-192"></a>

### 🦀️.rs:192

📬️ Admits one effect, if it is THIS command's answer.

🧭️ **A command's answer is not an `Effect::Respond`.** The reactor publishes `Respond` for
exactly one inbound event — `Event::Request`, the extension-capability seam
(`⚛️reactor/🔄️turn/🦀️.rs`'s `inbound_request_effects`) — which this gateway never sends. A
command's answer goes through `route_app_frame`, which addresses every non-`UiPatch`
`AppFrame` to `Effect::SendMessage{target: Shell{instance}}` as encoded frame bytes,
correlated by the frame's own `in_reply_to` against the COMMAND's `seq`. The React host reads
exactly this lane (`🔌️PluginRuntime/🟦️.tsx`'s `shellFrameBytes`, "a `Shell{instance}`
send-message IS an `AppFrame` reply"), which is why mutation verbs have always worked there
and never here.

🐛️ Reading the respond lane meant the answer was *sitting in the same turn's effects*,
unread: the guest drove ingress to `Complete`, published its frames, and went `Idle` — and the
host reported "the guest acknowledged command seq 1 and then went idle without publishing a
response for it" (ticket 26/09/18, WR1 §5.4 → WR2; measured: the acknowledging turn published
`sendMessage(shell:0, Document(in_reply_to 1), 579 B)`).

🏁️ `AppFrame::Done` naming this seq is a reply STAMP, not an answer of its own. Every
transaction route publishes its terminal frame AND a `Done` on the same turn
(`🔌️plugin/🦀️.rs`'s `TransactionPrepare`/`Commit`/`Rollback` arms each push both), so
admitting the second one reported "the guest published more than one frame answering seq
N" for a guest that answered exactly once — measured 2026-09-20 the first time a real
`TransactionPrepare` crossed the headless ingress. A `Done` still IS the answer for the
verbs that publish nothing else (`LoadDocumentArchive`, `TransactionUndo`/`Redo`), which is why
it is dropped only once a real frame is already held. A genuine duplicate — two answering
frames neither of which is a stamp — stays as loud as WR2 made it.

<a id="source-notes-234"></a>

### 🦀️.rs:234

📮️ Wakes the guest until it publishes the frame answering `seq`, once its command ingress has
gone terminal.

🧭️ Acknowledging a command and ANSWERING it are two different events on this wire.
`CommandBatchProgress::Complete` means the guest consumed every page and acknowledged the
batch; the answer is a frame it publishes when the work is done — usually on the very turn
that acknowledges, sometimes on a later one. Bounded by [`COMMAND_RESUME_WALL_BUDGET`].

🏁️ A settled command that published no frame naming itself is **not** a fault: it is a
command whose verb has nothing to say, and the answer is a stamped `AppFrame::Done` — the
same rule the React host applies (`commandIngressNeedsReplyStampV1`). Making it a fault here
would refuse every archive admission/`TransactionUndo` the guest completed silently. What stays
a fault is a guest that answers ANOTHER request, because that is a routing defect.

🛑️ A guest that is `Idle` with no wake pending has nothing left to do, so waiting the
rest of the wall is pointless. A `Respond` under a request id this gateway never sent
is the one shape that is still a defect worth naming.
