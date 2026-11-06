# Design Notes

## Preserved Source Rationale

<a id="source-notes-23"></a>

### 🦀️.rs:23

📬️ One command's answer, as it arrives: the guest's own encoded `AppFrame` bytes off the
`Shell{instance}` message lane.

🧭️ This used to hold a single [`semio_framework::kernel::FixedCommandPage`], because the answer
was read off `Effect::Respond` and a `RequestOutcome`'s bytes are one page. An `AppFrame` is not:
`ReadDocument` answers a whole `Document{pack, spr}`, which for any real artifact is far past
4 KiB, so that ceiling would have refused every genuine answer the moment the lane was read
correctly. The ceiling that does apply is the declared one for a single host↔guest answer,
[`semio_framework::kernel::COMMAND_MAXIMUM_BYTES`] — 8 MiB — and it is still a hard bound, so an
unbounded guest answer is a typed fault rather than an allocation.

<a id="source-notes-37"></a>

### 🦀️.rs:37

🏁️ The command settled and the guest published no frame naming it — the same reading the
React host takes (`🔌️PluginRuntime/🟦️.tsx`'s `commandIngressNeedsReplyStampV1`, which stamps
`AppFrame::Done{in_reply_to}` on exactly this shape so a caller never waits on an answer a
completed command was never going to publish). A caller that needs a real frame reports the
`Done` as the unexpected variant it is; a caller whose verb genuinely answers `Done`
(`LoadDocumentArchive`'s admission and acknowledgement, `TransactionUndo`/`Redo`) reads its own success.

<a id="source-notes-104"></a>

### 🦀️.rs:104

🪟️ What a HEADLESS agent's transaction carries of one previewed verb: its document operations and owned children, never
its config or draft lanes — those are the view state (a selection, a viewport, a mode) of a shell the agent does not
have, and the real wire `TransactionPrepare` has no lane for them (coordinator decision, ticket 26/09/23 session 14c: a
headless agent edits DOCUMENTS). Beside a document change they are omitted and the preview says so
([`VIEW_STATE_OMITTED_WARNING`]); a verb whose whole effect is view state is refused
[`crate::actions::AGENT_LANE_UNCARRIED_FAULT_CODE`] at prepare — it runs only from the shell — instead of failing at
invoke with "no wire representation" (live hub coverage on 7800/p33: procedural's add/rename/removeGeneration publish
`SetSelectedGeneration` beside their document op).
