/** 🧪️ The site's catalog loads with every quiz it names, the TS core reports no issue for any of them, and the draft-07
 * contract agrees through a third-party validator (ajv), so the owned validation and the schema cannot drift apart
 * unnoticed on this content.
 * @see ../../🔣️.json — the catalog under test
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json — the contract ajv validates against */
import { catalogIssues, quizIssues, type Catalog, type Quiz } from "@semio-tech/quiz";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const catalogPath = resolve(siteRoot, "🔣️.json");
const schemaPath = resolve(siteRoot, "../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json");
const read = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));
const catalog = read(catalogPath) as Catalog;
const quizzes = catalog.quizzes.map((path) => ({ path, quiz: read(resolve(dirname(catalogPath), path)) as Quiz }));
const schema = read(schemaPath) as { readonly $id: string };
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const validQuiz = ajv.getSchema(schema.$id)!;
const validCatalog = ajv.compile({ $ref: `${schema.$id}#/$defs/Catalog` });

describe("architecture quiz catalog", () => {
  it("is the architecture catalog and names at least one quiz", () => {
    expect(catalog.id).toBe("architecture");
    expect(quizzes.length).toBeGreaterThan(0);
  });

  it("has no issue in the TS core", () => {
    expect(catalogIssues(catalog, quizzes.map(({ quiz }) => quiz))).toEqual([]);
  });

  it("is accepted by the draft-07 contract (ajv)", () => {
    expect(validCatalog(catalog) ? [] : validCatalog.errors).toEqual([]);
  });

  for (const { path, quiz } of quizzes) {
    it(`${path} has no issue in the TS core`, () => {
      expect(quizIssues(quiz)).toEqual([]);
    });

    it(`${path} is accepted by the draft-07 contract (ajv)`, () => {
      expect(validQuiz(quiz) ? [] : validQuiz.errors).toEqual([]);
    });
  }
});
