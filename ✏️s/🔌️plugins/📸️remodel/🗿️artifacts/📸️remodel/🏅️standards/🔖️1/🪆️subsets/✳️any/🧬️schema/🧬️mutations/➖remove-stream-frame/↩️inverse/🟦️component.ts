/** ↩️ remove-stream-frame inverse — an `add-stream-frame` restoring the captured BASE frame. */
export interface RemoveStreamFrameInverse {
  id: string;
  frame: { index: number; timestampMs: number; assetId: string };
  kind: "image-sequence" | "video";
}
