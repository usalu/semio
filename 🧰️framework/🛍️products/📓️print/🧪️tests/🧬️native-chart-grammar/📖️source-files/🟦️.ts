/** 📖️ The source files of the native chart grammar: the grammar run and the child folders that hold its compilations, read by the source-text laws that scan the grammar for its probe, scratch and paint calls.
 *
 * @see ../🟦️.ts — the grammar run
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const GRAMMAR = join(dirname(fileURLToPath(import.meta.url)), "..");

/** 📚️ The files of the grammar, relative to its folder: the run first, then its children in order. */
export const NATIVE_GRAMMAR_SOURCE_FILES = ["🟦️.ts", "🧰️grammar-kit/🟦️.ts", "🧱️basics/🟦️.ts", "🎛️family-and-scientific/🟦️.ts", "🗺️geo-planar/🟦️.ts", "🏭️domain-effects/🟦️.ts", "🧠️neural-and-domain/🟦️.ts"] as const;

/** 📄️ Every file of the grammar with its absolute path and its text. */
export function nativeGrammarSources(): readonly { readonly path: string; readonly text: string }[] {
  return NATIVE_GRAMMAR_SOURCE_FILES.map((file) => {
    const path = join(GRAMMAR, file);
    return { path, text: readFileSync(path, "utf8") };
  });
}
