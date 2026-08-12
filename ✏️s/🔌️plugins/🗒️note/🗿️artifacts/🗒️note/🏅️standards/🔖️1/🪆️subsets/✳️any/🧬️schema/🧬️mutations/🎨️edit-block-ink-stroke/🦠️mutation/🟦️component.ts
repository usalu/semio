/** 🎨 `edit-block-ink-stroke` mutation payload. */
export interface EditBlockInkStroke {
  id: string;
  newPoints: [number, number][];
  newX: number;
  newY: number;
  newWidth: number;
  newHeight: number;
}
