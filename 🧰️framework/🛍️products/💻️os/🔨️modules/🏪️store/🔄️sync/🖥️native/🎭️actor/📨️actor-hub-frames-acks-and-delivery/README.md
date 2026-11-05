
## 🪙️ Command-lane credit-based flow control: no client-side backpressure

implemented this wave (scope is frame plumbing, not congestion control) —
accepted and ignored.

## 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage: `Accepted`

just clears the pending batch; `Transformed`/`Rejected` both roll back the speculative
local head first (via {@link rollback_envelope}, replayed as remote operations), and
`Transformed` then delivers the semio_hub's replacement envelope the same way.

## 🗃️ Publishes a replaced recursive archive. An external rewrite that LOST events cannot be

expressed as new events, so it is a document replacement: the host reloads the store from
the archive by replaying its events — nothing is pushed into (or merged by) the live store.

## 🚫️async: E1 pure sync body — `broadcast::Sender::send` never suspends. Declaring this

`async fn` forced `&ArtifactActor` to live across an await, which demanded
`ArtifactActor: Sync` from every `!Sync` field it owns (the folder watcher's `Receiver`,
`connect_future`, the codec table's `dyn Fn`) and made the whole actor turn non-`Send`.
