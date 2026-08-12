/** ➕ `create-block` mutation payload. */
export interface CreateBlock {
  block: unknown;
  parentId?: string | null;
  index?: number | null;
}
