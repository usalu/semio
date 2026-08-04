# 🎬️ Document actor fixtures

Scripted test vectors for the schema-agnostic document sync actor. The same JSON files are replayed
by two harnesses to keep the Rust actor and its TypeScript twin (WS-E `🟦️backbone-worker.ts`) in lockstep:

- **Rust** — `store/sync/rs/lib.rs` `tests::actor_tests::fixtures_replay_matches_expected_events`
  drives a real `DocumentHost` over a temp folder binding.
- **TypeScript** (later, WS-E) — a vitest harness replays the same files against the TS backbone worker.

## Format (`ActorFixture`)

```jsonc
{
 "name": "basic-remote-operations",
 "schema": "demo/v1",
 "documentId": "fixture-basic",
 "inbound": [
  /* FixtureInbound[] — stimuli applied in order */
 ],
 "expectedEvents": [
  /* DocumentEvent variant tags, in order */
 ],
 "expectedEditIds": [
  /* edit ids expected in the timeline after replay */
 ],
}
```

`inbound` and `expectedEvents` pair 1:1 in the Rust harness: each stimulus is applied, then its paired
event is awaited before the next stimulus (removes write/poke races). Interleaved `status` events are
tolerated (skipped) while matching the expected tag.

### `FixtureInbound` variants (`kind`)

- `externalEdits { edits: Edit[] }` — append these `Edit` JSON objects to the stored envelope's
  `vcs.edits` out-of-band (append-only → `remoteOperations`).
- `replaceEnvelope { envelope: DocumentEnvelope }` — rewrite the whole stored envelope (divergent
  history → `snapshotReplaced`, or `conflict` when local operations are pending).
- `hubFrame { frameBytes: number[] }` — a raw hub server frame's encoded bytes (`protocol_wire`'s
  binary `ServerFrame` codec output, lane byte included — see `store/sync/fixtures/wire/`'s own
  fixtures for the codec itself). Driven by the TS twin; the folder-only Rust harness rejects
  these, so keep them out of Rust-replayed fixtures for now.

### `DocumentEvent` tags

`remoteOperations` · `snapshotReplaced` · `status` · `presence` · `conflict`
