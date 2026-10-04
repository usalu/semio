/** 📇️ Language-neutral graph output catalog contract. */
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
import { pathEmojiStatuteFindings } from "../../../🪪️identity/🛣️path/🟦️.ts";

export interface GraphOutputCatalog {
  readonly $schema?: string;
  readonly contractId: string;
  readonly version: 1;
  readonly inputAreas: readonly string[];
  readonly policy: Readonly<{ excludedInputPaths: readonly string[]; genericEmojiIdentities: readonly string[] }>;
  readonly shared: Readonly<{ rustRegistry: string; typescriptIndex: string; typescriptTypes: string }>;
  readonly manifests: readonly Readonly<{ id: string; rust: string; typescript: string }>[];
}

/** 📇️Validates the exact language-neutral output authority and its bijection to admitted manifests. */
export function parseGraphOutputCatalog(input: unknown, manifestIds: readonly string[]): GraphOutputCatalog {
  function record(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
    if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("graph output catalog requires records");
    const row = value as Record<string, unknown>;
    if (required.some((key) => !(key in row)) || Object.keys(row).some((key) => !required.includes(key) && !optional.includes(key))) throw new Error("graph output catalog has missing or unknown fields");
    return row;
  }
  const root = record(input, ["contractId", "version", "inputAreas", "policy", "shared", "manifests"], ["$schema"]);
  if (typeof root.contractId !== "string" || !/^[a-z][a-z0-9-]*$/u.test(root.contractId)) throw new Error("graph output catalog requires an explicit canonical contract identity");
  if (root.version !== 1 || root.$schema !== undefined && typeof root.$schema !== "string") throw new Error("graph output catalog version/schema is invalid");
  function relativePaths(value: unknown): readonly string[] {
    if (!Array.isArray(value) || value.some((area) => typeof area !== "string" || area !== area.normalize("NFC") || !area || /[\\%?#\u0000-\u001f]/u.test(area) || area.startsWith("/") || /^[A-Za-z]:/u.test(area) || area.split("/").some((segment) => !segment || segment === "." || segment === "..")) || new Set(value).size !== value.length) throw new Error("graph input paths must be exact unique root-relative owners");
    return Object.freeze([...value] as string[]);
  }
  const inputAreas = relativePaths(root.inputAreas);
  const declaredPolicy = record(root.policy, ["excludedInputPaths", "genericEmojiIdentities"]);
  const excludedInputPaths = relativePaths(declaredPolicy.excludedInputPaths);
  const generic = declaredPolicy.genericEmojiIdentities;
  if (!Array.isArray(generic) || generic.some(value => typeof value !== "string" || !value || value !== value.normalize("NFC")) || new Set(generic).size !== generic.length) throw new Error("graph generic emoji identities must be exact unique strings");
  const policy = Object.freeze({ excludedInputPaths, genericEmojiIdentities: Object.freeze([...generic] as string[]) });
  const shared = record(root.shared, ["rustRegistry", "typescriptIndex", "typescriptTypes"]);
  const seen = new Set<string>();
  const entries: { path: string; nodeKind: "file" | "directory"; reserved: boolean }[] = [];
  const directories = new Set<string>();
  function path(value: unknown, pattern: RegExp): string {
    if (typeof value !== "string" || value !== value.normalize("NFC") || !pattern.test(value) || /[\\%?#\u0000-\u001f]/u.test(value)) throw new Error("graph output path is not an exact safe identity");
    if (seen.has(value)) throw new Error(`duplicate graph output path ${value}`);
    seen.add(value);
    const parent = dirname(value);
    if (parent !== "." && !directories.has(parent)) {
      directories.add(parent);
      entries.push({ path: parent, nodeKind: "directory", reserved: false });
    }
    entries.push({ path: value, nodeKind: "file", reserved: false });
    return value;
  }
  const outputShared = Object.freeze({
    rustRegistry: path(shared.rustRegistry, /^[^/.]+\/🦀️\.rs$/u),
    typescriptIndex: path(shared.typescriptIndex, /^[^/.]+\.ts$/u),
    typescriptTypes: path(shared.typescriptTypes, /^[^/.]+\/🟦️\.ts$/u),
  });
  if (!Array.isArray(root.manifests)) throw new Error("graph output manifests must be an array");
  const ids = new Set<string>();
  const manifests = root.manifests.map((value) => {
    const row = record(value, ["id", "rust", "typescript"]);
    if (typeof row.id !== "string" || !/^[a-z][a-z0-9-]*$/u.test(row.id) || ids.has(row.id)) throw new Error("graph output manifest identity is invalid or duplicated");
    ids.add(row.id);
    const rust = path(row.rust, /^[^/.]+\/🦀️\.rs$/u);
    const typescript = path(row.typescript, /^[^/.]+\/🟦️\.ts$/u);
    if (dirname(rust) !== dirname(typescript)) throw new Error(`graph output language pair has different owners: ${row.id}`);
    return Object.freeze({ id: row.id, rust, typescript });
  });
  if (manifestIds.length !== ids.size || new Set(manifestIds).size !== manifestIds.length || manifestIds.some((id) => !ids.has(id))) throw new Error("graph output catalog and admitted manifest identities differ");
  const findings = pathEmojiStatuteFindings(entries, policy.genericEmojiIdentities);
  if (findings.length > 0) throw new Error(`graph output identities breach path statutes: ${JSON.stringify(findings)}`);
  return Object.freeze({ ...(root.$schema === undefined ? {} : { $schema: root.$schema as string }), contractId: root.contractId, version: 1, inputAreas, policy, shared: outputShared, manifests: Object.freeze(manifests) });
}

export function readGraphOutputCatalog(path:string): GraphOutputCatalog {
  try {
    const input = JSON.parse(readFileSync(path, "utf8"));
    if (!Array.isArray(input?.manifests)) throw new Error("graph output manifests must be an array");
    return parseGraphOutputCatalog(input,input.manifests.map((row:{id:string})=>row.id));
  } catch (error) {
    throw new Error(`cannot read the graph output catalog: ${error instanceof Error ? error.message : String(error)}`, { cause: error });
  }
}
