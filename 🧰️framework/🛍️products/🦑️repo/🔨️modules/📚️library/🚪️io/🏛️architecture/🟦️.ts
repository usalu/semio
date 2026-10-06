import { posix } from "node:path";
import { rustStringValue, rustTokens } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { inspectRustModuleGraphFacts, loadCatalogTaxonomy, type Taxonomy } from "../../🔍️discovery/🟦️.ts";
import { policyListPluginArtifactDirs } from "../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts";
import { policyReadFileSafe, policyReaddirSafe } from "../../🔍️discovery/📖️source-access/🟦️.ts";

export type ArtifactIoArchitectureBreach = Readonly<{ id: string; summary: string; kind: string; scope: string; priority: "high"; reason: string; solution: string }>;
export type ArtifactIoArchitectureOptions = Readonly<{ cancelled?: () => boolean; progress?: (event: Readonly<{ phase: "inventory" | "checking" | "complete"; path: string }>) => void }>;

function artifactIoCheckpoint(options: ArtifactIoArchitectureOptions, phase: "inventory" | "checking" | "complete", path: string): void {
  if (options.cancelled?.()) throw Error("Artifact I/O architecture verification cancelled");
  options.progress?.({ phase, path });
}

const codecTraits = new Set(["ArtifactDsl", "ArtifactPack", "ArtifactSqliteSnapshot", "OpText", "OpBinary", "DiffText", "DiffBinary", "DiffCodec", "PayloadCodec", "ArtifactCodec"]);

/** 🧠️ Identifies semantic mutation and diff declarations misplaced in a physical representation. */
export function semanticArtifactIoItems(source: string): string[] {
  const code = rustTokens(source).filter(token => token.kind !== "string").map(token => token.text).join(" "), names = new Set<string>();
  for (const match of code.matchAll(/\bimpl\b[^{};]*?\b(Mutation|MutationDiff|DiffAlgebra)\b[^{};]*\bfor\b/gu)) names.add(match[1]!);
  for (const match of code.matchAll(/\bfn\s+(apply_to_artifact|diff_\w+)\s*[^{};]*/gu)) {
    if (match[1] === "apply_to_artifact" || /->[^{};]*\b\w+Diff\b/u.test(match[0])) names.add(match[1]!);
  }
  return [...names].sort();
}

/** 🧵️ Finds physical TypeScript APIs through the platform parser without interpreting comments or literals. */
export function schemaTypeScriptWireSymbols(source: string): string[] {
  return new Bun.Transpiler({loader: "ts"}).scan(source).exports.filter(name => /Json(?:Text|Value|Projection)?$|^decode.*Protobuf$|^write(?:Value|Record)Json$|^floatLexeme$|^jsonGeometry$|^TxtProtobuf|^txtProtobuf(?:Key|String)$|^decodeRemodeling(?:Snapshot|Diff|Mutation|Artifact)$/u.test(name)).sort();
}

/** 🧭️ Reads actual TypeScript module dependencies, including type-only declarations. */
function schemaTypeScriptModules(source:string):string[]{
  const admitted=source.replace(/\b(import|export)(\s+)type\s+/gu,"$1$2").replace(/\b(?:import|export)\s*\{[^}]*\}\s*from(?=\s*["'])/gu,declaration=>declaration.replace(/\btype\s+(?=\w)/gu,""));
  const modules=new Bun.Transpiler({loader:"ts"}).scan(admitted).imports.map(row=>{
    if([...row.path].some(scalar=>scalar.charCodeAt(0)>255))return row.path;
    try{return new TextDecoder("utf-8",{fatal:true}).decode(Uint8Array.from(row.path,scalar=>scalar.charCodeAt(0)));}catch{return row.path;}
  });
  return [...new Set(modules)];
}

/** 🔺️ Requires a semantic diff's text and binary implementations in their separate I/O owners. */
export function missingArtifactDiffWireTypes(semantic: string, text: string, binary: string): string[] {
  const code = (source: string): string => rustTokens(source).filter(token => token.kind !== "string").map(token => token.text).join(" ");
  const name = (path: string): string => path.replace(/\s/gu, "").split("::").at(-1)!;
  const declared = [...code(semantic).matchAll(/\bimpl\b[^{};]*?\bMutationDiff\s*<[^{};]+>\s+for\s+((?:\w+\s*::\s*)*\w+)/gu)].map(match => name(match[1]!));
  const represented = (source: string, trait: string, macro: string): Set<string> => {
    const tokens = code(source);
    const implementations = [...tokens.matchAll(new RegExp(`\\bimpl\\b[^{};]*?\\b${trait}\\s+for\\s+((?:\\w+\\s*::\\s*)*\\w+)`, "gu"))];
    const generated = [...tokens.matchAll(new RegExp(`\\b${macro}\\s*!\\s*\\(\\s*((?:\\w+\\s*::\\s*)*\\w+)`, "gu"))];
    return new Set([...implementations, ...generated].map(match => name(match[1]!)));
  };
  const texts = represented(text, "DiffText", "diff_text"), binaries = represented(binary, "DiffBinary", "diff_binary");
  return [...new Set(declared)].filter(type => !texts.has(type) || !binaries.has(type)).sort();
}

/** 🗿️ Finds framework artifact owners without treating fixtures or external packages as production. */
function frameworkArtifactRoots(repoRoot: string, options: ArtifactIoArchitectureOptions): string[] {
  const roots: string[] = [];
  const walk = (path: string): void => {
    artifactIoCheckpoint(options, "inventory", path);
    for (const entry of policyReaddirSafe(repoRoot, path).filter(entry => entry.isDirectory && !["🧪️tests", "🧫️fixtures", "📚️examples", "📦️packages", "🔮️oracles"].includes(entry.name))) {
      const child = `${path}/${entry.name}`;
      if (entry.name === "🗿️artifacts") roots.push(...policyReaddirSafe(repoRoot, child).filter(entry => entry.isDirectory).map(entry => `${child}/${entry.name}`));
      else walk(child);
    }
  };
  walk("🧰️framework");
  return roots;
}

/** 🚪️ Checks native wire ownership without following symbolic links or scanning unrelated runtime owners. */
export function artifactIoArchitectureBreaches(repoRoot: string, roots: readonly string[] | undefined = undefined, taxonomy: Taxonomy = loadCatalogTaxonomy(), options: ArtifactIoArchitectureOptions = {}): ArtifactIoArchitectureBreach[] {
  artifactIoCheckpoint(options, "inventory", repoRoot);
  const owners = roots ?? [...policyListPluginArtifactDirs(repoRoot), ...frameworkArtifactRoots(repoRoot, options)];
  const breaches: ArtifactIoArchitectureBreach[] = [];
  const add = (path: string, rule: string, reason: string): void => {
    breaches.push({ id: `artifact-io-${rule}-${path}`, summary: `"${path}" violates artifact I/O ownership`, kind: `artifact-io/${rule}`, scope: path, priority: "high", reason, solution: "Keep semantic contracts under schema and wire codecs under 🚪️io/<representation>/<semantic-facet>/[member]." });
  };
  const inspectSchemaSource = (path: string): void => {
    if (!/\.(?:rs|ts)$/u.test(path)) return;
    artifactIoCheckpoint(options, "checking", path);
    const source = policyReadFileSafe(repoRoot, path);
    if (path.endsWith(".ts")) {
      let modules:string[]=[];
      try {
        const symbols = schemaTypeScriptWireSymbols(source);
        if (symbols.length) add(path, "schema-codec", `Physical TypeScript codec APIs belong to I/O: ${symbols.join(", ")}.`);
        modules=schemaTypeScriptModules(source);
      } catch (error) { add(path, "schema-source", `Semantic TypeScript source is invalid: ${String(error)}.`); }
      for (const module of modules) {
        const target = posix.normalize(posix.join(posix.dirname(path), module)), parts = target.split("/"), at = parts.lastIndexOf("🚪️io");
        if (at >= 0 && [...taxonomy.representationDirs, "🪶️sqlite-snapshot"].includes(parts[at + 1] ?? "")) add(path, "schema-codec-dependency", "Canonical schema must depend on semantic value interfaces; physical decoding belongs to I/O.");
      }
      for (const match of source.matchAll(/^\s*export\s*(?!type\b)(?:\*|\{[^}]*\})\s*from\s*["']([^"']+)["']/gm)) {
        if(!modules.includes(match[1]!))continue;
        const target = posix.normalize(posix.join(posix.dirname(path), match[1]!));
        const parts = target.split("/"), at = parts.lastIndexOf("🚪️io");
        if (at >= 0 && taxonomy.representationDirs.includes(parts[at + 1] ?? "")) add(path, "schema-codec-alias", "Semantic TypeScript barrels must not publish native wire codec APIs.");
      }
      return;
    }
    if (!path.endsWith(".rs")) return;
    const tokens = rustTokens(source);
    const code = tokens.filter(token => token.kind !== "string").map(token => token.text).join(" ");
    if (/\bimpl\b[^{};]*\b(?:ArtifactDsl|ArtifactPack|ArtifactSqliteSnapshot|OpText|OpBinary|DiffText|DiffBinary|DiffCodec|PayloadCodec|ArtifactCodec)\b[^{};]*\bfor\b/u.test(code) || /\bdiff_(?:text|binary)\s*!/u.test(code)) add(path, "schema-codec", "Wire codec implementations must be owned by artifact I/O.");
    if (tokens.some((token, index) => token.text === "trait" && codecTraits.has(tokens[index + 1]?.text ?? ""))) add(path, "schema-codec-contract", "Wire codec interfaces belong to framework I/O, separate from semantic mutation contracts.");
    for (const [index, token] of tokens.entries()) {
      if (token.text === "path" && tokens[index + 1]?.text === "=") {
        const target = rustStringValue(tokens[index + 2]);
        if (target && posix.normalize(posix.join(posix.dirname(path), target)).split("/").includes("🚪️io")) add(path, "schema-codec-mount", "Semantic schema modules must not mount I/O source through module aliases.");
      }
    }
    const alias = /\bpub\s+mod\s+(?:binary|text|dsl|pack|sqlite)\b/u.test(code)
      || /\bpub\s+use\b[^;{}]*\b(?:ArtifactDsl|ArtifactPack|ArtifactSqliteSnapshot|OpText|OpBinary|DiffText|DiffBinary|DiffCodec|PayloadCodec|ArtifactCodec)\b/u.test(code)
      || /\bpub\s+use\b[^;{}]*\bio\s*::/u.test(code)
      || /\bpub\s+use\b[^;{}]*\bschema\s*::\s*(?:snapshot|diff|mutations|inferences)\s*::\s*(?:binary|text|dsl|pack|sqlite)\b/u.test(code)
      || /\bpub\s+use\s+(?:(?:self|super)\s*::\s*)?(?:binary|text|dsl|pack|sqlite)\s*::/u.test(code);
    if (alias) add(path, "schema-codec-alias", "Semantic schema roots must not publish wire representation aliases.");
  };
  type DiffOwnership = {semantic: string[]; text: string[]; binary: string[]};
  const walkFacet = (root: string, facetPath: string, coverage: DiffOwnership): void => {
    artifactIoCheckpoint(options, "checking", root);
    for (const entry of policyReaddirSafe(repoRoot, root)) {
      if (["🧪️tests", "🧫️fixtures", "📚️examples", "🔮️oracles"].includes(entry.name)) continue;
      const path = `${root}/${entry.name}`;
      if (entry.isDirectory) {
        const nested = `${facetPath}/${entry.name}`;
        const schema = facetPath.startsWith("🧬️schema"), parts = nested.split("/");
        const native = taxonomy.representationDirs.includes(parts[1] ?? "");
        const forbidden = schema ? taxonomy.representationDirs.includes(entry.name) : parts.length === 2 ? taxonomy.schemaChildDirs.includes(entry.name) : native && parts.length === 3 ? !taxonomy.ioSemanticCollectionDirNames.includes(entry.name) && entry.name !== "🧪️tests" && entry.name !== "🧫️fixtures" : native && parts.length > 3 && taxonomy.representationDirs.includes(entry.name);
        if (forbidden) { add(path, "facet-path", `Undeclared semantic/I/O facet path: ${nested}.`); continue; }
        walkFacet(path, nested, coverage);
      } else if (facetPath.startsWith("🧬️schema")) {
        if (/\.(?:grammar\.semio|protocol\.semio|ebnf|abnf|g4|ksy|spicy|sql)$/u.test(entry.name)) add(path, "schema-wire-spec", "Wire grammar and binary protocol specifications belong to I/O.");
        inspectSchemaSource(path);
        if (entry.name.endsWith(".rs")) coverage.semantic.push(policyReadFileSafe(repoRoot, path));
      } else if (entry.name.endsWith(".rs") && taxonomy.representationDirs.includes(facetPath.split("/")[1] ?? "")) {
        const source = policyReadFileSafe(repoRoot, path), semantic = semanticArtifactIoItems(source);
        if (facetPath.split("/")[1] === "📝️text") coverage.text.push(source);
        if (facetPath.split("/")[1] === "💾️binary") coverage.binary.push(source);
        if (semantic.length) add(path, "io-semantic-implementation", `Mutation application and diff construction belong to schema: ${semantic.join(", ")}.`);
      }
    }
  };
  const walkOwners = (root: string): void => {
    artifactIoCheckpoint(options, "checking", root);
    const coverage: DiffOwnership = {semantic: [], text: [], binary: []};
    for (const entry of policyReaddirSafe(repoRoot, root)) {
      const path = `${root}/${entry.name}`;
      if (!entry.isDirectory) {
        if (entry.name === "🦀️.rs") {
          for (const module of inspectRustModuleGraphFacts(policyReadFileSafe(repoRoot, path)).modules) {
            if (!module.conditional && module.modulePath.includes("schema") && module.pathTarget?.split("/").includes("🚪️io")) add(path, "schema-codec-mount", `Owner assembly mounts I/O source in semantic namespace ${module.modulePath.join("::")}.`);
          }
        }
        continue;
      }
      if (entry.name === "🧬️schema" || entry.name === "🚪️io") walkFacet(path, entry.name, coverage);
      else if (entry.name === taxonomy.standardsDirName || entry.name === taxonomy.subsetsDirName || root.split("/").at(-1) === taxonomy.standardsDirName || root.split("/").at(-1) === taxonomy.subsetsDirName) walkOwners(path);
    }
    const missing = missingArtifactDiffWireTypes(coverage.semantic.join("\n"), coverage.text.join("\n"), coverage.binary.join("\n"));
    if (missing.length) add(root, "diff-completeness", `Semantic diffs require separate text and binary codecs: ${missing.join(", ")}.`);
  };
  owners.forEach(walkOwners);
  artifactIoCheckpoint(options, "complete", repoRoot);
  return breaches;
}
