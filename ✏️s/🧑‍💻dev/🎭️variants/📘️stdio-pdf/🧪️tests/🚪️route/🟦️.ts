import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { parse as parseToml } from "smol-toml";
import ts from "typescript";
import fixture from "./🧫️fixtures/🔣️.json" with {type: "json"};

const workspace = process.cwd();
const dev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { loadDevContribution } = await import(resolve(workspace, dev, "🧩️contribution/📥️loading/🟦️.ts"));
const declaration = loadDevContribution(workspace, {devContribution: fixture.contribution});
assert.ok(declaration, "Expected the specific Stdio PDF development contribution");

const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const { parsePlaygroundBlock } = await import(resolve(workspace, plugin, "📇️registry/🎮️playground/🔎️discovery/🟦️.ts"));
const original = readFileSync(resolve(workspace, fixture.manifest), "utf8");
const marker = `\nvariant = "${fixture.variant}"\n`;
const text = process.argv.includes("--staged-binding") && !original.includes(`devContribution = "${fixture.contribution}"`)
  ? original.replace(marker, `${marker}devContribution = "${fixture.contribution}"\n`)
  : original;
const block = text.split("[[package.metadata.semio.playground]]").find(part => part.includes(marker));
assert.ok(block);
const route = parsePlaygroundBlock(block.split("\n[")[0], fixture.plugin, dirname(fixture.manifest));
const semio = (parseToml(text) as unknown as {package: {metadata: {semio: {
  "depends-on": string[];
  playground: {variant: string; devContribution?: string}[];
}}}}).package.metadata.semio;
const independent = semio.playground.find(row => row.variant === fixture.variant);
assert.ok(independent);
assert.equal(route?.devContribution, independent.devContribution);
assert.equal(route?.devContribution, fixture.contribution);
assert.equal(route?.app, fixture.app);
assert.deepEqual(route?.ports, {react: fixture.reactPort, wgpu: fixture.wgpuPort});
assert.deepEqual(semio["depends-on"], fixture.dependsOn);
const repo = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { runtimeComponentClosure } = await import(resolve(workspace, repo, "🕸️dependencies/🧩️runtime/🟨️.mjs"));
const catalog = JSON.parse(readFileSync(resolve(workspace, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"), "utf8"));
const roots = [{id: fixture.plugin, appScoped: true}];
assert.deepEqual(runtimeComponentClosure(catalog, roots), [...fixture.dependsOn, fixture.plugin].sort());
const candidate = catalog.filter((row: {pluginId: string}) => row.pluginId === fixture.plugin);
assert.throws(() => runtimeComponentClosure(candidate, roots), {componentId: fixture.dependsOn[0]});

const { registryCatalogInputView } = await import(resolve(workspace, repo, "📦️packages/🟦️typescript/📇️inventory/🟦️.ts"));
const { TAXONOMY } = await import(resolve(workspace, plugin, "📇️registry/🔎️discovery/🟦️.ts"));
const { renderCatalogFiles } = await import(resolve(workspace, plugin, "📇️registry/📽️projection/🟦️.ts"));
const { specificCatalogPublicationV1 } = await import(resolve(workspace, "✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts"));
const inputView = registryCatalogInputView(workspace, TAXONOMY);
const projection = renderCatalogFiles(workspace, {
  ...inputView,
  readText: (path: string) => path === fixture.manifest ? text : inputView.readText(path),
  readBytes: (path: string) => path === fixture.manifest ? Buffer.from(text) : inputView.readBytes(path),
}, "exclude", specificCatalogPublicationV1((path: string) => readFileSync(resolve(workspace, path), "utf8")));
const projected = projection.playgrounds.find((row: {variant: string}) => row.variant === fixture.variant);
assert.equal(projected?.devContribution, fixture.contribution);
assert.equal(projected?.app, fixture.app);
assert.deepEqual(runtimeComponentClosure(projection.entries, roots), [...fixture.dependsOn, fixture.plugin].sort());

const renderer = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const { routePickedFileV1 } = await import(resolve(workspace, renderer, "🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts"));
const { S_FORMAT_CATALOG_V1 } = await import(resolve(workspace, "✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🟦️.ts"));
const specimen = readFileSync(resolve(workspace, fixture.input));
const picked = await routePickedFileV1(S_FORMAT_CATALOG_V1, {name: "input.pdf", type: "application/pdf"}, async (size: number) => specimen.subarray(0, size));
assert.ok(picked.kind === "routed");
assert.equal(picked.artifactRef, fixture.artifactRef);
assert.deepEqual(await routePickedFileV1(undefined, {name: "input.pdf"}, async (size: number) => specimen.subarray(0, size)),
  {kind: "refused", reason: "format-catalog-unavailable"});

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
assert.ok(ts.isCallExpression(catalogRows.initializer.expression));
assert.equal(catalogRows.initializer.expression.expression.getText(entry), "composeSpecificOsCatalogV1");

for (const name of [declaration.browserEntry, declaration.viteConfig]) {
  const path = resolve(workspace, name);
  assert.ok(path.length <= 256);
  const source = readFileSync(path, "utf8");
  const result = ts.transpileModule(source, {fileName: path, reportDiagnostics: true, compilerOptions: {target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext}});
  assert.deepEqual(result.diagnostics?.filter(row => row.category === ts.DiagnosticCategory.Error), []);
}
console.log("Selected Stdio PDF route agrees with TOML and opens the exact PDF 1.7 artifact through the product catalog");
