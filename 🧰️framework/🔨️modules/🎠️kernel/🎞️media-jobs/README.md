# Design Notes

## Preserved Source Rationale

<a id="source-notes-77"></a>

### 🦀️.rs:77

🔄️ The ONE rule that turns a transcript of `step-job` observations into the `Event::JobCompleted`
the host owes the guest. Every host on every renderer answers through this, and its TypeScript
twin `spawnedJobCompletion` in `🎠️kernel/🛠️spawned-job/🟦️.ts` answers identically; both drive
`🧫️fixtures/🧵️spawned-job-drive/🔣️.json`.

Observations AFTER the first terminal step are ignored rather than refused: a driver that stops
stepping the instant it sees `Done` and one that read a batch both hand over the same completion.

<a id="source-notes-610"></a>

### 🦀️.rs:610

🔪️ Slices one opened file's contents into the chunks a shell dispatches, so no single import
invocation asks the guest for a contiguous block above its own per-request ceiling.

🔤️ Sliced by UTF-8 EXTENT, not by code units: the guest measures `text.len()` in bytes, so a
slice counted in UTF-16 units would overrun the cap by up to 3× on non-ASCII text. No slice ever
splits a code point, and an empty payload still yields exactly one chunk — a picked empty file is
a real pick the guest must be told about.

TypeScript twin: `importPayloadChunks` in `🎠️kernel/🗄️file-open-import/🟦️.ts`; both drive
`🧫️fixtures/📤️file-open-import/🔣️.json`.
