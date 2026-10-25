/** 🧰️ Shared kit of the presence-client suites (`🚪️presence-room`, `🧭️presence-places-and-rooms`, `🖱️pointer-and-focus`, `🪟️presence-screens`): the sockets the test plays the proctor for, and the course, roster and snapshot fixtures. The suites together check shared presence in the web client (design §15–§17).
 *
 * @see ../🧫️fixtures/📡️presence-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
 */

import { cleanup, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { presenceSocketUrl } from "@semio-tech/framework-server";
import { learnerTag, type CourseView, type CursorState, type LeaderboardRow, type Place, type PresenceState, type ThinkingState } from "@semio-tech/course";
import { type PresenceConnect, type PresenceSocket, type CoursePresenceSnapshot, type CourseState } from "@semio-tech/course-react";

//#region 🔌️FakeSockets
/** 🎮️ A socket the test plays the proctor for: it records what the client sends and delivers what the test says. */
export class FakeSocket implements PresenceSocket {
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

export class FakeProctorSockets {
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
export const text = (en: string, de: string) => ({ en, de });
export const ME = "a".repeat(32);
export const MIRA = "b".repeat(32);
export const BEN = "c".repeat(32);
export const TIMING = { minMs: 100, maxMs: 1_000 };

export const COURSE: CourseView = {
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

export function presenceOf(learner: string, handle: string | undefined, place: Place, active = true): PresenceState {
  return { tag: learnerTag(learner), identity: handle === undefined ? { kind: "anonymous" } : { kind: "pseudonym", handle }, place, active };
}

export function row(rank: number, learner: string, handle: string): LeaderboardRow {
  return { rank, tag: learnerTag(learner), identity: { kind: "pseudonym", handle }, total: 100 - rank, reachedAt: rank, best: {}, badges: [], runs: 1, lastActivity: rank };
}

export const STATE: CourseState = {
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

export type Members<S> = readonly { readonly session: string; readonly colour: number; readonly state: S }[];

export function snapshot(
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

export const ROSTER = [
  { session: "s-me", colour: 0, state: presenceOf(ME, "Ada", { screen: "home" }) },
  { session: "s-mira", colour: 3, state: presenceOf(MIRA, "Mira K.", { screen: "run", quiz: "heating", task: "heating-task" }) },
  { session: "s-ben", colour: 5, state: presenceOf(BEN, "Ben", { screen: "leaderboard" }, false) },
];

export function stubSession() {
  return { open: vi.fn(), startRun: vi.fn(async () => undefined), resumeRun: vi.fn(async () => undefined), forgetLearner: vi.fn(), refreshLeaderboard: vi.fn(async () => undefined) };
}

export function box(element: Element, rect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): void {
  element.getBoundingClientRect = () => ({ ...rect, x: rect.left, y: rect.top, right: rect.left + rect.width, bottom: rect.top + rect.height, toJSON: () => rect }) as DOMRect;
}

export function pointer(target: Element, type: string, clientX: number, clientY: number, buttons = 0): void {
  target.dispatchEvent(new MouseEvent(type, { bubbles: true, clientX, clientY, buttons }));
}
export const HOME_ROSTER = ROSTER.map((member) => ({ ...member, state: { ...member.state, place: { screen: "home" as const }, active: true } }));
//#endregion 🏗️Fixtures
afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.restoreAllMocks();
});
