/** 📽️ Rust and TypeScript projections of admitted graph manifests. */
import { dirname, join, relative } from "node:path";
import { readGraphManifestDocuments, type ManifestDocument, type ManifestKindRow } from "../📥️admission/🟦️.ts";
import { readGraphOutputCatalog } from "../📇️catalog/🟦️.ts";

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

export type GraphArtifact = { path: string; content: string };

/** @emoji 🧾️ Renders the full graph catalog from lexically admitted manifest inputs without writes. */
export function renderGraphArtifacts(root: string, outDir: string, log = true, pluginAreas?: readonly string[]): { artifacts: readonly GraphArtifact[]; manifestCount: number } {
    const artifacts: GraphArtifact[] = [];
    const docs = readGraphManifestDocuments(root, log, pluginAreas);
    const outputs = readGraphOutputCatalog(docs.map((doc) => doc.id));
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
      rustModules.map((m) => `#[path = "${relative(dirname(outputs.shared.rustRegistry), m.path).replaceAll("\\", "/")}"]\npub mod ${m.modName};`).join("\n\n") +
      `\n\nuse crate::manifest::Manifest;\n\npub const MANIFEST_IDS: &[&str] = &[${docs.map((d) => rustStr(d.id)).join(", ")}];\n\n` +
      `pub fn manifest_by_id(id: &str) -> Option<Manifest> {\n    match id {\n` +
      docs.map((d) => `        ${rustStr(d.id)} => Some(${rustModName(d.id)}::${rustFnName(d.id)}()),`).join("\n") +
      `\n        _ => None,\n    }\n}\n`;
    artifacts.push({ path: join(outDir, outputs.shared.rustRegistry), content: registryRs });
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
