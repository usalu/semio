import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { parse as parseToml } from "smol-toml";
import ts from "typescript";
import fixture from "./🧫️fixtures/🔣️.json" with {type: "json"};

/** 🚪️ The standalone XLSX pane route: the Cargo playground rows of every XLSX variant bind the owned development contribution, the independent TOML parser agrees with the registry parser, and the browser entry mounts the shell with the product format catalog.
 * @see ../../../🧬️schema/🔣️.json */
const workspace = process.cwd();
const dev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { loadDevContribution } = await import(resolve(workspace, dev, "🧩️contribution/📥️loading/🟦️.ts"));
const declaration = loadDevContribution(workspace, {devContribution: fixture.contribution});
assert.ok(declaration, "Expected the specific Stdio XLSX development contribution");

const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const { parsePlaygroundBlock } = await import(resolve(workspace, plugin, "📇️registry/🎮️playground/🔎️discovery/🟦️.ts"));
const text = readFileSync(resolve(workspace, fixture.manifest), "utf8");
const semio = (parseToml(text) as unknown as {package: {metadata: {semio: {
  "depends-on": string[];
  playground: {variant: string; devContribution?: string; app: string; ports: {react: number; wgpu: number}}[];
}}}}).package.metadata.semio;
for (const variant of [fixture.variant, ...fixture.siblings]) {
  const marker = `\nvariant = "${variant}"\n`;
  const block = text.split("[[package.metadata.semio.playground]]").find(part => part.includes(marker));
  assert.ok(block, variant);
  const route = parsePlaygroundBlock(block.split("\n[")[0], fixture.plugin, dirname(fixture.manifest));
  const independent = semio.playground.find(row => row.variant === variant);
  assert.ok(independent, variant);
  assert.equal(route?.devContribution, independent.devContribution);
  assert.equal(route?.devContribution, fixture.contribution);
  assert.equal(route?.app, independent.app);
  assert.deepEqual(route?.ports, independent.ports);
}
const selected = semio.playground.find(row => row.variant === fixture.variant);
assert.equal(selected?.app, fixture.app);
assert.deepEqual(selected?.ports, {react: fixture.reactPort, wgpu: fixture.wgpuPort});
assert.deepEqual(semio["depends-on"], fixture.dependsOn);

const entrySource = readFileSync(resolve(workspace, declaration.browserEntry), "utf8");
const entry = ts.createSourceFile("entry.ts", entrySource, ts.ScriptTarget.ES2022, true);
const call = entry.statements.flatMap(statement => ts.isVariableStatement(statement) ? [...statement.declarationList.declarations] : [])
  .map(declaration => declaration.initializer).find(initializer => initializer && ts.isAwaitExpression(initializer)
    && ts.isCallExpression(initializer.expression) && initializer.expression.expression.getText(entry) === "bootFrameworkOsDev");
assert.ok(call && ts.isAwaitExpression(call) && ts.isCallExpression(call.expression));
const ports = call.expression.arguments[0];
assert.ok(ports && ts.isObjectLiteralExpression(ports));
const field = (name: string) => ports.properties.find(property => property.name?.getText(entry) === name);
const formatCatalog = field("formatCatalog");
assert.ok(formatCatalog && ts.isPropertyAssignment(formatCatalog));
assert.equal(formatCatalog.initializer.getText(entry), "S_FORMAT_CATALOG_V1");
const catalogRows = field("catalogRows");
assert.ok(catalogRows && ts.isPropertyAssignment(catalogRows) && ts.isPropertyAccessExpression(catalogRows.initializer));
assert.equal(catalogRows.initializer.name.text, "rows");
assert.equal(catalogRows.initializer.expression.getText(entry).startsWith("composeSpecificOsCatalogV1"), true);
const guard = /\/\^(.+)\$\/u\.test\(variant\)/u.exec(entrySource);
assert.ok(guard);
for (const variant of [fixture.variant, ...fixture.siblings]) assert.ok(new RegExp(`^${guard[1]}$`, "u").test(variant), variant);

for (const name of [declaration.browserEntry, declaration.viteConfig]) {
  const path = resolve(workspace, name);
  assert.ok(path.length <= 256);
  const result = ts.transpileModule(readFileSync(path, "utf8"), {fileName: path, reportDiagnostics: true, compilerOptions: {target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext}});
  assert.deepEqual(result.diagnostics?.filter(row => row.category === ts.DiagnosticCategory.Error), []);
}
console.log("Selected Stdio XLSX route agrees with TOML and binds every XLSX variant to the owned development contribution");
