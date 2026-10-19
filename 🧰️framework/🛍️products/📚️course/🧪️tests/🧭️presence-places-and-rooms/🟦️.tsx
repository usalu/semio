/** 🧭️ The places and rooms of presence (design §16): the rooms a place joins (home watching every page behind the cards and every thinking room), the pointer and keyboard focus it shares there, the drafts it shares at most twice a second — with `lodash`'s `throttle` as the third-party oracle — and the item a learner drags; the place and watch vectors are shared JSON.
 *
 * @see ../🧰️presence-client-kit/🟦️.tsx — the sockets and fixtures
 * @see ../🧫️fixtures/📡️presence-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
 */

import { screen } from "@testing-library/react";
import throttle from "lodash/throttle";
import { describe, expect, it, vi } from "vitest";
import { presenceSocketUrl } from "@semio-tech/framework-server";
import { cursorProblem, learnerTag, presenceProblem, thinkingProblem, type Place, type ThinkingState } from "@semio-tech/course";
import { PRESENCE_FRAME_INTERVAL_MS, CoursePresence, THINKING_FRAME_INTERVAL_MS, WATCH_INTERVAL_MS, homeWatchScopes, presencePlace, type CourseState, type CourseStep } from "@semio-tech/course-react";
import vectors from "../🧫️fixtures/📡️presence-client/🔣️.json";
import { COURSE, FakeProctorSockets, ME, TIMING, pointer, presenceOf } from "../🧰️presence-client-kit/🟦️.tsx";

describe("🧭️ presence places and rooms", () => {
  for (const vector of vectors.places) {
    it(`places ${vector.id}`, () => {
      const place = presencePlace(vector.step as CourseStep, { runs: vector.runs as unknown as CourseState["runs"], course: { ...COURSE, quizzes: COURSE.quizzes.filter((quiz) => vectors.quizzes.includes(quiz.id)) } }, vector.task ?? undefined);
      expect(place ?? null).toEqual(vector.expected);
      if (place !== undefined) expect(presenceProblem(presenceOf(ME, "Ada", place.place))).toBeUndefined();
    });
  }

  it("joins the roster and the room of each place, shares where the learner is and leaves rooms it no longer needs", async () => {
    vi.useFakeTimers();
    const proctor = new FakeProctorSockets();
    const presence = new CoursePresence({ proctor: "", connect: proctor.connect, timing: TIMING });
    const self = { tag: learnerTag(ME), identity: { kind: "pseudonym" as const, handle: "Ada" } };
    presence.update({ course: "arch", self: undefined, at: { place: { screen: "introduction" }, room: true }, active: true });
    expect(proctor.sockets).toEqual([]);
    for (const vector of vectors.places) {
      if (vector.expected === null) continue;
      presence.update({ course: "arch", self, at: vector.expected as { readonly place: Place; readonly room: boolean }, active: true });
      const roster = proctor.at("arch");
      expect(roster.url).toBe(presenceSocketUrl(window.location.origin, "arch", "course"));
      if (roster.sent.length === 0) roster.welcome("s-me", 0);
      await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
      expect(roster.states().at(-1)).toEqual(presenceOf(ME, "Ada", vector.expected.place as Place));
      const rooms = proctor.open().filter((socket) => socket !== roster);
      if (vector.roomScope === null) {
        expect(rooms).toEqual([]);
        continue;
      }
      const room = proctor.at(vector.roomScope);
      if (room.sent.length === 0) {
        expect(room.url).toBe(presenceSocketUrl(window.location.origin, vector.roomScope, (vector.expected.place as Place).screen));
        room.welcome("r-me", 0);
      }
      expect(room.states().at(-1)).toEqual({ tag: learnerTag(ME), place: vector.expected.place });
    }
    presence.update({ course: "arch", self, at: { place: { screen: "home" }, room: true }, active: false });
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    expect(proctor.at("arch").states().at(-1)).toEqual(presenceOf(ME, "Ada", { screen: "home" }, false));
    presence.stop();
    expect(proctor.open()).toEqual([]);
  });

  it("shares the pointer and keyboard focus as anchors in [0, 1], latest wins, and forgets them in another room", async () => {
    vi.useFakeTimers();
    const proctor = new FakeProctorSockets();
    const presence = new CoursePresence({ proctor: "", connect: proctor.connect, timing: TIMING });
    const self = { tag: learnerTag(ME), identity: { kind: "anonymous" as const } };
    presence.update({ course: "arch", self, at: { place: { screen: "home" }, room: true }, active: true });
    const room = proctor.at("arch/home");
    room.welcome("r-me", 0);
    presence.point({ anchor: "home:quiz:heating", x: 0.25, y: 0.75 });
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    presence.focusOn("home:board");
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    presence.point(undefined);
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    expect(room.states()).toEqual([
      { tag: learnerTag(ME) },
      { tag: learnerTag(ME), cursor: { anchor: "home:quiz:heating", x: 0.25, y: 0.75 } },
      { tag: learnerTag(ME), cursor: { anchor: "home:quiz:heating", x: 0.25, y: 0.75 }, focus: "home:board" },
      { tag: learnerTag(ME), focus: "home:board" },
    ].map((state) => ({ ...state, place: { screen: "home" } })));
    for (const state of room.states()) {
      expect(cursorProblem(state)).toBeUndefined();
      expect(Object.keys(state as object).every((key) => ["tag", "cursor", "focus", "place"].includes(key))).toBe(true);
    }
    presence.update({ course: "arch", self, at: { place: { screen: "leaderboard" }, room: true }, active: true });
    expect(room.closed).toBe(true);
    const next = proctor.at("arch/leaderboard");
    next.welcome("r-me-2", 0);
    expect(next.states()).toEqual([{ tag: learnerTag(ME), place: { screen: "leaderboard" } }]);
    presence.stop();
  });

  for (const vector of vectors.watchScopes) {
    it(`watches from home: ${vector.id}`, () => {
      expect(homeWatchScopes(vector.course, vector.quizzes, vector.joined)).toEqual(vector.expected);
    });
  }

  it("watches every page and thinking room from home over the room it joined, and nothing elsewhere", async () => {
    vi.useFakeTimers();
    const proctor = new FakeProctorSockets();
    const presence = new CoursePresence({ proctor: "", connect: proctor.connect, timing: TIMING });
    const self = { tag: learnerTag(ME), identity: { kind: "pseudonym" as const, handle: "Ada" } };
    const input = { course: "arch", quizzes: vectors.quizzes, self, active: true };
    presence.update({ ...input, at: { place: { screen: "home" }, room: true }, home: true });
    const home = proctor.at("arch/home");
    home.welcome("r-me", 0);
    expect(home.sent).toEqual([
      { type: "state", state: { tag: learnerTag(ME), place: { screen: "home" } } },
      { type: "watch", scopes: homeWatchScopes("arch", vectors.quizzes, "arch/home"), intervalMs: WATCH_INTERVAL_MS },
    ]);
    presence.update({ ...input, at: { place: { screen: "home" }, room: true }, home: true });
    expect(home.sent).toHaveLength(2);
    presence.update({ ...input, at: { place: { screen: "leaderboard" }, room: true }, home: true });
    const board = proctor.at("arch/leaderboard");
    board.welcome("r-me-2", 0);
    expect(board.sent.at(-1)).toEqual({ type: "watch", scopes: homeWatchScopes("arch", vectors.quizzes, "arch/leaderboard"), intervalMs: WATCH_INTERVAL_MS });
    presence.update({ ...input, at: { place: { screen: "introduction" }, room: true }, home: false });
    const introduction = proctor.at("arch/introduction");
    introduction.welcome("r-me-3", 0);
    expect(introduction.sent).toEqual([{ type: "state", state: { tag: learnerTag(ME), place: { screen: "introduction" } } }]);
    presence.stop();
  });

  it("clears pointers when moving between nested home grids or quiz/run/results surfaces sharing one room", async () => {
    vi.useFakeTimers();
    const proctor = new FakeProctorSockets();
    const presence = new CoursePresence({ proctor: "", connect: proctor.connect, timing: TIMING });
    const input = { course: "arch", self: { tag: learnerTag(ME), identity: { kind: "anonymous" as const } }, active: true };
    presence.update({ ...input, at: { place: { screen: "home" }, room: true } });
    const home = proctor.at("arch/home");
    home.welcome("r-me", 0);
    presence.point({ anchor: "viewport", x: 0.25, y: 0.5 });
    presence.focusOn("home:quizzes");
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    presence.update({ ...input, at: { place: { screen: "home" }, room: true }, view: { home: "quizzes" } });
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    expect(home.closed).toBe(false);
    expect(home.states().at(-1)).toEqual({ tag: learnerTag(ME), place: { screen: "home" }, home: "quizzes" });
    expect(presence.getSnapshot()).toMatchObject({ place: { screen: "home" }, home: "quizzes" });
    presence.update({ ...input, at: { place: { screen: "quiz", quiz: "heating" }, room: true } });
    const quiz = proctor.at("arch/quiz/heating");
    quiz.welcome("r-me-quiz", 0);
    presence.point({ anchor: "viewport", x: 0.25, y: 0.5 });
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    presence.update({ ...input, at: { place: { screen: "run", quiz: "heating", task: "heat" }, room: true } });
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    expect(quiz.closed).toBe(false);
    expect(quiz.states().at(-1)).toEqual({ tag: learnerTag(ME), place: { screen: "run", quiz: "heating", task: "heat" } });
    presence.stop();
  });
  it("shares the drafts of the run on screen in the quiz's thinking room at most twice a second, the latest winning", async () => {
    vi.useFakeTimers();
    const proctor = new FakeProctorSockets();
    const presence = new CoursePresence({ proctor: "", connect: proctor.connect, timing: TIMING });
    const self = { tag: learnerTag(ME), identity: { kind: "anonymous" as const } };
    const at = { place: { screen: "run", quiz: "heating", task: "heat" } as Place, room: true };
    const draft = (step: number) => ({ quiz: "heating", answers: { masses: { kind: "sorting" as const, order: step % 2 === 0 ? ["mouse", "cat"] : ["cat", "mouse"] }, heat: { kind: "classification" as const, assignments: { kettle: `c${step}` } } } });
    presence.update({ course: "arch", self, at, drafts: draft(0), active: true });
    const thinking = proctor.at("arch/quiz/heating/thinking");
    expect(thinking.url).toBe(presenceSocketUrl(window.location.origin, "arch/quiz/heating/thinking", "thinking"));
    const start = Date.now();
    const sent: [number, string][] = [];
    const send = thinking.send.bind(thinking);
    thinking.send = (data) => {
      const state = (JSON.parse(data) as { readonly state: ThinkingState }).state;
      expect(thinkingProblem(state)).toBeUndefined();
      sent.push([Date.now() - start, (state.answers.heat as { readonly assignments: Readonly<Record<string, string>> }).assignments.kettle!]);
      send(data);
    };
    const oracle: [number, string][] = [];
    const reference = throttle((kettle: string) => oracle.push([Date.now() - start, kettle]), THINKING_FRAME_INTERVAL_MS);
    thinking.welcome("t-me", 0);
    reference("c0");
    for (let step = 1; step <= 12; step += 1) {
      await vi.advanceTimersByTimeAsync(170);
      presence.update({ course: "arch", self, at, drafts: draft(step), active: true });
      reference(`c${step}`);
    }
    await vi.advanceTimersByTimeAsync(1_000);
    expect(sent.map(([, kettle]) => kettle)).toEqual(oracle.map(([, kettle]) => kettle));
    for (const [index, [time]] of sent.entries()) {
      expect(time).toBeLessThanOrEqual(oracle[index]![0]);
      if (index > 0) expect(time - sent[index - 1]![0]).toBeGreaterThanOrEqual(THINKING_FRAME_INTERVAL_MS);
    }
    expect(thinking.states().at(-1)).toEqual({ tag: learnerTag(ME), answers: draft(12).answers });
    presence.update({ course: "arch", self, at: { place: { screen: "results", quiz: "heating" }, room: true }, active: true });
    expect(thinking.closed).toBe(true);
    presence.stop();
  });

  it("shares the item a learner drags with the pointer in the room of the place", async () => {
    vi.useFakeTimers();
    const proctor = new FakeProctorSockets();
    const presence = new CoursePresence({ proctor: "", connect: proctor.connect, timing: TIMING });
    presence.update({ course: "arch", self: { tag: learnerTag(ME), identity: { kind: "anonymous" } }, at: { place: { screen: "run", quiz: "heating", task: "heat" }, room: true }, active: true });
    const room = proctor.at("arch/quiz/heating");
    room.welcome("r-me", 0);
    presence.point({ anchor: "item:kettle", x: 0.1, y: 0.5 });
    presence.drag("kettle");
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    presence.drag(undefined);
    await vi.advanceTimersByTimeAsync(PRESENCE_FRAME_INTERVAL_MS);
    expect(room.states().slice(-2)).toEqual([
      { tag: learnerTag(ME), cursor: { anchor: "item:kettle", x: 0.1, y: 0.5 }, drag: { item: "kettle" } },
      { tag: learnerTag(ME), cursor: { anchor: "item:kettle", x: 0.1, y: 0.5 } },
    ].map((state) => ({ ...state, place: { screen: "run", quiz: "heating", task: "heat" } })));
    for (const state of room.states()) expect(cursorProblem(state)).toBeUndefined();
    presence.stop();
  });
});
