/** 👥️ Shared presence, cursors and drafts: the rooms learners meet in, the checks a proctor admits their states with, and the roster and thinking crowd a client shows.
 *
 * The catalog-wide roster room carries every learner's {@link PresenceState}; one room per place carries the
 * {@link CursorState}s of the learners there; the thinking room of a quiz carries every open run's
 * {@link ThinkingState}. States are ephemeral and carry the public learner tag, never the learner id.
 *
 * @see ../../README.md — presence and cursors, what the others think
 * @see ../✅️validation/🟦️.ts — `presenceIssues` / `cursorIssues` / `thinkingIssues`, the checks behind the problems
 * @see ./🦀️.rs — the Rust twin
 */
import type { Answer, Identity, Place, PresenceState, SheetTask, TaskKind, ThinkingAnswer, ThinkingState } from "../../🧬️schema/🟦️.ts";
import { valueKey } from "../👁️views/🟦️.ts";
import { answerRejection, compareCodePoints, cursorIssues, presenceIssues, thinkingIssues, type ValidationIssue } from "../✅️validation/🟦️.ts";

/** 🧍️ One roster entry as the presence socket delivers it: the server session and the learner's latest state. */
export type PresenceEntry = { readonly session: string; readonly state: PresenceState };

/** 🧑‍🤝‍🧑️ One learner online: the state of its representative session (active first, then the smallest session id), every session and its display label. */
export type PresenceLearner = {
  readonly tag: string;
  readonly identity: Identity;
  readonly display: string;
  readonly place: Place;
  readonly active: boolean;
  readonly session: string;
  readonly sessions: readonly string[];
};

/** 📋️ Who is online: distinct learners, how many are active, how many are in each quiz now, and the learners sorted by display. */
export type PresenceRoster = {
  readonly online: number;
  readonly active: number;
  readonly quizzes: Readonly<Record<string, number>>;
  readonly learners: readonly PresenceLearner[];
};

/** 🏛️ The scope of the catalog-wide roster room: the catalog id. */
export function rosterScope(catalog: string): string {
  return catalog;
}

/** 🚪️ The scope of the room of a place: `<catalog>/<screen>` for introduction, home, leaderboard and badges, `<catalog>/quiz/<quiz>` for the quiz page, a run or its results; `undefined` for identity (no tag exists yet), the personal learner and preferences pages, and a quiz page, run or results without a quiz. */
export function roomScope(catalog: string, place: Place): string | undefined {
  switch (place.screen) {
    case "identity":
    case "learner":
    case "preferences":
      return undefined;
    case "quiz":
    case "run":
    case "results":
      return place.quiz === undefined ? undefined : `${catalog}/quiz/${place.quiz}`;
    default:
      return `${catalog}/${place.screen}`;
  }
}

/** 🧭️ Whether a place is inside one quiz: its read-only page, a run or its results. */
function inQuiz(place: Place): boolean {
  return place.screen === "quiz" || place.screen === "run" || place.screen === "results";
}

/** 🚨️ The first issue of a presence state (smallest path, then code), or `undefined` when a proctor may admit it. */
export function presenceProblem(state: unknown): ValidationIssue | undefined {
  return presenceIssues(state)[0];
}

/** 🎯️ The first issue of a cursor state (smallest path, then code), or `undefined` when a proctor may admit it. */
export function cursorProblem(state: unknown): ValidationIssue | undefined {
  return cursorIssues(state)[0];
}

/** 🧮️ The roster of the given entries: sessions grouped by tag, counts per quiz over distinct learners on the page, a run or the results of that quiz, learners sorted by lowercase display, then display, then tag (code point order); `display` labels a state in the client's language. */
export function presenceRoster(entries: readonly PresenceEntry[], display: (state: PresenceState) => string): PresenceRoster {
  const byTag = new Map<string, PresenceEntry[]>();
  for (const entry of entries) byTag.set(entry.state.tag, [...(byTag.get(entry.state.tag) ?? []), entry]);
  const quizzes = new Map<string, number>();
  const learners = [...byTag.entries()].map(([tag, sessions]): PresenceLearner => {
    const ordered = [...sessions].sort((left, right) => Number(right.state.active) - Number(left.state.active) || compareCodePoints(left.session, right.session));
    const representative = ordered[0]!;
    const visited = new Set(sessions.flatMap((entry) => (inQuiz(entry.state.place) && entry.state.place.quiz !== undefined ? [entry.state.place.quiz] : [])));
    for (const quiz of visited) quizzes.set(quiz, (quizzes.get(quiz) ?? 0) + 1);
    return {
      tag,
      identity: representative.state.identity,
      display: display(representative.state),
      place: representative.state.place,
      active: representative.state.active,
      session: representative.session,
      sessions: sessions.map((entry) => entry.session).sort(compareCodePoints),
    };
  });
  learners.sort((left, right) => compareCodePoints(left.display.toLowerCase(), right.display.toLowerCase()) || compareCodePoints(left.display, right.display) || compareCodePoints(left.tag, right.tag));
  return {
    online: learners.length,
    active: learners.filter((learner) => learner.active).length,
    quizzes: Object.fromEntries([...quizzes.entries()].sort(([left], [right]) => compareCodePoints(left, right))),
    learners,
  };
}

/** 🗳️ The learners (tags) who put an item into one category or matched it with one value (`valueKey`). */
export type ThinkingVote = { readonly key: string; readonly tags: readonly string[] };

/** 📍️ Where one learner currently puts an item in a sorting: its normalized position in that learner's order (0 smallest … 1 largest). */
export type ThinkingPosition = { readonly tag: string; readonly position: number };

/** 💬️ What the others currently think of one item: who answered it, their votes per category or value (classification, matching) or positions (sorting). */
export type ThinkingItem = { readonly item: string; readonly tags: readonly string[]; readonly votes?: readonly ThinkingVote[]; readonly positions?: readonly ThinkingPosition[] };

/** 🧠️ The live crowd of one sheet task (one per dimension for matching), items in the viewer's sheet order, unanswered items left out. */
export type ThinkingTask = { readonly task: string; readonly kind: TaskKind; readonly dimension?: string; readonly items: readonly ThinkingItem[] };

/** 💡️ The scope of the thinking room of a quiz: `<catalog>/quiz/<quiz>/thinking`. */
export function thinkingScope(catalog: string, quiz: string): string {
  return `${catalog}/quiz/${quiz}/thinking`;
}

/** 🛃️ The first issue of a thinking state (smallest path, then code), or `undefined` when a proctor may admit it. */
export function thinkingProblem(state: unknown): ValidationIssue | undefined {
  return thinkingIssues(state)[0];
}

/** 🔁️ The publisher's own answer as a draft peers can read, or `undefined` when it does not fit the sheet task: classification and sorting unchanged, matching card indices replaced by the card values (non-finite card values left out). */
export function thinkingAnswer(sheetTask: SheetTask, answer: Answer): ThinkingAnswer | undefined {
  if (answerRejection(sheetTask, answer) !== undefined) return undefined;
  if (answer.kind !== "matching") return answer;
  if (sheetTask.kind !== "matching") return undefined;
  const values = Object.fromEntries(
    Object.entries(answer.assignments).map(([dimension, cards]) => {
      const deck = sheetTask.dimensions.find((candidate) => candidate.id === dimension)!.cards;
      return [dimension, Object.fromEntries(Object.entries(cards).flatMap(([item, card]) => (Number.isFinite(deck[card]) ? [[item, deck[card]!]] : [])))];
    }),
  );
  return { kind: "matching", values };
}

/** 🏷️ Tags in code point order. */
function sortedTags(tags: Iterable<string>): string[] {
  return [...tags].sort(compareCodePoints);
}

/** 🗂️ An item's votes from (tag, key) pairs, or none when nobody answered it: keys and tags in code point order. */
function voted(item: string, pairs: readonly (readonly [string, string])[]): ThinkingItem[] {
  if (pairs.length === 0) return [];
  const votes = new Map<string, string[]>();
  for (const [tag, key] of pairs) votes.set(key, [...(votes.get(key) ?? []), tag]);
  return [{ item, tags: sortedTags(pairs.map(([tag]) => tag)), votes: [...votes.entries()].sort(([left], [right]) => compareCodePoints(left, right)).map(([key, tags]) => ({ key, tags: sortedTags(tags) })) }];
}

/** 🫧️ The live drafts of the others aggregated semantically by item id for the viewer's sheet task, items in the viewer's sheet order: classification votes per category; sorting positions `index / (len − 1)` in each learner's own order (0 for a single item, by tag); matching votes per value key (`valueKey`) per dimension. One state per tag, the last given winning. */
export function thinkingCrowd(states: readonly ThinkingState[], sheetTask: SheetTask): ThinkingTask[] {
  const latest = [...new Map(states.map((state) => [state.tag, state] as const)).values()];
  const drafts = latest.flatMap((state) => {
    const answer = Object.hasOwn(state.answers, sheetTask.id) ? state.answers[sheetTask.id] : undefined;
    return answer && answer.kind === sheetTask.kind ? [{ tag: state.tag, answer }] : [];
  });
  switch (sheetTask.kind) {
    case "classification": {
      const items = sheetTask.items.flatMap((item) => voted(item.id, drafts.flatMap(({ tag, answer }) => (answer.kind === "classification" && Object.hasOwn(answer.assignments, item.id) ? [[tag, answer.assignments[item.id]!] as const] : []))));
      return [{ task: sheetTask.id, kind: sheetTask.kind, items }];
    }
    case "sorting": {
      const items = sheetTask.items.flatMap((item): ThinkingItem[] => {
        const positions = drafts.flatMap(({ tag, answer }): ThinkingPosition[] => {
          if (answer.kind !== "sorting") return [];
          const index = answer.order.indexOf(item.id);
          return index < 0 ? [] : [{ tag, position: answer.order.length > 1 ? index / (answer.order.length - 1) : 0 }];
        });
        if (positions.length === 0) return [];
        const ordered = positions.sort((left, right) => compareCodePoints(left.tag, right.tag));
        return [{ item: item.id, tags: ordered.map((position) => position.tag), positions: ordered }];
      });
      return [{ task: sheetTask.id, kind: sheetTask.kind, items }];
    }
    case "matching":
      return sheetTask.dimensions.map((dimension): ThinkingTask => ({
        task: sheetTask.id,
        kind: sheetTask.kind,
        dimension: dimension.id,
        items: sheetTask.items.flatMap((item) =>
          voted(
            item.id,
            drafts.flatMap(({ tag, answer }) => {
              const values = answer.kind === "matching" && Object.hasOwn(answer.values, dimension.id) ? answer.values[dimension.id]! : undefined;
              return values && Object.hasOwn(values, item.id) ? [[tag, valueKey(values[item.id]!)] as const] : [];
            }),
          ),
        ),
      }));
  }
}
