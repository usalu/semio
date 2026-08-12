/** 🔁 replace-stream-source mutation payload — whole-value swap of a stream's video provenance. */
export interface ReplaceStreamSource {
  id: string;
  source?: {
    name: string;
    container: string;
    codec: "avc" | "hevc" | "vp9" | "av1" | "mjpeg" | "unknown";
    durationMs: number;
    frameCount: number;
    width: number;
    height: number;
  };
}
