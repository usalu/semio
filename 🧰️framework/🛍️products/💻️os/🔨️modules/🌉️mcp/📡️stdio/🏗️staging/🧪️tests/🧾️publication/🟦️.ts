/** 🧾️ Normal MCP producer and warm-consumer publication identity law. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import ts from "typescript";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { posix, win32 } from "node:path";
import vectors from "./🧫️fixtures/🔣️.json";

const mcp = readFileSync(new URL("../../../../🟦️.ts", import.meta.url), "utf8");
const producer = readFileSync(new URL("../../../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), "utf8");
const projection = readFileSync(new URL("../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📤️published/🟦️.ts", import.meta.url), "utf8");
const ast = ts.createSourceFile("mcp.ts", mcp, ts.ScriptTarget.Latest, true);
const buildAst = ts.createSourceFile("build.ts", producer, ts.ScriptTarget.Latest, true);
const declarations = (names: readonly string[]) => ast.statements.filter(node => ts.isFunctionDeclaration(node) ? names.includes(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some(value => ts.isIdentifier(value.name) && names.includes(value.name.text)));
const compile = (text: string) => ts.transpileModule(text.replace(/^export /gmu, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
const binaryCode = compile(declarations(["MCP_ARTIFACT_REL", "MCP_BINARY_NAME", "pathApi", "resolveMcpBinaryPath"]).map(node => node.getText(ast)).join("\n"));
const publicationAst = ts.createSourceFile("published.ts", projection, ts.ScriptTarget.Latest, true);
const publication = publicationAst.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "publishedCargoArtifactDirectoryV1");
if (!publication) throw Error("Actual publication projection required");
const build = buildAst.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "BuildScript");
if (!build || !ts.isClassDeclaration(build)) throw Error("Actual normal MCP BuildScript required");
const run = build.members.find(node => ts.isMethodDeclaration(node) && node.name.getText(buildAst) === "run");
if (!run) throw Error("Actual MCP build method required");
const calls: ts.CallExpression[] = [];
const collect = (node: ts.Node): void => { if (ts.isCallExpression(node) && node.expression.getText(buildAst) === "buildAdmittedCargoArtifactsV1") calls.push(node); ts.forEachChild(node, collect); };
collect(run);

test("closed publication vectors agree with the independent SQLite path oracle", () => {
  expect(Object.keys(vectors).sort()).toEqual(["cases", "version"]);
  expect(vectors.version).toBe(1);
  expect(vectors.cases.map(row => row.id)).toEqual(["linux-prototype", "linux-release", "darwin-prototype", "darwin-release", "win32-prototype", "win32-release"]);
  for (const row of vectors.cases) {
    expect(Object.keys(row).sort()).toEqual(["arguments", "backingRoot", "binary", "id", "manifest", "manifestKey", "mode", "platform", "publication", "repoRoot", "target"]);
    expect(row.id).toBe(row.platform + "-" + row.mode);
    expect(row.manifestKey).toMatch(/^[a-f0-9]{20}$/u);
  }
  expect(calls).toHaveLength(1);
  const options = calls[0]!.arguments[3];
  expect(options && ts.isObjectLiteralExpression(options)).toBe(true);
  if (!options || !ts.isObjectLiteralExpression(options)) throw Error("Actual producer options required");
  expect(options.properties.some(node => node.name?.getText(buildAst) === "output")).toBe(false);
  expect(options.properties.some(node => node.name?.getText(buildAst) === "sourcesRecord")).toBe(true);
  const db = new Database(":memory:");
  try {
    for (const row of vectors.cases) {
      const oracle = db.query("SELECT ?1 || ?2 || 'published' || ?2 || ?3 || ?2 || ?4 publication, json_array('run',?5,'--output=stream') arguments").get(row.backingRoot, row.platform === "win32" ? "\\" : "/", row.manifestKey, row.mode, row.target) as { publication: string; arguments: string };
      expect(row.publication).toBe(oracle.publication);
      expect(row.arguments).toEqual(JSON.parse(oracle.arguments));
      const api = row.platform === "win32" ? win32 : posix;
      const actual = new Function("join", "resolve", "isAbsolute", "createHash", "readBuildMode", compile(publication.getText(publicationAst)) + ";return publishedCargoArtifactDirectoryV1;")(api.join, api.resolve, api.isAbsolute, createHash, (mode: string) => { if (!["prototype", "release"].includes(mode)) throw Error("Fixture mode required"); return mode; });
      expect(actual(row.backingRoot, row.manifest, row.mode)).toBe(oracle.publication);
    }
  } finally { db.close(); }
});

for (const row of vectors.cases) test("normal MCP consumer selects its producer publication on " + row.id, async () => {
  const api = row.platform === "win32" ? win32 : posix;
  const resolveConsumer = new Function("posix", "win32", "process", binaryCode + ";return resolveMcpBinaryPath;")(posix, win32, { env: {}, platform: row.platform });
  const warm = declarations(["serveWarmMcpStdio"]);
  expect(warm).toHaveLength(1);
  let command: string | undefined;
  const gateway = (options: { command: () => string }) => { command = options.command(); return Promise.resolve(0); };
  const processPort = { env: {}, platform: row.platform, stdin: undefined, stdout: undefined };
  const actualWarm = new Function("serveWarmMcpGateway", "stageMcpBinaryAsync", "requireMcpBinary", "withMcpHostDriverPolicy", "process", compile(warm[0]!.getText(ast)) + ";return serveWarmMcpStdio;")(gateway, () => Promise.resolve(0), resolveConsumer, (args: readonly string[]) => args, processPort);
  expect(await actualWarm(row.repoRoot, [], {}, row.platform)).toBe(0);
  expect(command).toBe(api.join(row.publication, row.binary));
});
