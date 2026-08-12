/** mutation payload — mirrors `CreateFace`. */
export interface CreateFace {
  id: string;
  outerLoop: string;
  innerLoops: string[];
  surface: unknown;
  orientation: boolean;
}
