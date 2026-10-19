import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ownedScriptRoutes, resolveOwnedScriptRoute } from "../../🏃️process/🧭️routing/🧩️contributions/🟦️.ts";
import { policyRootScriptModulePaths } from "../../⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { rootScriptSources } from "../📎️root-script-sources/🟦️.ts";
import fixture from "../🧫️fixtures/🧱️owned-script-routes/🔣️.json";
import schema from "../../🧬️schema/🧱️owned-script-routes/🔣️.json";


test("portable command ownership matrix agrees with independent JSON schema and declarative selection", () => {
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const routes = fixture.routes.filter((route) => !row.absent.includes(route.project));
    const selected = resolveOwnedScriptRoute(routes, row.segments);
    const projection = selected ? { target: selected.route.target, args: selected.args } : null;
    expect(projection, row.id).toEqual(row.expected);
    const reference = routes.map((route) => ({ route, prefix: row.segments.slice(0, route.command.length) })).filter(({ route, prefix }) => JSON.stringify(prefix) === JSON.stringify(route.command)).sort((a, b) => b.prefix.length - a.prefix.length)[0];
    expect(projection, row.id).toEqual(reference ? { target: reference.route.target, args: row.segments.slice(reference.prefix.length) } : null);
  }
  
  expect(() => resolveOwnedScriptRoute([fixture.routes[0]!, fixture.routes[0]!], ["verify", "sample"])).toThrow("Ambiguous");
});

test("real contributions bind Nx package commands and root coordinator imports no specific implementation", () => {
  const root = findWorkspaceRoot(import.meta.dir), routes = ownedScriptRoutes(root);
  expect(resolveOwnedScriptRoute(routes, ["verify", "fem2d-window-config-contract"])?.route.project).toBe("@semio-tech/s-fem-composition-tests");
  expect(resolveOwnedScriptRoute(routes, ["verify", "puzzle-fill-policy-self-tests"])?.route.project).toBe("@semio-tech/s-puzzle-composition-tests");
  expect(resolveOwnedScriptRoute(routes, ["schema", "runtime-graph", "--report", "graph.json"])).toEqual({route:{command:["schema","runtime-graph"],project:"os-hub",target:"runtime-fixture-graph",packageRoot:"🌎️hub/📦️packages/🦀️rust"},args:["--report","graph.json"]});
  for (const route of routes) {
    const project = JSON.parse(readFileSync(join(root, route.packageRoot, "📋️project.json"), "utf8"));
    expect(project.name).toBe(route.project);
    expect(project.targets[route.target].options.command).toContain("📜️script.ts");
    {const cwd = project.targets[route.target].options.cwd as string;expect(cwd === "{projectRoot}" || cwd === route.packageRoot || route.packageRoot.startsWith(cwd + "/"), route.project + ":" + route.target).toBe(true);}
  }
  const imports: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) imports.push(node.moduleSpecifier.text);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) imports.push(node.arguments[0].text);
    ts.forEachChild(node, visit);
  };
  for (const { path, text } of rootScriptSources(root)) visit(ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true));
  expect(imports.filter((specifier) => fixture.removedOwnerPrefixes.some(owner=>specifier.startsWith("./"+owner+"/")) || specifier.includes("/🗿️artifacts/"))).toEqual([]);
}, 60_000);

test("every real contribution is reachable from a dispatcher call site and no verify route shadows a root verify command", () => {
  const root = findWorkspaceRoot(import.meta.dir), routes = ownedScriptRoutes(root);
  const scripts = execFileSync("git", ["ls-files", "-co", "--exclude-standard", "--", "📜️script.ts", "*/📜️script.ts"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).split("\n").filter((path) => path && !path.includes("/🎫️tickets/") && existsSync(join(root, path)));
  const rootPaths = new Set(policyRootScriptModulePaths(root));
  for (const path of rootPaths) if (!scripts.includes(path)) scripts.push(path);
  const dispatchers: { prefix: string[]; open: boolean; site: string }[] = [];
  let builtins: string[] = [];
  for (const path of scripts) {
    const source = readFileSync(join(root, path), "utf8");
    if (!source.includes("dispatchOwnedScriptRoute(")) continue;
    const visit = (node: ts.Node): void => {
      if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "dispatchOwnedScriptRoute" && node.arguments[2] && ts.isArrayLiteralExpression(node.arguments[2])) {
        const elements = node.arguments[2].elements, prefix: string[] = [];
        for (const element of elements) { if (!ts.isStringLiteral(element)) break; prefix.push(element.text); }
        dispatchers.push({ prefix, open: prefix.length < elements.length, site: path });
      }
      if (rootPaths.has(path) && ts.isClassDeclaration(node) && node.name?.text === "VerifyScript") builtins = [...node.getText().matchAll(/segments\[0\] === "([^"]+)"/gu)].map((match) => match[1]!);
      ts.forEachChild(node, visit);
    };
    visit(ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true));
  }
  expect(dispatchers.some(({ prefix, open, site }) => rootPaths.has(site) && open && prefix.join(" ") === "verify")).toBe(true);
  expect(builtins).toContain("taxonomy");
  const reachable = (command: readonly string[]): boolean => dispatchers.some(({ prefix, open }) => prefix.length > 0 && prefix.slice(0, command.length).every((word, index) => command[index] === word) && (open ? command.length >= prefix.length : command.length <= prefix.length));
  expect(routes.filter((route) => !reachable(route.command)).map((route) => `${route.project}:${route.target} ${route.command.join(" ")}`)).toEqual([]);
  expect(routes.filter((route) => route.command[0] === "verify" && builtins.includes(route.command[1]!)).map((route) => route.command.join(" "))).toEqual([]);
}, 120_000);


test("the neutral repository coordinator builds with every domain plugin source removed from the resolver", async () => {
  const root = findWorkspaceRoot(import.meta.dir);
  const { build } = await import("esbuild");
  const output = await build({
    absWorkingDir: root,
    entryPoints: ["📜️script.ts"],
    write: false,
    bundle: true,
    platform: "node",
    format: "esm",
    packages: "external",
    external: ["*.wasm"],
    plugins: [{ name: "absent-domain-owners", setup(builder) {
      builder.onLoad({ filter: /./ }, (row) => fixture.removedOwnerPrefixes.some(owner=>row.path.replaceAll("\\","/").startsWith(join(root,owner).replaceAll("\\","/")+"/")) ? { errors: [{ text: `Absent domain owner: ${row.path}` }] } : undefined);
    } }],
  });
  expect(output.outputFiles.length).toBeGreaterThan(0);
}, 60_000);
