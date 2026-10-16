import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import contract from "../🧫️fixtures/🔮️ownership/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");

describe("Space testing inputs remain test-only in the production composition", () => {
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
