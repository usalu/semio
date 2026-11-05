/** 🧪️ Offline contract of the artifact acceptance matrix: every well-known artifact has a row, each row agrees with the artifact definition, the generated catalog, the family component manifests, a natural-file sample judged by a third-party oracle and an invalid input the oracle refuses.
 * Run: `repo tools exec bun -- test ./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts`
 * @see ../🧬️schema/🔣️.json */
import { describe, expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";
import { ARTIFACTS_ROOT, REPO_ROOT, candidateSamples, hexBytes, invalidSamples, readMatrix, readSampleManifest, readSampleSchema, readSchema, resolveSample, selectEditorProgram } from "../📇️matrix/🟦️.ts";
import { ORACLES, judgeBytes, persistedContent } from "../🔬️oracles/🟦️.ts";
import { zipSync } from "fflate";
import { detailsControlId, detailsControlSelector } from "../🏃️browser/🎯️details/🟦️.ts";

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
const ownedApps = new Map<string, string>();
const apps = new Map<string, { plugin: string; roles: Set<string> }>();
for (const path of manifests(join(REPO_ROOT, "🌎️hub/🧩️compositions"))) {
  const descriptor = JSON.parse(readFileSync(path, "utf8")) as { manifest?: { pluginId: string; apps?: { role: string; dialect: { artifactKind: string } }[] } };
  for (const app of descriptor.manifest?.apps ?? []) {
    ownedApps.set(`${descriptor.manifest!.pluginId}:${(app as unknown as { id: string }).id}`, app.role);
    if (!app.dialect.artifactKind.startsWith("s.stdio.")) continue;
    const entry = apps.get(app.dialect.artifactKind) ?? { plugin: descriptor.manifest!.pluginId, roles: new Set<string>() };
    entry.roles.add(app.role);
    apps.set(app.dialect.artifactKind, entry);
  }
}
const lifecycle = JSON.parse(readFileSync(join(REPO_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json"), "utf8")) as { formats: { appId: string; formatKind: string }[] };
const catalog = JSON.parse(readFileSync(join(REPO_ROOT, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"), "utf8")) as { pluginId: string; activationEvents: string[] }[];

describe("matrix", () => {
  test("binds every published native codec index to its own authored protocol", () => {
    const owner = join(REPO_ROOT, "🌎️hub/🧩️compositions/🗄️stdio");
    const factories = JSON.parse(readFileSync(join(owner, "🔌️plugin/📇️catalog/📜️native-codec-factories.json"), "utf8")).receipts as { factory_id: string; protocol_path: string }[];
    const publication = JSON.parse(readFileSync(join(owner, "📇️publication/🔣️.json"), "utf8")).publications.find((entry: { id: string }) => entry.id === "stdio-native-codecs") as { integrity: { pointer: string; path: string }[] };
    const codecs = JSON.parse(readFileSync(join(owner, "📇️publication/📜️native-catalog.json"), "utf8")).nativeCodecs as { factoryId: string; protocolSourceSha256: string }[];
    expect(publication.integrity.filter((binding) => binding.pointer.startsWith("/nativeCodecs/")).length).toBe(codecs.length);
    for (const [index, codec] of codecs.entries()) {
      const factory = factories.find((entry) => entry.factory_id === codec.factoryId)!;
      const bindings = publication.integrity.filter((binding) => binding.pointer === `/nativeCodecs/${index}/protocolSourceSha256`);
      expect(bindings.length).toBe(1);
      const expected = resolve(REPO_ROOT, factory.protocol_path);
      expect(resolve(owner, "📇️publication", bindings[0]!.path)).toBe(expected);
      expect(createHash("sha256").update(readFileSync(expected)).digest("hex")).toBe(codec.protocolSourceSha256);
    }
  });
  test("keeps arbitrary binary bytes valid and tests unknown-format refusal separately", () => {
    const row = matrix.artifacts.find((entry) => entry.artifact === "binary")!;
    expect(row.invalid.kind).toBe("unsupported-format");
    expect(matrix.artifacts.flatMap((entry) => definitionOf(entry.directory).representations.flatMap((representation) => representation.extensions))).not.toContain(row.invalid.extension);
    expect(row.validPayloads).toEqual(["", "00", "deadbeef", "ffc328"]);
    for (const payload of row.validPayloads!) {
      const bytes = hexBytes(payload);
      expect(bytes).toEqual(new Uint8Array(Buffer.from(payload, "hex")));
      expect(judgeBytes(row.oracle, bytes).ok).toBe(true);
    }
    const malformed = structuredClone(matrix);
    const binary = malformed.artifacts.find((entry) => entry.artifact === "binary")! as { invalid: { kind: string; extension?: string } };
    binary.invalid.kind = "malformed";
    delete binary.invalid.extension;
    expect(new Ajv2020({ strict: false }).validate(readSchema() as object, malformed)).toBe(false);
  });
  test("requires deterministic typed scalar edits instead of scanning controls", () => {
    const validate = new Ajv2020({ strict: false }).compile(readSchema());
    for (const mode of ["scan", "none"]) {
      const invalid = structuredClone(matrix);
      (invalid.artifacts[0].edit as { mode: string }).mode = mode;
      expect(validate(invalid)).toBe(false);
    }
    const invalid = structuredClone(matrix);
    const details = invalid.artifacts.find((row) => row.edit.path === "details")!;
    delete (details.edit as { valueKind?: string }).valueKind;
    expect(validate(invalid)).toBe(false);
  });
  test("targets native Details scalar node keys rather than translated labels", () => {
    expect(detailsControlId("", "text")).toBe("stdio-snapshot-details-root-text");
    expect(detailsControlId("a", "number")).toBe("sd-af63dc4c8601ec8c-number");
    expect(detailsControlSelector("", "text")).toBe('[role="tree"] [data-ui-node-key="stdio-snapshot-details-root-text"]');
  });
  test("compares edited content across natural-file exports with independent parsers", () => {
    const bytes = (text: string) => new TextEncoder().encode(text);
    expect(persistedContent(bytes('{"a":1,"b":2}'), "json")).toBe(persistedContent(bytes('{"b":2,"a":1}'), "json"));
    expect(persistedContent(bytes('{"a":1}'), "json")).not.toBe(persistedContent(bytes('{"a":2}'), "json"));
    const original = zipSync({ "document.txt": bytes("original") }, { level: 0 });
    const repacked = zipSync({ "document.txt": bytes("original") }, { level: 9 });
    const edited = zipSync({ "document.txt": bytes("edited") });
    expect(persistedContent(original, "zip-members")).toBe(persistedContent(repacked, "zip-members"));
    expect(persistedContent(original, "zip-members")).not.toBe(persistedContent(edited, "zip-members"));
    expect(persistedContent(bytes("original"), "bytes")).not.toBe(persistedContent(bytes("edited"), "bytes"));
  });
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
  test("selects explicit foreign editor owners before matching artifact names", () => {
    const row = matrix.artifacts.find((entry) => entry.ownerApp !== undefined)!;
    const owner = { pluginId: row.plugin!, appId: row.ownerApp! };
    expect(selectEditorProgram(row, [{ pluginId: row.plugin!, appId: row.kind + "@1/*#editor" }, owner])).toEqual(owner);
    expect(selectEditorProgram(row, [{ pluginId: "wrong", appId: row.ownerApp! }])).toBeUndefined();
  });
  test("rejects empty or duplicate sample lists", () => {
    for (const field of ["valid", "invalid"] as const) {
      const ajv = new Ajv2020({ allErrors: true, strict: false });
      const validate = ajv.compile(readSampleSchema() as object);
      const sample = "sample.file";
      for (const list of [[], [sample, sample]]) {
        const entry = { valid: [sample], invalid: [sample], [field]: list };
        expect(validate({ schema: "semio.stdio.artifact-acceptance-samples/v1", samples: { test: entry } })).toBe(false);
      }
    }
  });
  test("requires exact sample and editor pins", () => {
    const ajv = new Ajv2020({ strict: false });
    const validate = ajv.compile(readSchema() as object);
    for (const key of ["sample", "ownerApp"]) {
      const changed = structuredClone(matrix);
      delete (changed.artifacts[0] as unknown as Record<string, unknown>)[key];
      expect(validate(changed)).toBe(false);
    }
  });
  test("selects the pinned standard even when another editor is listed first", () => {
    const row = matrix.artifacts.find(row => row.artifact === "gif")!;
    const wrong = { pluginId: row.plugin!, appId: "s.stdio.gif@87a/*#editor" };
    const correct = { pluginId: row.plugin!, appId: row.ownerApp };
    expect(selectEditorProgram(row, [wrong, correct])).toBe(correct);
    expect(selectEditorProgram(row, [wrong])).toBeUndefined();
  });
  test("does not substitute another subset when a pinned sample is missing or refused", () => {
    const row = matrix.artifacts.find(row => row.artifact === "gif")!;
    expect(resolveSample({ ...row, sample: "absent.gif" }, () => true)).toBeNull();
    expect(resolveSample(row, () => false)).toBeNull();
    const pinned = resolveSample(row, bytes => judgeBytes(row.oracle, bytes).ok);
    expect(pinned).toBe(join(REPO_ROOT, row.sample!));
    expect(readFileSync(pinned!).subarray(0, 6).toString()).toBe("GIF89a");
  });
  test("names an oracle that exists for every row", () => {
    for (const row of matrix.artifacts) expect(Object.keys(ORACLES)).toContain(row.oracle);
  });
});

describe.each(matrix.artifacts.map((row) => [`${row.batch}/${row.artifact}`, row] as const))("%s", (_name, row) => {
  test("every listed sample exists and every valid sample is nonempty", () => {
    const sample = readSampleManifest().samples[row.artifact];
    expect(sample).toBeDefined();
    for (const path of [...sample!.valid, ...sample!.invalid]) expect(existsSync(join(REPO_ROOT, path))).toBe(true);
    for (const path of sample!.valid) expect(statSync(join(REPO_ROOT, path)).size).toBeGreaterThan(0);
  });
  test("pins its exact valid natural sample", () => {
    expect(readSampleManifest().samples[row.artifact]!.valid).toContain(row.sample);
    expect(resolveSample(row, bytes => judgeBytes(row.oracle, bytes).ok)).toBe(join(REPO_ROOT, row.sample!));
  });
  test("agrees with its artifact definition", () => {
    const definition = definitionOf(row.directory);
    expect(definition.id).toBe(row.kind);
    expect(definition.representations.flatMap((representation) => representation.extensions)).toContain(row.extension);
  });
  test("ships an editor and a viewer in its family component and the generated catalog", () => {
    expect(ownedApps.get(`${row.plugin}:${row.ownerApp}`)).toBe("editor");
    expect(ownedApps.get(`${row.plugin}:${row.ownerApp.replace(/#editor$/u, "#viewer")}`)).toBe("viewer");
    if (!row.ownerApp.startsWith(row.kind + "@")) {
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
