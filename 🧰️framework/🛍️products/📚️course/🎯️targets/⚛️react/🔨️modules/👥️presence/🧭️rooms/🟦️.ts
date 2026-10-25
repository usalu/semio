/** 🧭️ The presence rooms of the quiz client: who stands where and what they draft, per page and task, and the watch over the rooms of the home pages. */

import { presenceSocketUrl } from "@semio-tech/framework-server";
import { cursorProblem, presenceProblem, roomScope, rosterScope, thinkingAnswer, thinkingProblem, thinkingScope, type Anchor, type Cursor, type CursorState, type Identity, type Place, type PresencePresentation, type PresenceState, type RunView, type ThinkingAnswer, type ThinkingState } from "@semio-tech/course";
import { localized, type CourseLocale } from "../../🌐️i18n/🟦️.ts";
import { type RetryTiming } from "../../🛂️proctor/🟦️.ts";
import { HOME_PAGES, type CourseState, type CourseStep } from "../../🧭️session/🧮️state/🟦️.ts";
import { browserPresenceConnect, MAX_WATCHED_ROOMS, type PresenceConnect, PresenceRoom, type PresenceRoomOptions, type RoomSnapshot, THINKING_FRAME_INTERVAL_MS } from "../🔌️socket/🟦️.ts";

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

/** 📙️ The drafts of the open run on screen: its quiz and the answers per task. */
export interface PresenceDrafts {
  readonly quiz: string;
  readonly answers: Readonly<Record<string, ThinkingAnswer>>;
}

/** 📥️ What the client knows right now: the course and its quizzes, the learner (none before identification), where it
 * is, whether it is on home (which watches the rooms of every page behind the cards), the drafts of the open run on
 * screen, and whether its tab is visible. */
export interface CoursePresenceInput {
  readonly course: string | undefined;
  readonly quizzes?: readonly string[];
  readonly self: PresenceSelf | undefined;
  readonly at: PresencePlace | undefined;
  readonly home?: boolean;
  readonly homeRoute?: string;
  readonly view?: PresencePresentation;
  readonly drafts?: PresenceDrafts;
  readonly active: boolean;
}

/** 👀️ A state of a watched room: a cursor state (a page, a quiz) or a thinking state (a quiz's thinking room). */
export type WatchedState = CursorState | ThinkingState;

/** 📸️ The rooms as the client sees them: the own tag, the roster, the joined room of the place with every watched
 * room, the joined thinking room, and the scopes joined. */
export interface CoursePresenceSnapshot {
  readonly tag: string | undefined;
  readonly place?: Place;
  readonly home?: string;
  readonly roster: RoomSnapshot<PresenceState> | undefined;
  readonly room: RoomSnapshot<CursorState, WatchedState> | undefined;
  readonly thinking: RoomSnapshot<ThinkingState> | undefined;
  readonly scopes: { readonly room: string | undefined; readonly thinking: string | undefined };
}

/** ⚙️ The proctor base URL (`""` is this page's origin) and the seams: sockets, backoff bounds and jitter. */
export interface CoursePresenceOptions {
  readonly proctor: string;
  readonly connect?: PresenceConnect;
  readonly timing?: RetryTiming;
  readonly random?: () => number;
}

export const THINKING_SUFFIX = "/thinking";
const parsePresence = (state: unknown): PresenceState | undefined => (presenceProblem(state) === undefined ? (state as PresenceState) : undefined);
const parseCursor = (state: unknown): CursorState | undefined => (cursorProblem(state) === undefined ? (state as CursorState) : undefined);
const parseThinking = (state: unknown): ThinkingState | undefined => (thinkingProblem(state) === undefined ? (state as ThinkingState) : undefined);
const parseWatched = (scope: string, state: unknown): WatchedState | undefined => (scope.endsWith(THINKING_SUFFIX) ? parseThinking(state) : parseCursor(state));

/** 🪜️ Where a step is in presence terms. Home is the overview, or the page opened over it: a quiz's page is in that
 * quiz, the leaderboard, badges, learner and preferences pages are their places with cursor rooms. A run or its results is in the quiz of the run (the run on screen, with the
 * task of `task`); identity has no cursor room either. `undefined` while a run's quiz is not known yet. */
export function presencePlace(step: CourseStep, state: Pick<CourseState, "runs" | "course">, task: string | undefined): PresencePlace | undefined {
  switch (step.screen) {
    case "introduction":
      return { place: { screen: "introduction" }, room: true };
    case "identity":
      return { place: { screen: "identity" }, room: false };
    case "home":
      switch (step.page) {
        case undefined:
          return { place: { screen: "home" }, room: true };
        case HOME_PAGES.leaderboard:
          return { place: { screen: "leaderboard" }, room: true };
        case HOME_PAGES.badges:
          return { place: { screen: "badges" }, room: true };
        case HOME_PAGES.learner:
          return { place: { screen: "learner" }, room: true };
        case HOME_PAGES.preferences:
          return { place: { screen: "preferences" }, room: true };
        default:
          return state.course?.quizzes.some((quiz) => quiz.id === step.page) === true ? { place: { screen: "quiz", quiz: step.page }, room: true } : { place: { screen: "home" }, room: true };
      }
    case "run":
    case "results": {
      const quiz = state.runs[step.run]?.quiz;
      if (quiz === undefined) return undefined;
      return { place: { screen: step.screen, quiz, ...(task === undefined ? {} : { task }) }, room: true };
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
export function sheetItemLabels(view: RunView | undefined, locale: CourseLocale): ((item: string) => string | undefined) | undefined {
  if (view === undefined) return undefined;
  const labels = new Map(view.sheet.tasks.flatMap((sheetTask) => sheetTask.items.map((item) => [item.id, localized(item.label, locale)] as const)));
  return (item) => labels.get(item);
}

/** 🏠️ The rooms home watches besides the one it joined: the room of every page behind the cards (introduction,
 * leaderboard, badges, learner, preferences, every quiz) and every quiz's thinking room, at most {@link MAX_WATCHED_ROOMS}. */
export function homeWatchScopes(course: string, quizzes: readonly string[], joined: string | undefined): readonly string[] {
  const pages: readonly Place[] = [{ screen: "introduction" }, { screen: "leaderboard" }, { screen: "badges" }, { screen: "learner" }, { screen: "preferences" }];
  const scopes = [...pages.map((place) => roomScope(course, place)!), ...quizzes.map((quiz) => roomScope(course, { screen: "quiz", quiz })!), ...quizzes.map((quiz) => thinkingScope(course, quiz))];
  return scopes.filter((scope) => scope !== joined).slice(0, MAX_WATCHED_ROOMS);
}

type Joined<S, W = never> = { readonly scope: string; readonly room: PresenceRoom<S, W>; readonly off: () => void };

/** 👥️ The client's presence: the roster room while a learner is identified, the room of its place while that place has
 * one (watching the rooms of the pages behind the cards while on home), the thinking room of the open run on screen,
 * its presence state, its drafts, and its cursor, keyboard focus and drag in the place's room. Feed it with
 * {@link update}, {@link point}, {@link focusOn} and {@link drag}; read the rooms through {@link subscribe}/{@link getSnapshot}. */
export class CoursePresence {
  private readonly listeners = new Set<() => void>();
  private snapshot: CoursePresenceSnapshot = { tag: undefined, roster: undefined, room: undefined, thinking: undefined, scopes: { room: undefined, thinking: undefined } };
  private roster: Joined<PresenceState> | undefined;
  private room: Joined<CursorState, WatchedState> | undefined;
  private thinking: Joined<ThinkingState> | undefined;
  private tag: string | undefined;
  private place: Place | undefined;
  private home: string | undefined;
  private cursor: Cursor | undefined;
  private focus: Anchor | undefined;
  private dragged: string | undefined;

  constructor(private readonly options: CoursePresenceOptions) {}

  readonly subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  readonly getSnapshot = (): CoursePresenceSnapshot => this.snapshot;

  /** 🔄️ Joins, switches or leaves rooms for `input`, sets what the place's room watches and shares the presence state
   * and drafts it implies. */
  update(input: CoursePresenceInput): void {
    const { course, self, at, active, drafts, view } = input;
    const rosterKey = course !== undefined && self !== undefined ? rosterScope(course) : undefined;
    const roomKey = rosterKey !== undefined && at?.room === true ? roomScope(course!, at.place) : undefined;
    const thinkingKey = rosterKey !== undefined && drafts !== undefined ? thinkingScope(course!, drafts.quiz) : undefined;
    const home = at?.place.screen === "home" ? input.homeRoute ?? view?.home : undefined;
    if (this.room?.scope !== roomKey || this.tag !== self?.tag || this.place?.screen !== at?.place.screen || this.place?.quiz !== at?.place.quiz || this.place?.task !== at?.place.task || this.home !== home) {
      this.cursor = undefined;
      this.focus = undefined;
      this.dragged = undefined;
    }
    this.tag = self?.tag;
    this.place = at?.place;
    this.home = home;
    if (this.roster?.scope !== rosterKey) this.roster = this.replace(this.roster, rosterKey, "course", { parse: parsePresence });
    if (this.room?.scope !== roomKey) this.room = this.replace(this.room, roomKey, at?.place.screen ?? "course", { parse: parseCursor, parseWatched });
    if (this.thinking?.scope !== thinkingKey) this.thinking = this.replace(this.thinking, thinkingKey, "thinking", { parse: parseThinking, intervalMs: THINKING_FRAME_INTERVAL_MS });
    if (this.roster !== undefined && self !== undefined && at !== undefined) this.roster.room.publish({ tag: self.tag, identity: self.identity, place: at.place, active, ...(view === undefined ? {} : { view }) });
    if (this.room !== undefined) this.room.room.watch(input.home === true && course !== undefined ? homeWatchScopes(course, input.quizzes ?? [], roomKey) : []);
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
    this.roster = this.replace(this.roster, undefined, "course", { parse: parsePresence });
    this.room = this.replace(this.room, undefined, "course", { parse: parseCursor, parseWatched });
    this.thinking = this.replace(this.thinking, undefined, "thinking", { parse: parseThinking });
    this.tag = undefined;
    this.place = undefined;
    this.home = undefined;
    this.cursor = undefined;
    this.focus = undefined;
    this.dragged = undefined;
    this.refresh();
  }

  private shareCursor(): void {
    if (this.room === undefined || this.tag === undefined) return;
    this.room.room.publish({
      tag: this.tag,
      ...(this.place === undefined ? {} : { place: this.place }),
      ...(this.home === undefined ? {} : { home: this.home }),
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
    if (previous.tag === this.tag && previous.place === this.place && previous.home === this.home && previous.roster === roster && previous.room === room && previous.thinking === thinking && previous.scopes.room === this.room?.scope && previous.scopes.thinking === this.thinking?.scope) return;
    this.snapshot = { tag: this.tag, place: this.place, home: this.home, roster, room, thinking, scopes: { room: this.room?.scope, thinking: this.thinking?.scope } };
    for (const listener of this.listeners) listener();
  }
}
//#endregion 🧭️Rooms
