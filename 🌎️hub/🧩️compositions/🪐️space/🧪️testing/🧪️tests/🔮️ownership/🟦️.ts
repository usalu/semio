import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import contract from "../../🧫️fixtures/🔮️ownership/🔣️.json";
import admissionSchema from "../../../../../../✏️s/🔌️plugins/🪐️space/🫀️core/📄️documents/🧬️schema/🔣️.json";
import vectors from "../../../../../../✏️s/🔌️plugins/🪐️space/🫀️core/📄️documents/🧫️fixtures/🔣️.json";
import { prepareSpaceDocumentSources } from "../../../../../../✏️s/🔌️plugins/🪐️space/🫀️core/📄️documents/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const codec = (format: string, text: string) => {
  if (format === "json") return text;
  if (format === "dsl" && text === "shape id=owned") return '{"schema":"shape.v1","id":"owned"}';
  throw new Error("test codec refuses malformed source");
};

describe("Space runtime documents are caller-owned and testing inputs remain test-only", () => {
  test("language-neutral source cases agree with the independent Ajv domain-schema oracle", () => {
    for (const vector of vectors.vectors) {
      expect(validateJsonSchemaSubset(admissionSchema, [vector.source])).toEqual([]);
      expect(new Ajv().validate(admissionSchema, [vector.source])).toBe(true);
    }
    for (const value of [[], [{ slug: "owned", format: "xml", codec: "owned", text: "owned" }]]) {
      expect(validateJsonSchemaSubset(admissionSchema, value).length).toBeGreaterThan(0);
      expect(new Ajv().validate(admissionSchema, value)).toBe(false);
    }
  });

  test("document preparation decodes caller inputs and refuses every hostile batch", () => {
    const codecs = [{ id: "shape-v1", decode: codec }];
    for (const vector of vectors.vectors) expect(JSON.parse(prepareSpaceDocumentSources([vector.source], codecs)[0].document)).toEqual(vector.expected);
    for (const vector of vectors.invalid) expect(() => prepareSpaceDocumentSources(vector.sources as Parameters<typeof prepareSpaceDocumentSources>[0], codecs), vector.id).toThrow();
    const source = vectors.vectors[0]!.source;
    expect(() => prepareSpaceDocumentSources([source, source], codecs)).toThrow();
    expect(() => prepareSpaceDocumentSources([source], [...codecs, ...codecs])).toThrow();
  });

  test("production composition and host have no testing input admission or registry", () => {
    const source = read(`${contract.hub}/🦀️.rs`);
    expect(source).toMatch(/#\[cfg\(test\)\]\s*#\[path = "🧪️testing\/🦀️\.rs"\]\s*mod test_documents;/u);
    expect(source.slice(source.indexOf("pub fn plugin()"))).not.toContain("test_documents::");
    const host = read(`${contract.host}/🦀️.rs`);
    expect(host).not.toMatch(/OS_FIXTURE_DOCUMENTS|register_os_fixture_documents|os_fixture_document/u);
    expect(read(contract.lowerRust)).not.toMatch(/fixture|draw|writer/u);
  });

  test("export requires the selected user's document instead of a bundled testing example", () => {
    const source = read(`${contract.hub}/⚙️engine/🪐️space/🎮️commands/📤️export-media/🦀️.rs`);
    expect(source).toContain("pub document_json: String");
    expect(source).toContain("payload.document_json.parse::<Value>()");
    expect(source).not.toContain("test_documents");
    expect(source).toContain("materialize_os_app_instance_document_json(&payload.document_json");
    expect(source).toContain("export_os_app_instance_media_kind");
  });
});
