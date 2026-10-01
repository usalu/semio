import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { ownedScriptRoutes, resolveOwnedScriptRoute } from "../../🏃️process/🧭️routing/🧩️contributions/🟦️.ts";
import fixture from "../../🧫️fixtures/🧱️owned-script-routes/🔣️.json";
import schema from "../../🧬️schema/🧱️owned-script-routes/🔣️.json";

test("portable command ownership matrix agrees with independent JSON schema and declarative selection", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const row of fixture.cases) {
    const routes = fixture.routes.filter((route) => !row.absent.includes(route.project));
    const selected = resolveOwnedScriptRoute(routes, row.segments);
    const projection = selected ? { target: selected.route.target, args: selected.args } : null;
    expect(projection, row.id).toEqual(row.expected);
    const reference = routes.map((route) => ({ route, prefix: row.segments.slice(0, route.command.length) })).filter(({ route, prefix }) => JSON.stringify(prefix) === JSON.stringify(route.command)).sort((a, b) => b.prefix.length - a.prefix.length)[0];
    expect(projection, row.id).toEqual(reference ? { target: reference.route.target, args: row.segments.slice(reference.prefix.length) } : null);
  }
  expect(validate({ ...fixture, routes: [{ ...fixture.routes[0], command: ["verify", "../owner"] }] })).toBe(false);
  expect(() => resolveOwnedScriptRoute([fixture.routes[0]!, fixture.routes[0]!], ["verify", "sample"])).toThrow("Ambiguous");
});

test("real contributions bind Nx package commands and root coordinator imports no specific implementation", () => {
  const root = findWorkspaceRoot(import.meta.dir), routes = ownedScriptRoutes(root);
  expect(resolveOwnedScriptRoute(routes, ["verify", "fem2d-window-config-contract"])?.route.project).toBe("@semio-tech/s-fem-composition-tests");
  expect(resolveOwnedScriptRoute(routes, ["verify", "puzzle-fill-policy-self-tests"])?.route.project).toBe("@semio-tech/s-puzzle-composition-tests");
  for (const route of routes) {
    const project = JSON.parse(readFileSync(join(root, route.packageRoot, "📋️project.json"), "utf8"));
    expect(project.name).toBe(route.project);
    expect(project.targets[route.target].options.command).toContain("📜️script.ts");
    expect(project.targets[route.target].options.cwd).toBe(route.packageRoot);
  }
  const source = readFileSync(join(root, "📜️script.ts"), "utf8"), ast = ts.createSourceFile("script.ts", source, ts.ScriptTarget.Latest, true);
  const imports: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) imports.push(node.moduleSpecifier.text);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) imports.push(node.arguments[0].text);
    ts.forEachChild(node, visit);
  };
  visit(ast);
  expect(imports.filter((specifier) => specifier.startsWith("./✏️s") || specifier.includes("/🗿️artifacts/"))).toEqual([]);
}, 60_000);


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
      builder.onLoad({ filter: /./ }, (row) => row.path.includes("/✏️s/🔌️plugins/") || row.path.includes("/✏️s/🧑‍💻dev/") ? { errors: [{ text: `Absent domain owner: ${row.path}` }] } : undefined);
    } }],
  });
  expect(output.outputFiles.length).toBeGreaterThan(0);
}, 60_000);
