import { createRequire } from "node:module";

/** 🎭️Test-owned color admission supplied by the independent color-string library. */
export const colorOracle: { get(input: string): { model: string; value: number[] } | null } = createRequire(import.meta.url)("color-string");
