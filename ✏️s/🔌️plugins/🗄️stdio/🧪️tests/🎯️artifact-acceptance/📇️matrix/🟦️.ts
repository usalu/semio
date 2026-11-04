/** 📇️ Loader of the artifact acceptance fixture: typed rows, repository roots and the natural-file sample of a row.
 * @see ../🧬️schema/🔣️.json */
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** 🧾️ One row of the matrix. */
export type AcceptanceRow = Readonly<{
  batch: string; artifact: string; directory: string; kind: string; plugin: string | null; extension: string; oracle: string; surface: "shipped" | "missing"; ownerApp?: string; sample?: string;
  edit: Readonly<{ mode: "verb" | "rendered" | "scan" | "none"; path?: "main" | "details"; field?: string; verb?: string; args?: Readonly<Record<string, string>>; control?: string; value?: string }>;
  export: Readonly<{ format: string }>;
  invalid: Readonly<{ bytes: string; expect: "refusal-notice" }>;
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

/** 🧪️ The row's candidate sample files: the manifest's valid samples, smallest first (ties by path); files that do not exist are dropped. */
export function candidateSamples(row: AcceptanceRow): string[] {
  const listed = (readSampleManifest().samples[row.artifact]?.valid ?? []).map((path) => join(REPO_ROOT, path)).filter((path) => existsSync(path) && statSync(path).size > 0);
  return listed.sort((a, b) => statSync(a).size - statSync(b).size || a.localeCompare(b));
}

/** 🚫️ The manifest's invalid samples of the row that exist on disk. */
export const invalidSamples = (row: AcceptanceRow): string[] => (readSampleManifest().samples[row.artifact]?.invalid ?? []).map((path) => join(REPO_ROOT, path)).filter((path) => existsSync(path));

/** 🧪️ The first candidate that the row's third-party oracle accepts, or null when the tree holds no valid natural file. */
export function resolveSample(row: AcceptanceRow, accepts: (bytes: Uint8Array) => boolean): string | null {
  return candidateSamples(row).find((path) => accepts(new Uint8Array(readFileSync(path)))) ?? null;
}
