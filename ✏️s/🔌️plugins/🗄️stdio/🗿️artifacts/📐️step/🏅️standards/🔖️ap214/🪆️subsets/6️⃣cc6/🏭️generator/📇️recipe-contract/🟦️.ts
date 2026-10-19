/** 📇️ The fixture recipe contract of the step ap214 cc6 generator. */

/** 👪️ The fixture families the corpus is sharded and reported by — never at artifact level. */
export type Family = "spatial-relationship" | "shape-complexity" | "robustness" | "mechanical" | "failure";

/**
 * 🧪️ One corpus entry. `outcome` is the DECLARED semantic class: a fixture that accepts any
 * non-crash result measures nothing, and different kernels legitimately classify exact contact
 * differently, so every entry says which answer it expects rather than discovering it.
 */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  outcome: "applied" | "no-op" | "empty" | "disjoint" | "rejected";
  tolerance: string;
  notes: string;
  build: (b: Kernel) => { operands: { role: string; shape: unknown }[]; result: unknown };
}>;

export type Kernel = Record<string, (...args: never[]) => unknown>;

export function unwrap(value: unknown, what: string): unknown {
  if (value !== null && typeof value === "object" && "ok" in (value as Record<string, unknown>)) {
    const result = value as { ok: boolean; value?: unknown; error?: unknown };
    if (!result.ok) throw new Error(`${what}: ${JSON.stringify(result.error)}`);
    return result.value;
  }
  return value;
}

/** 📐️ `box(dx, dy, dz)` sits CORNER-at-origin; `cylinder(r, h)` sits AXIS-at-origin; `rotate(shape,
 * angleDegrees, {at, axis})` takes ONE options object. Every one of these was measured, not assumed. */
export const call = (b: Kernel, name: string, ...args: unknown[]): unknown => unwrap((b[name] as unknown as (...a: unknown[]) => unknown)(...args), name);
