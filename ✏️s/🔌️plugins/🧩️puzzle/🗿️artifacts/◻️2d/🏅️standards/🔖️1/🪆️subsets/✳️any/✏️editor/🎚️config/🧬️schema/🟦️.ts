/** 🧬️ Shared Puzzle 2D generator preferences. */
export interface Puzzle2dConfig {
  /** @state config */
  nodeKindWeights: Record<string, number>;
  /** @state config */
  handleKindWeights: Record<string, number>;
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

export function parsePuzzle2dConfig(value: unknown, at = "$"): Puzzle2dConfig {
  const row = record(value, at);
  return {
    nodeKindWeights: weights(row.nodeKindWeights, `${at}.nodeKindWeights`),
    handleKindWeights: weights(row.handleKindWeights, `${at}.handleKindWeights`),
  };
}
