/** 🧬️ XlsxSnapshot schema (🔒️strict subset) meta — reuses the 🧱️base subset's schema verbatim. */
export type { XlsxArtifact } from '../../🧱️base/🧬️schema/🟦️.ts';
export const meta = {
  artifactKind: "s.stdio.xlsx",
  standard: "ecma-376",
  subset: "strict",
} as const;
