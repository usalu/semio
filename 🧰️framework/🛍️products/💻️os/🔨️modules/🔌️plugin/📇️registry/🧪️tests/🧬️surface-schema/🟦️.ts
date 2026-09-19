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
import { GRAPHQL_STATE_PREAMBLE } from "../../../../../🔨️modules/🧬️schema/🟦️.ts";
import { projectSurfaceSchemaLane, surfaceSchemaBinding, surfaceSchemaProjectionBlocker, surfaceSchemaTypeTail, type SurfaceSchemaLane, type SurfaceSchemaOwner } from "../../🧬️surface-schema/🟦️.ts";

type FixtureCase = {
  readonly id: string;
  readonly lane: SurfaceSchemaLane;
  readonly appId: string;
  readonly typeName: string;
  readonly sourceRel: string;
  readonly source: string;
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

describe("surface-schema-projection", () => {
  it("projects every fixture case to the exact frozen bytes", () => {
    expect(fixture.cases.length).toBeGreaterThan(0);
    for (const row of fixture.cases) {
      const extract = policyExtractRustSchemaFields(row.source, row.typeName);
      expect(extract.typeName, row.id).toBe(row.typeName);
      expect(surfaceSchemaProjectionBlocker(extract), row.id).toBeNull();
      expect(projectSurfaceSchemaLane(repoRoot, ownerOf(row), extract, row.sourceRel), row.id).toEqual(row.expected);
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
      const properties = document.properties as Record<string, Record<string, unknown>>;
      const conforming = Object.fromEntries(
        (document.required as string[]).map((name) => {
          const property = properties[name]!;
          return [name, property.type === "array" ? [] : property.type === "string" ? "x" : property.type === "boolean" ? true : 1];
        }),
      );
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

  it("refuses to project a shape no cross-format spelling can hold identically", () => {
    const composite = policyExtractRustSchemaFields("pub struct A {\n    pub camera: Camera,\n}", "A");
    expect(surfaceSchemaProjectionBlocker(composite)).toContain("composite field type");
    const mapped = policyExtractRustSchemaFields("pub struct B {\n    pub weights: BTreeMap<String, f64>,\n}", "B");
    expect(surfaceSchemaProjectionBlocker(mapped)).toContain("no cross-format-identical spelling");
    const optionalList = policyExtractRustSchemaFields("pub struct C {\n    pub tags: Option<Vec<String>>,\n}", "C");
    expect(surfaceSchemaProjectionBlocker(optionalList)).toContain("no cross-format-identical spelling");
  });
});
