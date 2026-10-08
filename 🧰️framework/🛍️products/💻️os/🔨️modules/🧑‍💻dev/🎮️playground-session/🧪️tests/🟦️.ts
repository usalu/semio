import { test, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { transformSync } from "esbuild";
import ts from "typescript";
import schema from "../🧬️schema/🔣️.json";
import cases from "../🧫️fixtures/🔣️.json";
import { parsePlaygroundSessionPublicationRequestV1 } from "../🧬️schema/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../.."), general = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev", specific = "✏️s/🧑‍💻dev", library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const read = (path: string) => readFileSync(join(root, path), "utf8");
const oracle = new Ajv({ strict: false }).compile(schema);

test("actual session owner routes admit domain instances and never read test fixtures", async () => {
  const specificSource = read(specific + "/📜️script.ts");
  expect(specificSource).not.toContain("./🧩️catalog/🎮️session/🧫️fixtures/🔣️.json");
  const { runtimeFixturePathV1 } = await import("../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🟦️.ts");
  for (const script of [general + "/📦️packages/🟦️typescript/📜️script.ts", specific + "/📜️script.ts"]) {
    const syntax = ts.createSourceFile(script, read(script), ts.ScriptTarget.Latest, true);
    const owner = syntax.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "PlaygroundSessionScript"); expect(owner).toBeDefined();
    const imports: string[] = [];
    const collect = (node: ts.Node): void => { if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) imports.push(relative(root, resolve(join(root, script, ".."), node.arguments[0].text)).replaceAll("\\", "/")); ts.forEachChild(node, collect); };
    collect(owner!); expect(imports.length).toBe(3); for (const path of imports) { expect(runtimeFixturePathV1(path), path).toBe(false); expect(existsSync(join(root, path)), path).toBe(true); }
  }
  for (const path of [general + "/🎮️playground-session/🔣️.json", specific + "/🧩️catalog/🎮️session/🔣️.json"]) {
    const request = JSON.parse(read(path)); expect(oracle(request)).toBe(true); expect(parsePlaygroundSessionPublicationRequestV1(request)).toEqual(request);
  }
  const generalRequest = JSON.parse(read(general + "/🎮️playground-session/🔣️.json"));
  const specificRequest = JSON.parse(read(specific + "/🧩️catalog/🎮️session/🔣️.json"));
  expect(generalRequest).toEqual(cases.cases.find(row => row.id === "explicit-empty")!.input);
  expect(specificRequest).toEqual(JSON.parse(read(specific + "/🧩️catalog/🎮️session/🧫️fixtures/🔣️.json")).publication);
  for (const row of cases.cases) { expect(oracle(row.input), row.id).toBe(row.shape); if (row.accepted) expect(parsePlaygroundSessionPublicationRequestV1(row.input)).toEqual(row.input); else expect(() => parsePlaygroundSessionPublicationRequestV1(row.input)).toThrow(); }
  const { loadCatalogTaxonomy, generatorPreviewExecution } = await import("../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
  const taxonomy = loadCatalogTaxonomy();
  for (const id of ["playground-session", "s-playground-session"]) {
    const contract = taxonomy.generatorContracts![id]!; expect(contract.inputPatterns!.every(path => !path.includes("🧫️fixtures") && !path.includes("🧪️tests"))).toBe(true);
    expect(contract.inputPatterns).toContain(id === "playground-session" ? general + "/🎮️playground-session/🔣️.json" : specific + "/🧩️catalog/🎮️session/🔣️.json");
    const project = JSON.parse(read(contract.ownerPath + "/📋️project.json")), target = project.targets[contract.previewTarget!.split(":").at(-1)!];
    expect(generatorPreviewExecution(contract, target).args).toEqual(["./📜️script.ts", "playground-session", "preview"]);
  }
  console.log("[DEBUG] canonical publication request instances/ten plain cases agree with independent Ajv; strict actual taxonomy and both semantic previews have no fixture source dependencies");
});

test("actual General read-only preview preserves bytes and output override with independent esbuild semantics", async () => {
  const script = join(root, general, "📦️packages/🟦️typescript/📜️script.ts"), expectedPath = general + "/🤖️generated/🎮️playground-session/🟦️.ts";
  const before = existsSync(join(root, expectedPath)) ? read(expectedPath) : undefined;
  const overrides = [undefined, process.env.SEMIO_TEST_ARTIFACT_DIR]; expect(overrides[1]).toBeTruthy();
  for (const output of overrides) {
    const env = { ...process.env }; delete env.SEMIO_PLAYGROUND_SESSION_OUTPUT_ROOT; if (output) env.SEMIO_PLAYGROUND_SESSION_OUTPUT_ROOT = join(output, "preview-output");
    const actual = Bun.spawnSync([process.execPath, script, "playground-session", "preview"], { cwd: root, env, timeout: 20000 });
    expect(actual.exitCode, new TextDecoder().decode(actual.stderr)).toBe(0);
    const preview = JSON.parse(new TextDecoder().decode(actual.stdout)), file = preview.nodes.find((node: any) => node.nodeKind === "file");
    expect(preview.contractId).toBe("playground-session"); expect(preview.schemaVersion).toBe(1);
    expect(file.path).toBe(output ? relative(root, join(output, "preview-output/🎮️playground-session/🟦️.ts")).replaceAll("\\", "/") : expectedPath);
    const bytes = Buffer.from(file.bytesBase64, "base64").toString("utf8");
    const { renderPlaygroundSessionTypeScript } = await import("../../../🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts");
    expect(bytes).toBe(renderPlaygroundSessionTypeScript(undefined, { entries: [], playgrounds: [] }));
    const owned = new Bun.Transpiler({ loader: "ts" }).transformSync(bytes), independent = transformSync(bytes, { loader: "ts", format: "esm", target: "es2022" }).code;
    const modules = await Promise.all([owned, independent].map(code => import("data:text/javascript;base64," + Buffer.from(code).toString("base64"))));
    expect(modules[0].PLAYGROUND_SESSION).toEqual(modules[1].PLAYGROUND_SESSION);
    expect(modules[0].PLAYGROUND_SESSION).toBeUndefined(); expect(existsSync(join(root, file.path))).toBe(output ? false : before !== undefined);
  }
  expect(existsSync(join(root, expectedPath)) ? read(expectedPath) : undefined).toBe(before);
  console.log("[DEBUG] actual General two read-only previews preserve canonical/output-env paths and rendered bytes; Bun/esbuild module semantics agree; generated writes0/Cargo/server0");
}, 40000);
