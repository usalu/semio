/** 📇️ Loader of the artifact acceptance fixture: typed rows, repository roots and the natural-file sample of a row.
 * @see ../🧬️schema/🔣️.json */
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** 🧾️ One row of the matrix. */
export type AcceptanceRow = Readonly<{
  batch: string; artifact: string; directory: string; kind: string; plugin: string | null; extension: string; oracle: string; surface: "shipped" | "missing"; ownerApp: string; sample: string;
  authoredExample?: Readonly<{id:string;label:Readonly<{en:string;de:string}>;field:string;baseline:string;documentSchema:string;source:string}>;
  edit: Readonly<{ mode: "verb" | "rendered"; path?: "main" | "details"; field?: string; valueKind?: "text" | "number" | "toggle"; verb?: string; args?: Readonly<Record<string, string>>; control?: string; value?: string }>;
  persistence: Readonly<{ comparison: "bytes" | "json" | "zip-members"; requireEditedExport: true; requireUndoRedoExport: true; requirePersistedUuid: true }>;
  export: Readonly<{ format: string }>;
  validPayloads?: readonly string[];
  invalid: Readonly<{ kind: "malformed" | "unsupported-format"; extension?: string; bytes: string; expect: "refusal-notice" }>;
}>;
export type AcceptanceMatrix = Readonly<{ schema: string; nonArtifacts: readonly string[]; artifacts: readonly AcceptanceRow[] }>;

const HERE = dirname(fileURLToPath(import.meta.url));
export const ACCEPTANCE_DIRECTORY = resolve(HERE, "..");
export const REPO_ROOT = resolve(HERE, "../../../../../..");
export const ARTIFACTS_ROOT = join(REPO_ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
export const readSchema = (): unknown => JSON.parse(readFileSync(join(ACCEPTANCE_DIRECTORY, "🧬️schema/🔣️.json"), "utf8"));
export const readMatrix = (): AcceptanceMatrix => JSON.parse(readFileSync(join(ACCEPTANCE_DIRECTORY, "🧫️fixtures/🔣️.json"), "utf8")) as AcceptanceMatrix;
export const hexBytes = (hex: string): Uint8Array => Uint8Array.from(hex.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));

/** 📇️ The schema-defined sample manifest: repository-relative valid and invalid files per artifact. */
export type SampleManifest = Readonly<{ schema: string; samples: Readonly<Record<string, Readonly<{ valid: readonly string[]; invalid: readonly string[] }>>> }>;
export const readSampleManifest = (): SampleManifest => JSON.parse(readFileSync(join(ACCEPTANCE_DIRECTORY, "🧫️fixtures/📇️samples.json"), "utf8")) as SampleManifest;
export const readSampleSchema = (): unknown => JSON.parse(readFileSync(join(ACCEPTANCE_DIRECTORY, "🧬️schema/📇️samples.json"), "utf8"));

/** 🎯️ The row's exact manifest-listed sample; a missing or refused pin never falls back to another subset. */
export function candidateSamples(row: AcceptanceRow): string[] {
  if (!(readSampleManifest().samples[row.artifact]?.valid ?? []).includes(row.sample)) return [];
  const path = join(REPO_ROOT, row.sample);
  return existsSync(path) && statSync(path).isFile() && statSync(path).size > 0 ? [path] : [];
}

/** 🚫️ The manifest's invalid samples of the row that exist on disk. */
export const invalidSamples = (row: AcceptanceRow): string[] => (readSampleManifest().samples[row.artifact]?.invalid ?? []).map((path) => join(REPO_ROOT, path)).filter((path) => existsSync(path));

/** 🧪️ The exact pinned sample when its independent oracle accepts it, otherwise no sample. */
export function resolveSample(row: AcceptanceRow, accepts: (bytes: Uint8Array) => boolean): string | null {
  return candidateSamples(row).find((path) => accepts(new Uint8Array(readFileSync(path)))) ?? null;
}

/** 🎯️ Resolves the editor declared by a row within its owning plugin. */
export function selectEditorProgram<T extends Readonly<{ pluginId: string; appId: string }>>(row: AcceptanceRow, programs: readonly T[]): T | undefined {
  return programs.find((program) => program.pluginId === row.plugin && program.appId === row.ownerApp);
}
