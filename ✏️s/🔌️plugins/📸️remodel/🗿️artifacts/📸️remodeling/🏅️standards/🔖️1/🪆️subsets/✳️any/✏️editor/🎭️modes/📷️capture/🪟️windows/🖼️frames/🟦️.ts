/** 🖼️ Remodeling editor — Frames window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(scene: &RemodelingSnapshot, config: &RemodelingConfig)` boundary — the cursored frame
 * image (as a data URL) plus every ground-control-point observation planted on it, as point
 * markers (`frames_layers_json`'s own layer union). The read-only viewer has no mutation-capable
 * twin for this window (the model window is the only one the viewer ports today). */

/** 🖼️ The cursored frame image, decoded straight from the stored `ImageAsset`. */
export interface RemodelingFrameImageLayer {
  type: "image";
  assetId: string;
  dataUrl: string;
  width: number;
  height: number;
}

/** 📍️ Every GCP observation planted on the cursored frame, as point markers. */
export interface RemodelingFramePointsLayer {
  type: "points";
  id: "remodeling-gcp-observations";
  points: Array<{ x: number; y: number; label: string }>;
}

export type RemodelingFrameLayer = RemodelingFrameImageLayer | RemodelingFramePointsLayer;

/** 🎞️ Which frame is cursored — mirrors Rust `RemodelingFrameCursor`. */
export interface RemodelingFrameCursor {
  streamId: string | null;
  frameIndex: number;
}

/** 🖼️ The Frames window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface RemodelingFramesViewModel {
  windowKindId: "remodeling-frames";
  bodyKey: "remodeling.play.frames";
  surfaceId: "remodeling.play.frames";
  frameCursor: RemodelingFrameCursor;
  layers: RemodelingFrameLayer[];
}

export const REMODELING_PLAY_WINDOW_FRAMES = "remodeling-frames" as const;
export const REMODELING_PLAY_BODY_FRAMES = "remodeling.play.frames" as const;
