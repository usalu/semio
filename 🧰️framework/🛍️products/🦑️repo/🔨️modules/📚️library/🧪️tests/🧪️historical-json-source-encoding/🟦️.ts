import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { findNodeAtLocation, getNodeValue, parseTree } from "jsonc-parser";
import { loadCatalogTaxonomy, validateFrozenCoordinateEvidenceContracts } from "../../🔍️discovery/🟦️.ts";
import { canonicalJson, frozenCoordinateEvidenceCoordinates } from "../../🧹️normalization/🟦️.ts";

const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const historical = JSON.parse(readFileSync(join(import.meta.dir, "🧬️energy-source-coordinates/🔣️.json"), "utf8"));
const sha = (value: string) => createHash("sha256").update(value).digest("hex");
const libraryRoot = resolve(import.meta.dir, "../.."), root = resolve(libraryRoot, "../../../../..");

test("historical escaped-source vectors bind one JSON string layer and an explicit root", () => {
  const validate = new Ajv().compile({ type: "object", required: ["schemaVersion", "contract", "semantics", "cases"], properties: { schemaVersion: { const: 1 }, contract: { const: "historical-json-escaped-source-coordinates-v1" }, cases: { type: "array", minItems: 15, items: { type: "object", required: ["id", "source", "pointer", "accepted"] } } } });
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const row of vector.cases.filter((row: any) => row.accepted)) {
    const tree = parseTree(row.source)!;
    expect(getNodeValue(tree)).toEqual(JSON.parse(row.source));
    const node = findNodeAtLocation(tree, row.pointer.slice(1).split("/").map((part: string) => /^\d+$/.test(part) ? Number(part) : part))!;
    expect(node.type).toBe("string");
    const raw = row.source.slice(node.offset + 1, node.offset + node.length - 1);
    expect(JSON.parse('"' + raw + '"')).toBe(node.value);
    expect(raw).not.toBe(node.value);
  }
});

for (const row of vector.cases) test("historical escaped-source authority: " + row.id, () => {
  const contract = { path: "🧪️tests/🔣️history.json", sha256: sha(row.source), schemaVersion: null, ...(row.rootKind ? { rootKind: row.rootKind } : {}), coordinates: [{ pointer: row.pointer, kind: "source", representation: "json-escaped-source-path" }] };
  const run = () => frozenCoordinateEvidenceCoordinates(contract.path, Buffer.from(row.source), { history: contract } as never);
  if (!row.accepted) expect(run).toThrow(/frozen-coordinate-evidence-invalid/u);
  else {
    const node = findNodeAtLocation(parseTree(row.source)!, row.pointer.slice(1).split("/").map((part: string) => /^\d+$/.test(part) ? Number(part) : part))!;
    expect(run()).toEqual([{ pointer: row.pointer, start: node.offset + 1, end: node.offset + node.length - 1, value: row.source.slice(node.offset + 1, node.offset + node.length - 1), kind: "source" }]);
  }
});

test("escaped-source authority retains exact representation root digest and selector boundaries", () => {
  const source = vector.cases[0].source, bytes = Buffer.from(source), path = "🧪️tests/🔣️history.json";
  const contract = { path, sha256: sha(source), schemaVersion: null, rootKind: "array", coordinates: [{ pointer: "/0/path", kind: "source", representation: "json-escaped-source-path" }] };
  const run = (value: any, input = bytes) => frozenCoordinateEvidenceCoordinates(value.path, input, { history: value });
  expect(validateFrozenCoordinateEvidenceContracts({ history: contract })).toEqual([]);
  expect(() => run(contract, Buffer.concat([bytes, Buffer.from("\n")]))).toThrow(/digest/u);
  for (const alter of [
    (value: any) => { value.rootKind = "object"; },
    (value: any) => { value.rootKind = null; },
    (value: any) => { value.schemaVersion = 1; },
    (value: any) => { value.extra = true; },
    (value: any) => { value.coordinates[0].kind = "destination"; },
    (value: any) => { value.coordinates[0].representation = "json"; },
    (value: any) => { value.coordinates[0].recordedRepositoryRoot = "/recorded"; },
    (value: any) => { value.coordinates[0].pointer = "/00/path"; },
    (value: any) => { value.coordinates.push({ ...value.coordinates[0], pointer: "/*/path" }); },
    (value: any) => { delete value.coordinates[0].representation; },
  ]) {
    const changed = structuredClone(contract);
    alter(changed);
    expect(() => run(changed)).toThrow(/frozen-coordinate-evidence-invalid/u);
  }
  const invalid = Buffer.concat([bytes, Buffer.from([0xff])]);
  expect(() => run({ ...contract, sha256: createHash("sha256").update(invalid).digest("hex") }, invalid)).toThrow(/UTF-8/u);
  const object = '{"path":"tests/source.rs"}', plain = { path, sha256: sha(object), schemaVersion: null, coordinates: [{ pointer: "/path", kind: "source" as const }] };
  const node = findNodeAtLocation(parseTree(object)!, ["path"])!;
  expect(frozenCoordinateEvidenceCoordinates(path, Buffer.from(object), { history: plain })).toEqual([{ pointer: "/path", start: node.offset + 1, end: node.offset + node.length - 1, value: node.value, kind: "source" }]);
  expect(() => run({ ...contract, sha256: sha(object) }, Buffer.from(object))).toThrow(/root/u);
});

test("one exact encoded historical source is registered without changing the previous 38 JSON contracts", () => {
  const contracts = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts;
  expect(contracts[historical.id]).toEqual(historical.contract);
  expect(validateFrozenCoordinateEvidenceContracts(contracts)).toEqual([]);
  const original = Object.fromEntries(Object.entries(contracts).filter(([id]) => id !== historical.id));
  expect(Object.keys(original)).toHaveLength(historical.originalContracts.count);
  expect(sha(canonicalJson(original))).toBe(historical.originalContracts.canonicalSha256);
});

test("the genuine 164-entry snapshot preserves the exact escaped source span and physical bytes", () => {
  let physical = root;
  const parts = historical.contract.path.split("/");
  expect(/^(?:compose|temp\/compose)(?:\/|$)/u.test(historical.contract.path)).toBe(false);
  for (const [index, part] of parts.entries()) {
    expect(part !== "" && part !== "." && part !== "..").toBe(true);
    physical = join(physical, part);
    const stat = lstatSync(physical);
    expect(stat.isSymbolicLink()).toBe(false);
    expect(index === parts.length - 1 ? stat.isFile() : stat.isDirectory()).toBe(true);
  }
  const stat = lstatSync(physical), bytes = readFileSync(physical), source = bytes.toString("utf8"), tree = parseTree(source)!;
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(historical.contract.sha256);
  expect(bytes.length).toBe(historical.size);
  expect(stat.mode & 0o7777).toBe(historical.mode);
  expect(JSON.parse(source)).toHaveLength(historical.rootEntries);
  expect(getNodeValue(tree)).toEqual(JSON.parse(source));
  const node = findNodeAtLocation(tree, [16, "fixtures", 0, "path"])!, raw = source.slice(node.offset + 1, node.offset + node.length - 1);
  expect(node.type).toBe("string");
  expect(JSON.parse('"' + raw + '"')).toBe(node.value);
  expect({ start: node.offset + 1, end: node.offset + node.length - 1, rawSha256: sha(raw), valueSha256: sha(node.value), rawSize: Buffer.byteLength(raw), valueSize: Buffer.byteLength(node.value) }).toEqual(historical.coordinate);
  expect(frozenCoordinateEvidenceCoordinates(historical.contract.path, bytes, { [historical.id]: historical.contract })).toEqual([{ pointer: historical.contract.coordinates[0].pointer, start: historical.coordinate.start, end: historical.coordinate.end, value: raw, kind: "source" }]);
  expect(readFileSync(physical)).toEqual(bytes);
  expect(lstatSync(physical).mode).toBe(stat.mode);
});

test("both historical source gates are mounted through Nx and exact launch registrations", () => {
  const projectSource = readFileSync(join(libraryRoot, "📦️packages/🟦️typescript/📋️project.json"), "utf8"), project = JSON.parse(projectSource);
  expect(getNodeValue(parseTree(projectSource)!)).toEqual(project);
  for (const row of vector.execution) {
    expect(project.targets["test-" + row.id]?.options.command).toBe("bun ./📜️script.ts test " + row.id);
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const document = getNodeValue(parseTree(readFileSync(join(root, path), "utf8"))!), matches = document.configurations.filter((entry: any) => entry.name === row.name);
      expect(matches).toEqual([{ name: row.name, type: "node-terminal", request: "launch", command: "bun nx run @semio-tech/repo-lib:test-" + row.id + " --skip-nx-cache", cwd: "${workspaceFolder}", presentation: { group: "4_gate", order: row.order } }]);
    }
  }
});
