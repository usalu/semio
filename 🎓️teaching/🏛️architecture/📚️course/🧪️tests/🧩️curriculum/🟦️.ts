/** 🧩️ Language-neutral curriculum vectors exercise authored concepts through the domain validator and AJV. */
import { quizIssues, sheetOf, scoreRun, type Answer, type SheetItem, type Course, type Quiz } from "@semio-tech/course";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const read = (file: string): any => JSON.parse(readFileSync(resolve(root, file), "utf8"));
const curriculum = read("🧫️fixtures/🧩️curriculum/🔣️.json") as { topics: Record<string, string>; tasks: Record<string, string[]> };
const course = read("🔣️.json") as Course;
const quizzes = course.quizzes.map(path => read(path) as Quiz);
const schema = read("../../../🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json");
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const valid = ajv.compile({ $ref: `${schema.$id}#/$defs/Quiz` });
describe("remaining architecture curriculum", () => {
  for (const [topic, id] of Object.entries(curriculum.topics)) it(`implements ${topic} as a complete bilingual quiz`, () => {
    expect(course.topics?.find(entry => entry.id === topic)?.quizzes).toContain(id);
    const quiz = quizzes.find(quiz => quiz.id === id);
    expect(quiz).toBeDefined();
    expect(quizIssues(quiz)).toEqual([]);
    expect(valid(quiz) ? [] : valid.errors).toEqual([]);
    expect(quiz!.tasks.length).toBeGreaterThanOrEqual(2);
    for (const task of quiz!.tasks) for (const item of task.items) {
      expect(item.explanation?.en, `${task.id}/${item.id}`).toBeTruthy();
      expect(item.explanation?.de, `${task.id}/${item.id}`).toBeTruthy();
    }
    if (curriculum.tasks[id]) expect(quiz!.tasks.map(task => task.id)).toEqual(curriculum.tasks[id]);
  });
  for (const area of ["🏗️bim", "📐️parametric", "📊️excel", "🦗️grasshopper", "🎯️optimization"]) it(`holds ${area} to independent authored answers`, () => {
    const fixture = read(`🧫️fixtures/${area}/🔣️.json`) as { quiz: string; classifications: Record<string, Record<string, string>> };
    const quiz = quizzes.find(quiz => quiz.id === fixture.quiz);
    expect(quiz).toBeDefined();
    for (const [id, answers] of Object.entries(fixture.classifications)) {
      const task = quiz!.tasks.find(task => task.id === id);
      expect(task?.kind).toBe("classification");
      if (task?.kind !== "classification") continue;
      for (const [item, category] of Object.entries(answers)) expect(task.items.find(entry => entry.id === item)?.category, `${id}/${item}`).toBe(category);
    }
  });
});

/** 🎲️ Every new item survives rotation and earns feedback through the same event-driven quiz core. */
describe("complete subject runs", () => {
  for (const id of Object.values(curriculum.topics)) it(`${id} is solvable at every challenge without revealing classifications`, () => {
    const quiz = quizzes.find(quiz => quiz.id === id)!;
    expect(quiz).toBeDefined();
    for (const challenge of ["easy", "medium", "hard", "expert"] as const) for (let attempt = 0; attempt < 5; attempt++) {
      const sheet = sheetOf(quiz, (0x73510000 | attempt) >>> 0, challenge);
      const answers: Record<string, Answer> = {};
      for (const shown of sheet.tasks) {
        const task = quiz.tasks.find(task => task.id === shown.id)!;
        expect(task.kind).toBe("classification");
        if (task.kind !== "classification") continue;
        answers[task.id] = { kind: "classification", assignments: Object.fromEntries(shown.items.map(item => [item.id, task.items.find(source => source.id === item.id)!.category])) };
        for (const item of shown.items) {
          expect(item).not.toHaveProperty("category");
          expect(item).not.toHaveProperty("explanation");
        }
      }
      const result = scoreRun(quiz, sheet, answers);
      expect(result?.score).toBe(1);
      expect(result?.points).toBe({ easy: 100, medium: 200, hard: 300, expert: 400 }[challenge]);
      expect(scoreRun(quiz, sheet, {})?.score).toBe(0);
    }
  });
});

/** 🕸️ Radar scales and strict Pareto boundary vectors are regression contracts as well as answer keys. */
describe("parametric profiles and Pareto semantics", () => {
  it("preserves all four illustrative radar axes and complete profiles", () => {
    const fixture = read("🧫️fixtures/📐️parametric/🔣️.json") as { axes: string[]; profiles: Record<string, Record<string, number>> };
    const task = quizzes.find(quiz => quiz.id === "parametric-design")!.tasks.find(task => task.id === "design-tool-profiles")!;
    expect(task.kind).toBe("classification");
    if (task.kind !== "classification") return;
    expect(task.axes?.map(axis => axis.id)).toEqual(fixture.axes);
    expect(task.axes?.every(axis => axis.min === 1 && axis.max === 5)).toBe(true);
    expect(Object.fromEntries(task.categories.map(category => [category.id, category.profile]))).toEqual(fixture.profiles);
  });
  it("requires no-worse in all and strictly-better in at least one minimized objective", () => {
    const fixture = read("🧫️fixtures/🎯️optimization/🔣️.json") as { pareto: { baseline: number[]; minimize: string[]; cases: { id: string; values: number[]; dominated: boolean }[] } };
    const task = quizzes.find(quiz => quiz.id === "optimization-decisions")!.tasks.find(task => task.id === "pareto-dominance")!;
    expect(task.kind).toBe("classification");
    if (task.kind !== "classification") return;
    expect(fixture.pareto.minimize).toEqual(["cost", "carbon"]);
    for (const vector of fixture.pareto.cases) {
      const dominated = fixture.pareto.baseline.every((value, index) => value <= vector.values[index]!) && fixture.pareto.baseline.some((value, index) => value < vector.values[index]!);
      expect(dominated, vector.id).toBe(vector.dominated);
      const item = task.items.find(item => item.id === vector.id)!;
      expect(item.category, vector.id).toBe(dominated ? "dominated" : "pareto");
      for (const language of ["en", "de"] as const) for (const value of vector.values) expect(item.label[language]).toContain(String(value));
    }
  });
});

/** 📊️ The formula bar belongs to the active cell and formula results never overwrite their own inputs. */
describe("Excel worksheet presentation", () => {
  const quiz = quizzes.find(quiz => quiz.id === "excel-workbook")!;
  const fixture = read("🧫️fixtures/📊️excel/🔣️.json").presentation as {
    formulaCells: { item: string; active: string; formula: string; result: number; inputs: Record<string, number> }[];
    pasteValueCells: { item: string; active: string; formula: string; result: number }[];
  };
  const address = (mockup: any): string => mockup.columns[mockup.selection.column] + (mockup.selection.row + 1);
  const cell = (mockup: any, address: string, language: "en" | "de"): string => {
    const [, column, row] = /^([A-Z]+)(\d+)$/.exec(address)!;
    return mockup.rows[Number(row) - 1][mockup.columns.indexOf(column)][language];
  };
  const references = (formula: string): string[] => [...formula.matchAll(/\$?([A-Z]+)\$?(\d+)(?::\$?([A-Z]+)\$?(\d+))?/g)].flatMap(match => {
    if (!match[3]) return [match[1]! + match[2]!];
    const output: string[] = [];
    for (let column = match[1]!.charCodeAt(0); column <= match[3]!.charCodeAt(0); column++)
      for (let row = Number(match[2]); row <= Number(match[4]); row++) output.push(String.fromCharCode(column) + row);
    return output;
  });
  it("binds every terminology formula to its independent D2 result and visible source cells", () => {
    const task = quiz.tasks.find(task => task.id === "workbook-terminology")!;
    for (const vector of fixture.formulaCells) {
      const mockup: any = task.items.find(item => item.id === vector.item)!.mockup!;
      expect(address(mockup), vector.item).toBe(vector.active);
      for (const language of ["en", "de"] as const) {
        expect(mockup.formula[language], vector.item).toBe(vector.formula);
        expect(Number(cell(mockup, vector.active, language).replace(",", ".")), vector.item).toBe(vector.result);
        for (const [source, value] of Object.entries(vector.inputs))
          expect(Number(cell(mockup, source, language).replace(",", ".")), vector.item + "/" + source).toBe(value);
      }
      expect(references(vector.formula), vector.item).not.toContain(vector.active);
    }
  });
  it("associates all 110 formula bars with their actual selected cells", () => {
    expect(quiz.tasks.flatMap<SheetItem>(task => task.items)).toHaveLength(110);
    for (const task of quiz.tasks) for (const item of task.items) {
      const mockup: any = item.mockup!;
      for (const language of ["en", "de"] as const) {
        if (task.id === "copying-references") {
          expect(mockup.formula[language], item.id).toBe(cell(mockup, address(mockup), language));
          expect(mockup.caption[language], item.id).toContain(language === "en" ? "Formula display mode" : "Formelanzeigemodus");
        } else if (!["formula", "reference", "values"].includes((item as any).category)) {
          expect(mockup.formula[language], item.id).toBe(cell(mockup, address(mockup), language));
        }
      }
    }
    for (const vector of fixture.pasteValueCells) {
      const mockup: any = quiz.tasks.flatMap<SheetItem>(task => task.items).find(item => item.id === vector.item)!.mockup!;
      expect(address(mockup), vector.item).toBe(vector.active);
      for (const language of ["en", "de"] as const) {
        expect(mockup.formula[language], vector.item).toBe(vector.formula);
        expect(Number(cell(mockup, vector.active, language)), vector.item).toBe(vector.result);
      }
      expect(references(vector.formula), vector.item).not.toContain(vector.active);
    }
  });
});
