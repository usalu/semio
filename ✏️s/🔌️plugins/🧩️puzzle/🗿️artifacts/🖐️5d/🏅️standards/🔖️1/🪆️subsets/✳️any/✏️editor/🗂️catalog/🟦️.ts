/** ⚖️ Normalizes the authored dictionary while preserving its kind identifiers and JSON object wire shape. */
export function normalizeKindWeights(weights: Readonly<Record<string, number>>, ids: readonly string[], changed: string, value: number): Record<string, number> {
  const result: Record<string, number> = Object.create(null);
  if (ids.length === 0) return result;
  if (ids.length === 1) { result[ids[0]!] = 1; return result; }
  const requested = Math.max(0, Math.min(1, value));
  const weight = (id: string): number => Object.hasOwn(weights, id) ? weights[id]! : 0;
  const others = ids.filter(id => id !== changed);
  const remainder = Math.max(0, 1 - requested);
  const sum = others.reduce((total, id) => total + weight(id), 0);
  result[changed] = requested;
  for (const id of others) result[id] = remainder <= Number.EPSILON ? 0 : sum <= Number.EPSILON ? remainder / others.length : weight(id) / sum * remainder;
  return result;
}
