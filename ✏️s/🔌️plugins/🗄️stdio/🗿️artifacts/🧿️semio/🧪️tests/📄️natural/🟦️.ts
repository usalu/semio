/** 📄️ Metadata-only source contract; actual SDK byte execution belongs to Rust. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync, readdirSync, existsSync } from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
const root = new URL("../../🏅️standards/🔖️v1/🪆️subsets/", import.meta.url);
const sources = readdirSync(root).flatMap(directory => { const path = new URL(directory + "/✏️editor/🦀️.rs", root); return existsSync(path) ? [{ directory, source: readFileSync(path, "utf8") }] : []; });
const editorBody = (source: string): string => { const start = source.indexOf("impl ArtifactEditor for "); const open = source.indexOf("{", start); let depth = 1, end = open + 1; while (depth && end < source.length) { if (source[end] === "{") depth++; if (source[end] === "}") depth--; end++; } return source.slice(open + 1, end - 1); };
test("validates nineteen neutral native metadata rows with independent Ajv", () => { const validate = new Ajv().compile(schema); expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true); expect(new Set(fixture.rows.map(row => row.subset)).size).toBe(19); });
test("matches the exact authored nineteen source schemas", () => { expect(sources.map(row => row.source.match(/pub const SEMIO_[A-Z_]+DOCUMENT_SCHEMA: &str = "([^"]+)"/)![1]).sort()).toEqual(fixture.rows.map(row => row.schema).sort()); });
test("declares natural codecs inside each actual ArtifactEditor implementation", () => { const missing: string[] = []; for (const row of fixture.rows) { const source = sources.find(entry => entry.source.includes("= \"" + row.schema + "\";"))!.source; const body = editorBody(source); const match = body.match(/semio_natural_file!\([^,]+,\s*"([^"]+)"/); if (!match) { missing.push(row.schema); continue; } expect({ formatKind: match[1] + "@v1", extension: ".semio", mediaType: "application/vnd.semio", binary: true }).toEqual({ formatKind: row.formatKind, extension: row.extension, mediaType: row.mediaType, binary: row.binary }); } expect(missing).toEqual([]); });
