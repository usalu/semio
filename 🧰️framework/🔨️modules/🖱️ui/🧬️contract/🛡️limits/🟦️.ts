import type { UiDocumentLimits } from "@semio-tech/framework";

/** 🛡️ Cross-renderer document admission limits shared by browser and native UI hosts. */
export const DEFAULT_UI_DOCUMENT_LIMITS = {
  maxNodes: 20_000,
  maxDepth: 128,
  maxChildren: 4_096,
  maxTextBytes: 65_536,
  maxPatchOps: 4_096,
  maxPatchBytes: 1_048_576,
} satisfies UiDocumentLimits;

/** 🧲️ The detent law: finite, strictly ascending, inside `min..=max` — the twin of the Rust admission gate `snaps_are_valid`. */
export function snapsAreValid(snaps: readonly number[], min: number, max: number): boolean {
  let previous = Number.NEGATIVE_INFINITY;
  return snaps.every((snap) => {
    const valid = Number.isFinite(snap) && snap > previous && snap >= min && snap <= max;
    previous = snap;
    return valid;
  });
}
