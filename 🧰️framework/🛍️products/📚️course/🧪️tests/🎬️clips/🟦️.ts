import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import contract from "../../🧬️schema/🔣️.json" with { type: "json" };
import cases from "../../🧫️fixtures/🎬️clips/🔣️.json" with { type: "json" };
import { courseIssues, courseViewIssues } from "../../🔨️modules/✅️validation/🟦️.ts";
import { courseView } from "../../🔨️modules/👁️views/🟦️.ts";
import type { Course, Quiz } from "../../🧬️schema/🟦️.ts";

const validate = new Ajv({ strict: false }).compile({ $ref: "#/$defs/Clip", $defs: contract.$defs });
const validateCourse = new Ajv({ strict: false }).compile({ $ref: "#/$defs/Course", $defs: contract.$defs });
const quiz = { schema: "semio.quiz/v1", id: "sample", emoji: "📐", title: { en: "Sample", de: "Beispiel" }, description: { en: "Sample", de: "Beispiel" }, tasks: [] } as unknown as Quiz;
const course = (clips: unknown[], extra: Record<string, unknown> = {}) => ({ schema: "semio.course/v1", id: "sample-course", title: quiz.title, introduction: { title: quiz.title, paragraphs: [quiz.title] }, quizzes: ["sample.json"], topics: [{ id: "sample-topic", emoji: "📐", title: quiz.title, quizzes: ["sample"] }], badges: [], clips, ...extra });

/** 🎬️ The valid sample clips with one JSON pointer replaced or removed. */
function mutated(sample: { readonly remove?: string; readonly replace?: string; readonly value?: unknown }): Record<string, unknown> {
  const clip = structuredClone(cases.valid[1]) as Record<string, unknown>;
  const pointer = (sample.remove ?? sample.replace)!.split("/").slice(1);
  const key = pointer.pop()!;
  let parent = clip;
  for (const segment of pointer) parent = parent[segment] as Record<string, unknown>;
  if (sample.remove !== undefined) delete parent[key];
  else parent[key] = sample.value;
  return clip;
}

describe("learning clips with posters and later hosting", () => {
  for (const clip of cases.valid) it(`accepts and projects ${clip.id}`, () => {
    expect(validate(clip)).toBe(true);
    expect(courseIssues(course([clip]), [quiz])).toEqual([]);
    expect(validateCourse(course([clip]))).toBe(true);
    const view = courseView(course([clip]) as unknown as Course, [quiz]);
    expect(view.clips).toEqual([clip]);
    expect(courseViewIssues(view)).toEqual([]);
  });
  it("keeps a clip without an address free of one", () => {
    expect(Object.hasOwn(cases.valid[0]!, "urls")).toBe(false);
    expect(courseView(course([cases.valid[0]]) as unknown as Course, [quiz]).clips).toEqual([cases.valid[0]]);
  });
  for (const sample of cases.invalid) it(sample.case, () => {
    const clip = mutated(sample);
    expect(validate(clip)).toBe(false);
    expect(courseIssues(course([clip]), [quiz]).length).toBeGreaterThan(0);
    expect(courseViewIssues(courseView(course([clip]) as unknown as Course, [quiz])).length).toBeGreaterThan(0);
  });
  it("rejects unknown topics and duplicate clips", () => {
    expect(courseIssues(course([{ ...cases.valid[0], topic: "unknown" }]), [quiz]).length).toBeGreaterThan(0);
    expect(courseIssues(course([cases.valid[0], cases.valid[0]]), [quiz]).length).toBeGreaterThan(0);
  });
  it("gives every page one address across quizzes, topics, root pages, assignments and clips", () => {
    for (const id of ["sample", "sample-topic", "clips", "assignments", "quizzes", "board"]) expect(courseIssues(course([{ ...cases.valid[0], id }]), [quiz]).length, id).toBeGreaterThan(0);
    const assignment = { id: "sample-clip", topic: "sample-topic", emoji: "📝", title: quiz.title, description: quiz.title, documents: { en: { namespace: "course", id: "a-en", file: "a-en.pdf", producer: "@teaching/course:build-a-en" }, de: { namespace: "course", id: "a-de", file: "a-de.pdf", producer: "@teaching/course:build-a-de" } } };
    expect(courseIssues(course([cases.valid[0]], { assignments: [assignment] }), [quiz]).length).toBeGreaterThan(0);
    expect(courseIssues(course([cases.valid[0]], { assignments: [{ ...assignment, id: "sample-assignment" }] }), [quiz])).toEqual([]);
  });
});
