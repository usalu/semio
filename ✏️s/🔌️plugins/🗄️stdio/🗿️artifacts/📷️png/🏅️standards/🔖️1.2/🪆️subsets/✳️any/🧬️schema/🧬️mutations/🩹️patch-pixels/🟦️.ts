/** 🩹️ Bounded PNG pixel-range patch. */
export interface PatchPixelsMutation {
  readonly index: number;
  readonly removeCount: number;
  readonly pixels: ReadonlyArray<number>;
  readonly moveTo?: number;
}
