/** 🪐️ Space document admission: language-neutral source cases agree with an independent Ajv oracle, and hostile caller batches are refused. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import admissionSchema from "../../🧬️schema/🔣️.json";
import vectors from "../🧫️fixtures/🔣️.json";
import { prepareSpaceDocumentSources } from "../../🟦️.ts";
import { validateJsonSchemaSubset } from "@semio-tech/framework/schema";

type Sources = Parameters<typeof prepareSpaceDocumentSources>[0];
const codec = (format: string, text: string) => {
  if (format === "json") return text;
  if (format === "dsl" && text === "shape id=owned") return '{"schema":"shape.v1","id":"owned"}';
  throw new Error("test codec refuses malformed source");
};

describe("Space document admission is caller-owned", () => {
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
    for (const vector of vectors.vectors) expect(JSON.parse(prepareSpaceDocumentSources([vector.source] as Sources, codecs)[0].document)).toEqual(vector.expected);
    for (const vector of vectors.invalid) expect(() => prepareSpaceDocumentSources(vector.sources as Sources, codecs), vector.id).toThrow();
    const source = vectors.vectors[0]!.source as Sources[number];
    expect(() => prepareSpaceDocumentSources([source, source], codecs)).toThrow();
    expect(() => prepareSpaceDocumentSources([source], [...codecs, ...codecs])).toThrow();
  });
});
