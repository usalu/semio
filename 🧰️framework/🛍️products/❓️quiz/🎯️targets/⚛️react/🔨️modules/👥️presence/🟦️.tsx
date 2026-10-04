/** 👥️ Shared presence, cursors and what the others think (design §15–§17): every learner sees who else is online and
 * where, the pointers, keyboard focus and drags of the learners on the same page — on the home page also inside every
 * page behind the cards — and, in a quiz, the draft answers of everyone thinking along.
 *
 * Presence sockets of the proctor carry it — ephemeral shared state, never persisted, one socket per room kind: the
 * catalog's roster room with each learner's {@link PresenceState} (tag, identity, place, whether the tab is visible);
 * the room of the current place with each {@link CursorState}, which on home also *watches* the rooms of every page
 * behind the cards and every quiz's thinking room read-only at about 4 Hz; and in a run the quiz's thinking room, where
 * the learner's drafts go as a {@link ThinkingState} at most twice a second. A cursor is relative (0…1) to an anchor
 * every learner at that place renders (`data-presence-anchor`: cards, the leaderboard, a task card, an item
 * `item:<id>`, a category `category:<id>`), so it survives other viewports and randomized item orders; a drag names the
 * item it carries. State frames go out at most {@link PRESENCE_FRAME_HZ} times a second, latest state wins; a lost socket
 * reconnects with jittered backoff — ever more slowly while sessions keep ending right after they began — and sends its
 * latest state and watch again. Others' cursors are decorative (`aria-hidden`); their name labels are written in the ink
 * that reads on their palette colour and step aside from the control that has keyboard focus. The roster is text for
 * everyone: the online count in the navbar, who is where on the learner card, learners in a quiz on its card and online
 * marks in the leaderboard.
 *
 * @see ../../../../🔨️modules/👥️presence/🟦️.ts — rooms, admission checks, the roster and the thinking crowd of the core
 * @see ../../../../../🖥️server/🟦️.ts — `presenceSocketUrl`, the `semio.presence.v1` frames including `watch`
 */

import { createContext, useContext, useEffect, useLayoutEffect, useRef, useState, type CSSProperties, type ReactElement, type ReactNode } from "react";
import { retryWithJitteredBackoff } from "@semio-tech/framework";
import { PRESENCE_PROTOCOL, decodePresenceFrame, encodePresenceFrame, presenceSocketUrl, type PresenceEntry as SocketEntry } from "@semio-tech/framework-server";
import {
  cursorProblem,
  learnerTag,
  presenceProblem,
  presenceRoster,
  roomScope,
  rosterScope,
  thinkingAnswer,
  thinkingProblem,
  thinkingScope,
  type Anchor,
  type CatalogView,
  type Cursor,
  type CursorState,
  type Identity,
  type Place,
  type PresenceRoster,
  type PresenceState,
  type RunView,
  type ThinkingAnswer,
  type ThinkingState,
} from "@semio-tech/quiz";
import { Icon, presenceColor, presencePaint } from "@semio-tech/ui-react/chrome";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { learnerName } from "../🪪️identity/🟦️.tsx";
import { RETRY_TIMING, type RetryTiming } from "../🛂️proctor/🟦️.ts";
import { HOME_PAGES, type QuizState, type QuizStep } from "../🧭️session/🟦️.ts";
import { cn } from "../🪟️chrome/🟦️.tsx";

//#region 🔌️Socket
/** ⏱️ The most state frames a room sends per second (design §15: pointer frames ≤ 15 Hz). */
export const PRESENCE_FRAME_HZ = 15;

/** ⏱️ The least time between two state frames of one room. */
export const PRESENCE_FRAME_INTERVAL_MS = Math.ceil(1000 / PRESENCE_FRAME_HZ);

/** ⏱️ The least time between two thinking frames (design §17: drafts ≤ 2 Hz, coalesced). */
export const THINKING_FRAME_INTERVAL_MS = 500;

/** ⏱️ How often the watched rooms report their changes (design §17: about 4 Hz). */
export const WATCH_INTERVAL_MS = 250;

/** 🔢️ The most rooms one socket watches (the framework's bound). */
export const MAX_WATCHED_ROOMS = 16;

/** ⏱️ How long a joined session must last to count as stable; one that ends sooner is a flap. */
export const PRESENCE_STABLE_MS = 10_000;

/** ⏳️ The pause before a room joins again after `flaps` short-lived sessions in a row (`random` in [0, 1]): a first loss
 * is a blip and is rejoined within the shortest backoff; from the second flap on the pause is at least that backoff
 * plus a random share of an exponentially growing ceiling, capped at the longest — so a proctor that admits and drops
 * sockets (a connection limit) is asked ever more rarely. */
export function rejoinDelay(flaps: number, timing: RetryTiming, random: number): number {
  if (flaps <= 1) return random * timing.minMs;
  return timing.minMs + random * (Math.min(timing.maxMs, timing.minMs * 2 ** (flaps - 1)) - timing.minMs);
}

/** 🔌️ What a presence room needs of a WebSocket; the browser's `WebSocket` is one. */
export interface PresenceSocket {
  readonly readyState: number;
  onopen: ((event: Event) => void) | null;
  onmessage: ((event: MessageEvent) => void) | null;
  onclose: ((event: CloseEvent) => void) | null;
  onerror: ((event: Event) => void) | null;
  send(data: string): void;
  close(code?: number, reason?: string): void;
}

/** 🔌️ Opens a presence socket at `url` speaking `protocol`. */
export type PresenceConnect = (url: string, protocol: string) => PresenceSocket;

/** 🌐️ The browser's WebSocket. */
export const browserPresenceConnect: PresenceConnect = (url, protocol) => new WebSocket(url, protocol);

const CONNECTING = 0;
const OPEN = 1;

/** 👋️ Closes a socket that is left: at once when it is open, else as soon as it opens — a socket closed while it still
 * connects is reported by the browser as a failed connection, though nothing failed. */
function leave(socket: PresenceSocket | undefined): void {
  if (socket === undefined) return;
  if (socket.readyState === CONNECTING) socket.onopen = () => socket.close(1000, "left");
  else socket.close(1000, "left");
}

/** 🚦️ Where a room is: joining, joined, waiting to join again after a loss, or left. */
export type RoomStatus = "connecting" | "open" | "waiting" | "stopped";

/** 🧍️ One session of a room with an admitted state. */
export interface RoomMember<S> {
  readonly session: string;
  readonly colour: number;
  readonly surface: string;
  readonly state: S;
}

/** 📸️ What a room knows: its status, the own session and palette slot once joined, the members, the members of every
 * watched room by scope, and the last refusal. */
export interface RoomSnapshot<S, W = never> {
  readonly status: RoomStatus;
  readonly self: string | undefined;
  readonly colour: number | undefined;
  readonly members: readonly RoomMember<S>[];
  readonly watched: ReadonlyMap<string, readonly RoomMember<W>[]>;
  readonly refused: string | undefined;
}

/** ⚙️ One room: its socket URL, the socket factory, the checks that admit a member's state (of the joined room, and of
 * a watched room by scope), the frame interval, the backoff bounds and the random source of the jitter. */
export interface PresenceRoomOptions<S, W = never> {
  readonly url: string;
  readonly connect: PresenceConnect;
  readonly parse: (state: unknown) => S | undefined;
  readonly parseWatched?: (scope: string, state: unknown) => W | undefined;
  readonly intervalMs?: number;
  readonly timing?: RetryTiming;
  readonly random?: () => number;
}

function pause(ms: number, signal: AbortSignal): Promise<void> {
  return new Promise((resolve) => {
    const timer = setTimeout(done, ms);
    signal.addEventListener("abort", done, { once: true });
    function done(): void {
      clearTimeout(timer);
      signal.removeEventListener("abort", done);
      resolve();
    }
  });
}

/** 🚪️ One presence room over one socket: joins on {@link start}, keeps the admitted members of the latest `welcome` and
 * every `batch`, sends the latest {@link publish}ed state at most once per interval, watches the rooms of
 * {@link watch} (their `watched` frames: a snapshot replaces a room, else changes apply), sends state and watch again
 * after every rejoin, rejoins a lost socket with jittered backoff and leaves on {@link stop}. */
export class PresenceRoom<S, W = never> {
  private readonly listeners = new Set<() => void>();
  private snapshot: RoomSnapshot<S, W> = { status: "stopped", self: undefined, colour: undefined, members: [], watched: new Map(), refused: undefined };
  private members = new Map<string, RoomMember<S>>();
  private watchedMembers = new Map<string, Map<string, RoomMember<W>>>();
  private socket: PresenceSocket | undefined;
  private self: string | undefined;
  private colour: number | undefined;
  private refused: string | undefined;
  private welcomed = false;
  private desired: string | undefined;
  private sent: string | undefined;
  private sentAt = Number.NEGATIVE_INFINITY;
  private watching: { readonly scopes: readonly string[]; readonly frame: string } | undefined;
  private watchSent: string | undefined;
  private flushTimer: ReturnType<typeof setTimeout> | undefined;
  private lifetime: AbortController | undefined;

  constructor(private readonly options: PresenceRoomOptions<S, W>) {}

  readonly subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  readonly getSnapshot = (): RoomSnapshot<S, W> => this.snapshot;

  /** 🚪️ Joins the room, and keeps joining until {@link stop}. */
  start(): void {
    if (this.lifetime !== undefined) return;
    const lifetime = new AbortController();
    this.lifetime = lifetime;
    this.emit("connecting");
    void this.run(lifetime.signal).catch(() => undefined);
  }

  /** 👋️ Leaves the room: closes the socket, drops the members and forgets what was sent. */
  stop(): void {
    this.lifetime?.abort();
    this.lifetime = undefined;
    clearTimeout(this.flushTimer);
    this.flushTimer = undefined;
    leave(this.detach());
    this.members = new Map();
    this.watchedMembers = new Map();
    this.self = undefined;
    this.colour = undefined;
    this.emit("stopped");
  }

  /** 📤️ Shares `state`, replacing any state not sent yet. */
  publish(state: S): void {
    this.desired = JSON.stringify(encodePresenceFrame({ type: "state", state }));
    this.schedule();
  }

  /** 👀️ Watches the rooms of `scopes` read-only (at most {@link MAX_WATCHED_ROOMS}), their changes every `intervalMs`;
   * rooms no longer watched are forgotten. */
  watch(scopes: readonly string[], intervalMs: number = WATCH_INTERVAL_MS): void {
    const kept = [...new Set(scopes)].slice(0, MAX_WATCHED_ROOMS);
    if (kept.length === 0 && this.watching === undefined) return;
    const frame = JSON.stringify(encodePresenceFrame({ type: "watch", scopes: kept, intervalMs }));
    if (frame === this.watching?.frame) return;
    this.watching = { scopes: kept, frame };
    const before = this.watchedMembers.size;
    for (const scope of [...this.watchedMembers.keys()]) if (!kept.includes(scope)) this.watchedMembers.delete(scope);
    this.sendWatch();
    if (this.watchedMembers.size !== before) this.emit(this.snapshot.status);
  }

  private async run(signal: AbortSignal): Promise<void> {
    const timing = this.options.timing ?? RETRY_TIMING;
    let flaps = 0;
    while (!signal.aborted) {
      const joined = await retryWithJitteredBackoff(() => this.join(signal), { ...timing, signal });
      const since = Date.now();
      await joined.ended;
      if (signal.aborted) return;
      flaps = Date.now() - since < PRESENCE_STABLE_MS ? flaps + 1 : 0;
      await pause(rejoinDelay(flaps, timing, (this.options.random ?? Math.random)()), signal);
    }
  }

  private join(signal: AbortSignal): Promise<{ readonly ended: Promise<void> }> {
    return new Promise((resolve, reject) => {
      if (signal.aborted) {
        reject(signal.reason);
        return;
      }
      const socket = this.options.connect(this.options.url, PRESENCE_PROTOCOL);
      let end: () => void = () => undefined;
      const ended = new Promise<void>((done) => (end = done));
      signal.addEventListener("abort", () => (reject(signal.reason), end()), { once: true });
      this.socket = socket;
      socket.onmessage = (event) => {
        if (this.receive(socket, event.data) === "welcome") resolve({ ended });
      };
      socket.onclose = () => {
        if (this.socket === socket) this.lost();
        reject(new Error("presence socket closed"));
        end();
      };
      socket.onerror = () => undefined;
    });
  }

  private receive(socket: PresenceSocket, data: unknown): string | undefined {
    if (socket !== this.socket || typeof data !== "string") return undefined;
    let frame: ReturnType<typeof decodePresenceFrame>;
    try {
      frame = decodePresenceFrame(JSON.parse(data));
    } catch {
      return undefined;
    }
    switch (frame.type) {
      case "welcome":
        this.welcomed = true;
        this.self = frame.session;
        this.colour = frame.colour;
        this.refused = undefined;
        this.members = new Map();
        for (const entry of frame.roster) this.upsert(entry);
        this.emit("open");
        this.schedule();
        this.sendWatch();
        break;
      case "batch":
        for (const session of frame.left) this.members.delete(session);
        for (const entry of frame.entries) this.upsert(entry);
        this.emit("open");
        break;
      case "watched":
        this.receiveWatched(frame.scope, frame.entries, frame.left, frame.snapshot === true);
        break;
      case "refused":
        this.refused = frame.reason;
        this.emit("open");
        break;
      case "state":
      case "watch":
        break;
    }
    return frame.type;
  }

  private receiveWatched(scope: string, entries: readonly SocketEntry[], left: readonly string[], snapshot: boolean): void {
    const parse = this.options.parseWatched;
    if (parse === undefined || !this.watching?.scopes.includes(scope)) return;
    const room = snapshot ? new Map<string, RoomMember<W>>() : new Map(this.watchedMembers.get(scope) ?? []);
    for (const session of left) room.delete(session);
    for (const entry of entries) {
      const state = parse(scope, entry.state);
      if (state === undefined) room.delete(entry.session);
      else room.set(entry.session, { session: entry.session, colour: entry.colour, surface: entry.surface, state });
    }
    this.watchedMembers.set(scope, room);
    this.emit(this.snapshot.status);
  }

  private upsert(entry: SocketEntry): void {
    const state = this.options.parse(entry.state);
    if (state === undefined) this.members.delete(entry.session);
    else this.members.set(entry.session, { session: entry.session, colour: entry.colour, surface: entry.surface, state });
  }

  private lost(): void {
    this.detach();
    this.members = new Map();
    this.watchedMembers = new Map();
    this.emit("waiting");
  }

  private detach(): PresenceSocket | undefined {
    const socket = this.socket;
    this.socket = undefined;
    this.welcomed = false;
    this.sent = undefined;
    this.watchSent = undefined;
    clearTimeout(this.flushTimer);
    this.flushTimer = undefined;
    if (socket !== undefined) {
      socket.onopen = null;
      socket.onmessage = null;
      socket.onclose = null;
      socket.onerror = null;
    }
    return socket;
  }

  private sendWatch(): void {
    const socket = this.socket;
    const frame = this.watching?.frame;
    if (socket === undefined || socket.readyState !== OPEN || !this.welcomed || frame === undefined || frame === this.watchSent) return;
    socket.send(frame);
    this.watchSent = frame;
  }

  private schedule(): void {
    if (this.flushTimer !== undefined) return;
    const wait = this.sentAt + (this.options.intervalMs ?? PRESENCE_FRAME_INTERVAL_MS) - Date.now();
    if (wait <= 0) this.flush();
    else
      this.flushTimer = setTimeout(() => {
        this.flushTimer = undefined;
        this.flush();
      }, wait);
  }

  private flush(): void {
    const socket = this.socket;
    if (socket === undefined || socket.readyState !== OPEN || !this.welcomed || this.desired === undefined || this.desired === this.sent) return;
    socket.send(this.desired);
    this.sent = this.desired;
    this.sentAt = Date.now();
  }

  private emit(status: RoomStatus): void {
    const watched = new Map([...this.watchedMembers.entries()].map(([scope, room]) => [scope, [...room.values()]] as const));
    this.snapshot = { status, self: this.self, colour: this.colour, members: [...this.members.values()], watched, refused: this.refused };
    for (const listener of this.listeners) listener();
  }
}
//#endregion 🔌️Socket

//#region 🧭️Rooms
/** 🪪️ Who shares presence: the public tag and the identity of the learner on this device. */
export interface PresenceSelf {
  readonly tag: string;
  readonly identity: Identity;
}

/** 📌️ Where the learner is, and whether that place has a cursor room (its anchors are on screen). */
export interface PresencePlace {
  readonly place: Place;
  readonly room: boolean;
}

/** 💭️ The drafts of the open run on screen: its quiz and the answers per task. */
export interface PresenceDrafts {
  readonly quiz: string;
  readonly answers: Readonly<Record<string, ThinkingAnswer>>;
}

/** 🧭️ What the client knows right now: the catalog and its quizzes, the learner (none before identification), where it
 * is, whether it is on home (which watches the rooms of every page behind the cards), the drafts of the open run on
 * screen, and whether its tab is visible. */
export interface QuizPresenceInput {
  readonly catalog: string | undefined;
  readonly quizzes?: readonly string[];
  readonly self: PresenceSelf | undefined;
  readonly at: PresencePlace | undefined;
  readonly home?: boolean;
  readonly drafts?: PresenceDrafts;
  readonly active: boolean;
}

/** 👀️ A state of a watched room: a cursor state (a page, a quiz) or a thinking state (a quiz's thinking room). */
export type WatchedState = CursorState | ThinkingState;

/** 📸️ The rooms as the client sees them: the own tag, the roster, the joined room of the place with every watched
 * room, the joined thinking room, and the scopes joined. */
export interface QuizPresenceSnapshot {
  readonly tag: string | undefined;
  readonly roster: RoomSnapshot<PresenceState> | undefined;
  readonly room: RoomSnapshot<CursorState, WatchedState> | undefined;
  readonly thinking: RoomSnapshot<ThinkingState> | undefined;
  readonly scopes: { readonly room: string | undefined; readonly thinking: string | undefined };
}

/** ⚙️ The proctor base URL (`""` is this page's origin) and the seams: sockets, backoff bounds and jitter. */
export interface QuizPresenceOptions {
  readonly proctor: string;
  readonly connect?: PresenceConnect;
  readonly timing?: RetryTiming;
  readonly random?: () => number;
}

const THINKING_SUFFIX = "/thinking";
const parsePresence = (state: unknown): PresenceState | undefined => (presenceProblem(state) === undefined ? (state as PresenceState) : undefined);
const parseCursor = (state: unknown): CursorState | undefined => (cursorProblem(state) === undefined ? (state as CursorState) : undefined);
const parseThinking = (state: unknown): ThinkingState | undefined => (thinkingProblem(state) === undefined ? (state as ThinkingState) : undefined);
const parseWatched = (scope: string, state: unknown): WatchedState | undefined => (scope.endsWith(THINKING_SUFFIX) ? parseThinking(state) : parseCursor(state));

/** 📌️ Where a step is in presence terms. Home is the overview, or the page opened over it: a quiz's page is in that
 * quiz, the introduction, leaderboard and badges pages are their places, the learner's profile and preferences are
 * personal places without a cursor room. A run or its results is in the quiz of the run (the run on screen, with the
 * task of `task`); identity has no cursor room either. `undefined` while a run's quiz is not known yet. */
export function presencePlace(step: QuizStep, state: Pick<QuizState, "runs" | "catalog">, task: string | undefined): PresencePlace | undefined {
  switch (step.screen) {
    case "introduction":
      return { place: { screen: "introduction" }, room: true };
    case "identity":
      return { place: { screen: "identity" }, room: false };
    case "home":
      switch (step.page) {
        case undefined:
          return { place: { screen: "home" }, room: true };
        case HOME_PAGES.introduction:
          return { place: { screen: "introduction" }, room: true };
        case HOME_PAGES.leaderboard:
          return { place: { screen: "leaderboard" }, room: true };
        case HOME_PAGES.badges:
          return { place: { screen: "badges" }, room: true };
        case HOME_PAGES.learner:
          return { place: { screen: "learner" }, room: false };
        case HOME_PAGES.preferences:
          return { place: { screen: "preferences" }, room: false };
        default:
          return state.catalog?.quizzes.some((quiz) => quiz.id === step.page) === true ? { place: { screen: "quiz", quiz: step.page }, room: true } : { place: { screen: "home" }, room: true };
      }
    case "run":
    case "results": {
      const quiz = state.runs[step.run]?.quiz;
      if (quiz === undefined) return undefined;
      return { place: step.screen === "run" && task !== undefined ? { screen: "run", quiz, task } : { screen: step.screen, quiz }, room: true };
    }
  }
}

/** 💭️ The drafts of an open run as its learner shares them while thinking along: each answer in the form peers can read. */
export function presenceDrafts(view: RunView | undefined): PresenceDrafts | undefined {
  if (view === undefined || view.status !== "open") return undefined;
  const answers = view.sheet.tasks.flatMap((sheetTask) => {
    const answer = view.answers[sheetTask.id];
    const draft = answer === undefined ? undefined : thinkingAnswer(sheetTask, answer);
    return draft === undefined ? [] : [[sheetTask.id, draft] as const];
  });
  return { quiz: view.quiz, answers: Object.fromEntries(answers) };
}

/** 🏷️ The labels of the items of a run's sheet by id in `locale`, for what the others drag. */
export function sheetItemLabels(view: RunView | undefined, locale: QuizLocale): ((item: string) => string | undefined) | undefined {
  if (view === undefined) return undefined;
  const labels = new Map(view.sheet.tasks.flatMap((sheetTask) => sheetTask.items.map((item) => [item.id, localized(item.label, locale)] as const)));
  return (item) => labels.get(item);
}

/** 👀️ The rooms home watches besides the one it joined: the room of every page behind the cards (introduction,
 * leaderboard, badges, every quiz) and every quiz's thinking room, at most {@link MAX_WATCHED_ROOMS}. */
export function homeWatchScopes(catalog: string, quizzes: readonly string[], joined: string | undefined): readonly string[] {
  const pages: readonly Place[] = [{ screen: "introduction" }, { screen: "leaderboard" }, { screen: "badges" }];
  const scopes = [...pages.map((place) => roomScope(catalog, place)!), ...quizzes.map((quiz) => roomScope(catalog, { screen: "quiz", quiz })!), ...quizzes.map((quiz) => thinkingScope(catalog, quiz))];
  return scopes.filter((scope) => scope !== joined).slice(0, MAX_WATCHED_ROOMS);
}

type Joined<S, W = never> = { readonly scope: string; readonly room: PresenceRoom<S, W>; readonly off: () => void };

/** 👥️ The client's presence: the roster room while a learner is identified, the room of its place while that place has
 * one (watching the rooms of the pages behind the cards while on home), the thinking room of the open run on screen,
 * its presence state, its drafts, and its cursor, keyboard focus and drag in the place's room. Feed it with
 * {@link update}, {@link point}, {@link focusOn} and {@link drag}; read the rooms through {@link subscribe}/{@link getSnapshot}. */
export class QuizPresence {
  private readonly listeners = new Set<() => void>();
  private snapshot: QuizPresenceSnapshot = { tag: undefined, roster: undefined, room: undefined, thinking: undefined, scopes: { room: undefined, thinking: undefined } };
  private roster: Joined<PresenceState> | undefined;
  private room: Joined<CursorState, WatchedState> | undefined;
  private thinking: Joined<ThinkingState> | undefined;
  private tag: string | undefined;
  private cursor: Cursor | undefined;
  private focus: Anchor | undefined;
  private dragged: string | undefined;

  constructor(private readonly options: QuizPresenceOptions) {}

  readonly subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  readonly getSnapshot = (): QuizPresenceSnapshot => this.snapshot;

  /** 🧭️ Joins, switches or leaves rooms for `input`, sets what the place's room watches and shares the presence state
   * and drafts it implies. */
  update(input: QuizPresenceInput): void {
    const { catalog, self, at, active, drafts } = input;
    const rosterKey = catalog !== undefined && self !== undefined ? rosterScope(catalog) : undefined;
    const roomKey = rosterKey !== undefined && at?.room === true ? roomScope(catalog!, at.place) : undefined;
    const thinkingKey = rosterKey !== undefined && drafts !== undefined ? thinkingScope(catalog!, drafts.quiz) : undefined;
    if (this.roster?.scope !== rosterKey) this.roster = this.replace(this.roster, rosterKey, "quiz", { parse: parsePresence });
    if (this.room?.scope !== roomKey) {
      this.cursor = undefined;
      this.focus = undefined;
      this.dragged = undefined;
      this.room = this.replace(this.room, roomKey, at?.place.screen ?? "quiz", { parse: parseCursor, parseWatched });
    }
    if (this.thinking?.scope !== thinkingKey) this.thinking = this.replace(this.thinking, thinkingKey, "thinking", { parse: parseThinking, intervalMs: THINKING_FRAME_INTERVAL_MS });
    this.tag = self?.tag;
    if (this.roster !== undefined && self !== undefined && at !== undefined) this.roster.room.publish({ tag: self.tag, identity: self.identity, place: at.place, active });
    if (this.room !== undefined) this.room.room.watch(input.home === true && catalog !== undefined ? homeWatchScopes(catalog, input.quizzes ?? [], roomKey) : []);
    if (this.thinking !== undefined && self !== undefined && drafts !== undefined) this.thinking.room.publish({ tag: self.tag, answers: drafts.answers });
    this.shareCursor();
    this.refresh();
  }

  /** 🖱️ Shares the pointer position (`undefined`: not over an anchor). */
  point(cursor: Cursor | undefined): void {
    if (cursor?.anchor === this.cursor?.anchor && cursor?.x === this.cursor?.x && cursor?.y === this.cursor?.y) return;
    this.cursor = cursor;
    this.shareCursor();
  }

  /** ⌨️ Shares the anchor keyboard focus is in (`undefined`: none, or not the keyboard). */
  focusOn(anchor: Anchor | undefined): void {
    if (anchor === this.focus) return;
    this.focus = anchor;
    this.shareCursor();
  }

  /** 🤏️ Shares the item being dragged (`undefined`: none). */
  drag(item: string | undefined): void {
    if (item === this.dragged) return;
    this.dragged = item;
    this.shareCursor();
  }

  /** 👋️ Leaves every room. */
  stop(): void {
    this.roster = this.replace(this.roster, undefined, "quiz", { parse: parsePresence });
    this.room = this.replace(this.room, undefined, "quiz", { parse: parseCursor, parseWatched });
    this.thinking = this.replace(this.thinking, undefined, "thinking", { parse: parseThinking });
    this.cursor = undefined;
    this.focus = undefined;
    this.dragged = undefined;
    this.refresh();
  }

  private shareCursor(): void {
    if (this.room === undefined || this.tag === undefined) return;
    this.room.room.publish({
      tag: this.tag,
      ...(this.cursor === undefined ? {} : { cursor: this.cursor }),
      ...(this.focus === undefined ? {} : { focus: this.focus }),
      ...(this.dragged === undefined ? {} : { drag: { item: this.dragged } }),
    });
  }

  private replace<S, W = never>(current: Joined<S, W> | undefined, scope: string | undefined, surface: string, options: Pick<PresenceRoomOptions<S, W>, "parse" | "parseWatched" | "intervalMs">): Joined<S, W> | undefined {
    current?.off();
    current?.room.stop();
    if (scope === undefined) return undefined;
    const base = this.options.proctor === "" && typeof window !== "undefined" ? window.location.origin : this.options.proctor;
    const room = new PresenceRoom<S, W>({ ...options, url: presenceSocketUrl(base, scope, surface), connect: this.options.connect ?? browserPresenceConnect, timing: this.options.timing, random: this.options.random });
    const off = room.subscribe(() => this.refresh());
    room.start();
    return { scope, room, off };
  }

  private refresh(): void {
    const roster = this.roster?.room.getSnapshot();
    const room = this.room?.room.getSnapshot();
    const thinking = this.thinking?.room.getSnapshot();
    const previous = this.snapshot;
    if (previous.tag === this.tag && previous.roster === roster && previous.room === room && previous.thinking === thinking && previous.scopes.room === this.room?.scope && previous.scopes.thinking === this.thinking?.scope) return;
    this.snapshot = { tag: this.tag, roster, room, thinking, scopes: { room: this.room?.scope, thinking: this.thinking?.scope } };
    for (const listener of this.listeners) listener();
  }
}
//#endregion 🧭️Rooms

//#region 👁️View
/** 🖱️ Another learner in a room: its label, its palette slot (the one of its roster session, so a learner has one
 * colour everywhere; the room session's until the roster knows it), its pointer, keyboard focus and dragged item. */
export interface PeerCursor {
  readonly session: string;
  readonly tag: string;
  readonly label: string;
  readonly colour: number;
  readonly cursor: Cursor | undefined;
  readonly focus: Anchor | undefined;
  readonly drag: string | undefined;
}

/** 👁️ What the screens show of presence: the roster (undefined until the roster room is joined), the own tag, the
 * palette slot of every learner online, the other learners in this place's room, the other learners of every room the
 * client joined or watches by scope, and the drafts of the others in every thinking room by scope. */
export interface PresenceView {
  readonly me: string | undefined;
  readonly roster: PresenceRoster | undefined;
  readonly colours: ReadonlyMap<string, number>;
  readonly peers: readonly PeerCursor[];
  readonly rooms: ReadonlyMap<string, readonly PeerCursor[]>;
  readonly thinking: ReadonlyMap<string, readonly ThinkingState[]>;
}

/** 👁️ Presence while nothing is shared. */
export const EMPTY_PRESENCE_VIEW: PresenceView = { me: undefined, roster: undefined, colours: new Map(), peers: [], rooms: new Map(), thinking: new Map() };

function isThinking(state: WatchedState): state is ThinkingState {
  return Object.hasOwn(state, "answers");
}

/** 👁️ The view of a snapshot, labelling learners in `text`'s language (anonymous learners by their tag); the own
 * sessions (any tab of the own learner) are left out of every room. */
export function presenceView(snapshot: QuizPresenceSnapshot, text: QuizText): PresenceView {
  const rosterRoom = snapshot.roster;
  const joined = rosterRoom !== undefined && rosterRoom.status === "open";
  const display = (state: PresenceState): string => learnerName(state.identity, state.tag, text);
  const roster = joined ? presenceRoster(rosterRoom.members, display) : undefined;
  const slot = new Map(rosterRoom?.members.map((member) => [member.session, member.colour]) ?? []);
  const colours = new Map((roster?.learners ?? []).map((learner) => [learner.tag, slot.get(learner.session) ?? 0]));
  const names = new Map((roster?.learners ?? []).map((learner) => [learner.tag, learner.display]));
  const others = <S extends { readonly tag: string }>(members: readonly RoomMember<S>[], self: string | undefined): readonly RoomMember<S>[] => members.filter((member) => member.session !== self && member.state.tag !== snapshot.tag);
  const cursors = (members: readonly RoomMember<CursorState>[], self: string | undefined): readonly PeerCursor[] =>
    others(members, self).map((member) => ({
      session: member.session,
      tag: member.state.tag,
      label: names.get(member.state.tag) ?? learnerName(undefined, member.state.tag, text),
      colour: colours.get(member.state.tag) ?? member.colour,
      cursor: member.state.cursor,
      focus: member.state.focus,
      drag: member.state.drag?.item,
    }));
  const room = snapshot.room;
  const open = room !== undefined && room.status === "open";
  const peers = open ? cursors(room.members, room.self) : [];
  const rooms = new Map<string, readonly PeerCursor[]>();
  const thinking = new Map<string, readonly ThinkingState[]>();
  if (open && snapshot.scopes.room !== undefined) rooms.set(snapshot.scopes.room, peers);
  for (const [scope, members] of open ? room.watched : []) {
    if (scope.endsWith(THINKING_SUFFIX))
      thinking.set(
        scope,
        others(members, undefined).flatMap((member) => (isThinking(member.state) ? [member.state] : [])),
      );
    else
      rooms.set(
        scope,
        cursors(
          members.flatMap((member) => (isThinking(member.state) ? [] : [{ ...member, state: member.state }])),
          undefined,
        ),
      );
  }
  const drafts = snapshot.thinking;
  if (drafts !== undefined && drafts.status === "open" && snapshot.scopes.thinking !== undefined)
    thinking.set(
      snapshot.scopes.thinking,
      others(drafts.members, drafts.self).map((member) => member.state),
    );
  return { me: snapshot.tag, roster, colours, peers, rooms, thinking };
}

/** 🗺️ Where a learner is, in words. */
export function placeText(place: Place, catalog: CatalogView | undefined, text: QuizText, locale: QuizLocale): string {
  const quiz = catalog?.quizzes.find((candidate) => candidate.id === place.quiz);
  const title = quiz === undefined ? (place.quiz ?? "") : localized(quiz.title, locale);
  switch (place.screen) {
    case "introduction":
      return text("quiz.presence.atIntroduction");
    case "identity":
      return text("quiz.presence.atIdentity");
    case "home":
      return text("quiz.presence.atHome");
    case "leaderboard":
      return text("quiz.presence.atLeaderboard");
    case "badges":
      return text("quiz.presence.atBadges");
    case "learner":
      return text("quiz.presence.atLearner");
    case "preferences":
      return text("quiz.presence.atPreferences");
    case "quiz":
      return text("quiz.presence.atQuiz", { quiz: title });
    case "run":
      return text("quiz.presence.atRun", { quiz: title });
    case "results":
      return text("quiz.presence.atResults", { quiz: title });
  }
}

/** 🪪️ The presence self of an identified learner, once its identity is known. */
export function presenceSelf(learner: { readonly id: string; readonly identity?: Identity } | undefined): PresenceSelf | undefined {
  return learner?.identity === undefined ? undefined : { tag: learnerTag(learner.id), identity: learner.identity };
}

/** ⚓️ The anchor keys of the landmarks every learner at a place renders. */
export const PRESENCE_ANCHORS = {
  introduction: "introduction",
  leaderboard: "leaderboard",
  badges: "badges",
  run: "run",
  results: "results",
  home: (card: string): Anchor => `home:${card}`,
  quiz: (quiz: string): Anchor => `quiz:${quiz}`,
  quizTasks: (quiz: string): Anchor => `quiz:${quiz}:tasks`,
  task: (task: string): Anchor => `task:${task}`,
  result: (task: string): Anchor => `result:${task}`,
  item: (item: string): Anchor => `item:${item}`,
  category: (category: string): Anchor => `category:${category}`,
} as const;
//#endregion 👁️View

//#region ⚛️React
interface PresenceContextValue {
  readonly view: PresenceView;
  readonly showCursors: boolean;
  readonly setTask: (task: string | undefined) => void;
}

const PresenceContext = createContext<PresenceContextValue>({ view: EMPTY_PRESENCE_VIEW, showCursors: true, setTask: () => undefined });

/** 👥️ Makes `view` the presence every screen below shows (others' cursors only while `showCursors`), and `setTask`
 * where the run screen reports its task. */
export function PresenceProvider(props: { readonly view: PresenceView; readonly showCursors?: boolean; readonly setTask: (task: string | undefined) => void; readonly children: ReactNode }): ReactElement {
  return <PresenceContext.Provider value={{ view: props.view, showCursors: props.showCursors ?? true, setTask: props.setTask }}>{props.children}</PresenceContext.Provider>;
}

/** 👁️ The presence the screens show. */
export function usePresenceView(): PresenceView {
  return useContext(PresenceContext).view;
}

/** 📌️ Reports the task on screen for the learner's place, and none once the screen leaves. */
export function usePresenceTask(task: string | undefined): void {
  const { setTask } = useContext(PresenceContext);
  useEffect(() => {
    setTask(task);
    return () => setTask(undefined);
  }, [setTask, task]);
}

/** 👀️ Whether the page is visible (a hidden tab is away). */
export function useDocumentVisible(): boolean {
  const [visible, setVisible] = useState(() => typeof document === "undefined" || document.visibilityState !== "hidden");
  useEffect(() => {
    const change = (): void => setVisible(document.visibilityState !== "hidden");
    document.addEventListener("visibilitychange", change);
    return () => document.removeEventListener("visibilitychange", change);
  }, []);
  return visible;
}

/** 📐️ The cursor at viewport point (`x`, `y`) relative to `anchor`'s box, clamped to [0, 1] and rounded to 10⁻⁴;
 * `undefined` for an element without an anchor key or without a box. */
export function cursorAt(anchor: HTMLElement, x: number, y: number): Cursor | undefined {
  const key = anchor.dataset.presenceAnchor;
  const box = anchor.getBoundingClientRect();
  if (key === undefined || box.width <= 0 || box.height <= 0) return undefined;
  const unit = (value: number): number => Math.round(Math.min(1, Math.max(0, value)) * 10_000) / 10_000;
  return { anchor: key, x: unit((x - box.left) / box.width), y: unit((y - box.top) / box.height) };
}

function anchorOf(target: EventTarget | null): HTMLElement | null {
  return target instanceof Element ? target.closest<HTMLElement>("[data-presence-anchor]") : null;
}

function draggedItem(target: EventTarget | null): string | undefined {
  if (!(target instanceof Element) || target.closest("[data-quiz-grip]") === null) return undefined;
  return target.closest<HTMLElement>("[data-quiz-item]")?.dataset.quizItem;
}

/** 🖱️ Feeds `presence` from the document: the pointer over its nearest anchor (a card, an item, a category), also while
 * pressed, the item a grip drags until release, nothing once the pointer leaves the page or the window loses focus, and
 * the anchor holding keyboard focus while the keyboard moves it. */
export function usePresencePointer(presence: QuizPresence): void {
  useEffect(() => {
    if (typeof document === "undefined") return;
    let keyboard = false;
    const move = (event: PointerEvent): void => {
      const anchor = anchorOf(event.target);
      presence.point(anchor === null ? undefined : cursorAt(anchor, event.clientX, event.clientY));
    };
    const down = (event: PointerEvent): void => {
      keyboard = false;
      presence.drag(draggedItem(event.target));
    };
    const up = (): void => presence.drag(undefined);
    const leave = (): void => {
      presence.point(undefined);
      presence.drag(undefined);
    };
    const key = (event: KeyboardEvent): void => {
      keyboard = true;
      if (event.key === "Escape") presence.drag(undefined);
    };
    const focusIn = (event: FocusEvent): void => presence.focusOn(keyboard ? anchorOf(event.target)?.dataset.presenceAnchor : undefined);
    const focusOut = (event: FocusEvent): void => {
      if (event.relatedTarget === null) presence.focusOn(undefined);
    };
    const root = document.documentElement;
    document.addEventListener("pointermove", move, { passive: true });
    document.addEventListener("pointerdown", down, { capture: true, passive: true });
    window.addEventListener("pointerup", up, { capture: true });
    window.addEventListener("pointercancel", up, { capture: true });
    root.addEventListener("pointerleave", leave);
    window.addEventListener("blur", leave);
    document.addEventListener("keydown", key, { capture: true });
    document.addEventListener("focusin", focusIn);
    document.addEventListener("focusout", focusOut);
    return () => {
      document.removeEventListener("pointermove", move);
      document.removeEventListener("pointerdown", down, { capture: true });
      window.removeEventListener("pointerup", up, { capture: true });
      window.removeEventListener("pointercancel", up, { capture: true });
      root.removeEventListener("pointerleave", leave);
      window.removeEventListener("blur", leave);
      document.removeEventListener("keydown", key, { capture: true });
      document.removeEventListener("focusin", focusIn);
      document.removeEventListener("focusout", focusOut);
    };
  }, [presence]);
}

function appearance(): "light" | "dark" {
  return typeof document !== "undefined" && document.documentElement.classList.contains("dark") ? "dark" : "light";
}

/** 🖋️ The relative luminance above which black ink contrasts more with a colour than white ink does: where
 * `(L + 0.05) / 0.05` equals `1.05 / (L + 0.05)`. Either ink then reaches at least 4.58 : 1 on any colour. */
const INK_FLIP_LUMINANCE = Math.sqrt(1.05 * 0.05) - 0.05;

function channel(value: number): number {
  return value <= 0.03928 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
}

/** 🖋️ The ink that reads on the palette colour of roster slot `colour` in `appearance`: black or white, whichever
 * contrasts more by the WCAG relative luminance of that colour. */
export function peerInk(colour: number, appearance: "light" | "dark"): string {
  const { h, s, l } = presenceColor(colour, appearance);
  const chroma = s * Math.min(l, 1 - l);
  const part = (offset: number): number => {
    const turn = (offset + h / 30) % 12;
    return l - chroma * Math.max(-1, Math.min(turn - 3, 9 - turn, 1));
  };
  const luminance = 0.2126 * channel(part(0)) + 0.7152 * channel(part(8)) + 0.0722 * channel(part(4));
  return luminance > INK_FLIP_LUMINANCE ? "#000000" : "#ffffff";
}

/** 🎨️ The custom properties that colour a learner of roster slot `colour`: `--quiz-peer` in the current appearance, and
 * the ink of its name label for either appearance (`--quiz-peer-ink-light`, `--quiz-peer-ink-dark`), of which the
 * stylesheet takes the one the document shows. */
export function paintStyle(colour: number): CSSProperties {
  return { ["--quiz-peer" as string]: presencePaint(colour, appearance()), ["--quiz-peer-ink-light" as string]: peerInk(colour, "light"), ["--quiz-peer-ink-dark" as string]: peerInk(colour, "dark") } as CSSProperties;
}

function anchorSelector(anchor: string): string {
  return `[data-presence-anchor="${typeof CSS !== "undefined" && typeof CSS.escape === "function" ? CSS.escape(anchor) : anchor}"]`;
}

/** 🫣️ How far (px) a name label keeps from the control that has focus (WCAG 2.2 SC 2.4.11, focus not obscured). */
export const PEER_LABEL_CLEARANCE_PX = 24;

const LABEL_OFFSET = { cursor: { left: 16, top: 24 }, focus: { left: -2, top: -20 } } as const;

/** ⌨️ The box of the control that has focus in the document, if any. */
function focusedBox(): DOMRect | undefined {
  const active = typeof document === "undefined" ? null : document.activeElement;
  if (active === null || active === document.body) return undefined;
  const box = active.getBoundingClientRect();
  return box.width > 0 && box.height > 0 ? box : undefined;
}

/** 🫣️ Whether the name label of a mark placed at (`left`, `top`) would lie over `focus` or closer to it than
 * {@link PEER_LABEL_CLEARANCE_PX}. */
function covers(mark: HTMLElement, left: number, top: number, focus: DOMRect): boolean {
  const label = mark.querySelector<HTMLElement>(".quiz-peer-label");
  if (label === null) return false;
  const offset = LABEL_OFFSET[mark.dataset.peer === "focus" ? "focus" : "cursor"];
  const x = left + offset.left;
  const y = top + offset.top;
  const reach = PEER_LABEL_CLEARANCE_PX;
  return x < focus.right + reach && x + label.offsetWidth > focus.left - reach && y < focus.bottom + reach && y + label.offsetHeight > focus.top - reach;
}

/** 📍️ Moves every peer mark of `layer` onto its anchor's box as laid out now; marks whose anchor is not on screen hide.
 * Without `within`, anchors are looked up in the document outside inert pages and placed in viewport pixels, and a
 * mark whose name label would cover the control that has keyboard focus is marked `data-shy` (its label hides, the
 * pointer or frame stays); with `within`, inside it only and placed in its own (unscaled) pixels, so marks inside a
 * page scaled on the overview follow. */
export function placePeers(layer: HTMLElement, within?: HTMLElement): void {
  const frame = within?.getBoundingClientRect();
  const scale = within === undefined || frame === undefined || within.offsetWidth <= 0 ? 1 : frame.width / within.offsetWidth;
  const focus = within === undefined ? focusedBox() : undefined;
  for (const mark of layer.querySelectorAll<HTMLElement>("[data-peer]")) {
    const selector = anchorSelector(mark.dataset.anchor ?? "");
    const target = within === undefined ? ([...document.querySelectorAll<HTMLElement>(selector)].find((element) => element.closest("[inert]") === null) ?? null) : within.querySelector<HTMLElement>(selector);
    if (target === null) {
      mark.hidden = true;
      continue;
    }
    const seen = target.getBoundingClientRect();
    const box = frame === undefined ? seen : { left: (seen.left - frame.left) / scale, top: (seen.top - frame.top) / scale, width: seen.width / scale, height: seen.height / scale };
    mark.hidden = false;
    const left = mark.dataset.peer === "focus" ? box.left : box.left + Number(mark.dataset.x) * box.width;
    const top = mark.dataset.peer === "focus" ? box.top : box.top + Number(mark.dataset.y) * box.height;
    mark.style.transform = `translate(${left}px, ${top}px)`;
    if (mark.dataset.peer === "focus") {
      mark.style.width = `${box.width}px`;
      mark.style.height = `${box.height}px`;
    }
    mark.toggleAttribute("data-shy", focus !== undefined && covers(mark, left, top, focus));
  }
}

function PeerMarks(props: { readonly peers: readonly PeerCursor[]; readonly itemLabel?: (item: string) => string | undefined }): ReactElement {
  const { itemLabel } = props;
  return (
    <>
      {props.peers.map((peer) => {
        const label = peer.drag === undefined ? peer.label : `${peer.label} ▸ ${itemLabel?.(peer.drag) ?? peer.drag}`;
        return (
          <div key={peer.session} style={paintStyle(peer.colour)}>
            {peer.focus === undefined ? null : (
              <div data-peer="focus" data-anchor={peer.focus} data-tag={peer.tag} hidden className="quiz-peer-focus">
                <span className="quiz-peer-label">{peer.label}</span>
              </div>
            )}
            {peer.cursor === undefined ? null : (
              <div data-peer="cursor" data-anchor={peer.cursor.anchor} data-x={peer.cursor.x} data-y={peer.cursor.y} data-tag={peer.tag} data-drag={peer.drag} hidden className="quiz-peer">
                <svg viewBox="0 0 14.666391 23.899563" width="14.666391" height="23.899563" className="quiz-peer-arrow">
                  <g transform="translate(-219.91582,-287.95767)">
                    <path d="m 220.04082,288.26777 v 23.46446 h 5.64666 v -17.59835 z" />
                    <path d="m 226.42314,294.89902 7.98557,8.29594 -3.99279,4.14797 -3.99278,-4.14797 z" />
                  </g>
                </svg>
                <span className="quiz-peer-label">{label}</span>
              </div>
            )}
          </div>
        );
      })}
    </>
  );
}

function usePlacing(layer: { readonly current: HTMLDivElement | null }, active: boolean, within?: () => HTMLElement | null, everyMs = 500): void {
  useLayoutEffect(() => {
    if (layer.current !== null) placePeers(layer.current, within?.() ?? undefined);
  });
  useEffect(() => {
    if (!active) return;
    let frame = 0;
    const place = (): void => {
      if (layer.current !== null) placePeers(layer.current, within?.() ?? undefined);
    };
    const schedule = (): void => {
      if (typeof requestAnimationFrame !== "function") return place();
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(place);
    };
    window.addEventListener("resize", schedule);
    document.addEventListener("scroll", schedule, { capture: true, passive: true });
    document.addEventListener("focusin", schedule);
    const timer = setInterval(schedule, everyMs);
    return () => {
      window.removeEventListener("resize", schedule);
      document.removeEventListener("scroll", schedule, { capture: true });
      document.removeEventListener("focusin", schedule);
      clearInterval(timer);
      if (typeof cancelAnimationFrame === "function") cancelAnimationFrame(frame);
    };
  }, [active]);
}

/** 🖱️ Other learners' pointers, keyboard focus and drags over the page in their palette colours, labelled with their
 * names ("Anna ▸ Tea light" while dragging) — decorative (`aria-hidden`, never hit by the pointer); positions follow
 * layout on every render, scroll and resize, and glide between frames unless the learner prefers reduced motion. */
export function PresenceOverlay(props: { readonly view: PresenceView; readonly show: boolean; readonly itemLabel?: (item: string) => string | undefined }): ReactElement | null {
  const { view, show } = props;
  const layer = useRef<HTMLDivElement>(null);
  const active = show && view.peers.length > 0;
  usePlacing(layer, active);
  if (!active) return null;
  return (
    <div ref={layer} aria-hidden="true" data-presence-layer="" className="pointer-events-none fixed inset-0 z-40 overflow-hidden">
      <PeerMarks peers={view.peers} itemLabel={props.itemLabel} />
    </div>
  );
}

/** 🥞️ The others inside one page behind the overview's cards: the pointers and focus of the learners in that page's
 * room (`scope`, watched from home), placed on the page's own anchors so they scale and move with the page. Decorative;
 * nothing while the page is opened (the overlay of the page shows them then) or others' cursors are switched off. */
export function PanePeers(props: { readonly scope: string | undefined; readonly opened: boolean }): ReactElement | null {
  const { view, showCursors } = useContext(PresenceContext);
  const peers = props.scope === undefined ? [] : (view.rooms.get(props.scope) ?? []);
  const layer = useRef<HTMLDivElement>(null);
  const active = showCursors && !props.opened && peers.length > 0;
  usePlacing(layer, active, () => layer.current?.parentElement ?? null, 250);
  if (!active) return null;
  return (
    <div ref={layer} aria-hidden="true" data-pane-peers={props.scope} className="pointer-events-none absolute inset-0 z-10 overflow-hidden">
      <PeerMarks peers={peers} />
    </div>
  );
}

/** 🟢️ A learner's online mark in its palette colour, named for assistive technology. */
export function OnlineMark(props: { readonly colour: number; readonly text: QuizText }): ReactElement {
  return (
    <>
      <span aria-hidden="true" className="quiz-online" style={paintStyle(props.colour)} />
      <span className="sr-only">{props.text("quiz.presence.onlineMark")}</span>
    </>
  );
}

/** 👥️ The online count for the navbar: a people icon and the number, the words for assistive technology and wide
 * screens. Nothing until the roster room is joined. */
export function PresenceStatus(props: { readonly text: QuizText }): ReactElement | null {
  const { roster } = usePresenceView();
  if (roster === undefined) return null;
  return (
    <p data-presence-status="" className="m-0 flex min-w-0 items-center gap-single text-xs text-muted-foreground md:px-single">
      <Icon icon="users" size="small" className="shrink-0" />
      <span aria-hidden="true" className="tabular-nums md:hidden">
        {roster.online}
      </span>
      <span className="max-md:sr-only">{props.text("quiz.presence.online", { count: roster.online })}</span>
    </p>
  );
}

/** 🗒️ Who is where, as text: the online count and a disclosure listing every learner online with the place, the own
 * entry marked and learners whose tab is hidden marked away. */
export function PresenceList(props: { readonly catalog: CatalogView; readonly text: QuizText; readonly locale: QuizLocale; readonly className?: string }): ReactElement | null {
  const { catalog, text, locale } = props;
  const { roster, me, colours } = usePresenceView();
  if (roster === undefined) return null;
  return (
    <details data-presence-list="" className={cn("relative min-h-0 overflow-auto text-xs", props.className)}>
      <summary className="quiz-target cursor-pointer text-foreground">
        {text("quiz.presence.online", { count: roster.online })} · {text("quiz.presence.whoIsWhere")}
      </summary>
      <ul role="list" className="m-0 mt-single flex list-none flex-col gap-single p-0 text-muted-foreground">
        {roster.learners.map((learner) => (
          <li key={learner.tag} className="flex min-w-0 flex-wrap items-center gap-x-single">
            <OnlineMark colour={colours.get(learner.tag) ?? 0} text={text} />
            <span className="quiz-name text-foreground">
              {learner.display}
              {learner.tag === me ? ` (${text("quiz.presence.you")})` : ""}
            </span>
            <span>
              – {placeText(learner.place, catalog, text, locale)}
              {learner.active ? "" : ` (${text("quiz.presence.away")})`}
            </span>
          </li>
        ))}
      </ul>
    </details>
  );
}
//#endregion ⚛️React
