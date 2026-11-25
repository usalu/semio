/** 🏷️ Tags carry a title in every language: the schema defines `Tag`, every asset of the catalog conforms, and an id reads the same wherever it stands.
 *
 * THE REFERENCE is ajv (the schema judges the shape of every tag as it lies in the catalog documents).
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧬️schema/🔣️.json — `$defs/Tag`
 */
import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Tag } from "../../🧬️schema/🟦️.ts";
import { STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";

const schema = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const conforms = ajv.compile({ $ref: `${schema.$id}#/$defs/Tag` });

const catalog = join(STAFFAGE_ROOT, "🗂️catalog");
const documents = readdirSync(catalog, { withFileTypes: true })
  .filter((category) => category.isDirectory())
  .flatMap((category) => readdirSync(join(catalog, category.name), { withFileTypes: true }).filter((asset) => asset.isDirectory()).map((asset) => join(catalog, category.name, asset.name, "🔣️.json")))
  .map((file) => (JSON.parse(readFileSync(file, "utf8")) as { asset: { id: string; tags: readonly unknown[] } }).asset);

describe("🏷️ tag titles", () => {
  it("defines a tag as an identifier with a bilingual title", () => {
    expect(conforms({ id: "second-hand", title: { en: "second-hand", de: "gebraucht" } })).toBe(true);
    for (const refused of ["second-hand", { id: "second-hand" }, { id: "second-hand", title: { en: "second-hand" } }, { id: "second-hand", title: { en: "second-hand", de: "" } }, { id: "Second Hand", title: { en: "a", de: "b" } }, { id: "a", title: { en: "a", de: "b" }, note: 1 }])
      expect(conforms(refused), JSON.stringify(refused)).toBe(false);
  });

  it("is met by every tag of the catalog, named once per asset", () => {
    expect(documents.length).toBeGreaterThanOrEqual(375);
    for (const asset of documents) {
      expect(asset.tags.length, asset.id).toBeGreaterThan(0);
      for (const tag of asset.tags) expect(conforms(tag), `${asset.id}: ${JSON.stringify(tag)} ${JSON.stringify(conforms.errors)}`).toBe(true);
      const ids = (asset.tags as readonly Tag[]).map((tag) => tag.id);
      expect(new Set(ids).size, `${asset.id} names a tag twice`).toBe(ids.length);
    }
  });

  it("never shows two tags of one asset with the same title", () => {
    for (const asset of documents)
      for (const language of ["en", "de"] as const) {
        const titles = (asset.tags as readonly Tag[]).map((tag) => tag.title[language].toLowerCase());
        expect(new Set(titles).size, `${asset.id} ${language}: ${titles.join(", ")}`).toBe(titles.length);
      }
  });

  it("reads the same in every asset", () => {
    const titles = new Map<string, Tag["title"]>();
    for (const asset of documents)
      for (const tag of asset.tags as readonly Tag[]) {
        const known = titles.get(tag.id);
        if (known === undefined) titles.set(tag.id, tag.title);
        else expect(tag.title, `${asset.id}: ${tag.id}`).toEqual(known);
      }
    expect(titles.size).toBeGreaterThan(1000);
    const translated = [...titles.values()].filter((title) => title.en !== title.de).length;
    expect(translated / titles.size).toBeGreaterThan(0.6);
  });
});
