/** 📇️ The fixture recipe contract of the semio brep generator: families, mutation kinds, corpus entry and kernel call. */

/** 👪️ The fixture families the corpus is sharded and reported by. */
export type Family = "topology-build" | "topology-remove" | "geometry-replace" | "move-vertex" | "booleans";

/** 🧬️ The 13-verb vocabulary this subset's `🧬️schema/🧬️mutations/🦀️.rs` dispatches — one kind per recipe. */
export type MutationKind =
  | "create-vertex"
  | "delete-vertex"
  | "create-edge"
  | "delete-edge"
  | "create-face"
  | "delete-face"
  | "create-shell"
  | "delete-shell"
  | "create-solid"
  | "delete-solid"
  | "replace-curve"
  | "replace-surface"
  | "move-vertex";

/**
 * 🧪️ One corpus entry. `outcome` is the DECLARED semantic class: a fixture that accepts any non-crash
 * result measures nothing, and a kernel that merely fails to throw is not the same as a kernel that
 * accepted the edit, so every entry says which answer it expects rather than discovering it.
 *
 * `build` returns the BEFORE operand(s) and, where the edit is legal, the AFTER shape. A `result` of
 * `null` is itself a DECLARED, MEASURED fact for `outcome: "rejected"` — see `📓️` note on
 * `generateOne` for how a rejection is proven rather than asserted.
 */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  kind: MutationKind;
  outcome: "applied" | "rejected" | "disjoint" | "no-op" | "empty";
  tolerance: string;
  notes: string;
  build: (b: Kernel) => { operands: { role: string; shape: unknown }[]; result: unknown | null; rejectionReason?: string };
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

/** 📐️ `box(dx,dy,dz)` sits CORNER-at-origin; `cylinder(r,h)` sits AXIS-at-origin extending +z;
 * `sphere(r)`/`cone` centre-or-base-at `at`; `rotate(shape, angleDegrees, {at, axis})` takes ONE
 * options object. Every one of these was MEASURED against the installed `brepjs@18.119.8`, not
 * assumed from documentation. */
export const call = (b: Kernel, name: string, ...args: unknown[]): unknown => unwrap((b[name] as unknown as (...a: unknown[]) => unknown)(...args), name);
