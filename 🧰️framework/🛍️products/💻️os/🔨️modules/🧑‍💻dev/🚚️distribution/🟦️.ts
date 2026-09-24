import catalog from "./📇️layout.json";
import { installationDirectoryEmoji } from "../../🧩️extension/🟦️.ts";

export type DistributionChunkOwner = { readonly kind: "facade" | "module"; readonly source: string; readonly output: string };
export type DistributionAssetOwner = { readonly kind: "source" | "name"; readonly source: string; readonly emittedName?: string; readonly output: string };
export type DistributionLayout = { readonly version: 1; readonly directory: string; readonly bundles: string; readonly entry: { readonly source: string; readonly output: string }; readonly manifest: string; readonly chunks: readonly DistributionChunkOwner[]; readonly assets: readonly DistributionAssetOwner[] };
export type DistributionChunkFacts = { readonly facadeModuleId: string | null; readonly moduleIds: readonly string[] };
export type DistributionAssetFacts = { readonly originalFileNames: readonly string[]; readonly names: readonly string[] };

/** 🧾️ Validates literal source coordinates without deriving output identities from them. */
function sourceCoordinate(value: unknown): string {
  if (typeof value !== "string" || !value || value !== value.normalize("NFC") || /[\\\u0000-\u001F\u007F:]/u.test(value) || value.split("/").some((part) => !part || part === "." || part === "..")) throw new Error("Distribution source must be one literal relative coordinate");
  return value;
}

/** 🚚️ Admits a closed hand-authored output inventory and rejects shared sibling emoji. */
export function parseDistributionLayout(input: unknown): DistributionLayout {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Invalid distribution layout");
  const value = input as Record<string, unknown>;
  if (Object.keys(value).sort().join(",") !== "assets,bundles,chunks,directory,entry,manifest,version" || value.version !== 1 || !Array.isArray(value.chunks) || !value.chunks.length || !Array.isArray(value.assets) || !value.assets.length) throw new Error("Invalid distribution layout fields");
  installationDirectoryEmoji(value.directory);
  installationDirectoryEmoji(value.bundles);
  const entry = value.entry as Record<string, unknown>;
  if (!entry || typeof entry !== "object" || Array.isArray(entry) || Object.keys(entry).sort().join(",") !== "output,source" || typeof entry.output !== "string" || !entry.output.endsWith(".html") || entry.output.includes("/") || typeof value.manifest !== "string" || !value.manifest.endsWith(".json") || value.manifest.includes("/")) throw new Error("Invalid distribution entry or manifest filename");
  sourceCoordinate(entry.source);
  const rootEmoji = [installationDirectoryEmoji(value.bundles), installationDirectoryEmoji(`${entry.output.slice(0, -5)}file`), installationDirectoryEmoji(`${value.manifest.slice(0, -5)}file`)];
  if (new Set(rootEmoji).size !== rootEmoji.length) throw new Error("Distribution root sibling emoji collision");
  const sources = new Set<string>(), outputs = new Set<string>(), emittedNames = new Set<string>(), siblings = new Map<string, string>();
  const parseRows = (rows: unknown[], kinds: readonly string[]) => rows.map((inputRow) => {
    if (!inputRow || typeof inputRow !== "object" || Array.isArray(inputRow)) throw new Error("Invalid distribution owner");
    const row = inputRow as Record<string, unknown>;
    const keys = Object.keys(row).sort().join(",");
    if (typeof row.kind !== "string" || !kinds.includes(row.kind) || (keys !== "kind,output,source" && !(row.kind === "source" && keys === "emittedName,kind,output,source"))) throw new Error("Invalid distribution owner fields");
    const source = sourceCoordinate(row.source);
    const emittedName = row.emittedName === undefined ? undefined : sourceCoordinate(row.emittedName);
    if (row.kind === "name" && source.includes("/")) throw new Error("Asset name owner must be one literal basename");
    if (emittedName?.includes("/")) throw new Error("Pre-emission asset name must be one literal basename");
    const assetName = row.kind === "name" ? source : emittedName;
    if (assetName && emittedNames.has(assetName)) throw new Error("Duplicate pre-emission asset identity");
    if (assetName) emittedNames.add(assetName);
    if (typeof row.output !== "string" || !/^(?:[^/]+\/)*[^/]+-\[hash\]\.(?:js|mjs|css|wasm|woff2?|ttf)$/u.test(row.output)) throw new Error("Distribution output requires one explicit filename with only a hash placeholder");
    const output = row.output, parts = output.split("/");
    if (sources.has(`${row.kind}\0${source}`) || outputs.has(output)) throw new Error("Duplicate distribution source or output");
    sources.add(`${row.kind}\0${source}`);
    outputs.add(output);
    for (let index = 0; index < parts.length; index++) {
      const name = parts[index]!, stem = index === parts.length - 1 ? name.replace(/-\[hash\]\.[a-z0-9]+$/u, "") : name;
      const emoji = installationDirectoryEmoji(stem), key = `${parts.slice(0, index).join("/")}\0${emoji}`;
      if (siblings.has(key) && siblings.get(key) !== name) throw new Error(`Distribution sibling emoji collision: ${name}`);
      siblings.set(key, name);
    }
    return Object.freeze({ kind: row.kind, source, output, ...(emittedName ? { emittedName } : {}) });
  });
  const chunks = parseRows(value.chunks, ["facade", "module"]) as DistributionChunkOwner[];
  const assets = parseRows(value.assets, ["source", "name"]) as DistributionAssetOwner[];
  return Object.freeze({ version: 1, directory: value.directory as string, bundles: value.bundles as string, entry: Object.freeze({ source: entry.source as string, output: entry.output }), manifest: value.manifest, chunks: Object.freeze(chunks), assets: Object.freeze(assets) });
}

export const DISTRIBUTION_LAYOUT = parseDistributionLayout(catalog);

export type DistributionStaticInputs = { readonly version: 1; readonly paths: readonly string[]; readonly moduleEntries: readonly string[] };

/** 🔗️ Admits explicit compiler entry points and static data without widening their coordinates. */
export function parseDistributionStaticInputs(input: unknown): DistributionStaticInputs {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Invalid distribution static-input authority");
  const value = input as Record<string, unknown>;
  if (Object.keys(value).sort().join(",") !== "moduleEntries,paths,version" || value.version !== 1) throw new Error("Invalid distribution static-input fields");
  const paths = (raw: unknown) => {
    if (!Array.isArray(raw) || !raw.length) throw new Error("Static inputs require nonempty exact paths");
    const rows = raw.map(sourceCoordinate);
    if (new Set(rows).size !== rows.length) throw new Error("Duplicate distribution static input");
    return Object.freeze(rows);
  };
  const files = paths(value.paths), moduleEntries = paths(value.moduleEntries);
  if (moduleEntries.some(path => !files.includes(path))) throw new Error("Compiler entry must be present in the static input roster");
  return Object.freeze({ version: 1, paths: files, moduleEntries });
}

/** 🧵️ Resolves a chunk only through its explicit facade or shared-runtime owner. */
export function distributionChunkName(layout: DistributionLayout, facts: DistributionChunkFacts): string {
  const matches = layout.chunks.filter((row) => facts.facadeModuleId === null ? row.kind === "module" && facts.moduleIds.includes(row.source) : row.kind === "facade" && facts.facadeModuleId === row.source);
  if (matches.length !== 1) throw new Error(`Unowned or ambiguous distribution chunk: ${JSON.stringify(facts)}`);
  return `${layout.bundles}/${matches[0]!.output}`;
}

/** 🖼️ Resolves source-backed assets without falling back to generic basename guesses. */
export function distributionAssetName(layout: DistributionLayout, facts: DistributionAssetFacts): string {
  const kind = facts.originalFileNames.length ? "source" : "name", identities = kind === "source" ? facts.originalFileNames : facts.names;
  const identityFor = (row: DistributionAssetOwner) => kind === "source" ? row.kind === "source" ? row.source : undefined : row.kind === "name" ? row.source : row.emittedName;
  const matches = layout.assets.filter((row) => identities.includes(identityFor(row) ?? ""));
  if (matches.length !== 1 || identities.some((identity) => identity !== identityFor(matches[0]!))) throw new Error(`Unowned or ambiguous distribution asset: ${JSON.stringify(facts)}`);
  return `${layout.bundles}/${matches[0]!.output}`;
}

export type DistributionInput = { readonly path: string; readonly bytes: number; readonly sha256: string };
export type DistributionOutput = DistributionInput & { readonly ownerId: string };
export type DistributionManifest = { readonly version: 1; readonly contractId: "dev-distribution-bundle"; readonly layoutSha256: string; readonly inputs: readonly DistributionInput[]; readonly outputs: readonly DistributionOutput[] };

/** 🪪️ Matches one declared output template, which is also its stable source-owner identity. */
export function distributionOutputOwner(layout: DistributionLayout, path: string): string {
  sourceCoordinate(path);
  if (path === layout.entry.output) return layout.entry.output;
  const matches = [...layout.chunks, ...layout.assets].filter(row => {
    const [prefix, suffix] = `${layout.bundles}/${row.output}`.split("[hash]");
    if (!path.startsWith(prefix!) || !path.endsWith(suffix!)) return false;
    return /^[A-Za-z0-9_-]{8,64}$/u.test(path.slice(prefix!.length, path.length - suffix!.length));
  });
  if (matches.length !== 1) throw new Error(`Unowned or ambiguous emitted distribution output: ${path}`);
  return matches[0]!.output;
}

/** 🔏️ Validates a bounded compiler manifest without admitting aliases or copied namespaces. */
export function parseDistributionManifest(input: unknown, rawLayout: DistributionLayout): DistributionManifest {
  const layout = parseDistributionLayout(rawLayout);
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Invalid distribution manifest");
  const value = input as Record<string, unknown>, digest = (item: unknown) => typeof item === "string" && /^[a-f0-9]{64}$/u.test(item);
  if (Object.keys(value).sort().join(",") !== "contractId,inputs,layoutSha256,outputs,version" || value.version !== 1 || value.contractId !== "dev-distribution-bundle" || !digest(value.layoutSha256)) throw new Error("Invalid distribution manifest fields");
  const parseRows = (inputRows: unknown, output: boolean) => {
    if (!Array.isArray(inputRows) || inputRows.length === 0) throw new Error("Distribution manifest requires a nonempty exact inventory");
    const paths = new Set<string>(), owners = new Set<string>();
    return inputRows.map(inputRow => {
      if (!inputRow || typeof inputRow !== "object" || Array.isArray(inputRow)) throw new Error("Invalid distribution manifest row");
      const row = inputRow as Record<string, unknown>;
      if (Object.keys(row).sort().join(",") !== (output ? "bytes,ownerId,path,sha256" : "bytes,path,sha256") || !Number.isSafeInteger(row.bytes) || (row.bytes as number) < 0 || !digest(row.sha256)) throw new Error("Invalid distribution manifest row fields");
      const path = sourceCoordinate(row.path);
      if (paths.has(path)) throw new Error("Duplicate distribution manifest path");
      paths.add(path);
      if (output) {
        const ownerId = distributionOutputOwner(layout, path);
        if (row.ownerId !== ownerId || owners.has(ownerId)) throw new Error("Unknown, mismatched or repeated distribution owner");
        owners.add(ownerId);
        return Object.freeze({ ownerId, path, bytes: row.bytes as number, sha256: row.sha256 as string });
      }
      return Object.freeze({ path, bytes: row.bytes as number, sha256: row.sha256 as string });
    });
  };
  const inputs = parseRows(value.inputs, false), outputs = parseRows(value.outputs, true) as DistributionOutput[];
  if (!outputs.some(row => row.path === layout.entry.output)) throw new Error("Distribution manifest is missing the declared HTML entry");
  return Object.freeze({ version: 1, contractId: "dev-distribution-bundle", layoutSha256: value.layoutSha256 as string, inputs: Object.freeze(inputs), outputs: Object.freeze(outputs) });
}
