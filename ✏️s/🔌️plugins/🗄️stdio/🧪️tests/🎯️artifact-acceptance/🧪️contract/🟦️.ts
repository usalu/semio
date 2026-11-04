/** 🧪️ Offline contract of the artifact acceptance matrix: every well-known artifact has a row, each row agrees with the artifact definition, the generated catalog, the family component manifests, a natural-file sample judged by a third-party oracle and an invalid input the oracle refuses.
 * Run: `repo tools exec bun -- test ✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts`
 * @see ../🧬️schema/🔣️.json */
import { describe, expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { ARTIFACTS_ROOT, REPO_ROOT, candidateSamples, hexBytes, invalidSamples, readMatrix, readSampleManifest, readSampleSchema, readSchema, resolveSample } from "../📇️matrix/🟦️.ts";
import { ORACLES, judgeBytes } from "../🔬️oracles/🟦️.ts";

const matrix = readMatrix();
const LENIENT = new Set(["binary", "none"]);
const TEXTUAL = new Set(["text", "csv", "tsv", "html", "xml", "svg", "json", "obj", "step", "ifc", "dxf", "epw", "gltf", "semio"]);
const FULL_PARSE = new Set(["png", "zip", "ooxml", "bcf", "zlib", "json", "xml", "svg"]);
const definitionOf = (directory: string) => JSON.parse(readFileSync(join(ARTIFACTS_ROOT, directory, "📜️artifact-definition.json"), "utf8")) as { id: string; artifact: string; representations: { extensions: string[]; mimes: string[] }[] };

function* manifests(directory: string): Generator<string> {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === "target" || entry.name === "node_modules") continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) yield* manifests(path);
    else if (entry.name === "🔣️.json" && path.includes("🧩️component")) yield path;
  }
}
const appIds = new Set<string>();
const apps = new Map<string, { plugin: string; roles: Set<string> }>();
for (const path of manifests(join(REPO_ROOT, "🌎️hub/🧩️compositions"))) {
  const descriptor = JSON.parse(readFileSync(path, "utf8")) as { manifest?: { pluginId: string; apps?: { role: string; dialect: { artifactKind: string } }[] } };
  for (const app of descriptor.manifest?.apps ?? []) {
    appIds.add((app as unknown as { id: string }).id);
    if (!app.dialect.artifactKind.startsWith("s.stdio.")) continue;
    const entry = apps.get(app.dialect.artifactKind) ?? { plugin: descriptor.manifest!.pluginId, roles: new Set<string>() };
    entry.roles.add(app.role);
    apps.set(app.dialect.artifactKind, entry);
  }
}
const lifecycle = JSON.parse(readFileSync(join(REPO_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json"), "utf8")) as { formats: { appId: string; formatKind: string }[] };
const catalog = JSON.parse(readFileSync(join(REPO_ROOT, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"), "utf8")) as { pluginId: string; activationEvents: string[] }[];

describe("matrix", () => {
  test("validates against its schema", () => {
    const ajv = new Ajv2020({ allErrors: true, strict: false });
    const valid = ajv.validate(readSchema() as object, matrix);
    expect(ajv.errorsText()).toBe("No errors");
    expect(valid).toBe(true);
  });
  test("covers every artifact directory exactly once", () => {
    const directories = readdirSync(ARTIFACTS_ROOT).filter((name) => statSync(join(ARTIFACTS_ROOT, name)).isDirectory());
    const defined = directories.filter((name) => { try { definitionOf(name); return true; } catch { return false; } });
    expect(matrix.artifacts.map((row) => row.directory).sort()).toEqual(defined.sort());
    expect(matrix.nonArtifacts.slice().sort()).toEqual(readdirSync(ARTIFACTS_ROOT).filter((name) => !defined.includes(name)).sort());
  });
  test("sample manifest validates against its schema and lists every row", () => {
    const ajv = new Ajv2020({ allErrors: true, strict: false });
    expect(ajv.validate(readSampleSchema() as object, readSampleManifest())).toBe(true);
    expect(Object.keys(readSampleManifest().samples).sort()).toEqual(matrix.artifacts.map((row) => row.artifact).sort());
  });
  test("names an oracle that exists for every row", () => {
    for (const row of matrix.artifacts) expect(Object.keys(ORACLES)).toContain(row.oracle);
  });
});

describe.each(matrix.artifacts.map((row) => [`${row.batch}/${row.artifact}`, row] as const))("%s", (_name, row) => {
  test("agrees with its artifact definition", () => {
    const definition = definitionOf(row.directory);
    expect(definition.id).toBe(row.kind);
    expect(definition.representations.flatMap((representation) => representation.extensions)).toContain(row.extension);
  });
  test("ships an editor and a viewer in its family component and the generated catalog", () => {
    if (row.ownerApp !== undefined) {
      expect(appIds.has(row.ownerApp)).toBe(true);
      expect(lifecycle.formats.some((format) => format.appId === row.ownerApp && format.formatKind.startsWith(`${row.kind}@`))).toBe(true);
      return;
    }
    const shipped = apps.get(row.kind);
    expect(shipped?.roles.has("editor") && shipped.roles.has("viewer") ? "shipped" : "missing").toBe("shipped");
    expect(shipped?.plugin).toBe(row.plugin);
    expect(catalog.some((plugin) => plugin.pluginId === row.plugin && plugin.activationEvents.includes(`on-artifact-kind:${row.kind}`))).toBe(true);
  });
  test("has a natural-file sample accepted by its third-party oracle", () => {
    const candidates = candidateSamples(row);
    expect(candidates.length).toBeGreaterThan(0);
    const sample = resolveSample(row, (bytes) => judgeBytes(row.oracle, bytes).ok);
    expect(sample === null ? `no candidate accepted of ${candidates.length}` : "accepted").toBe("accepted");
  });
  test("has manifest invalid samples refused where its oracle can tell", () => {
    const refusable = (path: string): boolean => (path.includes("🪄️wrong-magic") && !TEXTUAL.has(row.oracle) && !LENIENT.has(row.oracle)) || (path.includes("✂️truncated") && FULL_PARSE.has(row.oracle));
    const expected = invalidSamples(row).filter(refusable);
    const refused = expected.filter((path) => !judgeBytes(row.oracle, new Uint8Array(readFileSync(path))).ok);
    expect(refused.length).toBe(expected.length);
  });
  test("has an invalid input its oracle refuses", () => {
    if (LENIENT.has(row.oracle)) return;
    expect(judgeBytes(row.oracle, hexBytes(row.invalid.bytes)).ok).toBe(false);
  });
});
