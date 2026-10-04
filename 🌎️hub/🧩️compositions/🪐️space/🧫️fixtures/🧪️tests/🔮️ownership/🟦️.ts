import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import contract from "../../🧫️fixtures/🔮️ownership/🔣️.json";
import ownershipSchema from "../../🧬️schema/🔮️ownership/🔣️.json";
import admissionSchema from "../../../../../../✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🧬️schema/🔣️.json";
import vectors from "../../../../../../✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🧫️fixtures/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const hash = (path: string) => createHash("sha256").update(readFileSync(resolve(root, path))).digest("hex");
const codec = (format: string, text: string) => {
  if (format === "json") return text;
  if (format === "dsl" && text === "shape id=owned") return '{"schema":"shape.v1","id":"owned"}';
  throw new Error("fixture codec refuses malformed source");
};

describe("Space fixtures enter through selected typed source owners", () => {
  test("closed ownership and source formats agree with existing independent Ajv", () => {
    expect(validateJsonSchemaSubset(ownershipSchema, contract)).toEqual([]);
    expect(new Ajv().validate(ownershipSchema, contract)).toBe(true);
    for (const vector of vectors.vectors) {
      expect(validateJsonSchemaSubset(admissionSchema, [vector.source])).toEqual([]);
      expect(new Ajv().validate(admissionSchema, [vector.source])).toBe(true);
    }
    for (const value of [[], [{ slug: "owned", format: "xml", codec: "owned", text: "owned" }]]) {
      expect(validateJsonSchemaSubset(admissionSchema, value).length).toBeGreaterThan(0);
      expect(new Ajv().validate(admissionSchema, value)).toBe(false);
    }
  });

  test("actual lower Space core and fixture operation depend on no foreign artifact source", () => {
    expect(inspectRustCompileReferences(read(`${contract.core}/🦀️.rs`)).filter(({ path }) => /🖍️draw|✒️writer/u.test(path))).toEqual([]);
    expect(existsSync(resolve(root, contract.lowerRust))).toBe(true);
    const source = read(contract.lowerRust);
    expect(inspectRustCompileReferences(source).filter(({ path }) => path.includes("🗿️artifacts"))).toEqual([]);
    expect(source).toContain("pub enum SpaceFixtureFormat");
    expect(source).toContain("pub struct SpaceFixtureSource");
    expect(source).toContain("pub struct SpaceFixtureCodec");
    expect(source).toContain("pub fn prepare_space_fixture_sources");
    expect(source).toContain("register_os_fixture_documents");
    expect(source).not.toMatch(/draw|writer|Demo Studio|semio\.draw|jack/u);
    const nativePath = resolve(dirname(resolve(root, contract.lowerRust)), "🧪️tests/🔬️unit/🦀️.rs");
    const actualNative = readFileSync(nativePath, "utf8");
    const references = inspectRustCompileReferences(actualNative);
    expect(references.length).toBeGreaterThan(0);
    for (const reference of references) expect(existsSync(resolve(dirname(nativePath), reference.path)), reference.path).toBe(true);
  });

  test("generic source admission has a real TypeScript implementation and refuses all hostile inputs", async () => {
    expect(existsSync(resolve(root, contract.lowerTypescript))).toBe(true);
    const { prepareSpaceFixtureSources } = await import(resolve(root, contract.lowerTypescript));
    const codecs = [{ id: "shape-v1", decode: codec }];
    for (const vector of vectors.vectors) expect(JSON.parse(prepareSpaceFixtureSources([vector.source], codecs)[0].document)).toEqual(vector.expected);
    for (const vector of vectors.invalid) expect(() => prepareSpaceFixtureSources(vector.sources, codecs), vector.id).toThrow();
    const source = vectors.vectors[0]!.source;
    expect(() => prepareSpaceFixtureSources([source, source], codecs)).toThrow();
    expect(() => prepareSpaceFixtureSources([source], [...codecs, ...codecs])).toThrow();
    expect(() => prepareSpaceFixtureSources([source, vectors.invalid.find(({ id }) => id === "malformed-document")!.sources[0]], codecs)).toThrow();
  });

  test("canonical OS admission validates a whole batch and exposes only owned string interfaces", () => {
    const source = read(`${contract.host}/🦀️.rs`);
    for (const retired of contract.retired.slice(1)) expect(source).not.toContain(retired);
    expect(source).toContain("pub fn register_os_fixture_documents");
    expect(source).toContain("pub fn os_fixture_document");
    const start = source.indexOf("pub fn register_os_fixture_documents"), end = source.indexOf("pub fn os_fixture_document", start), body = source.slice(start, end);
    expect(body.indexOf("serde_json::from_str")).toBeGreaterThan(0);
    expect(body.indexOf("serde_json::from_str")).toBeLessThan(body.indexOf("os_fixture_document_registry().lock()"));
    expect(body).toContain("Result<(), String>");
  });

  test("higher actual Space composition owns all real provider selection and admission ordering", () => {
    const manifest = toml.parse(read(`${contract.hub}/📦️packages/🦀️rust/Cargo.toml`)) as { dependencies: Record<string, unknown> };
    for (const example of contract.examples) expect(manifest.dependencies[example.package]).toBeDefined();
    const source = read(`${contract.hub}/🦀️.rs`), start = source.indexOf("pub fn plugin()");
    expect(source.slice(start).indexOf("demo_fixtures::admit()")).toBeLessThan(source.slice(start).indexOf("Plugin::<SpaceApps>::builder"));
    expect(source.slice(start)).toContain("demo_fixtures::admit()");
    const fixture = read(`${contract.owner}/🦀️.rs`);
    for (const example of contract.examples) {
      expect(fixture).toContain(`${example.library}::examples::demo::source()`);
      expect(fixture).toContain(example.slug);
      expect(fixture).toContain(example.snapshot);
      expect(fixture).toContain(example.codec);
    }
    expect(fixture).toContain("ArtifactDsl>::parse_dsl");
    expect(fixture).toContain("store::os_pack::json::to_json_string");
    expect(inspectRustCompileReferences(fixture).filter(({ path }) => path.includes("🗿️artifacts"))).toEqual([]);
  });

  test("export receives the actual admitted demo document with explicit failure and no empty substitute", () => {
    const source = read(`${contract.hub}/⚙️engine/🪐️space/🎮️commands/📤️export-media/🦀️.rs`);
    expect(source).toContain("crate::demo_fixtures::document_for_app");
    expect(source).not.toContain("schema_json");
    expect(source).not.toContain("Value::Object(Default::default())");
    expect(source).toContain("materialize_os_app_instance_document_json");
    expect(source).toContain("export_os_app_instance_media_kind");
  });

  test("every production delta is exactly the declared transformation of retained original bytes", () => {
    for (const row of contract.modified) {
      expect(createHash("sha256").update(row.original).digest("hex"), row.path).toBe(row.sha256);
      let expected = row.original;
      for (const rewrite of contract.transformations.filter(({ path }) => path === row.path)) {
        expect(expected.split(rewrite.previous).length, row.path).toBe(2);
        expected = expected.replace(rewrite.previous, rewrite.current);
      }
      expect(read(row.path), row.path).toBe(expected);
    }
  });

  test("all original unrelated seed/export laws and unique foreign example bytes remain unchanged", () => {
    const changed = new Set(contract.modified.map(({ path }) => path));
    for (const row of contract.originalInputs.filter(({ path }) => !changed.has(path))) expect(hash(row.path), row.path).toBe(row.sha256);
    for (const example of contract.examples) {
      expect(hash(example.asset)).toBe(example.assetSha256);
      expect(hash(example.example)).toBe(example.exampleSha256);
      expect(read(example.example)).toContain("PRIMARY_TEXT");
    }
  });
});
