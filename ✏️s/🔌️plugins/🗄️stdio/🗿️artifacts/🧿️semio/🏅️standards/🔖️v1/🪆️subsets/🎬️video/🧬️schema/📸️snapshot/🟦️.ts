/** 🧬️ SemioVideoSnapshot — streams{kind, codec, width, height, rate, samples{pts, key, data}},
 * container-typed / payload-opaque (sample `data` is the format's own compressed bytes, never
 * decoded by this subset). Mirrors `📸️snapshot/🦀️.rs` field for field. */
export type SemioVideoStreamKind = "video" | "audio" | "subtitle";

export interface SemioRational {
  num: number;
  den: number;
}

export interface SemioVideoSample {
  pts: number;
  key: boolean;
  /** hex-encoded opaque bytes on the wire */
  data: number[];
}

export interface SemioVideoStream {
  kind: SemioVideoStreamKind;
  codec: string;
  width: number;
  height: number;
  rate: SemioRational;
  samples: SemioVideoSample[];
}

export interface SemioVideoSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ streams: SemioVideoStream[];
}
