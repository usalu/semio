import type { IdmlSnapshot } from "../📸️snapshot/🟦️.ts";
/** 🧬️ Semantic IDML edits become validated package diffs before publication. */
export type IdmlMutation = { mutation: "replacePackage"; document: IdmlSnapshot } | { mutation: "replacePart"; path: string; bytes: number[] };
