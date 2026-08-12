/** 🌱 create-stream mutation payload — brings a new media stream into existence. */
export interface CreateStream {
  stream: MediaStream;
}

export interface MediaStream {
  id: string;
  name: string;
  kind: "image-sequence" | "video";
  cameraId?: string;
  syncOffsetMs: number;
  fpsHint: number;
  frames: FrameRef[];
  source?: VideoSource;
}

export interface FrameRef {
  index: number;
  timestampMs: number;
  assetId: string;
}

export interface VideoSource {
  name: string;
  container: string;
  codec: "avc" | "hevc" | "vp9" | "av1" | "mjpeg" | "unknown";
  durationMs: number;
  frameCount: number;
  width: number;
  height: number;
}
