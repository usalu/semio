import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  cursorIssues,
  cursorProblem,
  isAnchor,
  isTag,
  learnerTag,
  presenceIssues,
  presenceProblem,
  presenceRoster,
  roomScope,
  rosterScope,
  type Place,
  type PresenceEntry,
  type PresenceState,
  type ValidationIssue,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

type Mutable = Record<string, any>;

const SCHEMA = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧬️schema/🔣️.json"), "utf8")) as { readonly $id: string };
const AJV = new Ajv({ strict: false, allErrors: true });
AJV.addSchema(SCHEMA);
const AJV_PRESENCE = AJV.getSchema(`${SCHEMA.$id}#/$defs/PresenceState`)!;
const AJV_CURSOR = AJV.getSchema(`${SCHEMA.$id}#/$defs/CursorState`)!;
const TAG = learnerTag("0123456789abcdef0123456789abcdef");

/** 🧭️ Whether an issue states a place rule the schema cannot express. */
function beyondSchema(issue: ValidationIssue): boolean {
  return issue.code === "quiz-outside-run" || issue.code === "task-without-run" || (issue.path === "/place/quiz" && issue.code === "required");
}

/** 🟢️ The presence issues, checked against ajv: schema-valid exactly when every issue is a place rule beyond the schema. */
function checkedPresence(state: unknown): ValidationIssue[] {
  const issues = presenceIssues(state);
  expect(AJV_PRESENCE(state), JSON.stringify(issues)).toBe(issues.every(beyondSchema));
  expect(presenceProblem(state)).toEqual(issues[0]);
  return issues;
}

/** 👆️ The cursor issues, checked against ajv: schema-valid exactly when there is no issue. */
function checkedCursor(state: unknown): ValidationIssue[] {
  const issues = cursorIssues(state);
  expect(AJV_CURSOR(state), JSON.stringify(issues)).toBe(issues.length === 0);
  expect(cursorProblem(state)).toEqual(issues[0]);
  return issues;
}

/** 🟢️ A valid presence state of a pseudonymous learner on a run task. */
function presence(): Mutable {
  return { tag: TAG, identity: { kind: "pseudonym", handle: "Ada" }, place: { screen: "run", quiz: "physics", task: "units" }, active: true };
}

/** 👆️ A valid cursor state with pointer and focus. */
function cursor(): Mutable {
  return { tag: TAG, cursor: { anchor: "task:units", x: 0.25, y: 1 }, focus: "card:leaderboard" };
}

/** 🦠️ A state after `mutate`. */
function mutated(state: Mutable, mutate: (state: Mutable) => void): Mutable {
  mutate(state);
  return state;
}

describe("scopes", () => {
  it("names the roster room by the catalog and one room per place", () => {
    expect(rosterScope("architecture")).toBe("architecture");
    const rooms: readonly [Place, string | undefined][] = [
      [{ screen: "introduction" }, "architecture/introduction"],
      [{ screen: "identity" }, undefined],
      [{ screen: "home" }, "architecture/home"],
      [{ screen: "leaderboard" }, "architecture/leaderboard"],
      [{ screen: "run", quiz: "physics" }, "architecture/quiz/physics"],
      [{ screen: "run", quiz: "physics", task: "units" }, "architecture/quiz/physics"],
      [{ screen: "results", quiz: "heating" }, "architecture/quiz/heating"],
      [{ screen: "run" }, undefined],
      [{ screen: "results" }, undefined],
      [{ screen: "quiz", quiz: "cooling" }, "architecture/quiz/cooling"],
      [{ screen: "quiz" }, undefined],
      [{ screen: "badges" }, "architecture/badges"],
      [{ screen: "learner" }, undefined],
      [{ screen: "preferences" }, undefined],
    ];
    for (const [place, room] of rooms) expect(roomScope("architecture", place), JSON.stringify(place)).toBe(room);
  });
});

describe("presenceProblem", () => {
  it("admits valid states on every screen", () => {
    const places: readonly Place[] = [
      { screen: "introduction" },
      { screen: "identity" },
      { screen: "home" },
      { screen: "quiz", quiz: "cooling" },
      { screen: "leaderboard" },
      { screen: "run", quiz: "physics" },
      { screen: "run", quiz: "physics", task: "units" },
      { screen: "results", quiz: "heating" },
      { screen: "learner" },
      { screen: "badges" },
      { screen: "preferences" },
    ];
    for (const place of places) expect(checkedPresence({ ...presence(), place }), JSON.stringify(place)).toEqual([]);
    expect(checkedPresence({ ...presence(), identity: { kind: "anonymous" }, active: false })).toEqual([]);
    expect(checkedPresence({ ...presence(), identity: { kind: "name", handle: "Zoë O'Neill-Müller".padEnd(64, "x") } })).toEqual([]);
  });

  const cases: readonly [string, (state: Mutable) => void, readonly ValidationIssue[]][] = [
    ["a non-object", () => undefined, []],
    ["an uppercase tag", (state) => (state.tag = "ABCDEF12"), [{ path: "/tag", code: "tag-invalid" }]],
    ["a learner id as tag", (state) => (state.tag = "8d3ab6ac9d4fd9067ae6a4deac46190d"), [{ path: "/tag", code: "tag-invalid" }]],
    ["a numeric tag", (state) => (state.tag = 12345678), [{ path: "/tag", code: "type-invalid" }]],
    ["a smuggled learner id", (state) => (state.learner = "8d3ab6ac9d4fd9067ae6a4deac46190d"), [{ path: "/learner", code: "property-unknown" }]],
    ["a missing active flag", (state) => delete state.active, [{ path: "/active", code: "required" }]],
    ["a textual active flag", (state) => (state.active = "yes"), [{ path: "/active", code: "type-invalid" }]],
    ["an unknown identity kind", (state) => (state.identity.kind = "alias"), [{ path: "/identity/kind", code: "value-invalid" }]],
    ["a pseudonym without handle", (state) => delete state.identity.handle, [{ path: "/identity/handle", code: "required" }]],
    ["an anonymous identity with a handle", (state) => (state.identity = { kind: "anonymous", handle: "Ada" }), [{ path: "/identity/handle", code: "property-unknown" }]],
    ["an empty handle", (state) => (state.identity.handle = ""), [{ path: "/identity/handle", code: "handle-invalid" }]],
    ["a handle of 65 code points", (state) => (state.identity.handle = "🎲".repeat(65)), [{ path: "/identity/handle", code: "handle-invalid" }]],
    ["an unknown screen", (state) => (state.place = { screen: "settings" }), [{ path: "/place/screen", code: "value-invalid" }]],
    ["a place without screen", (state) => (state.place = { quiz: "physics" }), [{ path: "/place/screen", code: "required" }]],
    ["a smuggled answer", (state) => (state.place.answer = "joule"), [{ path: "/place/answer", code: "property-unknown" }]],
    ["a quiz that is no slug", (state) => (state.place.quiz = "Physics Basics"), [{ path: "/place/quiz", code: "slug-invalid" }]],
    ["a task that is no slug", (state) => (state.place.task = "Units"), [{ path: "/place/task", code: "slug-invalid" }]],
    ["a run without quiz", (state) => (state.place = { screen: "run" }), [{ path: "/place/quiz", code: "required" }]],
    ["results without quiz", (state) => (state.place = { screen: "results" }), [{ path: "/place/quiz", code: "required" }]],
    ["a quiz on the home screen", (state) => (state.place = { screen: "home", quiz: "physics" }), [{ path: "/place/quiz", code: "quiz-outside-run" }]],
    ["a task on the results screen", (state) => (state.place = { screen: "results", quiz: "heating", task: "units" }), [{ path: "/place/task", code: "task-without-run" }]],
    ["a task on the leaderboard", (state) => (state.place = { screen: "leaderboard", task: "units" }), [{ path: "/place/task", code: "task-without-run" }]],
    ["a quiz page without quiz", (state) => (state.place = { screen: "quiz" }), [{ path: "/place/quiz", code: "required" }]],
    ["a task on the quiz page", (state) => (state.place = { screen: "quiz", quiz: "cooling", task: "units" }), [{ path: "/place/task", code: "task-without-run" }]],
    ["a quiz on the learner page", (state) => (state.place = { screen: "learner", quiz: "physics" }), [{ path: "/place/quiz", code: "quiz-outside-run" }]],
    ["a quiz on the badges page", (state) => (state.place = { screen: "badges", quiz: "physics" }), [{ path: "/place/quiz", code: "quiz-outside-run" }]],
    ["a quiz on the preferences page", (state) => (state.place = { screen: "preferences", quiz: "physics" }), [{ path: "/place/quiz", code: "quiz-outside-run" }]],
  ];
  for (const [name, mutate, expected] of cases) {
    it(`reports ${name}`, () => {
      const state = name === "a non-object" ? null : mutated(presence(), mutate);
      expect(checkedPresence(state)).toEqual(name === "a non-object" ? [{ path: "", code: "type-invalid" }] : expected);
    });
  }

  it("returns the first issue by path, then code", () => {
    const state = mutated(presence(), (value) => {
      value.tag = "x";
      value.place = { screen: "home", quiz: "Physics", task: "units" };
      value.active = 1;
    });
    expect(checkedPresence(state)).toEqual([
      { path: "/active", code: "type-invalid" },
      { path: "/place/quiz", code: "quiz-outside-run" },
      { path: "/place/quiz", code: "slug-invalid" },
      { path: "/place/task", code: "task-without-run" },
      { path: "/tag", code: "tag-invalid" },
    ]);
    expect(presenceProblem(state)).toEqual({ path: "/active", code: "type-invalid" });
  });
});

describe("cursorProblem", () => {
  it("admits a tag alone, a pointer on the boundaries and a focus", () => {
    for (const state of [{ tag: TAG }, cursor(), { tag: TAG, cursor: { anchor: "home", x: 0, y: 1 } }, { tag: TAG, cursor: { anchor: "home", x: -0, y: 0.5 } }, { tag: TAG, focus: "card:quiz:energy-basics" }, { tag: TAG, cursor: { anchor: "a".repeat(64), x: 0.5, y: 0.5 } }]) {
      expect(checkedCursor(state), JSON.stringify(state)).toEqual([]);
    }
  });

  const cases: readonly [string, (state: Mutable) => void, readonly ValidationIssue[]][] = [
    ["a missing tag", (state) => delete state.tag, [{ path: "/tag", code: "required" }]],
    ["a tag that is not hex", (state) => (state.tag = "ghijklmn"), [{ path: "/tag", code: "tag-invalid" }]],
    ["x above one", (state) => (state.cursor.x = 1.0000001), [{ path: "/cursor/x", code: "out-of-range" }]],
    ["y below zero", (state) => (state.cursor.y = -0.001), [{ path: "/cursor/y", code: "out-of-range" }]],
    ["a pixel coordinate", (state) => (state.cursor.x = 1920), [{ path: "/cursor/x", code: "out-of-range" }]],
    ["a textual coordinate", (state) => (state.cursor.x = "0.5"), [{ path: "/cursor/x", code: "type-invalid" }]],
    ["a null coordinate", (state) => (state.cursor.y = null), [{ path: "/cursor/y", code: "type-invalid" }]],
    ["a missing coordinate", (state) => delete state.cursor.y, [{ path: "/cursor/y", code: "required" }]],
    ["an empty anchor", (state) => (state.cursor.anchor = ""), [{ path: "/cursor/anchor", code: "anchor-invalid" }]],
    ["an uppercase anchor", (state) => (state.cursor.anchor = "Task:units"), [{ path: "/cursor/anchor", code: "anchor-invalid" }]],
    ["a trailing separator", (state) => (state.cursor.anchor = "task:"), [{ path: "/cursor/anchor", code: "anchor-invalid" }]],
    ["a double separator", (state) => (state.cursor.anchor = "task::units"), [{ path: "/cursor/anchor", code: "anchor-invalid" }]],
    ["an anchor of 65 characters", (state) => (state.cursor.anchor = "a".repeat(65)), [{ path: "/cursor/anchor", code: "anchor-invalid" }]],
    ["a focus with a space", (state) => (state.focus = "task units"), [{ path: "/focus", code: "anchor-invalid" }]],
    ["a smuggled drag target", (state) => (state.target = "joule"), [{ path: "/target", code: "property-unknown" }]],
    ["a smuggled item", (state) => (state.cursor.item = "joule"), [{ path: "/cursor/item", code: "property-unknown" }]],
    ["a cursor that is no object", (state) => (state.cursor = [0.5, 0.5]), [{ path: "/cursor", code: "type-invalid" }]],
  ];
  for (const [name, mutate, expected] of cases) it(`reports ${name}`, () => expect(checkedCursor(mutated(cursor(), mutate))).toEqual(expected));

  it("refuses coordinates JSON cannot carry", () => {
    for (const value of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) expect(cursorProblem(mutated(cursor(), (state) => (state.cursor.x = value)))).toEqual({ path: "/cursor/x", code: "out-of-range" });
  });

  it("agrees with the tag and anchor predicates", () => {
    expect([isTag(TAG), isTag("0000000g"), isTag("0000000"), isTag("000000000")]).toEqual([true, false, false, false]);
    expect([isAnchor("home"), isAnchor("task:power-ladder"), isAnchor("a-b:c"), isAnchor(""), isAnchor("-a"), isAnchor("a".repeat(65))]).toEqual([true, true, true, false, false, false]);
  });
});

describe("presenceRoster", () => {
  const state = (tag: string, identity: PresenceState["identity"], place: Place, active: boolean): PresenceState => ({ tag, identity, place, active });
  const label = (value: PresenceState): string => (value.identity.kind === "anonymous" ? `Anonymous #${value.tag}` : value.identity.handle);
  const entries: readonly PresenceEntry[] = [
    { session: "s9", state: state("0000000b", { kind: "pseudonym", handle: "bea" }, { screen: "home" }, false) },
    { session: "s2", state: state("0000000a", { kind: "name", handle: "Ada" }, { screen: "run", quiz: "physics", task: "units" }, false) },
    { session: "s7", state: state("0000000a", { kind: "name", handle: "Ada" }, { screen: "results", quiz: "heating" }, true) },
    { session: "s1", state: state("0000000a", { kind: "name", handle: "Ada" }, { screen: "run", quiz: "physics" }, true) },
    { session: "s3", state: state("0000000c", { kind: "anonymous" }, { screen: "run", quiz: "physics" }, true) },
    { session: "s4", state: state("0000000d", { kind: "pseudonym", handle: "Bea" }, { screen: "quiz", quiz: "cooling" }, true) },
    { session: "s5", state: state("0000000e", { kind: "pseudonym", handle: "ada" }, { screen: "results", quiz: "physics" }, false) },
  ];

  it("groups sessions by tag, counts learners per quiz and sorts by display", () => {
    const roster = presenceRoster(entries, label);
    expect(roster.online).toBe(5);
    expect(roster.active).toBe(3);
    expect(roster.quizzes).toEqual({ cooling: 1, heating: 1, physics: 3 });
    expect(Object.keys(roster.quizzes)).toEqual(["cooling", "heating", "physics"]);
    expect(roster.learners.map((learner) => [learner.display, learner.tag])).toEqual([
      ["Ada", "0000000a"],
      ["ada", "0000000e"],
      ["Anonymous #0000000c", "0000000c"],
      ["Bea", "0000000d"],
      ["bea", "0000000b"],
    ]);
  });

  it("represents a learner by an active session, then the smallest session id, and lists every session", () => {
    const ada = presenceRoster(entries, label).learners[0]!;
    expect(ada).toEqual({ tag: "0000000a", identity: { kind: "name", handle: "Ada" }, display: "Ada", place: { screen: "run", quiz: "physics" }, active: true, session: "s1", sessions: ["s1", "s2", "s7"] });
    const idle = presenceRoster([entries[1]!, { ...entries[1]!, session: "s0" }], label).learners[0]!;
    expect([idle.session, idle.active]).toEqual(["s0", false]);
  });

  it("does not depend on the order of the entries", () => {
    const reference = presenceRoster(entries, label);
    for (let shift = 1; shift < entries.length; shift++) expect(presenceRoster([...entries.slice(shift), ...entries.slice(0, shift)], label)).toEqual(reference);
    expect(presenceRoster([...entries].reverse(), label)).toEqual(reference);
  });

  it("is empty without entries", () => {
    expect(presenceRoster([], label)).toEqual({ online: 0, active: 0, quizzes: {}, learners: [] });
  });
});
