/** 🧬️ Shared Puzzle 5D generator preferences. */
export interface Puzzle5dConfig {
  /** @state config */
  overlapBudget: number;
  /** @state config */
  objectKindWeights: Record<string, number>;
  /** @state config */
  vortexKindWeights: Record<string, number>;
}

export class Puzzle5dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}

const record = (value: unknown, at: string): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Puzzle5dConfigGuardRefusal(at, "value is not an object");
  return value as Record<string, unknown>;
};

const finite = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Puzzle5dConfigGuardRefusal(at, "value is not a finite number");
  return value;
};

const weights = (value: unknown, at: string): Record<string, number> => Object.fromEntries(Object.entries(record(value, at)).map(([key, item]) => [key, finite(item, `${at}.${key}`)]));

export function parsePuzzle5dConfig(value: unknown, at = "$"): Puzzle5dConfig {
  const row = record(value, at);
  return { overlapBudget: finite(row.overlapBudget, `${at}.overlapBudget`), objectKindWeights: weights(row.objectKindWeights, `${at}.objectKindWeights`), vortexKindWeights: weights(row.vortexKindWeights, `${at}.vortexKindWeights`) };
}
