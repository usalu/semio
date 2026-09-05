#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-graph` — the semio graph crate: graph-manifest codegen, cargo test and clippy gates. */
import { existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, rmdirSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { BundleScript, getWorkspaceRoot, ScriptRouter, runBundleScriptMain, runCargoLint, runCargoTestBudgeted, resolveTestLevel, runCmd } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { loadTaxonomy, pathEmojiStatuteFindings, pathIsExcluded } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

interface GraphOutputCatalog {
  readonly $schema?: string;
  readonly version: 1;
  readonly shared: Readonly<{ rustRegistry: string; typescriptIndex: string; typescriptTypes: string; jsonSchema: string }>;
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
  const root = record(input, ["version", "shared", "manifests"], ["$schema"]);
  if (root.version !== 1 || root.$schema !== undefined && typeof root.$schema !== "string") throw new Error("graph output catalog version/schema is invalid");
  const shared = record(root.shared, ["rustRegistry", "typescriptIndex", "typescriptTypes", "jsonSchema"]);
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
    rustRegistry: path(shared.rustRegistry, /^[^/.]+\.rs$/u),
    typescriptIndex: path(shared.typescriptIndex, /^[^/.]+\.ts$/u),
    typescriptTypes: path(shared.typescriptTypes, /^[^/.]+\.ts$/u),
    jsonSchema: path(shared.jsonSchema, /^[^/.]+\.schema\.json$/u),
  });
  if (!Array.isArray(root.manifests) || root.manifests.length === 0) throw new Error("graph output manifests must be nonempty");
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
  const findings = pathEmojiStatuteFindings(entries, loadTaxonomy().pathEmojiPolicy.genericEmojiIdentities);
  if (findings.length > 0) throw new Error(`graph output identities breach path statutes: ${JSON.stringify(findings)}`);
  return Object.freeze({ ...(root.$schema === undefined ? {} : { $schema: root.$schema as string }), version: 1, shared: outputShared, manifests: Object.freeze(manifests) });
}

//#region 🔖️ManifestSource
type ManifestAxes = { portModel?: string; directedness?: string };

type ManifestPropertyDef = {
  name: string;
  kind: "data" | "derived";
  valueType?: unknown;
  expr?: string;
};

type ManifestKindRow = {
  id: string;
  name?: string;
  properties?: ManifestPropertyDef[];
  ports?: string[];
  presentation?: Record<string, unknown>;
};

type ManifestDocument = {
  schema: string;
  id: string;
  name?: string;
  axes?: ManifestAxes;
  nodeKinds?: ManifestKindRow[];
  edgeKinds?: ManifestKindRow[];
  portKinds?: ManifestKindRow[];
  wireKinds?: ManifestKindRow[];
  layerKinds?: ManifestKindRow[];
  languageKinds?: ManifestKindRow[];
  surfaceKinds?: ManifestKindRow[];
  windowKinds?: ManifestKindRow[];
  fileNodeKinds?: ManifestKindRow[];
  descriptorKinds?: ManifestKindRow[];
  edgeTips?: Record<string, unknown>[];
  kindCompatibility?: Record<string, unknown>[];
};

function findManifestFiles(root: string): string[] {
  const out: string[] = [];
  const taxonomy = loadTaxonomy();
  function walk(dir: string) {
    for (const name of readdirSync(dir)) {
      // 🌳️ Skip dot-directories outright — `.claude/worktrees/<agent-id>` holds a full parallel checkout of the repo (see `git worktree list`), and walking into it re-discovers every manifest.json under a second, identical id, producing duplicate `pub mod`/`MANIFEST_IDS`/match-arm entries in the generated registry.
      if (name === "node_modules" || name === "generated" || name === "🤖️generated" || name === "target" || name.startsWith(".")) continue;
      const path = join(dir, name);
      if (pathIsExcluded(root, path, taxonomy)) continue;
      let st: ReturnType<typeof statSync>;
      try {
        st = statSync(path);
      } catch {
        continue;
      }
      if (st.isDirectory()) {
        walk(path);
      } else if (name.endsWith("manifest.json")) {
        out.push(path);
      }
    }
  }
  for (const area of taxonomy.pluginAreas) {
    const scanRoot = join(root, area);
    if (!pathIsExcluded(root, scanRoot, taxonomy) && existsSync(scanRoot)) walk(scanRoot);
  }
  return out.sort();
}

function pascalCase(id: string): string {
  const parts = id
    .replace(/[^a-zA-Z0-9]+/g, " ")
    .trim()
    .split(/\s+/)
    .filter(Boolean);
  if (parts.length === 0) return "Unknown";
  const result = parts.map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join("");
  return /^[0-9]/.test(result) ? `R${result}` : result;
}

function snakeUpper(id: string): string {
  return pascalCase(id)
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .toUpperCase();
}

function rustModName(id: string): string {
  return id.replace(/[^a-zA-Z0-9_]/g, "_");
}

function rustFnName(id: string): string {
  return `${rustModName(id)}_manifest`;
}

function rustStr(s: string): string {
  return JSON.stringify(s);
}

function tsStringLiteral(s: string): string {
  return JSON.stringify(s);
}

function familyRows(doc: ManifestDocument, family: keyof ManifestDocument): ManifestKindRow[] {
  const rows = doc[family];
  return Array.isArray(rows) ? (rows as ManifestKindRow[]) : [];
}

function emitRustFamily(prefix: string, familyName: string, rows: ManifestKindRow[]): string {
  if (rows.length === 0) return "";
  const enumName = `${prefix}${familyName}Kind`;
  let body = "";
  let consts = "";
  const ids: string[] = [];
  for (const row of rows) {
    const variant = pascalCase(row.id);
    const id = row.id;
    ids.push(id);
    body += `    ${variant},\n`;
    consts += `pub const ${prefix.toUpperCase()}_${familyName.toUpperCase()}_${snakeUpper(id)}: &str = ${rustStr(id)};\n`;
  }
  return (
    `${consts}\n` +
    `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n` +
    `pub enum ${enumName} {\n${body}}\n\n` +
    `impl ${enumName} {\n` +
    `    pub const ALL: &'static [Self] = &[${rows.map((r) => `${enumName}::${pascalCase(r.id)}`).join(", ")}];\n` +
    `    pub fn as_str(self) -> &'static str {\n` +
    `        match self {\n${rows.map((r) => `            Self::${pascalCase(r.id)} => ${rustStr(r.id)},`).join("\n")}\n` +
    `        }\n` +
    `    }\n` +
    `    pub fn parse(s: &str) -> Result<Self, String> {\n` +
    `        match s {\n${rows.map((r) => `            ${rustStr(r.id)} => Ok(Self::${pascalCase(r.id)}),`).join("\n")}\n` +
    `            other => Err(format!("unknown ${familyName.toLowerCase()} kind {other:?} for ${prefix}")),` +
    `\n        }\n    }\n` +
    `}\n\n` +
    `pub const ${prefix.toUpperCase()}_${familyName.toUpperCase()}_IDS: &[&str] = &[${ids.map((id) => rustStr(id)).join(", ")}];\n`
  );
}

function emitTsFamily(prefix: string, familyName: string, rows: ManifestKindRow[]): string {
  if (rows.length === 0) return "";
  const typeName = `${prefix}${familyName}KindId`;
  const union = rows.map((r) => tsStringLiteral(r.id)).join(" | ");
  const consts = rows.map((r) => `export const ${prefix.toUpperCase()}_${familyName.toUpperCase()}_${snakeUpper(r.id)} = ${tsStringLiteral(r.id)} as const;`).join("\n");
  return `${consts}\n\nexport type ${typeName} = ${union};\nexport const ${prefix.toUpperCase()}_${familyName.toUpperCase()}_IDS = [${rows.map((r) => tsStringLiteral(r.id)).join(", ")}] as const satisfies readonly ${typeName}[];\n`;
}

function emitRustManifest(doc: ManifestDocument): string {
  const prefix = pascalCase(doc.id);
  const modName = rustModName(doc.id);
  const fnName = rustFnName(doc.id);
  const json = JSON.stringify(doc);
  const families =
    emitRustFamily(prefix, "Node", familyRows(doc, "nodeKinds")) +
    emitRustFamily(prefix, "Edge", familyRows(doc, "edgeKinds")) +
    emitRustFamily(prefix, "Port", familyRows(doc, "portKinds")) +
    emitRustFamily(prefix, "Wire", familyRows(doc, "wireKinds")) +
    emitRustFamily(prefix, "Layer", familyRows(doc, "layerKinds")) +
    emitRustFamily(prefix, "Language", familyRows(doc, "languageKinds")) +
    emitRustFamily(prefix, "Surface", familyRows(doc, "surfaceKinds")) +
    emitRustFamily(prefix, "Window", familyRows(doc, "windowKinds")) +
    emitRustFamily(prefix, "FileNode", familyRows(doc, "fileNodeKinds")) +
    emitRustFamily(prefix, "Descriptor", familyRows(doc, "descriptorKinds"));
  let out = `// Generated from ${doc.id}.manifest.json\n\nuse crate::manifest::Manifest;\n\n`;
  out += families;
  out += `pub const ${prefix.toUpperCase()}_MANIFEST_JSON: &str = ${rustStr(json)};\n\n`;
  // 🌉️ `dsl_core::json::from_json_str` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
  // 26/09/02 Phase 2) — the `ToValue`/`FromValue` analog of `serde_json::from_str`, routed through
  // `Manifest`'s own hand-written `FromValue` impl instead of a derive-generated `Deserialize`.
  out += `pub fn ${fnName}() -> Manifest {\n    dsl_core::json::from_json_str(${prefix.toUpperCase()}_MANIFEST_JSON).expect("manifest json")\n}\n`;
  return out;
}

function emitTsManifest(doc: ManifestDocument, typesSpecifier: string): string {
  const prefix = pascalCase(doc.id);
  let out = `// Generated from ${doc.id}.manifest.json\n\nimport type { GraphManifestDocument, KindCatalogBundle } from ${JSON.stringify(typesSpecifier)};\n\n`;
  out += emitTsFamily(prefix, "Node", familyRows(doc, "nodeKinds"));
  out += emitTsFamily(prefix, "Edge", familyRows(doc, "edgeKinds"));
  out += emitTsFamily(prefix, "Port", familyRows(doc, "portKinds"));
  out += emitTsFamily(prefix, "Wire", familyRows(doc, "wireKinds"));
  out += emitTsFamily(prefix, "Layer", familyRows(doc, "layerKinds"));
  out += emitTsFamily(prefix, "Language", familyRows(doc, "languageKinds"));
  out += emitTsFamily(prefix, "Surface", familyRows(doc, "surfaceKinds"));
  out += emitTsFamily(prefix, "Window", familyRows(doc, "windowKinds"));
  out += emitTsFamily(prefix, "FileNode", familyRows(doc, "fileNodeKinds"));
  out += emitTsFamily(prefix, "Descriptor", familyRows(doc, "descriptorKinds"));
  out += `\nexport const ${prefix.toUpperCase()}_MANIFEST_DOCUMENT = ${JSON.stringify(doc, null, 2)} as const satisfies GraphManifestDocument;\n`;
  out += `\nexport function ${rustModName(doc.id)}ManifestCatalogBundle(): KindCatalogBundle {\n`;
  out += `  const doc: GraphManifestDocument = ${prefix.toUpperCase()}_MANIFEST_DOCUMENT;\n`;
  out += `  return {\n`;
  out += `    handles: doc.portKinds?.map((row) => ({\n`;
  out += `      id: row.id,\n      name: row.name ?? row.id,\n`;
  out += `      color: String((row.presentation as { color?: string })?.color ?? "hsl(215 52% 48%)"),\n`;
  out += `      defaultWireKind: (row.presentation as { defaultWireKind?: string })?.defaultWireKind,\n`;
  out += `    })),\n`;
  out += `    wires: doc.wireKinds?.map((row) => ({\n`;
  out += `      id: row.id,\n      name: row.name ?? row.id,\n`;
  out += `      defaultEdgeKind: (row.presentation as { defaultEdgeKind?: string })?.defaultEdgeKind,\n`;
  out += `    })),\n`;
  out += `    nodes: doc.nodeKinds?.map((row) => ({\n`;
  out += `      id: row.id,\n      name: row.name ?? row.id,\n`;
  out += `      color: (row.presentation as { color?: string })?.color,\n`;
  out += `      stroke: (row.presentation as { stroke?: string })?.stroke,\n`;
  out += `      icon: (row.presentation as { icon?: string })?.icon,\n`;
  out += `      handles: (row.presentation as { handles?: readonly { handleKind: string; angle: number; radius?: number }[] })?.handles,\n`;
  out += `    })),\n`;
  out += `    edges: doc.edgeKinds?.map((row) => ({\n`;
  out += `      id: row.id,\n      name: row.name ?? row.id,\n`;
  out += `      color: (row.presentation as { color?: string })?.color,\n`;
  out += `      stroke: (row.presentation as { stroke?: string | number })?.stroke as string | undefined,\n`;
  out += `      pattern: (row.presentation as { pattern?: string })?.pattern,\n`;
  out += `      shape: (row.presentation as { shape?: "bezier" | "line" })?.shape,\n`;
  out += `      sourceTip: (row.presentation as { sourceTip?: string })?.sourceTip,\n`;
  out += `      targetTip: (row.presentation as { targetTip?: string })?.targetTip,\n`;
  out += `      directed: (row.presentation as { directed?: boolean })?.directed,\n`;
  out += `    })),\n`;
  out += `    edgeTips: doc.edgeTips as KindCatalogBundle["edgeTips"],\n`;
  out += `  };\n}\n`;
  return out;
}

function emitJsonSchema(): string {
  return JSON.stringify(
    {
      $schema: "https://json-schema.org/draft/2020-12/schema",
      $id: "manifest",
      title: "GraphManifestDocument",
      type: "object",
      required: ["schema", "id"],
      properties: {
        schema: { const: "manifest" },
        id: { type: "string" },
        name: { type: "string" },
        axes: {
          type: "object",
          properties: {
            portModel: { enum: ["normal", "ported"] },
            directedness: { enum: ["directed", "undirected"] },
          },
        },
        nodeKinds: { type: "array" },
        edgeKinds: { type: "array" },
        portKinds: { type: "array" },
        wireKinds: { type: "array" },
        layerKinds: { type: "array" },
        languageKinds: { type: "array" },
        surfaceKinds: { type: "array" },
        windowKinds: { type: "array" },
        fileNodeKinds: { type: "array" },
        descriptorKinds: { type: "array" },
        edgeTips: { type: "array" },
        kindCompatibility: { type: "array" },
      },
    },
    null,
    2,
  );
}

type GraphArtifact = { path: string; content: string };

/** @emoji 🧾️ Renders the full graph catalog from lexically admitted manifest inputs without writes. */
export function renderGraphArtifacts(root: string, outDir: string, log = true): { artifacts: readonly GraphArtifact[]; manifestCount: number } {
    const artifacts: GraphArtifact[] = [];
    const files = findManifestFiles(root);
    if (files.length === 0) {
      throw new Error("no *.manifest.json files found");
    }
    const docs: ManifestDocument[] = [];
    for (const path of files) {
      const doc = JSON.parse(readFileSync(path, "utf8")) as ManifestDocument;
      if (doc.schema !== "manifest") {
        if (log) console.log(`[framework-graph] skip ${relative(root, path)} (${doc.schema})`);
        continue;
      }
      if (log) console.log(`[framework-graph] ${relative(root, path)}`);
      docs.push(doc);
    }
    if (docs.length === 0) {
      throw new Error("no graph manifest documents found");
    }
    const outputs = parseGraphOutputCatalog(JSON.parse(readFileSync(resolve(import.meta.dir, "../../🛂️manifest/📇️outputs.json"), "utf8")), docs.map((doc) => doc.id));
    const byId = new Map(outputs.manifests.map((row) => [row.id, row]));
    const tsSpecifier = (from: string, to: string): string => {
      const path = relative(dirname(from), to).replaceAll("\\", "/").replace(/\.ts$/u, ".js");
      return path.startsWith(".") ? path : `./${path}`;
    };
    const rustModules = docs.map((doc) => {
      const modName = rustModName(doc.id);
      const path = byId.get(doc.id)!.rust;
      artifacts.push({ path: join(outDir, path), content: emitRustManifest(doc) });
      return { modName, path };
    });
    const registryRs =
      `// Generated manifest registry\n\n` +
      rustModules.map((m) => `#[path = "${m.path}"]\npub mod ${m.modName};`).join("\n\n") +
      `\n\nuse crate::manifest::Manifest;\n\npub const MANIFEST_IDS: &[&str] = &[${docs.map((d) => rustStr(d.id)).join(", ")}];\n\n` +
      `pub fn manifest_by_id(id: &str) -> Option<Manifest> {\n    match id {\n` +
      docs.map((d) => `        ${rustStr(d.id)} => Some(${rustModName(d.id)}::${rustFnName(d.id)}()),`).join("\n") +
      `\n        _ => None,\n    }\n}\n`;
    artifacts.push({ path: join(outDir, outputs.shared.rustRegistry), content: registryRs });
    artifacts.push({ path: join(outDir, outputs.shared.jsonSchema), content: emitJsonSchema() });
    const manifestByIdCases = docs.map((d) => `    case ${tsStringLiteral(d.id)}: return ${pascalCase(d.id).toUpperCase()}_MANIFEST_DOCUMENT;`).join("\n");
    const manifestByIdImports = docs.map((d) => `import { ${pascalCase(d.id).toUpperCase()}_MANIFEST_DOCUMENT } from ${JSON.stringify(tsSpecifier(outputs.shared.typescriptIndex, byId.get(d.id)!.typescript))};`).join("\n");
    const tsTypes = `/** Generated graph manifest shared types */\n\nexport interface GraphManifestPropertyDef {\n  readonly name: string;\n  readonly kind: "data" | "derived";\n  readonly valueType?: unknown;\n  readonly expr?: string;\n}\n\nexport interface GraphManifestKindRow {\n  readonly id: string;\n  readonly name?: string;\n  readonly properties?: readonly GraphManifestPropertyDef[];\n  readonly ports?: readonly string[];\n  readonly direction?: string;\n  readonly presentation?: Readonly<Record<string, unknown>>;\n}\n\nexport interface GraphManifestDocument {\n  readonly schema: "manifest";\n  readonly id: string;\n  readonly name?: string;\n  readonly axes?: { readonly portModel?: "normal" | "ported"; readonly directedness?: "directed" | "undirected" };\n  readonly nodeKinds?: readonly GraphManifestKindRow[];\n  readonly edgeKinds?: readonly GraphManifestKindRow[];\n  readonly portKinds?: readonly GraphManifestKindRow[];\n  readonly wireKinds?: readonly GraphManifestKindRow[];\n  readonly layerKinds?: readonly GraphManifestKindRow[];\n  readonly blockKinds?: readonly GraphManifestKindRow[];\n  readonly languageKinds?: readonly GraphManifestKindRow[];\n  readonly surfaceKinds?: readonly GraphManifestKindRow[];\n  readonly windowKinds?: readonly GraphManifestKindRow[];\n  readonly fileNodeKinds?: readonly GraphManifestKindRow[];\n  readonly descriptorKinds?: readonly GraphManifestKindRow[];\n  readonly edgeTips?: readonly Record<string, unknown>[];\n  readonly kindCompatibility?: readonly Record<string, unknown>[];\n}\n\nexport interface HandleKind {\n  readonly color: string;\n  readonly defaultWireKind?: string;\n  readonly id: string;\n  readonly name: string;\n}\n\nexport interface WireKind {\n  readonly defaultEdgeKind?: string;\n  readonly id: string;\n  readonly name: string;\n}\n\nexport interface NodeKindHandleTemplate {\n  readonly handleKind: string;\n  readonly angle: number;\n  readonly radius?: number;\n}\n\nexport interface NodeKind {\n  readonly color?: string;\n  readonly defaultHandleKind?: string;\n  readonly icon?: string;\n  readonly id: string;\n  readonly name: string;\n  readonly stroke?: string;\n  readonly handles?: readonly NodeKindHandleTemplate[];\n}\n\nexport interface EdgeTip {\n  readonly filled?: boolean;\n  readonly geometry?: "arrow" | "fine-arrow" | "diamond" | "circle" | "bar";\n  readonly id: string;\n  readonly scale?: number;\n}\n\nexport interface EdgeKind {\n  readonly color?: string;\n  readonly directed?: boolean;\n  readonly id: string;\n  readonly name: string;\n  readonly pattern?: string;\n  readonly shape?: "bezier" | "line";\n  readonly sourceTip?: string;\n  readonly stroke?: string;\n  readonly targetTip?: string;\n}\n\nexport interface KindCatalogBundle {\n  readonly edgeTips?: readonly EdgeTip[];\n  readonly edges?: readonly EdgeKind[];\n  readonly handles?: readonly HandleKind[];\n  readonly nodes?: readonly NodeKind[];\n  readonly wires?: readonly WireKind[];\n}\n\nexport const MANIFEST_IDS = [${docs.map((d) => tsStringLiteral(d.id)).join(", ")}] as const;\nexport type ManifestId = (typeof MANIFEST_IDS)[number];\n\nexport function mergeManifestCatalogBundles(...bundles: readonly KindCatalogBundle[]): KindCatalogBundle {\n  function mergedSlice<T extends { id: string }>(slices: readonly (readonly T[] | undefined)[]): readonly T[] | undefined {\n    const byId = new Map<string, T>();\n    let any = false;\n    for (const slice of slices) {\n      if (!slice) continue;\n      any = true;\n      for (const row of slice) {\n        byId.set(row.id, row);\n      }\n    }\n    if (!any) return undefined;\n    return [...byId.values()].sort((left, right) => left.id.localeCompare(right.id));\n  }\n  return {\n    edgeTips: mergedSlice(bundles.map((bundle) => bundle.edgeTips)),\n    edges: mergedSlice(bundles.map((bundle) => bundle.edges)),\n    handles: mergedSlice(bundles.map((bundle) => bundle.handles)),\n    nodes: mergedSlice(bundles.map((bundle) => bundle.nodes)),\n    wires: mergedSlice(bundles.map((bundle) => bundle.wires)),\n  };\n}\n`;
    artifacts.push({ path: join(outDir, outputs.shared.typescriptTypes), content: tsTypes });
    artifacts.push({
      path: join(outDir, outputs.shared.typescriptIndex),
      content: `export * from ${JSON.stringify(tsSpecifier(outputs.shared.typescriptIndex, outputs.shared.typescriptTypes))};\n` +
        docs.map((d) => `export * from ${JSON.stringify(tsSpecifier(outputs.shared.typescriptIndex, byId.get(d.id)!.typescript))};`).join("\n") +
        `\n\n${manifestByIdImports}\nimport type { GraphManifestDocument } from ${JSON.stringify(tsSpecifier(outputs.shared.typescriptIndex, outputs.shared.typescriptTypes))};\n\nexport function manifestById(id: string): GraphManifestDocument | undefined {\n  switch (id) {\n${manifestByIdCases}\n    default: return undefined;\n  }\n}\n`,
    });
    for (const doc of docs) {
      const path = byId.get(doc.id)!.typescript;
      artifacts.push({ path: join(outDir, path), content: emitTsManifest(doc, tsSpecifier(path, outputs.shared.typescriptTypes)) });
    }
    return { artifacts: artifacts.sort((left, right) => left.path.localeCompare(right.path)), manifestCount: docs.length };
}

type GraphOutputNode = Readonly<{ path: string; nodeKind: "file" | "directory" }>;

/** 🌳️Reads the exact no-follow output inventory, including nested manifest owners. */
export function graphOutputInventory(outDir: string): readonly GraphOutputNode[] {
  if (!existsSync(outDir)) return [];
  if (!lstatSync(outDir).isDirectory()) throw new Error("graph output root must be a real directory");
  const nodes: GraphOutputNode[] = [];
  function visit(dir: string): void {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (!entry.isDirectory() && !entry.isFile()) throw new Error(`graph output contains an unsupported entry: ${entry.name}`);
      const absolute = join(dir, entry.name);
      nodes.push({ path: relative(outDir, absolute).replaceAll("\\", "/"), nodeKind: entry.isDirectory() ? "directory" : "file" });
      if (entry.isDirectory()) visit(absolute);
    }
  }
  visit(outDir);
  return nodes.sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}

/** 🗺️Derives parent directories solely from already explicit rendered output paths. */
function graphOutputNodes(outDir: string, artifacts: readonly GraphArtifact[]): readonly GraphOutputNode[] {
  const nodes = new Map<string, "file" | "directory">();
  for (const artifact of artifacts) {
    const path = relative(resolve(outDir), resolve(artifact.path)).replaceAll("\\", "/");
    if (!path || path.startsWith("/") || /^[A-Za-z]:/u.test(path) || path.split("/").some((part) => part === ".." || part === ".") || nodes.has(path)) throw new Error("graph output artifact is duplicated or outside its owner");
    nodes.set(path, "file");
    let parent = dirname(path).replaceAll("\\", "/");
    while (parent !== ".") {
      if (nodes.get(parent) === "file") throw new Error("graph output file conflicts with a directory");
      nodes.set(parent, "directory");
      parent = dirname(parent).replaceAll("\\", "/");
    }
  }
  return [...nodes].map(([path, nodeKind]) => ({ path, nodeKind })).sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}

/** 🧹️Writes the exact nested set after preflight; removes stale leaves and only empty directories. */
export function writeGraphArtifacts(outDir: string, artifacts: readonly GraphArtifact[]): void {
  const expected = graphOutputNodes(outDir, artifacts);
  const actual = graphOutputInventory(outDir);
  const kinds = new Map(expected.map((entry) => [entry.path, entry.nodeKind]));
  const stale = actual.filter((entry) => kinds.get(entry.path) !== entry.nodeKind);
  mkdirSync(outDir, { recursive: true });
  for (const entry of stale.filter((entry) => entry.nodeKind === "file")) unlinkSync(join(outDir, entry.path));
  for (const entry of stale.filter((entry) => entry.nodeKind === "directory").sort((left, right) => right.path.length - left.path.length)) rmdirSync(join(outDir, entry.path));
  for (const entry of expected.filter((entry) => entry.nodeKind === "directory")) mkdirSync(join(outDir, entry.path), { recursive: true });
  for (const artifact of artifacts) writeFileSync(artifact.path, artifact.content, "utf8");
}

class GenerateScript extends BundleScript {
  run(): void {
    const root = getWorkspaceRoot();
    const outDir = join(this.root, "..", "..", "🤖️generated");
    const rendered = renderGraphArtifacts(root, outDir);
    writeGraphArtifacts(outDir, rendered.artifacts);
    console.log(`[framework-graph] wrote ${rendered.manifestCount} manifests to ${relative(root, outDir)}`);
  }
}

/** 🧾️Emits exact graph bytes, nested owners, and stale removals without writing the output root. */
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const root = getWorkspaceRoot();
    const outDir = join(this.root, "..", "..", "🤖️generated");
    const rendered = renderGraphArtifacts(root, outDir, false);
    const rootPath = relative(root, outDir).replaceAll("\\", "/").normalize("NFC");
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      ...graphOutputNodes(outDir, rendered.artifacts).filter((entry) => entry.nodeKind === "directory").map((entry) => ({ bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: `${rootPath}/${entry.path}` })),
      ...rendered.artifacts.map((artifact) => ({ bytesBase64: Buffer.from(artifact.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(root, artifact.path).replaceAll("\\", "/").normalize("NFC") })),
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const expected = new Map(graphOutputNodes(outDir, rendered.artifacts).map((entry) => [entry.path, entry.nodeKind]));
    const staleRemovals = graphOutputInventory(outDir)
      .filter((entry) => expected.get(entry.path) !== entry.nodeKind)
      .map((entry) => `${rootPath}/${entry.path.normalize("NFC")}`)
      .sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
    process.stdout.write(`${JSON.stringify({ contractId: "graph-catalog", nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}

/** @emoji ✅️ Checks exact graph catalog bytes and output membership without rewriting artifacts. */
class CheckGeneratedScript extends BundleScript {
  run(): void {
    const root = getWorkspaceRoot();
    const outDir = join(this.root, "..", "..", "🤖️generated");
    const rendered = renderGraphArtifacts(root, outDir);
    const expected = graphOutputNodes(outDir, rendered.artifacts);
    const actual = graphOutputInventory(outDir);
    const stale = rendered.artifacts.filter((artifact) => !existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content).map((artifact) => basename(artifact.path));
    if (JSON.stringify(actual) !== JSON.stringify(expected) || stale.length > 0) throw new Error(`framework-graph generated catalog is stale: membership=${JSON.stringify(actual) !== JSON.stringify(expected)}, files=${JSON.stringify(stale)}`);
    runCmd("bun", ["test", resolve(this.root, "../../🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 60_000 });
    console.log(`[framework-graph] ${rendered.manifestCount} generated manifests are fresh`);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCmd("bun", ["test", resolve(this.root, "../../🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
    runCargoTestBudgeted(["semio-framework-graph"], this.repoRoot, rest);
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-graph --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework-graph"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check-generated", CheckGeneratedScript).register("test", TestScript).register("lint", LintScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
