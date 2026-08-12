/** 🔺 change-stream-sync diff — populates `RemodelDiff.streams` with the full patched stream list. */
export interface ChangeStreamSyncDiff {
  streams: { values: unknown[] };
}
