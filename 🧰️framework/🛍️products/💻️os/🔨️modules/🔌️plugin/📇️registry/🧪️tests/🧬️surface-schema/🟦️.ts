import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import { buildSchema, parse } from "graphql";
import { describe, expect, it } from "vitest";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { policyExtractGraphqlSchemaFields } from "../../../../../../🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🔗️graphql/🟦️.ts";
import { policyExtractJsonSchemaFields } from "../../../../../../🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🔣️json-schema/🟦️.ts";
import { policyExtractProtobufSchemaFields } from "../../../../../../🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🛰️protobuf/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../../../../🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";
import type { PolicySchemaLeafExtract } from "../../../../../../🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts";
import { GRAPHQL_STATE_PREAMBLE } from "../../../../../../../🔨️modules/🧬️schema/🟦️.ts";
import { SURFACE_SCHEMA_BANNER, projectSurfaceSchemaLane, resetSurfaceSchemaIndexes, resolveSurfaceSchemaDefinitions, surfaceSchemaBinding, surfaceSchemaLaneOwnership, surfaceSchemaProjectionBlocker, surfaceSchemaTypeTail, type SurfaceSchemaLane, type SurfaceSchemaOwner } from "../../🧬️surface-schema/🟦️.ts";

type FixtureCase = {
  readonly id: string;
  readonly lane: SurfaceSchemaLane;
  readonly appId: string;
  readonly typeName: string;
  readonly sourceRel: string;
  readonly source: string;
  readonly definitions?: readonly { readonly typeName: string; readonly source: string }[];
  readonly expected: Readonly<Record<string, string>>;
};

const repoRoot = getWorkspaceRoot();
const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/🧬️surface-schema/🔣️.json"), "utf8")) as { readonly version: number; readonly cases: readonly FixtureCase[] };

const ownerOf = (row: FixtureCase): SurfaceSchemaOwner => {
  const laneDir = row.lane === "config" ? "🎚️config" : "👥️presence";
  const ownerAbs = join(repoRoot, "✏️s/🔌️plugins/🧪️demo");
  return {
    pluginId: "🧪️demo", pluginRoot: ownerAbs, ownerAbs, ownerLabel: "fixture", surfaceAbs: null,
    lane: row.lane, laneAbs: join(ownerAbs, laneDir), facetAbs: join(ownerAbs, laneDir, "🧬️schema"),
    typeName: row.typeName, appId: row.appId,
  };
};

const definitionsOf = (row: FixtureCase): PolicySchemaLeafExtract[] =>
  (row.definitions ?? []).map((definition) => policyExtractRustSchemaFields(definition.source, definition.typeName));

/** 🧪️ One instance a schema node accepts, resolving `$ref` into the document's own `$defs` and
 * honouring a declared `minimum` — the 64-bit lanes declare one, so a blind `1` would not prove the
 * bound is real. */
const conformingValue = (node: Record<string, unknown>, document: Record<string, unknown>): unknown => {
  if (typeof node.$ref === "string") {
    const defs = (document.$defs ?? {}) as Record<string, Record<string, unknown>>;
    return conformingValue(defs[node.$ref.split("/").pop()!]!, document);
  }
  if (node.type === "array") return [];
  if (node.type === "string") return "x";
  if (node.type === "boolean") return true;
  if (node.type === "integer" || node.type === "number") return typeof node.minimum === "number" ? node.minimum : 1;
  const properties = (node.properties ?? {}) as Record<string, Record<string, unknown>>;
  return Object.fromEntries(((node.required ?? []) as string[]).map((name) => [name, conformingValue(properties[name]!, document)]));
};

describe("surface-schema-projection", () => {
  it("projects every fixture case to the exact frozen bytes", () => {
    expect(fixture.cases.length).toBeGreaterThan(0);
    for (const row of fixture.cases) {
      const extract = policyExtractRustSchemaFields(row.source, row.typeName);
      expect(extract.typeName, row.id).toBe(row.typeName);
      expect(surfaceSchemaProjectionBlocker(extract, definitionsOf(row)), row.id).toBeNull();
      expect(projectSurfaceSchemaLane(repoRoot, ownerOf(row), extract, row.sourceRel, definitionsOf(row)), row.id).toEqual(row.expected);
    }
  });

  it("emits every leaf the taxonomy declares for a 🧬️data facet", () => {
    for (const row of fixture.cases) {
      expect(Object.keys(row.expected).sort(), row.id).toEqual(["🔗️.graphql", "🔣️.json", "🛰️.proto", "🟦️.ts", "🦀️.rs"].sort());
    }
  });

  it("validates every projected JSON Schema with Ajv, a third-party draft-07 implementation", () => {
    for (const row of fixture.cases) {
      const document = JSON.parse(row.expected["🔣️.json"]!) as Record<string, unknown>;
      const ajv = new Ajv({ strict: false });
      const validate = ajv.compile(document);
      const conforming = conformingValue(document, document) as Record<string, unknown>;
      expect(validate(conforming), `${row.id}: ${ajv.errorsText(validate.errors)}`).toBe(true);
      expect(validate({ ...conforming, "not-a-declared-field": 1 }), `${row.id} must reject undeclared fields`).toBe(false);
    }
  });

  it("parses every projected GraphQL leaf with the graphql reference implementation", () => {
    for (const row of fixture.cases) {
      const sdl = `${GRAPHQL_STATE_PREAMBLE}\n${row.expected["🔗️.graphql"]!}`;
      expect(() => parse(sdl), row.id).not.toThrow();
      const built = buildSchema(`${sdl}\ntype Query { probe: Boolean }`);
      expect(built.getType(row.typeName), row.id).toBeDefined();
    }
  });

  it("compiles every projected Rust leaf with rustc, standalone and dependency-free", () => {
    const capture = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/surface-schema-check");
    rmSync(capture, { recursive: true, force: true });
    for (const row of fixture.cases) {
      const crateDir = join(capture, row.id);
      mkdirSync(crateDir, { recursive: true });
      const entry = join(crateDir, "leaf.rs");
      mkdirSync(dirname(entry), { recursive: true });
      writeFileSync(entry, row.expected["🦀️.rs"]!);
      const compiled = spawnSync("rustc", ["--crate-type", "lib", "--edition", "2021", "--emit=metadata=leaf.rmeta", "-D", "warnings", "leaf.rs"], { cwd: crateDir, encoding: "utf8" });
      expect(compiled.status, `${row.id}: ${compiled.stderr}`).toBe(0);
    }
  });

  it("keeps the five leaves field-identical when read back by the repo library's own parsers", () => {
    for (const row of fixture.cases) {
      const canonical = policyExtractJsonSchemaFields(row.expected["🔣️.json"]!);
      const mirrors = {
        "🦀️.rs": policyExtractRustSchemaFields(row.expected["🦀️.rs"]!, row.typeName),
        "🔗️.graphql": policyExtractGraphqlSchemaFields(row.expected["🔗️.graphql"]!, row.typeName),
        "🛰️.proto": policyExtractProtobufSchemaFields(row.expected["🛰️.proto"]!, row.typeName),
      };
      for (const [leaf, extract] of Object.entries(mirrors)) {
        expect(extract.typeName, `${row.id} ${leaf}`).toBe(row.typeName);
        expect(extract.fields.map((field) => `${field.name}:${field.optional}:${field.cardinality}`), `${row.id} ${leaf}`).toEqual(
          canonical.fields.map((field) => `${field.name}:${field.optional}:${field.cardinality}`),
        );
      }
      for (const field of canonical.fields) expect(field.state, `${row.id} ${field.name}`).toBe(row.lane);
    }
  });

  it("reads a surface's associated-type bindings through their last path segment", () => {
    const component = "impl ArtifactEditor for X {\n    type Config = crate::cfg::HomeConfig;\n    type Presence = semio_framework_plugin::NoPresence;\n}";
    expect(surfaceSchemaBinding(component, "Config")).toBe("HomeConfig");
    expect(surfaceSchemaBinding(component, "Presence")).toBe("NoPresence");
    expect(surfaceSchemaTypeTail("a::b::C")).toBe("C");
    expect(surfaceSchemaBinding("no bindings here", "Config")).toBeNull();
  });

  it("cedes a complete authored lane but fills the holes of a partly authored one", () => {
    const leaves = ["🔣️.json", "🦀️.rs", "🟦️.ts", "🔗️.graphql", "🛰️.proto"];
    const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/surface-schema-ownership");
    rmSync(root, { recursive: true, force: true });
    const lane = (id: string, present: Readonly<Record<string, string>>): string => {
      const facetAbs = join(root, id);
      mkdirSync(facetAbs, { recursive: true });
      for (const [filename, body] of Object.entries(present)) writeFileSync(join(facetAbs, filename), body);
      return facetAbs;
    };

    const projected = lane("projected", Object.fromEntries(leaves.map((filename) => [filename, `${SURFACE_SCHEMA_BANNER}\nbody`])));
    expect(surfaceSchemaLaneOwnership(projected, leaves)).toEqual({ authored: [], absent: [], hollow: false, ceded: false });

    const authored = lane("authored", Object.fromEntries(leaves.map((filename) => [filename, "hand carved"])));
    expect(surfaceSchemaLaneOwnership(authored, leaves)).toEqual({ authored: leaves, absent: [], hollow: false, ceded: true });

    const hollow = lane("hollow", { "🔣️.json": "hand carved" });
    const ownership = surfaceSchemaLaneOwnership(hollow, leaves);
    expect(ownership.ceded, "a lane with holes must never be ceded — that is how 🔋️energy kept 4 findings").toBe(false);
    expect(ownership.hollow).toBe(true);
    expect(ownership.authored).toEqual(["🔣️.json"]);
    expect(ownership.absent).toEqual(["🦀️.rs", "🟦️.ts", "🔗️.graphql", "🛰️.proto"]);

    const empty = lane("empty", {});
    expect(surfaceSchemaLaneOwnership(empty, leaves)).toEqual({ authored: [], absent: leaves, hollow: false, ceded: false });
    rmSync(root, { recursive: true, force: true });
  });

  it("bounds every 64-bit integer lane to the exact range JSON and JavaScript share", () => {
    const row = fixture.cases.find((entry) => entry.id === "wide-integer-config")!;
    const document = JSON.parse(row.expected["🔣️.json"]!) as Record<string, unknown>;
    const properties = document.properties as Record<string, Record<string, unknown>>;
    expect(properties.generation).toMatchObject({ type: "integer", format: "uint64", minimum: 0, maximum: Number.MAX_SAFE_INTEGER });
    expect(properties.drift).toMatchObject({ type: "integer", format: "int64", minimum: -Number.MAX_SAFE_INTEGER, maximum: Number.MAX_SAFE_INTEGER });
    expect(row.expected["🦀️.rs"]).toContain("pub generation: u64,");
    expect(row.expected["🛰️.proto"]).toContain("uint64 generation = 1;");
    expect(row.expected["🔗️.graphql"]).toContain("generation: Long! @state(class: CONFIG)");
    expect(GRAPHQL_STATE_PREAMBLE, "Long is a 64-bit lane the shared preamble must declare — GraphQL Int is 32-bit by specification").toContain("scalar Long");

    const ajv = new Ajv({ strict: false });
    const validate = ajv.compile(document);
    const conforming = conformingValue(document, document) as Record<string, unknown>;
    expect(validate({ ...conforming, generation: Number.MAX_SAFE_INTEGER })).toBe(true);
    expect(validate({ ...conforming, generation: Number.MAX_SAFE_INTEGER + 1 }), "2^53 must be rejected by the schema, exactly as packValueToExactJson throws on it at runtime").toBe(false);
    expect(validate({ ...conforming, generation: -1 }), "an unsigned lane must reject a negative value").toBe(false);
  });

  it("nests a composite field through $defs and a mirror declaration in every other format", () => {
    const row = fixture.cases.find((entry) => entry.id === "nested-config")!;
    const document = JSON.parse(row.expected["🔣️.json"]!) as Record<string, unknown>;
    expect(Object.keys(document.$defs as Record<string, unknown>), "dependencies are declared before the shape that references them").toEqual(["IndexDialect", "IndexRow"]);
    expect((document.properties as Record<string, Record<string, unknown>>).rows!.items).toEqual({ $ref: "#/$defs/IndexRow" });
    expect(row.expected["🦀️.rs"]!.indexOf("pub struct IndexDialect")).toBeLessThan(row.expected["🦀️.rs"]!.indexOf("pub struct IndexRow"));
    expect(row.expected["🟦️.ts"]).toContain("rows: readonly IndexRow[];");
    expect(row.expected["🔗️.graphql"]).toContain("rows: [IndexRow!]! @state(class: CONFIG)");
    expect(row.expected["🛰️.proto"]).toContain("repeated IndexRow rows = 2;");
  });

  it("resolves nested declarations dependencies-first, and refuses a cycle or an undeclared type", () => {
    const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/surface-schema-nesting");
    rmSync(root, { recursive: true, force: true });
    const owner = (pluginRoot: string): SurfaceSchemaOwner => ({
      pluginId: "🧪️demo", pluginRoot, ownerAbs: pluginRoot, ownerLabel: "fixture", surfaceAbs: null,
      lane: "config", laneAbs: join(pluginRoot, "🎚️config"), facetAbs: join(pluginRoot, "🎚️config", "🧬️schema"),
      typeName: "Root", appId: "s.demo.thing",
    });
    const tree = (id: string, source: string): string => {
      const pluginRoot = join(root, id);
      mkdirSync(pluginRoot, { recursive: true });
      writeFileSync(join(pluginRoot, "🦀️.rs"), source);
      return pluginRoot;
    };
    const rootOf = (source: string): PolicySchemaLeafExtract => policyExtractRustSchemaFields(source, "Root");

    resetSurfaceSchemaIndexes();
    const diamondSource = "pub struct Root {\n    pub left: Leaf,\n    pub right: Branch,\n}\npub struct Branch {\n    pub leaf: Leaf,\n}\npub struct Leaf {\n    pub name: String,\n}\n";
    const diamond = resolveSurfaceSchemaDefinitions(repoRoot, owner(tree("diamond", diamondSource)), rootOf(diamondSource));
    expect(diamond.definitions.map((definition) => definition.typeName), "each nested type appears once, after everything it needs").toEqual(["Leaf", "Branch"]);
    expect(diamond.unresolved).toEqual([]);
    expect(diamond.cyclic).toEqual([]);

    resetSurfaceSchemaIndexes();
    const cyclicSource = "pub struct Root {\n    pub node: Node,\n}\npub struct Node {\n    pub child: Node,\n}\n";
    const cyclic = resolveSurfaceSchemaDefinitions(repoRoot, owner(tree("cyclic", cyclicSource)), rootOf(cyclicSource));
    expect(cyclic.cyclic, "a self-referential struct needs indirection the projection does not own").toEqual(["Node"]);

    resetSurfaceSchemaIndexes();
    const danglingSource = "pub struct Root {\n    pub camera: Camera,\n}\n";
    const dangling = resolveSurfaceSchemaDefinitions(repoRoot, owner(tree("dangling", danglingSource)), rootOf(danglingSource));
    expect(dangling.unresolved).toEqual(["camera: Camera"]);
    expect(surfaceSchemaProjectionBlocker(rootOf(danglingSource), dangling.definitions)).toContain("composite field type");
    resetSurfaceSchemaIndexes();
    rmSync(root, { recursive: true, force: true });
  });

  it("refuses to project a shape no cross-format spelling can hold identically", () => {
    const composite = policyExtractRustSchemaFields("pub struct A {\n    pub camera: Camera,\n}", "A");
    expect(surfaceSchemaProjectionBlocker(composite)).toContain("composite field type");
    const mapped = policyExtractRustSchemaFields("pub struct B {\n    pub weights: BTreeMap<String, f64>,\n}", "B");
    expect(surfaceSchemaProjectionBlocker(mapped)).toContain("no cross-format-identical spelling");
    const optionalList = policyExtractRustSchemaFields("pub struct C {\n    pub tags: Option<Vec<String>>,\n}", "C");
    expect(surfaceSchemaProjectionBlocker(optionalList)).toContain("no cross-format-identical spelling");
  });
});
