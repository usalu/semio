/** 🧰️ Shared kit of the native grammar compilations: the equality check every compilation uses on records it compares.
 *
 * @see ../🟦️.ts — the native grammar run that calls these compilations
 */

/** ⚖️ Checks numeric probes against independent expectations with the protocol's fixed precision. */
export function equal(actual: readonly number[], expected: readonly number[], label: string, tolerance = 1e-4): void {
  if (actual.length !== expected.length || actual.some((n, i) => !Number.isFinite(n) || Math.abs(n - expected[i]!) > tolerance)) throw new Error(`${label}: actual=${JSON.stringify(actual)}, expected=${JSON.stringify(expected)}`);
}
