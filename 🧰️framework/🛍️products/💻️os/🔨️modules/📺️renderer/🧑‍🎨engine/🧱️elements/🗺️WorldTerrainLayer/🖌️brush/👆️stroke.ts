/** 🖌️ One captured pointer's ordered authored-grid samples. */
export type TerrainBrushSample = readonly [column: number, row: number];

export const MAX_TERRAIN_STROKE_SAMPLES = 64;

const distance = (left: TerrainBrushSample, right: TerrainBrushSample): number =>
  Math.max(Math.abs(left[0] - right[0]), Math.abs(left[1] - right[1]));

type ActiveStroke = {
  readonly pointer: number;
  readonly spacing: number;
  readonly samples: TerrainBrushSample[];
  readonly release: () => void;
  stamps: number;
};

/** 🎟️ Collects one bounded gesture and exposes its samples exactly once on a valid release. */
export class TerrainBrushStroke {
  private active: ActiveStroke | undefined;

  begin(pointer: number, radius: number, sample: TerrainBrushSample, release: () => void): boolean {
    if (this.active) return false;
    this.active = { pointer, spacing: Math.max(1, Math.floor((radius + 1) / 2)), samples: [sample], stamps: 1, release };
    return true;
  }

  owns(pointer: number): boolean {
    return this.active?.pointer === pointer;
  }

  move(pointer: number, sample: TerrainBrushSample | undefined): void {
    this.record(pointer, sample, false);
  }

  finish(pointer: number, sample: TerrainBrushSample | undefined): readonly TerrainBrushSample[] | undefined {
    const stroke = this.record(pointer, sample, true);
    if (!stroke) return undefined;
    this.cancel(pointer);
    return stroke.samples;
  }

  captureLost(pointer: number, defer: (cancel: () => void) => void): void {
    const stroke = this.active;
    if (stroke?.pointer === pointer) {
      defer(() => { if (this.active === stroke) this.cancel(pointer); });
    }
  }

  cancel(pointer?: number): void {
    if (!this.active || (pointer !== undefined && this.active.pointer !== pointer)) return;
    const { release } = this.active;
    this.active = undefined;
    release();
  }

  private record(pointer: number, sample: TerrainBrushSample | undefined, final: boolean): ActiveStroke | undefined {
    const stroke = this.active;
    if (!stroke || stroke.pointer !== pointer) return undefined;
    if (!sample) {
      this.cancel(pointer);
      return undefined;
    }
    const gap = distance(stroke.samples[stroke.samples.length - 1]!, sample);
    if (gap === 0 || (!final && gap < stroke.spacing)) return stroke;
    const stamps = stroke.stamps + Math.ceil(gap / stroke.spacing);
    if (stamps > MAX_TERRAIN_STROKE_SAMPLES) {
      this.cancel(pointer);
      return undefined;
    }
    stroke.stamps = stamps;
    stroke.samples.push(sample);
    return stroke;
  }
}
