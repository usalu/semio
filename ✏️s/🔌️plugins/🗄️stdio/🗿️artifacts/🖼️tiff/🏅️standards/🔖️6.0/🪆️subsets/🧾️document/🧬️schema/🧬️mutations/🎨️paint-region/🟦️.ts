/** 🎨️ Revision-guarded tiled TIFF region paint. */
export interface PaintRegionMutation {
  readonly revision: string;
  readonly ifdIndex: number;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly red: number;
  readonly green: number;
  readonly blue: number;
  readonly alpha: number;
}
