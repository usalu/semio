/** 🧪️ The site's catalog loads with every quiz it names, the TS core reports no issue for any of them, and the draft-07
 * contract agrees through a third-party validator (ajv), so the owned validation and the schema cannot drift apart
 * unnoticed on this content. Content rules hold what the core cannot ask of any quiz: icons that tell items apart, badges
 * for perfection from medium on, classifications with something hard hides (category descriptions or axes), short forms
 * that keep hint sentences readable, item names that count as nouns ("1,000 × “Litre of heating oil”"), and familiar
 * items the hints can measure the others against.
 * @see ../../🔣️.json — the catalog under test
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json — the contract ajv validates against */
import { MOTIONS, catalogIssues, challengeMeets, quizIssues, sheetOf, type Catalog, type Quiz } from "@semio-tech/quiz";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import * as material from "../../📚️catalog/🟦️.ts";

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
const SHORT = 40;
const LANGUAGES = ["en", "de"] as const;
const PLAIN = /^[^\d()][^()]*$/u;
const COUNTABLE = { en: /^(?!(?:The|A|An|One) )(?!\p{L}+ing (?:(?:an?|the)\b|\d))[^,']*$/u, de: /^(?!(?:Der|Die|Das|Ein|Eine) )(?!\p{Lu}\p{Ll}+en (?:eines|einer|des|der) )[^,']*$/u } as const;
const length = (text: string): number => [...text].length;

describe("architecture quiz catalog", () => {
  it("is the architecture catalog and names at least one quiz", () => {
    expect(catalog.id).toBe("architecture");
    expect(quizzes.length).toBeGreaterThan(0);
  });

  it("has no issue in the TS core", () => {
    expect(catalogIssues(catalog, quizzes.map(({ quiz }) => quiz))).toEqual([]);
  });

  it("labels the physics question as power or work in German", () => {
    const physics = quizzes.find(({ quiz }) => quiz.id === "physics")?.quiz;
    const task = physics?.tasks.find((candidate) => candidate.id === "power-or-energy");
    expect(task?.title.de).toBe("Leistung oder Arbeit?");
    expect(task?.prompt.de).toContain("oder Arbeit");
    expect(task?.kind === "classification" ? task.categories.find(({ id }) => id === "energy")?.label.de : undefined).toBe("Arbeit");
  });

  it("is accepted by the draft-07 contract (ajv)", () => {
    expect(validCatalog(catalog) ? [] : validCatalog.errors).toEqual([]);
  });

  it("asks every badge for perfection at medium or harder, so the hints of easy earn none", () => {
    const perfect = catalog.badges.filter(({ rule }) => rule.kind !== "completed-quizzes");
    expect(perfect.length).toBeGreaterThan(0);
    expect(perfect.filter(({ rule }) => rule.kind === "completed-quizzes" || rule.challenge === undefined || !challengeMeets(rule.challenge, "medium")).map(({ id }) => id)).toEqual([]);
  });

  it("is what the site ships as its material: the module's static imports equal the documents on disk, in the catalog's order, and it exports nothing else", () => {
    expect(material.ARCHITECTURE_QUIZ_MATERIAL).toEqual({ catalog, quizzes: quizzes.map(({ quiz }) => quiz) });
    expect(Object.keys(material)).toEqual(["ARCHITECTURE_QUIZ_MATERIAL"]);
  });

  for (const { path, quiz } of quizzes) {
    it(`${path} has no issue in the TS core`, () => {
      expect(quizIssues(quiz)).toEqual([]);
    });

    it(`${path} gives every task an icon with a distinct emoji`, () => {
      const icons = quiz.tasks.map((task) => task.icon);
      expect(icons.every((icon) => icon !== undefined)).toBe(true);
      expect(new Set(icons.map((icon) => icon!.emoji)).size).toBe(icons.length);
      for (const icon of icons) expect(MOTIONS).toContain(icon!.motion);
    });

    it(`${path} gives every item, category and dimension an icon, and no two items of a task look and move alike`, () => {
      for (const task of quiz.tasks) {
        const parts = [...task.items, ...(task.kind === "classification" ? task.categories : []), ...(task.kind === "matching" ? task.dimensions : [])];
        expect(parts.filter((part) => part.icon === undefined).map((part) => `${task.id}/${part.id}`)).toEqual([]);
        for (const part of parts) expect(MOTIONS).toContain(part.icon!.motion);
        const looks = task.items.map((item) => `${item.icon!.emoji} ${item.icon!.motion}`);
        expect(looks.filter((look, index) => looks.indexOf(look) !== index)).toEqual([]);
      }
    });

    it(`${path} gives every classification category descriptions or axes, so hiding them makes hard differ from medium`, () => {
      const indistinct = quiz.tasks.filter((task) => task.kind === "classification" && (task.axes ?? []).length === 0 && task.categories.every((category) => category.description === undefined));
      expect(indistinct.map((task) => task.id)).toEqual([]);
      for (const seed of [1, 2, 3]) {
        const [medium, hard] = (["medium", "hard"] as const).map((challenge) => sheetOf(quiz, seed, challenge).tasks.filter((task) => task.kind === "classification"));
        for (const [index, shown] of medium!.entries()) expect(hard![index], `${shown.id} at seed ${seed}`).not.toEqual(shown);
      }
    });

    it(`${path} gives every label longer than ${SHORT} characters a short form, keeps every short form brief, plain and distinct in its task`, () => {
      for (const task of quiz.tasks) {
        const parts = [...task.items, ...(task.kind === "classification" ? [...task.categories, ...(task.axes ?? [])] : []), ...(task.kind === "matching" ? task.dimensions.map(({ id, quantity }) => ({ id, ...quantity })) : []), ...(task.kind === "sorting" ? [{ id: "quantity", ...task.quantity }] : [])];
        expect(parts.filter((part) => part.short === undefined && LANGUAGES.some((language) => length(part.label[language]) > SHORT)).map(({ id }) => `${task.id}/${id}`)).toEqual([]);
        for (const part of parts.filter((candidate) => candidate.short !== undefined)) {
          for (const language of LANGUAGES) expect(part.short![language], `${task.id}/${part.id}`).toMatch(PLAIN);
          for (const language of LANGUAGES) expect(length(part.short![language]), `${task.id}/${part.id}`).toBeLessThanOrEqual(SHORT);
        }
        for (const language of LANGUAGES) {
          const names = task.items.map((item) => (item.short ?? item.label)[language]);
          expect(names.filter((name, index) => names.indexOf(name) !== index), `${task.id} ${language}`).toEqual([]);
        }
      }
    });

    it(`${path} names every item in hints by a noun phrase that reads after "N ×": no leading article, no verb with an object, no comma, no straight apostrophe`, () => {
      for (const task of quiz.tasks) for (const item of task.items) for (const language of LANGUAGES) expect((item.short ?? item.label)[language], `${task.id}/${item.id}`).toMatch(COUNTABLE[language]);
    });

    it(`${path} marks at least one familiar item in every sorting and matching`, () => {
      const unfamiliar = quiz.tasks.filter((task) => task.kind !== "classification" && !task.items.some((item) => item.familiar === true));
      expect(unfamiliar.map(({ id }) => id)).toEqual([]);
    });

    it(`${path} is accepted by the draft-07 contract (ajv)`, () => {
      expect(validQuiz(quiz) ? [] : validQuiz.errors).toEqual([]);
    });
  }
});
