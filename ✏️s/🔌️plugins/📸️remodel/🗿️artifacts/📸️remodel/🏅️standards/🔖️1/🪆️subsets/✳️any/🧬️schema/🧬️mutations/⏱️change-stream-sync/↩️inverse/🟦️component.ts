/** ↩️ change-stream-sync inverse — a `change-stream-sync` restoring the OLD sync offset. */
export interface ChangeStreamSyncInverse {
  id: string;
  newSyncOffsetMs: number;
}
