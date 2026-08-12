/** mutation payload — mirrors `CreateVertex`. */
export interface CreateVertex {
  id: string;
  point: { x: number; y: number; z: number };
}
