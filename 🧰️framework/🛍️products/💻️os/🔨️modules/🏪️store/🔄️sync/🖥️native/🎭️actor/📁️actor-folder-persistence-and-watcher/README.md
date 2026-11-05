
## ✍️ Persists one archive to the folder binding and reports whether the folder acknowledged it. Only an

acknowledged write records `last_written_hash` and `folder_held_hash`, so the still-stale on-disk content is never
mistaken for a self-write; a refused write counts as a failure, schedules a backoff retry and puts the typed
[`SyncWriteFailure`] on the status. A folder that could not be read is never written over.

Bytes the folder is known to hold already — the last archive read or written — are not written again.

## 🔁️ Retries once due. A folder that was unreadable is re-read first. An unsaved archive is written only after the

folder was re-read and still holds what this actor last read or wrote: a folder that changed meanwhile is reported
as `externalDivergence` and never overwritten.

## 🧬️ Adopts the store's genesis: an empty binding persists it as the document's

initial snapshot with an event-free history; a bound document must share it, otherwise
the store names a different document and the mismatch is reported, never merged.

## 👁️ Re-reads the folder binding and classifies the change: append-only → `RemoteMutations`,

divergence → `DocumentArchiveReplaced`, divergence with local pending operations → `Conflict`. Self-writes
(content hash match) are ignored.

Append-only spr events decoded without HistoryOpMeta carry HLT(0,0). Ingest merges
by HLC, so a zero stamp would reorder under already-applied local edits and leave
the live snapshot at the older head. Stamp each new envelope with this actor clock.

The first answered read of an open that finds no archive, with none read or written before, announces
[`ArtifactEvent::DocumentArchiveAbsent`] once; a later empty answer only forgets the archive the folder held.

## 🔀️ A binding path with a file extension addresses one document's text blob directly

(`Text`, generalizing the deleted single-file `FileJsonStorage` beyond `.json`); an extensionless
directory path is the canonical multi-document append-only store (`EventLog`).
