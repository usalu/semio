import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import picomatch from "picomatch";
import ts from "typescript";
import { parse } from "jsonc-parser";
import { artifactFacetPathIsDeclared, loadCatalogTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { newScaffoldIoTree, newScaffoldSubsetTree } from "../../🏗️authoring/🗿️artifact-tree/🟦️.ts";
import { artifactIoArchitectureBreaches, missingArtifactDiffWireTypes, semanticArtifactIoItems, schemaTypeScriptWireSymbols } from "../../🚪️io/🏛️architecture/🟦️.ts";
import Parser from "web-tree-sitter";
import type { ArtifactScaffoldLeaf } from "../../🏗️builder/🟦️.ts";

const library = resolve(import.meta.dir, "../..");
const vector = JSON.parse(readFileSync(join(library, "🧫️fixtures/🚪️artifact-io-ownership/🔣️.json"), "utf8")) as { accepted: string[]; rejected: string[] };
const schema = JSON.parse(readFileSync(join(library, "🧬️schema/🚪️artifact-io-ownership/🔣️.json"), "utf8"));
const taxonomy = loadCatalogTaxonomy();

test("framework products with paired snapshot schema and IO are architecture owners", () => {
  const fixture = JSON.parse(readFileSync(join(library, "🧫️fixtures/🚪️artifact-io-framework-owner-roots/🔣️.json"), "utf8")) as {owners: {path: string; schema: boolean; io: boolean; snapshot: boolean; checked: boolean}[]; semanticSource: string; physicalSource: string};
  expect(new Ajv({strict: true}).compile({type:"object",required:["owners","semanticSource","physicalSource"],additionalProperties:false,properties:{semanticSource:{type:"string"},physicalSource:{type:"string"},owners:{type:"array",items:{type:"object",required:["path","schema","io","snapshot","checked"],additionalProperties:false,properties:{path:{type:"string"},schema:{type:"boolean"},io:{type:"boolean"},snapshot:{type:"boolean"},checked:{type:"boolean"}}}}}})(fixture)).toBe(true);
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(output, {recursive:true});
  const root = mkdtempSync(join(output, "framework-owner-discovery-"));
  const ignored = picomatch("**/{🧪️tests,🧫️fixtures,📚️examples,📦️packages,🔮️oracles}/**");
  try {
    for (const owner of fixture.owners) {
      expect(!ignored(owner.path) && (owner.path.includes("/🗿️artifacts/") || owner.schema && owner.snapshot && owner.io)).toBe(owner.checked);
      if (owner.schema) {
        const dir = join(root, owner.path, "🧬️schema", owner.snapshot ? "📸️snapshot" : "💡️inferences");
        mkdirSync(dir, {recursive:true}); writeFileSync(join(dir,"🦀️.rs"),fixture.semanticSource);
      }
      if (owner.io) {
        const dir = join(root,owner.path,"🚪️io/📝️text/📸️snapshot");
        mkdirSync(dir,{recursive:true}); writeFileSync(join(dir,"🦀️.rs"),fixture.physicalSource);
      }
    }
    const expected = fixture.owners.filter(owner => owner.checked).map(owner => `${owner.path}/🧬️schema/📸️snapshot/🦀️.rs`).sort();
    expect(artifactIoArchitectureBreaches(root,undefined,taxonomy).filter(row => row.kind === "artifact-io/schema-codec").map(row => row.scope).sort()).toEqual(expected);
  } finally {rmSync(root,{recursive:true,force:true});}
  console.log("[DEBUG] framework paired snapshot+IO owner discovery oracle=Ajv+picomatch");
});

test("semantic and physical source declarations remain in separate owners", async () => {
  const rows = JSON.parse(readFileSync(join(library, "🧫️fixtures/🚪️artifact-io-source-boundary/🔣️.json"), "utf8")) as Record<"rust" | "typescript", {source: string; forbidden: string[]}[]>;
  expect(new Ajv({strict: true}).compile(JSON.parse(readFileSync(join(library, "🧬️schema/🚪️artifact-io-source-boundary/🔣️.json"), "utf8")))(rows)).toBe(true);
  await Parser.init();
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", library)), "out/tree-sitter-rust.wasm")));
  try {
    for (const row of rows.rust) {
      const tree = parser.parse(row.source)!, names = new Set<string>();
      expect(tree.rootNode.hasError()).toBe(false);
      const visit = (node: Parser.SyntaxNode): void => {
        const trait = node.type === "impl_item" ? node.childForFieldName("trait")?.text.split("<")[0]!.split("::").at(-1) : undefined;
        if (trait && ["Mutation", "MutationDiff", "DiffAlgebra"].includes(trait)) names.add(trait);
        if (node.type === "function_item") {
          const name = node.childForFieldName("name")!.text, result = node.childForFieldName("return_type")?.text;
          if (name === "apply_to_artifact" || name.startsWith("diff_") && result?.endsWith("Diff")) names.add(name);
        }
        for (const child of node.namedChildren) visit(child);
      };
      visit(tree.rootNode); tree.delete();
      expect([...names].sort()).toEqual(row.forbidden);
      expect(semanticArtifactIoItems(row.source)).toEqual(row.forbidden);
    }
  } finally { parser.delete(); }
  for (const row of rows.typescript) {
    const file = ts.createSourceFile("boundary.ts", row.source, ts.ScriptTarget.Latest, true), names: string[] = [];
    for (const statement of file.statements) {
      if (ts.isExportDeclaration(statement) && statement.exportClause && ts.isNamedExports(statement.exportClause)) names.push(...statement.exportClause.elements.map(element => element.name.text));
      if (ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isVariableStatement(statement)) {
        if (!statement.modifiers?.some(modifier => modifier.kind === ts.SyntaxKind.ExportKeyword)) continue;
        if (ts.isVariableStatement(statement)) names.push(...statement.declarationList.declarations.flatMap(declaration => ts.isIdentifier(declaration.name) ? [declaration.name.text] : []));
        else if (statement.name) names.push(statement.name.text);
      }
    }
    expect(names.filter(name => /Json(?:Text|Value|Projection)?$|^decode.*Protobuf$|^TxtProtobuf|^txtProtobuf(?:Key|String)$/u.test(name)).sort()).toEqual(row.forbidden);
    expect(schemaTypeScriptWireSymbols(row.source)).toEqual(row.forbidden);
  }
  console.log("[DEBUG] artifact-io-source-boundary oracle=tree-sitter+TypeScript+Ajv");
});

test("semantic diffs have independent text and binary IO owners", async () => {
  const fixture = JSON.parse(readFileSync(join(library, "🧫️fixtures/🚪️artifact-diff-codec-ownership/🔣️.json"), "utf8")) as {cases: {semantic: string; text: string; binary: string; missing: string[]}[]};
  const validate = new Ajv({strict: true}).compile({type: "object", required: ["cases"], additionalProperties: false, properties: {cases: {type: "array", items: {type: "object", required: ["semantic", "text", "binary", "missing"], additionalProperties: false, properties: {semantic: {type: "string"}, text: {type: "string"}, binary: {type: "string"}, missing: {type: "array", items: {type: "string"}, uniqueItems: true}}}}}});
  expect(validate(fixture)).toBe(true);
  await Parser.init();
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", library)), "out/tree-sitter-rust.wasm")));
  const names = (source: string, trait: string, macro = ""): Set<string> => {
    const tree = parser.parse(source)!;
    expect(tree.rootNode.hasError()).toBe(false);
    const found = new Set<string>();
    const visit = (node: Parser.SyntaxNode): void => {
      if (node.type === "impl_item" && node.childForFieldName("trait")?.text.split("<")[0]!.split("::").at(-1) === trait) found.add(node.childForFieldName("type")!.text.split("<")[0]!.split("::").at(-1)!);
      if (macro && node.type === "macro_invocation" && node.childForFieldName("macro")?.text.split("::").at(-1) === macro) found.add(node.namedChildren.find(child => child.type === "token_tree")!.text.slice(1, -1).trim().split("::").at(-1)!);
      for (const child of node.namedChildren) visit(child);
    };
    visit(tree.rootNode);
    tree.delete();
    return found;
  };
  try {
    for (const row of fixture.cases) {
      const text = names(row.text, "DiffText", "diff_text"), binary = names(row.binary, "DiffBinary", "diff_binary");
      expect([...names(row.semantic, "MutationDiff")].filter(type => !text.has(type) || !binary.has(type)).sort()).toEqual(row.missing);
      expect(missingArtifactDiffWireTypes(row.semantic, row.text, row.binary)).toEqual(row.missing);
    }
  } finally { parser.delete(); }
  console.log("[DEBUG] diff-io-coverage paired representations oracle=tree-sitter+Ajv");
});
const oracle = picomatch([
  "🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}",
  "🧬️schema/🧬️mutations/➕️add-node", "🧬️schema/💡️inferences/📦️bounds",
  "🚪️io/{📝️text,💾️binary}/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}", "🚪️io/🪶️sqlite/📸️snapshot",
  "🚪️io/{📝️text,💾️binary}/🧬️mutations/➕️add-node", "🚪️io/{📝️text,💾️binary}/💡️inferences/📦️bounds",
  "🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/📄️txt",
]);

test("artifact IO ownership language-neutral corpus agrees with independent JSON Schema and glob oracles", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const [paths, accepted] of [[vector.accepted, true], [vector.rejected, false]] as const) {
    for (const path of paths) {
      expect(oracle(path), `oracle: ${path}`).toBe(accepted);
      expect(artifactFacetPathIsDeclared(path, taxonomy), path).toBe(accepted);
    }
  }
});

test("native artifact authoring places representations above semantic codecs", () => {
  const leaves: ArtifactScaffoldLeaf[] = [];
  newScaffoldIoTree("🚪️io", taxonomy, leaves);
  const paths = leaves.map(leaf => leaf.path);
  for (const representation of taxonomy.nativeRepresentationDirs) {
    for (const facet of taxonomy.ioSemanticCollectionDirNames) {
      expect(paths.some(path => path.startsWith(`🚪️io/${representation}/${facet}/`)), `${representation}/${facet}`).toBe(true);
    }
  }
  expect(paths.some(path => /^🚪️io\/(?:📸️snapshot|🔺️diff|🧬️mutations|💡️inferences)\//u.test(path))).toBe(false);
});


test("artifact IO ownership rejects misplaced directories, specs, inline codecs, and schema aliases", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "artifact-io-ownership-"));
  const put = (path: string, content: string): void => {
    const full = join(root, "artifact", path);
    mkdirSync(dirname(full), { recursive: true });
    writeFileSync(full, content);
  };
  try {
    put("🧬️schema/📸️snapshot/🦀️.rs", "pub struct Snapshot;\n// impl ArtifactPack for Snapshot {}\nconst MESSAGE: &str = \"impl ArtifactDsl for Snapshot {}\";\npub use crate::standards::v1::subsets::text::schema::mutations::EditText;");
    put("🚪️io/💾️binary/📸️snapshot/🦀️.rs", "impl ArtifactPack for Snapshot {}");
    put("🚪️io/📝️text/🧬️mutations/➕️add-node/🦀️.rs", "impl OpText for AddNode {}");
    const progress: string[] = [];
    expect(artifactIoArchitectureBreaches(root, ["artifact"], taxonomy, { progress: event => progress.push(event.phase) })).toEqual([]);
    expect(progress.at(-1)).toBe("complete");
    expect(() => artifactIoArchitectureBreaches(root, ["artifact"], taxonomy, { cancelled: () => true })).toThrow("cancelled");
    put("🧬️schema/📸️snapshot/🦀️.rs", "impl ArtifactPack for Snapshot {}");
    put("🧬️schema/🧬️mutations/🦀️.rs", "#[path = \"../../🚪️io/💾️binary/🧬️mutations/🦀️.rs\"] pub mod binary;");
    put("🧬️schema/🔺️diff/📡️.protocol.semio", "protocol Diff {}");
    put("🦀️.rs", 'pub mod schema { #[path = "🚪️io/💾️binary/📸️snapshot/🦀️.rs"] pub mod wire; }');
    put("🧬️schema/🟦️.ts", 'export { decodeSnapshot } from "../🚪️io/💾️binary/📸️snapshot/🟦️.ts";');
    mkdirSync(join(root, "artifact/🧬️schema/🧬️mutations/💾️binary"), { recursive: true });
    mkdirSync(join(root, "artifact/🚪️io/🧬️mutations/📝️text"), { recursive: true });
    expect(artifactIoArchitectureBreaches(root, ["artifact"], taxonomy).map(row => row.kind).sort()).toEqual([
      "artifact-io/facet-path", "artifact-io/facet-path", "artifact-io/schema-codec", "artifact-io/schema-codec-alias", "artifact-io/schema-codec-mount", "artifact-io/schema-wire-spec", "artifact-io/schema-codec-alias", "artifact-io/schema-codec-mount",
      "artifact-io/schema-codec-dependency",
    ].sort());

    for (const source of ["impl DiffText for Diff {}", "impl DiffBinary for Diff {}", "protocol::io::diff_text!(Diff);", "protocol::io::diff_binary!(Diff);"]) {
      put("🧬️schema/🔺️diff/🦀️.rs", source);
      expect(artifactIoArchitectureBreaches(root, ["artifact"], taxonomy).some(row => row.scope.endsWith("🧬️schema/🔺️diff/🦀️.rs") && row.kind === "artifact-io/schema-codec")).toBe(true);
    }
    put("🚪️io/📝️text/🔺️diff/🦀️.rs", "impl MutationDiff<Snapshot> for Diff {}");
    put("🧬️schema/📸️snapshot/🟦️.ts", "export function snapshotToJsonText(value: unknown): string{return JSON.stringify(value)}");
    const sourceBreaches = artifactIoArchitectureBreaches(root, ["artifact"], taxonomy);
    expect(sourceBreaches.some(row => row.scope.endsWith("🚪️io/📝️text/🔺️diff/🦀️.rs") && row.kind === "artifact-io/io-semantic-implementation")).toBe(true);
    expect(sourceBreaches.some(row => row.scope.endsWith("🧬️schema/📸️snapshot/🟦️.ts") && row.kind === "artifact-io/schema-codec")).toBe(true);
    put("🧬️schema/🔺️diff/🦀️.rs", "impl MutationDiff<Snapshot> for Delta {}");
    put("🚪️io/📝️text/🔺️diff/🦀️.rs", "impl DiffText for Delta {}");
    expect(artifactIoArchitectureBreaches(root, ["artifact"], taxonomy).some(row => row.kind === "artifact-io/diff-completeness")).toBe(true);
    put("🚪️io/💾️binary/🔺️diff/🦀️.rs", "impl DiffBinary for Delta {}");
    expect(artifactIoArchitectureBreaches(root, ["artifact"], taxonomy).some(row => row.kind === "artifact-io/diff-completeness")).toBe(false);
    put("🧬️schema/📸️snapshot/🟦️.ts", "import {read} from '../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts';");
    expect(artifactIoArchitectureBreaches(root, ["artifact"], taxonomy).some(row => row.kind === "artifact-io/schema-codec-dependency")).toBe(true);
    for(const [source,forbidden] of [
      ['import{read}from"../../../🧰️framework/🔨️modules/🚪️io/📝️text/🟦️.ts";',true],
      ['// import {read} from "../../../🚪️io/💾️binary/🟦️.ts";\nexport const semantic=1;',false],
      ['const literal=\'import {read} from "../../../🚪️io/💾️binary/🟦️.ts";\';',false],
    ] as const){
      put("🧬️schema/📸️snapshot/🟦️.ts",source);
      const syntax=ts.createSourceFile("boundary.ts",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
      expect(syntax.statements.some(statement=>ts.isImportDeclaration(statement))).toBe(forbidden);
      expect(artifactIoArchitectureBreaches(root,["artifact"],taxonomy).some(row=>row.scope.endsWith("🧬️schema/📸️snapshot/🟦️.ts")&&row.kind==="artifact-io/schema-codec-dependency")).toBe(forbidden);
    }

  } finally { rmSync(root, { recursive: true, force: true }); }
});


test("artifact native IO authoring publishes its declared representation-first files", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "artifact-io-authoring-"));
  const authoring = JSON.parse(readFileSync(join(library, "🧫️fixtures/🪶️artifact-empty-facet-authoring/🔣️.json"), "utf8"));
  const owner = authoring.subsetSegments.join("/");
  mkdirSync(dirname(join(root, owner)), { recursive: true });
  try {
    const result = newScaffoldSubsetTree(root, owner, taxonomy, false);
    expect(result.created.map(path => path.slice(owner.length + 1)).sort()).toEqual([...authoring.subsetLayout].sort());
    expect(result.created.every(path => readFileSync(join(root, path), "utf8").length > 0)).toBe(true);
    expect(result.created.some(path => path.includes("/🪶️sqlite/"))).toBe(false);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("artifact IO contracts and command registration parse independently", () => {
  const roots = ["🚪️io/🏛️architecture/🟦️.ts", "🏗️authoring/🗿️artifact-tree/🟦️.ts", "🏗️builder/🟦️.ts"].map(path => join(library, path));
  const program = ts.createProgram(roots, { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noUncheckedIndexedAccess: true, allowImportingTsExtensions: true, skipLibCheck: true, noEmit: true, types: ["node", "bun"] });
  for (const root of roots) {
    const source = program.getSourceFile(root);
    expect(source).toBeDefined();
    expect([...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)].map(diagnostic => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
  }
  const repository = resolve(library, "../../../../.."), packageRoot = join(library, "📦️packages/🟦️typescript");
  const targets = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8")).targets;
  const scripts = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8")).scripts;
  for (const verb of ["test", "lint"]) {
    const id = `${verb}-artifact-io-ownership`;
    expect(targets[id].options.command).toBe(`bun ./📜️script.ts ${verb} artifact-io-ownership`);
    expect(scripts[id]).toBe(`nx run @semio-tech/repo-lib:${id}`);
    for (const filename of ["launch.json", "🧩️launch.seed.jsonc"]) {
      const launch = parse(readFileSync(join(repository, ".vscode", filename), "utf8"));
      expect(launch.configurations.filter((entry: { command?: string }) => entry.command === `bun nx run @semio-tech/repo-lib:${id} --skip-nx-cache`)).toHaveLength(1);
    }
  }
},{timeout:60000});
