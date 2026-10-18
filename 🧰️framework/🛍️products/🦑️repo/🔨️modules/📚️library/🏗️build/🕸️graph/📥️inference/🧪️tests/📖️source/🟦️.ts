/** 📖️ Reads the Source files of the build inference module through its component roster, so Source readers see the complete component graph instead of only its root. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const inference = resolve(import.meta.dir, "../..");
const manifestPath = join(inference, "🗂️components/🔣️.json");

/** 📖️ Names one Source file of the build inference module: the root, the revision-graph kernel, the roster or one component. */
export interface InferenceSourceFileV1 { readonly name: string; readonly path: string }

/** 📖️ Lists every Source file of the build inference module in roster order, so readers see the complete component graph instead of only its root. */
export function inferenceSourceFilesV1(): readonly InferenceSourceFileV1[] {
  const manifest: { readonly components: readonly { readonly name: string; readonly path: string }[] } = JSON.parse(readFileSync(manifestPath, "utf8"));
  return [
    { name: "root", path: join(inference, "🟨️.mjs") },
    { name: "revision-graph", path: join(inference, "🪢️revision-graph/🟨️.mjs") },
    { name: "components", path: manifestPath },
    ...manifest.components.map((entry) => ({ name: entry.name, path: join(inference, entry.path) })),
  ];
}

/** 📖️ Reads one named Source file of the build inference module. */
export function readInferenceSourceV1(name: string): string {
  const file = inferenceSourceFilesV1().find((row) => row.name === name);
  if (!file) throw Error(`Unknown build inference Source ${name}`);
  return readFileSync(file.path, "utf8");
}

/** 📖️ Reads every Source file of the build inference module as one text, for laws that must hold across the whole component graph. */
export function readInferenceGraphSourceV1(): string {
  return inferenceSourceFilesV1().map((row) => readFileSync(row.path, "utf8")).join("\n");
}
