#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ Second, independent reader of the `s.stdio.semio@v1/✉️base` ENVELOPE's JSON carrier.
//
// The platform's own `JSON.parse` reads every committed vector this subset registers, and Ajv 8 — a
// third-party JSON Schema validator no production code links — holds each document to the carrier the
// subset PUBLISHES (`../🧬️schema/📸️snapshot/🔣️.json`, `../🧬️schema/🧬️mutations/🔣️.json` and every
// schema they reference). Then the envelope's published law is applied in TypeScript, sharing no code
// with the Rust subject or the json-rust oracle: `setSnapshot` replaces the envelope with its payload,
// an `apply<Arm>` wrapper whose arm does not match is refused with `mutation.target-missing`. Those two
// laws are what the carrier can express, so those vectors are REPRODUCED from `before` + `mutation`
// and compared with the committed `after`. What a delegated arm verb does to its arm is the arm's own
// semantics; for those vectors this reader checks the routing (the wrapper names the arm both envelopes
// carry, and the arm really changed) and the schema conformance of all three documents, and says so.
//
// Usage — one typed report on stdout:
//   bun 📜️script.ts carrier-reproduce
//
// @see ../🔮️oracles/🔣️.json — the registered vectors this reads, and the oracle registration
// @see ../🔮️oracles/🦀️.rs — the json-rust reader the parity case runs
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport

//#endregion 🧲️Header

//#region 🔌️Adapters
import Ajv, { type ValidateFunction } from "ajv";
import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🔬️ The typed report this probe emits. */
type ProbeReport = {
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion: string;
  engine: { family: string; implementation: string; version: string };
  status: "ok" | "failed";
  durationMs: number;
  measurements: Record<string, unknown>;
  diagnostics?: { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
};

/** 🧫️ One registered fixture manifest of this subset, reduced to what the reader consumes. */
type FixtureManifest = { id: string; mutation: string; outcome?: string; files: { role: string; path: string }[] };

/** 🧭️ The routed result the envelope law gives for one vector. */
type Routed = { envelope: unknown; refused: string[] };

const ENGINE = { family: "ajv", implementation: "Ajv 8 JSON Schema validator over the platform JSON.parse", version: "8.20.0" } as const;
const SCHEMA_ROOT = "https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/base";
const TARGET_MISSING = "mutation.target-missing";
const SUBSET_DIR = resolve(import.meta.dir, "..");
const REPO_ROOT = resolve(SUBSET_DIR, "../../../../../../../../..");
//#endregion 🧬️Contract

//#region 📖️Schemas
/** 📖️ Every published schema the envelope carrier reaches: the semio subsets and the os store/io contracts they reference. */
const SCHEMA_TREES = [resolve(SUBSET_DIR, ".."), join(REPO_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"), join(REPO_ROOT, "🧰️framework/🔨️modules/🚪️io")];
const SKIPPED_DIRECTORIES = new Set(["🧫️fixtures", "🖼️assets", "🧪️tests", "📚️examples", "target", "node_modules"]);

function readJson(path: string): unknown {
  return JSON.parse(readFileSync(path, "utf8"));
}

function registerSchemas(ajv: Ajv, directory: string, registered: Set<string>): void {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (!SKIPPED_DIRECTORIES.has(entry.name)) registerSchemas(ajv, path, registered);
      continue;
    }
    if (entry.name !== "🔣️.json" || !path.includes("🧬️schema")) continue;
    const schema = readJson(path) as { $id?: unknown };
    if (typeof schema.$id !== "string" || registered.has(schema.$id)) continue;
    registered.add(schema.$id);
    ajv.addSchema(schema as object);
  }
}

function validators(): { snapshot: ValidateFunction; mutation: ValidateFunction; registered: number } {
  const ajv = new Ajv({ strict: false, allErrors: true });
  const registered = new Set<string>();
  for (const tree of SCHEMA_TREES) registerSchemas(ajv, tree, registered);
  const snapshot = ajv.getSchema(`${SCHEMA_ROOT}/snapshot.json`);
  const mutation = ajv.getSchema(`${SCHEMA_ROOT}/mutations.json`);
  if (snapshot === undefined || mutation === undefined) throw new Error("the envelope's published snapshot or mutation schema is not registered");
  return { snapshot, mutation, registered: registered.size };
}

function conformance(validate: ValidateFunction, value: unknown): string[] {
  if (validate(value)) return [];
  return (validate.errors ?? []).filter((error) => error.keyword !== "oneOf" && error.keyword !== "const" && error.keyword !== "if").map((error) => `${error.instancePath || "/"} ${error.message}`);
}
//#endregion 📖️Schemas

//#region 🚦️Law
function member(value: unknown, key: string): unknown {
  return value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>)[key] : undefined;
}

function armOf(envelope: unknown): string | undefined {
  const arm = member(member(envelope, "subset"), "subset");
  return typeof arm === "string" ? arm : undefined;
}

/** 🏷️ `applyBrep` → `brep`; `undefined` for anything that is not an `apply<Arm>` wrapper. */
function wrappedArm(tag: string): string | undefined {
  const rest = tag.startsWith("apply") ? tag.slice("apply".length) : "";
  return rest.length > 0 ? rest[0]!.toLowerCase() + rest.slice(1) : undefined;
}

type Json = Record<string, any>;

/** ✂️ Inserts `item` at `index` of a copied list — every positional insert verb's published semantics. */
function inserted(list: readonly unknown[], index: number, item: unknown): unknown[] {
  if (index > list.length) throw new Error(`insert index ${index} is past the end of a ${list.length}-item list`);
  return [...list.slice(0, index), item, ...list.slice(index)];
}

/** 🔍️ The one element whose `key` equals `value`; a missing target is the arm's refusal, never a silent no-op. */
function located(list: readonly Json[], key: string, value: unknown): number {
  const index = list.findIndex((entry) => canonical(entry[key]) === canonical(value));
  if (index < 0) throw new Error(`no element carries ${key} ${JSON.stringify(value)}`);
  return index;
}

/**
 * 📖️ Each arm verb the committed vectors exercise, written from that arm's published leaf schema
 * (`<arm>/🧬️schema/🧬️mutations/<verb>/🧬️schema/🔣️.json`) and never from the Rust that implements it.
 * The key is the arm mutation's wire tag: its `mutation` member when internally tagged, its single
 * member name when externally tagged.
 */
const ARM_LAWS: Readonly<Record<string, (subset: Json, payload: Json) => Json>> = {
  addLayer: (subset, payload) => ({ ...subset, layers: [...subset.layers, payload.layer] }),
  ChangeNodeLabel: (subset, payload) => ({ ...subset, nodes: subset.nodes.map((node: Json, index: number) => (index === located(subset.nodes, "id", payload.id) ? { ...node, label: payload.new_label } : node)) }),
  CreateLayer: (subset, payload) => ({ ...subset, layers: inserted(subset.layers, payload.index, payload.layer) }),
  CreateMaterial: (subset, payload) => ({ ...subset, materials: [...subset.materials, payload.material] }),
  CreateVertex: (subset, payload) => ({ ...subset, vertices: [...subset.vertices, { id: payload.id, point: payload.point, tol: payload.tol }] }),
  EditCell: (subset, payload) => {
    const column = located(subset.columns, "name", payload.column_name);
    return { ...subset, rows: subset.rows.map((row: Json, index: number) => (index === payload.row_index ? { ...row, cells: row.cells.map((cell: unknown, at: number) => (at === column ? payload.new_value : cell)) } : row)) };
  },
  EditRun: (subset, payload) => ({ ...subset, runs: subset.runs.map((run: Json, index: number) => (index === payload.index ? { ...run, content: payload.new_content } : run)) }),
  insertChannel: (subset, payload) => ({ ...subset, channels: inserted(subset.channels, payload.index, payload.channel) }),
  insertImage: (subset, payload) => ({ ...subset, images: [...subset.images, payload.image] }),
  insertListItem: (subset, payload) => ({ ...subset, root: insertedAtPath(subset.root, payload.path, payload.index, payload.value) }),
  insertNode: (subset, payload) => ({ ...subset, nodes: [...subset.nodes, payload.node] }),
  insertSlide: (subset, payload) => ({ ...subset, slides: inserted(subset.slides, payload.index, payload.slide) }),
  insertSpatialNode: (subset, payload) => ({ ...subset, spatial: [...subset.spatial, payload.node] }),
  insertStream: (subset, payload) => ({ ...subset, streams: inserted(subset.streams, payload.index, payload.stream) }),
  insertTimeline: (subset, payload) => ({ ...subset, timelines: inserted(subset.timelines, payload.index, payload.timeline) }),
  MoveObject: (subset, payload) => ({ ...subset, transform: { ...subset.transform, translation: payload.translation } }),
  RenameType: (subset, payload) => ({ ...subset, types: subset.types.map((type: Json, index: number) => (index === located(subset.types, "id", payload.id) ? { ...type, name: payload.new_name } : type)) }),
  setDimensions: (subset, payload) => ({ ...subset, width: payload.width, height: payload.height }),
};

/** 🌳️ A value-tree insert: `path` walks map entries by `key` and list items by `index`, then the list there takes `value` at `index`. */
function insertedAtPath(node: Json, path: readonly Json[], index: number, value: unknown): Json {
  if (path.length === 0) return { ...node, items: inserted(node.items, index, value) };
  const [step, ...rest] = path;
  if (step!.kind === "key") return { ...node, entries: node.entries.map((entry: Json) => (entry.key === step!.key ? { ...entry, value: insertedAtPath(entry.value, rest, index, value) } : entry)) };
  return { ...node, items: node.items.map((item: Json, at: number) => (at === step!.index ? insertedAtPath(item, rest, index, value) : item)) };
}

/** 🏷️ An arm mutation's wire tag and payload, whichever tagging its arm publishes. */
function armVerb(inner: Json): { verb: string; payload: Json } {
  if (typeof inner.mutation === "string") return { verb: inner.mutation, payload: inner };
  const keys = Object.keys(inner);
  if (keys.length !== 1) throw new Error(`arm mutation ${JSON.stringify(keys)} is neither internally nor externally tagged`);
  return { verb: keys[0]!, payload: inner[keys[0]!] };
}

/** ▶️ The envelope law, then the arm's own verb where this implementation states it; `undefined` when it does not. */
function route(before: unknown, mutation: unknown): Routed | undefined {
  const tag = member(mutation, "mutation");
  if (tag === "setSnapshot") return { envelope: member(member(mutation, "payload"), "snapshot"), refused: [] };
  const arm = typeof tag === "string" ? wrappedArm(tag) : undefined;
  if (arm === undefined) throw new Error(`mutation tag ${JSON.stringify(tag)} is neither setSnapshot nor an apply<Arm> wrapper`);
  if (armOf(before) !== arm) return { envelope: before, refused: [TARGET_MISSING] };
  const { verb, payload } = armVerb(member(member(mutation, "payload"), "mutation") as Json);
  const law = ARM_LAWS[verb];
  if (law === undefined) return undefined;
  const envelope = before as Json;
  return { envelope: { ...envelope, subset: law(envelope.subset, payload) }, refused: [] };
}

/** 🔣️ Key-order-free identity: the carrier's objects are unordered, its arrays are not. */
function canonical(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value !== null && typeof value === "object")
    return `{${Object.keys(value)
      .sort()
      .map((key) => `${JSON.stringify(key)}:${canonical((value as Record<string, unknown>)[key])}`)
      .join(",")}}`;
  return JSON.stringify(value);
}
//#endregion 🚦️Law

//#region 🔬️Probe
function carrierReproduce(): ProbeReport {
  const started = Date.now();
  const contributionDir = join(SUBSET_DIR, "🔮️oracles");
  const manifests = (readJson(join(contributionDir, "🔣️.json")) as { fixtureManifests: FixtureManifest[] }).fixtureManifests;
  const { snapshot, mutation, registered } = validators();
  const diagnostics: NonNullable<ProbeReport["diagnostics"]> = [];
  const rows = manifests.map((manifest) => {
    const file = (role: string): unknown => {
      const entry = manifest.files.find((candidate) => candidate.role === role);
      if (entry === undefined) throw new Error(`${manifest.id} registers no ${role}`);
      return readJson(resolve(contributionDir, entry.path));
    };
    const before = file("expected-before-json");
    const after = file("expected-after-json");
    const wrapped = file("mutation-json");
    const schema = { before: conformance(snapshot, before), mutation: conformance(mutation, wrapped), after: conformance(snapshot, after) };
    const routed = route(before, wrapped);
    const expressible = routed !== undefined;
    const reproduced = expressible ? canonical(routed.envelope) === canonical(after) : null;
    const refused = expressible ? routed.refused : [];
    const armTag = member(wrapped, "mutation");
    const routedToArm = expressible || (armOf(before) === wrappedArm(String(armTag)) && armOf(after) === armOf(before) && canonical(before) !== canonical(after));
    const conforms = schema.before.length === 0 && schema.mutation.length === 0 && schema.after.length === 0;
    if (!conforms) diagnostics.push({ severity: expressible ? "error" : "warning", message: `${manifest.id}: a committed document does not conform to the published carrier schema`, detail: JSON.stringify(schema) });
    if (expressible && !reproduced) diagnostics.push({ severity: "error", message: `${manifest.id}: the envelope law does not reproduce the committed after-envelope` });
    if (!routedToArm) diagnostics.push({ severity: "error", message: `${manifest.id}: the wrapper does not route into the arm both envelopes carry, or the arm did not change` });
    return { id: manifest.id, mutation: manifest.mutation, expressible, conforms, reproduced, refused, routedToArm, schema };
  });
  const errors = diagnostics.filter((entry) => entry.severity === "error").length;
  return {
    schema: "semio.repository-test.probe-report/v2",
    probe: "semio-base-carrier-reproduce",
    probeVersion: "ajv@8.20.0",
    engine: ENGINE,
    status: errors === 0 ? "ok" : "failed",
    durationMs: Date.now() - started,
    measurements: {
      registeredSchemas: registered,
      vectors: rows.length,
      reproduced: rows.filter((row) => row.reproduced === true).length,
      expressible: rows.filter((row) => row.expressible).length,
      conforming: rows.filter((row) => row.conforms).length,
      rows,
    },
    diagnostics,
  };
}

function main(argv: readonly string[]): number {
  if (argv[0] !== "carrier-reproduce") {
    console.error(`[probe] unknown command ${JSON.stringify(argv[0] ?? "")} — expected carrier-reproduce`);
    return 2;
  }
  const report = carrierReproduce();
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  return report.status === "ok" ? 0 : 1;
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export { carrierReproduce };
//#endregion 🔬️Probe
