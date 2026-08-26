#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-graph` — the semio graph crate: graph-manifest codegen, cargo test and clippy gates. */
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { BundleScript, getWorkspaceRoot, ScriptRouter, runBundleScriptMain, runCargoLint, runCargoTestBudgeted, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { loadTaxonomy, pathIsExcluded } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

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
        // 🏷️ A manifest source is tagged by its filename, not by living in a directory named "manifest" — the
        // `🗿️artifacts/<component>/🛂️manifest.json` taxonomy (one manifest per artifact, no descriptor needed)
        // sits directly under the component's own artifact folder with no "manifest"-named parent directory at
        // all. Requiring `path.includes("/manifest/")` silently dropped every manifest fixture that migrated to
        // that layout (flow-dag, writer-languages, note-blocks, wires) out of codegen with no error — the import
        // just dangled. Matching on the `🛂️manifest.json` filename prefix alone (bare for a single manifest per
        // directory, or suffixed with `<descriptor>.manifest.json` to disambiguate multiple in one directory,
        // e.g. `🛂️manifest.jsonnakagin.manifest.json`) is the actual invariant and needs no directory convention.
      } else if (name.startsWith("🛂️manifest.json") && name.endsWith(".json")) {
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

function rustModuleFileName(modName: string): string {
  return `🦀️${modName}.rs`;
}

function tsManifestFileName(id: string): string {
  return `🟦️${id}.ts`;
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
    body += `    #[serde(rename = ${rustStr(id)})]\n    ${variant},\n`;
    consts += `pub const ${prefix.toUpperCase()}_${familyName.toUpperCase()}_${snakeUpper(id)}: &str = ${rustStr(id)};\n`;
  }
  return (
    `${consts}\n` +
    `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]\n` +
    `#[serde(rename_all = "camelCase")]\n` +
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
  // families only reference Serialize/Deserialize when at least one recognized kind family is present — an unconditional import would leave manifests with no recognized family (e.g. note-blocks' blockKinds) with an unused import.
  const serdeImport = families.length > 0 ? "use serde::{Deserialize, Serialize};\n" : "";
  let out = `// Generated from ${doc.id}.manifest.json\n\n${serdeImport}use crate::manifest::Manifest;\n\n`;
  out += families;
  out += `pub const ${prefix.toUpperCase()}_MANIFEST_JSON: &str = ${rustStr(json)};\n\n`;
  out += `pub fn ${fnName}() -> Manifest {\n    serde_json::from_str(${prefix.toUpperCase()}_MANIFEST_JSON).expect("manifest json")\n}\n`;
  return out;
}

function emitTsManifest(doc: ManifestDocument): string {
  const prefix = pascalCase(doc.id);
  let out = `// Generated from ${doc.id}.manifest.json\n\nimport type { GraphManifestDocument, KindCatalogBundle } from "./🟦️types.ts";\n\n`;
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
function renderGraphArtifacts(root: string, outDir: string, log = true): { artifacts: readonly GraphArtifact[]; manifestCount: number } {
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
    const rustModules = docs.map((doc) => {
      const modName = rustModName(doc.id);
      artifacts.push({ path: join(outDir, rustModuleFileName(modName)), content: emitRustManifest(doc) });
      return modName;
    });
    const registryRs =
      `// Generated manifest registry\n\n` +
      rustModules.map((m) => `#[path = "${rustModuleFileName(m)}"]\npub mod ${m};`).join("\n\n") +
      `\n\nuse crate::manifest::Manifest;\n\npub const MANIFEST_IDS: &[&str] = &[${docs.map((d) => rustStr(d.id)).join(", ")}];\n\n` +
      `pub fn manifest_by_id(id: &str) -> Option<Manifest> {\n    match id {\n` +
      docs.map((d) => `        ${rustStr(d.id)} => Some(${rustModName(d.id)}::${rustFnName(d.id)}()),`).join("\n") +
      `\n        _ => None,\n    }\n}\n`;
    artifacts.push({ path: join(outDir, "🦀️registry.rs"), content: registryRs });
    artifacts.push({ path: join(outDir, "🔣️manifest.schema.json"), content: emitJsonSchema() });
    const manifestByIdCases = docs.map((d) => `    case ${tsStringLiteral(d.id)}: return ${pascalCase(d.id).toUpperCase()}_MANIFEST_DOCUMENT;`).join("\n");
    const manifestByIdImports = docs.map((d) => `import { ${pascalCase(d.id).toUpperCase()}_MANIFEST_DOCUMENT } from "./${tsManifestFileName(d.id).replace(/\.ts$/, ".js")}";`).join("\n");
    const tsTypes = `/** Generated graph manifest shared types */\n\nexport interface GraphManifestPropertyDef {\n  readonly name: string;\n  readonly kind: "data" | "derived";\n  readonly valueType?: unknown;\n  readonly expr?: string;\n}\n\nexport interface GraphManifestKindRow {\n  readonly id: string;\n  readonly name?: string;\n  readonly properties?: readonly GraphManifestPropertyDef[];\n  readonly ports?: readonly string[];\n  readonly direction?: string;\n  readonly presentation?: Readonly<Record<string, unknown>>;\n}\n\nexport interface GraphManifestDocument {\n  readonly schema: "manifest";\n  readonly id: string;\n  readonly name?: string;\n  readonly axes?: { readonly portModel?: "normal" | "ported"; readonly directedness?: "directed" | "undirected" };\n  readonly nodeKinds?: readonly GraphManifestKindRow[];\n  readonly edgeKinds?: readonly GraphManifestKindRow[];\n  readonly portKinds?: readonly GraphManifestKindRow[];\n  readonly wireKinds?: readonly GraphManifestKindRow[];\n  readonly layerKinds?: readonly GraphManifestKindRow[];\n  readonly blockKinds?: readonly GraphManifestKindRow[];\n  readonly languageKinds?: readonly GraphManifestKindRow[];\n  readonly surfaceKinds?: readonly GraphManifestKindRow[];\n  readonly windowKinds?: readonly GraphManifestKindRow[];\n  readonly fileNodeKinds?: readonly GraphManifestKindRow[];\n  readonly descriptorKinds?: readonly GraphManifestKindRow[];\n  readonly edgeTips?: readonly Record<string, unknown>[];\n  readonly kindCompatibility?: readonly Record<string, unknown>[];\n}\n\nexport interface HandleKind {\n  readonly color: string;\n  readonly defaultWireKind?: string;\n  readonly id: string;\n  readonly name: string;\n}\n\nexport interface WireKind {\n  readonly defaultEdgeKind?: string;\n  readonly id: string;\n  readonly name: string;\n}\n\nexport interface NodeKindHandleTemplate {\n  readonly handleKind: string;\n  readonly angle: number;\n  readonly radius?: number;\n}\n\nexport interface NodeKind {\n  readonly color?: string;\n  readonly defaultHandleKind?: string;\n  readonly icon?: string;\n  readonly id: string;\n  readonly name: string;\n  readonly stroke?: string;\n  readonly handles?: readonly NodeKindHandleTemplate[];\n}\n\nexport interface EdgeTip {\n  readonly filled?: boolean;\n  readonly geometry?: "arrow" | "fine-arrow" | "diamond" | "circle" | "bar";\n  readonly id: string;\n  readonly scale?: number;\n}\n\nexport interface EdgeKind {\n  readonly color?: string;\n  readonly directed?: boolean;\n  readonly id: string;\n  readonly name: string;\n  readonly pattern?: string;\n  readonly shape?: "bezier" | "line";\n  readonly sourceTip?: string;\n  readonly stroke?: string;\n  readonly targetTip?: string;\n}\n\nexport interface KindCatalogBundle {\n  readonly edgeTips?: readonly EdgeTip[];\n  readonly edges?: readonly EdgeKind[];\n  readonly handles?: readonly HandleKind[];\n  readonly nodes?: readonly NodeKind[];\n  readonly wires?: readonly WireKind[];\n}\n\nexport const MANIFEST_IDS = [${docs.map((d) => tsStringLiteral(d.id)).join(", ")}] as const;\nexport type ManifestId = (typeof MANIFEST_IDS)[number];\n\nexport function mergeManifestCatalogBundles(...bundles: readonly KindCatalogBundle[]): KindCatalogBundle {\n  function mergedSlice<T extends { id: string }>(slices: readonly (readonly T[] | undefined)[]): readonly T[] | undefined {\n    const byId = new Map<string, T>();\n    let any = false;\n    for (const slice of slices) {\n      if (!slice) continue;\n      any = true;\n      for (const row of slice) {\n        byId.set(row.id, row);\n      }\n    }\n    if (!any) return undefined;\n    return [...byId.values()].sort((left, right) => left.id.localeCompare(right.id));\n  }\n  return {\n    edgeTips: mergedSlice(bundles.map((bundle) => bundle.edgeTips)),\n    edges: mergedSlice(bundles.map((bundle) => bundle.edges)),\n    handles: mergedSlice(bundles.map((bundle) => bundle.handles)),\n    nodes: mergedSlice(bundles.map((bundle) => bundle.nodes)),\n    wires: mergedSlice(bundles.map((bundle) => bundle.wires)),\n  };\n}\n`;
    artifacts.push({ path: join(outDir, "🟦️types.ts"), content: tsTypes });
    artifacts.push({
      path: join(outDir, "📦️index.ts"),
      content: `export * from "./🟦️types.js";\n` +
        docs.map((d) => `export * from "./${tsManifestFileName(d.id).replace(/\.ts$/, ".js")}";`).join("\n") +
        `\n\n${manifestByIdImports}\nimport type { GraphManifestDocument } from "./🟦️types.js";\n\nexport function manifestById(id: string): GraphManifestDocument | undefined {\n  switch (id) {\n${manifestByIdCases}\n    default: return undefined;\n  }\n}\n`,
    });
    for (const doc of docs) {
      artifacts.push({ path: join(outDir, tsManifestFileName(doc.id)), content: emitTsManifest(doc) });
    }
    return { artifacts: artifacts.sort((left, right) => left.path.localeCompare(right.path)), manifestCount: docs.length };
}

/** @emoji 🧹️ Writes the exact rendered set and removes only stale files inside its owned output root. */
function writeGraphArtifacts(outDir: string, artifacts: readonly GraphArtifact[]): void {
  mkdirSync(outDir, { recursive: true });
  const expected = new Set(artifacts.map((artifact) => basename(artifact.path)));
  for (const name of readdirSync(outDir)) if (!expected.has(name)) rmSync(join(outDir, name), { recursive: true });
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

/** 🧾️ Emits exact graph bytes and stale top-level removals without writing the output root. */
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const root = getWorkspaceRoot();
    const outDir = join(this.root, "..", "..", "🤖️generated");
    const rendered = renderGraphArtifacts(root, outDir, false);
    const rootPath = relative(root, outDir).replaceAll("\\", "/").normalize("NFC");
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      ...rendered.artifacts.map((artifact) => ({ bytesBase64: Buffer.from(artifact.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(root, artifact.path).replaceAll("\\", "/").normalize("NFC") })),
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const expected = new Set(rendered.artifacts.map((artifact) => basename(artifact.path)));
    const staleRemovals = (existsSync(outDir) ? readdirSync(outDir) : [])
      .filter((name) => !expected.has(name))
      .map((name) => `${rootPath}/${name.normalize("NFC")}`)
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
    const expected = rendered.artifacts.map((artifact) => basename(artifact.path)).sort();
    const actual = existsSync(outDir) ? readdirSync(outDir).sort() : [];
    const stale = rendered.artifacts.filter((artifact) => !existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content).map((artifact) => basename(artifact.path));
    if (JSON.stringify(actual) !== JSON.stringify(expected) || stale.length > 0) throw new Error(`framework-graph generated catalog is stale: membership=${JSON.stringify(actual) !== JSON.stringify(expected)}, files=${JSON.stringify(stale)}`);
    console.log(`[framework-graph] ${rendered.manifestCount} generated manifests are fresh`);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
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

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
