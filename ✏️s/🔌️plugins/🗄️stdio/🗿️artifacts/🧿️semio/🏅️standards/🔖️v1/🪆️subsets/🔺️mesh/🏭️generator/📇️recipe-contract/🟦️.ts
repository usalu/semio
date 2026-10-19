/** 📇️ The fixture recipe contract of the semio mesh generator: families, toolkit and corpus entry. */

import type { CrossSection, Manifold } from "manifold-3d";

/** 👪️ The fixture families the corpus is sharded and reported by — never at artifact level. */
export type Family = "primitives" | "booleans" | "topology" | "scale" | "degenerate";

/** 🧰️ What a recipe is handed to build with: the two manifold-3d entry points a shape needs. */
export type Toolkit = Readonly<{ Manifold: typeof Manifold; CrossSection: typeof CrossSection }>;

/**
 * 🧪️ One corpus entry. A recipe DESCRIBES a shape; it computes nothing. `build` returns the RESULT to
 * export and measure, plus optionally the named operands that produced it (a boolean's two inputs), so
 * a boolean fixture is reviewable without re-deriving the operands from the chain that built them.
 */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  tolerance: string;
  notes: string;
  build: (t: Toolkit) => { operands?: { role: string; shape: Manifold }[]; result: Manifold };
}>;
