/** 🚚 `move-block-to-container` mutation payload. */
export interface MoveBlockToContainer {
  id: string;
  newParentId?: string | null;
  index: number;
}
