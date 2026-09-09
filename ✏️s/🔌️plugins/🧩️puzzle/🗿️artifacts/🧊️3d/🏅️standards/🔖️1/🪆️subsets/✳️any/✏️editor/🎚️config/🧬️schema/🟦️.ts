export interface Puzzle3dConfig {
  /** @state config */ fillCount: number;
  /** @state config */ overlapBudget: number;
  /** @state config */ objectKindWeights: Record<string, number>;
  /** @state config */ vortexKindWeights: Record<string, number>;
}
export class Puzzle3dConfigGuardRefusal extends Error { constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); } }
const record = (value: unknown, at: string): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Puzzle3dConfigGuardRefusal(at, "value is not an object");
  return value as Record<string, unknown>;
};
const number = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Puzzle3dConfigGuardRefusal(at, "value is not a finite number");
  return value;
};
const weights = (value: unknown, at: string): Record<string, number> => Object.fromEntries(Object.entries(record(value, at)).map(([key, item]) => [key, number(item, `${at}.${key}`)]));
export function parsePuzzle3dConfig(value: unknown, at = "$"): Puzzle3dConfig {
  const row = record(value, at);
  const fillCount = number(row.fillCount, `${at}.fillCount`);
  if (!Number.isSafeInteger(fillCount) || fillCount < 0) throw new Puzzle3dConfigGuardRefusal(`${at}.fillCount`, "value is not an unsigned integer");
  return { fillCount, overlapBudget: number(row.overlapBudget, `${at}.overlapBudget`), objectKindWeights: weights(row.objectKindWeights, `${at}.objectKindWeights`), vortexKindWeights: weights(row.vortexKindWeights, `${at}.vortexKindWeights`) };
}
