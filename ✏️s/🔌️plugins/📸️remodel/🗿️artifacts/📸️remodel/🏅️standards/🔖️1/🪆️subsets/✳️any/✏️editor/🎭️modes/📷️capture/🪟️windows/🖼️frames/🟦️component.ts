/** 🖼️ Remodel editor — Frames window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * pane's `render(scene: &RemodelSnapshot, config: &RemodelConfig)` boundary — the cursored frame
 * image (as a data URL) plus every ground-control-point observation planted on it, as point
 * markers (`frames_layers_json`'s own layer union). The read-only viewer has no mutation-capable
 * twin for this window (the model window is the only one the viewer ports today). */

/** 🖼️ The cursored frame image, decoded straight from the stored `ImageAsset`. */
export interface RemodelFrameImageLayer {
  type: "image";
  assetId: string;
  dataUrl: string;
  width: number;
  height: number;
}

/** 📍️ Every GCP observation planted on the cursored frame, as point markers. */
export interface RemodelFramePointsLayer {
  type: "points";
  id: "remodel-gcp-observations";
  points: Array<{ x: number; y: number; label: string }>;
}

export type RemodelFrameLayer = RemodelFrameImageLayer | RemodelFramePointsLayer;

/** 🎞️ Which frame is cursored — mirrors Rust `RemodelFrameCursor`. */
export interface RemodelFrameCursor {
  streamId: string | null;
  frameIndex: number;
}

/** 🖼️ The Frames window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface RemodelFramesViewModel {
  windowKindId: "remodel-frames";
  bodyKey: "remodel.play.frames";
  surfaceId: "remodel.play.frames";
  frameCursor: RemodelFrameCursor;
  layers: RemodelFrameLayer[];
}

export const REMODEL_PLAY_WINDOW_FRAMES = "remodel-frames" as const;
export const REMODEL_PLAY_BODY_FRAMES = "remodel.play.frames" as const;
