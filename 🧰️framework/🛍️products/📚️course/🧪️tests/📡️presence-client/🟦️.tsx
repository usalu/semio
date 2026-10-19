/** 📡️ Shared presence in the web client (design §15–§17): joining a room, rendering its batches, leaving, throttling
 * the frames it sends, rejoining after a lost socket, watching other rooms read-only, the rooms a place joins (home
 * watching every page behind the cards and every thinking room), the pointer, keyboard focus and drag it shares — on
 * the nearest anchor, an item or a card, also while pressed — the drafts it shares at most twice a second, and how the
 * screens show the others, inside the pages behind the cards too. The throttles have `lodash`'s `throttle` (leading and
 * trailing edge) as their third-party oracle; the place, cursor and watch vectors are shared JSON.
 *
 * @see ../../🧫️fixtures/📡️presence-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
 */

import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import throttle from "lodash/throttle";
import clamp from "lodash/clamp";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { PRESENCE_PROTOCOL, presenceSocketUrl } from "@semio-tech/framework-server";
import { cursorProblem, learnerTag, presenceProblem, thinkingProblem, type CourseView, type CursorState, type LeaderboardRow, type Place, type PresenceState, type ThinkingState } from "@semio-tech/course";
import {
  LeaderboardCard,
  CardPresence,
  LeaderboardPage,
  LearnerCard,
  MAX_WATCHED_ROOMS,
  PRESENCE_FRAME_HZ,
  PRESENCE_FRAME_INTERVAL_MS,
  PreferencesPanel,
  PresenceOverlay,
  PresenceProvider,
  PresenceRoom,
  QuizCardView,
  CoursePresence,
  THINKING_FRAME_INTERVAL_MS,
  WATCH_INTERVAL_MS,
  cursorAt,
  viewportCursor,
  homeWatchScopes,
  PEER_LABEL_CLEARANCE_PX,
  placePeers,
  presencePlace,
  presenceView,
  courseText,
  readPreferences,
  challengeOf,
  usePresencePointer,
  localStore,
  memoryStorageOrigin,
  type PresenceConnect,
  type PresenceSocket,
  type PresenceView,
  type CoursePresenceSnapshot,
  type CourseSession,
  type CourseState,
  type CourseStep,
} from "@semio-tech/course-react";
import { defaultCursorLightSvg as semioCursor } from "@semio-tech/assets/cursor/default/light";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import vectors from "../🧫️fixtures/📡️presence-client/🔣️.json";

//#region 🔌️FakeSockets
/** 🔌️ A socket the test plays the proctor for: it records what the client sends and delivers what the test says. */
class FakeSocket implements PresenceSocket {
  readyState = 1;
  onopen: ((event: Event) => void) | null = null;
  onmessage: ((event: MessageEvent) => void) | null = null;
  onclose: ((event: CloseEvent) => void) | null = null;
  onerror: ((event: Event) => void) | null = null;
  readonly sent: unknown[] = [];
  closed = false;

  constructor(
    readonly url: string,
    readonly protocol: string,
  ) {}

  send(data: string): void {
    this.sent.push(JSON.parse(data));
  }

  close(): void {
    this.closed = true;
    this.readyState = 3;
  }

  deliver(frame: unknown): void {
    this.onmessage?.(new MessageEvent("message", { data: JSON.stringify(frame) }));
  }

  welcome(session: string, colour: number, roster: readonly unknown[] = []): void {
    this.deliver({ type: "welcome", session, colour, roster });
  }

  drop(): void {
    this.readyState = 3;
    this.onclose?.(new CloseEvent("close"));
  }

  /** 🔌️ The connection of a socket that was still connecting is established. */
  open(): void {
    this.readyState = 1;
    this.onopen?.(new Event("open"));
  }

  states(): unknown[] {
    return this.sent.map((frame) => (frame as { readonly state: unknown }).state);
  }
}

class FakeProctorSockets {
  readonly sockets: FakeSocket[] = [];
  readonly connect: PresenceConnect = (url, protocol) => {
    const socket = new FakeSocket(url, protocol);
    this.sockets.push(socket);
    return socket;
  };

  open(): FakeSocket[] {
    return this.sockets.filter((socket) => !socket.closed);
  }

  at(scope: string): FakeSocket {
    const path = presenceSocketUrl(window.location.origin, scope, "").split("?")[0]!;
    const found = this.open().filter((socket) => socket.url.split("?")[0] === path);
    expect(
      found,
      `open sockets at ${path}: ${this.open()
        .map((socket) => socket.url)
        .join(", ")}`,
    ).toHaveLength(1);
    return found[0]!;
  }
}
//#endregion 🔌️FakeSockets

//#region 🏗️Fixtures
const text = (en: string, de: string) => ({ en, de });
const ME = "a".repeat(32);
const MIRA = "b".repeat(32);
const BEN = "c".repeat(32);
const TIMING = { minMs: 100, maxMs: 1_000 };

const COURSE: CourseView = {
  id: "arch",
  title: text("Architecture", "Architektur"),
  introduction: { title: text("How it works", "So geht's"), paragraphs: [text("Classify, sort and match.", "Klassifiziere, sortiere und ordne zu.")] },
  quizzes: ["physics", "heating", "cooling", "demand"].map((id) => ({
    id,
    emoji: "🔥",
    title: text(`Quiz ${id}`, `Quiz ${id}`),
    description: text(`About ${id}.`, `Über ${id}.`),
    tasks: [{ id: `${id}-task`, kind: "sorting" as const, title: text("Task", "Aufgabe") }],
  })),
  badges: [],
};

function presenceOf(learner: string, handle: string | undefined, place: Place, active = true): PresenceState {
  return { tag: learnerTag(learner), identity: handle === undefined ? { kind: "anonymous" } : { kind: "pseudonym", handle }, place, active };
}

function row(rank: number, learner: string, handle: string): LeaderboardRow {
  return { rank, tag: learnerTag(learner), identity: { kind: "pseudonym", handle }, total: 100 - rank, reachedAt: rank, best: {}, badges: [], runs: 1, lastActivity: rank };
}

const STATE: CourseState = {
  step: { screen: "home" },
  trail: { back: [], forward: [] },
  introduced: true,
  learner: { id: ME, identity: { kind: "pseudonym", handle: "Ada" } },
  course: COURSE,
  learnerView: { learner: ME, identity: { kind: "pseudonym", handle: "Ada" }, runs: [], badges: [], best: {}, total: 0 },
  runs: {},
  awards: {},
  crowds: {},
  asked: [],
  board: { period: "all-time" },
  leaderboards: { "all-time": { board: { period: "all-time", rows: [row(1, MIRA, "Mira K."), row(2, BEN, "Ben"), row(3, ME, "Ada")], learners: 3, submissions: 3, own: row(3, ME, "Ada") }, at: Date.UTC(2026, 8, 29, 12) } },
};

type Members<S> = readonly { readonly session: string; readonly colour: number; readonly state: S }[];

function snapshot(
  roster: Members<PresenceState>,
  room: Members<CursorState> = [],
  watched: Readonly<Record<string, Members<CursorState | ThinkingState>>> = {},
  thinking?: { readonly scope: string; readonly members: Members<ThinkingState> },
): CoursePresenceSnapshot {
  const open = <S, W = never>(members: Members<S>, self: string, rooms: Readonly<Record<string, Members<W>>> = {}) => ({
    status: "open" as const,
    self,
    colour: 0,
    members: members.map((member) => ({ ...member, surface: "course" })),
    watched: new Map(Object.entries(rooms).map(([scope, entries]) => [scope, entries.map((member) => ({ ...member, surface: "course" }))] as const)),
    refused: undefined,
  });
  return {
    tag: learnerTag(ME),
    roster: open(roster, "s-me"),
    room: open(room, "r-me", watched),
    thinking: thinking === undefined ? undefined : open(thinking.members, "t-me"),
    scopes: { room: "arch/home", thinking: thinking?.scope },
  };
}

const ROSTER = [
  { session: "s-me", colour: 0, state: presenceOf(ME, "Ada", { screen: "home" }) },
  { session: "s-mira", colour: 3, state: presenceOf(MIRA, "Mira K.", { screen: "run", quiz: "heating", task: "heating-task" }) },
  { session: "s-ben", colour: 5, state: presenceOf(BEN, "Ben", { screen: "leaderboard" }, false) },
];

function stubSession() {
  return { open: vi.fn(), startRun: vi.fn(async () => undefined), resumeRun: vi.fn(async () => undefined), forgetLearner: vi.fn(), refreshLeaderboard: vi.fn(async () => undefined) };
}

function box(element: Element, rect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): void {
  element.getBoundingClientRect = () => ({ ...rect, x: rect.left, y: rect.top, right: rect.left + rect.width, bottom: rect.top + rect.height, toJSON: () => rect }) as DOMRect;
}

function pointer(target: Element, type: string, clientX: number, clientY: number, buttons = 0): void {
  target.dispatchEvent(new MouseEvent(type, { bubbles: true, clientX, clientY, buttons }));
}
const HOME_ROSTER = ROSTER.map((member) => ({ ...member, state: { ...member.state, place: { screen: "home" as const }, active: true } }));
//#endregion 🏗️Fixtures

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.restoreAllMocks();
});

describe("👥️ presence room", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  it("joins with the presence subprotocol, keeps the admitted members of the welcome and sends its state after joining", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({
      url: "ws://proctor.test/scopes/arch%2Fhome/presence/ws?surface=home",
      connect: proctor.connect,
      parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined),
      timing: TIMING,
    });
    room.start();
    const socket = proctor.sockets[0]!;
    expect(socket.protocol).toBe(PRESENCE_PROTOCOL);
    expect(room.getSnapshot().status).toBe("connecting");
    room.publish({ tag: learnerTag(ME) });
    expect(socket.sent).toEqual([]);
    socket.welcome("r-me", 2, [
      { session: "r-mira", colour: 3, surface: "home", state: { cursor: { anchor: "home:board", x: 0.5, y: 0.25 }, tag: learnerTag(MIRA) } },
      { session: "r-new", colour: 4, surface: "home", state: null },
      { session: "r-bad", colour: 5, surface: "home", state: { tag: learnerTag(BEN), cursor: { anchor: "Home Board", x: 2, y: 0 } } },
    ]);
    expect(room.getSnapshot()).toMatchObject({ status: "open", self: "r-me", colour: 2 });
    expect(room.getSnapshot().members).toEqual([{ session: "r-mira", colour: 3, surface: "home", state: { cursor: { anchor: "home:board", x: 0.5, y: 0.25 }, tag: learnerTag(MIRA) } }]);
    expect(socket.sent).toEqual([{ type: "state", state: { tag: learnerTag(ME) } }]);
    room.stop();
    expect(socket.closed).toBe(true);
    expect(room.getSnapshot()).toMatchObject({ status: "stopped", members: [] });
  });

  it("leaves a socket that still connects only once it is open, so no connection is reported failed, and ignores what it says meanwhile", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined), timing: TIMING });
    room.start();
    const socket = proctor.sockets[0]!;
    socket.readyState = 0;
    room.stop();
    expect(socket.closed).toBe(false);
    expect(room.getSnapshot().status).toBe("stopped");
    socket.open();
    expect(socket.closed).toBe(true);
    socket.welcome("r-late", 1, [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA) } }]);
    expect(room.getSnapshot()).toMatchObject({ status: "stopped", members: [] });
    await vi.advanceTimersByTimeAsync(TIMING.maxMs * 4);
    expect(proctor.sockets).toHaveLength(1);
  });

  it("renders every batch — joins, moves and leaves — and keeps a refusal without closing", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined) });
    room.start();
    const socket = proctor.sockets[0]!;
    socket.welcome("r-me", 0);
    socket.deliver({ type: "batch", entries: [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA), cursor: { anchor: "home:board", x: 0.1, y: 0.2 } } }], left: [] });
    socket.deliver({
      type: "batch",
      entries: [
        { session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA), cursor: { anchor: "home:learner", x: 0.3, y: 0.4 } } },
        { session: "r-ben", colour: 5, surface: "home", state: { tag: learnerTag(BEN), focus: "home:board" } },
      ],
      left: [],
    });
    expect(room.getSnapshot().members.map((member) => [member.session, member.state])).toEqual([
      ["r-mira", { tag: learnerTag(MIRA), cursor: { anchor: "home:learner", x: 0.3, y: 0.4 } }],
      ["r-ben", { tag: learnerTag(BEN), focus: "home:board" }],
    ]);
    socket.deliver({ type: "batch", entries: [], left: ["r-mira"] });
    expect(room.getSnapshot().members.map((member) => member.session)).toEqual(["r-ben"]);
    socket.deliver({ type: "refused", reason: "tag-invalid /tag" });
    expect(room.getSnapshot()).toMatchObject({ status: "open", refused: "tag-invalid /tag" });
    expect(socket.closed).toBe(false);
    socket.deliver("not json");
    socket.deliver({ type: "surprise" });
    expect(room.getSnapshot().members.map((member) => member.session)).toEqual(["r-ben"]);
  });

  for (const vector of vectors.throttle) {
    it(`sends at most ${PRESENCE_FRAME_HZ} frames a second, latest state first: ${vector.id}`, async () => {
      const proctor = new FakeProctorSockets();
      const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined });
      room.start();
      const socket = proctor.sockets[0]!;
      socket.welcome("r-me", 0);
      const start = Date.now();
      const sent: [number, number][] = [];
      const send = socket.send.bind(socket);
      socket.send = (data) => {
        sent.push([Date.now() - start, (JSON.parse(data) as { readonly state: { readonly cursor: { readonly x: number } } }).state.cursor.x * 100]);
        send(data);
      };
      const oracle: [number, number][] = [];
      const reference = throttle((value: number) => oracle.push([Date.now() - start, value]), PRESENCE_FRAME_INTERVAL_MS);
      for (const [at, value] of vector.calls as [number, number][]) {
        await vi.advanceTimersByTimeAsync(at - (Date.now() - start));
        room.publish({ tag: learnerTag(ME), cursor: { anchor: "home:board", x: value / 100, y: 0 } });
        reference(value);
      }
      await vi.advanceTimersByTimeAsync(1_000);
      expect(sent).toEqual(vector.sent);
      const values = vector.calls.map((call) => call[1]);
      if (new Set(values).size === values.length) expect(sent).toEqual(oracle);
    });
  }

  it("keeps a continuous stream under the rate and delivers the last position", async () => {
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: () => undefined });
    room.start();
    const socket = proctor.sockets[0]!;
    socket.welcome("r-me", 0);
    const times: number[] = [];
    const send = socket.send.bind(socket);
    socket.send = (data) => {
      times.push(Date.now());
      send(data);
    };
    for (let step = 1; step <= 1_000; step += 1) {
      room.publish({ tag: learnerTag(ME), cursor: { anchor: "home:board", x: step / 1_000, y: 0.5 } });
      await vi.advanceTimersByTimeAsync(1);
    }
    await vi.advanceTimersByTimeAsync(1_000);
    expect(times.length).toBeLessThanOrEqual(PRESENCE_FRAME_HZ + 1);
    for (const [index, time] of times.entries()) if (index > 0) expect(time - times[index - 1]!).toBeGreaterThanOrEqual(PRESENCE_FRAME_INTERVAL_MS);
    expect(socket.states().at(-1)).toEqual({ tag: learnerTag(ME), cursor: { anchor: "home:board", x: 1, y: 0.5 } });
  });

  it("rejoins a lost socket with jittered backoff, forgets the members meanwhile and sends the latest state again", async () => {
    vi.spyOn(Math, "random").mockReturnValue(0.5);
    const proctor = new FakeProctorSockets();
    const room = new PresenceRoom<CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: (state) => (cursorProblem(state) === undefined ? (state as CursorState) : undefined), timing: TIMING, random: () => 0.5 });
    room.start();
    const first = proctor.sockets[0]!;
    first.welcome("r-1", 0, [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA) } }]);
    room.publish({ tag: learnerTag(ME), focus: "home:board" });
    expect(first.states()).toEqual([{ tag: learnerTag(ME), focus: "home:board" }]);
    first.drop();
    expect(room.getSnapshot()).toMatchObject({ status: "waiting", members: [] });
    await vi.advanceTimersByTimeAsync(49);
    expect(proctor.sockets).toHaveLength(1);
    await vi.advanceTimersByTimeAsync(1);
    expect(proctor.sockets).toHaveLength(2);
    proctor.sockets[1]!.drop();
    await vi.advanceTimersByTimeAsync(149);
    expect(proctor.sockets).toHaveLength(2);
    await vi.advanceTimersByTimeAsync(1);
    expect(proctor.sockets).toHaveLength(3);
    const third = proctor.sockets[2]!;
    expect(third.sent).toEqual([]);
    third.welcome("r-3", 1, [{ session: "r-mira", colour: 3, surface: "home", state: { tag: learnerTag(MIRA) } }]);
    expect(third.states()).toEqual([{ tag: learnerTag(ME), focus: "home:board" }]);
    expect(room.getSnapshot()).toMatchObject({ status: "open", self: "r-3" });
    expect(room.getSnapshot().members.map((member) => member.session)).toEqual(["r-mira"]);
    room.stop();
    await vi.advanceTimersByTimeAsync(10_000);
    expect(proctor.sockets).toHaveLength(3);
  });

  it("watches rooms once joined, replaces a room by its snapshot, applies changes, forgets unwatched rooms and watches again after a rejoin", async () => {
    const proctor = new FakeProctorSockets();
    const cursor = (state: unknown): CursorState | undefined => (cursorProblem(state) === undefined ? (state as CursorState) : undefined);
    const room = new PresenceRoom<CursorState, CursorState>({ url: "ws://proctor.test/x", connect: proctor.connect, parse: cursor, parseWatched: (_, state) => cursor(state), timing: TIMING, random: () => 0.5 });
    room.start();
    room.watch([]);
    room.watch(["arch/leaderboard", "arch/badges", "arch/leaderboard"]);
    const first = proctor.sockets[0]!;
    expect(first.sent).toEqual([]);
    first.welcome("r-me", 0);
    expect(first.sent).toEqual([{ type: "watch", scopes: ["arch/leaderboard", "arch/badges"], intervalMs: WATCH_INTERVAL_MS }]);
    room.watch(["arch/leaderboard", "arch/badges"]);
    expect(first.sent).toHaveLength(1);
    const mira = { tag: learnerTag(MIRA), cursor: { anchor: "leaderboard", x: 0.5, y: 0.5 } };
    first.deliver({ type: "watched", scope: "arch/leaderboard", entries: [{ session: "w-mira", colour: 3, surface: "leaderboard", state: mira }], left: [], snapshot: true });
    first.deliver({ type: "watched", scope: "arch/badges", entries: [{ session: "w-ben", colour: 5, surface: "badges", state: { tag: learnerTag(BEN) } }], left: [] });
    first.deliver({ type: "watched", scope: "arch/secret", entries: [{ session: "w-eve", colour: 1, surface: "x", state: { tag: learnerTag(BEN) } }], left: [] });
    first.deliver({ type: "watched", scope: "arch/badges", entries: [{ session: "w-bad", colour: 1, surface: "badges", state: { tag: "Not A Tag" } }], left: [] });
    const watched = () => Object.fromEntries([...room.getSnapshot().watched].map(([scope, members]) => [scope, members.map((member) => member.session)]));
    expect(watched()).toEqual({ "arch/leaderboard": ["w-mira"], "arch/badges": ["w-ben"] });
    expect(room.getSnapshot().watched.get("arch/leaderboard")?.[0]?.state).toEqual(mira);
    first.deliver({ type: "watched", scope: "arch/leaderboard", entries: [{ session: "w-ann", colour: 2, surface: "leaderboard", state: { tag: learnerTag(ME) } }], left: ["w-mira"] });
    first.deliver({ type: "watched", scope: "arch/badges", entries: [{ session: "w-ann", colour: 2, surface: "badges", state: { tag: learnerTag(ME) } }], left: [], snapshot: true });
    expect(watched()).toEqual({ "arch/leaderboard": ["w-ann"], "arch/badges": ["w-ann"] });
    room.watch(["arch/badges"]);
    expect(first.sent.at(-1)).toEqual({ type: "watch", scopes: ["arch/badges"], intervalMs: WATCH_INTERVAL_MS });
    expect(watched()).toEqual({ "arch/badges": ["w-ann"] });
    room.watch(Array.from({ length: MAX_WATCHED_ROOMS + 4 }, (_, index) => `arch/quiz/q${index}`));
    expect((first.sent.at(-1) as { readonly scopes: readonly string[] }).scopes).toHaveLength(MAX_WATCHED_ROOMS);
    first.drop();
    expect(watched()).toEqual({});
    await vi.advanceTimersByTimeAsync(TIMING.minMs);
    const second = proctor.sockets.at(-1)!;
    expect(second).not.toBe(first);
    second.welcome("r-me-2", 0);
    expect(second.sent).toEqual([first.sent.at(-1)]);
    room.stop();
  });
});

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

describe("🖱️ pointer and focus", () => {
  function Harness(props: { readonly presence: CoursePresence }) {
    usePresencePointer(props.presence);
    return (
      <div>
        <section data-presence-anchor="task:u-values">
          <ul>
            <li data-course-item="glazing" data-presence-anchor="item:glazing">
              <span data-course-grip="">⠿</span>
              <button type="button">Single glazing</button>
            </li>
          </ul>
        </section>
        <button type="button">Outside</button>
      </div>
    );
  }

  for (const vector of vectors.cursors) {
    it(`measures a cursor: ${vector.id}`, () => {
      const anchor = document.createElement("div");
      anchor.dataset.presenceAnchor = "home:board";
      box(anchor, vector.box);
      const cursor = cursorAt(anchor, vector.point[0]!, vector.point[1]!);
      expect(cursor ?? null).toEqual(vector.expected === null ? null : { anchor: "home:board", ...vector.expected });
    });
  }

  for (const vector of vectors.viewportCursors) {
    it(`tracks every viewport gap: ${vector.id}`, () => {
      const width = window.innerWidth;
      const height = window.innerHeight;
      Object.defineProperties(window, { innerWidth: { configurable: true, value: vector.viewport[0] }, innerHeight: { configurable: true, value: vector.viewport[1] } });
      try {
        expect(viewportCursor(vector.point[0]!, vector.point[1]!)).toEqual(vector.expected);
        expect([vector.expected.x, vector.expected.y]).toEqual(vector.point.map((value, index) => Math.round(clamp(value / vector.viewport[index]!, 0, 1) * 10_000) / 10_000));
        expect(cursorProblem({ tag: learnerTag(ME), cursor: vector.expected })).toBeUndefined();
      } finally {
        Object.defineProperties(window, { innerWidth: { configurable: true, value: width }, innerHeight: { configurable: true, value: height } });
      }
    });
  }

  it("keeps pointing across captured drags, clears cancel and blur, and never shares observer interactions", () => {
    const presence = { point: vi.fn(), focusOn: vi.fn(), drag: vi.fn() };

    const rendered = render(<Harness presence={presence as unknown as CoursePresence} />);
    const item = document.querySelector('[data-presence-anchor="item:glazing"]')!;
    const outside = screen.getByRole("button", { name: "Outside" });
    box(item, { left: 100, top: 80, width: 200, height: 40 });
    Object.defineProperty(document, "elementFromPoint", { configurable: true, value: vi.fn(() => outside) });
    const grip = item.querySelector("[data-course-grip]")!;
    pointer(grip, "pointerdown", 110, 100, 1);
    pointer(grip, "pointermove", 10, 10, 1);
    expect(presence.point).toHaveBeenLastCalledWith(viewportCursor(10, 10));
    pointer(grip, "pointercancel", 10, 10);
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    fireEvent.keyDown(document, { key: "Tab" });
    item.querySelector("button")!.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith("item:glazing");
    window.dispatchEvent(new Event("blur"));
    expect(presence.point).toHaveBeenLastCalledWith(undefined);
    expect(presence.focusOn).toHaveBeenLastCalledWith(undefined);
    rendered.unmount();
    function Disabled() { usePresencePointer(presence as unknown as CoursePresence, false); return <button type="button">Observer</button>; }
    render(<Disabled />);
    presence.point.mockClear();
    pointer(screen.getByRole("button", { name: "Observer" }), "pointermove", 500, 300);
    expect(presence.point).not.toHaveBeenCalled();
    Reflect.deleteProperty(document, "elementFromPoint");
  });
  it("names the nearest anchor under the pointer — an item before its card — also while pressed, the item a grip drags until release, and keyboard focus only", () => {
    const presence = { point: vi.fn(), focusOn: vi.fn(), drag: vi.fn() };
    render(<Harness presence={presence as unknown as CoursePresence} />);
    box(document.querySelector('[data-presence-anchor="task:u-values"]')!, { left: 100, top: 50, width: 200, height: 100 });
    const item = document.querySelector('[data-presence-anchor="item:glazing"]')!;
    box(item, { left: 100, top: 80, width: 200, height: 40 });
    const button = screen.getByRole("button", { name: "Single glazing" });
    const outside = screen.getByRole("button", { name: "Outside" });
    const grip = item.querySelector("[data-course-grip]")!;
    pointer(button, "pointermove", 150, 100);
    expect(presence.point).toHaveBeenLastCalledWith({ anchor: "item:glazing", x: 0.25, y: 0.5 });
    pointer(document.querySelector("section")!, "pointermove", 150, 60);
    expect(presence.point).toHaveBeenLastCalledWith({ anchor: "task:u-values", x: 0.25, y: 0.1 });
    pointer(button, "pointermove", 160, 100, 1);
    expect(presence.point).toHaveBeenLastCalledWith({ anchor: "item:glazing", x: 0.3, y: 0.5 });
    pointer(outside, "pointermove", 10, 10);
    expect(presence.point).toHaveBeenLastCalledWith(viewportCursor(10, 10));
    pointer(grip, "pointerdown", 110, 100, 1);
    expect(presence.drag).toHaveBeenLastCalledWith("glazing");
    pointer(grip, "pointerup", 110, 100);
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    pointer(grip, "pointerdown", 110, 100, 1);
    expect(presence.drag).toHaveBeenLastCalledWith("glazing");
    fireEvent.keyDown(document, { key: "Escape" });
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    pointer(button, "pointerdown", 150, 100, 1);
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    button.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith(undefined);
    fireEvent.keyDown(document, { key: "Tab" });
    outside.focus();
    button.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith("item:glazing");
    outside.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith(undefined);
    for (const call of presence.point.mock.calls) if (call[0] !== undefined) expect(cursorProblem({ tag: learnerTag(ME), cursor: call[0], drag: { item: "glazing" } })).toBeUndefined();
  });
});

describe("👁️ what the screens show", () => {
  for (const vector of vectors.cardActivity) {
    it(`shows card activity: ${vector.id}`, () => {
      const roster = [{ session: "s-me", colour: 0, state: presenceOf(ME, "Ada", { screen: "home" }) }, { session: "s-mira", colour: 3, state: presenceOf(MIRA, "Mira K.", vector.place as Place, vector.active) }];
      const member = { tag: learnerTag(MIRA), ...("cursor" in vector ? { cursor: vector.cursor } : {}), ...("focus" in vector ? { focus: vector.focus } : {}) } as CursorState;
      const view = presenceView(snapshot(roster, [ { session: "r-mira", colour: 3, state: member } ]), courseText("en"));
      render(<PresenceProvider view={view} setTask={() => undefined}><CardPresence anchor={vector.anchor} accepts={(place) => place.quiz === "heating"} text={courseText("en")} /></PresenceProvider>);
      expect(document.querySelector(`[data-card-presence] [data-tag="${learnerTag(MIRA)}"]`) !== null).toBe(vector.expected);
    });
  }
  it("places observed card cursors and focus on the visible grid without using hidden owner or preview anchors", () => {
    const { container } = render(<>
      <div hidden inert><div data-presence-anchor="home:board">Held owner</div></div>
      <div data-follow-screen="home">
        <div data-course-grid="root" data-grid-active="">
          <div data-layered-pane="preview" inert><div data-presence-anchor="home:board">Preview</div></div>
          <div data-layered-overlay="" inert><div data-presence-anchor="home:board">Observed card</div></div>
        </div>
      </div>
      <div data-presence-layer="">
        <div data-peer="cursor" data-anchor="home:board" data-x="0.5" data-y="0.5" hidden />
        <div data-peer="focus" data-anchor="home:board" hidden />
      </div>
    </>);
    const anchors = [...container.querySelectorAll<HTMLElement>("[data-presence-anchor]")];
    box(anchors[0]!, { left: 0, top: 0, width: 400, height: 400 });
    box(anchors[1]!, { left: 900, top: 900, width: 400, height: 400 });
    box(anchors[2]!, { left: 50, top: 250, width: 200, height: 100 });
    const layer = container.querySelector<HTMLElement>("[data-presence-layer]")!;
    const [cursor, focus] = [...layer.children] as HTMLElement[];
    placePeers(layer);
    expect([cursor!.hidden, cursor!.style.transform]).toEqual([false, "translate(150px, 300px)"]);
    expect([focus!.hidden, focus!.style.transform, focus!.style.width, focus!.style.height]).toEqual([false, "translate(50px, 250px)", "200px", "100px"]);
    container.querySelector("[data-grid-active]")!.removeAttribute("data-grid-active");
    placePeers(layer);
    expect([cursor!.hidden, focus!.hidden]).toEqual([true, true]);
  });

  it("maps viewport cursors over gaps above cards and scales them inside preview pages", () => {
    const layer = document.createElement("div");
    layer.innerHTML = '<div data-peer="cursor" data-anchor="viewport" data-x="0.25" data-y="0.75" hidden></div>';
    document.body.append(layer);
    const mark = layer.firstElementChild as HTMLElement;
    placePeers(layer);
    expect(mark.hidden).toBe(false);
    expect(mark.style.transform).toBe(`translate(${window.innerWidth / 4}px, ${window.innerHeight * 0.75}px)`);
    const page = document.createElement("div");
    Object.defineProperties(page, { offsetWidth: { value: 800 }, offsetHeight: { value: 600 } });
    page.append(layer);
    document.body.append(page);
    placePeers(layer, page);
    expect(mark.style.transform).toBe("translate(200px, 450px)");
    page.remove();
  });

  it("shows named colour badges for card occupants, hover and keyboard focus, excluding away learners", () => {
    const roster = ROSTER.map((member) => member.state.tag === learnerTag(BEN) ? { ...member, state: { ...member.state, active: true, place: { screen: "home" as const } } } : member);
    const view = presenceView(snapshot(roster, [{ session: "r-ben", colour: 5, state: { tag: learnerTag(BEN), focus: "home:quiz:heating" } }]), courseText("en"));
    const { rerender } = render(<PresenceProvider view={view} setTask={() => undefined}><CardPresence anchor="home:quiz:heating" accepts={(place) => place.quiz === "heating"} text={courseText("en")} /></PresenceProvider>);
    const list = document.querySelector('[data-card-presence="home:quiz:heating"]')!;
    expect(screen.getByRole("list", { name: "Online: 2" })).toBe(list);
    expect(list.getAttribute("role")).toBe("list");
    expect(within(list as HTMLElement).getAllByRole("listitem").map((item) => [item.getAttribute("data-tag"), item.textContent])).toEqual([[learnerTag(BEN), "Ben"], [learnerTag(MIRA), "Mira K."]]);
    expect((list.querySelector(`[data-tag="${learnerTag(MIRA)}"]`) as HTMLElement).style.getPropertyValue("--course-peer")).toBe("var(--presence-3)");
    const away = presenceView(snapshot(roster.map((member) => ({ ...member, state: { ...member.state, active: false } }))), courseText("en"));
    rerender(<PresenceProvider view={away} setTask={() => undefined}><CardPresence anchor="home:quiz:heating" accepts={(place) => place.quiz === "heating"} text={courseText("en")} /></PresenceProvider>);
    expect(document.querySelector("[data-card-presence]")).toBeNull();
  });

  it("keeps root and nested overview viewport cursors on their own public home route", () => {
    const roster = HOME_ROSTER.map((member) => member.state.tag === learnerTag(MIRA) ? { ...member, state: { ...member.state, view: { home: "quizzes" } } } : member);
    const captured = snapshot(roster, [{ session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), place: { screen: "home" }, home: "quizzes", cursor: { anchor: "viewport", x: 0.5, y: 0.5 } } }]);
    expect(presenceView({ ...captured, place: { screen: "home" } }, courseText("en")).peers).toEqual([]);
    expect(presenceView({ ...captured, place: { screen: "home" }, home: "quizzes" }, courseText("en")).peers.map((peer) => peer.session)).toEqual(["r-mira"]);
    const stale = { ...captured, room: { ...captured.room!, members: captured.room!.members.map((member) => ({ ...member, state: { ...member.state, home: undefined } })) } };
    expect(presenceView({ ...stale, place: { screen: "home" }, home: "quizzes" }, courseText("en")).peers).toEqual([]);
    render(<PresenceProvider view={presenceView(captured, courseText("en"))} setTask={() => undefined}><CardPresence anchor="home:quizzes" accepts={(place, view) => place.screen === "home" && view?.home === "quizzes"} text={courseText("en")} /></PresenceProvider>);
    expect(document.querySelector('[data-card-presence="home:quizzes"]')?.textContent).toBe("Mira K.");
  });
  it("filters stale, away, missing-roster and mixed-screen pointers within a shared quiz room", () => {
    const at: Place = { screen: "run", quiz: "heating", task: "heat" };
    const roster = [
      { session: "s-me", colour: 0, state: presenceOf(ME, "Ada", at) },
      { session: "s-mira", colour: 3, state: presenceOf(MIRA, "Mira K.", at) },
      { session: "s-ben", colour: 5, state: presenceOf(BEN, "Ben", { screen: "results", quiz: "heating" }) },
    ];
    const cursor = { anchor: "viewport", x: 0.25, y: 0.5 };
    const captured = snapshot(roster, [
      { session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), place: at, cursor } },
      { session: "r-stale", colour: 3, state: { tag: learnerTag(MIRA), place: { screen: "quiz", quiz: "heating" }, cursor } },
      { session: "r-ben", colour: 5, state: { tag: learnerTag(BEN), place: { screen: "results", quiz: "heating" }, cursor } },
      { session: "r-unknown", colour: 7, state: { tag: "0badc0de", place: at, cursor } },
    ]);
    expect(presenceView({ ...captured, scopes: { ...captured.scopes, room: "arch/quiz/heating" } }, courseText("en")).peers.map((peer) => peer.session)).toEqual(["r-mira"]);
    const hidden = { ...captured, roster: { ...captured.roster!, members: captured.roster!.members.map((member) => ({ ...member, state: { ...member.state, active: false } })) }, scopes: { ...captured.scopes, room: "arch/quiz/heating" } };
    expect(presenceView(hidden, courseText("en")).peers).toEqual([]);
  });
  it("labels the others by name, colours them by their roster slot everywhere and leaves the own sessions out of the cursors", () => {
    const view = presenceView(
      snapshot(HOME_ROSTER, [
        { session: "r-me", colour: 0, state: { tag: learnerTag(ME), cursor: { anchor: "home:board", x: 0.1, y: 0.1 } } },
        { session: "r-me-other-tab", colour: 1, state: { tag: learnerTag(ME) } },
        { session: "r-mira", colour: 9, state: { tag: learnerTag(MIRA), cursor: { anchor: "home:board", x: 0.5, y: 0.5 } } },
        { session: "r-anon", colour: 7, state: { tag: "0badc0de", focus: "home:learner" } },
      ]),
      courseText("en"),
    );
    expect(view.roster).toMatchObject({ online: 3, active: 3, quizzes: {} });
    expect(view.colours.get(learnerTag(MIRA))).toBe(3);
    expect(view.peers.map((peer) => [peer.label, peer.colour, peer.cursor, peer.focus])).toEqual([
      ["Mira K.", 3, { anchor: "home:board", x: 0.5, y: 0.5 }, undefined],
    ]);
  });

  it("draws the others' pointers and focus frames on their anchors, decoratively, and nothing when switched off", () => {
    const view = presenceView(
      snapshot(HOME_ROSTER, [
        { session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), cursor: { anchor: "home:board", x: 0.5, y: 0.25 } } },
        { session: "r-ben", colour: 5, state: { tag: learnerTag(BEN), focus: "home:learner" } },
      ]),
      courseText("en"),
    );
    const { rerender } = render(
      <>
        <section data-presence-anchor="home:board">Board</section>
        <section data-presence-anchor="home:learner">Learner</section>
        <PresenceOverlay view={view} show={false} />
      </>,
    );
    expect(document.querySelector("[data-presence-layer]")).toBeNull();
    box(document.querySelector('[data-presence-anchor="home:board"]')!, { left: 400, top: 100, width: 300, height: 200 });
    box(document.querySelector('[data-presence-anchor="home:learner"]')!, { left: 10, top: 20, width: 100, height: 50 });
    rerender(
      <>
        <section data-presence-anchor="home:board">Board</section>
        <section data-presence-anchor="home:learner">Learner</section>
        <PresenceOverlay view={view} show />
      </>,
    );
    const layer = document.querySelector<HTMLElement>("[data-presence-layer]")!;
    expect(layer.getAttribute("aria-hidden")).toBe("true");
    expect(layer.className).toContain("pointer-events-none");
    const cursor = layer.querySelector<HTMLElement>('[data-peer="cursor"]')!;
    expect(cursor.hidden).toBe(false);
    expect(cursor.style.transform).toBe("translate(550px, 150px)");
    expect(cursor.textContent).toBe("Mira K.");
    expect([...cursor.querySelectorAll(".course-peer-arrow path")].map((path) => path.getAttribute("d"))).toEqual([...semioCursor.matchAll(/\sd="([^"]+)"/gu)].map((match) => match[1]));
    expect((cursor.parentElement as HTMLElement).style.getPropertyValue("--course-peer")).toBe("var(--presence-3)");
    const focus = layer.querySelector<HTMLElement>('[data-peer="focus"]')!;
    expect([focus.style.transform, focus.style.width, focus.style.height]).toEqual(["translate(10px, 20px)", "100px", "50px"]);
    expect(focus.textContent).toBe("Ben");
  });

  it("gathers the others of every watched room and the drafts of every thinking room, never the own", () => {
    const mira: ThinkingState = { tag: learnerTag(MIRA), answers: { heat: { kind: "classification", assignments: { kettle: "power" } } } };
    const ben: ThinkingState = { tag: learnerTag(BEN), answers: {} };
    const mine: ThinkingState = { tag: learnerTag(ME), answers: {} };
    const view = presenceView(
      snapshot(
        ROSTER.map((member) => member.state.tag === learnerTag(MIRA) ? { ...member, state: { ...member.state, place: { screen: "leaderboard" as const } } } : member),
        [{ session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), focus: "home:board" } }],
        {
          "arch/leaderboard": [
            { session: "w-mira", colour: 9, state: { tag: learnerTag(MIRA), cursor: { anchor: "leaderboard", x: 0.5, y: 0.5 } } },
            { session: "w-me", colour: 0, state: { tag: learnerTag(ME), cursor: { anchor: "leaderboard", x: 0.1, y: 0.1 } } },
          ],
          "arch/quiz/heating/thinking": [
            { session: "t-mira", colour: 9, state: mira },
            { session: "t-me-other-tab", colour: 0, state: mine },
          ],
        },
        {
          scope: "arch/quiz/cooling/thinking",
          members: [
            { session: "t-ben", colour: 5, state: ben },
            { session: "t-me", colour: 0, state: mine },
          ],
        },
      ),
      courseText("en"),
    );
    expect(view.rooms.get("arch/home")?.map((peer) => peer.label)).toEqual([]);
    expect(view.rooms.get("arch/leaderboard")?.map((peer) => [peer.label, peer.colour, peer.cursor])).toEqual([["Mira K.", 3, { anchor: "leaderboard", x: 0.5, y: 0.5 }]]);
    expect(view.rooms.has("arch/quiz/heating/thinking")).toBe(false);
    expect(view.thinking.get("arch/quiz/heating/thinking")).toEqual([mira]);
    expect(view.thinking.get("arch/quiz/cooling/thinking")).toEqual([ben]);
  });

  it("names the item a learner drags beside the pointer", () => {
    const view = presenceView(snapshot(HOME_ROSTER, [{ session: "r-mira", colour: 3, state: { tag: learnerTag(MIRA), cursor: { anchor: "item:candle", x: 0.5, y: 0.5 }, drag: { item: "candle" } } }]), courseText("en"));
    render(
      <>
        <li data-presence-anchor="item:candle">Tea light</li>
        <PresenceOverlay view={view} show itemLabel={(item) => (item === "candle" ? "Tea light" : undefined)} />
      </>,
    );
    const cursor = document.querySelector<HTMLElement>('[data-presence-layer] [data-peer="cursor"]')!;
    expect(cursor.textContent).toBe("Mira K. ▸ Tea light");
    expect(cursor.dataset.drag).toBe("candle");
  });

  it("places the others inside a page scaled on the overview in the page's own pixels, on its own anchors only", () => {
    const page = document.createElement("div");
    Object.defineProperty(page, "offsetWidth", { value: 800 });
    box(page, { left: 100, top: 50, width: 400, height: 300 });
    const anchor = document.createElement("section");
    anchor.dataset.presenceAnchor = "leaderboard";
    box(anchor, { left: 150, top: 100, width: 100, height: 50 });
    const layer = document.createElement("div");
    layer.innerHTML = '<div data-peer="cursor" data-anchor="leaderboard" data-x="0.5" data-y="0.5" hidden></div><div data-peer="focus" data-anchor="leaderboard" hidden></div><div data-peer="cursor" data-anchor="badges" data-x="0" data-y="0"></div>';
    page.append(anchor, layer);
    const elsewhere = document.createElement("section");
    elsewhere.dataset.presenceAnchor = "badges";
    document.body.append(page, elsewhere);
    placePeers(layer, page);
    const [cursor, focus, outside] = [...layer.querySelectorAll<HTMLElement>("[data-peer]")];
    expect([cursor!.hidden, cursor!.style.transform]).toEqual([false, "translate(200px, 150px)"]);
    expect([focus!.style.transform, focus!.style.width, focus!.style.height]).toEqual(["translate(100px, 100px)", "200px", "100px"]);
    expect(outside!.hidden).toBe(true);
    page.remove();
    elsewhere.remove();
  });

  it("hides a name label that would cover the focused control or come closer than its clearance, and shows it again elsewhere", () => {
    const anchor = document.createElement("section");
    anchor.dataset.presenceAnchor = "task:masses";
    box(anchor, { left: 0, top: 0, width: 1000, height: 600 });
    const control = document.createElement("button");
    box(control, { left: 400, top: 300, width: 120, height: 24 });
    const layer = document.createElement("div");
    layer.innerHTML = '<div data-peer="cursor" data-anchor="task:masses" data-x="0.41" data-y="0.5" hidden><span class="course-peer-label">Mira K.</span></div><div data-peer="cursor" data-anchor="task:masses" data-x="0.9" data-y="0.9" hidden><span class="course-peer-label">Ben</span></div>';
    for (const label of layer.querySelectorAll(".course-peer-label")) {
      Object.defineProperty(label, "offsetWidth", { value: 60 });
      Object.defineProperty(label, "offsetHeight", { value: 18 });
    }
    document.body.append(anchor, control, layer);
    const [near, far] = [...layer.querySelectorAll<HTMLElement>("[data-peer]")];
    placePeers(layer);
    expect([near!.hasAttribute("data-shy"), far!.hasAttribute("data-shy")]).toEqual([false, false]);
    control.focus();
    placePeers(layer);
    expect([near!.hidden, near!.hasAttribute("data-shy"), far!.hasAttribute("data-shy")]).toEqual([false, true, false]);
    near!.dataset.x = String((400 - 16 - 60 - PEER_LABEL_CLEARANCE_PX - 2) / 1000);
    placePeers(layer);
    expect(near!.hasAttribute("data-shy")).toBe(false);
    near!.dataset.x = String((400 - 16 - 60 - PEER_LABEL_CLEARANCE_PX + 2) / 1000);
    placePeers(layer);
    expect(near!.hasAttribute("data-shy")).toBe(true);
    control.blur();
    placePeers(layer);
    expect(near!.hasAttribute("data-shy")).toBe(false);
    expect(stylesheet).toMatch(/\[data-shy\] > \.course-peer-label \{\s*visibility: hidden;/u);
    for (const element of [anchor, control, layer]) element.remove();
  });

  it("shows who is where on the learner card, learners in a quiz on its card and online marks in the leaderboard", () => {
    const view: PresenceView = presenceView(snapshot(ROSTER), courseText("en"));
    const common = { session: stubSession() as unknown as CourseSession, state: STATE, text: courseText("en"), locale: "en" as const };
    const card = { revealed: false, onOpen: () => undefined };
    render(
      <PresenceProvider view={view} setTask={() => undefined}>
        <LearnerCard {...common} {...card} />
        {COURSE.quizzes.map((quiz) => (
          <QuizCardView key={quiz.id} {...common} {...card} quiz={quiz} busy={false} act={() => undefined} challenge="medium" />
        ))}
        <LeaderboardCard {...common} {...card} />
        <LeaderboardPage {...common} view={{ opened: true, revealed: false }} />
      </PresenceProvider>,
    );
    const learner = screen.getByRole("region", { name: "Ada" });
    expect(learner.getAttribute("data-presence-anchor")).toBe("home:learner");
    const list = within(learner).getByRole("group");
    expect(within(list).getByText("Online: 3 · Who is where")).toBeTruthy();
    expect(
      within(list)
        .getAllByRole("listitem")
        .map((item) => item.textContent),
    ).toEqual(["onlineAda (you)– on the overview", "onlineBen– on the leaderboard (away)", "onlineMira K.– working on Quiz heating"]);
    const heating = screen.getByRole("region", { name: "Quiz heating" });
    expect(heating.getAttribute("data-presence-anchor")).toBe("home:quiz:heating");
    expect(within(heating).getByText("Learning now: 1").getAttribute("aria-hidden")).toBeNull();
    const nobody = within(screen.getByRole("region", { name: "Quiz cooling" })).getByText("Learning now: 0");
    expect([nobody.getAttribute("aria-hidden"), nobody.classList.contains("invisible")]).toEqual(["true", true]);
    const table = screen.getAllByRole("table")[1]!;
    const names = within(table)
      .getAllByRole("rowheader")
      .map((cell) => [cell.textContent, cell.querySelector(".course-online") === null ? "offline" : "online"]);
    expect(names).toEqual([
      ["Mira K.online", "online"],
      ["Benonline", "online"],
      ["Ada (you)online", "online"],
    ]);
    for (const anchor of document.querySelectorAll<HTMLElement>("[data-presence-anchor]")) expect(cursorProblem({ tag: learnerTag(ME), focus: anchor.dataset.presenceAnchor })).toBeUndefined();
  });

  it("keeps the cursor preference on the device, shown unless switched off", () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, "arch");
    expect(readPreferences(store).showCursors).toBe(true);
    store.write("preferences", { theme: "dark", textSize: "large", showCursors: false });
    expect(readPreferences(store)).toMatchObject({ theme: "dark", textSize: "large", showCursors: false });
    const onChange = vi.fn();
    render(<PreferencesPanel preferences={readPreferences(store)} locale="de" text={courseText("de")} onChange={onChange} />);
    const box = screen.getByRole("checkbox", { name: "Cursor der anderen anzeigen" }) as HTMLInputElement;
    expect(box.checked).toBe(false);
    fireEvent.click(box);
    expect(onChange).toHaveBeenLastCalledWith({ theme: "dark", textSize: "large", showCursors: true, others: "submitted", animateIcons: true, iconsChosen: false, pets: "lively", petsLiveliness: "lively", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {}, locale: undefined });
    store.write("preferences", { challenges: { physics: "expert", heating: "easy" } });
    expect(readPreferences(store).challenges).toEqual({ physics: "expert", heating: "easy" });
    expect(["physics", "heating", "cooling"].map((quiz) => challengeOf(readPreferences(store), quiz))).toEqual(["expert", "easy", "medium"]);
    store.write("preferences", { challenges: { physics: "lenient", "Not A Slug": "hard", "../heating": "hard", cooling: 3, demand: "hard" } });
    expect(readPreferences(store).challenges).toEqual({ demand: "hard" });
    for (const stored of [{}, { challenge: "expert" }, { challenges: "expert" }, { challenges: 3 }, { challenges: ["hard"] }]) {
      store.write("preferences", stored);
      expect(readPreferences(store).challenges, JSON.stringify(stored)).toEqual({});
    }
    expect(screen.queryByRole("group", { name: "Herausforderung" })).toBeNull();
  });
});
