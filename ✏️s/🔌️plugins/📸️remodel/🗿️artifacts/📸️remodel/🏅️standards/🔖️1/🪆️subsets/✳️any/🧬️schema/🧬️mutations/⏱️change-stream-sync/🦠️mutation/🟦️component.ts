/** ⏱ change-stream-sync mutation payload — sets one media stream's sync offset. */
export interface ChangeStreamSync {
  id: string;
  newSyncOffsetMs: number;
}
