import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import contract from "../../🧬️schema/🔣️.json" with { type: "json" };
import cases from "../../🧫️fixtures/📝️assignments/🔣️.json" with { type: "json" };
import { courseIssues, courseViewIssues } from "../../🔨️modules/✅️validation/🟦️.ts";
import { courseView } from "../../🔨️modules/👁️views/🟦️.ts";
import type { Course, Quiz } from "../../🧬️schema/🟦️.ts";

const validate = new Ajv({ strict: false }).compile({ $ref: "#/$defs/Assignment", $defs: contract.$defs });
const quiz = { schema: "semio.quiz/v1", id: "sample", emoji: "📐", title: { en: "Sample", de: "Beispiel" }, description: { en: "Sample", de: "Beispiel" }, tasks: [] } as unknown as Quiz;
const course = (assignment: unknown) => ({ schema: "semio.course/v1", id: "sample-course", title: quiz.title, introduction: { title: quiz.title, paragraphs: [quiz.title] }, quizzes: ["sample.json"], topics: [{ id: "sample-topic", emoji: "📐", title: quiz.title, quizzes: ["sample"] }], badges: [], assignments: [assignment] });
describe("localized downloadable assignments", () => {
  for (const assignment of cases.valid) it(`accepts and projects ${assignment.id}`, () => {
    expect(validate(assignment)).toBe(true);
    expect(courseIssues(course(assignment), [quiz])).toEqual([]);
    const view = courseView(course(assignment) as Course, [quiz]);
    expect(view.assignments).toEqual([assignment]);
    expect(courseViewIssues(view)).toEqual([]);
  });
  for (const sample of cases.invalid) it(sample.case, () => {
    const assignment = structuredClone(cases.valid[0]) as Record<string, unknown>;
    const pointer = ("remove" in sample ? sample.remove : sample.replace)!.split("/").slice(1);
    const key = pointer.pop()!;
    let parent = assignment;
    for (const segment of pointer) parent = parent[segment] as Record<string, unknown>;
    if ("remove" in sample) delete parent[key];
    else parent[key] = sample.value;
    expect(validate(assignment)).toBe(false);
    expect(courseIssues(course(assignment), [quiz]).length).toBeGreaterThan(0);
  });
  it("rejects unknown topics and duplicate assignments", () => {
    const unknown = course({ ...cases.valid[0], topic: "unknown" });
    expect(courseIssues(unknown, [quiz]).length).toBeGreaterThan(0);
    const duplicate = { ...course(cases.valid[0]), assignments: [cases.valid[0], cases.valid[0]] };
    expect(courseIssues(duplicate, [quiz]).length).toBeGreaterThan(0);
  });
});
