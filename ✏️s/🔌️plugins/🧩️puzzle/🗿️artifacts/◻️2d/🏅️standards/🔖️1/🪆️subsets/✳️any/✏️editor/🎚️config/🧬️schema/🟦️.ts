/** 🧬️ Shared Puzzle 2D generator preferences and the fill tool's requested count. */
export interface Puzzle2dConfig {
  /** @state config */
  nodeKindWeights: Record<string, number>;
  /** @state config */
  handleKindWeights: Record<string, number>;
  /** @state config */
  fillCount: number;
}

export class Puzzle2dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}

const record = (value: unknown, at: string): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Puzzle2dConfigGuardRefusal(at, "value is not an object");
  return value as Record<string, unknown>;
};

const weights = (value: unknown, at: string): Record<string, number> => {
  const row = record(value, at);
  return Object.fromEntries(Object.entries(row).map(([key, item]) => {
    if (typeof item !== "number" || !Number.isFinite(item)) throw new Puzzle2dConfigGuardRefusal(`${at}.${key}`, "value is not a finite number");
    return [key, item];
  }));
};

const count = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > 4294967295) throw new Puzzle2dConfigGuardRefusal(at, "value is not a u32 count");
  return value;
};

export function parsePuzzle2dConfig(value: unknown, at = "$"): Puzzle2dConfig {
  const row = record(value, at);
  return {
    nodeKindWeights: weights(row.nodeKindWeights, `${at}.nodeKindWeights`),
    handleKindWeights: weights(row.handleKindWeights, `${at}.handleKindWeights`),
    fillCount: count(row.fillCount, `${at}.fillCount`),
  };
}
