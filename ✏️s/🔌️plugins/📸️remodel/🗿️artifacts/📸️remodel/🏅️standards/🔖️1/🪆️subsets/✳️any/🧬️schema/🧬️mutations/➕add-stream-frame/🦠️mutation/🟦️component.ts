/** ➕ add-stream-frame mutation payload — appends one frame to an existing media stream. */
export interface AddStreamFrame {
  id: string;
  frame: { index: number; timestampMs: number; assetId: string };
  kind: "image-sequence" | "video";
}
