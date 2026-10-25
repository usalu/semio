import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import { runInNewContext } from "node:vm";
import Ajv from "ajv";
import ts from "typescript";
import { build, type Plugin } from "esbuild";
import { BundleScript, findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import corpus from "../🧫️fixtures/🧭️router-ownership/🔣️.json";
import { runtimeExecutableV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";


const root = findWorkspaceRoot(import.meta.dir);
const read = (path: string) => readFileSync(join(root, path), "utf8");
const parse = (path: string) => ts.createSourceFile(path, read(path), ts.ScriptTarget.Latest, true);
const require = createRequire(import.meta.url);
const artifacts = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-strict-boundary/ui-router-node");

function routes(source: ts.SourceFile): string[] {
  const result: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) result.push(node.arguments[0].text);
    ts.forEachChild(node, visit);
  };
  visit(source);
  return result.sort();
}

function productsRefused(): Plugin {
  return { name: "independent-products-refused", setup(builder) {
    builder.onResolve({ filter: /.*/ }, (args) => {
      if (args.path.startsWith("node:")) return;
      let target: string;
      if (args.path.startsWith(".") || isAbsolute(args.path)) target = resolve(args.resolveDir, args.path);
      else {
        if (args.path === "@repo-lib" || args.path.startsWith("@repo-lib/")) target = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
        else try { target = require.resolve(args.path, { paths: [args.resolveDir] }); } catch { return; }
      }
      if (target.split(/[\\/]/u).includes("node_modules")) return;
      if (corpus.productRoots.some(area => target === join(root, area) || target.startsWith(join(root, area) + "/") || target.startsWith(join(root, area) + "\\"))) return { errors: [{ text: "product dependency refused: " + target }] };
      return;
    });
  } };
}

function nativeModuleProvenance(): Plugin {
  return { name: "native-module-provenance", setup(builder) {
    builder.onLoad({ filter: /\.ts$/ }, args => {
      const text = readFileSync(args.path, "utf8");
      const source = ts.createSourceFile(args.path, text, ts.ScriptTarget.Latest, true);
      const edits: { start: number; end: number; text: string }[] = [];
      const visit = (node: ts.Node): void => {
        if (ts.isPropertyAccessExpression(node) && ts.isMetaProperty(node.expression) && node.expression.keywordToken === ts.SyntaxKind.ImportKeyword && (node.name.text === "url" || node.name.text === "dir")) edits.push({ start: node.getStart(source), end: node.getEnd(), text: JSON.stringify(node.name.text === "url" ? pathToFileURL(args.path).href : dirname(args.path)) });
        ts.forEachChild(node, visit);
      };
      visit(source);
      let contents = text;
      for (const edit of edits.sort((a, b) => b.start - a.start)) contents = contents.slice(0, edit.start) + edit.text + contents.slice(edit.end);
      return { contents, loader: "ts" };
    });
  } };
}

test("portable UI ownership schema keeps exact generic and workspace routes at their owner", () => {
  
  expect(corpus["version"]).toEqual(1);expect(corpus["parentKind"]).toEqual("repo-server-library");expect(corpus["directoryKind"]).toEqual("repo-library-ui");
  expect(routes(parse(corpus.generalScript))).toEqual([...corpus.retainedRoutes].sort());
  const general = JSON.parse(read(corpus.generalProject));
  const repo = JSON.parse(read(corpus.repoProject));
  for (const row of corpus.movedRoutes) {
    expect(general.targets[row.command]).toBeUndefined();
    expect(repo.targets[row.target].options.command).toBe("repo ./📜️script.ts ui " + row.command);
    expect(routes(parse(corpus.repoScript))).toContain(row.command);
    if (row.command.startsWith("check-")) expect(repo.targets[row.target].cache === false || repo.targets[row.target].inputs?.includes("reads"), row.target).toBe(true);
  }
  const taxonomy = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
  expect(taxonomy.semanticDirectoryKinds[corpus.directoryKind].parentKindIds).toEqual([corpus.parentKind]);
});

test("all original workspace census declarations retain their exact TypeScript AST text", () => {
  const sources = [parse(corpus.repoScript), parse(corpus.repoOwner)];
  const declared = (source: ts.SourceFile, name: string) => source.statements.find(node => ((ts.isClassDeclaration(node) || ts.isFunctionDeclaration(node) || ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name?.getText(source) === name) || (ts.isVariableStatement(node) && node.declarationList.declarations.some(declaration => declaration.name.getText(source) === name)));
  const lines = (text: string) => text.split("\n").map(line => line.trim()).join("\n");
  for (const row of corpus.censusDeclarations) {
    const owner = sources.find(source => declared(source, row.name));
    expect(owner, row.name).toBeDefined();
    expect(createHash("sha256").update(declared(owner!, row.name)!.getText(owner!)).digest("hex"), row.name).toBe(row.sha256);
    if (row.runner === undefined) continue;
    const runner = declared(sources[1]!, row.runner) as ts.FunctionDeclaration | undefined;
    expect(runner?.body, row.runner).toBeDefined();
    expect<unknown>(createHash("sha256").update(lines(runner!.body!.getText(sources[1]!).replaceAll("context.", "this."))).digest("hex"), row.runner).toBe(row.runnerBodySha256);
  }
});

test("the original Storybook handlers preserve root commands, caller arguments and UI environment", () => withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "router-ownership-test", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async (invocation) => {
  const source = parse(corpus.repoScript), owner = parse(corpus.repoOwner);
  const declarations = [
    ...source.statements.filter(node => ts.isClassDeclaration(node) && corpus.storybookCases.some(row => node.name?.text === row.declaration)).map(node => node.getText(source)),
    ...owner.statements.filter(node => (ts.isFunctionDeclaration(node) && corpus.storybookCases.some(row => node.name?.text === row.runner)) || (ts.isVariableStatement(node) && node.declarationList.declarations.some(row => row.name.getText(owner) === "storybookEnv"))).map(node => node.getText(owner).replace(/^export /, "")),
  ];
  expect(declarations).toHaveLength(corpus.storybookCases.length * 2 + 1);
  const calls: { command: string; args: string[]; options: { cwd: string; env: Record<string, string> } }[] = [];
  const environment = { WATCHPACK_POLLING: "explicit-watch", CHOKIDAR_USEPOLLING: "explicit-chokidar" };
  const context = { BundleScript, runtimeExecutableV1, process: { env: environment }, devToolingEnv: (env: Record<string, string>) => ({ ...environment, ...env }), runCmd: (command: string, args: string[], options: { cwd: string; env: Record<string, string> }) => calls.push({ command, args, options }), root, invocation };
  const executable = ts.transpileModule(declarations.join("\n") + "\nglobalThis.handlers = [new DevScript(root, root, invocation), new BuildScript(root, root, invocation)];", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const handlers = runInNewContext(executable + "\nhandlers;", context) as BundleScript[];
  for (let index = 0; index < corpus.storybookCases.length; index++) {
    const row = corpus.storybookCases[index]!;
    await handlers[index]!.run(["--owner-proof"]);
    const call = calls[index]!;
    expect(call.command).toBe(runtimeExecutableV1());
    expect([...call.args]).toEqual(row.arguments);
    expect(call.options.cwd).toBe(root);
    expect(call.options.env.WATCHPACK_POLLING).toBe(environment.WATCHPACK_POLLING);
    expect(call.options.env.CHOKIDAR_USEPOLLING).toBe(environment.CHOKIDAR_USEPOLLING);
    if (row.scope === null) expect(call.options.env.STORYBOOK_SCOPE ?? null).toBeNull();
    else expect(call.options.env.STORYBOOK_SCOPE).toBe(row.scope);
  }
}));

test("independent esbuild and native Node load the generic router while refusing every product area", async () => {
  mkdirSync(artifacts, { recursive: true });
  const output = await build({ absWorkingDir: root, entryPoints: [corpus.generalScript], bundle: true, write: false, packages: "external", platform: "node", format: "esm", metafile: true, plugins: [productsRefused(), nativeModuleProvenance()], logLevel: "silent" });
  expect(Object.keys(output.metafile!.inputs).some(path => corpus.productRoots.some(area => path.startsWith(area + "/")))).toBe(false);
  const entry = join(artifacts, "📜️script.mjs");
  writeFileSync(entry, output.outputFiles[0]!.text);
  const observed = spawnSync(process.env.SEMIO_NODE_BINARY ?? "node", [entry, "ownership-unregistered"], { cwd: root, encoding: "utf8", timeout: 6_000, env: process.env });
  expect(observed.error).toBeUndefined();
  expect(observed.status).toBe(1);
  expect(observed.stderr).toContain('unknown command "ownership-unregistered"');
  for (const route of corpus.retainedRoutes) expect(observed.stderr).toContain(route);
});

test("the independent loader refuses injected physical and aliased product imports", async () => {
  for (const area of corpus.productRoots) {
    await expect(build({ absWorkingDir: root, stdin: { contents: "import " + JSON.stringify(join(root, area, "🟦️.ts")), resolveDir: root, loader: "ts" }, bundle: true, write: false, platform: "node", format: "esm", plugins: [productsRefused()], logLevel: "silent" })).rejects.toThrow("product dependency refused");
  }
  await expect(build({ absWorkingDir: root, stdin: { contents: 'import "@repo-lib"', resolveDir: root, loader: "ts" }, bundle: true, write: false, platform: "node", plugins: [productsRefused()], logLevel: "silent" })).rejects.toThrow("product dependency refused");
});
