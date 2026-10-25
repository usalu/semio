/** 🧪️ The site's course loads with every quiz it names, the TS core reports no issue for any of them, and the draft-07
 * contract agrees through a third-party validator (ajv), so the owned validation and the schema cannot drift apart
 * unnoticed on this content. Content rules hold what the core cannot ask of any quiz: icons that tell items apart, badges
 * for perfection from medium on, classifications with something hard hides (category descriptions or axes), short forms
 * that keep hint sentences readable, item names that count as nouns ("1,000 × “Litre of heating oil”"), and familiar
 * items the hints can measure the others against.
 * @see ../../🔣️.json — the course under test
 * @see ../../../../../🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json — the contract ajv validates against */
import { MOTIONS, courseIssues, courseView, courseViewIssues, challengeMeets, quizIssues, sheetOf, type Course, type Quiz } from "@semio-tech/course";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import * as material from "../../📚️course/🟦️.ts";
import courseContract from "../../🧫️fixtures/📚️course/🔣️.json";
import { isCourseView } from "../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const coursePath = resolve(siteRoot, "🔣️.json");
const schemaPath = resolve(siteRoot, "../../../🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json");
const read = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));
const course = read(coursePath) as Course;
const quizzes = course.quizzes.map((path) => ({ path, quiz: read(resolve(dirname(coursePath), path)) as Quiz }));
const schema = read(schemaPath) as { readonly $id: string };
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const validQuiz = ajv.compile({ $ref: `${schema.$id}#/$defs/Quiz` });
const validCourse = ajv.compile({ $ref: `${schema.$id}#/$defs/Course` });
const validView = ajv.compile({ $ref: `${schema.$id}#/$defs/CourseView` });
const topicFixture = read(resolve(siteRoot, "🧫️fixtures/🗂️topics/🔣️.json")) as { topics: readonly unknown[]; reserved: readonly string[]; rejected: readonly { id: string; topic: number; property: string; value: unknown; path: string; code: string }[]; viewRejected: readonly { id: string; path: readonly string[]; value: unknown }[] };
const SHORT = 40;
const LANGUAGES = ["en", "de"] as const;
const PLAIN = /^[^\d()][^()]*$/u;
const COUNTABLE = { en: /^(?!(?:The|A|An|One) )(?!\p{L}+ing (?:(?:an?|the)\b|\d))[^,']*$/u, de: /^(?!(?:Der|Die|Das|Ein|Eine) )(?!\p{Lu}\p{Ll}+en (?:eines|einer|des|der) )[^,']*$/u } as const;
const length = (text: string): number => [...text].length;

describe("architecture course", () => {
  it("shares valid proctor fixtures with the Rust unit and HTTP suites", () => {
    const proctor = resolve(siteRoot, "../../🛂️proctor");
    const source = readFileSync(resolve(proctor, "🔨️modules/📚️course/🧰️fixtures/🦀️.rs"), "utf8");
    const relative = source.match(/fixture_path!\("([^"]+)"\)/u)?.[1];
    expect(relative).toBeDefined();
    const path = resolve(proctor, "🔨️modules/🧱️base/📦️packages/🦀️rust", relative!);
    const fixture = read(path) as Course;
    const documents = fixture.quizzes.map(file => read(resolve(dirname(path), file)) as Quiz);
    expect(courseIssues(fixture, documents)).toEqual([]);
    expect(validCourse(fixture) ? [] : validCourse.errors).toEqual([]);
    for (const quiz of documents) {
      expect(quizIssues(quiz)).toEqual([]);
      expect(validQuiz(quiz) ? [] : validQuiz.errors).toEqual([]);
    }
    for (const [document, file] of [[fixture, path], ...documents.map((quiz, index) => [quiz, resolve(dirname(path), fixture.quizzes[index]!)] as const)] as const) {
      expect(resolve(dirname(file), document.$schema!.split("#")[0]!)).toBe(schemaPath);
    }
    const harness = readFileSync(resolve(proctor, "🧪️tests/🌐️end-to-end/🧰️harness/🦀️.rs"), "utf8");
    const directory = harness.match(/fixture_path!\("([^"]+)"\)/u)?.[1];
    expect(directory).toBeDefined();
    expect(resolve(proctor, "📦️packages/🦀️rust", directory!, "📚️catalog/🔣️.json")).toBe(path);
  });

  it("owns the course identity while preserving its quiz components", () => {
    expect(course.schema).toBe(courseContract.schema);
    expect(course.id).toBe(courseContract.id);
    expect(course.title).toEqual(courseContract.title);
    expect(course.short).toEqual(courseContract.short);
    expect(quizzes.map(({ quiz }) => quiz.id)).toEqual(courseContract.quizzes);
    expect(validCourse(course) ? [] : validCourse.errors).toEqual([]);
  });
  it("declares Energy and five placeholders in the same order in authored material and the read model", () => {
    expect(course.topics).toEqual(topicFixture.topics);
    expect(courseView(course, quizzes.map(({ quiz }) => quiz)).topics).toEqual(topicFixture.topics);
    expect(validCourse(course) ? [] : validCourse.errors).toEqual([]);
  });

  for (const vector of topicFixture.rejected) it(`rejects topic ownership: ${vector.id}`, () => {
    const document = structuredClone(course) as unknown as { topics: Record<string, unknown>[] };
    document.topics[vector.topic]![vector.property] = vector.value;
    expect(courseIssues(document, quizzes.map(({ quiz }) => quiz))).toContainEqual({ path: vector.path, code: vector.code });
    const view = structuredClone(courseView(course, quizzes.map(({ quiz }) => quiz))) as unknown as { topics: Record<string, unknown>[] };
    view.topics[vector.topic]![vector.property] = vector.value;
    expect(isCourseView(view)).toBe(false);
  });

  for (const reserved of topicFixture.reserved) it(`reserves ${reserved} for root navigation`, () => {
    const document = structuredClone(course) as unknown as { topics: { id: string }[] };
    document.topics[1]!.id = reserved;
    expect(courseIssues(document, quizzes.map(({ quiz }) => quiz))).toContainEqual({ path: "/topics/1/id", code: "value-invalid" });
    expect(courseIssues(course, quizzes.map(({ quiz }, index) => index === 0 ? { ...quiz, id: reserved } : quiz))).toContainEqual({ path: "/quizzes/0", code: "value-invalid" });
    const view = courseView(course, quizzes.map(({ quiz }) => quiz));
    expect(isCourseView({ ...view, topics: view.topics!.map((topic, index) => index === 1 ? { ...topic, id: reserved } : topic) })).toBe(false);
  });

  it("accepts cached grouped views with empty placeholders and flat views", () => {
    const view = courseView(course, quizzes.map(({ quiz }) => quiz));
    expect(isCourseView(view)).toBe(true);
    expect(courseViewIssues(view)).toEqual([]);
    expect(validView(view) ? [] : validView.errors).toEqual([]);
    const { topics, ...flat } = view;
    expect(isCourseView(flat)).toBe(true);
    expect(isCourseView({ ...view, quizzes: [...view.quizzes, view.quizzes[0]] })).toBe(false);
  });

  for (const vector of topicFixture.viewRejected) it(`rejects malformed cached course views: ${vector.id}`, () => {
    const view = structuredClone(courseView(course, quizzes.map(({ quiz }) => quiz))) as unknown as Record<string, any>;
    const holder = vector.path.slice(0, -1).reduce((current, segment) => current[segment], view);
    holder[vector.path.at(-1)!] = vector.value;
    expect(validView(view)).toBe(false);
    expect(isCourseView(view)).toBe(false);
  });

  it("is the architecture course and names at least one quiz", () => {
    expect(course.id).toBe("architecture");
    expect(quizzes.length).toBeGreaterThan(0);
  });

  it("has no issue in the TS core", () => {
    expect(courseIssues(course, quizzes.map(({ quiz }) => quiz))).toEqual([]);
  });

  it("labels the physics question as power or work in German", () => {
    const physics = quizzes.find(({ quiz }) => quiz.id === "physics")?.quiz;
    const task = physics?.tasks.find((candidate) => candidate.id === "power-or-energy");
    expect(task?.title.de).toBe("Leistung oder Arbeit?");
    expect(task?.prompt.de).toContain("oder Arbeit");
    expect(task?.kind === "classification" ? task.categories.find(({ id }) => id === "energy")?.label.de : undefined).toBe("Arbeit");
  });

  it("is accepted by the draft-07 contract (ajv)", () => {
    expect(validCourse(course) ? [] : validCourse.errors).toEqual([]);
  });

  it("asks every badge for perfection at medium or harder, so the hints of easy earn none", () => {
    const perfect = course.badges.filter(({ rule }) => rule.kind !== "completed-quizzes");
    expect(perfect.length).toBeGreaterThan(0);
    expect(perfect.filter(({ rule }) => rule.kind === "completed-quizzes" || rule.challenge === undefined || !challengeMeets(rule.challenge, "medium")).map(({ id }) => id)).toEqual([]);
  });

  it("is what the site ships as its material: the module's lazy imports equal the documents on disk, in the course's order, and it exports nothing else", async () => {
    expect(await material.loadArchitectureCourseMaterial()).toEqual({ course, quizzes: quizzes.map(({ quiz }) => quiz) });
    expect(Object.keys(material)).toEqual(["loadArchitectureCourseMaterial"]);
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

/** 🧺️ Real material keeps its authored topic distribution across five fresh attempts. */
describe("balanced question pools", () => {
  for (const { quiz } of quizzes) it(quiz.id, () => {
    for (const challenge of ["easy", "medium", "hard", "expert"] as const) {
      const seen = new Map<string, Set<string>>();
      for (let attempt = 0; attempt < 5; attempt++) {
        const sheet = sheetOf(quiz, (0x73510000 | attempt) >>> 0, challenge);
        for (const task of sheet.tasks) {
          const source = quiz.tasks.find(({ id }) => id === task.id)!;
          expect(source.pool).toBeDefined();
          expect(task.items).toHaveLength(source.draw!);
          expect(task.seconds).toBe(challenge === "expert" ? 300 : undefined);
          const previous = seen.get(task.id) ?? new Set<string>();
          for (const item of task.items) {
            expect(previous.has(item.id), item.id).toBe(false);
            previous.add(item.id);
          }
          seen.set(task.id, previous);
          for (const slot of source.pool!.slots) expect(task.items.filter(({ id }) => slot.includes(id))).toHaveLength(1);
        }
      }
    }
  });
});

/** 🌬️ The language-neutral ventilation vectors define ownership, statements, preserved rates and spider profiles. */
describe("standalone ventilation quiz", () => {
  const fixture = read(resolve(siteRoot, "🧫️fixtures/🌬️ventilation/🔣️.json")) as { order: string[]; tasks: { id: string; kind: string }[]; statements: Record<string, string>; rates: Record<string, number>; axes: string[]; profiles: Record<string, Record<string, number>>; systems: Record<string, string> };
  const ventilation = quizzes.find(({ quiz }) => quiz.id === "ventilation")?.quiz;

  it("places ventilation between physics and heating in the authored course and the lazy material", async () => {
    expect(quizzes.map(({ quiz }) => quiz.id)).toEqual(fixture.order);
    expect(course.topics![0]!.quizzes).toEqual(fixture.order);
    expect((await material.loadArchitectureCourseMaterial()).quizzes.map(({ id }) => id)).toEqual(fixture.order);
    expect(course.badges.find(({ id }) => id === "ventilation-expert")?.rule).toEqual({ kind: "perfect-quiz", quiz: "ventilation", challenge: "medium" });
  });

  it("owns the three requested task types and removes the air-change task from cooling", () => {
    expect(ventilation?.tasks.map(({ id, kind }) => ({ id, kind }))).toEqual(fixture.tasks);
    expect(quizzes.find(({ quiz }) => quiz.id === "cooling")?.quiz.tasks.map(({ id }) => id)).toEqual(["cooling-load-and-demand"]);
  });

  it("classifies the core ventilation claims with bilingual explanations", () => {
    const statements = ventilation?.tasks.find(({ id }) => id === "ventilation-statements");
    expect(statements?.kind).toBe("classification");
    if (statements?.kind !== "classification") return;
    expect(statements.categories.map(({ id }) => id)).toEqual(["true", "false"]);
    for (const [id, category] of Object.entries(fixture.statements)) {
      const item = statements.items.find((candidate) => candidate.id === id);
      expect(item?.category, id).toBe(category);
      for (const language of LANGUAGES) expect(item?.explanation?.[language], id).toBeTruthy();
    }
  });

  it("preserves every air-change example and value while changing the exercise to sorting", () => {
    const rates = ventilation?.tasks.find(({ id }) => id === "air-change-rates");
    expect(rates?.kind).toBe("sorting");
    if (rates?.kind !== "sorting") return;
    expect(rates.quantity.unit).toBe("1/h");
    expect(rates.quantity.scale).toBe("logarithmic");
    expect(Object.fromEntries(rates.items.map(({ id, value }) => [id, value]))).toEqual(fixture.rates);
  });

  it("assigns ventilation types to five-axis spider diagrams with consistent relative scales", () => {
    const profiles = ventilation?.tasks.find(({ id }) => id === "ventilation-profiles");
    expect(profiles?.kind).toBe("classification");
    if (profiles?.kind !== "classification") return;
    expect(profiles.axes?.map(({ id }) => id)).toEqual(fixture.axes);
    expect(profiles.axes?.every(({ min, max }) => min === 1 && max === 5)).toBe(true);
    expect(Object.fromEntries(profiles.categories.map(({ id, profile }) => [id, profile]))).toEqual(fixture.profiles);
    for (const [id, category] of Object.entries(fixture.systems)) expect(profiles.items.find((item) => item.id === id)?.category, id).toBe(category);
  });

  it("accepts ventilation with the independent AJV schema oracle and the domain validator", () => {
    expect(ventilation).toBeDefined();
    expect(validQuiz(ventilation) ? [] : validQuiz.errors).toEqual([]);
    expect(quizIssues(ventilation)).toEqual([]);
  });
});
