/** 🪣️ Composite-window option — the `paintBucket` utility's tolerance slider and foreground. Typed twin of `🦀️.rs`'s
 * `measure(config: &RasterConfig) -> WindowMeasure` — the values it reads off `RasterConfig.fillTolerance`/`.brushColor`,
 * not a persisted struct of its own. */
export interface RasterBucketOptions {
  tolerance: number;
  color: string;
}

export const RASTER_PLAY_BUCKET_OPTIONS_GROUP_ID = "raster-utility-options-paintBucket" as const;
