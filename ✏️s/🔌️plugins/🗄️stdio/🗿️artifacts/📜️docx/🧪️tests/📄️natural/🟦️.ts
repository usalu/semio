import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { strFromU8, strToU8, unzipSync, zipSync } from "fflate";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("./🧫️fixtures/📋️schema.json", import.meta.url), "utf8"));
const artifact = new URL("../../", import.meta.url);
const directories = { strict: "📏️strict", transitional: "🔄️transitional" } as const;
const escape = (text: string) => text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
const bodyOf = (source: string) => {
  const start = source.indexOf("impl ArtifactEditor for ");
  expect(start).toBeGreaterThanOrEqual(0);
  const opening = source.indexOf("{", start);
  let depth = 1, cursor = opening + 1;
  for (; depth && cursor < source.length; cursor++) {
    if (source[cursor] === "{") depth++;
    if (source[cursor] === "}") depth--;
  }
  expect(depth).toBe(0);
  return source.slice(opening + 1, cursor - 1);
};

describe("DOCX neutral native inputs (metadata and independent package proof only)", () => {
  test("validates the exact two variants with independent Ajv", () => {
    expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
    expect(fixture.rows.map((row: any) => row.variant).sort()).toEqual(["strict", "transitional"]);
  });
  for (const row of fixture.rows) {
    test(row.variant + " authored bilingual OPC input preserves declared namespaces", () => {
      const document = '<w:document xmlns:w="' + row.mainNamespace + '"' + (row.variant === "strict" ? ' conformance="strict"' : "") + '><w:body>' +
        ["en", "de"].map(locale => "<w:p><w:r><w:t>" + escape(row.paragraphs[locale]) + "</w:t></w:r></w:p>").join("") + "</w:body></w:document>";
      const relationship = '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="' + row.relationshipBase + '/officeDocument" Target="word/document.xml"/></Relationships>';
      const contentTypes = '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>';
      const encoded = zipSync({ "[Content_Types].xml": strToU8(contentTypes), "_rels/.rels": strToU8(relationship), "word/document.xml": strToU8(document) });
      const independent = unzipSync(encoded);
      expect(Object.keys(independent).sort()).toEqual(["[Content_Types].xml", "_rels/.rels", "word/document.xml"]);
      const decoded = strFromU8(independent["word/document.xml"]);
      expect(decoded.match(/xmlns:w="([^"]+)"/)?.[1]).toBe(row.mainNamespace);
      expect(strFromU8(independent["_rels/.rels"]).match(/Type="([^"]+)"/)?.[1]).toBe(row.relationshipBase + "/officeDocument");
      for (const locale of ["en", "de"]) expect(decoded).toContain("<w:t>" + escape(row.paragraphs[locale]) + "</w:t>");
      expect(decoded.match(/<w:t>/g)?.length).toBe(2);
    });
    test(row.variant + " actual editor declares the expected native metadata", () => {
      const directory = directories[row.variant as keyof typeof directories];
      const source = readFileSync(new URL("🏅️standards/🔖️ecma-376/🪆️subsets/" + directory + "/✏️editor/🦀️.rs", artifact), "utf8");
      const body = bodyOf(source);
      for (const [field, value] of [["format_kind", row.formatKind], ["extension", row.extension], ["media_type", row.mediaType]]) {
        expect(body).toContain(field + ': "' + value + '"');
      }
      expect(body).toContain("binary: true");
      expect(body).toContain("fn encode_natural_file(");
      expect(body).toContain("fn decode_natural_file(");
    });
  }
});
